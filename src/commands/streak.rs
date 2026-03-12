use colored::Colorize;
use rusqlite::Connection;

use crate::engine::spaced;

pub fn run(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    let streak = spaced::calculate_streak(conn);

    println!("\n{}", "🔥 Learning Streak".bold());
    println!("{}", "─".repeat(40));

    if streak == 0 {
        println!(
            "{}",
            "No active streak. Start learning today to begin one!".yellow()
        );
    } else {
        let fire = "🔥".repeat(streak.min(10) as usize);
        println!("Current streak: {} {} days", fire, streak);
        println!();

        // Streak milestones
        let milestone = match streak {
            1..=2 => "Just getting started — keep it up!",
            3..=6 => "Nice habit forming! 💪",
            7..=13 => "A whole week! You're building momentum! 🌟",
            14..=29 => "Two weeks strong! Impressive dedication! 🏆",
            30..=59 => "A full month! You're a learning machine! 🎯",
            60..=89 => "Two months! Extraordinary commitment! 🚀",
            90..=179 => "Three months! You're unstoppable! 👑",
            _ => "Legend status! Half a year or more! 🏅",
        };
        println!("{}", milestone.green());
    }

    // Show bonus info
    if streak >= 3 {
        let bonus = 1.0
            + ((streak - 3) as f64 / 11.0) * (spaced::MAX_STREAK_BONUS - 1.0);
        let bonus = bonus.min(spaced::MAX_STREAK_BONUS);
        println!();
        println!(
            "📈 Streak bonus: {:.0}% longer review intervals",
            (bonus - 1.0) * 100.0
        );
        println!(
            "   (earned by {} consecutive days of practice)",
            streak
        );
    }

    // Recent activity summary
    let recent_count: i64 = conn
        .query_row(
            "SELECT COUNT(DISTINCT topic_id) FROM session_log WHERE timestamp >= datetime('now', '-7 days')",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    let total_sessions: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM session_log WHERE timestamp >= datetime('now', '-7 days')",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);

    if total_sessions > 0 {
        println!();
        println!("{}", "📊 Last 7 days".bold());
        println!(
            "   {} sessions across {} topics",
            total_sessions, recent_count
        );
    }

    println!();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_streak_command_no_activity() {
        let conn = db::init_memory_db().unwrap();
        run(&conn).unwrap();
    }

    #[test]
    fn test_streak_command_with_activity() {
        let conn = db::init_memory_db().unwrap();
        conn.execute(
            "INSERT INTO session_log (topic_id, activity_type, timestamp) VALUES (1, 'learn', datetime('now'))",
            [],
        )
        .unwrap();
        run(&conn).unwrap();
    }
}
