use colored::*;
use rusqlite::Connection;
use crate::display;
use crate::engine::{adaptive, quiz as quiz_engine, spaced};

/// Daily learning plan: review due topics, then suggest new material.
pub fn run(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    let streak = spaced::calculate_streak(conn);

    display::print_header("Your Daily Learning Plan");

    // Show streak
    if streak > 0 {
        let flame = "🔥".repeat(streak.min(7) as usize);
        println!("  {} {} day streak! Keep it going!\n",
            flame, streak.to_string().bold().bright_yellow());
    } else {
        println!("  Start a new streak today! 💪\n");
    }

    // Phase 1: Due reviews (prioritized by urgency)
    let mut due_stmt = conn.prepare(
        "SELECT t.id, t.name, s.name, p.ease_factor, p.interval_days
         FROM user_progress p
         JOIN topics t ON t.id = p.topic_id
         JOIN subjects s ON s.id = t.subject_id
         WHERE p.next_review IS NOT NULL AND p.next_review <= datetime('now')
         ORDER BY p.next_review ASC
         LIMIT 5",
    )?;

    let due_topics: Vec<(i64, String, String, f64, i64)> = due_stmt
        .query_map([], |r| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    let has_reviews = !due_topics.is_empty();

    if has_reviews {
        display::print_section(&format!("📋 Review ({} topics due)", due_topics.len()));
        println!();

        for (topic_id, topic_name, subject_name, ease, interval) in &due_topics {
            let is_lapsed = spaced::is_card_lapsed(conn, *topic_id);
            let urgency = if is_lapsed {
                "⚠️ Lapsed".bright_red().to_string()
            } else if *interval < 7 {
                "🔨 Fragile".yellow().to_string()
            } else {
                "🔄 Review".normal().to_string()
            };

            println!("    {} {} ({}) — ease {:.1}",
                urgency, topic_name.bold(), subject_name.dimmed(), ease);

            // Show 2 quick review questions
            let questions = quiz_engine::get_questions(conn, *topic_id, 2)?;
            for (i, q) in questions.iter().enumerate() {
                println!("      {}. {}", i + 1, q.question);
                println!("         → {}", q.correct_answer.bright_green());
            }

            // Update progress
            let quality = if is_lapsed { 3 } else { 4 };
            spaced::update_spaced_repetition(conn, *topic_id, quality)?;
            adaptive::log_activity(conn, *topic_id, "review", Some(if is_lapsed { 60.0 } else { 80.0 }))?;
            println!();
        }
    } else {
        display::print_section("📋 Review");
        display::print_success("All caught up! No reviews due. 🎉");
        println!();
    }

    // Phase 2: Suggest new material
    display::print_section("🆕 Discover Something New");
    println!();

    // Find topics the user hasn't studied yet, from subjects they've touched
    let mut new_from_known = conn.prepare(
        "SELECT t.id, t.name, s.name FROM topics t
         JOIN subjects s ON s.id = t.subject_id
         WHERE t.subject_id IN (
             SELECT DISTINCT t2.subject_id FROM user_progress p
             JOIN topics t2 ON t2.id = p.topic_id
         )
         AND t.id NOT IN (SELECT topic_id FROM user_progress)
         ORDER BY t.sort_order ASC
         LIMIT 2"
    )?;
    let suggestions_known: Vec<(i64, String, String)> = new_from_known
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
        .collect::<Result<Vec<_>, _>>()?;

    // Also suggest from completely new subjects
    let mut new_subject = conn.prepare(
        "SELECT t.id, t.name, s.name FROM topics t
         JOIN subjects s ON s.id = t.subject_id
         WHERE t.subject_id NOT IN (
             SELECT DISTINCT t2.subject_id FROM user_progress p
             JOIN topics t2 ON t2.id = p.topic_id
         )
         AND t.sort_order = 1
         ORDER BY RANDOM()
         LIMIT 1"
    )?;
    let suggestion_new: Vec<(i64, String, String)> = new_subject
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
        .collect::<Result<Vec<_>, _>>()?;

    let all_suggestions: Vec<&(i64, String, String)> = suggestions_known.iter()
        .chain(suggestion_new.iter())
        .collect();

    if all_suggestions.is_empty() {
        display::print_success("You've explored all available topics! You're a learning machine! 🏆");
    } else {
        for (topic_id, topic_name, subject_name) in &all_suggestions {
            // Get a lesson preview
            let preview: Option<String> = conn.query_row(
                "SELECT content FROM lessons WHERE topic_id = ?1 ORDER BY sort_order LIMIT 1",
                [topic_id],
                |r| r.get(0),
            ).ok();

            println!("    🌟 {} ({})", topic_name.bold().bright_white(), subject_name.dimmed());
            if let Some(content) = &preview {
                let first_lines: String = content.lines().take(3).collect::<Vec<_>>().join("\n");
                let truncated = if first_lines.len() > 200 {
                    format!("{}...", &first_lines[..200])
                } else {
                    first_lines
                };
                for line in truncated.lines() {
                    println!("       {}", line.dimmed());
                }
            }
            println!("       → {}", format!("opentutor learn {}", subject_name.to_lowercase()).bright_cyan());
            println!();
        }
    }

    // Phase 2.5: Leech warnings
    let leeches = spaced::get_leeches(conn).unwrap_or_default();
    if !leeches.is_empty() {
        display::print_section("🩹 Leech Cards");
        println!("    These topics need extra attention — try re-reading the lessons:\n");
        for (_id, name, subject, count) in &leeches {
            println!("    🩹 {} ({}) — {} leeches",
                name.bold().bright_yellow(), subject.dimmed(), count);
        }
        println!();
        display::print_hint("Use 'opentutor explain <concept>' for alternative explanations.");
        println!();
    }

    // Phase 3: Quick stats
    display::print_divider();
    let total_studied: i64 = conn.query_row(
        "SELECT COUNT(*) FROM user_progress", [], |r| r.get(0)
    )?;
    let total_topics: i64 = conn.query_row(
        "SELECT COUNT(*) FROM topics", [], |r| r.get(0)
    )?;
    let total_due: i64 = spaced::count_due_topics(conn).unwrap_or(0);

    println!();
    display::print_progress_bar("Overall progress", total_studied as f64, total_topics as f64);
    if total_due > 0 {
        println!("  {} topics still due after this session",
            total_due.to_string().bright_yellow());
    }

    println!();
    display::print_success("Great job showing up today! Consistency beats intensity. 📅");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_daily_empty() {
        let conn = db::init_memory_db().unwrap();
        run(&conn).unwrap();
    }

    #[test]
    fn test_daily_with_progress() {
        let conn = db::init_memory_db().unwrap();
        adaptive::update_progress(&conn, 1, true).unwrap();
        adaptive::log_activity(&conn, 1, "learn", Some(100.0)).unwrap();
        spaced::update_spaced_repetition(&conn, 1, 4).unwrap();
        // Force review to be due
        conn.execute(
            "UPDATE user_progress SET next_review = datetime('now', '-1 day') WHERE topic_id = 1",
            [],
        ).unwrap();
        run(&conn).unwrap();
    }

    #[test]
    fn test_daily_suggests_new_topics() {
        let conn = db::init_memory_db().unwrap();
        // Study one topic
        adaptive::update_progress(&conn, 1, true).unwrap();
        adaptive::log_activity(&conn, 1, "learn", Some(100.0)).unwrap();
        run(&conn).unwrap();
    }
}
