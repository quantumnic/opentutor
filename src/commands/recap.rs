use colored::Colorize;
use rusqlite::Connection;

/// Show a recap of recently learned material — what you studied, how you did,
/// and key concepts to reinforce.
pub fn run(conn: &Connection, days: usize) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "📖 Learning Recap".bold().cyan());
    println!("{}", "─".repeat(50));

    // Get recent sessions
    let mut stmt = conn.prepare(
        "SELECT sl.topic_id, t.name, s.name, sl.activity_type, sl.score,
                sl.timestamp, sl.duration_seconds
         FROM session_log sl
         JOIN topics t ON t.id = sl.topic_id
         JOIN subjects s ON s.id = t.subject_id
         WHERE sl.timestamp >= datetime('now', ?1)
         ORDER BY sl.timestamp DESC",
    )?;
    let offset = format!("-{} days", days);
    let rows: Vec<(i64, String, String, String, Option<f64>, String, Option<i64>)> = stmt
        .query_map([&offset], |r| {
            Ok((
                r.get(0)?,
                r.get(1)?,
                r.get(2)?,
                r.get(3)?,
                r.get(4)?,
                r.get(5)?,
                r.get(6)?,
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    if rows.is_empty() {
        println!(
            "\n{}",
            format!("No activity in the last {} day(s). Time to study! 📚", days).yellow()
        );
        return Ok(());
    }

    // Aggregate by subject
    let mut subject_stats: std::collections::HashMap<String, (usize, f64, usize)> =
        std::collections::HashMap::new();
    let mut topic_scores: std::collections::HashMap<String, Vec<f64>> =
        std::collections::HashMap::new();

    for (_tid, topic, subject, _activity, score, _ts, duration) in &rows {
        let entry = subject_stats
            .entry(subject.clone())
            .or_insert((0, 0.0, 0));
        entry.0 += 1; // sessions
        if let Some(s) = score {
            entry.1 += s;
            entry.2 += 1;
        }
        if let Some(s) = score {
            topic_scores.entry(topic.clone()).or_default().push(*s);
        }
        let _ = duration; // might use later
    }

    println!(
        "\n📊 {} session(s) across {} subject(s) in the last {} day(s)\n",
        rows.len(),
        subject_stats.len(),
        days
    );

    // Subject breakdown
    println!("{}", "Subjects studied:".bold());
    let mut subjects: Vec<_> = subject_stats.iter().collect();
    subjects.sort_by(|a, b| b.1 .0.cmp(&a.1 .0));
    for (subject, (sessions, total_score, scored)) in &subjects {
        let avg = if *scored > 0 {
            format!("{:.0}%", total_score / *scored as f64)
        } else {
            "—".to_string()
        };
        let bar_len = (*sessions).min(20);
        let bar: String = "█".repeat(bar_len);
        println!(
            "  {} {:<25} {} sessions  avg: {}",
            bar.green(),
            subject,
            sessions,
            avg.bold()
        );
    }

    // Strongest & weakest topics
    println!("\n{}", "Topic highlights:".bold());
    let mut scored_topics: Vec<(String, f64)> = topic_scores
        .iter()
        .filter(|(_, scores)| !scores.is_empty())
        .map(|(name, scores)| {
            let avg = scores.iter().sum::<f64>() / scores.len() as f64;
            (name.clone(), avg)
        })
        .collect();
    scored_topics.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    if let Some((name, score)) = scored_topics.first() {
        println!("  🌟 Strongest: {} ({:.0}%)", name.green(), score);
    }
    if scored_topics.len() > 1 {
        if let Some((name, score)) = scored_topics.last() {
            println!("  🔧 Needs work: {} ({:.0}%)", name.red(), score);
        }
    }

    // Upcoming reviews
    let due_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM user_progress WHERE next_review <= datetime('now')",
        [],
        |r| r.get(0),
    )?;
    if due_count > 0 {
        println!(
            "\n⏰ {} topic(s) due for review — run {} to stay sharp!",
            due_count.to_string().yellow(),
            "opentutor review".bold()
        );
    }

    // Motivational streak info
    let streak_days: i64 = conn
        .query_row(
            "SELECT COUNT(DISTINCT date(timestamp)) FROM session_log
             WHERE timestamp >= datetime('now', '-7 days')",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    if streak_days >= 3 {
        println!(
            "\n🔥 {} active day(s) this week — keep the momentum!",
            streak_days.to_string().bold()
        );
    }

    println!();
    Ok(())
}
