use colored::*;
use rusqlite::Connection;
use crate::display;

/// Reset all user progress (keep content, wipe learning history).
pub fn run(conn: &Connection, subject_filter: &Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    match subject_filter {
        Some(subject) => {
            // Reset progress for a specific subject
            let subject_row: Result<(i64, String), _> = conn.query_row(
                "SELECT id, name FROM subjects WHERE LOWER(name) = LOWER(?1)",
                [subject.as_str()],
                |r| Ok((r.get(0)?, r.get(1)?)),
            );

            let (subject_id, subject_name) = match subject_row {
                Ok(row) => row,
                Err(_) => {
                    display::print_error(&format!("Subject '{}' not found.", subject));
                    display::print_info("Use 'opentutor subjects' to see available subjects.");
                    return Ok(());
                }
            };

            let deleted_progress: usize = conn.execute(
                "DELETE FROM user_progress WHERE topic_id IN (SELECT id FROM topics WHERE subject_id = ?1)",
                [subject_id],
            )?;
            let deleted_logs: usize = conn.execute(
                "DELETE FROM session_log WHERE topic_id IN (SELECT id FROM topics WHERE subject_id = ?1)",
                [subject_id],
            )?;

            display::print_header(&format!("Reset: {}", subject_name));
            display::print_success(&format!(
                "Cleared {} progress records and {} session logs for {}.",
                deleted_progress, deleted_logs, subject_name.bold()
            ));
        }
        None => {
            // Reset all progress
            let deleted_progress: usize = conn.execute("DELETE FROM user_progress", [])?;
            let deleted_logs: usize = conn.execute("DELETE FROM session_log", [])?;

            display::print_header("Reset All Progress");
            display::print_success(&format!(
                "Cleared {} progress records and {} session logs.",
                deleted_progress, deleted_logs
            ));
        }
    }

    display::print_info("Your content library is untouched — only learning history was reset.");
    display::print_info(&format!("Start fresh: {}", "opentutor learn <subject>".bright_cyan()));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use crate::engine::adaptive;

    #[test]
    fn test_reset_all() {
        let conn = db::init_memory_db().unwrap();
        adaptive::update_progress(&conn, 1, true).unwrap();
        adaptive::log_activity(&conn, 1, "learn", Some(100.0)).unwrap();

        let count_before: i64 = conn.query_row(
            "SELECT COUNT(*) FROM user_progress", [], |r| r.get(0)
        ).unwrap();
        assert!(count_before > 0);

        run(&conn, &None).unwrap();

        let count_after: i64 = conn.query_row(
            "SELECT COUNT(*) FROM user_progress", [], |r| r.get(0)
        ).unwrap();
        assert_eq!(count_after, 0);
    }

    #[test]
    fn test_reset_specific_subject() {
        let conn = db::init_memory_db().unwrap();
        // Add progress in Mathematics (topic 1) and Science (topic 6)
        adaptive::update_progress(&conn, 1, true).unwrap();
        adaptive::update_progress(&conn, 6, true).unwrap();
        adaptive::log_activity(&conn, 1, "learn", Some(100.0)).unwrap();
        adaptive::log_activity(&conn, 6, "learn", Some(100.0)).unwrap();

        run(&conn, &Some("Mathematics".to_string())).unwrap();

        // Math progress should be gone
        let math_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM user_progress WHERE topic_id = 1", [], |r| r.get(0)
        ).unwrap();
        assert_eq!(math_count, 0);

        // Science progress should remain
        let science_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM user_progress WHERE topic_id = 6", [], |r| r.get(0)
        ).unwrap();
        assert_eq!(science_count, 1);
    }

    #[test]
    fn test_reset_invalid_subject() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, &Some("NonexistentSubject".to_string())).unwrap();
    }

    #[test]
    fn test_reset_empty_db() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, &None).unwrap(); // Should not error on empty progress
    }
}
