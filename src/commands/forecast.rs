use colored::*;
use rusqlite::Connection;
use crate::display;
use crate::engine::spaced;

/// Show a 7-day review forecast: how many topics are due each day.
pub fn run(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    display::print_header("Review Forecast (Next 7 Days)");

    let studied: i64 = conn.query_row(
        "SELECT COUNT(*) FROM user_progress WHERE next_review IS NOT NULL", [], |r| r.get(0)
    )?;

    if studied == 0 {
        display::print_info("No review schedule yet. Start learning to build one!");
        display::print_info(&format!("Try: {}", "opentutor learn <subject>".bright_cyan()));
        return Ok(());
    }

    // Overdue
    let overdue: i64 = conn.query_row(
        "SELECT COUNT(*) FROM user_progress WHERE next_review <= datetime('now')",
        [], |r| r.get(0)
    )?;

    if overdue > 0 {
        let bar_len = overdue.min(30) as usize;
        println!("  {} {} {}",
            "Overdue".bright_red().bold(),
            "█".repeat(bar_len).bright_red(),
            format!("{} topics", overdue).bold().bright_red()
        );
    }

    // Next 7 days
    let mut max_count: i64 = overdue;
    let mut daily_counts = Vec::new();

    for day in 0..7 {
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM user_progress
             WHERE next_review > datetime('now', ?1)
               AND next_review <= datetime('now', ?2)",
            rusqlite::params![
                format!("+{} days", day),
                format!("+{} days", day + 1)
            ],
            |r| r.get(0),
        )?;
        daily_counts.push((day, count));
        if count > max_count {
            max_count = count;
        }
    }

    let scale = if max_count > 0 { 30.0 / max_count as f64 } else { 1.0 };

    let day_names = ["Today", "Tomorrow", "Day 3", "Day 4", "Day 5", "Day 6", "Day 7"];
    println!();

    for (day, count) in &daily_counts {
        let bar_len = ((*count as f64) * scale).round() as usize;
        let bar = "█".repeat(bar_len);
        let label = day_names[*day as usize];

        let colored_bar = if *count == 0 {
            bar.dimmed().to_string()
        } else if *count <= 3 {
            bar.bright_green().to_string()
        } else if *count <= 6 {
            bar.yellow().to_string()
        } else {
            bar.bright_red().to_string()
        };

        let count_str = if *count == 0 {
            "—".dimmed().to_string()
        } else {
            format!("{} topics", count).to_string()
        };

        println!("  {:>10} {} {}",
            if *day == 0 { label.bold().to_string() } else { label.to_string() },
            colored_bar,
            count_str
        );
    }

    println!();
    display::print_divider();

    // Summary stats
    let total_upcoming: i64 = daily_counts.iter().map(|(_, c)| c).sum();
    let total_reviews = overdue + total_upcoming;
    let daily_avg = if total_reviews > 0 { total_reviews as f64 / 7.0 } else { 0.0 };

    println!();
    println!("  Total reviews this week: {}", total_reviews.to_string().bold());
    println!("  Daily average: {:.1} topics/day", daily_avg);

    if overdue > 0 {
        println!();
        display::print_hint(&format!(
            "You have {} overdue topics! Run: {}",
            overdue, "opentutor review".bright_cyan()
        ));
    }

    // Show retention snapshot
    println!();
    display::print_section("Memory Retention Snapshot");
    let mut ret_stmt = conn.prepare(
        "SELECT p.topic_id, t.name FROM user_progress p
         JOIN topics t ON t.id = p.topic_id
         ORDER BY p.ease_factor ASC
         LIMIT 5"
    )?;
    let weakest: Vec<(i64, String)> = ret_stmt.query_map([], |r| {
        Ok((r.get(0)?, r.get(1)?))
    })?.collect::<Result<Vec<_>, _>>()?;

    if !weakest.is_empty() {
        for (topic_id, topic_name) in &weakest {
            let retention = spaced::estimate_retention(conn, *topic_id);
            let pct = (retention * 100.0) as u32;
            let indicator = if pct >= 80 {
                "🟢"
            } else if pct >= 50 {
                "🟡"
            } else {
                "🔴"
            };
            println!("    {} {} — {}% retention",
                indicator, topic_name, pct);
        }
    }

    println!();
    display::print_info("Review consistently to keep retention high! 📈");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use crate::engine::{adaptive, spaced};

    #[test]
    fn test_forecast_empty() {
        let conn = db::init_memory_db().unwrap();
        run(&conn).unwrap();
    }

    #[test]
    fn test_forecast_with_scheduled_reviews() {
        let conn = db::init_memory_db().unwrap();
        // Set up some topics with various next_review dates
        adaptive::update_progress(&conn, 1, true).unwrap();
        spaced::update_spaced_repetition(&conn, 1, 4).unwrap();
        adaptive::update_progress(&conn, 2, true).unwrap();
        spaced::update_spaced_repetition(&conn, 2, 3).unwrap();
        adaptive::update_progress(&conn, 6, true).unwrap();
        spaced::update_spaced_repetition(&conn, 6, 5).unwrap();
        run(&conn).unwrap();
    }

    #[test]
    fn test_forecast_with_overdue() {
        let conn = db::init_memory_db().unwrap();
        adaptive::update_progress(&conn, 1, true).unwrap();
        spaced::update_spaced_repetition(&conn, 1, 4).unwrap();
        conn.execute(
            "UPDATE user_progress SET next_review = datetime('now', '-2 days') WHERE topic_id = 1",
            [],
        ).unwrap();
        run(&conn).unwrap();
    }
}
