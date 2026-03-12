use colored::*;
use rusqlite::Connection;
use crate::display;

pub fn run(conn: &Connection, limit: usize) -> Result<(), Box<dyn std::error::Error>> {
    display::print_header("Session History");

    let mut stmt = conn.prepare(
        "SELECT sl.timestamp, sl.activity_type, t.name, s.name, sl.score
         FROM session_log sl
         JOIN topics t ON t.id = sl.topic_id
         JOIN subjects s ON s.id = t.subject_id
         ORDER BY sl.timestamp DESC
         LIMIT ?1"
    )?;

    let rows: Vec<(String, String, String, String, Option<f64>)> = stmt
        .query_map([limit as i64], |r| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    if rows.is_empty() {
        display::print_info("No activity yet. Start learning with 'opentutor learn <subject>'!");
        return Ok(());
    }

    let activity_icon = |a: &str| -> &str {
        match a {
            "learn" => "📖",
            "quiz" => "📝",
            "review" => "🔄",
            "challenge" => "⚔️",
            _ => "📌",
        }
    };

    for (timestamp, activity, topic, subject, score) in &rows {
        let score_str = match score {
            Some(s) => format!(" — {:.0}%", s),
            None => String::new(),
        };
        println!(
            "  {} {} {} {} [{}]{}",
            timestamp.dimmed(),
            activity_icon(activity),
            activity.bold(),
            topic.bright_cyan(),
            subject.dimmed(),
            score_str.bright_yellow()
        );
    }

    println!();
    display::print_info(&format!("Showing last {} activities.", rows.len()));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use crate::engine::adaptive;

    #[test]
    fn test_history_empty() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, 10).unwrap();
    }

    #[test]
    fn test_history_with_data() {
        let conn = db::init_memory_db().unwrap();
        adaptive::log_activity(&conn, 1, "learn", None).unwrap();
        adaptive::log_activity(&conn, 1, "quiz", Some(80.0)).unwrap();
        adaptive::log_activity(&conn, 2, "learn", None).unwrap();
        run(&conn, 10).unwrap();
    }

    #[test]
    fn test_history_limited() {
        let conn = db::init_memory_db().unwrap();
        for i in 1..=5 {
            adaptive::log_activity(&conn, 1, "learn", Some(i as f64 * 20.0)).unwrap();
        }
        run(&conn, 3).unwrap();
    }
}
