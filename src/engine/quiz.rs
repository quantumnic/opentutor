use rand::seq::SliceRandom;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuizQuestion {
    pub id: i64,
    pub topic_id: i64,
    pub question: String,
    pub question_type: String,
    pub correct_answer: String,
    pub options: Vec<String>,
    pub hint: Option<String>,
    pub explanation: String,
}

/// Fetch quiz questions for a topic, shuffled.
pub fn get_questions(
    conn: &Connection,
    topic_id: i64,
    count: usize,
) -> Result<Vec<QuizQuestion>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, topic_id, question, question_type, correct_answer,
                option_a, option_b, option_c, option_d, hint, explanation
         FROM quiz_questions WHERE topic_id = ?1",
    )?;
    let mut questions: Vec<QuizQuestion> = stmt
        .query_map([topic_id], |r| {
            let options: Vec<String> = [
                r.get::<_, Option<String>>(5)?,
                r.get::<_, Option<String>>(6)?,
                r.get::<_, Option<String>>(7)?,
                r.get::<_, Option<String>>(8)?,
            ]
            .into_iter()
            .flatten()
            .collect();
            Ok(QuizQuestion {
                id: r.get(0)?,
                topic_id: r.get(1)?,
                question: r.get(2)?,
                question_type: r.get(3)?,
                correct_answer: r.get(4)?,
                options,
                hint: r.get(9)?,
                explanation: r.get(10)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    let mut rng = rand::thread_rng();
    questions.shuffle(&mut rng);
    questions.truncate(count);

    // Shuffle options for each question
    for q in &mut questions {
        q.options.shuffle(&mut rng);
    }

    Ok(questions)
}

/// Check if an answer is correct (case-insensitive, trimmed).
#[allow(dead_code)]
pub fn check_answer(question: &QuizQuestion, answer: &str) -> bool {
    let answer = answer.trim().to_lowercase();
    let correct = question.correct_answer.trim().to_lowercase();

    if answer == correct {
        return true;
    }

    // For true/false, accept t/f shortcuts
    if question.question_type == "true_false" {
        return match answer.as_str() {
            "t" => correct == "true",
            "f" => correct == "false",
            _ => false,
        };
    }

    // Also check by option letter (a, b, c, d)
    if let Some(idx) = match answer.as_str() {
        "a" => Some(0),
        "b" => Some(1),
        "c" => Some(2),
        "d" => Some(3),
        _ => None,
    } {
        if let Some(opt) = question.options.get(idx) {
            return opt.trim().to_lowercase() == correct;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_get_questions() {
        let conn = db::init_memory_db().unwrap();
        let qs = get_questions(&conn, 1, 3).unwrap();
        assert!(!qs.is_empty());
        assert!(qs.len() <= 3);
    }

    #[test]
    fn test_check_answer_exact() {
        let q = QuizQuestion {
            id: 1, topic_id: 1,
            question: "Test?".into(),
            question_type: "multiple_choice".into(),
            correct_answer: "42".into(),
            options: vec!["10".into(), "42".into(), "50".into(), "100".into()],
            hint: None,
            explanation: "The answer is 42.".into(),
        };
        assert!(check_answer(&q, "42"));
        assert!(check_answer(&q, "  42  "));
        assert!(!check_answer(&q, "43"));
    }

    #[test]
    fn test_check_answer_by_letter() {
        let q = QuizQuestion {
            id: 1, topic_id: 1,
            question: "Test?".into(),
            question_type: "multiple_choice".into(),
            correct_answer: "Paris".into(),
            options: vec!["London".into(), "Paris".into(), "Berlin".into(), "Rome".into()],
            hint: None,
            explanation: "Paris is the capital.".into(),
        };
        assert!(check_answer(&q, "b")); // Paris is at index 1
        assert!(check_answer(&q, "B"));
        assert!(check_answer(&q, "paris"));
    }

    #[test]
    fn test_check_answer_true_false() {
        let q = QuizQuestion {
            id: 1, topic_id: 1,
            question: "True or false: The sky is blue.".into(),
            question_type: "true_false".into(),
            correct_answer: "true".into(),
            options: vec!["true".into(), "false".into()],
            hint: None,
            explanation: "Yes, scattering.".into(),
        };
        assert!(check_answer(&q, "true"));
        assert!(check_answer(&q, "True"));
        assert!(check_answer(&q, "t"));
        assert!(!check_answer(&q, "false"));
        assert!(!check_answer(&q, "f"));
    }

    #[test]
    fn test_get_questions_empty_topic() {
        let conn = db::init_memory_db().unwrap();
        let qs = get_questions(&conn, 9999, 5).unwrap();
        assert!(qs.is_empty());
    }
}
