use colored::*;
use rusqlite::Connection;
use crate::display;

/// Show a study activity heatmap for the last N weeks (like GitHub contributions).
pub fn run(conn: &Connection, weeks: usize) -> Result<(), Box<dyn std::error::Error>> {
    let weeks = weeks.clamp(1, 52);
    let days = weeks * 7;

    display::print_header("Study Activity Heatmap");
    println!("  Last {} weeks of activity\n", weeks.to_string().bold());

    // Fetch daily activity counts
    let mut stmt = conn.prepare(
        "SELECT DATE(timestamp) as day, COUNT(*) as cnt
         FROM session_log
         WHERE timestamp >= DATE('now', ?1)
         GROUP BY day
         ORDER BY day ASC",
    )?;
    let offset = format!("-{} days", days);
    let rows: Vec<(String, i64)> = stmt
        .query_map([&offset], |r| Ok((r.get(0)?, r.get(1)?)))?
        .collect::<Result<Vec<_>, _>>()?;

    let mut day_counts = std::collections::HashMap::new();
    let mut max_count: i64 = 0;
    for (day, cnt) in &rows {
        day_counts.insert(day.clone(), *cnt);
        if *cnt > max_count {
            max_count = *cnt;
        }
    }

    // Calculate total stats
    let total_sessions: i64 = rows.iter().map(|(_, c)| c).sum();
    let active_days = rows.len();
    let streak = calculate_current_streak(conn);

    // Build the heatmap grid (columns = weeks, rows = days of week)
    // We need to figure out the start date
    let start_date: String = conn
        .query_row(
            "SELECT DATE('now', ?1)",
            [&format!("-{} days", days - 1)],
            |r| r.get(0),
        )?;

    // Parse start date to get day of week offset
    let start_dow = day_of_week_from_date(conn, &start_date)?;

    let day_labels = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

    // Print month headers
    print!("       "); // indent for day labels
    let mut month_positions = Vec::new();
    for w in 0..weeks {
        let day_offset = w * 7;
        let date = date_plus_days(conn, &start_date, day_offset as i64)?;
        let month = &date[5..7];
        let month_name = match month {
            "01" => "Jan",
            "02" => "Feb",
            "03" => "Mar",
            "04" => "Apr",
            "05" => "May",
            "06" => "Jun",
            "07" => "Jul",
            "08" => "Aug",
            "09" => "Sep",
            "10" => "Oct",
            "11" => "Nov",
            "12" => "Dec",
            _ => "???",
        };
        if month_positions.last().map(|m: &String| m.as_str()) != Some(month_name) {
            month_positions.push(month_name.to_string());
            print!("{}", month_name);
        } else {
            print!("  ");
        }
    }
    println!();

    // Print each row (day of week)
    for (dow, label) in day_labels.iter().enumerate() {
        print!("  {} ", label.dimmed());
        for w in 0..weeks {
            let day_offset = w as i64 * 7 + (dow as i64 - start_dow as i64 + 7) % 7;
            if day_offset < 0 || day_offset >= days as i64 {
                print!("  ");
                continue;
            }
            let date = date_plus_days(conn, &start_date, day_offset);
            let count = date
                .ok()
                .and_then(|d| day_counts.get(&d).copied())
                .unwrap_or(0);

            let block = intensity_block(count, max_count);
            print!("{} ", block);
        }
        println!();
    }

    println!();

    // Legend
    print!("  {} ", "Less".dimmed());
    print!("{} ", "░".dimmed());
    print!("{} ", "▒".bright_green());
    print!("{} ", "▓".green());
    print!("{} ", "█".bright_green().bold());
    println!("{}", "More".dimmed());
    println!();

    // Stats
    println!(
        "  {} {} sessions across {} active days",
        "Total:".bold(),
        total_sessions.to_string().bright_cyan(),
        active_days.to_string().bright_cyan(),
    );
    if active_days > 0 {
        let avg = total_sessions as f64 / active_days as f64;
        println!(
            "  {} {:.1} sessions per active day",
            "Average:".bold(),
            avg,
        );
    }
    if streak > 0 {
        println!(
            "  {} {} days 🔥",
            "Current streak:".bold(),
            streak.to_string().bright_yellow(),
        );
    }

    Ok(())
}

fn intensity_block(count: i64, max: i64) -> ColoredString {
    if count == 0 {
        return "░".dimmed();
    }
    let max = max.max(1);
    let ratio = count as f64 / max as f64;
    if ratio > 0.75 {
        "█".bright_green().bold()
    } else if ratio > 0.4 {
        "▓".green()
    } else {
        "▒".bright_green()
    }
}

fn day_of_week_from_date(conn: &Connection, date: &str) -> Result<usize, rusqlite::Error> {
    // SQLite strftime('%w') returns 0=Sunday, 1=Monday, ..., 6=Saturday
    // We want 0=Monday, ..., 6=Sunday
    let dow: i64 = conn.query_row(
        "SELECT CAST(strftime('%w', ?1) AS INTEGER)",
        [date],
        |r| r.get(0),
    )?;
    // Convert: Sunday(0)->6, Monday(1)->0, ..., Saturday(6)->5
    Ok(((dow + 6) % 7) as usize)
}

fn date_plus_days(conn: &Connection, base: &str, offset: i64) -> Result<String, rusqlite::Error> {
    conn.query_row(
        "SELECT DATE(?1, ?2)",
        rusqlite::params![base, format!("+{} days", offset)],
        |r| r.get(0),
    )
}

fn calculate_current_streak(conn: &Connection) -> i64 {
    let mut streak: i64 = 0;
    let mut day = 0i64;
    loop {
        let offset = format!("-{} days", day);
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM session_log WHERE DATE(timestamp) = DATE('now', ?1)",
                [&offset],
                |r| r.get(0),
            )
            .unwrap_or(0);
        if count > 0 {
            streak += 1;
            day += 1;
        } else if day == 0 {
            // Today might not have activity yet — check yesterday
            day += 1;
        } else {
            break;
        }
    }
    streak
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_heatmap_empty() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, 4).unwrap();
    }

    #[test]
    fn test_heatmap_with_data() {
        let conn = db::init_memory_db().unwrap();
        // Insert some activity
        for i in 0..10 {
            conn.execute(
                "INSERT INTO session_log (topic_id, activity_type, score, timestamp)
                 VALUES (1, 'quiz', 80.0, datetime('now', ?1))",
                [format!("-{} days", i)],
            ).unwrap();
        }
        run(&conn, 4).unwrap();
    }

    #[test]
    fn test_heatmap_single_week() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, 1).unwrap();
    }

    #[test]
    fn test_heatmap_full_year() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, 52).unwrap();
    }

    #[test]
    fn test_intensity_block() {
        assert_eq!(intensity_block(0, 10).to_string(), "░");
        // Non-zero should not be the empty block
        assert_ne!(intensity_block(5, 10).to_string(), "░");
    }

    #[test]
    fn test_streak_no_data() {
        let conn = db::init_memory_db().unwrap();
        let s = calculate_current_streak(&conn);
        assert_eq!(s, 0);
    }
}
