use colored::*;
use rusqlite::Connection;
use crate::display;

pub fn run(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    display::print_header("Subject Comparison");

    let mut stmt = conn.prepare(
        "SELECT s.name,
                COUNT(DISTINCT t.id) as topic_count,
                COUNT(DISTINCT p.topic_id) as studied_count,
                COALESCE(AVG(p.score), 0.0) as avg_score,
                COALESCE(SUM(p.attempts), 0) as total_attempts,
                COALESCE(AVG(p.ease_factor), 2.5) as avg_ease
         FROM subjects s
         LEFT JOIN topics t ON t.subject_id = s.id
         LEFT JOIN user_progress p ON p.topic_id = t.id AND p.attempts > 0
         GROUP BY s.id
         ORDER BY avg_score DESC, studied_count DESC"
    )?;

    let rows: Vec<(String, i64, i64, f64, i64, f64)> = stmt
        .query_map([], |r| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?, r.get(5)?))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    if rows.is_empty() {
        println!("{}", "No subjects found.".yellow());
        return Ok(());
    }

    // Find max values for relative bars
    let max_score = rows.iter().map(|r| r.3).fold(0.0_f64, f64::max);
    let _max_attempts = rows.iter().map(|r| r.4).max().unwrap_or(1).max(1);

    println!();
    for (name, topic_count, studied, avg_score, attempts, avg_ease) in &rows {
        let coverage = if *topic_count > 0 {
            (*studied as f64 / *topic_count as f64 * 100.0) as i64
        } else {
            0
        };

        let score_bar_len = if max_score > 0.0 {
            (avg_score / max_score * 20.0) as usize
        } else {
            0
        };
        let score_bar = "█".repeat(score_bar_len);
        let score_empty = "░".repeat(20 - score_bar_len);

        let strength = if avg_ease >= &2.5 && avg_score >= &70.0 {
            "Strong".green()
        } else if avg_ease >= &2.0 && avg_score >= &40.0 {
            "Growing".yellow()
        } else if *attempts > 0 {
            "Needs Work".red()
        } else {
            "Not Started".dimmed()
        };

        println!("  📘 {}", name.bold());
        println!("     Score: {}{} {:.0}%", score_bar.green(), score_empty.dimmed(), avg_score);
        println!("     Coverage: {}/{} topics ({}%)", studied, topic_count, coverage);
        println!("     Attempts: {}  |  Ease: {:.2}  |  {}", attempts, avg_ease, strength);
        println!();
    }

    // Overall summary
    let total_studied: i64 = rows.iter().map(|r| r.2).sum();
    let total_topics: i64 = rows.iter().map(|r| r.1).sum();
    let overall_avg: f64 = {
        let active: Vec<&(String, i64, i64, f64, i64, f64)> = rows.iter().filter(|r| r.4 > 0).collect();
        if active.is_empty() { 0.0 } else {
            active.iter().map(|r| r.3).sum::<f64>() / active.len() as f64
        }
    };

    println!("{}", "─".repeat(50).dimmed());
    println!("  📊 Studied {}/{} topics across {} subjects",
        total_studied, total_topics, rows.len());
    println!("  🎯 Overall average score: {:.1}%", overall_avg);

    // Recommendation
    let weakest = rows.iter()
        .filter(|r| r.4 > 0 && r.3 < 60.0)
        .min_by(|a, b| a.3.partial_cmp(&b.3).unwrap());
    let unstudied = rows.iter()
        .find(|r| r.2 == 0);

    if let Some(weak) = weakest {
        println!("  💡 Focus area: {} (avg {:.0}%)", weak.0.yellow(), weak.3);
    }
    if let Some(new) = unstudied {
        println!("  🌱 Try something new: {}", new.0.cyan());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_compare_empty_progress() {
        let conn = db::init_memory_db().unwrap();
        assert!(run(&conn).is_ok());
    }

    #[test]
    fn test_compare_with_progress() {
        let conn = db::init_memory_db().unwrap();
        // Add some progress
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days)
             VALUES (1, 85.0, 10, 8, 2.5, 3)",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO user_progress (topic_id, score, attempts, correct, ease_factor, interval_days)
             VALUES (6, 40.0, 5, 2, 1.8, 1)",
            [],
        ).unwrap();
        assert!(run(&conn).is_ok());
    }
}
