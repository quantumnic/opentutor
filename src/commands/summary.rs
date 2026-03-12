use colored::*;
use rusqlite::Connection;
use crate::display;
use crate::engine::spaced;

/// Quick summary of current learning state — one screen, at a glance.
pub fn run(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    display::print_header("Learning Summary");

    let total_topics: i64 = conn.query_row("SELECT COUNT(*) FROM topics", [], |r| r.get(0))?;
    let studied: i64 = conn.query_row("SELECT COUNT(*) FROM user_progress", [], |r| r.get(0))?;
    let due = spaced::count_due_topics(conn)?;
    let lapsed = spaced::count_lapsed_topics(conn)?;
    let streak = spaced::calculate_streak(conn);
    let avg_retention = spaced::average_retention(conn);
    let leeches = spaced::get_leeches(conn)?;

    // Streak
    if streak > 0 {
        let flame = "🔥".repeat(streak.min(7) as usize);
        println!("  {} {} day streak", flame, streak.to_string().bold().bright_yellow());
    }
    println!();

    // Coverage
    display::print_progress_bar("Coverage", studied as f64, total_topics as f64);
    println!();

    // Retention
    if studied > 0 {
        let retention_pct = (avg_retention * 100.0).round();
        let retention_color = if retention_pct >= 85.0 {
            format!("{}%", retention_pct).bright_green()
        } else if retention_pct >= 60.0 {
            format!("{}%", retention_pct).bright_yellow()
        } else {
            format!("{}%", retention_pct).bright_red()
        };
        println!("  🧠 Average retention: {}", retention_color);
    }

    // Due / lapsed
    if due > 0 || lapsed > 0 {
        println!();
        display::print_section("Reviews");
        if due > 0 {
            println!("    📋 {} topics due for review", due.to_string().bold());
        }
        if lapsed > 0 {
            println!("    ⚠️  {} topics lapsed (overdue >7 days)", lapsed.to_string().bold().bright_red());
        }
    }

    // Leeches
    if !leeches.is_empty() {
        println!();
        display::print_section("Leeches 🧛");
        for (_, name, subject, count) in leeches.iter().take(3) {
            println!("    {} ({}) — {} failures", name.bright_red(), subject.dimmed(), count);
        }
        if leeches.len() > 3 {
            println!("    ... and {} more", leeches.len() - 3);
        }
    }

    // Suggestion
    println!();
    if due > 0 {
        display::print_hint(&format!("Run {} to review due topics.", "opentutor review".bright_cyan()));
    } else if studied < total_topics {
        display::print_hint(&format!("Run {} to learn something new!", "opentutor subjects".bright_cyan()));
    } else {
        display::print_success("All caught up! Great work. 🎉");
    }

    println!();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_summary_runs_empty() {
        let conn = db::init_memory_db().unwrap();
        run(&conn).unwrap();
    }

    #[test]
    fn test_summary_with_progress() {
        let conn = db::init_memory_db().unwrap();
        // Add some progress
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days, last_reviewed, next_review)
             VALUES (1, 80.0, 5, 4, 2.5, 7, datetime('now', '-3 days'), datetime('now', '-1 day'))",
            [],
        ).unwrap();
        run(&conn).unwrap();
    }
}
