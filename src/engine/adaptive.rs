use rusqlite::Connection;

/// Difficulty levels
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
}

#[allow(dead_code)]
impl Difficulty {
    pub fn as_str(&self) -> &str {
        match self {
            Difficulty::Beginner => "beginner",
            Difficulty::Intermediate => "intermediate",
            Difficulty::Advanced => "advanced",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "intermediate" => Difficulty::Intermediate,
            "advanced" => Difficulty::Advanced,
            _ => Difficulty::Beginner,
        }
    }
}

/// Get the recommended difficulty for a topic based on user performance.
pub fn recommended_difficulty(conn: &Connection, topic_id: i64) -> Difficulty {
    let result: Result<(f64, i64), _> = conn.query_row(
        "SELECT score, attempts FROM user_progress WHERE topic_id = ?1",
        [topic_id],
        |r| Ok((r.get(0)?, r.get(1)?)),
    );
    match result {
        Ok((score, attempts)) if attempts >= 3 && score >= 80.0 => Difficulty::Advanced,
        Ok((score, attempts)) if attempts >= 2 && score >= 50.0 => Difficulty::Intermediate,
        _ => Difficulty::Beginner,
    }
}

/// Update user progress after an activity.
pub fn update_progress(
    conn: &Connection,
    topic_id: i64,
    correct: bool,
) -> Result<(), rusqlite::Error> {
    // Upsert progress
    conn.execute(
        "INSERT INTO user_progress (topic_id, score, attempts, correct, last_reviewed)
         VALUES (?1, ?2, 1, ?3, datetime('now'))
         ON CONFLICT(topic_id) DO UPDATE SET
           attempts = attempts + 1,
           correct = correct + ?3,
           score = CAST((correct + ?3) AS REAL) / CAST((attempts + 1) AS REAL) * 100.0,
           last_reviewed = datetime('now')",
        rusqlite::params![topic_id, if correct { 100.0 } else { 0.0 }, if correct { 1 } else { 0 }],
    )?;
    Ok(())
}

/// Log a session activity.
pub fn log_activity(
    conn: &Connection,
    topic_id: i64,
    activity_type: &str,
    score: Option<f64>,
) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO session_log (topic_id, activity_type, score) VALUES (?1, ?2, ?3)",
        rusqlite::params![topic_id, activity_type, score],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_difficulty_from_str() {
        assert_eq!(Difficulty::from_str("beginner"), Difficulty::Beginner);
        assert_eq!(Difficulty::from_str("intermediate"), Difficulty::Intermediate);
        assert_eq!(Difficulty::from_str("advanced"), Difficulty::Advanced);
        assert_eq!(Difficulty::from_str("unknown"), Difficulty::Beginner);
    }

    #[test]
    fn test_recommended_difficulty_default() {
        let conn = db::init_memory_db().unwrap();
        assert_eq!(recommended_difficulty(&conn, 1), Difficulty::Beginner);
    }

    #[test]
    fn test_update_progress() {
        let conn = db::init_memory_db().unwrap();
        update_progress(&conn, 1, true).unwrap();
        update_progress(&conn, 1, true).unwrap();
        update_progress(&conn, 1, false).unwrap();
        let attempts: i64 = conn.query_row(
            "SELECT attempts FROM user_progress WHERE topic_id = 1", [], |r| r.get(0)
        ).unwrap();
        assert_eq!(attempts, 3);
    }

    #[test]
    fn test_log_activity() {
        let conn = db::init_memory_db().unwrap();
        log_activity(&conn, 1, "learn", Some(100.0)).unwrap();
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM session_log WHERE topic_id = 1", [], |r| r.get(0)
        ).unwrap();
        assert_eq!(count, 1);
    }
}
