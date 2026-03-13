use rusqlite::Connection;

use crate::engine::spaced;

/// Show the optimal study focus based on forgetting curve analysis.
/// Identifies topics at the steepest part of their forgetting curve — where
/// a review would have the highest impact on long-term retention.
pub fn run(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Focus — What to Study Right Now\n");

    // Get all topics with progress
    let mut stmt = conn.prepare(
        "SELECT t.id, t.name, s.name, p.ease_factor, p.interval_days, p.last_reviewed
         FROM user_progress p
         JOIN topics t ON t.id = p.topic_id
         JOIN subjects s ON s.id = t.subject_id
         WHERE p.last_reviewed IS NOT NULL
         ORDER BY p.next_review ASC",
    )?;

    let topics: Vec<(i64, String, String, f64, i64, String)> = stmt
        .query_map([], |r| {
            Ok((
                r.get(0)?,
                r.get(1)?,
                r.get(2)?,
                r.get(3)?,
                r.get(4)?,
                r.get::<_, String>(5)?,
            ))
        })?
        .filter_map(|r| r.ok())
        .collect();

    if topics.is_empty() {
        println!("  No study history yet. Start with `opentutor learn <subject>` to begin!\n");
        return Ok(());
    }

    // Calculate review impact score for each topic
    // Impact = how much retention we'd gain by reviewing now vs. waiting
    let mut scored: Vec<(String, String, f64, f64, &'static str)> = topics
        .iter()
        .map(|(id, name, subj, ease, interval, _last)| {
            let retention = spaced::estimate_retention(conn, *id);
            let urgency = spaced::review_urgency(conn, *id);
            let half_life = spaced::stability_half_life(*interval, *ease);

            // Impact score: reviews are most valuable when retention is between 0.5-0.85
            // (below 0.5 = nearly forgotten, above 0.85 = still fresh, 0.6-0.8 = sweet spot)
            let retention_impact = if retention < 0.3 {
                0.9 // Nearly forgotten — urgent but some value lost
            } else if retention < 0.6 {
                1.0 // Maximum impact zone — catching before it's gone
            } else if retention < 0.85 {
                0.7 // Good time to reinforce
            } else {
                0.2 // Still fresh — reviewing now has low marginal value
            };

            let impact = retention_impact * (1.0 + urgency * 0.5) / (half_life.max(1.0).sqrt());

            let status = if retention < 0.3 {
                "🔴 Fading fast"
            } else if retention < 0.6 {
                "🟡 Review now"
            } else if retention < 0.85 {
                "🟢 Good shape"
            } else {
                "✨ Fresh"
            };

            (name.clone(), subj.clone(), retention, impact, status)
        })
        .collect();

    scored.sort_by(|a, b| b.3.partial_cmp(&a.3).unwrap_or(std::cmp::Ordering::Equal));

    // Show top recommendations
    let remaining = spaced::remaining_reviews_today(conn);
    let avg_ret = spaced::average_retention(conn);

    println!("  📊 Overall retention: {:.0}% | Reviews remaining today: {}\n", avg_ret * 100.0, remaining);

    let show_count = scored.len().min(10);
    println!("  Top {} study priorities:\n", show_count);
    println!("  {:<30} {:<20} {:>10}  Status", "Topic", "Subject", "Retention");
    println!("  {}", "─".repeat(78));

    for (name, subj, retention, _impact, status) in scored.iter().take(show_count) {
        let subj_short = if subj.len() > 18 { &subj[..18] } else { subj };
        println!(
            "  {:<30} {:<20} {:>9.0}%  {}",
            if name.len() > 28 { &name[..28] } else { name },
            subj_short,
            retention * 100.0,
            status
        );
    }

    // Actionable suggestion
    if let Some((name, _subj, retention, _impact, _status)) = scored.first() {
        println!();
        if *retention < 0.5 {
            println!("  💡 \"{}\" is fading — review it now for maximum impact!", name);
        } else if *retention < 0.8 {
            println!("  💡 \"{}\" is at the sweet spot for reinforcement. Review it to lock it in!", name);
        } else {
            println!("  ✅ Your top topics are in great shape! Consider learning something new.");
        }
    }

    println!();
    Ok(())
}
