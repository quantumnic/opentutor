use colored::*;
use rusqlite::Connection;
use crate::commands::config;
use crate::display;
use crate::engine::spaced;

/// Personalized study recommendations based on retention curves, leeches, and coverage gaps.
pub fn run(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    display::print_header("Personalized Recommendations");

    let studied: i64 = conn.query_row(
        "SELECT COUNT(*) FROM user_progress", [], |r| r.get(0)
    )?;

    if studied == 0 {
        display::print_section("🚀 Getting Started");
        println!("    You haven't studied anything yet! Here are some great starting points:\n");
        let mut stmt = conn.prepare(
            "SELECT s.name, s.description FROM subjects s ORDER BY RANDOM() LIMIT 5"
        )?;
        let rows: Vec<(String, String)> = stmt.query_map([], |r| {
            Ok((r.get(0)?, r.get(1)?))
        })?.collect::<Result<Vec<_>, _>>()?;
        for (name, desc) in &rows {
            println!("    📘 {} — {}", name.bold().bright_white(), desc.dimmed());
        }
        println!();
        display::print_info(&format!("Start with: {}", "opentutor learn <subject>".bright_cyan()));
        return Ok(());
    }

    // 1. Leeches — topics with high consecutive_fails
    display::print_section("🩹 Leech Cards (Struggling Topics)");
    let mut leech_stmt = conn.prepare(
        "SELECT t.name, s.name, p.leech_count, p.consecutive_fails, p.ease_factor
         FROM user_progress p
         JOIN topics t ON t.id = p.topic_id
         JOIN subjects s ON s.id = t.subject_id
         WHERE p.leech_count > 0 OR p.consecutive_fails >= 2
         ORDER BY p.consecutive_fails DESC, p.leech_count DESC
         LIMIT 5"
    )?;
    let leeches: Vec<(String, String, i64, i64, f64)> = leech_stmt.query_map([], |r| {
        Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?))
    })?.collect::<Result<Vec<_>, _>>()?;

    if leeches.is_empty() {
        display::print_success("No leech cards! You're learning effectively.");
    } else {
        println!("    These topics keep tripping you up — try re-reading the lessons:\n");
        for (topic, subject, leech_count, fails, ease) in &leeches {
            let severity = if *leech_count >= 3 { "🔴" } else if *leech_count >= 1 { "🟡" } else { "🟠" };
            println!("    {} {} ({}) — {} leeches, {} consecutive fails, ease {:.2}",
                severity, topic.bold().bright_yellow(), subject.dimmed(),
                leech_count, fails, ease);
        }
        println!();
        display::print_hint("Leech cards often mean the material needs a different approach. Try 'opentutor explain <concept>'.");
    }
    println!();

    // 2. Lowest retention — topics about to be forgotten
    display::print_section("🧠 Fading Memories (Low Retention)");
    let mut ret_stmt = conn.prepare(
        "SELECT p.topic_id, t.name, s.name
         FROM user_progress p
         JOIN topics t ON t.id = p.topic_id
         JOIN subjects s ON s.id = t.subject_id
         WHERE p.last_reviewed IS NOT NULL
         ORDER BY p.ease_factor ASC, p.interval_days ASC
         LIMIT 10"
    )?;
    let retention_topics: Vec<(i64, String, String)> = ret_stmt.query_map([], |r| {
        Ok((r.get(0)?, r.get(1)?, r.get(2)?))
    })?.collect::<Result<Vec<_>, _>>()?;

    let desired_ret = config::get_desired_retention(conn);
    let low_threshold = (desired_ret - 0.15).max(0.3); // flag topics well below target

    let mut low_retention = Vec::new();
    for (tid, tname, sname) in &retention_topics {
        let ret = spaced::estimate_retention(conn, *tid);
        if ret < low_threshold {
            low_retention.push((tid, tname, sname, ret));
        }
    }

    if low_retention.is_empty() {
        display::print_success(&format!(
            "All studied topics have good retention (≥{}%)!",
            (low_threshold * 100.0).round()
        ));
    } else {
        for (_, tname, sname, ret) in &low_retention {
            let pct = (*ret * 100.0) as u32;
            let indicator = if pct < 30 { "🔴" } else { "🟡" };
            println!("    {} {} ({}) — {}% retention",
                indicator, tname.bold(), sname.dimmed(), pct);
        }
        println!();
        display::print_hint(&format!("Review these now: {}", "opentutor review".bright_cyan()));
    }
    println!();

    // 3. Coverage gaps — subjects never touched
    display::print_section("🗺️  Unexplored Subjects");
    let mut gap_stmt = conn.prepare(
        "SELECT s.name, s.description, COUNT(t.id) as topic_count
         FROM subjects s
         JOIN topics t ON t.subject_id = s.id
         WHERE s.id NOT IN (
             SELECT DISTINCT t2.subject_id FROM user_progress p
             JOIN topics t2 ON t2.id = p.topic_id
         )
         GROUP BY s.id
         ORDER BY RANDOM()"
    )?;
    let gaps: Vec<(String, String, i64)> = gap_stmt.query_map([], |r| {
        Ok((r.get(0)?, r.get(1)?, r.get(2)?))
    })?.collect::<Result<Vec<_>, _>>()?;

    if gaps.is_empty() {
        display::print_success("You've explored every subject! 🏆");
    } else {
        for (name, desc, count) in &gaps {
            println!("    🌟 {} ({} topics) — {}",
                name.bold().bright_white(), count, desc.dimmed());
        }
        println!();
        display::print_hint("Broaden your knowledge by trying a new subject!");
    }
    println!();

    // 4. Next steps for partially completed subjects
    display::print_section("📈 Continue Your Progress");
    let mut next_stmt = conn.prepare(
        "SELECT t.name, s.name, t.difficulty
         FROM topics t
         JOIN subjects s ON s.id = t.subject_id
         WHERE t.subject_id IN (
             SELECT DISTINCT t2.subject_id FROM user_progress p
             JOIN topics t2 ON t2.id = p.topic_id
         )
         AND t.id NOT IN (SELECT topic_id FROM user_progress)
         ORDER BY t.sort_order ASC
         LIMIT 5"
    )?;
    let next_topics: Vec<(String, String, String)> = next_stmt.query_map([], |r| {
        Ok((r.get(0)?, r.get(1)?, r.get(2)?))
    })?.collect::<Result<Vec<_>, _>>()?;

    if next_topics.is_empty() {
        display::print_success("You've covered all topics in your active subjects!");
    } else {
        for (tname, sname, diff) in &next_topics {
            let badge = match diff.as_str() {
                "intermediate" => "🟡",
                "advanced" => "🔴",
                _ => "🟢",
            };
            println!("    {} {} ({}) [{}]",
                badge, tname.bold(), sname.dimmed(), diff);
        }
    }
    println!();

    // Summary
    display::print_divider();
    let total_topics: i64 = conn.query_row("SELECT COUNT(*) FROM topics", [], |r| r.get(0))?;
    display::print_progress_bar("Overall coverage", studied as f64, total_topics as f64);
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use crate::engine::adaptive;

    #[test]
    fn test_recommend_empty() {
        let conn = db::init_memory_db().unwrap();
        run(&conn).unwrap();
    }

    #[test]
    fn test_recommend_with_progress() {
        let conn = db::init_memory_db().unwrap();
        adaptive::update_progress(&conn, 1, true).unwrap();
        adaptive::update_progress(&conn, 6, true).unwrap();
        adaptive::log_activity(&conn, 1, "learn", Some(100.0)).unwrap();
        run(&conn).unwrap();
    }

    #[test]
    fn test_recommend_with_leeches() {
        let conn = db::init_memory_db().unwrap();
        adaptive::update_progress(&conn, 1, true).unwrap();
        // Simulate a leech
        conn.execute(
            "UPDATE user_progress SET leech_count = 3, consecutive_fails = 4, ease_factor = 1.3 WHERE topic_id = 1",
            [],
        ).unwrap();
        run(&conn).unwrap();
    }
}
