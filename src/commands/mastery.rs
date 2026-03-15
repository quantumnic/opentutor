use colored::*;
use rusqlite::Connection;
use crate::display;
use crate::engine::spaced;

pub fn run(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    display::print_header("Mastery Overview");

    // Show mastery summary
    let summary = spaced::mastery_summary(conn)?;
    if summary.is_empty() {
        display::print_info("No topics found. Start learning with 'opentutor learn <subject>'!");
        return Ok(());
    }

    println!("  {}", "Mastery Distribution".bold());
    println!();
    let total: i64 = summary.iter().map(|(_, c)| c).sum();
    for (level, count) in &summary {
        let pct = (*count as f64 / total as f64) * 100.0;
        let bar_len = ((pct / 100.0) * 30.0).round() as usize;
        let bar = "█".repeat(bar_len);
        let pad = "░".repeat(30 - bar_len);
        println!(
            "  {} {:<12} {} {} ({}, {:.0}%)",
            level.emoji(),
            level.as_str(),
            bar.bright_green(),
            pad.dimmed(),
            count,
            pct
        );
    }
    println!();

    // Show per-subject mastery breakdown
    let mut stmt = conn.prepare(
        "SELECT s.id, s.name FROM subjects s ORDER BY s.name",
    )?;
    let subjects: Vec<(i64, String)> = stmt
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    println!("  {}", "Per-Subject Breakdown".bold());
    println!();

    for (subj_id, subj_name) in &subjects {
        let mut topic_stmt = conn.prepare(
            "SELECT t.id FROM topics t WHERE t.subject_id = ?1",
        )?;
        let topic_ids: Vec<i64> = topic_stmt
            .query_map([subj_id], |r| r.get(0))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();

        if topic_ids.is_empty() {
            continue;
        }

        let mut level_counts = std::collections::HashMap::new();
        for tid in &topic_ids {
            let level = spaced::assess_mastery(conn, *tid);
            *level_counts.entry(level).or_insert(0u32) += 1;
        }

        let total_topics = topic_ids.len();
        let mastered = level_counts.get(&spaced::MasteryLevel::Mastered).copied().unwrap_or(0)
            + level_counts.get(&spaced::MasteryLevel::Expert).copied().unwrap_or(0);
        let learning = level_counts.get(&spaced::MasteryLevel::Learning).copied().unwrap_or(0)
            + level_counts.get(&spaced::MasteryLevel::Developing).copied().unwrap_or(0);
        let new_count = level_counts.get(&spaced::MasteryLevel::New).copied().unwrap_or(0);

        let mastery_pct = mastered as f64 / total_topics as f64 * 100.0;
        let indicator = if mastery_pct >= 80.0 {
            "⭐".to_string()
        } else if mastery_pct >= 50.0 {
            "🌿".to_string()
        } else if learning > 0 {
            "📖".to_string()
        } else {
            "🌱".to_string()
        };

        println!(
            "  {} {:<28} {:>2}/{} mastered  {:>2} learning  {:>2} new",
            indicator,
            subj_name.bold(),
            mastered,
            total_topics,
            learning,
            new_count
        );
    }

    println!();
    display::print_info("Use 'opentutor weak' to see which topics need the most work.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_mastery_command_runs() {
        let conn = db::init_memory_db().unwrap();
        run(&conn).unwrap();
    }

    #[test]
    fn test_mastery_with_progress() {
        let conn = db::init_memory_db().unwrap();
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days, consecutive_fails, leech_count)
             VALUES (1, 95.0, 10, 9, 2.5, 90, 0, 0)",
            [],
        ).unwrap();
        run(&conn).unwrap();
    }
}
