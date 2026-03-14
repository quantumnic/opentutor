use colored::*;
use rusqlite::Connection;
use crate::display;
use crate::engine::spaced;

/// Show time-of-day performance analysis.
pub fn run(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    display::print_header("Best Study Hours");
    println!("  Performance by time of day (based on your review history)\n");

    let mut stmt = conn.prepare(
        "SELECT hour_bucket, total_reviews, correct_reviews, avg_quality
         FROM time_of_day_stats
         WHERE total_reviews >= 1
         ORDER BY hour_bucket ASC",
    )?;

    let rows: Vec<(i64, i64, i64, f64)> = stmt
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)))?
        .collect::<Result<Vec<_>, _>>()?;

    if rows.is_empty() {
        display::print_info("No study data yet. Complete some reviews to see your best hours!");
        return Ok(());
    }

    let max_reviews = rows.iter().map(|r| r.1).max().unwrap_or(1);

    for (hour, total, correct, avg_q) in &rows {
        let accuracy = if *total > 0 {
            (*correct as f64 / *total as f64) * 100.0
        } else {
            0.0
        };

        let bar_len = ((*total as f64 / max_reviews as f64) * 20.0).round() as usize;
        let bar = "█".repeat(bar_len);

        let hour_label = format!("{:02}:00", hour);
        let quality_indicator = if *avg_q >= 4.0 {
            "🌟".to_string()
        } else if *avg_q >= 3.0 {
            "✓".to_string()
        } else {
            "⚠".to_string()
        };

        println!(
            "  {} {} {} ({} reviews, {:.0}% accuracy, avg quality {:.1})",
            hour_label.bold(),
            bar.bright_green(),
            quality_indicator,
            total,
            accuracy,
            avg_q,
        );
    }

    println!();

    if let Some(best) = spaced::best_study_hour(conn) {
        display::print_success(&format!(
            "Your peak study hour: {:02}:00 — try to schedule reviews around this time!",
            best
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use crate::engine::spaced;

    #[test]
    fn test_best_hours_empty() {
        let conn = db::init_memory_db().unwrap();
        run(&conn).unwrap();
    }

    #[test]
    fn test_best_hours_with_data() {
        let conn = db::init_memory_db().unwrap();
        for _ in 0..5 {
            spaced::record_time_of_day_performance(&conn, 9, 5).unwrap();
        }
        spaced::record_time_of_day_performance(&conn, 14, 3).unwrap();
        spaced::record_time_of_day_performance(&conn, 21, 2).unwrap();
        run(&conn).unwrap();
    }
}
