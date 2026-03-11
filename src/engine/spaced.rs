use rusqlite::Connection;

/// SM-2 algorithm parameters
const MIN_EASE: f64 = 1.3;

/// Calculate next review date using a simplified SM-2 algorithm.
/// quality: 0-5 (0-2 = fail, 3-5 = pass)
pub fn update_spaced_repetition(
    conn: &Connection,
    topic_id: i64,
    quality: u8,
) -> Result<(), rusqlite::Error> {
    let quality = quality.min(5);

    let (ease, interval): (f64, i64) = conn
        .query_row(
            "SELECT ease_factor, interval_days FROM user_progress WHERE topic_id = ?1",
            [topic_id],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .unwrap_or((2.5, 1));

    let (new_ease, new_interval) = if quality >= 3 {
        // Correct answer
        let new_interval = match interval {
            0 | 1 => 1,
            2 => 6,
            n => (n as f64 * ease).round() as i64,
        };
        let new_ease = ease + (0.1 - (5.0 - quality as f64) * (0.08 + (5.0 - quality as f64) * 0.02));
        (new_ease.max(MIN_EASE), new_interval)
    } else {
        // Incorrect — reset interval
        (ease.max(MIN_EASE), 1)
    };

    conn.execute(
        "INSERT INTO user_progress (topic_id, ease_factor, interval_days, next_review, last_reviewed)
         VALUES (?1, ?2, ?3, datetime('now', '+' || ?3 || ' days'), datetime('now'))
         ON CONFLICT(topic_id) DO UPDATE SET
           ease_factor = ?2,
           interval_days = ?3,
           next_review = datetime('now', '+' || ?3 || ' days'),
           last_reviewed = datetime('now')",
        rusqlite::params![topic_id, new_ease, new_interval],
    )?;
    Ok(())
}

/// Get topics due for review.
#[allow(dead_code)]
pub fn get_due_topics(conn: &Connection) -> Result<Vec<(i64, String, String)>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT t.id, t.name, s.name FROM topics t
         JOIN subjects s ON t.subject_id = s.id
         JOIN user_progress p ON p.topic_id = t.id
         WHERE p.next_review <= datetime('now')
         ORDER BY p.next_review ASC",
    )?;
    let rows = stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?;
    rows.collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_spaced_repetition_correct() {
        let conn = db::init_memory_db().unwrap();
        // First insert a progress entry
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days)
             VALUES (1, 100.0, 1, 1, 2.5, 1)", []
        ).unwrap();
        update_spaced_repetition(&conn, 1, 4).unwrap();
        let (ease, interval): (f64, i64) = conn.query_row(
            "SELECT ease_factor, interval_days FROM user_progress WHERE topic_id = 1",
            [], |r| Ok((r.get(0)?, r.get(1)?))
        ).unwrap();
        assert!(ease >= 2.4);
        assert!(interval >= 1);
    }

    #[test]
    fn test_spaced_repetition_fail_resets_interval() {
        let conn = db::init_memory_db().unwrap();
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days)
             VALUES (1, 50.0, 5, 3, 2.5, 6)", []
        ).unwrap();
        update_spaced_repetition(&conn, 1, 1).unwrap();
        let interval: i64 = conn.query_row(
            "SELECT interval_days FROM user_progress WHERE topic_id = 1",
            [], |r| r.get(0)
        ).unwrap();
        assert_eq!(interval, 1, "Failed review should reset interval to 1");
    }

    #[test]
    fn test_get_due_topics_empty() {
        let conn = db::init_memory_db().unwrap();
        let due = get_due_topics(&conn).unwrap();
        assert!(due.is_empty());
    }

    #[test]
    fn test_ease_never_below_min() {
        let conn = db::init_memory_db().unwrap();
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days)
             VALUES (1, 0.0, 10, 0, 1.3, 1)", []
        ).unwrap();
        update_spaced_repetition(&conn, 1, 0).unwrap();
        let ease: f64 = conn.query_row(
            "SELECT ease_factor FROM user_progress WHERE topic_id = 1",
            [], |r| r.get(0)
        ).unwrap();
        assert!(ease >= 1.3, "Ease factor should never go below 1.3");
    }
}
