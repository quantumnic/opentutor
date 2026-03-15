use colored::*;
use rusqlite::Connection;
use crate::display;
use crate::engine::{quiz as quiz_engine, spaced};

/// Cram mode: intensive review of the lowest-retention topics.
/// Unlike normal review (which follows spaced repetition schedules),
/// cram ignores scheduling and focuses on the material you're most
/// likely to forget — perfect for last-minute exam prep.
pub fn run(conn: &Connection, count: usize) -> Result<(), Box<dyn std::error::Error>> {
    display::print_header("Cram Mode — Lowest Retention First");
    println!("  {}\n", "Ignoring spaced repetition schedule — focusing on weakest material.".dimmed());

    // Get all topics with progress, sorted by retrievability (lowest first)
    let mut stmt = conn.prepare(
        "SELECT t.id, t.name, s.name
         FROM topics t
         JOIN subjects s ON t.subject_id = s.id
         JOIN user_progress up ON up.topic_id = t.id
         WHERE up.attempts > 0
         ORDER BY up.score ASC, up.ease_factor ASC
         LIMIT 50",
    )?;

    let topics: Vec<(i64, String, String)> = stmt
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
        .filter_map(|r| r.ok())
        .collect();

    if topics.is_empty() {
        // Fall back: if no progress yet, pick topics with lowest difficulty
        display::print_info("No study history yet. Use 'review' or 'quiz' first to build a history, then cram!");
        return Ok(());
    }

    // Score each topic by retrievability (lower = more urgent to cram)
    let mut scored: Vec<(i64, String, String, f64)> = topics
        .into_iter()
        .map(|(id, name, subject)| {
            let ret = spaced::retrievability(conn, id);
            (id, name, subject, ret)
        })
        .collect();

    // Sort by retrievability ascending (lowest retention first)
    scored.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap_or(std::cmp::Ordering::Equal));

    let cram_count = count.min(scored.len());
    println!("  Cramming {} topics (out of {} studied):\n", cram_count.to_string().bold(), scored.len());

    let mut total_questions = 0;

    for (i, (topic_id, topic_name, subject_name, retention)) in scored.iter().take(cram_count).enumerate() {
        let ret_pct = (retention * 100.0) as u32;
        let ret_color = if ret_pct < 40 {
            format!("{}%", ret_pct).bright_red()
        } else if ret_pct < 70 {
            format!("{}%", ret_pct).bright_yellow()
        } else {
            format!("{}%", ret_pct).bright_green()
        };

        println!("  {} {} {} (retention: {})",
            format!("{}.", i + 1).bold().bright_cyan(),
            topic_name.bold(),
            format!("[{}]", subject_name).dimmed(),
            ret_color,
        );

        // Show 2 quiz questions per topic for quick review
        let questions = quiz_engine::get_questions(conn, *topic_id, 2)?;
        for q in &questions {
            println!("     {} {}", "Q:".dimmed(), q.question);
            println!("     {} {}", "A:".bright_green(), q.correct_answer);
            if let Some(hint) = &q.hint {
                println!("     {} {}", "💡".dimmed(), hint.dimmed());
            }
            println!();
            total_questions += 1;
        }
    }

    println!("  {}", "─".repeat(50).dimmed());
    println!("  Crammed {} questions across {} topics.", 
        total_questions.to_string().bold(),
        cram_count.to_string().bold());
    println!("  {}", "Tip: Follow up with 'opentutor review' for proper spaced repetition.".dimmed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_cram_no_progress() {
        let conn = db::init_memory_db().unwrap();
        // Should not crash when no progress exists
        let result = run(&conn, 5);
        assert!(result.is_ok());
    }

    #[test]
    fn test_cram_with_progress() {
        let conn = db::init_memory_db().unwrap();
        // Simulate some progress
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days)
             VALUES (1, 0.3, 5, 1, 1.5, 1)",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days)
             VALUES (2, 0.8, 10, 8, 2.5, 7)",
            [],
        ).unwrap();
        let result = run(&conn, 5);
        assert!(result.is_ok());
    }
}
