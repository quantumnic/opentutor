use colored::*;
use rusqlite::Connection;
use crate::display;

/// Show topics where the user has made the most mistakes, along with
/// accuracy rates, to help targeted review of problem areas.
pub fn run(conn: &Connection, limit: usize) -> Result<(), Box<dyn std::error::Error>> {
    display::print_header("Quiz Mistakes — Learn From Your Errors");

    let mut stmt = conn.prepare(
        "SELECT t.name, s.name, p.attempts, p.correct,
                CAST(p.correct AS REAL) / CAST(p.attempts AS REAL) * 100.0 as accuracy,
                p.attempts - p.correct as wrong_count
         FROM user_progress p
         JOIN topics t ON t.id = p.topic_id
         JOIN subjects s ON s.id = t.subject_id
         WHERE p.attempts > 0 AND p.correct < p.attempts
         ORDER BY wrong_count DESC, accuracy ASC
         LIMIT ?1",
    )?;

    let rows: Vec<(String, String, i64, i64, f64, i64)> = stmt
        .query_map([limit as i64], |r| {
            Ok((
                r.get(0)?, r.get(1)?, r.get(2)?,
                r.get(3)?, r.get(4)?, r.get(5)?,
            ))
        })?
        .filter_map(|r| r.ok())
        .collect();

    if rows.is_empty() {
        display::print_info("No mistakes recorded yet! Keep studying and quizzing.");
        return Ok(());
    }

    println!("  {:<30} {:<18} {:>8} {:>8} {:>10}",
        "Topic".bold().underline(),
        "Subject".bold().underline(),
        "Wrong".bold().underline(),
        "Total".bold().underline(),
        "Accuracy".bold().underline(),
    );
    println!();

    for (name, subject, attempts, _correct, accuracy, wrong_count) in &rows {
        let acc_display = format!("{:.0}%", accuracy);
        let acc_colored = if *accuracy < 40.0 {
            acc_display.bright_red()
        } else if *accuracy < 70.0 {
            acc_display.bright_yellow()
        } else {
            acc_display.bright_green()
        };

        let subject_short = if subject.len() > 16 {
            format!("{}…", &subject[..15])
        } else {
            subject.clone()
        };
        let name_short = if name.len() > 28 {
            format!("{}…", &name[..27])
        } else {
            name.clone()
        };

        println!("  {:<30} {:<18} {:>8} {:>8} {:>10}",
            name_short, subject_short, wrong_count, attempts, acc_colored);
    }

    let total_wrong: i64 = rows.iter().map(|r| r.5).sum();
    let total_attempts: i64 = rows.iter().map(|r| r.2).sum();
    println!();
    display::print_divider();
    println!("  Total mistakes: {} across {} attempts", 
        total_wrong.to_string().bright_red().bold(),
        total_attempts);
    println!();
    display::print_hint("Use 'opentutor review' to revisit these topics with spaced repetition.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_mistakes_no_data() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, 10).unwrap();
    }

    #[test]
    fn test_mistakes_with_errors() {
        let conn = db::init_memory_db().unwrap();
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days)
             VALUES (1, 40.0, 10, 4, 2.0, 1)",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days)
             VALUES (2, 80.0, 5, 4, 2.5, 3)",
            [],
        ).unwrap();
        run(&conn, 10).unwrap();
    }

    #[test]
    fn test_mistakes_no_errors() {
        let conn = db::init_memory_db().unwrap();
        // Perfect score — should not appear
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days)
             VALUES (1, 100.0, 5, 5, 2.8, 10)",
            [],
        ).unwrap();
        run(&conn, 10).unwrap();
    }
}
