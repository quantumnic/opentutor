use colored::*;
use rusqlite::Connection;
use crate::display;
use crate::engine::spaced;

/// Show the user's weakest topics — those with lowest scores, highest fail rates,
/// or lowest retention. Helps focus study time where it matters most.
pub fn run(conn: &Connection, limit: usize) -> Result<(), Box<dyn std::error::Error>> {
    display::print_header("Weak Spots — Topics That Need Attention");

    // Gather topics with progress, sorted by weakness (low score + low retention + high fails)
    let mut stmt = conn.prepare(
        "SELECT t.id, t.name, s.name,
                p.score, p.attempts, p.correct, p.consecutive_fails, p.leech_count,
                p.ease_factor, p.interval_days
         FROM user_progress p
         JOIN topics t ON t.id = p.topic_id
         JOIN subjects s ON s.id = t.subject_id
         WHERE p.attempts >= 1
         ORDER BY p.score ASC, p.ease_factor ASC
         LIMIT ?1",
    )?;

    #[allow(clippy::type_complexity)]
    let rows: Vec<(i64, String, String, f64, i64, i64, i64, i64, f64, i64)> = stmt
        .query_map([limit as i64], |r| {
            Ok((
                r.get(0)?, r.get(1)?, r.get(2)?,
                r.get(3)?, r.get(4)?, r.get(5)?,
                r.get(6)?, r.get(7)?, r.get(8)?, r.get(9)?,
            ))
        })?
        .filter_map(|r| r.ok())
        .collect();

    if rows.is_empty() {
        display::print_info("No study data yet! Take some quizzes first to identify weak spots.");
        display::print_info("Try: opentutor quiz <topic>");
        return Ok(());
    }

    for (i, (topic_id, name, subject, score, attempts, correct, consec_fails, leech_count, _ease, _interval)) in rows.iter().enumerate() {
        let retention = spaced::estimate_retention(conn, *topic_id);
        let is_leech = spaced::is_leech(conn, *topic_id);

        let rank = format!("  {}.", i + 1).bold();
        let topic_display = format!("{} ({})", name, subject);

        // Color-code by severity
        let topic_colored = if *score < 40.0 || is_leech {
            topic_display.bright_red().bold()
        } else if *score < 60.0 {
            topic_display.bright_yellow().bold()
        } else {
            topic_display.bright_white().bold()
        };

        println!("{} {}", rank, topic_colored);
        println!("     Score: {:.0}% ({}/{} correct) | Retention: {:.0}%",
            score, correct, attempts, retention * 100.0);

        let mut flags = Vec::new();
        if is_leech {
            flags.push(format!("🩸 Leech (failed {} times)", leech_count));
        }
        if *consec_fails > 0 {
            flags.push(format!("🔥 {} consecutive fails", consec_fails));
        }
        if retention < 0.5 {
            flags.push("⚠️  Memory fading fast".to_string());
        }
        if !flags.is_empty() {
            println!("     {}", flags.join(" | ").bright_red());
        }

        println!();
    }

    display::print_divider();
    println!();
    display::print_hint("Focus on these topics first. Use 'opentutor review' for spaced repetition.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_weak_no_data() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, 10).unwrap();
    }

    #[test]
    fn test_weak_with_progress() {
        let conn = db::init_memory_db().unwrap();
        // Simulate some bad performance
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days, consecutive_fails, leech_count)
             VALUES (1, 30.0, 10, 3, 1.5, 1, 3, 1)",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days, consecutive_fails, leech_count)
             VALUES (2, 50.0, 8, 4, 2.0, 3, 0, 0)",
            [],
        ).unwrap();
        run(&conn, 10).unwrap();
    }
}
