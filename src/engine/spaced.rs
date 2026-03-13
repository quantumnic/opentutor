use rusqlite::Connection;

/// SM-2 algorithm parameters with FSRS-inspired enhancements
pub const DEFAULT_DESIRED_RETENTION: f64 = 0.85;
const MIN_EASE: f64 = 1.3;
/// Maximum interval in days (cap at ~6 months)
const MAX_INTERVAL: i64 = 180;
/// Days overdue before a card is considered lapsed
const LAPSE_THRESHOLD_DAYS: i64 = 7;
/// Graduated re-learning steps for lapsed cards (in days)
const RELEARN_STEPS: [i64; 3] = [1, 3, 7];
/// Minimum streak days for a bonus
const STREAK_BONUS_THRESHOLD: i64 = 3;
/// Maximum streak multiplier
pub const MAX_STREAK_BONUS: f64 = 1.3;
/// Maximum fuzz percentage applied to intervals (±5%)
const FUZZ_FACTOR: f64 = 0.05;
/// Consecutive failures before a card is marked as a leech
const LEECH_THRESHOLD: i64 = 4;
/// FSRS-inspired decay constant for the power-law forgetting curve
const FSRS_DECAY: f64 = 0.5;
/// Factor relating stability to the 90% retention interval (from FSRS-4.5)
const FSRS_FACTOR: f64 = 19.0 / 81.0;
/// FSRS-5 initial stability estimates (in days) based on first rating (quality 0-5).
/// Derived from FSRS-5 paper default weights: initial stability grows with answer quality.
const INITIAL_STABILITY: [f64; 6] = [0.4, 0.6, 1.0, 2.0, 4.0, 6.0];
/// Same-day review bonus: fraction of normal interval awarded when re-reviewing
/// a topic on the same day (prevents wasted reviews but gives partial credit).
const SAME_DAY_REVIEW_FACTOR: f64 = 0.25;

/// Returns a difficulty multiplier for initial intervals based on topic difficulty.
/// Harder topics get shorter initial intervals (multiplier < 1.0) to reinforce
/// learning more frequently. Easy topics may get slightly longer intervals.
fn topic_difficulty_factor(conn: &Connection, topic_id: i64) -> f64 {
    let difficulty: String = conn
        .query_row(
            "SELECT difficulty FROM topics WHERE id = ?1",
            [topic_id],
            |r| r.get(0),
        )
        .unwrap_or_else(|_| "beginner".to_string());

    // Also consider user's ease factor — if they're struggling, shorten intervals
    let ease: f64 = conn
        .query_row(
            "SELECT COALESCE(ease_factor, 2.5) FROM user_progress WHERE topic_id = ?1",
            [topic_id],
            |r| r.get(0),
        )
        .unwrap_or(2.5);

    let base = match difficulty.as_str() {
        "advanced" => 0.6,       // 60% of normal interval (review sooner)
        "intermediate" => 0.8,   // 80% of normal interval
        _ => 1.0,                // beginner: normal intervals
    };

    // If ease is very low (user struggling), compress further
    if ease < 1.8 {
        base * 0.75
    } else if ease < 2.2 {
        base * 0.9
    } else {
        base
    }
}

/// Apply a small random fuzz to an interval to prevent review clustering.
/// For intervals >= 4 days, adds ±FUZZ_FACTOR jitter (at least ±1 day).
fn fuzz_interval(interval: i64) -> i64 {
    if interval < 4 {
        return interval; // Don't fuzz very short intervals
    }
    let jitter_range = ((interval as f64) * FUZZ_FACTOR).max(1.0).round() as i64;
    let jitter = (rand::random::<u64>() % (2 * jitter_range as u64 + 1)) as i64 - jitter_range;
    (interval + jitter).clamp(1, MAX_INTERVAL)
}

/// Calculate next review date using an improved SM-2 algorithm.
/// quality: 0-5 (0-2 = fail, 3-5 = pass)
pub fn update_spaced_repetition(
    conn: &Connection,
    topic_id: i64,
    quality: u8,
) -> Result<(), rusqlite::Error> {
    let quality = quality.min(5);

    let current: Option<(f64, i64, Option<String>)> = conn
        .query_row(
            "SELECT ease_factor, interval_days, next_review FROM user_progress WHERE topic_id = ?1",
            [topic_id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
        )
        .ok();

    let (ease, interval, _next_review) = current.unwrap_or((2.5, 0, None));

    // Detect same-day review: if already reviewed today, give partial credit
    // instead of a full interval bump (prevents gaming via repeated same-day reviews).
    let is_same_day: bool = conn
        .query_row(
            "SELECT COUNT(*) > 0 FROM user_progress
             WHERE topic_id = ?1 AND last_reviewed IS NOT NULL
             AND DATE(last_reviewed) = DATE('now')",
            [topic_id],
            |r| r.get(0),
        )
        .unwrap_or(false);

    // Track consecutive failures for leech detection
    let (consecutive_fails, leech_count): (i64, i64) = conn
        .query_row(
            "SELECT consecutive_fails, leech_count FROM user_progress WHERE topic_id = ?1",
            [topic_id],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .unwrap_or((0, 0));

    // Check if the card has lapsed (overdue by more than LAPSE_THRESHOLD_DAYS)
    let is_lapsed = is_card_lapsed(conn, topic_id);

    // Calculate streak bonus: consistent daily practice earns longer intervals
    let streak = calculate_streak(conn);
    let streak_bonus = if streak >= STREAK_BONUS_THRESHOLD {
        // Bonus scales from 1.0 to MAX_STREAK_BONUS over streaks 3-14
        let bonus = 1.0 + ((streak - STREAK_BONUS_THRESHOLD) as f64 / 11.0) * (MAX_STREAK_BONUS - 1.0);
        bonus.min(MAX_STREAK_BONUS)
    } else {
        1.0
    };

    let (new_ease, new_interval) = if quality >= 3 {
        if is_lapsed {
            // Graduated re-learning: pick a step based on how far along the
            // user is in recovery. Use the current interval to determine
            // which re-learning step to assign.
            let step_interval = if interval <= RELEARN_STEPS[0] {
                RELEARN_STEPS[1] // advance to step 2
            } else if interval <= RELEARN_STEPS[1] {
                RELEARN_STEPS[2] // advance to step 3
            } else {
                // Beyond re-learning: restore to reduced interval
                (interval as f64 * 0.5).max(RELEARN_STEPS[2] as f64).round() as i64
            };
            let new_ease = (ease - 0.15).max(MIN_EASE);
            (new_ease, step_interval.min(MAX_INTERVAL))
        } else {
            // Normal correct answer — SM-2 graduation with FSRS-5 initial stability
            // First review uses FSRS-5 stability estimate based on quality rating
            let difficulty_factor = topic_difficulty_factor(conn, topic_id);
            let new_interval = match interval {
                0 => {
                    // FSRS-5: use quality-dependent initial stability
                    let s0 = INITIAL_STABILITY[quality as usize];
                    (s0 * difficulty_factor).round().max(1.0) as i64
                }
                1 => (3.0 * difficulty_factor).round().max(1.0) as i64,
                n => {
                    let calculated = (n as f64 * ease).round() as i64;
                    let quality_bonus = if quality == 5 { 1.1 } else { 1.0 };
                    let same_day_factor = if is_same_day { SAME_DAY_REVIEW_FACTOR } else { 1.0 };
                    (calculated as f64 * quality_bonus * streak_bonus * same_day_factor)
                        .round()
                        .max(n as f64) as i64 // Never shrink interval on success
                }
            };
            let new_ease = ease + (0.1 - (5.0 - quality as f64) * (0.08 + (5.0 - quality as f64) * 0.02));
            (new_ease.max(MIN_EASE), new_interval.min(MAX_INTERVAL))
        }
    } else {
        // Incorrect — reset interval but penalize ease less for near-misses
        let ease_penalty = match quality {
            2 => 0.10,
            1 => 0.15,
            _ => 0.20,
        };
        let new_ease = (ease - ease_penalty).max(MIN_EASE);
        (new_ease, 1)
    };

    // Scale interval by desired retention: higher retention → shorter intervals.
    // Uses the FSRS power-law forgetting curve to adjust.
    let desired_retention = crate::commands::config::get_desired_retention(conn);
    let retention_scaled_interval = if (desired_retention - DEFAULT_DESIRED_RETENTION).abs() > 0.001 && new_interval > 1 {
        // Derived from R(t) = (1 + t / (S * FSRS_FACTOR))^(-FSRS_DECAY)
        // Ratio of intervals for two retention targets (at same stability):
        //   t2/t1 = ((1/R2)^(1/FSRS_DECAY) - 1) / ((1/R1)^(1/FSRS_DECAY) - 1)
        let ratio_new = (1.0 / desired_retention).powf(1.0 / FSRS_DECAY) - 1.0;
        let ratio_default = (1.0 / DEFAULT_DESIRED_RETENTION).powf(1.0 / FSRS_DECAY) - 1.0;
        if ratio_default > 0.0 {
            ((new_interval as f64) * (ratio_new / ratio_default)).round().max(1.0) as i64
        } else {
            new_interval
        }
    } else {
        new_interval
    };

    // Apply interval fuzzing to prevent review clustering on the same day
    let final_interval = fuzz_interval(retention_scaled_interval);

    // Update leech tracking
    let (new_consecutive_fails, new_leech_count) = if quality < 3 {
        let new_fails = consecutive_fails + 1;
        let new_leeches = if new_fails >= LEECH_THRESHOLD && (new_fails - LEECH_THRESHOLD) % LEECH_THRESHOLD == 0 {
            leech_count + 1
        } else {
            leech_count
        };
        (new_fails, new_leeches)
    } else {
        (0, leech_count) // Reset consecutive fails on success, but keep leech count
    };

    conn.execute(
        "INSERT INTO user_progress (topic_id, ease_factor, interval_days, next_review, last_reviewed, consecutive_fails, leech_count)
         VALUES (?1, ?2, ?3, datetime('now', '+' || ?3 || ' days'), datetime('now'), ?4, ?5)
         ON CONFLICT(topic_id) DO UPDATE SET
           ease_factor = ?2,
           interval_days = ?3,
           next_review = datetime('now', '+' || ?3 || ' days'),
           last_reviewed = datetime('now'),
           consecutive_fails = ?4,
           leech_count = ?5",
        rusqlite::params![topic_id, new_ease, final_interval, new_consecutive_fails, new_leech_count],
    )?;
    Ok(())
}

/// Check if a card has lapsed (overdue by more than the lapse threshold).
pub fn is_card_lapsed(conn: &Connection, topic_id: i64) -> bool {
    conn.query_row(
        "SELECT COUNT(*) > 0 FROM user_progress
         WHERE topic_id = ?1
           AND next_review IS NOT NULL
           AND next_review <= datetime('now', ?2)",
        rusqlite::params![topic_id, format!("-{} days", LAPSE_THRESHOLD_DAYS)],
        |r| r.get(0),
    )
    .unwrap_or(false)
}

/// Get count of topics due for review.
pub fn count_due_topics(conn: &Connection) -> Result<i64, rusqlite::Error> {
    conn.query_row(
        "SELECT COUNT(*) FROM user_progress WHERE next_review IS NOT NULL AND next_review <= datetime('now')",
        [],
        |r| r.get(0),
    )
}

/// Get count of lapsed topics (overdue by more than threshold).
pub fn count_lapsed_topics(conn: &Connection) -> Result<i64, rusqlite::Error> {
    conn.query_row(
        &format!(
            "SELECT COUNT(*) FROM user_progress
             WHERE next_review IS NOT NULL
               AND next_review <= datetime('now', '-{} days')",
            LAPSE_THRESHOLD_DAYS
        ),
        [],
        |r| r.get(0),
    )
}

/// Get topics due for review.
#[allow(dead_code)]
pub fn get_due_topics(conn: &Connection) -> Result<Vec<(i64, String, String)>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT t.id, t.name, s.name FROM topics t
         JOIN subjects s ON t.subject_id = s.id
         JOIN user_progress p ON p.topic_id = t.id
         WHERE p.next_review <= datetime('now')
         ORDER BY p.next_review ASC",
    )?;
    let rows = stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?;
    rows.collect()
}

/// Calculate review urgency score. Higher = more urgent.
/// Factors: how overdue, ease factor (lower = more fragile), lapsed status.
pub fn review_urgency(conn: &Connection, topic_id: i64) -> f64 {
    let result: Option<(f64, i64, Option<String>)> = conn
        .query_row(
            "SELECT ease_factor, interval_days, next_review FROM user_progress WHERE topic_id = ?1",
            [topic_id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
        )
        .ok();

    match result {
        Some((ease, interval, Some(_next_review))) => {
            // Overdue ratio: how many intervals overdue
            let overdue_days: f64 = conn
                .query_row(
                    "SELECT CAST(julianday('now') - julianday(next_review) AS REAL)
                     FROM user_progress WHERE topic_id = ?1",
                    [topic_id],
                    |r| r.get(0),
                )
                .unwrap_or(0.0);

            if overdue_days <= 0.0 {
                return 0.0; // Not due yet
            }

            let overdue_ratio = overdue_days / (interval.max(1) as f64);
            let ease_penalty = (3.0 - ease).max(0.0); // Lower ease = higher urgency
            let lapsed_bonus = if is_card_lapsed(conn, topic_id) { 2.0 } else { 0.0 };

            overdue_ratio + ease_penalty + lapsed_bonus
        }
        _ => 0.0,
    }
}

/// Calculate the current learning streak (consecutive days with activity).
pub fn calculate_streak(conn: &Connection) -> i64 {
    let mut stmt = match conn.prepare(
        "SELECT DISTINCT DATE(timestamp) as day FROM session_log ORDER BY day DESC LIMIT 30",
    ) {
        Ok(s) => s,
        Err(_) => return 0,
    };

    let days: Vec<String> = stmt
        .query_map([], |r| r.get(0))
        .ok()
        .map(|rows| rows.filter_map(|r| r.ok()).collect())
        .unwrap_or_default();

    if days.is_empty() {
        return 0;
    }

    let today = chrono::Local::now().date_naive();
    let mut streak = 0i64;
    let mut expected = today;

    for day_str in &days {
        if let Ok(day) = chrono::NaiveDate::parse_from_str(day_str, "%Y-%m-%d") {
            if day == expected {
                streak += 1;
                expected = match expected.pred_opt() {
                    Some(d) => d,
                    None => break,
                };
            } else if streak == 0 && day == today.pred_opt().unwrap_or(today) {
                streak += 1;
                expected = match day.pred_opt() {
                    Some(d) => d,
                    None => break,
                };
            } else {
                break;
            }
        }
    }

    streak
}

/// Estimate current memory retention using an FSRS-inspired power-law forgetting curve.
/// Returns a value between 0.0 and 1.0 (100% = perfect retention).
/// FSRS formula: R = (1 + FACTOR * t/S)^(-1/DECAY)
/// where t = elapsed days, S = stability (days for retention to drop to ~90%).
/// This is more accurate than the exponential Ebbinghaus model because real
/// memory decay follows a power law — memories decline steeply at first, then
/// plateau, matching empirical findings from Wozniak (SuperMemo) and Ye (FSRS-4.5).
pub fn estimate_retention(conn: &Connection, topic_id: i64) -> f64 {
    let result: Option<(f64, i64, Option<String>)> = conn
        .query_row(
            "SELECT ease_factor, interval_days, last_reviewed FROM user_progress WHERE topic_id = ?1",
            [topic_id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
        )
        .ok();

    match result {
        Some((ease, interval, Some(last_reviewed))) => {
            let days_since: f64 = conn
                .query_row(
                    "SELECT MAX(0, julianday('now') - julianday(?1))",
                    [&last_reviewed],
                    |r| r.get(0),
                )
                .unwrap_or(0.0);

            // Stability derived from scheduled interval and ease factor
            let stability = (interval as f64) * ease / 2.5;
            if stability <= 0.0 {
                return 0.0;
            }

            // FSRS power-law forgetting curve
            let retention = (1.0 + FSRS_FACTOR * days_since / stability).powf(-1.0 / FSRS_DECAY);
            retention.clamp(0.0, 1.0)
        }
        _ => 0.0, // No progress data
    }
}

/// Compute the optimal interval for a desired retention target.
#[allow(dead_code)]
/// Uses the inverse of the FSRS power-law forgetting curve:
///   t = S / FACTOR × (R^(-DECAY) - 1)
/// where S = stability, R = desired retention.
/// Returns None if no progress data exists.
pub fn optimal_interval_for_retention(
    conn: &Connection,
    topic_id: i64,
    desired_retention: Option<f64>,
) -> Option<i64> {
    let (ease, interval): (f64, i64) = conn
        .query_row(
            "SELECT ease_factor, interval_days FROM user_progress WHERE topic_id = ?1",
            [topic_id],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .ok()?;

    let r = desired_retention.unwrap_or(DEFAULT_DESIRED_RETENTION).clamp(0.5, 0.99);
    let stability = (interval as f64) * ease / 2.5;
    if stability <= 0.0 {
        return Some(1);
    }
    // Inverse of FSRS power-law: t = S / FACTOR × (R^(-DECAY) - 1)
    let optimal = (stability / FSRS_FACTOR * (r.powf(-FSRS_DECAY) - 1.0)).round() as i64;
    Some(optimal.clamp(1, MAX_INTERVAL))
}

/// Get average retention across all studied topics.
pub fn average_retention(conn: &Connection) -> f64 {
    let mut stmt = match conn.prepare(
        "SELECT topic_id FROM user_progress WHERE last_reviewed IS NOT NULL",
    ) {
        Ok(s) => s,
        Err(_) => return 0.0,
    };
    let ids: Vec<i64> = stmt
        .query_map([], |r| r.get(0))
        .ok()
        .map(|rows| rows.filter_map(|r| r.ok()).collect())
        .unwrap_or_default();

    if ids.is_empty() {
        return 0.0;
    }

    let total: f64 = ids.iter().map(|&id| estimate_retention(conn, id)).sum();
    total / ids.len() as f64
}

/// Check if a topic is a leech (repeatedly failed).
pub fn is_leech(conn: &Connection, topic_id: i64) -> bool {
    conn.query_row(
        "SELECT leech_count > 0 FROM user_progress WHERE topic_id = ?1",
        [topic_id],
        |r| r.get(0),
    )
    .unwrap_or(false)
}

/// Get leech count for a topic.
#[allow(dead_code)]
pub fn get_leech_count(conn: &Connection, topic_id: i64) -> i64 {
    conn.query_row(
        "SELECT leech_count FROM user_progress WHERE topic_id = ?1",
        [topic_id],
        |r| r.get(0),
    )
    .unwrap_or(0)
}

/// Get all leech topics.
pub fn get_leeches(conn: &Connection) -> Result<Vec<(i64, String, String, i64)>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT t.id, t.name, s.name, p.leech_count
         FROM user_progress p
         JOIN topics t ON t.id = p.topic_id
         JOIN subjects s ON s.id = t.subject_id
         WHERE p.leech_count > 0
         ORDER BY p.leech_count DESC"
    )?;
    let rows = stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)))?;
    rows.collect()
}

/// Get a topic's memory strength as a descriptive string.
#[allow(dead_code)]
pub fn memory_strength(interval_days: i64, ease_factor: f64) -> &'static str {
    if interval_days >= 60 && ease_factor >= 2.3 {
        "Strong 💪"
    } else if interval_days >= 21 {
        "Good 🌟"
    } else if interval_days >= 7 {
        "Growing 🌱"
    } else if interval_days >= 3 {
        "Fragile 🔨"
    } else {
        "New 🌱"
    }
}

/// Calculate the memory stability (half-life in days) using the FSRS power-law model.
/// Stability represents the interval at which retrievability drops to ~50%.
/// Higher stability = more durable memory.
pub fn stability_half_life(interval_days: i64, ease_factor: f64) -> f64 {
    // Using FSRS-4.5 model: R(t) = (1 + t/(9·S))^(-1)
    // where S is stability. We approximate stability from the current interval
    // and ease factor: a card with a longer interval and higher ease has
    // demonstrated more durable memory.
    let base_stability = interval_days as f64 * ease_factor / 2.5;
    // Half-life: time t where R(t) = 0.5
    // 0.5 = (1 + t/(9·S))^(-1) → t = 9·S·(2^1 - 1) = 9·S
    // But with decay exponent: t_half = S · (2^(1/FSRS_DECAY) - 1) · FSRS_FACTOR^(-1/FSRS_DECAY)
    let t_half = base_stability * (2.0_f64.powf(1.0 / FSRS_DECAY) - 1.0) / FSRS_FACTOR.powf(1.0 / FSRS_DECAY);
    t_half.max(0.5) // at least half a day
}

/// Compute the expected retrievability (0.0–1.0) given elapsed days since last review.
/// Uses the FSRS power-law forgetting curve with the card's estimated stability.
pub fn retrievability(conn: &Connection, topic_id: i64) -> f64 {
    let data: Option<(f64, i64, Option<String>)> = conn
        .query_row(
            "SELECT ease_factor, interval_days, last_reviewed FROM user_progress WHERE topic_id = ?1",
            [topic_id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
        )
        .ok();
    let (ease, interval, last_reviewed) = match data {
        Some(d) => d,
        None => return 0.0,
    };
    let elapsed = match last_reviewed {
        Some(ref dt) => {
            let reviewed = chrono::NaiveDate::parse_from_str(dt, "%Y-%m-%d")
                .or_else(|_| {
                    chrono::NaiveDateTime::parse_from_str(dt, "%Y-%m-%d %H:%M:%S")
                        .map(|ndt| ndt.date())
                })
                .unwrap_or_else(|_| chrono::Local::now().date_naive());
            let today = chrono::Local::now().date_naive();
            (today - reviewed).num_days().max(0) as f64
        }
        None => 0.0,
    };
    let stability = (interval as f64 * ease / 2.5).max(1.0);
    // FSRS power-law: R(t) = (1 + t / (9·S))^(-1)
    let r = (1.0 + elapsed / (9.0 * stability)).powf(-1.0);
    r.clamp(0.0, 1.0)
}

/// Sort due topics by priority: combine urgency (overdue ratio) with
/// retrievability to surface the cards most at risk of being forgotten.
pub fn prioritized_due_topics(conn: &Connection) -> Result<Vec<(i64, String, String, f64)>, rusqlite::Error> {
    let topics = get_due_topics(conn)?;
    let mut scored: Vec<(i64, String, String, f64)> = topics
        .into_iter()
        .map(|(id, name, subj)| {
            let urgency = review_urgency(conn, id);
            let ret = retrievability(conn, id);
            // Priority: high urgency + low retrievability = most important
            let priority = urgency * (1.0 - ret);
            (id, name, subj, priority)
        })
        .collect();
    scored.sort_by(|a, b| b.3.partial_cmp(&a.3).unwrap_or(std::cmp::Ordering::Equal));
    Ok(scored)
}

/// Default daily review cap (configurable via `opentutor config daily_review_cap`)
const DEFAULT_DAILY_REVIEW_CAP: usize = 50;

/// Get the user-configured daily review cap, falling back to default.
pub fn get_daily_review_cap(conn: &Connection) -> usize {
    conn.query_row(
        "SELECT value FROM user_config WHERE key = 'daily_review_cap'",
        [],
        |r| r.get::<_, String>(0),
    )
    .ok()
    .and_then(|v| v.parse::<usize>().ok())
    .unwrap_or(DEFAULT_DAILY_REVIEW_CAP)
}

/// Get prioritized due topics, capped at the user's daily review limit.
/// This prevents review overload when many cards become due at once.
#[allow(dead_code)]
pub fn capped_due_topics(conn: &Connection) -> Result<Vec<(i64, String, String, f64)>, rusqlite::Error> {
    let cap = get_daily_review_cap(conn);
    let mut topics = prioritized_due_topics(conn)?;
    topics.truncate(cap);
    Ok(topics)
}

/// Count how many reviews the user has already done today.
pub fn reviews_done_today(conn: &Connection) -> i64 {
    conn.query_row(
        "SELECT COUNT(*) FROM session_log
         WHERE activity_type = 'review' AND date(timestamp) = date('now')",
        [],
        |r| r.get(0),
    )
    .unwrap_or(0)
}

/// Remaining reviews for today based on cap and completed reviews.
pub fn remaining_reviews_today(conn: &Connection) -> usize {
    let cap = get_daily_review_cap(conn);
    let done = reviews_done_today(conn) as usize;
    cap.saturating_sub(done)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_spaced_repetition_correct() {
        let conn = db::init_memory_db().unwrap();
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days)
             VALUES (1, 100.0, 1, 1, 2.5, 1)", []
        ).unwrap();
        update_spaced_repetition(&conn, 1, 4).unwrap();
        let (ease, interval): (f64, i64) = conn.query_row(
            "SELECT ease_factor, interval_days FROM user_progress WHERE topic_id = 1",
            [], |r| Ok((r.get(0)?, r.get(1)?))
        ).unwrap();
        assert!(ease >= 2.4);
        assert_eq!(interval, 3, "Second review should be 3 days");
    }

    #[test]
    fn test_spaced_repetition_graduation() {
        let conn = db::init_memory_db().unwrap();
        // First: interval 0 -> FSRS-5 initial stability for quality 4 = 4 days
        update_spaced_repetition(&conn, 1, 4).unwrap();
        let interval: i64 = conn.query_row(
            "SELECT interval_days FROM user_progress WHERE topic_id = 1",
            [], |r| r.get(0)
        ).unwrap();
        assert!(interval >= 3 && interval <= 5, "First review (q4) should use FSRS-5 initial stability ~4 days, got {}", interval);

        // Second: same-day review gets partial credit, but with fuzz
        // interval stays around the same or grows slightly
        update_spaced_repetition(&conn, 1, 4).unwrap();
        let interval2: i64 = conn.query_row(
            "SELECT interval_days FROM user_progress WHERE topic_id = 1",
            [], |r| r.get(0)
        ).unwrap();
        assert!(interval2 >= 3, "Second review (same-day) should maintain reasonable interval, got {}", interval2);

        // Third: further same-day review, interval should stay stable or grow
        update_spaced_repetition(&conn, 1, 4).unwrap();
        let interval3: i64 = conn.query_row(
            "SELECT interval_days FROM user_progress WHERE topic_id = 1",
            [], |r| r.get(0)
        ).unwrap();
        assert!(interval3 >= 3, "Third review should maintain interval (±fuzz), got {}", interval3);
    }

    #[test]
    fn test_spaced_repetition_fail_resets_interval() {
        let conn = db::init_memory_db().unwrap();
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days)
             VALUES (1, 50.0, 5, 3, 2.5, 6)", []
        ).unwrap();
        update_spaced_repetition(&conn, 1, 1).unwrap();
        let interval: i64 = conn.query_row(
            "SELECT interval_days FROM user_progress WHERE topic_id = 1",
            [], |r| r.get(0)
        ).unwrap();
        assert_eq!(interval, 1, "Failed review should reset interval to 1");
    }

    #[test]
    fn test_interval_capped_at_max() {
        let conn = db::init_memory_db().unwrap();
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days)
             VALUES (1, 100.0, 10, 10, 3.0, 170)", []
        ).unwrap();
        update_spaced_repetition(&conn, 1, 5).unwrap();
        let interval: i64 = conn.query_row(
            "SELECT interval_days FROM user_progress WHERE topic_id = 1",
            [], |r| r.get(0)
        ).unwrap();
        assert!(interval <= MAX_INTERVAL, "Interval should be capped at {}, got {}", MAX_INTERVAL, interval);
    }

    #[test]
    fn test_get_due_topics_empty() {
        let conn = db::init_memory_db().unwrap();
        let due = get_due_topics(&conn).unwrap();
        assert!(due.is_empty());
    }

    #[test]
    fn test_count_due_topics() {
        let conn = db::init_memory_db().unwrap();
        assert_eq!(count_due_topics(&conn).unwrap(), 0);
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days, next_review)
             VALUES (1, 80.0, 3, 2, 2.5, 1, datetime('now', '-1 day'))", []
        ).unwrap();
        assert_eq!(count_due_topics(&conn).unwrap(), 1);
    }

    #[test]
    fn test_count_lapsed_topics() {
        let conn = db::init_memory_db().unwrap();
        assert_eq!(count_lapsed_topics(&conn).unwrap(), 0);
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days, next_review)
             VALUES (1, 80.0, 3, 2, 2.5, 1, datetime('now', '-10 days'))", []
        ).unwrap();
        assert_eq!(count_lapsed_topics(&conn).unwrap(), 1);
    }

    #[test]
    fn test_ease_never_below_min() {
        let conn = db::init_memory_db().unwrap();
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days)
             VALUES (1, 0.0, 10, 0, 1.3, 1)", []
        ).unwrap();
        update_spaced_repetition(&conn, 1, 0).unwrap();
        let ease: f64 = conn.query_row(
            "SELECT ease_factor FROM user_progress WHERE topic_id = 1",
            [], |r| r.get(0)
        ).unwrap();
        assert!(ease >= 1.3, "Ease factor should never go below 1.3");
    }

    #[test]
    fn test_memory_strength() {
        assert_eq!(memory_strength(90, 2.5), "Strong 💪");
        assert_eq!(memory_strength(30, 2.0), "Good 🌟");
        assert_eq!(memory_strength(10, 2.0), "Growing 🌱");
        assert_eq!(memory_strength(4, 2.0), "Fragile 🔨");
        assert_eq!(memory_strength(1, 2.0), "New 🌱");
    }

    #[test]
    fn test_review_urgency_not_due() {
        let conn = db::init_memory_db().unwrap();
        // No progress = no urgency
        assert_eq!(review_urgency(&conn, 1), 0.0);
    }

    #[test]
    fn test_review_urgency_overdue() {
        let conn = db::init_memory_db().unwrap();
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days, next_review)
             VALUES (1, 80.0, 3, 2, 2.5, 5, datetime('now', '-3 days'))", []
        ).unwrap();
        let urgency = review_urgency(&conn, 1);
        assert!(urgency > 0.0, "Overdue topic should have positive urgency, got {}", urgency);
    }

    #[test]
    fn test_graduated_relearning() {
        let conn = db::init_memory_db().unwrap();
        // Create a lapsed card (overdue by 10+ days)
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days, next_review)
             VALUES (1, 80.0, 5, 4, 2.5, 1, datetime('now', '-10 days'))", []
        ).unwrap();
        // Review it — should get re-learning step, not full reset
        update_spaced_repetition(&conn, 1, 4).unwrap();
        let interval: i64 = conn.query_row(
            "SELECT interval_days FROM user_progress WHERE topic_id = 1",
            [], |r| r.get(0)
        ).unwrap();
        assert_eq!(interval, 3, "Lapsed card at step 1 should advance to step 2 (3 days)");
    }

    #[test]
    fn test_calculate_streak_empty() {
        let conn = db::init_memory_db().unwrap();
        assert_eq!(calculate_streak(&conn), 0);
    }

    #[test]
    fn test_calculate_streak_with_activity() {
        let conn = db::init_memory_db().unwrap();
        // Insert activity for today
        conn.execute(
            "INSERT INTO session_log (topic_id, activity_type, timestamp) VALUES (1, 'learn', datetime('now'))",
            [],
        ).unwrap();
        let streak = calculate_streak(&conn);
        assert!(streak >= 1, "Should have at least 1 day streak, got {}", streak);
    }

    #[test]
    fn test_fuzz_interval_short_unchanged() {
        // Intervals < 4 should not be fuzzed
        assert_eq!(fuzz_interval(1), 1);
        assert_eq!(fuzz_interval(2), 2);
        assert_eq!(fuzz_interval(3), 3);
    }

    #[test]
    fn test_fuzz_interval_long_within_range() {
        // For interval=100, fuzz should be ±5 (5%), so result in [95, 105]
        for _ in 0..50 {
            let fuzzed = fuzz_interval(100);
            assert!(fuzzed >= 95 && fuzzed <= 105,
                "Fuzzed interval {} out of expected range [95, 105]", fuzzed);
        }
    }

    #[test]
    fn test_fuzz_interval_never_exceeds_max() {
        for _ in 0..50 {
            let fuzzed = fuzz_interval(MAX_INTERVAL);
            assert!(fuzzed <= MAX_INTERVAL,
                "Fuzzed interval {} exceeds MAX_INTERVAL {}", fuzzed, MAX_INTERVAL);
        }
    }

    #[test]
    fn test_leech_detection() {
        let conn = db::init_memory_db().unwrap();
        // Create progress
        update_spaced_repetition(&conn, 1, 4).unwrap();
        assert!(!is_leech(&conn, 1));

        // Fail 4 times consecutively to trigger leech
        for _ in 0..4 {
            update_spaced_repetition(&conn, 1, 1).unwrap();
        }
        assert!(is_leech(&conn, 1), "Should be a leech after 4 consecutive failures");
        assert_eq!(get_leech_count(&conn, 1), 1);
    }

    #[test]
    fn test_leech_resets_on_success() {
        let conn = db::init_memory_db().unwrap();
        update_spaced_repetition(&conn, 1, 4).unwrap();

        // Fail 3 times then succeed — should NOT become a leech
        for _ in 0..3 {
            update_spaced_repetition(&conn, 1, 1).unwrap();
        }
        update_spaced_repetition(&conn, 1, 4).unwrap();
        let fails: i64 = conn.query_row(
            "SELECT consecutive_fails FROM user_progress WHERE topic_id = 1",
            [], |r| r.get(0)
        ).unwrap();
        assert_eq!(fails, 0, "Consecutive fails should reset on success");
    }

    #[test]
    fn test_get_leeches_empty() {
        let conn = db::init_memory_db().unwrap();
        let leeches = get_leeches(&conn).unwrap();
        assert!(leeches.is_empty());
    }

    #[test]
    fn test_optimal_interval_for_retention() {
        let conn = db::init_memory_db().unwrap();
        // No progress → None
        assert!(optimal_interval_for_retention(&conn, 1, None).is_none());

        // Add progress
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days)
             VALUES (1, 100.0, 5, 5, 2.5, 10)", []
        ).unwrap();
        let interval = optimal_interval_for_retention(&conn, 1, Some(0.85)).unwrap();
        assert!(interval >= 1, "Should compute a positive interval, got {}", interval);
        assert!(interval <= MAX_INTERVAL);

        // Higher retention → shorter interval
        let high = optimal_interval_for_retention(&conn, 1, Some(0.95)).unwrap();
        let low = optimal_interval_for_retention(&conn, 1, Some(0.70)).unwrap();
        assert!(high <= low, "Higher retention ({}) should need shorter interval than lower ({})", high, low);
    }

    #[test]
    fn test_average_retention_empty() {
        let conn = db::init_memory_db().unwrap();
        assert_eq!(average_retention(&conn), 0.0);
    }

    #[test]
    fn test_average_retention_with_data() {
        let conn = db::init_memory_db().unwrap();
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days, last_reviewed)
             VALUES (1, 100.0, 5, 5, 2.5, 10, datetime('now'))", []
        ).unwrap();
        let avg = average_retention(&conn);
        // Just reviewed → retention should be very high (close to 1.0)
        assert!(avg > 0.9, "Just-reviewed topic should have high retention, got {}", avg);
    }

    #[test]
    fn test_quality_5_bonus() {
        let conn = db::init_memory_db().unwrap();
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days)
             VALUES (1, 100.0, 5, 5, 2.5, 10)", []
        ).unwrap();
        update_spaced_repetition(&conn, 1, 5).unwrap();
        let interval_q5: i64 = conn.query_row(
            "SELECT interval_days FROM user_progress WHERE topic_id = 1",
            [], |r| r.get(0)
        ).unwrap();

        // Reset and try quality 4
        conn.execute(
            "UPDATE user_progress SET ease_factor = 2.5, interval_days = 10 WHERE topic_id = 1", []
        ).unwrap();
        update_spaced_repetition(&conn, 1, 4).unwrap();
        let interval_q4: i64 = conn.query_row(
            "SELECT interval_days FROM user_progress WHERE topic_id = 1",
            [], |r| r.get(0)
        ).unwrap();

        assert!(interval_q5 >= interval_q4, "Quality 5 should give equal or longer interval than quality 4");
    }

    #[test]
    fn test_stability_half_life() {
        // A card with interval 10 days and ease 2.5 should have a reasonable half-life
        let hl = stability_half_life(10, 2.5);
        assert!(hl > 0.5, "Half-life should be positive");
        // Higher ease → higher half-life
        let hl_high_ease = stability_half_life(10, 3.0);
        assert!(hl_high_ease > hl, "Higher ease factor should give longer half-life");
        // Longer interval → higher half-life
        let hl_long_interval = stability_half_life(30, 2.5);
        assert!(hl_long_interval > hl, "Longer interval should give longer half-life");
    }

    #[test]
    fn test_stability_half_life_minimum() {
        // Even tiny intervals should return at least 0.5
        let hl = stability_half_life(0, 1.3);
        assert!((hl - 0.5).abs() < f64::EPSILON, "Minimum half-life should be 0.5");
    }

    #[test]
    fn test_retrievability_no_progress() {
        let conn = db::init_memory_db().unwrap();
        let r = retrievability(&conn, 9999);
        assert!((r - 0.0).abs() < f64::EPSILON, "No progress should return 0.0");
    }

    #[test]
    fn test_retrievability_just_reviewed() {
        let conn = db::init_memory_db().unwrap();
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days, last_reviewed)
             VALUES (1, 80.0, 3, 2, 2.5, 10, ?1)",
            [&today],
        ).unwrap();
        let r = retrievability(&conn, 1);
        assert!(r > 0.9, "Just-reviewed card should have high retrievability, got {}", r);
    }

    #[test]
    fn test_prioritized_due_topics_empty() {
        let conn = db::init_memory_db().unwrap();
        let topics = prioritized_due_topics(&conn).unwrap();
        assert!(topics.is_empty());
    }

    #[test]
    fn test_difficulty_factor_beginner() {
        let conn = db::init_memory_db().unwrap();
        // Topic 1 should be beginner difficulty
        let factor = topic_difficulty_factor(&conn, 1);
        assert!((factor - 1.0).abs() < 0.01, "Beginner should have factor ~1.0, got {}", factor);
    }

    #[test]
    fn test_difficulty_factor_with_low_ease() {
        let conn = db::init_memory_db().unwrap();
        // Simulate struggling user
        conn.execute(
            "INSERT INTO user_progress (topic_id, ease_factor, interval_days, score, attempts, correct)
             VALUES (1, 1.5, 1, 30.0, 5, 1)",
            [],
        ).unwrap();
        let factor = topic_difficulty_factor(&conn, 1);
        assert!(factor < 1.0, "Low ease should compress intervals, got {}", factor);
    }

    #[test]
    fn test_difficulty_aware_intervals() {
        let conn = db::init_memory_db().unwrap();
        // Find an intermediate topic
        let int_id: Option<i64> = conn.query_row(
            "SELECT id FROM topics WHERE difficulty = 'intermediate' LIMIT 1",
            [],
            |r| r.get(0),
        ).ok();
        if let Some(topic_id) = int_id {
            let factor = topic_difficulty_factor(&conn, topic_id);
            assert!(factor <= 1.0, "Intermediate should have factor <= 1.0, got {}", factor);
        }
    }

    #[test]
    fn test_daily_review_cap_default() {
        let conn = db::init_memory_db().unwrap();
        assert_eq!(get_daily_review_cap(&conn), DEFAULT_DAILY_REVIEW_CAP);
    }

    #[test]
    fn test_daily_review_cap_custom() {
        let conn = db::init_memory_db().unwrap();
        conn.execute(
            "INSERT INTO user_config (key, value) VALUES ('daily_review_cap', '25')",
            [],
        ).unwrap();
        assert_eq!(get_daily_review_cap(&conn), 25);
    }

    #[test]
    fn test_capped_due_topics_respects_cap() {
        let conn = db::init_memory_db().unwrap();
        // Set a very small cap
        conn.execute(
            "INSERT INTO user_config (key, value) VALUES ('daily_review_cap', '2')",
            [],
        ).unwrap();
        let topics = capped_due_topics(&conn).unwrap();
        assert!(topics.len() <= 2);
    }

    #[test]
    fn test_reviews_done_today_empty() {
        let conn = db::init_memory_db().unwrap();
        assert_eq!(reviews_done_today(&conn), 0);
    }

    #[test]
    fn test_remaining_reviews_today() {
        let conn = db::init_memory_db().unwrap();
        let remaining = remaining_reviews_today(&conn);
        assert_eq!(remaining, DEFAULT_DAILY_REVIEW_CAP);
    }
}


