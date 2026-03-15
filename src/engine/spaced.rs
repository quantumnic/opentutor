use rusqlite::Connection;

/// SM-2 algorithm parameters with FSRS-inspired enhancements
pub const DEFAULT_DESIRED_RETENTION: f64 = 0.85;
/// Minimum adaptive retention target
const MIN_ADAPTIVE_RETENTION: f64 = 0.75;
/// Maximum adaptive retention target
const MAX_ADAPTIVE_RETENTION: f64 = 0.95;
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
/// Fatigue decay: quality is reduced by this amount per review done in the
/// current session (beyond the first 10). Models cognitive fatigue during
/// long review sessions — later reviews are worth slightly less.
const FATIGUE_THRESHOLD: i64 = 10;
const FATIGUE_DECAY_PER_REVIEW: f64 = 0.05;
/// Sleep consolidation bonus: reviewing on a new day (after sleep) earns a
/// small interval multiplier. Research shows memory consolidation during
/// sleep strengthens traces reviewed the previous day (Walker, 2017).
const SLEEP_CONSOLIDATION_BONUS: f64 = 1.08;
/// Contextual strengthening: bonus when reviewing multiple topics from the
/// same subject in one session. Interleaved practice within a domain creates
/// stronger associative networks (Rohrer & Taylor, 2007).
const CONTEXT_STRENGTHENING_BONUS: f64 = 1.06;
/// Window (in seconds) for counting sibling-topic reviews in the same session.
const CONTEXT_WINDOW_SECONDS: i64 = 3600;

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

/// Retrieval practice bonus: harder question types earn a small quality boost.
/// Fill-in-blank and ordering require active recall (generation effect),
/// which produces stronger memory traces than recognition-based formats
/// like multiple choice (Karpicke & Roediger, 2008).
#[allow(dead_code)]
pub fn retrieval_practice_bonus(quality: u8, question_type: &str) -> u8 {
    if !(3..5).contains(&quality) {
        return quality; // No bonus on failure or already max
    }
    let bonus: u8 = match question_type {
        "fill_in_blank" => 1,  // Active recall: hardest
        "ordering" => 1,      // Requires full sequence knowledge
        _ => 0,               // multiple_choice, true_false: recognition-based
    };
    (quality + bonus).min(5)
}

/// Calculate fatigue-adjusted quality based on how many reviews done today.
/// After FATIGUE_THRESHOLD reviews, each additional review slightly reduces
/// the effective quality score, modeling cognitive fatigue. This encourages
/// users to spread reviews across multiple sessions.
pub fn fatigue_adjusted_quality(conn: &Connection, quality: u8) -> u8 {
    let done = reviews_done_today(conn);
    if done <= FATIGUE_THRESHOLD || quality <= 1 {
        return quality;
    }
    let fatigue_reviews = (done - FATIGUE_THRESHOLD) as f64;
    let penalty = (fatigue_reviews * FATIGUE_DECAY_PER_REVIEW).min(1.0);
    let adjusted = (quality as f64 - penalty).floor().max(1.0) as u8;
    adjusted.min(quality) // Never increase quality
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

    // Sleep consolidation: check if last review was on a previous day.
    // Memory research shows sleep-dependent consolidation strengthens
    // memories reviewed before sleep (Walker, 2017; Diekelmann & Born, 2010).
    let has_sleep_gap: bool = conn
        .query_row(
            "SELECT COUNT(*) > 0 FROM user_progress
             WHERE topic_id = ?1 AND last_reviewed IS NOT NULL
             AND DATE(last_reviewed) < DATE('now')",
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

    // Compute stability decay for long-abandoned cards
    let decay = stability_decay_factor(conn, topic_id);

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
            // Apply stability decay: severely abandoned cards get shorter relearn intervals
            let decayed_ease_penalty = if decay < 0.8 { 0.25 } else { 0.15 };
            let new_ease = (ease - decayed_ease_penalty).max(MIN_EASE);
            let decayed_interval = (step_interval as f64 * decay).round().max(1.0) as i64;
            (new_ease, decayed_interval.min(MAX_INTERVAL))
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
                    let sleep_bonus = if has_sleep_gap && !is_same_day { SLEEP_CONSOLIDATION_BONUS } else { 1.0 };
                    let spacing = spacing_bonus(conn, topic_id);
                    let interleave = interleaving_bonus(conn);
                    let context_bonus = context_strengthening(conn, topic_id);
                    (calculated as f64 * quality_bonus * streak_bonus * same_day_factor * sleep_bonus * spacing * interleave * context_bonus)
                        .round()
                        .max(n as f64) as i64 // Never shrink interval on success
                }
            };
            let new_ease = ease + (0.1 - (5.0 - quality as f64) * (0.08 + (5.0 - quality as f64) * 0.02));
            (new_ease.max(MIN_EASE), new_interval.min(MAX_INTERVAL))
        }
    } else {
        // Incorrect — reset interval but penalize ease less for near-misses.
        // New-card grace period: if the card has never been successfully reviewed
        // (interval == 0 or attempts <= 2), apply a softer ease penalty.
        // This prevents new topics from being immediately crushed by a single mistake.
        let attempts: i64 = conn
            .query_row(
                "SELECT COALESCE(attempts, 0) FROM user_progress WHERE topic_id = ?1",
                [topic_id],
                |r| r.get(0),
            )
            .unwrap_or(0);

        let is_new_card = interval <= 1 && attempts <= 2;
        let ease_penalty = if is_new_card {
            // Grace period: halve the normal penalty for brand-new cards
            match quality {
                2 => 0.05,
                1 => 0.08,
                _ => 0.10,
            }
        } else {
            match quality {
                2 => 0.10,
                1 => 0.15,
                _ => 0.20,
            }
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

    // Record time-of-day performance for future scheduling insights
    let current_hour: u32 = conn
        .query_row("SELECT CAST(strftime('%H', 'now', 'localtime') AS INTEGER)", [], |r| r.get::<_, i64>(0))
        .unwrap_or(12) as u32;
    let _ = record_time_of_day_performance(conn, current_hour, quality);

    Ok(())
}

/// Calculate a stability decay factor for long-abandoned cards.
/// Cards that haven't been reviewed in > 3× their scheduled interval suffer
/// accelerated forgetting — stability erodes over time. This models the
/// empirical finding that memories become increasingly fragile when left
/// unreinforced far beyond their optimal review point.
/// Returns a multiplier in [0.3, 1.0] where 1.0 = no decay.
pub fn stability_decay_factor(conn: &Connection, topic_id: i64) -> f64 {
    let data: Option<(i64, Option<String>, Option<String>)> = conn
        .query_row(
            "SELECT interval_days, next_review, last_reviewed FROM user_progress WHERE topic_id = ?1",
            [topic_id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
        )
        .ok();

    match data {
        Some((interval, Some(_next_review), Some(_last_reviewed))) if interval > 0 => {
            let overdue_days: f64 = conn
                .query_row(
                    "SELECT MAX(0, julianday('now') - julianday(next_review))
                     FROM user_progress WHERE topic_id = ?1",
                    [topic_id],
                    |r| r.get(0),
                )
                .unwrap_or(0.0);

            if overdue_days <= 0.0 {
                return 1.0; // Not overdue
            }

            let overdue_ratio = overdue_days / interval as f64;
            if overdue_ratio <= 3.0 {
                return 1.0; // Within normal lapse range
            }

            // Exponential decay: factor = e^(-0.1 * (ratio - 3))
            // Capped at 0.3 to prevent complete erasure
            let decay = (-0.1 * (overdue_ratio - 3.0)).exp();
            decay.clamp(0.3, 1.0)
        }
        _ => 1.0,
    }
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

/// Calculate the inter-day spacing bonus: reviewing a topic on different days
/// is more effective than cramming on the same day. Returns a multiplier > 1.0
/// when the user has reviewed this topic across multiple distinct days.
pub fn spacing_bonus(conn: &Connection, topic_id: i64) -> f64 {
    let distinct_days: i64 = conn
        .query_row(
            "SELECT COUNT(DISTINCT DATE(timestamp)) FROM session_log
             WHERE topic_id = ?1 AND activity_type IN ('review', 'quiz', 'learn')
             AND timestamp >= datetime('now', '-14 days')",
            [topic_id],
            |r| r.get(0),
        )
        .unwrap_or(0);

    // Bonus scales from 1.0 (1 day) to 1.2 (7+ distinct days in last 2 weeks)
    let bonus = 1.0 + (distinct_days.min(7) as f64 - 1.0).max(0.0) * 0.033;
    bonus.min(1.2)
}

/// Calculate an interleaving bonus: if the user reviewed topics from multiple
/// subjects in the current session, they get a small quality boost.
/// Research shows interleaved practice improves long-term retention by 10-30%
/// compared to blocked practice (Rohrer & Taylor, 2007).
pub fn interleaving_bonus(conn: &Connection) -> f64 {
    let distinct_subjects: i64 = conn
        .query_row(
            "SELECT COUNT(DISTINCT s.id)
             FROM session_log sl
             JOIN topics t ON t.id = sl.topic_id
             JOIN subjects s ON s.id = t.subject_id
             WHERE sl.activity_type IN ('review', 'quiz')
             AND DATE(sl.timestamp) = DATE('now')",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);

    // Bonus: 1.0 for single subject, up to 1.15 for 4+ subjects interleaved
    match distinct_subjects {
        0..=1 => 1.0,
        2 => 1.05,
        3 => 1.10,
        _ => 1.15,
    }
}

/// Contextual strengthening bonus: rewards reviewing multiple topics from the
/// same subject within a session window. This builds associative networks
/// between related concepts, improving long-term retention.
pub fn context_strengthening(conn: &Connection, topic_id: i64) -> f64 {
    let sibling_count: i64 = conn
        .query_row(
            "SELECT COUNT(DISTINCT sl.topic_id)
             FROM session_log sl
             JOIN topics t ON t.id = sl.topic_id
             JOIN topics t2 ON t2.subject_id = t.subject_id
             WHERE t2.id = ?1
             AND sl.topic_id != ?1
             AND sl.activity_type IN ('review', 'quiz')
             AND sl.timestamp >= datetime('now', ?2)",
            rusqlite::params![topic_id, format!("-{} seconds", CONTEXT_WINDOW_SECONDS)],
            |r| r.get(0),
        )
        .unwrap_or(0);

    if sibling_count >= 2 {
        CONTEXT_STRENGTHENING_BONUS
    } else {
        1.0
    }
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

/// Average minutes per review, estimated from session logs.
/// Defaults to 2.0 minutes if no data available.
#[allow(dead_code)]
const DEFAULT_MINUTES_PER_REVIEW: f64 = 2.0;

/// Estimate how many minutes the user should study today based on due reviews
/// and their historical review pace.
/// Returns (estimated_minutes, due_count, avg_minutes_per_review).
#[allow(dead_code)]
pub fn estimate_study_time(conn: &Connection) -> (f64, i64, f64) {
    let due = count_due_topics(conn).unwrap_or(0);
    let remaining = remaining_reviews_today(conn) as i64;
    let reviewable = due.min(remaining);

    // Estimate pace from recent session data: avg time between consecutive reviews
    let avg_pace = average_review_pace(conn).unwrap_or(DEFAULT_MINUTES_PER_REVIEW);
    let estimated = reviewable as f64 * avg_pace;

    (estimated, due, avg_pace)
}

/// Calculate the average minutes between consecutive reviews from session logs.
/// Uses the last 50 review entries to estimate pace.
#[allow(dead_code)]
fn average_review_pace(conn: &Connection) -> Option<f64> {
    let mut stmt = conn.prepare(
        "SELECT timestamp FROM session_log
         WHERE activity_type = 'review'
         ORDER BY timestamp DESC LIMIT 50",
    ).ok()?;
    let timestamps: Vec<String> = stmt
        .query_map([], |r| r.get(0))
        .ok()?
        .filter_map(|r| r.ok())
        .collect();

    if timestamps.len() < 2 {
        return None;
    }

    // Parse timestamps and compute average gap
    let mut total_gap_minutes = 0.0;
    let mut gaps = 0;

    for pair in timestamps.windows(2) {
        let t1 = chrono::NaiveDateTime::parse_from_str(&pair[0], "%Y-%m-%d %H:%M:%S").ok();
        let t2 = chrono::NaiveDateTime::parse_from_str(&pair[1], "%Y-%m-%d %H:%M:%S").ok();
        if let (Some(newer), Some(older)) = (t1, t2) {
            let gap = (newer - older).num_seconds() as f64 / 60.0;
            // Only count gaps under 30 minutes (ignore session breaks)
            if gap > 0.0 && gap < 30.0 {
                total_gap_minutes += gap;
                gaps += 1;
            }
        }
    }

    if gaps > 0 {
        let avg = total_gap_minutes / gaps as f64;
        Some(avg.clamp(0.5, 15.0)) // Sanity bounds
    } else {
        None
    }
}

/// A memory forecast entry: (topic_id, name, subject, days_until_critical, current_retention).
#[allow(dead_code)]
pub type ForecastEntry = (i64, String, String, f64, f64);

/// Forecast memory state: for each studied topic, predict when retention will
/// drop below the desired threshold. Returns a list sorted by urgency (soonest critical first).
#[allow(dead_code)]
pub fn memory_forecast(conn: &Connection) -> Result<Vec<ForecastEntry>, rusqlite::Error> {
    let desired_retention = crate::commands::config::get_desired_retention(conn);

    let mut stmt = conn.prepare(
        "SELECT p.topic_id, t.name, s.name, p.ease_factor, p.interval_days, p.last_reviewed
         FROM user_progress p
         JOIN topics t ON t.id = p.topic_id
         JOIN subjects s ON s.id = t.subject_id
         WHERE p.last_reviewed IS NOT NULL",
    )?;

    let mut forecasts: Vec<(i64, String, String, f64, f64)> = stmt
        .query_map([], |r| {
            let topic_id: i64 = r.get(0)?;
            let name: String = r.get(1)?;
            let subject: String = r.get(2)?;
            let ease: f64 = r.get(3)?;
            let interval: i64 = r.get(4)?;
            let last_reviewed: String = r.get(5)?;
            Ok((topic_id, name, subject, ease, interval, last_reviewed))
        })?
        .filter_map(|r| r.ok())
        .map(|(topic_id, name, subject, ease, interval, last_reviewed)| {
            let stability = (interval as f64 * ease / 2.5).max(1.0);
            let elapsed = chrono::NaiveDateTime::parse_from_str(&last_reviewed, "%Y-%m-%d %H:%M:%S")
                .or_else(|_| chrono::NaiveDate::parse_from_str(&last_reviewed, "%Y-%m-%d")
                    .map(|d| d.and_hms_opt(0, 0, 0).unwrap()))
                .map(|dt| {
                    let now = chrono::Local::now().naive_local();
                    (now - dt).num_seconds() as f64 / 86400.0
                })
                .unwrap_or(0.0)
                .max(0.0);

            // Current retention
            let current_r = (1.0 + FSRS_FACTOR * elapsed / stability).powf(-1.0 / FSRS_DECAY);

            // Days until retention drops to desired_retention
            // R(t) = (1 + FSRS_FACTOR * t/S)^(-1/FSRS_DECAY) = desired_retention
            // t = S / FSRS_FACTOR * (desired_retention^(-FSRS_DECAY) - 1)
            let critical_t = stability / FSRS_FACTOR * (desired_retention.powf(-FSRS_DECAY) - 1.0);
            let days_until = (critical_t - elapsed).max(0.0);

            (topic_id, name, subject, days_until, current_r.clamp(0.0, 1.0))
        })
        .collect();

    forecasts.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap_or(std::cmp::Ordering::Equal));
    Ok(forecasts)
}

/// Review load balancer: redistributes next_review dates to avoid review
/// spikes (too many reviews on one day). Looks ahead N days and shifts
/// reviews from overloaded days to adjacent lighter days, respecting a
/// daily cap. This smooths out the review workload without significantly
/// affecting retention (small shifts of 1-2 days have minimal impact).
#[allow(dead_code)]
pub fn balance_review_load(conn: &Connection, days_ahead: usize) -> Result<usize, rusqlite::Error> {
    let daily_cap = get_daily_review_cap(conn);
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    // Count reviews per day in the lookahead window
    let mut day_counts: Vec<(String, i64)> = Vec::new();
    for d in 0..days_ahead {
        let date = (chrono::Local::now() + chrono::Duration::days(d as i64))
            .format("%Y-%m-%d")
            .to_string();
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM user_progress WHERE next_review LIKE ?1 || '%'",
            [&date],
            |r| r.get(0),
        )?;
        day_counts.push((date, count));
    }

    let mut shifted = 0usize;

    // For each overloaded day, shift excess reviews to the next lighter day
    for i in 0..day_counts.len() {
        if day_counts[i].1 <= daily_cap as i64 {
            continue;
        }
        let excess = day_counts[i].1 - daily_cap as i64;
        // Find the next day with room
        let mut target = None;
        for (j, dc) in day_counts.iter().enumerate().skip(i + 1) {
            if dc.1 < daily_cap as i64 {
                target = Some(j);
                break;
            }
        }
        if let Some(t) = target {
            let shift_count = excess.min(daily_cap as i64 - day_counts[t].1);
            if shift_count > 0 {
                // Move `shift_count` reviews from day[i] to day[t]
                let mut stmt = conn.prepare(
                    "SELECT topic_id FROM user_progress WHERE next_review LIKE ?1 || '%' LIMIT ?2",
                )?;
                let topic_ids: Vec<i64> = stmt
                    .query_map(rusqlite::params![&day_counts[i].0, shift_count], |r| r.get(0))?
                    .filter_map(|r| r.ok())
                    .collect();

                for tid in &topic_ids {
                    // Don't shift today's reviews — those are due now
                    if day_counts[i].0 == today {
                        continue;
                    }
                    conn.execute(
                        "UPDATE user_progress SET next_review = ?1 WHERE topic_id = ?2",
                        rusqlite::params![&day_counts[t].0, tid],
                    )?;
                    shifted += 1;
                }
                day_counts[i].1 -= shifted as i64;
                day_counts[t].1 += shifted as i64;
            }
        }
    }

    Ok(shifted)
}

/// Review workload distribution: returns a Vec of (date_string, review_count)
/// for the next N days, useful for displaying review load forecasts.
#[allow(dead_code)]
pub fn review_load_forecast(conn: &Connection, days: usize) -> Result<Vec<(String, i64)>, rusqlite::Error> {
    let mut result = Vec::with_capacity(days);
    for d in 0..days {
        let date = (chrono::Local::now() + chrono::Duration::days(d as i64))
            .format("%Y-%m-%d")
            .to_string();
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM user_progress WHERE next_review <= ?1 || ' 23:59:59'",
            [&date],
            |r| r.get(0),
        )?;
        result.push((date, count));
    }
    Ok(result)
}

/// Compute an adaptive retention target for a topic based on recent performance.
/// Topics where the user consistently scores well get a higher target (longer intervals),
/// while struggling topics get a lower target (reviewed more often).
pub fn adaptive_retention_target(conn: &Connection, topic_id: i64) -> f64 {
    let base = crate::commands::config::get_desired_retention(conn);

    // Get recent quiz scores for this topic (last 10 attempts)
    let scores: Vec<f64> = conn
        .prepare(
            "SELECT score FROM session_log WHERE topic_id = ?1
             AND activity_type IN ('review', 'quiz') AND score IS NOT NULL
             ORDER BY timestamp DESC LIMIT 10",
        )
        .and_then(|mut stmt| {
            stmt.query_map([topic_id], |r| r.get(0))?
                .collect::<Result<Vec<f64>, _>>()
        })
        .unwrap_or_default();

    if scores.len() < 3 {
        return base; // Not enough data, use global setting
    }

    let avg = scores.iter().sum::<f64>() / scores.len() as f64;

    // Trend: compare recent 3 to older scores
    let recent_avg = scores.iter().take(3).sum::<f64>() / 3.0;
    let trend_bonus = if recent_avg > avg { 0.02 } else if recent_avg < avg * 0.8 { -0.03 } else { 0.0 };

    // Scale retention based on performance
    let adjustment = if avg >= 90.0 {
        0.05  // Mastering: push intervals longer
    } else if avg >= 70.0 {
        0.0   // On track
    } else if avg >= 50.0 {
        -0.03 // Struggling: review more often
    } else {
        -0.07 // Really struggling: much more frequent review
    };

    (base + adjustment + trend_bonus).clamp(MIN_ADAPTIVE_RETENTION, MAX_ADAPTIVE_RETENTION)
}

/// Data for the retention report command
#[derive(Debug)]
#[allow(dead_code)]
pub struct RetentionReport {
    pub topic_id: i64,
    pub topic_name: String,
    pub subject_name: String,
    pub current_retention: f64,
    pub target_retention: f64,
    pub stability_days: f64,
    pub days_since_review: i64,
    pub status: RetentionStatus,
}

#[derive(Debug)]
pub enum RetentionStatus {
    Fresh,      // Well above target
    Good,       // At or near target
    Fading,     // Below target but recoverable
    Critical,   // Far below target
}

impl std::fmt::Display for RetentionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RetentionStatus::Fresh => write!(f, "🟢 Fresh"),
            RetentionStatus::Good => write!(f, "🟡 Good"),
            RetentionStatus::Fading => write!(f, "🟠 Fading"),
            RetentionStatus::Critical => write!(f, "🔴 Critical"),
        }
    }
}

/// Generate a retention report for all studied topics.
pub fn retention_report(conn: &Connection) -> Result<Vec<RetentionReport>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT p.topic_id, t.name, s.name,
                COALESCE(p.interval_days, 0),
                COALESCE(p.ease_factor, 2.5),
                CAST(julianday('now') - julianday(COALESCE(p.last_reviewed, '2000-01-01')) AS INTEGER)
         FROM user_progress p
         JOIN topics t ON t.id = p.topic_id
         JOIN subjects s ON s.id = t.subject_id
         WHERE p.attempts > 0
         ORDER BY s.name, t.name",
    )?;

    let mut reports = Vec::new();
    let rows = stmt.query_map([], |r| {
        Ok((
            r.get::<_, i64>(0)?,
            r.get::<_, String>(1)?,
            r.get::<_, String>(2)?,
            r.get::<_, i64>(3)?,
            r.get::<_, f64>(4)?,
            r.get::<_, i64>(5)?,
        ))
    })?;

    for row in rows {
        let (topic_id, topic_name, subject_name, interval, ease, days_since) = row?;

        let ret = retrievability(conn, topic_id);
        let target = adaptive_retention_target(conn, topic_id);
        let stab = stability_half_life(interval, ease);

        let status = if ret >= target + 0.10 {
            RetentionStatus::Fresh
        } else if ret >= target - 0.05 {
            RetentionStatus::Good
        } else if ret >= target - 0.20 {
            RetentionStatus::Fading
        } else {
            RetentionStatus::Critical
        };

        reports.push(RetentionReport {
            topic_id,
            topic_name,
            subject_name,
            current_retention: ret,
            target_retention: target,
            stability_days: stab,
            days_since_review: days_since,
            status,
        });
    }

    Ok(reports)
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

    #[test]
    fn test_balance_review_load_empty() {
        let conn = db::init_memory_db().unwrap();
        let shifted = balance_review_load(&conn, 7).unwrap();
        assert_eq!(shifted, 0, "No reviews to balance when empty");
    }

    #[test]
    fn test_review_load_forecast_returns_days() {
        let conn = db::init_memory_db().unwrap();
        let forecast = review_load_forecast(&conn, 7).unwrap();
        assert_eq!(forecast.len(), 7, "Should return 7 days of forecast");
    }

    #[test]
    fn test_adaptive_retention_default() {
        let conn = db::init_memory_db().unwrap();
        // No data yet — should return default
        let target = adaptive_retention_target(&conn, 1);
        assert!((target - DEFAULT_DESIRED_RETENTION).abs() < 0.01);
    }

    #[test]
    fn test_adaptive_retention_with_high_scores() {
        let conn = db::init_memory_db().unwrap();
        // Simulate high scores
        for _ in 0..5 {
            crate::engine::adaptive::log_activity(&conn, 1, "quiz", Some(95.0)).unwrap();
        }
        let target = adaptive_retention_target(&conn, 1);
        assert!(target > DEFAULT_DESIRED_RETENTION, "High scores should raise target");
        assert!(target <= MAX_ADAPTIVE_RETENTION);
    }

    #[test]
    fn test_adaptive_retention_with_low_scores() {
        let conn = db::init_memory_db().unwrap();
        for _ in 0..5 {
            crate::engine::adaptive::log_activity(&conn, 1, "quiz", Some(40.0)).unwrap();
        }
        let target = adaptive_retention_target(&conn, 1);
        assert!(target < DEFAULT_DESIRED_RETENTION, "Low scores should lower target");
        assert!(target >= MIN_ADAPTIVE_RETENTION);
    }

    #[test]
    fn test_retention_report_empty() {
        let conn = db::init_memory_db().unwrap();
        let reports = retention_report(&conn).unwrap();
        assert!(reports.is_empty());
    }

    #[test]
    fn test_retrieval_practice_bonus_fill_in_blank() {
        // Fill-in-blank correct (quality 3-4) should get +1 bonus
        assert_eq!(retrieval_practice_bonus(3, "fill_in_blank"), 4);
        assert_eq!(retrieval_practice_bonus(4, "fill_in_blank"), 5);
        // Already max or failing: no bonus
        assert_eq!(retrieval_practice_bonus(5, "fill_in_blank"), 5);
        assert_eq!(retrieval_practice_bonus(2, "fill_in_blank"), 2);
    }

    #[test]
    fn test_retrieval_practice_bonus_multiple_choice() {
        // Multiple choice: no bonus
        assert_eq!(retrieval_practice_bonus(4, "multiple_choice"), 4);
        assert_eq!(retrieval_practice_bonus(3, "multiple_choice"), 3);
    }

    #[test]
    fn test_retrieval_practice_bonus_ordering() {
        assert_eq!(retrieval_practice_bonus(3, "ordering"), 4);
        assert_eq!(retrieval_practice_bonus(4, "ordering"), 5);
    }

    #[test]
    fn test_fatigue_no_penalty_under_threshold() {
        let conn = db::init_memory_db().unwrap();
        // No reviews done → no fatigue
        assert_eq!(fatigue_adjusted_quality(&conn, 5), 5);
        assert_eq!(fatigue_adjusted_quality(&conn, 3), 3);
    }

    #[test]
    fn test_fatigue_penalty_over_threshold() {
        let conn = db::init_memory_db().unwrap();
        // Insert many reviews to exceed threshold
        for _ in 0..(FATIGUE_THRESHOLD + 10) {
            conn.execute(
                "INSERT INTO session_log (topic_id, activity_type, timestamp) VALUES (1, 'review', datetime('now'))",
                [],
            ).unwrap();
        }
        let adjusted = fatigue_adjusted_quality(&conn, 5);
        assert!(adjusted < 5, "Quality should be reduced by fatigue, got {}", adjusted);
        assert!(adjusted >= 1, "Quality should never go below 1");
    }

    #[test]
    fn test_fatigue_never_increases_quality() {
        let conn = db::init_memory_db().unwrap();
        let q = fatigue_adjusted_quality(&conn, 2);
        assert!(q <= 2);
    }

    #[test]
    fn test_spacing_bonus_no_history() {
        let conn = db::init_memory_db().unwrap();
        let bonus = spacing_bonus(&conn, 1);
        assert!((bonus - 1.0).abs() < 0.01, "No history should give 1.0 bonus, got {}", bonus);
    }

    #[test]
    fn test_spacing_bonus_multiple_days() {
        let conn = db::init_memory_db().unwrap();
        // Insert activity on 3 different days
        conn.execute(
            "INSERT INTO session_log (topic_id, activity_type, timestamp) VALUES (1, 'review', datetime('now'))",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO session_log (topic_id, activity_type, timestamp) VALUES (1, 'review', datetime('now', '-1 day'))",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO session_log (topic_id, activity_type, timestamp) VALUES (1, 'review', datetime('now', '-3 days'))",
            [],
        ).unwrap();
        let bonus = spacing_bonus(&conn, 1);
        assert!(bonus > 1.0, "Multiple days should give bonus > 1.0, got {}", bonus);
        assert!(bonus <= 1.2, "Bonus should be capped at 1.2, got {}", bonus);
    }

    #[test]
    fn test_fatigue_minimum_quality_1() {
        let conn = db::init_memory_db().unwrap();
        // Many many reviews
        for _ in 0..100 {
            conn.execute(
                "INSERT INTO session_log (topic_id, activity_type, timestamp) VALUES (1, 'review', datetime('now'))",
                [],
            ).unwrap();
        }
        let q = fatigue_adjusted_quality(&conn, 3);
        assert!(q >= 1, "Fatigue should never push quality below 1, got {}", q);
    }
}



/// Mastery level for a topic based on long-term performance indicators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MasteryLevel {
    /// Not yet studied
    New,
    /// Actively learning (short intervals, low ease)
    Learning,
    /// Making progress (medium intervals, moderate ease)
    Developing,
    /// Well-learned (long intervals, high ease, no recent failures)
    Mastered,
    /// Deeply embedded in long-term memory
    Expert,
}

#[allow(dead_code)]
impl MasteryLevel {
    pub fn as_str(&self) -> &str {
        match self {
            MasteryLevel::New => "New",
            MasteryLevel::Learning => "Learning",
            MasteryLevel::Developing => "Developing",
            MasteryLevel::Mastered => "Mastered",
            MasteryLevel::Expert => "Expert",
        }
    }

    pub fn emoji(&self) -> &str {
        match self {
            MasteryLevel::New => "🌱",
            MasteryLevel::Learning => "📖",
            MasteryLevel::Developing => "🌿",
            MasteryLevel::Mastered => "⭐",
            MasteryLevel::Expert => "🏆",
        }
    }
}

#[allow(dead_code)]
/// Assess the mastery level for a topic based on multiple performance signals.
/// Considers: interval length, ease factor, consecutive successes, total attempts,
/// accuracy rate, and leech status.
pub fn assess_mastery(conn: &Connection, topic_id: i64) -> MasteryLevel {
    let data: Option<(f64, i64, i64, i64, i64, f64)> = conn
        .query_row(
            "SELECT ease_factor, interval_days, consecutive_fails, leech_count, attempts,
                    CASE WHEN attempts > 0 THEN CAST(correct AS REAL) / CAST(attempts AS REAL) ELSE 0.0 END
             FROM user_progress WHERE topic_id = ?1",
            [topic_id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?, r.get(5)?)),
        )
        .ok();

    match data {
        None => MasteryLevel::New,
        Some((ease, interval, consec_fails, leech_count, attempts, accuracy)) => {
            // Leech or currently failing → back to Learning
            if leech_count > 0 || consec_fails >= 2 {
                return MasteryLevel::Learning;
            }

            // Expert: long intervals, high ease, high accuracy, sufficient history
            if interval >= 60 && ease >= 2.3 && accuracy >= 0.85 && attempts >= 8 {
                return MasteryLevel::Expert;
            }

            // Mastered: good intervals and ease
            if interval >= 21 && ease >= 2.0 && accuracy >= 0.75 && attempts >= 5 {
                return MasteryLevel::Mastered;
            }

            // Developing: medium intervals or moderate performance
            if interval >= 5 && attempts >= 3 {
                return MasteryLevel::Developing;
            }

            MasteryLevel::Learning
        }
    }
}

#[allow(dead_code)]
/// Get a summary of mastery levels across all studied topics.
pub fn mastery_summary(conn: &Connection) -> Result<Vec<(MasteryLevel, i64)>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT topic_id FROM user_progress",
    )?;
    let ids: Vec<i64> = stmt
        .query_map([], |r| r.get(0))?
        .filter_map(|r| r.ok())
        .collect();

    let mut counts = std::collections::HashMap::new();
    for id in ids {
        let level = assess_mastery(conn, id);
        *counts.entry(level).or_insert(0i64) += 1;
    }

    // Include count of unstudied topics
    let total_topics: i64 = conn.query_row("SELECT COUNT(*) FROM topics", [], |r| r.get(0))?;
    let studied: i64 = counts.values().sum();
    let new_count = total_topics - studied;
    if new_count > 0 {
        counts.insert(MasteryLevel::New, new_count);
    }

    let mut result: Vec<(MasteryLevel, i64)> = counts.into_iter().collect();
    result.sort_by_key(|(level, _)| match level {
        MasteryLevel::New => 0,
        MasteryLevel::Learning => 1,
        MasteryLevel::Developing => 2,
        MasteryLevel::Mastered => 3,
        MasteryLevel::Expert => 4,
    });
    Ok(result)
}

#[allow(dead_code)]
/// Build an optimally ordered review batch that mixes subjects for better retention.
/// Interleaving (mixing topics from different subjects) is shown to improve long-term
/// retention compared to blocked practice (reviewing all items from one subject at a time).
/// This function takes prioritized due topics and reorders them so consecutive reviews
/// alternate between subjects when possible.
pub fn interleaved_review_batch(conn: &Connection) -> Result<Vec<(i64, String, String, f64)>, rusqlite::Error> {
    let cap = get_daily_review_cap(conn);
    let topics = prioritized_due_topics(conn)?;

    if topics.len() <= 2 {
        let mut result = topics;
        result.truncate(cap);
        return Ok(result);
    }

    // Group by subject
    let mut by_subject: std::collections::HashMap<String, Vec<(i64, String, String, f64)>> =
        std::collections::HashMap::new();
    for t in topics {
        by_subject.entry(t.2.clone()).or_default().push(t);
    }

    // Round-robin interleave: pick one from each subject in turn
    let mut queues: Vec<std::collections::VecDeque<(i64, String, String, f64)>> = by_subject
        .into_values()
        .map(|v| v.into_iter().collect())
        .collect();

    // Sort queues by highest priority item first
    queues.sort_by(|a, b| {
        let a_max = a.front().map(|t| t.3).unwrap_or(0.0);
        let b_max = b.front().map(|t| t.3).unwrap_or(0.0);
        b_max.partial_cmp(&a_max).unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut result = Vec::with_capacity(cap);
    let mut empty_count = 0;
    while result.len() < cap && empty_count < queues.len() {
        empty_count = 0;
        for queue in &mut queues {
            if result.len() >= cap {
                break;
            }
            if let Some(item) = queue.pop_front() {
                result.push(item);
            } else {
                empty_count += 1;
            }
        }
    }

    Ok(result)
}

// ── Time-of-Day Performance Tracking ─────────────────────────────────────

/// Record the user's quiz/review quality at a given hour of day (0-23).
/// This builds a profile of when the user performs best, which can be
/// used to give slight quality bonuses during peak hours.
pub fn record_time_of_day_performance(
    conn: &Connection,
    hour: u32,
    quality: u8,
) -> Result<(), rusqlite::Error> {
    let bucket = (hour % 24) as i64;
    let q = quality as f64;

    // Upsert: increment counters and update running average
    conn.execute(
        "INSERT INTO time_of_day_stats (hour_bucket, total_reviews, correct_reviews, avg_quality)
         VALUES (?1, 1, ?2, ?3)
         ON CONFLICT(hour_bucket) DO UPDATE SET
           total_reviews = total_reviews + 1,
           correct_reviews = correct_reviews + ?2,
           avg_quality = (avg_quality * total_reviews + ?3) / (total_reviews + 1)",
        rusqlite::params![bucket, if quality >= 3 { 1 } else { 0 }, q],
    )?;
    Ok(())
}

/// Find the hour bucket where the user has the highest average quality.
/// Returns None if no data exists.
pub fn best_study_hour(conn: &Connection) -> Option<u32> {
    conn.query_row(
        "SELECT hour_bucket FROM time_of_day_stats
         WHERE total_reviews >= 3
         ORDER BY avg_quality DESC, total_reviews DESC
         LIMIT 1",
        [],
        |r| r.get::<_, i64>(0),
    )
    .ok()
    .map(|h| h as u32)
}

/// Calculate a quality multiplier based on the current hour vs historical performance.
#[allow(dead_code)]
/// Returns 1.0 if no data, up to 1.1 during the user's best hours,
/// and down to 0.95 during their weakest hours.
pub fn time_of_day_quality_bonus(conn: &Connection, current_hour: u32) -> f64 {
    let stats: Result<(i64, f64), _> = conn.query_row(
        "SELECT total_reviews, avg_quality FROM time_of_day_stats WHERE hour_bucket = ?1",
        [current_hour as i64],
        |r| Ok((r.get(0)?, r.get(1)?)),
    );

    let (reviews, avg_q) = match stats {
        Ok(s) if s.0 >= 3 => s,
        _ => return 1.0,
    };

    // Get the overall average quality
    let overall_avg: f64 = conn
        .query_row(
            "SELECT COALESCE(SUM(avg_quality * total_reviews) / NULLIF(SUM(total_reviews), 0), 3.0)
             FROM time_of_day_stats WHERE total_reviews >= 3",
            [],
            |r| r.get(0),
        )
        .unwrap_or(3.0);

    if overall_avg <= 0.0 {
        return 1.0;
    }

    // Scale: if this hour is better than average, bonus up to 1.1
    // If worse, penalty down to 0.95
    let ratio = avg_q / overall_avg;
    let _ = reviews; // used for the minimum threshold above
    ratio.clamp(0.95, 1.1)
}

#[cfg(test)]
mod mastery_tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_mastery_new_topic() {
        let conn = db::init_memory_db().unwrap();
        // Topic with no progress → New
        assert_eq!(assess_mastery(&conn, 9999), MasteryLevel::New);
    }

    #[test]
    fn test_mastery_learning() {
        let conn = db::init_memory_db().unwrap();
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days, consecutive_fails, leech_count)
             VALUES (1, 50.0, 2, 1, 2.0, 2, 0, 0)", []
        ).unwrap();
        assert_eq!(assess_mastery(&conn, 1), MasteryLevel::Learning);
    }

    #[test]
    fn test_mastery_developing() {
        let conn = db::init_memory_db().unwrap();
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days, consecutive_fails, leech_count)
             VALUES (1, 70.0, 5, 4, 2.2, 10, 0, 0)", []
        ).unwrap();
        assert_eq!(assess_mastery(&conn, 1), MasteryLevel::Developing);
    }

    #[test]
    fn test_mastery_mastered() {
        let conn = db::init_memory_db().unwrap();
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days, consecutive_fails, leech_count)
             VALUES (1, 85.0, 8, 7, 2.3, 30, 0, 0)", []
        ).unwrap();
        assert_eq!(assess_mastery(&conn, 1), MasteryLevel::Mastered);
    }

    #[test]
    fn test_mastery_expert() {
        let conn = db::init_memory_db().unwrap();
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days, consecutive_fails, leech_count)
             VALUES (1, 95.0, 10, 9, 2.5, 90, 0, 0)", []
        ).unwrap();
        assert_eq!(assess_mastery(&conn, 1), MasteryLevel::Expert);
    }

    #[test]
    fn test_mastery_leech_demotes_to_learning() {
        let conn = db::init_memory_db().unwrap();
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days, consecutive_fails, leech_count)
             VALUES (1, 90.0, 10, 9, 2.5, 60, 0, 1)", []
        ).unwrap();
        // Despite great stats, leech status pulls back to Learning
        assert_eq!(assess_mastery(&conn, 1), MasteryLevel::Learning);
    }

    #[test]
    fn test_mastery_consecutive_fails_demotes() {
        let conn = db::init_memory_db().unwrap();
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days, consecutive_fails, leech_count)
             VALUES (1, 80.0, 8, 6, 2.3, 25, 2, 0)", []
        ).unwrap();
        assert_eq!(assess_mastery(&conn, 1), MasteryLevel::Learning);
    }

    #[test]
    fn test_mastery_summary_empty() {
        let conn = db::init_memory_db().unwrap();
        let summary = mastery_summary(&conn).unwrap();
        // All topics are new
        assert!(!summary.is_empty());
        assert!(summary.iter().any(|(level, count)| *level == MasteryLevel::New && *count > 0));
    }

    #[test]
    fn test_mastery_level_display() {
        assert_eq!(MasteryLevel::New.as_str(), "New");
        assert_eq!(MasteryLevel::Expert.emoji(), "🏆");
        assert_eq!(MasteryLevel::Learning.emoji(), "📖");
    }

    #[test]
    fn test_interleaved_review_batch_empty() {
        let conn = db::init_memory_db().unwrap();
        let batch = interleaved_review_batch(&conn).unwrap();
        assert!(batch.is_empty());
    }

    #[test]
    fn test_interleaving_bonus_no_reviews() {
        let conn = db::init_memory_db().unwrap();
        let bonus = interleaving_bonus(&conn);
        assert!((bonus - 1.0).abs() < f64::EPSILON, "No reviews should give bonus 1.0");
    }

    #[test]
    fn test_interleaving_bonus_single_subject() {
        let conn = db::init_memory_db().unwrap();
        // Log a review for a topic in subject 1
        conn.execute(
            "INSERT INTO session_log (topic_id, activity_type, score) VALUES (1, 'review', 80.0)",
            [],
        ).unwrap();
        let bonus = interleaving_bonus(&conn);
        assert!((bonus - 1.0).abs() < f64::EPSILON, "Single subject should give bonus 1.0");
    }

    #[test]
    fn test_new_card_grace_period() {
        // A brand-new card that fails should get a softer ease penalty
        let conn = db::init_memory_db().unwrap();
        // First attempt: fail with quality 1
        update_spaced_repetition(&conn, 1, 1).unwrap();
        let ease_after_fail: f64 = conn.query_row(
            "SELECT ease_factor FROM user_progress WHERE topic_id = 1", [], |r| r.get(0),
        ).unwrap();
        // With grace period, ease should be 2.5 - 0.08 = 2.42 (not 2.5 - 0.15 = 2.35)
        assert!(ease_after_fail > 2.35, "New card grace period should apply softer penalty, got {}", ease_after_fail);
        assert!(ease_after_fail <= 2.5, "Ease should decrease on failure, got {}", ease_after_fail);
    }

    #[test]
    fn test_established_card_full_penalty() {
        // An established card (many attempts) should get the full ease penalty
        let conn = db::init_memory_db().unwrap();
        // Build up some history
        update_spaced_repetition(&conn, 1, 4).unwrap();
        update_spaced_repetition(&conn, 1, 4).unwrap();
        update_spaced_repetition(&conn, 1, 4).unwrap();
        let ease_before: f64 = conn.query_row(
            "SELECT ease_factor FROM user_progress WHERE topic_id = 1", [], |r| r.get(0),
        ).unwrap();
        // Now fail
        update_spaced_repetition(&conn, 1, 1).unwrap();
        let ease_after: f64 = conn.query_row(
            "SELECT ease_factor FROM user_progress WHERE topic_id = 1", [], |r| r.get(0),
        ).unwrap();
        // Full penalty of 0.15 should be applied
        let penalty = ease_before - ease_after;
        assert!(penalty >= 0.14, "Established card should get full penalty, got {}", penalty);
    }

    #[test]
    fn test_estimate_study_time_no_due() {
        let conn = db::init_memory_db().unwrap();
        let (minutes, due, pace) = estimate_study_time(&conn);
        assert_eq!(due, 0);
        assert_eq!(minutes, 0.0);
        assert!((pace - DEFAULT_MINUTES_PER_REVIEW).abs() < f64::EPSILON);
    }

    #[test]
    fn test_estimate_study_time_with_due() {
        let conn = db::init_memory_db().unwrap();
        // Create a due topic
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days, next_review)
             VALUES (1, 80.0, 3, 2, 2.5, 1, datetime('now', '-1 day'))", []
        ).unwrap();
        let (minutes, due, _pace) = estimate_study_time(&conn);
        assert_eq!(due, 1);
        assert!(minutes > 0.0, "Should estimate positive study time, got {}", minutes);
    }

    #[test]
    fn test_memory_forecast_empty() {
        let conn = db::init_memory_db().unwrap();
        let forecast = memory_forecast(&conn).unwrap();
        assert!(forecast.is_empty());
    }

    #[test]
    fn test_memory_forecast_with_data() {
        let conn = db::init_memory_db().unwrap();
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days, last_reviewed)
             VALUES (1, 100.0, 5, 5, 2.5, 10, datetime('now'))", []
        ).unwrap();
        let forecast = memory_forecast(&conn).unwrap();
        assert_eq!(forecast.len(), 1);
        let (_, _, _, days_until, retention) = &forecast[0];
        assert!(*retention > 0.9, "Just-reviewed should have high retention, got {}", retention);
        assert!(*days_until > 0.0, "Should have positive days until critical, got {}", days_until);
    }

    #[test]
    fn test_stability_decay_not_overdue() {
        let conn = db::init_memory_db().unwrap();
        // No progress → no decay
        let decay = stability_decay_factor(&conn, 9999);
        assert!((decay - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_stability_decay_mildly_overdue() {
        let conn = db::init_memory_db().unwrap();
        // Overdue by 2× interval (within 3× threshold) → no decay
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days, next_review, last_reviewed)
             VALUES (1, 80.0, 5, 4, 2.5, 10, datetime('now', '-20 days'), datetime('now', '-30 days'))", []
        ).unwrap();
        let decay = stability_decay_factor(&conn, 1);
        assert!((decay - 1.0).abs() < f64::EPSILON, "2× overdue should not trigger decay, got {}", decay);
    }

    #[test]
    fn test_stability_decay_severely_overdue() {
        let conn = db::init_memory_db().unwrap();
        // Overdue by 10× interval → significant decay
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days, next_review, last_reviewed)
             VALUES (1, 80.0, 5, 4, 2.5, 10, datetime('now', '-100 days'), datetime('now', '-110 days'))", []
        ).unwrap();
        let decay = stability_decay_factor(&conn, 1);
        assert!(decay < 1.0, "10× overdue should trigger decay, got {}", decay);
        assert!(decay >= 0.3, "Decay should be capped at 0.3, got {}", decay);
    }

    #[test]
    fn test_tod_record_and_best_hour() {
        let conn = db::init_memory_db().unwrap();
        // No data yet
        assert_eq!(best_study_hour(&conn), None);
        // Record some performance (need >= 3 reviews per bucket for best_study_hour)
        record_time_of_day_performance(&conn, 9, 4).unwrap();
        record_time_of_day_performance(&conn, 9, 5).unwrap();
        record_time_of_day_performance(&conn, 9, 5).unwrap();
        record_time_of_day_performance(&conn, 21, 2).unwrap();
        record_time_of_day_performance(&conn, 21, 1).unwrap();
        record_time_of_day_performance(&conn, 21, 2).unwrap();
        let best = best_study_hour(&conn);
        assert_eq!(best, Some(9));
    }

    #[test]
    fn test_tod_quality_bonus() {
        let conn = db::init_memory_db().unwrap();
        // Without data, no bonus
        assert!((time_of_day_quality_bonus(&conn, 10) - 1.0).abs() < f64::EPSILON);
        // With data showing hour 10 is best
        for _ in 0..5 {
            record_time_of_day_performance(&conn, 10, 5).unwrap();
        }
        record_time_of_day_performance(&conn, 22, 2).unwrap();
        let bonus = time_of_day_quality_bonus(&conn, 10);
        assert!(bonus >= 1.0, "Best hour should have bonus >= 1.0, got {}", bonus);
    }

    #[test]
    fn test_memory_forecast_ordering() {
        let conn = db::init_memory_db().unwrap();
        // Topic 1: reviewed recently
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days, last_reviewed)
             VALUES (1, 100.0, 5, 5, 2.5, 10, datetime('now'))", []
        ).unwrap();
        // Topic 2: reviewed a while ago
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days, last_reviewed)
             VALUES (2, 80.0, 3, 2, 2.0, 5, datetime('now', '-8 days'))", []
        ).unwrap();
        let forecast = memory_forecast(&conn).unwrap();
        assert_eq!(forecast.len(), 2);
        // Topic 2 should be more urgent (fewer days until critical)
        assert!(forecast[0].3 <= forecast[1].3, "Forecast should be sorted by urgency");
    }
}

/// Composite retrieval strength metric combining stability, retrievability,
/// momentum, and accuracy into a single 0-100 score. Higher = stronger memory.
/// This gives a more holistic view than any single metric alone:
/// - Retrievability: current memory state (FSRS power-law)
/// - Ease/Stability: long-term durability signal
/// - Momentum: recent performance trajectory
/// - Accuracy: historical success rate
#[allow(dead_code)]
pub fn retrieval_strength(conn: &Connection, topic_id: i64) -> f64 {
    let data: Option<(f64, i64, i64, i64)> = conn
        .query_row(
            "SELECT ease_factor, interval_days, attempts,
                    CASE WHEN attempts > 0 THEN correct ELSE 0 END
             FROM user_progress WHERE topic_id = ?1",
            [topic_id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
        )
        .ok();

    let (ease, _interval, attempts, correct) = match data {
        Some(d) => d,
        None => return 0.0,
    };

    let ret = retrievability(conn, topic_id); // 0.0-1.0
    let momentum = learning_momentum(conn, topic_id); // roughly -20 to +20
    let accuracy = if attempts > 0 { correct as f64 / attempts as f64 } else { 0.0 }; // 0.0-1.0

    // Weight each component
    let retention_score = ret * 40.0;  // 0-40 points
    let ease_score = ((ease - MIN_EASE) / (3.0 - MIN_EASE)).clamp(0.0, 1.0) * 20.0;  // 0-20 points
    let momentum_score = ((momentum + 10.0) / 20.0).clamp(0.0, 1.0) * 20.0;  // 0-20 points
    let accuracy_score = accuracy * 20.0;  // 0-20 points

    (retention_score + ease_score + momentum_score + accuracy_score).clamp(0.0, 100.0)
}

/// Auto-promote a topic's difficulty when the user demonstrates mastery.
/// Criteria: ease_factor >= 2.3, interval >= 14 days, accuracy >= 80%, attempts >= 5.
/// Returns Some(new_difficulty) if promoted, None otherwise.
pub fn auto_promote_difficulty(conn: &Connection, topic_id: i64) -> Option<String> {
    let data: Option<(f64, i64, i64, i64)> = conn
        .query_row(
            "SELECT ease_factor, interval_days, attempts,
                    CASE WHEN attempts > 0 THEN correct ELSE 0 END
             FROM user_progress WHERE topic_id = ?1",
            [topic_id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
        )
        .ok();

    let (ease, interval, attempts, correct) = data?;
    if attempts < 5 || ease < 2.3 || interval < 14 {
        return None;
    }
    let accuracy = correct as f64 / attempts as f64;
    if accuracy < 0.80 {
        return None;
    }

    let current_difficulty: String = conn
        .query_row(
            "SELECT difficulty FROM topics WHERE id = ?1",
            [topic_id],
            |r| r.get(0),
        )
        .unwrap_or_else(|_| "beginner".to_string());

    let new_difficulty = match current_difficulty.as_str() {
        "beginner" => "intermediate",
        "intermediate" => "advanced",
        _ => return None, // Already advanced
    };

    conn.execute(
        "UPDATE topics SET difficulty = ?1 WHERE id = ?2",
        rusqlite::params![new_difficulty, topic_id],
    )
    .ok()?;

    Some(new_difficulty.to_string())
}

/// Calculate learning momentum for a topic — how quickly ease factor is improving.
/// Returns a value where positive = improving, negative = declining, 0 = stable/no data.
/// Looks at the last N reviews and fits a simple linear slope on quality ratings.
pub fn learning_momentum(conn: &Connection, topic_id: i64) -> f64 {
    let scores: Vec<f64> = conn
        .prepare(
            "SELECT COALESCE(score, 0) FROM session_log
             WHERE topic_id = ?1 AND activity_type IN ('quiz', 'review')
             ORDER BY id DESC LIMIT 10",
        )
        .and_then(|mut stmt| {
            stmt.query_map([topic_id], |r| r.get(0))
                .map(|rows| rows.filter_map(|r| r.ok()).collect())
        })
        .unwrap_or_default();

    if scores.len() < 3 {
        return 0.0;
    }

    // Reverse so oldest is first
    let scores: Vec<f64> = scores.into_iter().rev().collect();
    let n = scores.len() as f64;

    // Simple linear regression: slope of scores over time
    let x_mean = (n - 1.0) / 2.0;
    let y_mean: f64 = scores.iter().sum::<f64>() / n;

    let mut numerator = 0.0;
    let mut denominator = 0.0;

    for (i, score) in scores.iter().enumerate() {
        let x = i as f64;
        numerator += (x - x_mean) * (score - y_mean);
        denominator += (x - x_mean) * (x - x_mean);
    }

    if denominator.abs() < f64::EPSILON {
        return 0.0;
    }

    numerator / denominator
}

/// Compute the optimal review window: the hour range when the user performs best.
/// Returns (best_hour, avg_quality) based on time_of_day_stats.
/// If no data exists, returns None.
pub fn optimal_review_window(conn: &Connection) -> Option<(u8, f64)> {
    let result: Result<(i64, f64), _> = conn.query_row(
        "SELECT hour_bucket, avg_quality FROM time_of_day_stats
         WHERE total_reviews >= 3
         ORDER BY avg_quality DESC, total_reviews DESC
         LIMIT 1",
        [],
        |r| Ok((r.get(0)?, r.get(1)?)),
    );
    match result {
        Ok((hour, quality)) => Some((hour as u8, quality)),
        Err(_) => None,
    }
}

/// Record the current hour's review quality for time-of-day tracking.
pub fn record_time_of_day(conn: &Connection, quality: u8) -> Result<(), rusqlite::Error> {
    let hour: i64 = conn.query_row(
        "SELECT CAST(strftime('%H', 'now', 'localtime') AS INTEGER)",
        [],
        |r| r.get(0),
    )?;
    conn.execute(
        "INSERT INTO time_of_day_stats (hour_bucket, total_reviews, correct_reviews, avg_quality)
         VALUES (?1, 1, ?2, ?3)
         ON CONFLICT(hour_bucket) DO UPDATE SET
           total_reviews = total_reviews + 1,
           correct_reviews = correct_reviews + ?2,
           avg_quality = (avg_quality * total_reviews + ?3) / (total_reviews + 1)",
        rusqlite::params![hour, if quality >= 3 { 1 } else { 0 }, quality as f64],
    )?;
    Ok(())
}

/// Consecutive-correct streak for the current session. When a user gets
/// DIFFICULTY_SURGE_THRESHOLD correct answers in a row during a session,
/// the system applies a "difficulty surge": the next interval grows more
/// aggressively (rewarding flow state), and the quiz engine should select
/// harder questions. This keeps engaged learners challenged rather than
/// bored by easy reviews.
#[allow(dead_code)]
const DIFFICULTY_SURGE_THRESHOLD: i64 = 5;
/// Maximum interval multiplier during a difficulty surge.
#[allow(dead_code)]
const DIFFICULTY_SURGE_MULTIPLIER: f64 = 1.25;

/// Check whether the user is on a hot streak (consecutive correct answers)
/// in the current session. Returns the streak length and whether a surge
/// is active.
#[allow(dead_code)]
pub fn difficulty_surge_status(conn: &Connection) -> (i64, bool) {
    // Count consecutive correct reviews from the end of today's session log.
    let scores: Vec<f64> = conn
        .prepare(
            "SELECT COALESCE(score, 0) FROM session_log
             WHERE activity_type IN ('review', 'quiz')
             AND DATE(timestamp) = DATE('now')
             ORDER BY id DESC LIMIT 20",
        )
        .and_then(|mut stmt| {
            stmt.query_map([], |r| r.get(0))
                .map(|rows| rows.filter_map(|r| r.ok()).collect())
        })
        .unwrap_or_default();

    let mut streak: i64 = 0;
    for score in &scores {
        if *score >= 50.0 {
            streak += 1;
        } else {
            break;
        }
    }

    let surge_active = streak >= DIFFICULTY_SURGE_THRESHOLD;
    (streak, surge_active)
}

/// Get the interval multiplier for difficulty surge. Returns > 1.0 when
/// the user is on a hot streak, rewarding flow state with longer intervals.
#[allow(dead_code)]
pub fn difficulty_surge_multiplier(conn: &Connection) -> f64 {
    let (streak, surge_active) = difficulty_surge_status(conn);
    if !surge_active {
        return 1.0;
    }
    // Scale from 1.0 to DIFFICULTY_SURGE_MULTIPLIER over streaks 5-15
    let scale = ((streak - DIFFICULTY_SURGE_THRESHOLD) as f64 / 10.0).min(1.0);
    1.0 + scale * (DIFFICULTY_SURGE_MULTIPLIER - 1.0)
}

/// Determine the recommended question difficulty based on current surge status.
/// During a surge, prefer harder questions to keep the user challenged.
#[allow(dead_code)]
pub fn surge_recommended_difficulty(conn: &Connection) -> &'static str {
    let (streak, surge_active) = difficulty_surge_status(conn);
    if !surge_active {
        return "medium";
    }
    if streak >= DIFFICULTY_SURGE_THRESHOLD + 5 {
        "hard"
    } else {
        "medium"
    }
}

#[cfg(test)]
mod surge_tests {
    use super::*;
    use crate::db;
    use crate::engine::adaptive;

    #[test]
    fn test_no_surge_empty() {
        let conn = db::init_memory_db().unwrap();
        let (streak, active) = difficulty_surge_status(&conn);
        assert_eq!(streak, 0);
        assert!(!active);
    }

    #[test]
    fn test_surge_after_threshold() {
        let conn = db::init_memory_db().unwrap();
        for _ in 0..DIFFICULTY_SURGE_THRESHOLD {
            adaptive::log_activity(&conn, 1, "review", Some(80.0)).unwrap();
        }
        let (streak, active) = difficulty_surge_status(&conn);
        assert_eq!(streak, DIFFICULTY_SURGE_THRESHOLD);
        assert!(active);
    }

    #[test]
    fn test_surge_breaks_on_failure() {
        let conn = db::init_memory_db().unwrap();
        for _ in 0..3 {
            adaptive::log_activity(&conn, 1, "review", Some(80.0)).unwrap();
        }
        adaptive::log_activity(&conn, 1, "review", Some(30.0)).unwrap(); // fail
        for _ in 0..2 {
            adaptive::log_activity(&conn, 1, "review", Some(80.0)).unwrap();
        }
        let (streak, active) = difficulty_surge_status(&conn);
        assert_eq!(streak, 2); // only counts from the last failure
        assert!(!active);
    }

    #[test]
    fn test_surge_multiplier_inactive() {
        let conn = db::init_memory_db().unwrap();
        let mult = difficulty_surge_multiplier(&conn);
        assert!((mult - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_surge_multiplier_active() {
        let conn = db::init_memory_db().unwrap();
        for _ in 0..(DIFFICULTY_SURGE_THRESHOLD + 5) {
            adaptive::log_activity(&conn, 1, "review", Some(90.0)).unwrap();
        }
        let mult = difficulty_surge_multiplier(&conn);
        assert!(mult > 1.0, "Surge multiplier should be > 1.0, got {}", mult);
        assert!(mult <= DIFFICULTY_SURGE_MULTIPLIER, "Surge multiplier should be <= {}, got {}", DIFFICULTY_SURGE_MULTIPLIER, mult);
    }

    #[test]
    fn test_surge_recommended_difficulty() {
        let conn = db::init_memory_db().unwrap();
        assert_eq!(surge_recommended_difficulty(&conn), "medium");
        for _ in 0..(DIFFICULTY_SURGE_THRESHOLD + 6) {
            adaptive::log_activity(&conn, 1, "review", Some(85.0)).unwrap();
        }
        assert_eq!(surge_recommended_difficulty(&conn), "hard");
    }
}

#[cfg(test)]
mod momentum_tests {
    use super::*;
    use crate::db;
    use crate::engine::adaptive;

    #[test]
    fn test_retrieval_strength_no_progress() {
        let conn = db::init_memory_db().unwrap();
        assert!((retrieval_strength(&conn, 9999) - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_retrieval_strength_healthy_topic() {
        let conn = db::init_memory_db().unwrap();
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days, last_reviewed)
             VALUES (1, 90.0, 10, 9, 2.5, 30, datetime('now'))", []
        ).unwrap();
        let strength = retrieval_strength(&conn, 1);
        assert!(strength > 50.0, "Healthy topic should have strength > 50, got {}", strength);
    }

    #[test]
    fn test_retrieval_strength_weak_topic() {
        let conn = db::init_memory_db().unwrap();
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days, last_reviewed)
             VALUES (1, 20.0, 10, 2, 1.4, 1, datetime('now', '-10 days'))", []
        ).unwrap();
        let strength = retrieval_strength(&conn, 1);
        assert!(strength < 50.0, "Weak topic should have strength < 50, got {}", strength);
    }

    #[test]
    fn test_auto_promote_no_progress() {
        let conn = db::init_memory_db().unwrap();
        assert!(auto_promote_difficulty(&conn, 9999).is_none());
    }

    #[test]
    fn test_auto_promote_not_ready() {
        let conn = db::init_memory_db().unwrap();
        // Too few attempts
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days)
             VALUES (1, 90.0, 3, 3, 2.5, 20)", []
        ).unwrap();
        assert!(auto_promote_difficulty(&conn, 1).is_none());
    }

    #[test]
    fn test_auto_promote_beginner_to_intermediate() {
        let conn = db::init_memory_db().unwrap();
        // Meet all criteria
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days)
             VALUES (1, 90.0, 10, 9, 2.5, 21)", []
        ).unwrap();
        let result = auto_promote_difficulty(&conn, 1);
        assert_eq!(result, Some("intermediate".to_string()));
        // Verify DB was updated
        let diff: String = conn.query_row(
            "SELECT difficulty FROM topics WHERE id = 1", [], |r| r.get(0)
        ).unwrap();
        assert_eq!(diff, "intermediate");
    }

    #[test]
    fn test_auto_promote_intermediate_to_advanced() {
        let conn = db::init_memory_db().unwrap();
        // Set topic to intermediate first
        conn.execute("UPDATE topics SET difficulty = 'intermediate' WHERE id = 1", []).unwrap();
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days)
             VALUES (1, 95.0, 12, 11, 2.6, 30)", []
        ).unwrap();
        let result = auto_promote_difficulty(&conn, 1);
        assert_eq!(result, Some("advanced".to_string()));
    }

    #[test]
    fn test_auto_promote_already_advanced() {
        let conn = db::init_memory_db().unwrap();
        conn.execute("UPDATE topics SET difficulty = 'advanced' WHERE id = 1", []).unwrap();
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days)
             VALUES (1, 95.0, 15, 14, 2.8, 60)", []
        ).unwrap();
        assert!(auto_promote_difficulty(&conn, 1).is_none());
    }

    #[test]
    fn test_auto_promote_low_accuracy() {
        let conn = db::init_memory_db().unwrap();
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days)
             VALUES (1, 50.0, 10, 5, 2.5, 21)", []
        ).unwrap();
        assert!(auto_promote_difficulty(&conn, 1).is_none());
    }

    #[test]
    fn test_momentum_no_data() {
        let conn = db::init_memory_db().unwrap();
        assert!((learning_momentum(&conn, 1) - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_momentum_improving() {
        let conn = db::init_memory_db().unwrap();
        // Simulate improving scores
        for score in &[40.0, 50.0, 60.0, 70.0, 80.0] {
            adaptive::log_activity(&conn, 1, "quiz", Some(*score)).unwrap();
        }
        let m = learning_momentum(&conn, 1);
        assert!(m > 0.0, "Improving scores should have positive momentum, got {}", m);
    }

    #[test]
    fn test_momentum_declining() {
        let conn = db::init_memory_db().unwrap();
        // Simulate declining scores
        for score in &[90.0, 80.0, 70.0, 60.0, 50.0] {
            adaptive::log_activity(&conn, 1, "quiz", Some(*score)).unwrap();
        }
        let m = learning_momentum(&conn, 1);
        assert!(m < 0.0, "Declining scores should have negative momentum, got {}", m);
    }

    #[test]
    fn test_momentum_stable() {
        let conn = db::init_memory_db().unwrap();
        for _ in 0..5 {
            adaptive::log_activity(&conn, 1, "quiz", Some(75.0)).unwrap();
        }
        let m = learning_momentum(&conn, 1);
        assert!((m).abs() < 1.0, "Stable scores should have near-zero momentum, got {}", m);
    }
}

#[cfg(test)]
mod sleep_tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_sleep_consolidation_bonus_applied() {
        let conn = db::init_memory_db().unwrap();
        // Review a topic "yesterday" — next review today should get sleep bonus
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days, last_reviewed)
             VALUES (1, 100.0, 5, 5, 2.5, 10, datetime('now', '-1 day'))", []
        ).unwrap();
        update_spaced_repetition(&conn, 1, 4).unwrap();
        let interval_with_sleep: i64 = conn.query_row(
            "SELECT interval_days FROM user_progress WHERE topic_id = 1",
            [], |r| r.get(0)
        ).unwrap();

        // Compare with same-day review (no sleep bonus)
        conn.execute(
            "UPDATE user_progress SET ease_factor = 2.5, interval_days = 10, last_reviewed = datetime('now')
             WHERE topic_id = 1", []
        ).unwrap();
        update_spaced_repetition(&conn, 1, 4).unwrap();
        let interval_same_day: i64 = conn.query_row(
            "SELECT interval_days FROM user_progress WHERE topic_id = 1",
            [], |r| r.get(0)
        ).unwrap();

        // Sleep gap review should produce equal or longer interval
        // (same-day gets the SAME_DAY_REVIEW_FACTOR reduction, sleep gets the bonus)
        assert!(interval_with_sleep >= interval_same_day,
            "Sleep consolidation should give longer interval ({}) than same-day ({})",
            interval_with_sleep, interval_same_day);
    }

    #[test]
    fn test_context_strengthening_no_siblings() {
        let conn = db::init_memory_db().unwrap();
        // No sibling reviews → bonus should be 1.0
        let bonus = context_strengthening(&conn, 1);
        assert!((bonus - 1.0).abs() < f64::EPSILON,
            "No sibling reviews should give no bonus, got {}", bonus);
    }

    #[test]
    fn test_context_strengthening_with_siblings() {
        let conn = db::init_memory_db().unwrap();
        // Get subject_id for topic 1
        let subject_id: i64 = conn.query_row(
            "SELECT subject_id FROM topics WHERE id = 1", [], |r: &rusqlite::Row| r.get(0)
        ).unwrap();
        // Add two more topics in same subject
        conn.execute(
            "INSERT OR IGNORE INTO topics (id, subject_id, name, difficulty, sort_order) VALUES (9990, ?1, 'Sibling A', 'beginner', 90)",
            [subject_id],
        ).unwrap();
        conn.execute(
            "INSERT OR IGNORE INTO topics (id, subject_id, name, difficulty, sort_order) VALUES (9991, ?1, 'Sibling B', 'beginner', 91)",
            [subject_id],
        ).unwrap();
        // Log reviews for siblings
        conn.execute(
            "INSERT INTO session_log (topic_id, activity_type, score, timestamp) VALUES (9990, 'review', 80.0, datetime('now', '-60 seconds'))",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO session_log (topic_id, activity_type, score, timestamp) VALUES (9991, 'review', 85.0, datetime('now', '-30 seconds'))",
            [],
        ).unwrap();
        let bonus = context_strengthening(&conn, 1);
        assert!((bonus - CONTEXT_STRENGTHENING_BONUS).abs() < f64::EPSILON,
            "Two sibling reviews should give context bonus {}, got {}", CONTEXT_STRENGTHENING_BONUS, bonus);
    }

    #[test]
    fn test_recap_query_runs() {
        let conn = db::init_memory_db().unwrap();
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM session_log WHERE timestamp >= datetime('now', '-7 days')",
            [], |r: &rusqlite::Row| r.get(0)
        ).unwrap();
        assert_eq!(count, 0, "Fresh DB should have no session log entries");
    }
}
