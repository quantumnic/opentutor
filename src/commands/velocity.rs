use colored::*;
use rusqlite::Connection;
use crate::display;
use crate::engine::spaced;

/// Learning velocity: track how fast the user is acquiring and retaining knowledge.
/// Shows topics mastered per week, retention trend, and predicted mastery dates.
pub fn run(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    display::print_header("Learning Velocity 📈");

    // Weekly topic acquisition rate (last 4 weeks)
    let weeks = weekly_acquisition(conn)?;
    if weeks.is_empty() {
        display::print_info("No learning history yet. Start with: opentutor learn <subject>");
        return Ok(());
    }

    display::print_section("Topics Studied Per Week");
    println!();
    for (week, count) in &weeks {
        let bar_len = (*count as usize).min(40);
        let bar = "█".repeat(bar_len);
        println!("    {} {} {}",
            week.dimmed(),
            bar.bright_cyan(),
            count.to_string().bold());
    }
    println!();

    // Current velocity (topics per day, last 7 days)
    let daily_velocity = daily_velocity_7d(conn)?;
    println!("  Current pace: {} topics/day (last 7 days)",
        format!("{:.1}", daily_velocity).bold().bright_green());

    // Retention trend: compare average retention now vs 2 weeks ago
    let current_retention = spaced::average_retention(conn);
    let retention_pct = (current_retention * 100.0) as u32;
    let retention_indicator = if retention_pct >= 85 { "🟢" } else if retention_pct >= 70 { "🟡" } else { "🔴" };
    println!("  Average retention: {} {}%",
        retention_indicator,
        retention_pct.to_string().bold());

    // Mastery forecast: at current pace, when will all topics be studied?
    let total_topics: i64 = conn.query_row(
        "SELECT COUNT(*) FROM topics", [], |r| r.get(0)
    )?;
    let studied: i64 = conn.query_row(
        "SELECT COUNT(*) FROM user_progress WHERE attempts > 0", [], |r| r.get(0)
    )?;
    let remaining = total_topics - studied;

    if remaining > 0 && daily_velocity > 0.0 {
        let days_to_complete = remaining as f64 / daily_velocity;
        println!("  Remaining topics: {} / {}",
            remaining.to_string().bright_yellow(),
            total_topics.to_string().dimmed());
        if days_to_complete < 365.0 {
            println!("  Estimated completion: ~{} days at current pace",
                format!("{:.0}", days_to_complete).bold());
        } else {
            println!("  Estimated completion: {}+ days — try increasing your daily pace!",
                format!("{:.0}", days_to_complete).bold().bright_red());
        }
    } else if remaining == 0 {
        display::print_success("All topics studied! Focus on retention now. 🏆");
    }

    // Strong vs weak subject breakdown
    println!();
    display::print_section("Subject Mastery");
    println!();

    let mut stmt = conn.prepare(
        "SELECT s.name,
                COUNT(DISTINCT p.topic_id) as studied,
                COUNT(DISTINCT t.id) as total,
                AVG(p.ease_factor) as avg_ease
         FROM subjects s
         JOIN topics t ON t.subject_id = s.id
         LEFT JOIN user_progress p ON p.topic_id = t.id AND p.attempts > 0
         GROUP BY s.id
         HAVING studied > 0
         ORDER BY CAST(studied AS REAL) / total DESC"
    )?;

    let subject_stats: Vec<(String, i64, i64, f64)> = stmt
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get::<_, Option<f64>>(3)?.unwrap_or(2.5))))?
        .collect::<Result<Vec<_>, _>>()?;

    for (name, studied, total, avg_ease) in &subject_stats {
        let pct = (*studied as f64 / *total as f64 * 100.0) as u32;
        let ease_indicator = if *avg_ease >= 2.5 { "💪" } else if *avg_ease >= 2.0 { "📖" } else { "🔨" };
        let bar_len = (pct as usize / 5).min(20);
        let bar = "█".repeat(bar_len);
        let empty = "░".repeat(20 - bar_len);
        println!("    {} {}{} {}% ({}/{}) {} ease {:.1}",
            name.bold(),
            bar.bright_green(),
            empty.dimmed(),
            pct,
            studied,
            total,
            ease_indicator,
            avg_ease);
    }

    println!();
    display::print_hint("Tip: Consistent daily practice beats cramming! 📅");

    Ok(())
}

/// Get weekly topic acquisition counts for the last 4 weeks.
fn weekly_acquisition(conn: &Connection) -> Result<Vec<(String, i64)>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT strftime('%Y-W%W', timestamp) as week, COUNT(DISTINCT topic_id)
         FROM session_log
         WHERE timestamp >= datetime('now', '-28 days')
         GROUP BY week
         ORDER BY week ASC"
    )?;
    let rows = stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?;
    rows.collect()
}

/// Compute daily learning velocity over the last 7 days.
fn daily_velocity_7d(conn: &Connection) -> Result<f64, rusqlite::Error> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(DISTINCT topic_id) FROM session_log
         WHERE timestamp >= datetime('now', '-7 days')",
        [],
        |r| r.get(0),
    )?;
    Ok(count as f64 / 7.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use crate::engine::adaptive;

    #[test]
    fn test_velocity_empty() {
        let conn = db::init_memory_db().unwrap();
        run(&conn).unwrap();
    }

    #[test]
    fn test_velocity_with_history() {
        let conn = db::init_memory_db().unwrap();
        adaptive::log_activity(&conn, 1, "learn", Some(100.0)).unwrap();
        adaptive::log_activity(&conn, 2, "learn", Some(80.0)).unwrap();
        adaptive::log_activity(&conn, 3, "quiz", Some(90.0)).unwrap();
        run(&conn).unwrap();
    }

    #[test]
    fn test_weekly_acquisition_empty() {
        let conn = db::init_memory_db().unwrap();
        let weeks = weekly_acquisition(&conn).unwrap();
        assert!(weeks.is_empty());
    }

    #[test]
    fn test_daily_velocity() {
        let conn = db::init_memory_db().unwrap();
        let v = daily_velocity_7d(&conn).unwrap();
        assert!((v - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_daily_velocity_with_data() {
        let conn = db::init_memory_db().unwrap();
        adaptive::log_activity(&conn, 1, "learn", Some(100.0)).unwrap();
        adaptive::log_activity(&conn, 2, "learn", Some(100.0)).unwrap();
        let v = daily_velocity_7d(&conn).unwrap();
        assert!(v > 0.0, "Should have positive velocity, got {}", v);
    }
}
