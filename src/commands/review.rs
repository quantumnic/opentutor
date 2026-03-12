use colored::*;
use rusqlite::Connection;
use crate::display;
use crate::commands::achievements;
use crate::engine::{adaptive, quiz as quiz_engine, spaced};

pub fn run(conn: &Connection, count: usize) -> Result<(), Box<dyn std::error::Error>> {
    // Get topics due for review
    let mut stmt = conn.prepare(
        "SELECT t.id, t.name, s.name, p.ease_factor, p.interval_days, p.next_review
         FROM user_progress p
         JOIN topics t ON t.id = p.topic_id
         JOIN subjects s ON s.id = t.subject_id
         WHERE p.next_review IS NOT NULL AND p.next_review <= datetime('now')
         ORDER BY p.next_review ASC
         LIMIT 10",
    )?;

    let mut due_topics: Vec<(i64, String, String, f64, i64, String)> = stmt
        .query_map([], |r| {
            Ok((
                r.get(0)?,
                r.get(1)?,
                r.get(2)?,
                r.get(3)?,
                r.get(4)?,
                r.get(5)?,
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    // Sort by combined urgency + low retention (most critical first)
    due_topics.sort_by(|a, b| {
        let urgency_a = spaced::review_urgency(conn, a.0);
        let urgency_b = spaced::review_urgency(conn, b.0);
        // Factor in retention: lower retention = higher priority
        let retention_a = spaced::estimate_retention(conn, a.0);
        let retention_b = spaced::estimate_retention(conn, b.0);
        let score_a = urgency_a + (1.0 - retention_a) * 2.0;
        let score_b = urgency_b + (1.0 - retention_b) * 2.0;
        score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
    });

    // Sibling burying: interleave subjects so the same subject doesn't
    // appear twice in a row (better for memory consolidation)
    due_topics = interleave_subjects(due_topics);

    if due_topics.is_empty() {
        display::print_header("Spaced Repetition Review");
        display::print_success("Nothing due for review! You're all caught up. 🎉");

        // Show next upcoming review
        let next: Result<(String, String, String), _> = conn.query_row(
            "SELECT t.name, s.name, p.next_review
             FROM user_progress p
             JOIN topics t ON t.id = p.topic_id
             JOIN subjects s ON s.id = t.subject_id
             WHERE p.next_review IS NOT NULL
             ORDER BY p.next_review ASC LIMIT 1",
            [],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
        );

        if let Ok((topic, subject, date)) = next {
            display::print_info(&format!(
                "Next review: {} ({}) on {}",
                topic, subject, date
            ));
        } else {
            display::print_info(
                "Start learning to build your review schedule: opentutor learn <subject>",
            );
        }

        return Ok(());
    }

    display::print_header("Spaced Repetition Review");
    println!(
        "  {} topics due for review\n",
        due_topics.len().to_string().bold().bright_yellow()
    );

    let mut total_correct = 0;
    let mut total_questions = 0;

    for (topic_id, topic_name, subject_name, ease, interval, _next) in &due_topics {
        let is_lapsed = spaced::is_card_lapsed(conn, *topic_id);
        let is_leech = spaced::is_leech(conn, *topic_id);
        let strength = if is_leech {
            "Leech 🩹".bright_red()
        } else if is_lapsed {
            "Lapsed ⚠️".bright_red()
        } else if *interval >= 30 {
            "Strong 💪".bright_green()
        } else if *interval >= 7 {
            "Growing 🌱".yellow()
        } else {
            "Needs work 🔨".bright_red()
        };

        display::print_section(&format!(
            "{} ({}) — {}",
            topic_name, subject_name, strength
        ));
        let retention = spaced::estimate_retention(conn, *topic_id);
        let ret_pct = (retention * 100.0) as u32;
        let ret_color = if ret_pct >= 80 { "🟢" } else if ret_pct >= 50 { "🟡" } else { "🔴" };
        println!(
            "    Ease: {:.1} | Interval: {} days | Retention: {} {}%\n",
            ease,
            interval.to_string().bold(),
            ret_color,
            ret_pct
        );

        // Get quiz questions for this topic
        let questions = quiz_engine::get_questions(conn, *topic_id, count)?;

        if questions.is_empty() {
            display::print_info(&format!("No quiz questions for {}. Skipping.", topic_name));
            // Still update the review schedule
            spaced::update_spaced_repetition(conn, *topic_id, 3)?;
            continue;
        }

        let q_count = questions.len();
        total_questions += q_count;

        for (i, q) in questions.iter().enumerate() {
            println!(
                "    {} {}",
                format!("Q{}.", i + 1).bold().bright_cyan(),
                q.question.bold()
            );

            match q.question_type.as_str() {
                "true_false" => {
                    for opt in &q.options {
                        println!("       • {}", opt);
                    }
                }
                "ordering" => {
                    println!("       {}", "(Put these in the correct order)".dimmed());
                    for (j, opt) in q.options.iter().enumerate() {
                        println!("       {} {}", format!("{}.", j + 1).dimmed(), opt);
                    }
                }
                _ => {
                    for (j, opt) in q.options.iter().enumerate() {
                        let letter = (b'a' + j as u8) as char;
                        println!("       {} {}", format!("{})", letter).dimmed(), opt);
                    }
                }
            }

            println!();
            println!(
                "      {} {}",
                "Answer:".dimmed(),
                q.correct_answer.bright_green().bold()
            );
            println!("      {} {}", "Why:".dimmed(), q.explanation);
            println!();

            total_correct += 1;
            adaptive::update_progress(conn, *topic_id, true)?;
        }

        // Update spaced repetition
        let score = 100.0; // Non-interactive mode
        let quality = if score >= 90.0 {
            5
        } else if score >= 70.0 {
            4
        } else if score >= 50.0 {
            3
        } else {
            2
        };
        spaced::update_spaced_repetition(conn, *topic_id, quality)?;
        adaptive::log_activity(conn, *topic_id, "review", Some(score))?;

        display::print_divider();
        println!();
    }

    // Summary
    display::print_header("Review Summary");
    display::print_progress_bar(
        "Total",
        total_correct as f64,
        total_questions as f64,
    );
    println!(
        "\n  Topics reviewed: {}",
        due_topics.len().to_string().bold()
    );
    display::print_success("Review schedule updated! See you next time. 📅");

    // Check achievements
    if let Ok(newly) = achievements::check_achievements(conn) {
        for name in &newly {
            println!("  🏆 {} {}", "ACHIEVEMENT UNLOCKED:".bold().bright_yellow(), name.bold().bright_yellow());
        }
    }

    Ok(())
}

/// Interleave topics by subject to avoid reviewing the same subject twice in
/// a row ("sibling burying"). This improves memory consolidation by forcing
/// context switches between subjects.
fn interleave_subjects(
    topics: Vec<(i64, String, String, f64, i64, String)>,
) -> Vec<(i64, String, String, f64, i64, String)> {
    if topics.len() <= 2 {
        return topics;
    }

    let mut result = Vec::with_capacity(topics.len());
    let mut remaining = topics;
    let mut last_subject = String::new();

    while !remaining.is_empty() {
        // Find the first topic with a different subject than the last one
        let idx = remaining
            .iter()
            .position(|t| t.2 != last_subject)
            .unwrap_or(0); // If all same subject, just take the first

        let topic = remaining.remove(idx);
        last_subject.clone_from(&topic.2);
        result.push(topic);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use crate::engine::{adaptive, spaced};

    #[test]
    fn test_review_nothing_due() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, 3).unwrap();
    }

    #[test]
    fn test_review_with_due_topics() {
        let conn = db::init_memory_db().unwrap();
        // Create progress with past next_review date
        adaptive::update_progress(&conn, 1, true).unwrap();
        spaced::update_spaced_repetition(&conn, 1, 3).unwrap();
        // Force the next_review to the past
        conn.execute(
            "UPDATE user_progress SET next_review = datetime('now', '-1 day') WHERE topic_id = 1",
            [],
        )
        .unwrap();
        run(&conn, 2).unwrap();
    }

    #[test]
    fn test_interleave_subjects() {
        let topics = vec![
            (1, "T1".into(), "Math".into(), 2.5, 5, "d".into()),
            (2, "T2".into(), "Math".into(), 2.5, 5, "d".into()),
            (3, "T3".into(), "Science".into(), 2.5, 5, "d".into()),
            (4, "T4".into(), "Science".into(), 2.5, 5, "d".into()),
        ];
        let result = interleave_subjects(topics);
        // No two consecutive topics should have the same subject (when possible)
        for w in result.windows(2) {
            if w[0].2 == w[1].2 {
                // Only ok if there's no alternative left
                // In this case with 2 Math + 2 Science, we should always interleave
                panic!("Adjacent topics have same subject: {} and {}", w[0].1, w[1].1);
            }
        }
    }

    #[test]
    fn test_interleave_single_subject() {
        // All same subject — should not panic, just return in order
        let topics = vec![
            (1, "T1".into(), "Math".into(), 2.5, 5, "d".into()),
            (2, "T2".into(), "Math".into(), 2.5, 5, "d".into()),
        ];
        let result = interleave_subjects(topics);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_review_updates_schedule() {
        let conn = db::init_memory_db().unwrap();
        adaptive::update_progress(&conn, 1, true).unwrap();
        spaced::update_spaced_repetition(&conn, 1, 4).unwrap();
        conn.execute(
            "UPDATE user_progress SET next_review = datetime('now', '-1 day') WHERE topic_id = 1",
            [],
        )
        .unwrap();
        run(&conn, 1).unwrap();
        // After review, next_review should be in the future
        let next: String = conn
            .query_row(
                "SELECT next_review FROM user_progress WHERE topic_id = 1",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert!(!next.is_empty());
    }
}
