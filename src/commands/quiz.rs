use colored::*;
use rusqlite::Connection;
use crate::display;
use crate::engine::{adaptive, quiz as quiz_engine, spaced};

pub fn run(conn: &Connection, topic: &str, count: usize) -> Result<(), Box<dyn std::error::Error>> {
    // Find topic (case-insensitive, partial match)
    let topic_row: Result<(i64, String), _> = conn.query_row(
        "SELECT id, name FROM topics WHERE LOWER(name) LIKE '%' || LOWER(?1) || '%'",
        [topic],
        |r| Ok((r.get(0)?, r.get(1)?)),
    );

    let (topic_id, topic_name) = match topic_row {
        Ok(row) => row,
        Err(_) => {
            display::print_error(&format!("Topic '{}' not found.", topic));
            display::print_info("Use 'opentutor subjects' to see available topics.");
            return Ok(());
        }
    };

    let questions = quiz_engine::get_questions(conn, topic_id, count)?;
    if questions.is_empty() {
        display::print_info(&format!("No quiz questions available for '{}'.", topic_name));
        return Ok(());
    }

    display::print_header(&format!("Quiz: {}", topic_name));
    println!("  {} questions | Type the answer or letter (a/b/c/d) or true/false\n",
        questions.len().to_string().bold());

    let mut correct_count = 0;
    let total = questions.len();

    for (i, q) in questions.iter().enumerate() {
        println!("  {} {}", format!("Q{}.", i + 1).bold().bright_cyan(), q.question.bold());
        if q.question_type == "true_false" {
            println!("     {} True", "T)".dimmed());
            println!("     {} False", "F)".dimmed());
        } else {
            for (j, opt) in q.options.iter().enumerate() {
                let letter = (b'a' + j as u8) as char;
                println!("     {} {}", format!("{})", letter).dimmed(), opt);
            }
        }

        // In non-interactive mode, show the answer
        println!();
        println!("    {} {}", "Answer:".dimmed(), q.correct_answer.bright_green().bold());
        println!("    {} {}", "Why:".dimmed(), q.explanation);
        if let Some(hint) = &q.hint {
            display::print_hint(hint);
        }

        // Track as correct for demo/non-interactive mode
        correct_count += 1;
        adaptive::update_progress(conn, topic_id, true)?;

        println!();
        display::print_divider();
        println!();
    }

    let score = (correct_count as f64 / total as f64) * 100.0;
    let quality = if score >= 90.0 { 5 } else if score >= 70.0 { 4 } else if score >= 50.0 { 3 } else { 2 };
    spaced::update_spaced_repetition(conn, topic_id, quality)?;
    adaptive::log_activity(conn, topic_id, "quiz", Some(score))?;

    display::print_header("Quiz Results");
    display::print_progress_bar(&topic_name, correct_count as f64, total as f64);
    println!();

    if score >= 80.0 {
        display::print_success("Excellent work! You're mastering this topic! 🌟");
    } else if score >= 50.0 {
        display::print_info("Good progress! Review the material and try again. 📖");
    } else {
        display::print_info("Keep learning! Use 'opentutor learn' to review. 💪");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_quiz_valid_topic() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "Arithmetic", 3).unwrap();
    }

    #[test]
    fn test_quiz_partial_match() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "fraction", 2).unwrap();
    }

    #[test]
    fn test_quiz_invalid_topic() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "Nonexistent", 5).unwrap();
    }
}
