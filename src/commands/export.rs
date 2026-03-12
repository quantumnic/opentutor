use colored::*;
use rusqlite::Connection;
use serde_json::{json, Value};
use crate::display;
use crate::engine::spaced;

/// Export learning data as JSON for backup/analysis.
pub fn run(conn: &Connection, output: &Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    display::print_header("Export Learning Data");

    // Gather progress data
    let mut stmt = conn.prepare(
        "SELECT t.name, s.name, p.score, p.attempts, p.correct,
                p.ease_factor, p.interval_days, p.last_reviewed, p.next_review
         FROM user_progress p
         JOIN topics t ON t.id = p.topic_id
         JOIN subjects s ON s.id = t.subject_id
         ORDER BY s.name, t.name"
    )?;

    let progress: Vec<Value> = stmt.query_map([], |r| {
        let topic_id: i64 = conn.query_row(
            "SELECT t.id FROM topics t JOIN subjects s ON s.id = t.subject_id WHERE t.name = ?1 AND s.name = ?2",
            rusqlite::params![r.get::<_, String>(0)?, r.get::<_, String>(1)?],
            |row| row.get(0),
        ).unwrap_or(0);

        let retention = spaced::estimate_retention(conn, topic_id);

        Ok(json!({
            "topic": r.get::<_, String>(0)?,
            "subject": r.get::<_, String>(1)?,
            "score": r.get::<_, f64>(2)?,
            "attempts": r.get::<_, i64>(3)?,
            "correct": r.get::<_, i64>(4)?,
            "ease_factor": r.get::<_, f64>(5)?,
            "interval_days": r.get::<_, i64>(6)?,
            "last_reviewed": r.get::<_, Option<String>>(7)?,
            "next_review": r.get::<_, Option<String>>(8)?,
            "estimated_retention": format!("{:.1}%", retention * 100.0),
        }))
    })?.collect::<Result<Vec<_>, _>>()?;

    // Gather session log
    let mut log_stmt = conn.prepare(
        "SELECT t.name, sl.activity_type, sl.score, sl.timestamp
         FROM session_log sl
         JOIN topics t ON t.id = sl.topic_id
         ORDER BY sl.timestamp DESC
         LIMIT 100"
    )?;

    let sessions: Vec<Value> = log_stmt.query_map([], |r| {
        Ok(json!({
            "topic": r.get::<_, String>(0)?,
            "activity": r.get::<_, String>(1)?,
            "score": r.get::<_, Option<f64>>(2)?,
            "timestamp": r.get::<_, String>(3)?,
        }))
    })?.collect::<Result<Vec<_>, _>>()?;

    // Content stats
    let total_subjects: i64 = conn.query_row("SELECT COUNT(*) FROM subjects", [], |r| r.get(0))?;
    let total_topics: i64 = conn.query_row("SELECT COUNT(*) FROM topics", [], |r| r.get(0))?;
    let total_lessons: i64 = conn.query_row("SELECT COUNT(*) FROM lessons", [], |r| r.get(0))?;
    let total_questions: i64 = conn.query_row("SELECT COUNT(*) FROM quiz_questions", [], |r| r.get(0))?;

    let export = json!({
        "opentutor_export": {
            "version": "1.0",
            "exported_at": chrono::Local::now().to_rfc3339(),
        },
        "content_stats": {
            "subjects": total_subjects,
            "topics": total_topics,
            "lessons": total_lessons,
            "quiz_questions": total_questions,
        },
        "progress": progress,
        "recent_sessions": sessions,
    });

    let json_str = serde_json::to_string_pretty(&export)?;

    match output {
        Some(path) => {
            std::fs::write(path, &json_str)?;
            display::print_success(&format!("Exported to {}", path.bright_cyan()));
        }
        None => {
            println!("{}", json_str);
        }
    }

    display::print_info(&format!(
        "Exported {} progress records and {} session logs.",
        progress.len(), sessions.len()
    ));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use crate::engine::adaptive;

    #[test]
    fn test_export_empty() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, &None).unwrap();
    }

    #[test]
    fn test_export_with_data() {
        let conn = db::init_memory_db().unwrap();
        adaptive::update_progress(&conn, 1, true).unwrap();
        adaptive::log_activity(&conn, 1, "learn", Some(100.0)).unwrap();
        run(&conn, &None).unwrap();
    }

    #[test]
    fn test_export_to_file() {
        let conn = db::init_memory_db().unwrap();
        adaptive::update_progress(&conn, 1, true).unwrap();
        let path = "/tmp/opentutor_test_export.json".to_string();
        run(&conn, &Some(path.clone())).unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("opentutor_export"));
        std::fs::remove_file(&path).ok();
    }
}
