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

#[allow(dead_code)]
/// Compute a confidence-weighted quality score (0-5) from correctness and response time.
/// Fast correct answers → higher quality (5), slow correct → 3-4, incorrect → 0-2.
/// `time_ms` is the time taken to answer in milliseconds.
pub fn confidence_weighted_quality(correct: bool, time_ms: u64) -> u8 {
    if !correct {
        // Even wrong answers: very fast wrong = 2 (close guess), slow = 0
        return if time_ms < 5_000 { 2 } else if time_ms < 15_000 { 1 } else { 0 };
    }

    // Correct answers: faster = higher confidence
    match time_ms {
        0..=3_000 => 5,       // Very fast: strong recall
        3_001..=8_000 => 4,   // Moderate: decent recall
        8_001..=20_000 => 3,  // Slow: barely recalled
        _ => 3,               // Very slow but correct: still a pass
    }
}

#[allow(dead_code)]
/// Summary of a review session for post-session analytics.
#[derive(Debug, Clone)]
pub struct SessionSummary {
    pub topics_reviewed: usize,
    pub correct: usize,
    pub incorrect: usize,
    pub accuracy: f64,
    pub avg_quality: f64,
    pub subjects_covered: usize,
    pub leeches_encountered: usize,
    pub promotions: Vec<(String, String)>, // (topic_name, new_difficulty)
}

#[allow(dead_code)]
/// Build a session summary from today's activity log.
pub fn todays_session_summary(conn: &Connection) -> SessionSummary {
    let total: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM session_log WHERE activity_type IN ('review', 'quiz') AND DATE(timestamp) = DATE('now')",
            [], |r| r.get(0),
        ).unwrap_or(0);

    let correct: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM session_log WHERE activity_type IN ('review', 'quiz') AND score >= 50.0 AND DATE(timestamp) = DATE('now')",
            [], |r| r.get(0),
        ).unwrap_or(0);

    let avg_quality: f64 = conn
        .query_row(
            "SELECT COALESCE(AVG(score), 0) FROM session_log WHERE activity_type IN ('review', 'quiz') AND DATE(timestamp) = DATE('now')",
            [], |r| r.get(0),
        ).unwrap_or(0.0);

    let subjects: i64 = conn
        .query_row(
            "SELECT COUNT(DISTINCT t.subject_id) FROM session_log sl
             JOIN topics t ON t.id = sl.topic_id
             WHERE sl.activity_type IN ('review', 'quiz') AND DATE(sl.timestamp) = DATE('now')",
            [], |r| r.get(0),
        ).unwrap_or(0);

    let leeches: i64 = conn
        .query_row(
            "SELECT COUNT(DISTINCT sl.topic_id) FROM session_log sl
             JOIN user_progress p ON p.topic_id = sl.topic_id
             WHERE sl.activity_type IN ('review', 'quiz')
             AND DATE(sl.timestamp) = DATE('now')
             AND p.leech_count > 0",
            [], |r| r.get(0),
        ).unwrap_or(0);

    let incorrect = total - correct;
    let accuracy = if total > 0 { correct as f64 / total as f64 * 100.0 } else { 0.0 };

    SessionSummary {
        topics_reviewed: total as usize,
        correct: correct as usize,
        incorrect: incorrect as usize,
        accuracy,
        avg_quality: avg_quality / 20.0, // normalize 0-100 to 0-5 scale
        subjects_covered: subjects as usize,
        leeches_encountered: leeches as usize,
        promotions: Vec::new(), // filled by caller if any auto-promotions occurred
    }
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

    #[test]
    fn test_confidence_weighted_quality_correct_fast() {
        assert_eq!(confidence_weighted_quality(true, 2000), 5);
    }

    #[test]
    fn test_confidence_weighted_quality_correct_slow() {
        assert_eq!(confidence_weighted_quality(true, 15000), 3);
    }

    #[test]
    fn test_confidence_weighted_quality_incorrect() {
        assert_eq!(confidence_weighted_quality(false, 3000), 2);
        assert_eq!(confidence_weighted_quality(false, 10000), 1);
        assert_eq!(confidence_weighted_quality(false, 20000), 0);
    }

    #[test]
    fn test_session_summary_empty() {
        let conn = db::init_memory_db().unwrap();
        let summary = todays_session_summary(&conn);
        assert_eq!(summary.topics_reviewed, 0);
        assert_eq!(summary.accuracy, 0.0);
    }

    #[test]
    fn test_session_summary_with_data() {
        let conn = db::init_memory_db().unwrap();
        log_activity(&conn, 1, "review", Some(80.0)).unwrap();
        log_activity(&conn, 2, "quiz", Some(60.0)).unwrap();
        log_activity(&conn, 3, "review", Some(30.0)).unwrap();
        let summary = todays_session_summary(&conn);
        assert_eq!(summary.topics_reviewed, 3);
        assert_eq!(summary.correct, 2); // 80 and 60 >= 50
        assert_eq!(summary.incorrect, 1);
    }
}
