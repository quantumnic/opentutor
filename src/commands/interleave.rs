use colored::*;
use rusqlite::Connection;
use crate::display;
use crate::commands::achievements;
use crate::engine::{adaptive, quiz as quiz_engine, spaced};

/// Interleaved practice: mix questions from different subjects.
///
/// Research (Rohrer & Taylor, 2007; Kornell & Bjork, 2008) shows that
/// interleaving topics during practice produces better long-term retention
/// than blocked practice (studying one topic at a time). The brain learns
/// to discriminate between concepts and apply the right strategy.
pub fn run(conn: &Connection, count: usize) -> Result<(), Box<dyn std::error::Error>> {
    display::print_header("Interleaved Practice");
    println!(
        "  {} questions mixed across subjects for deeper learning\n",
        count.to_string().bold()
    );

    // Strategy: pick topics proportionally from subjects the user has studied,
    // biasing toward topics due for review or with lower scores.
    let mut questions: Vec<quiz_engine::QuizQuestion> = Vec::new();

    // 1) Gather all subjects that have quiz questions
    let mut stmt = conn.prepare(
        "SELECT DISTINCT s.id, s.name
         FROM subjects s
         JOIN topics t ON t.subject_id = s.id
         JOIN quiz_questions q ON q.topic_id = t.id
         ORDER BY RANDOM()"
    )?;
    let subjects: Vec<(i64, String)> = stmt
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
        .collect::<Result<Vec<_>, _>>()?;

    if subjects.is_empty() {
        display::print_info("No quiz questions available yet.");
        return Ok(());
    }

    // 2) For each subject, pick 1-3 questions weighted by due/weak status
    let per_subject = (count / subjects.len()).max(1);
    let mut remaining = count;

    for (subj_id, _subj_name) in &subjects {
        if remaining == 0 {
            break;
        }
        let take = per_subject.min(remaining);

        // Get topics in this subject, prioritizing due/weak ones
        let mut tstmt = conn.prepare(
            "SELECT t.id FROM topics t
             LEFT JOIN user_progress p ON p.topic_id = t.id
             WHERE t.subject_id = ?1
             ORDER BY
               CASE WHEN p.next_review IS NOT NULL AND p.next_review <= datetime('now') THEN 0 ELSE 1 END,
               COALESCE(p.score, 0) ASC,
               RANDOM()
             LIMIT 5"
        )?;
        let topic_ids: Vec<i64> = tstmt
            .query_map([subj_id], |r| r.get(0))?
            .collect::<Result<Vec<_>, _>>()?;

        let mut got = 0;
        for tid in &topic_ids {
            if got >= take {
                break;
            }
            let need = (take - got).min(2); // max 2 per topic to keep variety
            let qs = quiz_engine::get_questions(conn, *tid, need)?;
            got += qs.len();
            questions.extend(qs);
        }
        remaining = remaining.saturating_sub(got);
    }

    // Fill any remaining from random topics
    if remaining > 0 {
        let mut filler = conn.prepare(
            "SELECT t.id FROM topics t
             JOIN quiz_questions q ON q.topic_id = t.id
             GROUP BY t.id
             ORDER BY RANDOM()
             LIMIT ?1"
        )?;
        let fill_topics: Vec<i64> = filler
            .query_map([remaining as i64], |r| r.get(0))?
            .collect::<Result<Vec<_>, _>>()?;
        for tid in &fill_topics {
            if questions.len() >= count {
                break;
            }
            let qs = quiz_engine::get_questions(conn, *tid, 1)?;
            questions.extend(qs);
        }
    }

    // Shuffle to interleave
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    questions.shuffle(&mut rng);
    questions.truncate(count);

    // Sibling burying: avoid consecutive questions from the same topic
    sibling_bury(&mut questions);

    if questions.is_empty() {
        display::print_info("No quiz questions available.");
        return Ok(());
    }

    let total = questions.len();
    let mut correct_count = 0;

    for (i, q) in questions.iter().enumerate() {
        // Show which subject/topic this question belongs to
        let topic_name: String = conn
            .query_row(
                "SELECT s.name || ' → ' || t.name FROM topics t JOIN subjects s ON s.id = t.subject_id WHERE t.id = ?1",
                [q.topic_id],
                |r| r.get(0),
            )
            .unwrap_or_else(|_| "Unknown".to_string());

        println!("  {} {} {}", format!("Q{}.", i + 1).bold().bright_cyan(), format!("[{}]", topic_name).dimmed(), q.question.bold());

        match q.question_type.as_str() {
            "true_false" => {
                println!("     {} True", "T)".dimmed());
                println!("     {} False", "F)".dimmed());
            }
            "fill_in_blank" => {
                println!("     {}", "(Type your answer)".dimmed());
            }
            "ordering" => {
                println!("     {}", "(Put these in the correct order)".dimmed());
                for (j, opt) in q.options.iter().enumerate() {
                    println!("     {} {}", format!("{}.", j + 1).dimmed(), opt);
                }
            }
            _ => {
                for (j, opt) in q.options.iter().enumerate() {
                    let letter = (b'a' + j as u8) as char;
                    println!("     {} {}", format!("{})", letter).dimmed(), opt);
                }
            }
        }

        println!();
        println!("    {} {}", "Answer:".dimmed(), q.correct_answer.bright_green().bold());
        println!("    {} {}", "Why:".dimmed(), q.explanation);
        if let Some(hint) = &q.hint {
            display::print_hint(hint);
        }

        correct_count += 1;
        adaptive::update_progress(conn, q.topic_id, true)?;

        println!();
        display::print_divider();
        println!();
    }

    // Update spaced repetition for each touched topic
    let mut seen_topics: Vec<i64> = questions.iter().map(|q| q.topic_id).collect();
    seen_topics.sort_unstable();
    seen_topics.dedup();
    for tid in &seen_topics {
        spaced::update_spaced_repetition(conn, *tid, 4)?;
        adaptive::log_activity(conn, *tid, "interleave", Some(100.0))?;
    }

    let score = (correct_count as f64 / total as f64) * 100.0;
    display::print_header("Interleaved Practice Results");
    println!(
        "  {} questions across {} topics from {} subjects",
        total.to_string().bold(),
        seen_topics.len().to_string().bold(),
        subjects.len().min(seen_topics.len()).to_string().bold()
    );
    display::print_progress_bar("Score", correct_count as f64, total as f64);
    println!();
    display::print_success("Interleaved practice strengthens long-term retention! 🧠");

    // Check achievements
    if let Ok(newly) = achievements::check_achievements(conn) {
        for name in &newly {
            println!(
                "  🏆 {} {}",
                "ACHIEVEMENT UNLOCKED:".bold().bright_yellow(),
                name.bold().bright_yellow()
            );
        }
    }

    // Log a synthetic score if > 0
    if score > 0.0 {
        adaptive::log_activity(conn, seen_topics[0], "interleave_session", Some(score))?;
    }

    Ok(())
}

/// Sibling burying: rearrange questions so consecutive ones aren't from the
/// same topic. Uses a simple greedy approach — swap adjacent same-topic pairs.
fn sibling_bury(questions: &mut [quiz_engine::QuizQuestion]) {
    if questions.len() < 3 {
        return;
    }
    for i in 1..questions.len() {
        if questions[i].topic_id == questions[i - 1].topic_id {
            // Find a later question with a different topic to swap
            let mut swap_idx = None;
            for j in (i + 1)..questions.len() {
                if questions[j].topic_id != questions[i].topic_id {
                    swap_idx = Some(j);
                    break;
                }
            }
            if let Some(j) = swap_idx {
                questions.swap(i, j);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use crate::engine::quiz::QuizQuestion;

    #[test]
    fn test_interleave_run() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, 5).unwrap();
    }

    #[test]
    fn test_interleave_large() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, 20).unwrap();
    }

    #[test]
    fn test_sibling_bury_basic() {
        let make_q = |tid: i64| QuizQuestion {
            id: tid,
            topic_id: tid,
            question: format!("Q{}", tid),
            question_type: "multiple_choice".into(),
            difficulty: "medium".into(),
            correct_answer: "A".into(),
            options: vec![],
            hint: None,
            explanation: "E".into(),
        };
        let mut qs = vec![make_q(1), make_q(1), make_q(2), make_q(3)];
        sibling_bury(&mut qs);
        // After burying, consecutive questions shouldn't share topic_id
        for w in qs.windows(2) {
            if w[0].topic_id == w[1].topic_id {
                // Only acceptable if no other topic available
                let others: Vec<_> = qs.iter().filter(|q| q.topic_id != w[0].topic_id).collect();
                assert!(others.is_empty(), "Sibling burying failed to separate topics");
            }
        }
    }

    #[test]
    fn test_sibling_bury_empty() {
        let mut qs: Vec<QuizQuestion> = vec![];
        sibling_bury(&mut qs);
        assert!(qs.is_empty());
    }

    #[test]
    fn test_sibling_bury_single() {
        let q = QuizQuestion {
            id: 1,
            topic_id: 1,
            question: "Q".into(),
            question_type: "mc".into(),
            difficulty: "m".into(),
            correct_answer: "A".into(),
            options: vec![],
            hint: None,
            explanation: "E".into(),
        };
        let mut qs = vec![q];
        sibling_bury(&mut qs);
        assert_eq!(qs.len(), 1);
    }
}
