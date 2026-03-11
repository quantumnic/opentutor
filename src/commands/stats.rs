use colored::*;
use rusqlite::Connection;
use crate::display;

pub fn run(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    display::print_header("Learning Statistics");

    // Overall stats
    let total_topics: i64 = conn.query_row("SELECT COUNT(*) FROM topics", [], |r| r.get(0))?;
    let total_questions: i64 = conn.query_row("SELECT COUNT(*) FROM quiz_questions", [], |r| r.get(0))?;
    let total_lessons: i64 = conn.query_row("SELECT COUNT(*) FROM lessons", [], |r| r.get(0))?;
    let total_subjects: i64 = conn.query_row("SELECT COUNT(*) FROM subjects", [], |r| r.get(0))?;
    let total_sessions: i64 = conn.query_row("SELECT COUNT(*) FROM session_log", [], |r| r.get(0))?;
    let studied: i64 = conn.query_row("SELECT COUNT(*) FROM user_progress", [], |r| r.get(0))?;

    display::print_section("Content Library");
    println!("    📚 {} subjects", total_subjects.to_string().bold());
    println!("    📂 {} topics", total_topics.to_string().bold());
    println!("    📖 {} lessons", total_lessons.to_string().bold());
    println!("    ❓ {} quiz questions", total_questions.to_string().bold());
    println!();

    display::print_section("Your Activity");
    println!("    📝 {} topics studied", studied.to_string().bold());
    println!("    🎯 {} total sessions", total_sessions.to_string().bold());

    if studied > 0 {
        let avg_score: f64 = conn.query_row(
            "SELECT COALESCE(AVG(score), 0) FROM user_progress WHERE score > 0",
            [], |r| r.get(0),
        )?;
        println!("    📊 Average score: {:.1}%", avg_score);

        // Streak info
        let streak = calculate_streak(conn)?;
        if streak > 0 {
            println!("    🔥 Current streak: {} days", streak.to_string().bold().bright_yellow());
        }

        println!();

        // Spaced repetition health
        display::print_section("Spaced Repetition Health");

        let due_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM user_progress WHERE next_review IS NOT NULL AND next_review <= datetime('now')",
            [], |r| r.get(0),
        )?;
        let upcoming_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM user_progress WHERE next_review IS NOT NULL AND next_review > datetime('now') AND next_review <= datetime('now', '+7 days')",
            [], |r| r.get(0),
        )?;
        let mastered: i64 = conn.query_row(
            "SELECT COUNT(*) FROM user_progress WHERE interval_days >= 21 AND ease_factor >= 2.0",
            [], |r| r.get(0),
        )?;
        let lapsed: i64 = conn.query_row(
            "SELECT COUNT(*) FROM user_progress WHERE next_review IS NOT NULL AND next_review <= datetime('now', '-7 days')",
            [], |r| r.get(0),
        )?;

        println!("    📅 Due now: {}", if due_count > 0 { due_count.to_string().bright_red().bold().to_string() } else { "0 ✨".green().to_string() });
        println!("    📆 Due this week: {}", upcoming_count);
        println!("    ⭐ Mastered (21+ day interval): {}", mastered.to_string().bright_green());
        if lapsed > 0 {
            println!("    ⚠️  Lapsed (overdue 7+ days): {}", lapsed.to_string().bright_red());
        }

        println!();

        // Activity by type
        display::print_section("Activity Breakdown");
        let mut stmt = conn.prepare(
            "SELECT activity_type, COUNT(*), COALESCE(AVG(score), 0)
             FROM session_log GROUP BY activity_type ORDER BY COUNT(*) DESC"
        )?;
        let rows = stmt.query_map([], |r| {
            Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?, r.get::<_, f64>(2)?))
        })?;
        for row in rows {
            let (atype, count, avg) = row?;
            let emoji = match atype.as_str() {
                "learn" => "📖",
                "quiz" => "❓",
                "review" => "🔄",
                "explain" => "💡",
                _ => "📝",
            };
            println!("    {} {}: {} sessions (avg {:.0}%)", emoji, atype.bold(), count, avg);
        }

        println!();

        // Weakest topics
        display::print_section("Topics Needing Attention");
        let mut weak_stmt = conn.prepare(
            "SELECT t.name, s.name, p.score, p.ease_factor
             FROM user_progress p
             JOIN topics t ON t.id = p.topic_id
             JOIN subjects s ON s.id = t.subject_id
             WHERE p.score < 70.0 OR p.ease_factor < 2.0
             ORDER BY p.score ASC LIMIT 5"
        )?;
        let weak_rows: Vec<(String, String, f64, f64)> = weak_stmt.query_map([], |r| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?))
        })?.collect::<Result<Vec<_>, _>>()?;

        if weak_rows.is_empty() {
            display::print_success("All studied topics are in good shape!");
        } else {
            for (topic, subject, score, ease) in &weak_rows {
                let health = if *ease < 1.5 { "🔴" } else if *ease < 2.0 { "🟡" } else { "🟢" };
                println!("    {} {} ({}) — {:.0}% score, {:.2} ease",
                    health, topic.bright_yellow(), subject.dimmed(), score, ease);
            }
        }
    } else {
        println!();
        display::print_info("Start learning to see your statistics grow!");
        display::print_info(&format!("Try: {}", "opentutor learn <subject>".bright_cyan()));
    }

    Ok(())
}

fn calculate_streak(conn: &Connection) -> Result<i64, Box<dyn std::error::Error>> {
    // Check how many consecutive days (ending today or yesterday) have activity
    let mut stmt = conn.prepare(
        "SELECT DISTINCT DATE(timestamp) as day FROM session_log ORDER BY day DESC"
    )?;
    let days: Vec<String> = stmt.query_map([], |r| r.get(0))?.collect::<Result<Vec<_>, _>>()?;

    if days.is_empty() {
        return Ok(0);
    }

    let today = chrono::Local::now().date_naive();
    let mut streak = 0i64;
    let mut expected = today;

    for day_str in &days {
        if let Ok(day) = chrono::NaiveDate::parse_from_str(day_str, "%Y-%m-%d") {
            if day == expected {
                streak += 1;
                expected = expected.pred_opt().unwrap_or(expected);
            } else if streak == 0 && day == today.pred_opt().unwrap_or(today) {
                // Allow streak to start from yesterday
                streak += 1;
                expected = day.pred_opt().unwrap_or(day);
            } else {
                break;
            }
        }
    }

    Ok(streak)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use crate::engine::adaptive;

    #[test]
    fn test_stats_empty() {
        let conn = db::init_memory_db().unwrap();
        run(&conn).unwrap();
    }

    #[test]
    fn test_stats_with_data() {
        let conn = db::init_memory_db().unwrap();
        adaptive::update_progress(&conn, 1, true).unwrap();
        adaptive::update_progress(&conn, 1, true).unwrap();
        adaptive::update_progress(&conn, 2, false).unwrap();
        adaptive::log_activity(&conn, 1, "learn", Some(100.0)).unwrap();
        adaptive::log_activity(&conn, 1, "quiz", Some(80.0)).unwrap();
        adaptive::log_activity(&conn, 2, "quiz", Some(40.0)).unwrap();
        run(&conn).unwrap();
    }

    #[test]
    fn test_streak_empty() {
        let conn = db::init_memory_db().unwrap();
        assert_eq!(calculate_streak(&conn).unwrap(), 0);
    }
}
