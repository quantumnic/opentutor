use colored::*;
use rusqlite::Connection;
use crate::display;
use crate::engine::spaced;

/// Show performance trends over recent sessions grouped by day.
pub fn run(conn: &Connection, days: usize) -> Result<(), Box<dyn std::error::Error>> {
    display::print_header("Performance Trend");

    let days_i = days as i64;
    let mut stmt = conn.prepare(
        "SELECT DATE(timestamp) as day,
                COUNT(*) as sessions,
                COALESCE(AVG(score), 0) as avg_score,
                SUM(CASE WHEN score >= 80 THEN 1 ELSE 0 END) as good,
                SUM(CASE WHEN score < 50 THEN 1 ELSE 0 END) as poor
         FROM session_log
         WHERE timestamp >= datetime('now', ?1)
         GROUP BY day
         ORDER BY day ASC",
    )?;

    let offset = format!("-{} days", days_i);
    let rows: Vec<(String, i64, f64, i64, i64)> = stmt
        .query_map([&offset], |r| {
            Ok((
                r.get(0)?,
                r.get(1)?,
                r.get(2)?,
                r.get(3)?,
                r.get(4)?,
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    if rows.is_empty() {
        display::print_info(&format!(
            "No activity in the last {} days. Start learning to build your trend!",
            days
        ));
        return Ok(());
    }

    println!(
        "  Showing last {} days ({} active days)\n",
        days.to_string().bold(),
        rows.len().to_string().bold().bright_cyan()
    );

    // Table header
    println!(
        "  {}  {:>8}  {:>9}  {:>6}  {:>6}  {}",
        "Date".bold(),
        "Sessions".bold(),
        "Avg Score".bold(),
        "Good".bold(),
        "Poor".bold(),
        "Trend".bold()
    );
    println!("  {}", "─".repeat(60));

    let mut prev_avg: Option<f64> = None;
    let mut total_sessions = 0i64;
    let mut total_score_sum = 0.0f64;
    let mut total_days = 0usize;

    for (day, sessions, avg_score, good, poor) in &rows {
        let trend_arrow = match prev_avg {
            Some(prev) if *avg_score > prev + 5.0 => "📈".to_string(),
            Some(prev) if *avg_score < prev - 5.0 => "📉".to_string(),
            Some(_) => "➡️".to_string(),
            None => "  ".to_string(),
        };

        let score_color = if *avg_score >= 80.0 {
            format!("{:>8.1}%", avg_score).bright_green()
        } else if *avg_score >= 50.0 {
            format!("{:>8.1}%", avg_score).yellow()
        } else {
            format!("{:>8.1}%", avg_score).bright_red()
        };

        let bar_len = (avg_score / 5.0) as usize;
        let bar = "█".repeat(bar_len.min(20));
        let bar_color = if *avg_score >= 80.0 {
            bar.bright_green()
        } else if *avg_score >= 50.0 {
            bar.yellow()
        } else {
            bar.bright_red()
        };

        println!(
            "  {}  {:>8}  {}  {:>6}  {:>6}  {} {}",
            day.dimmed(),
            sessions,
            score_color,
            good.to_string().bright_green(),
            if *poor > 0 {
                poor.to_string().bright_red().to_string()
            } else {
                poor.to_string()
            },
            trend_arrow,
            bar_color,
        );

        prev_avg = Some(*avg_score);
        total_sessions += sessions;
        total_score_sum += avg_score;
        total_days += 1;
    }

    println!("  {}", "─".repeat(60));

    // Summary
    let overall_avg = if total_days > 0 {
        total_score_sum / total_days as f64
    } else {
        0.0
    };

    println!();
    display::print_section("Summary");
    println!(
        "    Total sessions: {}",
        total_sessions.to_string().bold()
    );
    println!(
        "    Overall average: {:.1}%",
        overall_avg
    );

    // Momentum indicator
    if rows.len() >= 3 {
        let recent: f64 = rows.iter().rev().take(3).map(|r| r.2).sum::<f64>() / 3.0;
        let earlier: f64 = rows.iter().take(3).map(|r| r.2).sum::<f64>() / 3.0;
        let diff = recent - earlier;

        if diff > 5.0 {
            println!(
                "    Momentum: {} Improving! (+{:.1}%)",
                "🚀".to_string().bold(),
                diff
            );
        } else if diff < -5.0 {
            println!(
                "    Momentum: {} Declining ({:.1}%)",
                "⚠️".to_string().bold(),
                diff
            );
        } else {
            println!(
                "    Momentum: {} Steady",
                "✅".to_string().bold()
            );
        }
    }

    // Spaced repetition health snapshot
    let due_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM user_progress WHERE next_review IS NOT NULL AND next_review <= datetime('now')",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);

    let avg_ease: f64 = conn
        .query_row(
            "SELECT COALESCE(AVG(ease_factor), 2.5) FROM user_progress WHERE attempts > 0",
            [],
            |r| r.get(0),
        )
        .unwrap_or(2.5);

    println!();
    display::print_section("SRS Health");
    println!("    Reviews due: {}", if due_count > 0 {
        due_count.to_string().bright_yellow().to_string()
    } else {
        "0 ✨".green().to_string()
    });
    println!("    Average ease factor: {:.2}", avg_ease);

    // Show per-topic momentum for active topics
    let mut topic_stmt = conn.prepare(
        "SELECT DISTINCT sl.topic_id, t.name
         FROM session_log sl
         JOIN topics t ON t.id = sl.topic_id
         WHERE sl.activity_type IN ('quiz', 'review')
         AND sl.timestamp >= datetime('now', ?1)
         ORDER BY t.name",
    )?;
    let active_topics: Vec<(i64, String)> = topic_stmt
        .query_map([&offset], |r| Ok((r.get(0)?, r.get(1)?)))?
        .collect::<Result<Vec<_>, _>>()?;

    if !active_topics.is_empty() {
        let mut momentum_entries: Vec<(String, f64)> = active_topics
            .iter()
            .filter_map(|(tid, name)| {
                let m = spaced::learning_momentum(conn, *tid);
                if m.abs() > 0.5 {
                    Some((name.clone(), m))
                } else {
                    None
                }
            })
            .collect();

        if !momentum_entries.is_empty() {
            momentum_entries.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            println!();
            display::print_section("Topic Momentum");
            for (name, m) in &momentum_entries {
                let indicator = if *m > 5.0 {
                    "🚀"
                } else if *m > 0.0 {
                    "📈"
                } else if *m > -5.0 {
                    "📉"
                } else {
                    "⚠️"
                };
                println!("    {} {} ({:+.1}/session)", indicator, name.bold(), m);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use crate::engine::adaptive;

    #[test]
    fn test_trend_empty() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, 14).unwrap();
    }

    #[test]
    fn test_trend_with_data() {
        let conn = db::init_memory_db().unwrap();
        adaptive::log_activity(&conn, 1, "quiz", Some(85.0)).unwrap();
        adaptive::log_activity(&conn, 1, "quiz", Some(70.0)).unwrap();
        adaptive::log_activity(&conn, 2, "learn", Some(100.0)).unwrap();
        run(&conn, 7).unwrap();
    }

    #[test]
    fn test_trend_custom_days() {
        let conn = db::init_memory_db().unwrap();
        adaptive::log_activity(&conn, 1, "quiz", Some(90.0)).unwrap();
        run(&conn, 30).unwrap();
    }
}
