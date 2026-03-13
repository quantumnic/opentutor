use colored::*;
use rusqlite::Connection;
use crate::display;
use crate::commands::achievements;
use crate::engine::{adaptive, quiz as quiz_engine, spaced};

/// Mixed cross-subject quiz: pulls questions from multiple subjects
/// for interleaved practice (proven to improve long-term retention).
pub fn run(conn: &Connection, count: usize, subjects_filter: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    display::print_header("Mixed Review Quiz");

    // Gather candidate topics, optionally filtered by subject
    let topics: Vec<(i64, String, String)> = if let Some(filter) = subjects_filter {
        let pattern = format!("%{}%", filter.to_lowercase());
        let mut stmt = conn.prepare(
            "SELECT t.id, t.name, s.name FROM topics t
             JOIN subjects s ON s.id = t.subject_id
             WHERE LOWER(s.name) LIKE ?1
             ORDER BY RANDOM()"
        )?;
        let rows = stmt.query_map([&pattern], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
            .collect::<Result<Vec<_>, _>>()?;
        rows
    } else {
        let mut stmt = conn.prepare(
            "SELECT t.id, t.name, s.name FROM topics t
             JOIN subjects s ON s.id = t.subject_id
             ORDER BY RANDOM()"
        )?;
        let rows = stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
            .collect::<Result<Vec<_>, _>>()?;
        rows
    };

    if topics.is_empty() {
        display::print_error("No topics found.");
        return Ok(());
    }

    // Collect 1 question from each topic until we have enough, cycling through
    let mut all_questions: Vec<(quiz_engine::QuizQuestion, i64, String, String)> = Vec::new();
    let mut topic_cycle = topics.iter().cycle();
    let mut attempts = 0;
    let max_attempts = topics.len() * 3; // Don't loop forever

    while all_questions.len() < count && attempts < max_attempts {
        let (topic_id, topic_name, subject_name) = topic_cycle.next().unwrap();
        let qs = quiz_engine::get_questions(conn, *topic_id, 1)?;
        for q in qs {
            // Avoid duplicates
            if !all_questions.iter().any(|(existing, _, _, _)| existing.id == q.id) {
                all_questions.push((q, *topic_id, topic_name.clone(), subject_name.clone()));
                if all_questions.len() >= count {
                    break;
                }
            }
        }
        attempts += 1;
    }

    if all_questions.is_empty() {
        display::print_info("No quiz questions available across topics.");
        return Ok(());
    }

    let total = all_questions.len();
    println!("  {} questions across {} subjects | Interleaved for deeper learning\n",
        total.to_string().bold(),
        {
            let mut subjects: Vec<&str> = all_questions.iter().map(|(_, _, _, s)| s.as_str()).collect();
            subjects.sort();
            subjects.dedup();
            subjects.len()
        }.to_string().bold().bright_cyan());

    let mut correct_count = 0;
    let mut subject_scores: std::collections::HashMap<String, (u32, u32)> = std::collections::HashMap::new();

    for (i, (q, topic_id, topic_name, subject_name)) in all_questions.iter().enumerate() {
        println!("  {} {} {}",
            format!("Q{}.", i + 1).bold().bright_cyan(),
            format!("[{}]", subject_name).dimmed(),
            q.question.bold());

        match q.question_type.as_str() {
            "true_false" => {
                for opt in &q.options {
                    println!("     • {}", opt);
                }
            }
            "ordering" => {
                println!("     {}", "(Put in correct order)".dimmed());
                for (j, opt) in q.options.iter().enumerate() {
                    println!("     {} {}", format!("{}.", j + 1).dimmed(), opt);
                }
            }
            _ => {
                for (j, opt) in q.options.iter().enumerate() {
                    let letter = (b'a' + j as u8) as char;
                    println!("     {} {}", format!("{})", letter).dimmed(), opt);
                }
            }
        }

        println!();
        println!("    {} {} ({})",
            "Answer:".dimmed(),
            q.correct_answer.bright_green().bold(),
            topic_name.dimmed());
        println!("    {} {}", "Why:".dimmed(), q.explanation);
        println!();
        display::print_divider();
        println!();

        correct_count += 1;
        adaptive::update_progress(conn, *topic_id, true)?;

        let entry = subject_scores.entry(subject_name.clone()).or_insert((0, 0));
        entry.0 += 1; // correct
        entry.1 += 1; // total
    }

    // Update spaced repetition for all touched topics
    let mut seen_topics = std::collections::HashSet::new();
    for (_, topic_id, _, _) in &all_questions {
        if seen_topics.insert(topic_id) {
            spaced::update_spaced_repetition(conn, *topic_id, 4)?;
            adaptive::log_activity(conn, *topic_id, "mix_review", Some(80.0))?;
        }
    }

    // Summary with per-subject breakdown
    display::print_header("Mixed Quiz Results");
    display::print_progress_bar("Overall", correct_count as f64, total as f64);
    println!();

    println!("  {}", "Per-subject breakdown:".bold());
    let mut sorted_subjects: Vec<_> = subject_scores.iter().collect();
    sorted_subjects.sort_by_key(|(name, _)| (*name).clone());
    for (subject, (correct, total)) in sorted_subjects {
        let pct = if *total > 0 { *correct as f64 / *total as f64 * 100.0 } else { 0.0 };
        let indicator = if pct >= 80.0 { "🟢" } else if pct >= 50.0 { "🟡" } else { "🔴" };
        println!("    {} {} — {}/{} ({}%)", indicator, subject, correct, total, pct as u32);
    }
    println!();

    display::print_success("Interleaved practice builds stronger, more flexible knowledge! 🧩");

    // Check achievements
    if let Ok(newly) = achievements::check_achievements(conn) {
        for name in &newly {
            println!("  🏆 {} {}", "ACHIEVEMENT UNLOCKED:".bold().bright_yellow(), name.bold().bright_yellow());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_mix_default() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, 5, None).unwrap();
    }

    #[test]
    fn test_mix_filtered() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, 3, Some("math")).unwrap();
    }

    #[test]
    fn test_mix_large_count() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, 20, None).unwrap();
    }

    #[test]
    fn test_mix_no_match() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, 5, Some("nonexistent_subject_xyz")).unwrap();
    }

    #[test]
    fn test_mix_single_subject() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, 3, Some("science")).unwrap();
    }
}
