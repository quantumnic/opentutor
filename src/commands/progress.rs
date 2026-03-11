use colored::*;
use rusqlite::Connection;
use crate::display;
use crate::engine::spaced;

pub fn run(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    display::print_header("Your Learning Progress");

    let total_topics: i64 = conn.query_row("SELECT COUNT(*) FROM topics", [], |r| r.get(0))?;
    let studied: i64 = conn.query_row("SELECT COUNT(*) FROM user_progress", [], |r| r.get(0))?;
    let total_sessions: i64 = conn.query_row("SELECT COUNT(*) FROM session_log", [], |r| r.get(0))?;

    display::print_section("Overview");
    display::print_progress_bar("Topics studied", studied as f64, total_topics as f64);
    println!("  Total sessions: {}", total_sessions.to_string().bold());
    println!();

    display::print_section("By Subject");
    let mut stmt = conn.prepare(
        "SELECT s.name,
                COUNT(DISTINCT t.id) as total_topics,
                COUNT(DISTINCT p.topic_id) as studied_topics,
                COALESCE(AVG(p.score), 0) as avg_score
         FROM subjects s
         LEFT JOIN topics t ON t.subject_id = s.id
         LEFT JOIN user_progress p ON p.topic_id = t.id
         GROUP BY s.id ORDER BY s.name"
    )?;

    let rows = stmt.query_map([], |r| {
        Ok((
            r.get::<_, String>(0)?,
            r.get::<_, i64>(1)?,
            r.get::<_, i64>(2)?,
            r.get::<_, f64>(3)?,
        ))
    })?;

    for row in rows {
        let (name, total, studied_count, avg_score) = row?;
        let status = if studied_count == 0 {
            "Not started".dimmed().to_string()
        } else if studied_count == total {
            "Complete \u{2713}".bright_green().to_string()
        } else {
            format!("{}/{} topics", studied_count, total)
        };
        println!("  \u{1F4D8} {} \u{2014} {} | Avg: {:.0}%",
            name.bold(), status, avg_score
        );
    }
    println!();

    let mut due_stmt = conn.prepare(
        "SELECT t.name, s.name, p.next_review
         FROM user_progress p
         JOIN topics t ON t.id = p.topic_id
         JOIN subjects s ON s.id = t.subject_id
         WHERE p.next_review IS NOT NULL AND p.next_review <= datetime('now')
         ORDER BY p.next_review ASC
         LIMIT 5"
    )?;

    let due_rows: Vec<(String, String, String)> = due_stmt.query_map([], |r| {
        Ok((r.get(0)?, r.get(1)?, r.get(2)?))
    })?.collect::<Result<Vec<_>, _>>()?;

    if !due_rows.is_empty() {
        display::print_section("Due for Review \u{1F504}");
        for (topic, subject, _date) in &due_rows {
            println!("  \u{2022} {} ({})", topic.bright_yellow(), subject.dimmed());
        }
        println!();
        display::print_hint("Review these topics to strengthen your memory!");
    }

    // Show total due for review
    let due_count = spaced::count_due_topics(conn).unwrap_or(0);
    if due_count > 0 {
        println!();
        display::print_info(&format!(
            "{} topics due for review! Run: {}",
            due_count,
            "opentutor review".bright_cyan()
        ));
    }

    // Show lapsed topics warning
    let lapsed_count = spaced::count_lapsed_topics(conn).unwrap_or(0);
    if lapsed_count > 0 {
        display::print_hint(&format!(
            "⚠️  {} topics are lapsed (overdue 7+ days) — review soon to prevent forgetting!",
            lapsed_count
        ));
    }

    if studied == 0 {
        println!();
        display::print_info("You haven't started yet! Try: opentutor learn <subject>");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use crate::engine::adaptive;

    #[test]
    fn test_progress_empty() {
        let conn = db::init_memory_db().unwrap();
        run(&conn).unwrap();
    }

    #[test]
    fn test_progress_with_data() {
        let conn = db::init_memory_db().unwrap();
        adaptive::update_progress(&conn, 1, true).unwrap();
        adaptive::update_progress(&conn, 2, false).unwrap();
        adaptive::log_activity(&conn, 1, "learn", Some(100.0)).unwrap();
        run(&conn).unwrap();
    }
}
