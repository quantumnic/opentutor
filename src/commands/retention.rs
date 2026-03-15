use colored::Colorize;
use rusqlite::Connection;

use crate::engine::spaced;

pub fn run(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    let reports = spaced::retention_report(conn)?;

    if reports.is_empty() {
        println!(
            "{}",
            "No study data yet! Start learning with `opentutor learn <subject>`.".yellow()
        );
        return Ok(());
    }

    println!("{}", "╔══════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║              📊 Retention Analysis Report                       ║".cyan());
    println!("{}", "╚══════════════════════════════════════════════════════════════════╝".cyan());
    println!();

    let mut current_subject = String::new();

    let mut total_ret = 0.0;
    let mut critical_count = 0;
    let mut fading_count = 0;

    for report in &reports {
        if report.subject_name != current_subject {
            current_subject = report.subject_name.clone();
            println!("  {} {}", "▸".cyan(), current_subject.bold());
        }

        let ret_pct = (report.current_retention * 100.0).round();
        let target_pct = (report.target_retention * 100.0).round();
        let bar_len = (ret_pct / 5.0).round() as usize;
        let bar: String = "█".repeat(bar_len.min(20));
        let empty: String = "░".repeat(20_usize.saturating_sub(bar_len));

        let colored_bar = if ret_pct >= target_pct {
            format!("{}", bar.green())
        } else if ret_pct >= target_pct - 15.0 {
            format!("{}", bar.yellow())
        } else {
            format!("{}", bar.red())
        };

        println!(
            "    {:<28} {}{} {:>3}% (target {:>3}%) {} | stab {:.0}d | last {}d ago",
            report.topic_name,
            colored_bar,
            empty.dimmed(),
            ret_pct,
            target_pct,
            report.status,
            report.stability_days,
            report.days_since_review,
        );

        total_ret += report.current_retention;
        match report.status {
            spaced::RetentionStatus::Critical => critical_count += 1,
            spaced::RetentionStatus::Fading => fading_count += 1,
            _ => {}
        }
    }

    let avg_ret = if !reports.is_empty() {
        total_ret / reports.len() as f64 * 100.0
    } else {
        0.0
    };

    println!();
    println!("{}", "─".repeat(66).dimmed());
    println!(
        "  {} topics tracked | avg retention: {:.0}% | {} critical | {} fading",
        reports.len().to_string().bold(),
        avg_ret,
        critical_count.to_string().red().bold(),
        fading_count.to_string().yellow().bold(),
    );

    if critical_count > 0 {
        println!();
        println!(
            "  💡 Review critical topics soon with: {}",
            "opentutor cram".cyan()
        );
    }

    println!();
    Ok(())
}
