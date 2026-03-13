use colored::*;
use rusqlite::Connection;
use crate::display;

/// Add, remove, or list bookmarked (favorite) topics for quick access.
pub fn run(
    conn: &Connection,
    action: &str,
    topic: &Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Ensure the bookmarks table exists
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS bookmarks (
            topic_id INTEGER PRIMARY KEY REFERENCES topics(id),
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        );",
    )?;

    match action {
        "add" => {
            let topic_name = topic
                .as_deref()
                .ok_or("Specify a topic to bookmark: opentutor bookmark add <topic>")?;
            let row: Result<(i64, String), _> = conn.query_row(
                "SELECT id, name FROM topics WHERE LOWER(name) LIKE '%' || LOWER(?1) || '%'",
                [topic_name],
                |r| Ok((r.get(0)?, r.get(1)?)),
            );
            match row {
                Ok((tid, name)) => {
                    conn.execute(
                        "INSERT OR IGNORE INTO bookmarks (topic_id) VALUES (?1)",
                        [tid],
                    )?;
                    println!("  {} Bookmarked: {}", "★".bright_yellow().bold(), name.bold());
                }
                Err(_) => {
                    display::print_error(&format!("Topic '{}' not found.", topic_name));
                    display::print_hint("Use 'opentutor subjects' to see available topics.");
                }
            }
        }
        "remove" | "rm" => {
            let topic_name = topic
                .as_deref()
                .ok_or("Specify a topic to remove: opentutor bookmark remove <topic>")?;
            let row: Result<(i64, String), _> = conn.query_row(
                "SELECT id, name FROM topics WHERE LOWER(name) LIKE '%' || LOWER(?1) || '%'",
                [topic_name],
                |r| Ok((r.get(0)?, r.get(1)?)),
            );
            match row {
                Ok((tid, name)) => {
                    let removed = conn.execute(
                        "DELETE FROM bookmarks WHERE topic_id = ?1",
                        [tid],
                    )?;
                    if removed > 0 {
                        println!("  {} Removed bookmark: {}", "☆".dimmed(), name);
                    } else {
                        display::print_info(&format!("'{}' was not bookmarked.", name));
                    }
                }
                Err(_) => {
                    display::print_error(&format!("Topic '{}' not found.", topic_name));
                }
            }
        }
        _ => {
            display::print_header("★ Bookmarked Topics");
            let mut stmt = conn.prepare(
                "SELECT t.name, s.name, t.difficulty,
                        COALESCE(p.score, 0) as score, COALESCE(p.attempts, 0) as attempts
                 FROM bookmarks b
                 JOIN topics t ON t.id = b.topic_id
                 JOIN subjects s ON s.id = t.subject_id
                 LEFT JOIN user_progress p ON p.topic_id = t.id
                 ORDER BY b.created_at DESC",
            )?;
            let bookmarks: Vec<(String, String, String, f64, i64)> = stmt
                .query_map([], |r| {
                    Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?))
                })?
                .collect::<Result<Vec<_>, _>>()?;

            if bookmarks.is_empty() {
                display::print_info("No bookmarked topics yet.");
                display::print_hint("Add one: opentutor bookmark add <topic>");
            } else {
                for (name, subject, diff, score, attempts) in &bookmarks {
                    let badge = match diff.as_str() {
                        "intermediate" => "🟡",
                        "advanced" => "🔴",
                        _ => "🟢",
                    };
                    let progress = if *attempts > 0 {
                        format!("{:.0}% ({} attempts)", score, attempts)
                    } else {
                        "not started".to_string()
                    };
                    println!(
                        "  {} {} ({}) — {}",
                        badge,
                        name.bold().bright_white(),
                        subject.dimmed(),
                        progress.dimmed()
                    );
                }
                println!();
                display::print_info(&format!(
                    "{} bookmarked topic{}",
                    bookmarks.len(),
                    if bookmarks.len() == 1 { "" } else { "s" }
                ));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_bookmark_list_empty() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "list", &None).unwrap();
    }

    #[test]
    fn test_bookmark_add_and_list() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "add", &Some("Arithmetic".to_string())).unwrap();
        run(&conn, "list", &None).unwrap();
    }

    #[test]
    fn test_bookmark_add_remove() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "add", &Some("Fractions".to_string())).unwrap();
        run(&conn, "remove", &Some("Fractions".to_string())).unwrap();
        // Verify empty
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS bookmarks (
                topic_id INTEGER PRIMARY KEY REFERENCES topics(id),
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            );"
        ).unwrap();
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM bookmarks", [], |r| r.get(0)
        ).unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_bookmark_add_invalid() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "add", &Some("Nonexistent_XYZ".to_string())).unwrap();
    }

    #[test]
    fn test_bookmark_add_duplicate() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "add", &Some("Arithmetic".to_string())).unwrap();
        run(&conn, "add", &Some("Arithmetic".to_string())).unwrap(); // should not error
    }

    #[test]
    fn test_bookmark_remove_not_bookmarked() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "remove", &Some("Arithmetic".to_string())).unwrap();
    }
}
