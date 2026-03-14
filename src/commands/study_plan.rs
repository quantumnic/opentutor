use colored::*;
use rusqlite::Connection;
use crate::display;
use crate::engine::spaced;

/// Generate a personalized study plan based on due reviews, weak areas, and learning paths.
pub fn run(conn: &Connection, days: usize) -> Result<(), Box<dyn std::error::Error>> {
    let days = days.clamp(1, 30);

    display::print_header(&format!("📅 {}-Day Study Plan", days));
    println!();

    // 1. Gather due reviews
    let due_count = spaced::count_due_topics(conn).unwrap_or(0);
    let lapsed_count = spaced::count_lapsed_topics(conn).unwrap_or(0);
    let remaining = spaced::remaining_reviews_today(conn);
    let streak = spaced::calculate_streak(conn);
    let avg_retention = spaced::average_retention(conn);

    // 2. Summary header
    println!("  {} Reviews due: {}, Lapsed: {}, Today's remaining: {}",
        "📊".bold(), due_count.to_string().bold(),
        if lapsed_count > 0 { lapsed_count.to_string().red().bold().to_string() } else { "0".to_string() },
        remaining.to_string().bold());
    println!("  {} Streak: {} days, Avg retention: {:.0}%",
        "🔥".bold(), streak.to_string().bold(),
        avg_retention * 100.0);
    println!();

    // 3. Priority review list (lapsed first, then most urgent)
    if lapsed_count > 0 {
        display::print_info(&format!("⚠️  {} lapsed topics need immediate attention!", lapsed_count));
        println!();
    }

    let prioritized = spaced::prioritized_due_topics(conn).unwrap_or_default();
    if !prioritized.is_empty() {
        println!("  {} {}", "🎯 Priority Reviews".bold().underline(), "(most urgent first)".dimmed());
        for (i, (_, name, subject, priority)) in prioritized.iter().take(10).enumerate() {
            let urgency_label = if *priority > 3.0 {
                "CRITICAL".red().bold().to_string()
            } else if *priority > 1.5 {
                "HIGH".yellow().bold().to_string()
            } else {
                "normal".dimmed().to_string()
            };
            println!("  {}. {} ({}) [{}]",
                (i + 1).to_string().bold(),
                name.bright_cyan(),
                subject.dimmed(),
                urgency_label);
        }
        println!();
    }

    // 4. Weak areas (leech topics)
    let leeches = spaced::get_leeches(conn).unwrap_or_default();
    if !leeches.is_empty() {
        println!("  {} {}", "🔨 Weak Areas".bold().underline(), "(repeatedly failed — need extra focus)".dimmed());
        for (_, name, subject, count) in leeches.iter().take(5) {
            println!("  • {} ({}) — {} lapses",
                name.bright_red(),
                subject.dimmed(),
                count.to_string().bold());
        }
        println!();
    }

    // 5. Unstudied topics (topics with no progress)
    let unstudied: Vec<(i64, String, String)> = {
        let mut stmt = conn.prepare(
            "SELECT t.id, t.name, s.name FROM topics t
             JOIN subjects s ON s.id = t.subject_id
             LEFT JOIN user_progress p ON p.topic_id = t.id
             WHERE p.id IS NULL
             ORDER BY t.subject_id, t.sort_order
             LIMIT 10"
        )?;
        let rows = stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?;
        rows.collect::<Result<Vec<_>, _>>()?
    };

    if !unstudied.is_empty() {
        println!("  {} {}", "🌱 New Topics to Explore".bold().underline(), "(not yet started)".dimmed());
        for (_, name, subject) in unstudied.iter().take(8) {
            println!("  • {} ({})", name.bright_green(), subject.dimmed());
        }
        println!();
    }

    // 6. Daily plan breakdown
    let daily_cap = spaced::get_daily_review_cap(conn);
    let reviews_per_day = (due_count as usize / days).max(1).min(daily_cap);
    let new_per_day = if due_count < daily_cap as i64 { 2 } else { 1 };

    println!("  {} {}", "📋 Recommended Daily Schedule".bold().underline(), format!("({} days)", days).dimmed());
    println!("  • Review {} due topics per day", reviews_per_day.to_string().bold());
    println!("  • Learn {} new topic(s) per day", new_per_day.to_string().bold());
    if lapsed_count > 0 {
        println!("  • {} Tackle lapsed topics {} — they're at risk of being forgotten",
            "❗".bold(), "first".bold().red());
    }

    // 7. Best study time
    if let Some((hour, _quality)) = spaced::optimal_review_window(conn) {
        let period = match hour {
            6..=11 => "morning",
            12..=17 => "afternoon",
            18..=22 => "evening",
            _ => "night",
        };
        println!("  • Your best study time: ~{}:00 ({}) based on past performance",
            format!("{:02}", hour).bold(), period);
    }

    // 8. Estimated time
    let est_minutes = reviews_per_day as f64 * 2.0 + new_per_day as f64 * 5.0;
    println!("  • Estimated daily time: ~{} minutes", (est_minutes as usize).to_string().bold());

    println!();
    display::print_success("Consistency beats intensity! Even 10 minutes daily builds strong retention. 💪");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_study_plan_empty_db() {
        let conn = db::init_memory_db().unwrap();
        // Should not panic on empty progress
        run(&conn, 7).unwrap();
    }

    #[test]
    fn test_study_plan_with_progress() {
        let conn = db::init_memory_db().unwrap();
        // Add some progress
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days, next_review)
             VALUES (1, 80.0, 5, 4, 2.5, 3, datetime('now', '-1 day'))", []
        ).unwrap();
        run(&conn, 7).unwrap();
    }

    #[test]
    fn test_study_plan_clamp_days() {
        let conn = db::init_memory_db().unwrap();
        // 0 days should clamp to 1
        run(&conn, 0).unwrap();
        // 100 days should clamp to 30
        run(&conn, 100).unwrap();
    }
}
