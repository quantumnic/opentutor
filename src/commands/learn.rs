use colored::*;
use rusqlite::Connection;
use crate::display;
use crate::commands::achievements;
use crate::engine::adaptive;

pub fn run(conn: &Connection, subject: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Find subject (case-insensitive)
    let subject_row: Result<(i64, String, String), _> = conn.query_row(
        "SELECT id, name, description FROM subjects WHERE LOWER(name) = LOWER(?1)",
        [subject],
        |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
    );

    let (subject_id, subject_name, subject_desc) = match subject_row {
        Ok(row) => row,
        Err(_) => {
            display::print_error(&format!("Subject '{}' not found.", subject));
            display::print_info("Use 'opentutor subjects' to see available subjects.");
            return Ok(());
        }
    };

    display::print_header(&format!("Learning: {}", subject_name));
    display::print_content(&subject_desc);

    // Get topics for this subject
    let mut stmt = conn.prepare(
        "SELECT id, name, difficulty FROM topics WHERE subject_id = ?1 ORDER BY sort_order"
    )?;
    let topics: Vec<(i64, String, String)> = stmt.query_map([subject_id], |r| {
        Ok((r.get(0)?, r.get(1)?, r.get(2)?))
    })?.collect::<Result<Vec<_>, _>>()?;

    for (topic_id, topic_name, _difficulty) in &topics {
        let rec = adaptive::recommended_difficulty(conn, *topic_id);
        let level_badge = match rec {
            adaptive::Difficulty::Beginner => "🟢 Beginner".green(),
            adaptive::Difficulty::Intermediate => "🟡 Intermediate".yellow(),
            adaptive::Difficulty::Advanced => "🔴 Advanced".red(),
        };

        display::print_section(&format!("{} [{}]", topic_name, level_badge));

        // Show lesson content
        let mut lstmt = conn.prepare(
            "SELECT title, content FROM lessons WHERE topic_id = ?1 ORDER BY sort_order"
        )?;
        let lessons: Vec<(String, String)> = lstmt.query_map([topic_id], |r| {
            Ok((r.get(0)?, r.get(1)?))
        })?.collect::<Result<Vec<_>, _>>()?;

        for (title, content) in &lessons {
            println!("    {} {}", "▹".dimmed(), title.bold());
            for line in content.lines() {
                println!("      {}", line);
            }
            println!();
        }

        // Log the activity
        adaptive::log_activity(conn, *topic_id, "learn", None)?;

        display::print_divider();
    }

    println!();
    display::print_success(&format!("Completed learning session for {}!", subject_name));
    display::print_info(&format!("Test yourself: {}",
        "opentutor quiz <topic>".bright_cyan()
    ));

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
    fn test_learn_valid_subject() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "Mathematics").unwrap();
    }

    #[test]
    fn test_learn_case_insensitive() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "mathematics").unwrap();
    }

    #[test]
    fn test_learn_invalid_subject() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "Quantum Physics").unwrap(); // should not error
    }
}
