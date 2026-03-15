use colored::*;
use rusqlite::Connection;
use crate::display;
use crate::engine::spaced;

/// Show the most urgent topics to review, ranked by a combination of
/// overdue ratio, current retention estimate, and stability decay.
pub fn run(conn: &Connection, limit: usize) -> Result<(), Box<dyn std::error::Error>> {
    let topics = spaced::prioritized_due_topics(conn)?;

    if topics.is_empty() {
        display::print_info("No topics are due for review. You're all caught up! 🎉");
        return Ok(());
    }

    display::print_header("📋 Review Priorities");
    println!(
        "  Topics ranked by urgency × forgetting risk. Review the top ones first.\n"
    );

    let shown = topics.len().min(limit);

    println!(
        "  {}  {:<30} {:<20} {:>10}  {:>10}  {}",
        "#".dimmed(),
        "Topic".bold(),
        "Subject".dimmed(),
        "Retention".bold(),
        "Priority".bold(),
        "Status".bold()
    );
    println!("  {}", "─".repeat(95).dimmed());

    for (i, (topic_id, name, subject, priority)) in topics.iter().take(shown).enumerate() {
        let retention = spaced::retrievability(conn, *topic_id);
        let decay = spaced::stability_decay_factor(conn, *topic_id);
        let is_leech = spaced::is_leech(conn, *topic_id);

        let ret_pct = (retention * 100.0).round() as u32;
        let ret_display = if ret_pct >= 80 {
            format!("{}%", ret_pct).bright_green()
        } else if ret_pct >= 50 {
            format!("{}%", ret_pct).bright_yellow()
        } else {
            format!("{}%", ret_pct).bright_red()
        };

        let priority_display = format!("{:.1}", priority);
        let priority_colored = if *priority > 5.0 {
            priority_display.bright_red().bold()
        } else if *priority > 2.0 {
            priority_display.bright_yellow()
        } else {
            priority_display.normal()
        };

        let mut status_parts = Vec::new();
        if is_leech {
            status_parts.push("🩸 leech".to_string());
        }
        if decay < 0.8 {
            status_parts.push(format!("📉 decayed ({:.0}%)", decay * 100.0));
        }
        if spaced::is_card_lapsed(conn, *topic_id) {
            status_parts.push("⏰ lapsed".to_string());
        }
        let status = if status_parts.is_empty() {
            "✅ due".to_string()
        } else {
            status_parts.join(", ")
        };

        println!(
            "  {}  {:<30} {:<20} {:>10}  {:>10}  {}",
            format!("{:>2}", i + 1).dimmed(),
            if name.len() > 28 { format!("{}…", &name[..27]) } else { name.clone() },
            if subject.len() > 18 { format!("{}…", &subject[..17]) } else { subject.clone() },
            ret_display,
            priority_colored,
            status,
        );
    }

    println!();

    // Summary stats
    let total_due = topics.len();
    let critical = topics.iter().filter(|(id, _, _, _)| spaced::retrievability(conn, *id) < 0.5).count();
    let lapsed = topics.iter().filter(|(id, _, _, _)| spaced::is_card_lapsed(conn, *id)).count();

    println!(
        "  {} due  •  {} critical (<50% retention)  •  {} lapsed",
        total_due.to_string().bold(),
        if critical > 0 { critical.to_string().bright_red().bold() } else { "0".normal() },
        if lapsed > 0 { lapsed.to_string().bright_yellow().bold() } else { "0".normal() },
    );

    if critical > 3 {
        println!(
            "\n  {} Several topics are at risk of being forgotten. Consider a {} session!",
            "⚠".bright_yellow(),
            "cram".bold()
        );
    }

    println!();
    Ok(())
}
