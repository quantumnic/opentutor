use colored::*;
use rusqlite::Connection;
use crate::display;
use crate::engine::spaced;

/// Knowledge gap analysis: identifies weak areas and recommends focused study.
pub fn run(conn: &Connection, limit: usize) -> Result<(), Box<dyn std::error::Error>> {
    display::print_header("Knowledge Gap Diagnosis");

    // 1. Find topics with low retrieval strength (combination of retention + momentum + ease)
    let mut stmt = conn.prepare(
        "SELECT p.topic_id, t.name, s.name,
                p.ease_factor, p.interval_days, p.attempts, p.correct,
                p.consecutive_fails, p.leech_count
         FROM user_progress p
         JOIN topics t ON t.id = p.topic_id
         JOIN subjects s ON s.id = t.subject_id
         WHERE p.attempts > 0
         ORDER BY p.ease_factor ASC, p.interval_days ASC",
    )?;

    struct GapEntry {
        topic_name: String,
        subject_name: String,
        retrieval_strength: f64,
        retention: f64,
        momentum: f64,
        accuracy: f64,
        is_leech: bool,
        diagnosis: String,
    }

    let rows: Vec<(i64, String, String, f64, i64, i64, i64, i64, i64)> = stmt
        .query_map([], |r| {
            Ok((
                r.get(0)?, r.get(1)?, r.get(2)?,
                r.get(3)?, r.get(4)?, r.get(5)?,
                r.get(6)?, r.get(7)?, r.get(8)?,
            ))
        })?
        .filter_map(|r| r.ok())
        .collect();

    if rows.is_empty() {
        display::print_info("No study data yet. Start learning with 'opentutor learn <subject>' to build your profile.");
        return Ok(());
    }

    let mut gaps: Vec<GapEntry> = Vec::new();

    for (topic_id, topic_name, subject_name, ease, interval, attempts, correct, consec_fails, leech_count) in &rows {
        let retention = spaced::estimate_retention(conn, *topic_id);
        let momentum = spaced::learning_momentum(conn, *topic_id);
        let accuracy = if *attempts > 0 { *correct as f64 / *attempts as f64 * 100.0 } else { 0.0 };
        let is_leech = *leech_count > 0;

        // Retrieval strength: composite metric
        // Low retention + low ease + negative momentum + low accuracy = weak
        let retention_score = retention * 40.0;  // 0-40 points
        let ease_score = ((*ease - 1.3) / (3.0 - 1.3)).clamp(0.0, 1.0) * 20.0;  // 0-20 points
        let momentum_score = ((momentum + 10.0) / 20.0).clamp(0.0, 1.0) * 20.0;  // 0-20 points
        let accuracy_score = accuracy / 100.0 * 20.0;  // 0-20 points

        let retrieval_strength = retention_score + ease_score + momentum_score + accuracy_score;

        // Diagnose the specific problem
        let diagnosis = if is_leech {
            "🔴 Leech — repeatedly failed, needs different approach".to_string()
        } else if *consec_fails >= 3 {
            "🔴 Struggling — consecutive failures, review fundamentals".to_string()
        } else if retention < 0.4 {
            "🟠 Fading fast — memory critically low, review urgently".to_string()
        } else if momentum < -5.0 {
            "🟠 Declining — recent scores dropping, needs reinforcement".to_string()
        } else if accuracy < 50.0 && *attempts >= 3 {
            "🟡 Weak foundation — low accuracy suggests gaps in understanding".to_string()
        } else if *ease < 1.8 {
            "🟡 Hard going — low ease factor indicates persistent difficulty".to_string()
        } else if *interval < 3 && *attempts >= 5 {
            "🟡 Not consolidating — many reviews but short intervals".to_string()
        } else {
            continue; // Skip healthy topics
        };

        gaps.push(GapEntry {
            topic_name: topic_name.clone(),
            subject_name: subject_name.clone(),
            retrieval_strength,
            retention,
            momentum,
            accuracy,
            is_leech,
            diagnosis,
        });
    }

    if gaps.is_empty() {
        println!("  {} No significant knowledge gaps detected!", "✨".bright_green());
        println!("  Your study habits are on track. Keep it up!\n");
        return Ok(());
    }

    // Sort by retrieval strength (weakest first)
    gaps.sort_by(|a, b| a.retrieval_strength.partial_cmp(&b.retrieval_strength).unwrap_or(std::cmp::Ordering::Equal));
    gaps.truncate(limit);

    println!("  Found {} knowledge gaps to address:\n", gaps.len().to_string().bold().bright_yellow());

    for (i, gap) in gaps.iter().enumerate() {
        let strength_bar = format_strength_bar(gap.retrieval_strength);
        println!("  {}. {} ({})",
            format!("{}", i + 1).bold().bright_cyan(),
            gap.topic_name.bold(),
            gap.subject_name.dimmed(),
        );
        println!("     Strength: {} {:.0}/100", strength_bar, gap.retrieval_strength);
        println!("     Retention: {:.0}% | Accuracy: {:.0}% | Momentum: {:.1}",
            gap.retention * 100.0, gap.accuracy, gap.momentum);
        if gap.is_leech {
            println!("     {} Leech topic — try a different study approach", "⚠".bright_red());
        }
        println!("     {}", gap.diagnosis);
        println!();
    }

    // Summary recommendations
    display::print_header("Recommendations");
    let leech_count = gaps.iter().filter(|g| g.is_leech).count();
    let fading_count = gaps.iter().filter(|g| g.retention < 0.4).count();
    let declining_count = gaps.iter().filter(|g| g.momentum < -5.0).count();

    if leech_count > 0 {
        println!("  {} {} leech topic(s) — try explaining these concepts in your own words",
            "📌".bright_red(), leech_count);
        println!("     or approach from a different angle (analogies, visual aids).\n");
    }
    if fading_count > 0 {
        println!("  {} {} topic(s) with critically low retention — prioritize these in your next review.",
            "⏰".bright_yellow(), fading_count);
        println!("     Use 'opentutor cram' for intensive refresher.\n");
    }
    if declining_count > 0 {
        println!("  {} {} topic(s) with declining momentum — recent scores are dropping.",
            "📉".bright_yellow(), declining_count);
        println!("     Revisit the lesson material before quizzing again.\n");
    }

    // Suggest subjects needing most attention
    let mut subject_gaps: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
    for gap in &gaps {
        *subject_gaps.entry(&gap.subject_name).or_insert(0) += 1;
    }
    let mut subject_list: Vec<(&&str, &usize)> = subject_gaps.iter().collect();
    subject_list.sort_by(|a, b| b.1.cmp(a.1));

    if !subject_list.is_empty() {
        println!("  {} Subjects needing most attention:", "🎯".bright_cyan());
        for (subject, count) in subject_list.iter().take(3) {
            println!("     • {} ({} weak topics)", subject.bold(), count);
        }
        println!();
    }

    Ok(())
}

fn format_strength_bar(strength: f64) -> String {
    let filled = (strength / 10.0).round() as usize;
    let empty = 10usize.saturating_sub(filled);
    let color = if strength < 30.0 { "red" } else if strength < 60.0 { "yellow" } else { "green" };
    let bar = format!("[{}{}]", "█".repeat(filled), "░".repeat(empty));
    match color {
        "red" => bar.bright_red().to_string(),
        "yellow" => bar.bright_yellow().to_string(),
        _ => bar.bright_green().to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_diagnose_empty_db() {
        let conn = db::init_memory_db().unwrap();
        // Should not error on empty DB
        run(&conn, 10).unwrap();
    }

    #[test]
    fn test_diagnose_with_progress() {
        let conn = db::init_memory_db().unwrap();
        // Add some progress data
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days, last_reviewed, consecutive_fails, leech_count)
             VALUES (1, 30.0, 5, 1, 1.5, 1, datetime('now', '-3 days'), 3, 0)", []
        ).unwrap();
        run(&conn, 10).unwrap();
    }

    #[test]
    fn test_diagnose_leech_detected() {
        let conn = db::init_memory_db().unwrap();
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days, last_reviewed, consecutive_fails, leech_count)
             VALUES (1, 20.0, 8, 2, 1.3, 1, datetime('now', '-1 day'), 4, 1)", []
        ).unwrap();
        run(&conn, 10).unwrap();
    }

    #[test]
    fn test_diagnose_no_gaps() {
        let conn = db::init_memory_db().unwrap();
        // Add healthy progress — high ease, long interval, good accuracy
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days, last_reviewed, consecutive_fails, leech_count)
             VALUES (1, 95.0, 10, 9, 2.8, 30, datetime('now'), 0, 0)", []
        ).unwrap();
        run(&conn, 10).unwrap();
    }

    #[test]
    fn test_strength_bar_format() {
        let bar = format_strength_bar(50.0);
        assert!(bar.contains("█"), "Bar should contain filled blocks");
        assert!(bar.contains("░"), "Bar should contain empty blocks");
    }
}
