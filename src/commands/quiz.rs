use colored::*;
use rusqlite::Connection;
use crate::display;
use crate::commands::achievements;
use crate::engine::{adaptive, quiz as quiz_engine, spaced};

pub fn run(conn: &Connection, topic: &str, count: usize, difficulty: Option<&str>, timed: bool) -> Result<(), Box<dyn std::error::Error>> {
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

    let questions = quiz_engine::get_questions_filtered(conn, topic_id, count, difficulty)?;
    if questions.is_empty() {
        display::print_info(&format!("No quiz questions available for '{}'.", topic_name));
        return Ok(());
    }

    display::print_header(&format!("Quiz: {}", topic_name));
    if timed {
        println!("  {} questions | {} | Type the answer or letter (a/b/c/d) or true/false\n",
            questions.len().to_string().bold(), "⏱ TIMED MODE".bold().bright_yellow());
    } else {
        println!("  {} questions | Type the answer or letter (a/b/c/d) or true/false\n",
            questions.len().to_string().bold());
    }

    let mut correct_count = 0;
    let mut streak = 0;      // Consecutive correct answers in this quiz
    let mut best_streak = 0; // Best streak in this quiz
    let total = questions.len();

    for (i, q) in questions.iter().enumerate() {
        println!("  {} {}", format!("Q{}.", i + 1).bold().bright_cyan(), q.question.bold());
        match q.question_type.as_str() {
            "true_false" => {
                println!("     {} True", "T)".dimmed());
                println!("     {} False", "F)".dimmed());
            }
            "fill_in_blank" => {
                println!("     {}", "(Type your answer)".dimmed());
            }
            "ordering" => {
                println!("     {}", "(Put these in the correct order)".dimmed());
                for (j, opt) in q.options.iter().enumerate() {
                    println!("     {} {}", format!("{}.", j + 1).dimmed(), opt);
                }
            }
            "select_all" => {
                println!("     {}", "(Select ALL that apply — separate with commas)".dimmed());
                for (j, opt) in q.options.iter().enumerate() {
                    let letter = (b'a' + j as u8) as char;
                    println!("     {} {}", format!("{})", letter).dimmed(), opt);
                }
            }
            "cloze" => {
                println!("     {}", "(Fill in ALL blanks — separate answers with semicolons)".dimmed());
            }
            "matching" => {
                println!("     {}", "(Match each item on the left with the correct item on the right)".dimmed());
                // Parse correct_answer pairs (e.g. "Dog=Mammal;Snake=Reptile") and display
                let pairs: Vec<(&str, &str)> = q.correct_answer.split(';')
                    .filter_map(|p| p.split_once('=').map(|(l, r)| (l.trim(), r.trim())))
                    .collect();
                for (j, (left, _)) in pairs.iter().enumerate() {
                    println!("     {} {} → ?", format!("{}.", j + 1).dimmed(), left);
                }
            }
            _ => {
                for (j, opt) in q.options.iter().enumerate() {
                    let letter = (b'a' + j as u8) as char;
                    println!("     {} {}", format!("{})", letter).dimmed(), opt);
                }
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
        streak += 1;
        if streak > best_streak {
            best_streak = streak;
        }
        adaptive::update_progress(conn, topic_id, true)?;

        // Streak encouragement messages
        if streak == 3 {
            println!("    {} {}", "🔥".bold(), "3 in a row! You're on fire!".bright_yellow());
        } else if streak == 5 {
            println!("    {} {}", "⚡".bold(), "5 streak! Unstoppable!".bright_yellow().bold());
        } else if streak >= 7 && streak % 2 == 1 {
            println!("    {} {}", "🏆".bold(), format!("{} streak! Legendary!", streak).bright_yellow().bold());
        }

        println!();
        display::print_divider();
        println!();
    }

    let score = (correct_count as f64 / total as f64) * 100.0;
    let quality = if score >= 90.0 { 5 } else if score >= 70.0 { 4 } else if score >= 50.0 { 3 } else { 2 };
    // In timed mode, apply confidence-weighted quality based on assumed fast recall
    let effective_quality = if timed {
        // Timed quiz: fast correct answers get full quality, boost by 1 if perfect
        let timed_q = adaptive::confidence_weighted_quality(score >= 50.0, 5_000);
        quality.max(timed_q)
    } else {
        quality
    };
    let effective_quality = spaced::fatigue_adjusted_quality(conn, effective_quality);
    spaced::update_spaced_repetition(conn, topic_id, effective_quality)?;
    adaptive::log_activity(conn, topic_id, "quiz", Some(score))?;

    display::print_header("Quiz Results");
    display::print_progress_bar(&topic_name, correct_count as f64, total as f64);
    if best_streak >= 3 {
        println!("  {} Best streak: {} in a row!", "🔥".bold(), best_streak.to_string().bright_yellow().bold());
    }
    println!();

    if score >= 80.0 {
        display::print_success("Excellent work! You're mastering this topic! 🌟");
    } else if score >= 50.0 {
        display::print_info("Good progress! Review the material and try again. 📖");
    } else {
        display::print_info("Keep learning! Use 'opentutor learn' to review. 💪");
    }

    // Check for difficulty auto-promotion
    if let Some(new_diff) = spaced::auto_promote_difficulty(conn, topic_id) {
        println!();
        display::print_success(&format!(
            "📈 Topic '{}' promoted to {} difficulty! Keep it up!",
            topic_name, new_diff
        ));
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

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_quiz_valid_topic() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "Arithmetic", 3, None, false).unwrap();
    }

    #[test]
    fn test_quiz_partial_match() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "fraction", 2, None, false).unwrap();
    }

    #[test]
    fn test_quiz_invalid_topic() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "Nonexistent", 5, None, false).unwrap();
    }

    #[test]
    fn test_quiz_fill_in_blank() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "Arithmetic", 10, None, false).unwrap();
    }

    #[test]
    fn test_quiz_music_topic() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "Musical Notes", 3, None, false).unwrap();
    }

    #[test]
    fn test_quiz_art_topic() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "Color Theory", 3, None, false).unwrap();
    }

    #[test]
    fn test_quiz_difficulty_filter() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "Arithmetic", 3, Some("easy"), false).unwrap();
    }

    #[test]
    fn test_quiz_difficulty_hard() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "Arithmetic", 3, Some("hard"), false).unwrap();
    }

    #[test]
    fn test_quiz_timed_mode() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "Arithmetic", 3, None, true).unwrap();
    }

    #[test]
    fn test_quiz_robotics_topic() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "Neural Networks", 3, None, false).unwrap();
    }

    #[test]
    fn test_quiz_number_theory_topic() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "Prime Numbers", 3, None, false).unwrap();
    }

    #[test]
    fn test_quiz_formal_languages_topic() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "Finite Automata", 3, None, false).unwrap();
    }

    #[test]
    fn test_quiz_philosophy_of_mind_topic() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "Consciousness", 3, None, false).unwrap();
    }

    #[test]
    fn test_quiz_organic_chemistry_topic() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "Hydrocarbons", 3, None, false).unwrap();
    }

    #[test]
    fn test_quiz_organic_chemistry_functional_groups() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "Functional Groups", 3, None, false).unwrap();
    }

    #[test]
    fn test_quiz_graph_theory_topic() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "Vertices and Edges", 3, None, false).unwrap();
    }

    #[test]
    fn test_quiz_graph_theory_trees() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "Trees", 3, None, false).unwrap();
    }

    #[test]
    fn test_quiz_graph_theory_planar() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "Planar Graphs", 3, None, false).unwrap();
    }
}
