use rand::seq::SliceRandom;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuizQuestion {
    pub id: i64,
    pub topic_id: i64,
    pub question: String,
    pub question_type: String,
    pub difficulty: String,
    pub correct_answer: String,
    pub options: Vec<String>,
    pub hint: Option<String>,
    pub explanation: String,
}

/// Fetch quiz questions for a topic with weighted selection.
/// Questions the user has struggled with (lower scores) are prioritized.
pub fn get_questions(
    conn: &Connection,
    topic_id: i64,
    count: usize,
) -> Result<Vec<QuizQuestion>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, topic_id, question, question_type, correct_answer,
                option_a, option_b, option_c, option_d, hint, explanation, difficulty
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
                difficulty: r.get::<_, Option<String>>(11)?.unwrap_or_else(|| "medium".to_string()),
                correct_answer: r.get(4)?,
                options,
                hint: r.get(9)?,
                explanation: r.get(10)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    let mut rng = rand::thread_rng();

    // Weighted selection: prioritize questions based on user performance
    // Check if user has progress for this topic
    let has_progress: bool = conn
        .query_row(
            "SELECT COUNT(*) > 0 FROM user_progress WHERE topic_id = ?1 AND attempts >= 2",
            [topic_id],
            |r| r.get(0),
        )
        .unwrap_or(false);

    if has_progress && questions.len() > count {
        // Assign weights based on question type AND user's ease factor
        // Topics with lower ease factor get more weight (user is struggling)
        let ease: f64 = conn
            .query_row(
                "SELECT COALESCE(ease_factor, 2.5) FROM user_progress WHERE topic_id = ?1",
                [topic_id],
                |r| r.get(0),
            )
            .unwrap_or(2.5);

        // Lower ease → higher difficulty multiplier (range ~1.0 – 2.0)
        let difficulty_mult = (3.0 - ease).clamp(1.0, 2.0);

        let weights: Vec<f64> = questions
            .iter()
            .map(|q| {
                let type_weight = match q.question_type.as_str() {
                    "fill_in_blank" => 1.5 * difficulty_mult,
                    "true_false" => 0.8,
                    _ => 1.0 * difficulty_mult,
                };
                // Adjust weight based on question difficulty vs user level
                let diff_weight = match q.difficulty.as_str() {
                    "easy" => 1.0 / difficulty_mult, // Easier Qs less likely when user is strong
                    "hard" => difficulty_mult,        // Harder Qs more likely when user struggles
                    _ => 1.0,                         // Medium stays neutral
                };
                type_weight * diff_weight
            })
            .collect();

        // Weighted shuffle: Fisher-Yates with weights
        let mut indices: Vec<usize> = (0..questions.len()).collect();
        let mut weighted_indices = Vec::with_capacity(count);
        let mut remaining_weights = weights.clone();

        for _ in 0..count.min(indices.len()) {
            let total: f64 = remaining_weights.iter().sum();
            if total <= 0.0 {
                break;
            }
            let mut r = rand::random::<f64>() * total;
            let mut chosen = 0;
            for (i, w) in remaining_weights.iter().enumerate() {
                r -= w;
                if r <= 0.0 {
                    chosen = i;
                    break;
                }
            }
            weighted_indices.push(indices[chosen]);
            indices.remove(chosen);
            remaining_weights.remove(chosen);
        }

        questions = weighted_indices.into_iter().map(|i| questions[i].clone()).collect();
    } else {
        questions.shuffle(&mut rng);
        questions.truncate(count);
    }

    // Shuffle options for each question
    for q in &mut questions {
        q.options.shuffle(&mut rng);
    }

    Ok(questions)
}

/// Fetch quiz questions for a topic with optional difficulty filter.
/// Delegates to get_questions after filtering.
pub fn get_questions_filtered(
    conn: &Connection,
    topic_id: i64,
    count: usize,
    difficulty: Option<&str>,
) -> Result<Vec<QuizQuestion>, rusqlite::Error> {
    match difficulty {
        Some(diff) => {
            let diff = diff.to_lowercase();
            let mut stmt = conn.prepare(
                "SELECT id, topic_id, question, question_type, correct_answer,
                        option_a, option_b, option_c, option_d, hint, explanation, difficulty
                 FROM quiz_questions WHERE topic_id = ?1 AND LOWER(difficulty) = ?2",
            )?;
            let mut questions: Vec<QuizQuestion> = stmt
                .query_map(rusqlite::params![topic_id, diff], |r| {
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
                        difficulty: r.get::<_, Option<String>>(11)?.unwrap_or_else(|| "medium".to_string()),
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
            for q in &mut questions {
                q.options.shuffle(&mut rng);
            }
            Ok(questions)
        }
        None => get_questions(conn, topic_id, count),
    }
}

/// Compute Levenshtein edit distance between two strings.
#[allow(dead_code)]
fn levenshtein(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let (m, n) = (a_chars.len(), b_chars.len());
    let mut prev = (0..=n).collect::<Vec<usize>>();
    let mut curr = vec![0usize; n + 1];
    for i in 1..=m {
        curr[0] = i;
        for j in 1..=n {
            let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
            curr[j] = (prev[j] + 1)
                .min(curr[j - 1] + 1)
                .min(prev[j - 1] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[n]
}

/// Accept fuzzy matches for fill-in-the-blank answers.
/// For short answers (≤4 chars): exact match only.
/// For longer answers: allow 1 edit per 5 characters (max 2 edits).
#[allow(dead_code)]
fn fuzzy_match(answer: &str, correct: &str) -> bool {
    if answer == correct {
        return true;
    }
    let max_len = answer.len().max(correct.len());
    if max_len <= 4 {
        return false; // Short answers must be exact
    }
    let allowed = (max_len / 5).clamp(1, 2);
    levenshtein(answer, correct) <= allowed
}

#[allow(dead_code)]
pub fn check_answer(question: &QuizQuestion, answer: &str) -> bool {
    let answer = answer.trim().to_lowercase();
    let correct = question.correct_answer.trim().to_lowercase();

    // For ordering questions, the correct_answer is a comma-separated sequence
    // and the user's answer should match the same sequence.
    if question.question_type == "ordering" {
        return check_ordering_answer(&correct, &answer);
    }

    // For matching questions, the correct_answer is a semicolon-separated list of
    // "left=right" pairs. The user's answer should contain the same pairs in any order.
    if question.question_type == "matching" {
        return check_matching_answer(&correct, &answer);
    }

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

    // For fill-in-the-blank, accept fuzzy matches (small typos)
    if question.question_type == "fill_in_blank" {
        return fuzzy_match(&answer, &correct);
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

/// Check matching answers: the correct answer and user answer are semicolon-separated
/// "left=right" pairs. Order doesn't matter, but all pairs must match.
#[allow(dead_code)]
fn check_matching_answer(correct: &str, answer: &str) -> bool {
    let mut correct_pairs: Vec<(String, String)> = correct
        .split(';')
        .filter_map(|pair| {
            let parts: Vec<&str> = pair.splitn(2, '=').collect();
            if parts.len() == 2 {
                Some((parts[0].trim().to_lowercase(), parts[1].trim().to_lowercase()))
            } else {
                None
            }
        })
        .collect();
    let mut answer_pairs: Vec<(String, String)> = answer
        .split(';')
        .filter_map(|pair| {
            let parts: Vec<&str> = pair.splitn(2, '=').collect();
            if parts.len() == 2 {
                Some((parts[0].trim().to_lowercase(), parts[1].trim().to_lowercase()))
            } else {
                None
            }
        })
        .collect();

    if correct_pairs.len() != answer_pairs.len() {
        return false;
    }

    correct_pairs.sort();
    answer_pairs.sort();
    correct_pairs == answer_pairs
}

/// Check ordering answers: compare comma-separated sequences.
/// Accepts both full item text and numeric positions (1,2,3,4).
#[allow(dead_code)]
fn check_ordering_answer(correct: &str, answer: &str) -> bool {
    let correct_items: Vec<&str> = correct.split(',').map(|s| s.trim()).collect();
    let answer_items: Vec<&str> = answer.split(',').map(|s| s.trim()).collect();

    if correct_items.len() != answer_items.len() {
        return false;
    }

    // Direct text match
    if correct_items == answer_items {
        return true;
    }

    // Check if answer uses numeric positions (1-indexed)
    let numeric: Vec<usize> = answer_items
        .iter()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();

    if numeric.len() == answer_items.len() {
        // Convert numeric positions to items (1-indexed)
        // The "options" in an ordering question are the shuffled items
        // But since we only have the correct sequence, check if the numbers
        // spell out 1,2,3,...,n (i.e., the user got the order right)
        let expected: Vec<usize> = (1..=correct_items.len()).collect();
        return numeric == expected;
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
            difficulty: "medium".into(),
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
            difficulty: "medium".into(),
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
            difficulty: "medium".into(),
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
    fn test_check_answer_fill_in_blank() {
        let q = QuizQuestion {
            id: 1, topic_id: 1,
            question: "3 + 4 × 2 = ___".into(),
            question_type: "fill_in_blank".into(),
            difficulty: "medium".into(),
            correct_answer: "11".into(),
            options: vec![],
            hint: None,
            explanation: "Order of operations.".into(),
        };
        assert!(check_answer(&q, "11"));
        assert!(!check_answer(&q, "14"));
        assert!(!check_answer(&q, "a")); // letter shortcuts shouldn't work
    }

    #[test]
    fn test_check_answer_ordering_text() {
        let q = QuizQuestion {
            id: 1, topic_id: 1,
            question: "Order these:".into(),
            question_type: "ordering".into(),
            difficulty: "medium".into(),
            correct_answer: "Alpha,Beta,Gamma".into(),
            options: vec!["Beta".into(), "Gamma".into(), "Alpha".into()],
            hint: None,
            explanation: "Greek alphabet order.".into(),
        };
        assert!(check_answer(&q, "Alpha,Beta,Gamma"));
        assert!(check_answer(&q, "alpha, beta, gamma"));
        assert!(!check_answer(&q, "Beta,Alpha,Gamma"));
    }

    #[test]
    fn test_check_answer_ordering_numeric() {
        let q = QuizQuestion {
            id: 1, topic_id: 1,
            question: "Order these:".into(),
            question_type: "ordering".into(),
            difficulty: "medium".into(),
            correct_answer: "First,Second,Third".into(),
            options: vec!["Second".into(), "Third".into(), "First".into()],
            hint: None,
            explanation: "Correct order.".into(),
        };
        assert!(check_answer(&q, "1,2,3"));
        assert!(!check_answer(&q, "2,1,3"));
    }

    #[test]
    fn test_get_questions_empty_topic() {
        let conn = db::init_memory_db().unwrap();
        let qs = get_questions(&conn, 9999, 5).unwrap();
        assert!(qs.is_empty());
    }

    #[test]
    fn test_levenshtein_identical() {
        assert_eq!(levenshtein("hello", "hello"), 0);
    }

    #[test]
    fn test_levenshtein_one_edit() {
        assert_eq!(levenshtein("hello", "helo"), 1);
        assert_eq!(levenshtein("cat", "bat"), 1);
    }

    #[test]
    fn test_levenshtein_empty() {
        assert_eq!(levenshtein("", "abc"), 3);
        assert_eq!(levenshtein("abc", ""), 3);
    }

    #[test]
    fn test_fuzzy_match_exact() {
        assert!(fuzzy_match("odysseus", "odysseus"));
    }

    #[test]
    fn test_fuzzy_match_one_typo() {
        // "odysseus" (8 chars) → allowed 1 edit
        assert!(fuzzy_match("odyseus", "odysseus"));
    }

    #[test]
    fn test_fuzzy_match_short_strict() {
        // Short answers (≤4 chars) must be exact
        assert!(!fuzzy_match("cat", "car"));
        assert!(fuzzy_match("1", "1"));
        assert!(!fuzzy_match("2", "1"));
        assert!(!fuzzy_match("yes", "no"));
    }

    #[test]
    fn test_fuzzy_match_too_many_edits() {
        assert!(!fuzzy_match("odiseos", "odysseus")); // 3 edits, only 1-2 allowed
    }

    #[test]
    fn test_check_matching_answer_correct() {
        let q = QuizQuestion {
            id: 1, topic_id: 1,
            question: "Match these:".into(),
            question_type: "matching".into(),
            difficulty: "medium".into(),
            correct_answer: "Dog=Mammal;Snake=Reptile;Frog=Amphibian".into(),
            options: vec![],
            hint: None,
            explanation: "Basic animal classification.".into(),
        };
        assert!(check_answer(&q, "Dog=Mammal;Snake=Reptile;Frog=Amphibian"));
        // Order doesn't matter
        assert!(check_answer(&q, "Frog=Amphibian;Dog=Mammal;Snake=Reptile"));
        // Case insensitive
        assert!(check_answer(&q, "dog=mammal;snake=reptile;frog=amphibian"));
    }

    #[test]
    fn test_check_matching_answer_wrong() {
        let q = QuizQuestion {
            id: 1, topic_id: 1,
            question: "Match these:".into(),
            question_type: "matching".into(),
            difficulty: "medium".into(),
            correct_answer: "Dog=Mammal;Snake=Reptile".into(),
            options: vec![],
            hint: None,
            explanation: "Classification.".into(),
        };
        assert!(!check_answer(&q, "Dog=Reptile;Snake=Mammal"));
        assert!(!check_answer(&q, "Dog=Mammal")); // Missing pair
    }

    #[test]
    fn test_fill_in_blank_fuzzy_acceptance() {
        let q = QuizQuestion {
            id: 1, topic_id: 1,
            question: "The Odyssey follows ___ on his journey home.".into(),
            question_type: "fill_in_blank".into(),
            difficulty: "medium".into(),
            correct_answer: "Odysseus".into(),
            options: vec![],
            hint: None,
            explanation: "The hero of the Odyssey.".into(),
        };
        assert!(check_answer(&q, "Odysseus"));
        assert!(check_answer(&q, "odysseus")); // case insensitive
        assert!(check_answer(&q, "Odyseus")); // 1 typo accepted
        assert!(!check_answer(&q, "Zeus")); // totally wrong
    }
}
