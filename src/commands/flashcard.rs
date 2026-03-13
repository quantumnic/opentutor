use colored::*;
use rusqlite::Connection;
use crate::display;
use crate::engine::{adaptive, spaced};

/// Show flashcards (concept → explanation) for a topic or subject.
/// Great for quick memorization sessions.
pub fn run(conn: &Connection, topic: &str, count: usize) -> Result<(), Box<dyn std::error::Error>> {
    display::print_header(&format!("Flashcards: {}", topic));

    // Find matching topics (by topic name or subject name)
    let topic_lower = topic.to_lowercase();
    let mut stmt = conn.prepare(
        "SELECT t.id, t.name, s.name
         FROM topics t
         JOIN subjects s ON s.id = t.subject_id
         WHERE LOWER(t.name) LIKE ?1 OR LOWER(s.name) LIKE ?1
         ORDER BY s.name, t.sort_order",
    )?;
    let pattern = format!("%{}%", topic_lower);
    let topics: Vec<(i64, String, String)> = stmt
        .query_map([&pattern], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
        .collect::<Result<Vec<_>, _>>()?;

    if topics.is_empty() {
        display::print_error(&format!("No topics found matching '{}'.", topic));
        display::print_hint("Try: opentutor subjects");
        return Ok(());
    }

    let mut total_cards = 0;

    for (topic_id, topic_name, subject_name) in &topics {
        // Get explanations as flashcards
        let mut expl_stmt = conn.prepare(
            "SELECT concept, explanation, analogy FROM explanations WHERE topic_id = ?1",
        )?;
        let explanations: Vec<(String, String, Option<String>)> = expl_stmt
            .query_map([topic_id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
            .collect::<Result<Vec<_>, _>>()?;

        // Also get quiz Q&A pairs as flashcards
        let mut quiz_stmt = conn.prepare(
            "SELECT question, correct_answer, explanation FROM quiz_questions WHERE topic_id = ?1 ORDER BY RANDOM() LIMIT ?2",
        )?;
        let remaining = if count > explanations.len() { count - explanations.len() } else { 0 };
        let quiz_cards: Vec<(String, String, String)> = quiz_stmt
            .query_map(rusqlite::params![topic_id, remaining], |r| {
                Ok((r.get(0)?, r.get(1)?, r.get(2)?))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        let card_count = explanations.len() + quiz_cards.len();
        if card_count == 0 {
            continue;
        }

        display::print_section(&format!("{} ({})", topic_name, subject_name));
        println!();

        // Show explanation-based flashcards
        for (i, (concept, explanation, analogy)) in explanations.iter().enumerate() {
            if total_cards >= count {
                break;
            }
            println!(
                "    {} {}",
                format!("Card {}.", i + 1).bold().bright_cyan(),
                "FRONT".dimmed()
            );
            println!("    📝 {}", concept.bold().bright_white());
            println!();
            println!("    {} {}", "BACK".dimmed(), "↓".dimmed());
            // Wrap explanation at ~70 chars for readability
            for line in explanation.lines() {
                println!("    {}", line);
            }
            if let Some(a) = analogy {
                println!();
                println!("    {} {}", "💡 Analogy:".yellow(), a);
            }
            println!();
            display::print_divider();
            println!();
            total_cards += 1;
        }

        // Show quiz-based flashcards
        for (q, answer, explanation) in &quiz_cards {
            if total_cards >= count {
                break;
            }
            let card_num = total_cards + 1;
            println!(
                "    {} {}",
                format!("Card {}.", card_num).bold().bright_cyan(),
                "FRONT".dimmed()
            );
            println!("    ❓ {}", q.bold().bright_white());
            println!();
            println!("    {} {}", "BACK".dimmed(), "↓".dimmed());
            println!("    ✅ {}", answer.bright_green().bold());
            println!("    {}", explanation);
            println!();
            display::print_divider();
            println!();
            total_cards += 1;
        }

        // Log flashcard study activity
        adaptive::log_activity(conn, *topic_id, "flashcard", Some(100.0))?;
        adaptive::update_progress(conn, *topic_id, true)?;
        spaced::update_spaced_repetition(conn, *topic_id, 4)?;
    }

    if total_cards == 0 {
        display::print_info("No flashcard content available for the matching topics.");
        display::print_hint("Try a broader search term or check available subjects.");
    } else {
        println!();
        display::print_success(&format!(
            "Reviewed {} flashcard{}! Spaced repetition updated.",
            total_cards,
            if total_cards == 1 { "" } else { "s" }
        ));
        display::print_hint("Tip: Review again tomorrow for better retention! 🧠");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_flashcard_by_subject() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "mathematics", 5).unwrap();
    }

    #[test]
    fn test_flashcard_by_topic() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "fractions", 3).unwrap();
    }

    #[test]
    fn test_flashcard_no_match() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "nonexistent_topic_xyz", 5).unwrap();
    }

    #[test]
    fn test_flashcard_count_limit() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "mathematics", 1).unwrap();
    }
}
