use colored::*;
use rusqlite::Connection;
use crate::display;
use crate::engine::{adaptive, quiz as quiz_engine, spaced};
use crate::commands::achievements;

/// Cross-topic challenge: mix questions from multiple subjects for comprehensive review.
pub fn run(conn: &Connection, count: usize) -> Result<(), Box<dyn std::error::Error>> {
    display::print_header("Cross-Topic Challenge");

    // Gather questions from all topics the user has studied (or random if none studied)
    let has_progress: bool = conn.query_row(
        "SELECT COUNT(*) > 0 FROM user_progress", [], |r| r.get(0)
    ).unwrap_or(false);

    let query = if has_progress {
        "SELECT t.id, t.name, s.name
         FROM topics t
         JOIN subjects s ON s.id = t.subject_id
         JOIN user_progress p ON p.topic_id = t.id
         ORDER BY RANDOM()
         LIMIT ?1"
    } else {
        "SELECT t.id, t.name, s.name
         FROM topics t
         JOIN subjects s ON s.id = t.subject_id
         ORDER BY RANDOM()
         LIMIT ?1"
    };
    let mut stmt = conn.prepare(query)?;
    let topic_ids: Vec<(i64, String, String)> = stmt
        .query_map([count as i64], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
        .collect::<Result<Vec<_>, _>>()?;

    if topic_ids.is_empty() {
        display::print_info("No topics available for a challenge.");
        return Ok(());
    }

    // Gather 1 question per topic
    let mut all_questions = Vec::new();
    for (tid, tname, sname) in &topic_ids {
        let qs = quiz_engine::get_questions(conn, *tid, 1)?;
        for q in qs {
            all_questions.push((q, tname.clone(), sname.clone(), *tid));
        }
        if all_questions.len() >= count {
            break;
        }
    }

    if all_questions.is_empty() {
        display::print_info("No quiz questions found for the challenge.");
        return Ok(());
    }

    let total = all_questions.len();
    println!("  {} questions across {} topics | Mixed difficulty\n",
        total.to_string().bold(),
        topic_ids.len().min(total).to_string().bold());

    let mut correct_count = 0;

    for (i, (q, topic_name, subject_name, topic_id)) in all_questions.iter().enumerate() {
        println!("  {} {} [{}→{}]",
            format!("Q{}.", i + 1).bold().bright_cyan(),
            q.question.bold(),
            subject_name.dimmed(),
            topic_name.dimmed()
        );

        match q.question_type.as_str() {
            "true_false" => {
                println!("     {} True", "T)".dimmed());
                println!("     {} False", "F)".dimmed());
            }
            "fill_in_blank" => {
                println!("     {}", "(Type your answer)".dimmed());
            }
            _ => {
                for (j, opt) in q.options.iter().enumerate() {
                    let letter = (b'a' + j as u8) as char;
                    println!("     {} {}", format!("{})", letter).dimmed(), opt);
                }
            }
        }

        println!();
        println!("    {} {}", "Answer:".dimmed(), q.correct_answer.bright_green().bold());
        println!("    {} {}", "Why:".dimmed(), q.explanation);
        if let Some(hint) = &q.hint {
            display::print_hint(hint);
        }
        println!();

        correct_count += 1;
        adaptive::update_progress(conn, *topic_id, true)?;
        display::print_divider();
        println!();
    }

    let score = (correct_count as f64 / total as f64) * 100.0;

    // Log as challenge activity for each topic
    for (_, _, _, topic_id) in &all_questions {
        adaptive::log_activity(conn, *topic_id, "challenge", Some(score))?;
        let quality = if score >= 90.0 { 5 } else if score >= 70.0 { 4 } else { 3 };
        spaced::update_spaced_repetition(conn, *topic_id, quality)?;
    }

    // Check for perfect quiz achievement
    if correct_count == total && total >= 5 {
        if let Ok(Some(name)) = achievements::unlock_perfect_quiz(conn) {
            println!();
            println!("  🏆 {} {}", "ACHIEVEMENT UNLOCKED:".bold().bright_yellow(), name.bold().bright_yellow());
        }
    }

    // Check general achievements
    if let Ok(newly) = achievements::check_achievements(conn) {
        for name in &newly {
            println!("  🏆 {} {}", "ACHIEVEMENT UNLOCKED:".bold().bright_yellow(), name.bold().bright_yellow());
        }
    }

    display::print_header("Challenge Results");
    display::print_progress_bar("Score", correct_count as f64, total as f64);
    println!();

    let subjects_hit: Vec<String> = {
        let mut s: Vec<String> = all_questions.iter().map(|(_, _, sn, _)| sn.clone()).collect();
        s.sort();
        s.dedup();
        s
    };
    println!("  Subjects covered: {}", subjects_hit.join(", ").bright_white());

    if score >= 80.0 {
        display::print_success("Outstanding performance across subjects! 🌟");
    } else if score >= 50.0 {
        display::print_info("Good effort! Keep studying across subjects for breadth. 📖");
    } else {
        display::print_info("Challenges are tough — keep learning and try again! 💪");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use crate::engine::adaptive;

    #[test]
    fn test_challenge_no_progress() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, 5).unwrap();
    }

    #[test]
    fn test_challenge_with_progress() {
        let conn = db::init_memory_db().unwrap();
        adaptive::update_progress(&conn, 1, true).unwrap();
        adaptive::update_progress(&conn, 6, true).unwrap();
        adaptive::update_progress(&conn, 10, true).unwrap();
        run(&conn, 3).unwrap();
    }

    #[test]
    fn test_challenge_large_count() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, 20).unwrap();
    }

    #[test]
    fn test_challenge_logs_activity() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, 3).unwrap();
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM session_log WHERE activity_type = 'challenge'",
            [], |r| r.get(0)
        ).unwrap();
        assert!(count > 0, "Challenge should log activities");
    }
}
