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
    // Try numeric normalization first (e.g., "3.0" == "3", "1/2" == "0.5")
    if numeric_match(answer, correct) {
        return true;
    }
    let max_len = answer.len().max(correct.len());
    if max_len <= 4 {
        return false; // Short answers must be exact
    }
    let allowed = (max_len / 5).clamp(1, 2);
    levenshtein(answer, correct) <= allowed
}

/// Attempt to parse a string as a numeric value, handling fractions like "1/2".
fn parse_numeric(s: &str) -> Option<f64> {
    let s = s.trim();
    // Try direct float parse
    if let Ok(v) = s.parse::<f64>() {
        return Some(v);
    }
    // Try fraction format "a/b"
    if let Some((num, den)) = s.split_once('/') {
        if let (Ok(n), Ok(d)) = (num.trim().parse::<f64>(), den.trim().parse::<f64>()) {
            if d != 0.0 {
                return Some(n / d);
            }
        }
    }
    // Try percentage "50%" → 0.5
    if let Some(pct) = s.strip_suffix('%') {
        if let Ok(v) = pct.trim().parse::<f64>() {
            return Some(v / 100.0);
        }
    }
    None
}

/// Check if two strings represent the same numeric value.
/// Handles integers, floats, fractions, and percentages.
fn numeric_match(answer: &str, correct: &str) -> bool {
    match (parse_numeric(answer), parse_numeric(correct)) {
        (Some(a), Some(b)) => (a - b).abs() < 1e-9,
        _ => false,
    }
}

/// Partial credit result for nuanced scoring.
/// Full = 1.0, Partial = 0.25–0.75, Wrong = 0.0
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AnswerResult {
    pub correct: bool,
    pub credit: f64,
    pub feedback: AnswerFeedback,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnswerFeedback {
    Correct,
    NearMiss,      // Close but not quite (typo, partial match)
    HalfRight,     // Got some parts right (ordering/matching)
    Wrong,
}

impl AnswerResult {
    pub fn full() -> Self {
        Self { correct: true, credit: 1.0, feedback: AnswerFeedback::Correct }
    }
    pub fn near_miss() -> Self {
        Self { correct: false, credit: 0.5, feedback: AnswerFeedback::NearMiss }
    }
    pub fn half_right(credit: f64) -> Self {
        Self { correct: false, credit: credit.clamp(0.0, 0.75), feedback: AnswerFeedback::HalfRight }
    }
    pub fn wrong() -> Self {
        Self { correct: false, credit: 0.0, feedback: AnswerFeedback::Wrong }
    }
}

/// Check an answer with partial credit scoring.
/// Returns an AnswerResult with credit between 0.0 and 1.0.
#[allow(dead_code)]
pub fn check_answer_scored(question: &QuizQuestion, answer: &str) -> AnswerResult {
    let answer_lower = answer.trim().to_lowercase();
    let correct_lower = question.correct_answer.trim().to_lowercase();

    // Exact match is always full credit
    if answer_lower == correct_lower {
        return AnswerResult::full();
    }

    // Ordering: partial credit for partially correct order
    if question.question_type == "ordering" {
        return check_ordering_scored(&correct_lower, &answer_lower);
    }

    // Matching: partial credit for some correct pairs
    if question.question_type == "matching" {
        return check_matching_scored(&correct_lower, &answer_lower);
    }

    // Select-all: partial credit for correct selections
    if question.question_type == "select_all" {
        return check_select_all_scored(&correct_lower, &answer_lower);
    }

    // Fill-in-blank: near-miss detection
    if question.question_type == "fill_in_blank" {
        if fuzzy_match(&answer_lower, &correct_lower) {
            return AnswerResult::full(); // Close enough = full credit
        }
        // Check if it's a near miss (2-3 edits for longer strings)
        let max_len = answer_lower.len().max(correct_lower.len());
        if max_len > 4 {
            let dist = levenshtein(&answer_lower, &correct_lower);
            let allowed_near = (max_len / 3).clamp(2, 4);
            if dist <= allowed_near {
                return AnswerResult::near_miss();
            }
        }
        // Numeric near-match
        if let (Some(a), Some(b)) = (parse_numeric(&answer_lower), parse_numeric(&correct_lower)) {
            let rel_error = ((a - b) / b.abs().max(1e-9)).abs();
            if rel_error < 0.01 {
                return AnswerResult::full();
            } else if rel_error < 0.15 {
                return AnswerResult::near_miss();
            }
        }
        return AnswerResult::wrong();
    }

    // True/false shortcuts
    if question.question_type == "true_false" {
        let is_correct = match answer_lower.as_str() {
            "t" => correct_lower == "true",
            "f" => correct_lower == "false",
            _ => false,
        };
        return if is_correct { AnswerResult::full() } else { AnswerResult::wrong() };
    }

    // Multiple choice by letter
    if let Some(idx) = match answer_lower.as_str() {
        "a" => Some(0), "b" => Some(1), "c" => Some(2), "d" => Some(3), _ => None,
    } {
        if let Some(opt) = question.options.get(idx) {
            if opt.trim().to_lowercase() == correct_lower {
                return AnswerResult::full();
            }
        }
    }

    AnswerResult::wrong()
}

/// Partial credit for ordering: fraction of items in correct position.
fn check_ordering_scored(correct: &str, answer: &str) -> AnswerResult {
    let correct_items: Vec<&str> = correct.split(',').map(|s| s.trim()).collect();
    let answer_items: Vec<&str> = answer.split(',').map(|s| s.trim()).collect();

    if correct_items.len() != answer_items.len() {
        return AnswerResult::wrong();
    }

    if correct_items == answer_items {
        return AnswerResult::full();
    }

    // Check numeric shorthand
    let numeric: Vec<usize> = answer_items.iter().filter_map(|s| s.parse().ok()).collect();
    if numeric.len() == answer_items.len() {
        let expected: Vec<usize> = (1..=correct_items.len()).collect();
        if numeric == expected {
            return AnswerResult::full();
        }
    }

    // Count items in correct position
    let correct_positions = correct_items.iter().zip(answer_items.iter())
        .filter(|(c, a)| c == a)
        .count();
    let fraction = correct_positions as f64 / correct_items.len() as f64;

    if fraction >= 1.0 {
        AnswerResult::full()
    } else if fraction > 0.0 {
        AnswerResult::half_right(fraction * 0.75)
    } else {
        AnswerResult::wrong()
    }
}

/// Partial credit for matching: fraction of pairs correct.
fn check_matching_scored(correct: &str, answer: &str) -> AnswerResult {
    let parse_pairs = |s: &str| -> Vec<(String, String)> {
        s.split(';')
            .filter_map(|p| p.split_once('=').map(|(l, r)| (l.trim().to_lowercase(), r.trim().to_lowercase())))
            .collect()
    };

    let correct_pairs = parse_pairs(correct);
    let answer_pairs = parse_pairs(answer);

    if correct_pairs.is_empty() || answer_pairs.is_empty() {
        return AnswerResult::wrong();
    }

    let mut correct_count = 0;
    for cp in &correct_pairs {
        if answer_pairs.contains(cp) {
            correct_count += 1;
        }
    }

    if correct_count == correct_pairs.len() {
        AnswerResult::full()
    } else if correct_count > 0 {
        AnswerResult::half_right(correct_count as f64 / correct_pairs.len() as f64 * 0.75)
    } else {
        AnswerResult::wrong()
    }
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

    // For select_all questions, the correct_answer is a comma-separated list of
    // correct options. The user must select all correct ones (order doesn't matter).
    if question.question_type == "select_all" {
        return check_select_all_answer(&correct, &answer);
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

/// Check select_all answers: correct_answer and user answer are both
/// comma-separated lists of items. Order doesn't matter.
#[allow(dead_code)]
fn check_select_all_answer(correct: &str, answer: &str) -> bool {
    let mut correct_items: Vec<String> = correct
        .split(',')
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect();
    let mut answer_items: Vec<String> = answer
        .split(',')
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect();

    correct_items.sort();
    answer_items.sort();
    correct_items == answer_items
}

/// Partial credit for select_all: fraction of correct items selected,
/// with penalty for wrong selections.
fn check_select_all_scored(correct: &str, answer: &str) -> AnswerResult {
    let correct_items: std::collections::HashSet<String> = correct
        .split(',')
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect();
    let answer_items: std::collections::HashSet<String> = answer
        .split(',')
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect();

    if correct_items.is_empty() {
        return AnswerResult::wrong();
    }

    if correct_items == answer_items {
        return AnswerResult::full();
    }

    let hits = correct_items.intersection(&answer_items).count();
    let false_positives = answer_items.difference(&correct_items).count();

    // Score: correct selections minus penalty for wrong ones
    let raw_score = hits as f64 / correct_items.len() as f64;
    let penalty = false_positives as f64 * 0.25; // each wrong pick costs 25%
    let final_score = (raw_score - penalty).max(0.0);

    if final_score >= 0.99 {
        AnswerResult::full()
    } else if final_score > 0.0 {
        AnswerResult::half_right(final_score * 0.75)
    } else {
        AnswerResult::wrong()
    }
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

    #[test]
    fn test_parse_numeric_integer() {
        assert_eq!(parse_numeric("42"), Some(42.0));
        assert_eq!(parse_numeric("  7 "), Some(7.0));
    }

    #[test]
    fn test_parse_numeric_float() {
        assert_eq!(parse_numeric("3.14"), Some(3.14));
        assert_eq!(parse_numeric("0.5"), Some(0.5));
    }

    #[test]
    fn test_parse_numeric_fraction() {
        assert!((parse_numeric("1/2").unwrap() - 0.5).abs() < 1e-9);
        assert!((parse_numeric("3/4").unwrap() - 0.75).abs() < 1e-9);
        assert!(parse_numeric("1/0").is_none()); // division by zero
    }

    #[test]
    fn test_parse_numeric_percentage() {
        assert!((parse_numeric("50%").unwrap() - 0.5).abs() < 1e-9);
        assert!((parse_numeric("100%").unwrap() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_parse_numeric_non_numeric() {
        assert!(parse_numeric("hello").is_none());
        assert!(parse_numeric("").is_none());
    }

    #[test]
    fn test_numeric_match_equivalent_forms() {
        assert!(numeric_match("3.0", "3"));
        assert!(numeric_match("1/2", "0.5"));
        assert!(numeric_match("0.25", "1/4"));
        assert!(numeric_match("100", "100.0"));
    }

    #[test]
    fn test_numeric_match_different_values() {
        assert!(!numeric_match("3", "4"));
        assert!(!numeric_match("1/2", "1/3"));
    }

    #[test]
    fn test_fill_in_blank_numeric_normalization() {
        let q = QuizQuestion {
            id: 1, topic_id: 1,
            question: "What is 1/2 as a decimal?".into(),
            question_type: "fill_in_blank".into(),
            difficulty: "easy".into(),
            correct_answer: "0.5".into(),
            options: vec![],
            hint: None,
            explanation: "1 divided by 2 = 0.5".into(),
        };
        assert!(check_answer(&q, "0.5"));
        assert!(check_answer(&q, "1/2"));
        assert!(check_answer(&q, ".5"));
        assert!(check_answer(&q, "50%"));
        assert!(!check_answer(&q, "0.6"));
    }

    #[test]
    fn test_fill_in_blank_integer_normalization() {
        let q = QuizQuestion {
            id: 1, topic_id: 1,
            question: "What is 2+1?".into(),
            question_type: "fill_in_blank".into(),
            difficulty: "easy".into(),
            correct_answer: "3".into(),
            options: vec![],
            hint: None,
            explanation: "Basic addition.".into(),
        };
        assert!(check_answer(&q, "3"));
        assert!(check_answer(&q, "3.0"));
        assert!(!check_answer(&q, "4"));
    }

    #[test]
    fn test_game_theory_quiz_loads() {
        let conn = db::init_memory_db().unwrap();
        // Find the Nash Equilibrium topic
        let nash_id: Option<i64> = conn.query_row(
            "SELECT id FROM topics WHERE name = 'Nash Equilibrium'",
            [], |r| r.get(0),
        ).ok();
        assert!(nash_id.is_some(), "Nash Equilibrium topic should exist");
        let qs = get_questions(&conn, nash_id.unwrap(), 5).unwrap();
        assert!(!qs.is_empty(), "Should have quiz questions for Nash Equilibrium");
    }

    #[test]
    fn test_game_theory_all_topics_have_quizzes() {
        let conn = db::init_memory_db().unwrap();
        let gt_id: i64 = conn.query_row(
            "SELECT id FROM subjects WHERE name = 'Game Theory'",
            [], |r| r.get(0),
        ).unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name FROM topics WHERE subject_id = ?1",
        ).unwrap();
        let topics: Vec<(i64, String)> = stmt
            .query_map([gt_id], |r| Ok((r.get(0)?, r.get(1)?)))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();
        assert!(topics.len() >= 6, "Game Theory should have at least 6 topics");
        for (tid, name) in &topics {
            let qs = get_questions(&conn, *tid, 10).unwrap();
            assert!(!qs.is_empty(), "Topic '{}' should have quiz questions", name);
        }
    }

    // ── Partial Credit Scoring Tests ──────────────────────────────────

    #[test]
    fn test_scored_exact_match_full_credit() {
        let q = QuizQuestion {
            id: 1, topic_id: 1,
            question: "What is 2+2?".into(),
            question_type: "fill_in_blank".into(),
            difficulty: "easy".into(),
            correct_answer: "4".into(),
            options: vec![],
            hint: None,
            explanation: "Basic math.".into(),
        };
        let result = check_answer_scored(&q, "4");
        assert!(result.correct);
        assert!((result.credit - 1.0).abs() < f64::EPSILON);
        assert_eq!(result.feedback, AnswerFeedback::Correct);
    }

    #[test]
    fn test_scored_near_miss_fill_in_blank() {
        let q = QuizQuestion {
            id: 1, topic_id: 1,
            question: "Who wrote Hamlet?".into(),
            question_type: "fill_in_blank".into(),
            difficulty: "medium".into(),
            correct_answer: "Shakespeare".into(),
            options: vec![],
            hint: None,
            explanation: "William Shakespeare.".into(),
        };
        // Close typo gets full credit (within fuzzy_match range)
        let result = check_answer_scored(&q, "Shakespear");
        assert!(result.credit >= 0.5, "One-letter-off should get at least partial credit, got {}", result.credit);
        // Totally wrong
        let result = check_answer_scored(&q, "Tolkien");
        assert!((result.credit - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_scored_ordering_partial_credit() {
        let q = QuizQuestion {
            id: 1, topic_id: 1,
            question: "Order these:".into(),
            question_type: "ordering".into(),
            difficulty: "medium".into(),
            correct_answer: "alpha,beta,gamma,delta".into(),
            options: vec![],
            hint: None,
            explanation: "Greek alphabet.".into(),
        };
        // Fully correct
        let result = check_answer_scored(&q, "alpha,beta,gamma,delta");
        assert!(result.correct);
        assert!((result.credit - 1.0).abs() < f64::EPSILON);

        // 2 out of 4 correct positions
        let result = check_answer_scored(&q, "alpha,gamma,beta,delta");
        assert!(!result.correct);
        assert!(result.credit > 0.0, "Should get partial credit for some correct positions");
        assert!(result.credit < 1.0);
    }

    #[test]
    fn test_scored_matching_partial_credit() {
        let q = QuizQuestion {
            id: 1, topic_id: 1,
            question: "Match:".into(),
            question_type: "matching".into(),
            difficulty: "medium".into(),
            correct_answer: "Dog=Mammal;Snake=Reptile;Frog=Amphibian".into(),
            options: vec![],
            hint: None,
            explanation: "Classification.".into(),
        };
        // All correct
        let result = check_answer_scored(&q, "Dog=Mammal;Snake=Reptile;Frog=Amphibian");
        assert!(result.correct);

        // One correct, two wrong
        let result = check_answer_scored(&q, "Dog=Mammal;Snake=Amphibian;Frog=Reptile");
        assert!(!result.correct);
        assert!(result.credit > 0.0, "One correct pair should give partial credit");
        assert!(result.credit <= 0.75);
    }

    #[test]
    fn test_scored_multiple_choice_wrong() {
        let q = QuizQuestion {
            id: 1, topic_id: 1,
            question: "Capital of France?".into(),
            question_type: "multiple_choice".into(),
            difficulty: "easy".into(),
            correct_answer: "Paris".into(),
            options: vec!["London".into(), "Paris".into(), "Berlin".into(), "Rome".into()],
            hint: None,
            explanation: "Paris.".into(),
        };
        let result = check_answer_scored(&q, "London");
        assert!(!result.correct);
        assert!((result.credit - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_scored_true_false() {
        let q = QuizQuestion {
            id: 1, topic_id: 1,
            question: "Water boils at 100°C.".into(),
            question_type: "true_false".into(),
            difficulty: "easy".into(),
            correct_answer: "true".into(),
            options: vec!["true".into(), "false".into()],
            hint: None,
            explanation: "At sea level.".into(),
        };
        assert!(check_answer_scored(&q, "t").correct);
        assert!(!check_answer_scored(&q, "f").correct);
    }

    #[test]
    fn test_scored_numeric_near_miss() {
        let q = QuizQuestion {
            id: 1, topic_id: 1,
            question: "What is pi to 2 decimal places?".into(),
            question_type: "fill_in_blank".into(),
            difficulty: "medium".into(),
            correct_answer: "3.14".into(),
            options: vec![],
            hint: None,
            explanation: "Pi ≈ 3.14159...".into(),
        };
        // Exact
        assert!(check_answer_scored(&q, "3.14").correct);
        // Close numeric value
        let result = check_answer_scored(&q, "3.15");
        assert!(result.credit >= 0.5, "Close numeric answer should get partial credit");
    }

    #[test]
    fn test_select_all_exact_match() {
        assert!(check_select_all_answer("a, b, c", "a, b, c"));
        assert!(check_select_all_answer("a, b, c", "c, a, b")); // order irrelevant
        assert!(!check_select_all_answer("a, b, c", "a, b"));   // missing one
        assert!(!check_select_all_answer("a, b", "a, b, c"));   // extra one
    }

    #[test]
    fn test_select_all_scored_full() {
        let result = check_select_all_scored("a, b, c", "b, c, a");
        assert!(result.correct);
        assert!((result.credit - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_select_all_scored_partial() {
        let result = check_select_all_scored("a, b, c", "a, b");
        assert!(!result.correct);
        assert!(result.credit > 0.0, "Partial selection should get partial credit");
    }

    #[test]
    fn test_select_all_scored_with_false_positives() {
        let result = check_select_all_scored("a, b", "a, b, x");
        // 2/2 correct but 1 false positive (penalty 0.25)
        assert!(!result.correct);
        assert!(result.credit > 0.0, "Should still get some credit");
    }

    #[test]
    fn test_select_all_scored_all_wrong() {
        let result = check_select_all_scored("a, b", "x, y");
        assert!(!result.correct);
        assert!((result.credit - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_formal_languages_quiz_loads() {
        let conn = db::init_memory_db().unwrap();
        let fl_id: Option<i64> = conn.query_row(
            "SELECT id FROM subjects WHERE name = 'Formal Languages'",
            [], |r| r.get(0),
        ).ok();
        assert!(fl_id.is_some(), "Formal Languages subject should exist");
        let mut stmt = conn.prepare(
            "SELECT id, name FROM topics WHERE subject_id = ?1",
        ).unwrap();
        let topics: Vec<(i64, String)> = stmt
            .query_map([fl_id.unwrap()], |r| Ok((r.get(0)?, r.get(1)?)))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();
        assert_eq!(topics.len(), 4, "Formal Languages should have 4 topics");
        for (tid, name) in &topics {
            let qs = get_questions(&conn, *tid, 10).unwrap();
            assert!(!qs.is_empty(), "Topic '{}' should have quiz questions", name);
        }
    }

    #[test]
    fn test_philosophy_of_mind_quiz_loads() {
        let conn = db::init_memory_db().unwrap();
        let pm_id: Option<i64> = conn.query_row(
            "SELECT id FROM subjects WHERE name = 'Philosophy of Mind'",
            [], |r| r.get(0),
        ).ok();
        assert!(pm_id.is_some(), "Philosophy of Mind subject should exist");
        let mut stmt = conn.prepare(
            "SELECT id, name FROM topics WHERE subject_id = ?1",
        ).unwrap();
        let topics: Vec<(i64, String)> = stmt
            .query_map([pm_id.unwrap()], |r| Ok((r.get(0)?, r.get(1)?)))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();
        assert_eq!(topics.len(), 4, "Philosophy of Mind should have 4 topics");
        for (tid, name) in &topics {
            let qs = get_questions(&conn, *tid, 10).unwrap();
            assert!(!qs.is_empty(), "Topic '{}' should have quiz questions", name);
        }
    }

    #[test]
    fn test_linear_algebra_quiz_loads() {
        let conn = db::init_memory_db().unwrap();
        let la_id: Option<i64> = conn.query_row(
            "SELECT id FROM subjects WHERE name = 'Linear Algebra'",
            [], |r| r.get(0),
        ).ok();
        assert!(la_id.is_some(), "Linear Algebra subject should exist");
        let mut stmt = conn.prepare(
            "SELECT id, name FROM topics WHERE subject_id = ?1",
        ).unwrap();
        let topics: Vec<(i64, String)> = stmt
            .query_map([la_id.unwrap()], |r| Ok((r.get(0)?, r.get(1)?)))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();
        assert!(topics.len() >= 5, "Linear Algebra should have at least 5 topics, got {}", topics.len());
        for (tid, name) in &topics {
            let qs = get_questions(&conn, *tid, 10).unwrap();
            assert!(!qs.is_empty(), "Topic '{}' should have quiz questions", name);
        }
    }

    #[test]
    fn test_organic_chemistry_quiz_loads() {
        let conn = db::init_memory_db().unwrap();
        let oc_id: i64 = conn.query_row(
            "SELECT id FROM subjects WHERE name = 'Organic Chemistry'",
            [], |r| r.get(0),
        ).unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name FROM topics WHERE subject_id = ?1",
        ).unwrap();
        let topics: Vec<(i64, String)> = stmt
            .query_map([oc_id], |r| Ok((r.get(0)?, r.get(1)?)))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();
        assert_eq!(topics.len(), 6, "Organic Chemistry should have 6 topics");
        for (tid, name) in &topics {
            let qs = get_questions(&conn, *tid, 10).unwrap();
            assert!(!qs.is_empty(), "Topic '{}' should have quiz questions", name);
        }
    }

    #[test]
    fn test_graph_theory_quiz_loads() {
        let conn = db::init_memory_db().unwrap();
        let gt_id: i64 = conn.query_row(
            "SELECT id FROM subjects WHERE name = 'Graph Theory'",
            [], |r| r.get(0),
        ).unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name FROM topics WHERE subject_id = ?1",
        ).unwrap();
        let topics: Vec<(i64, String)> = stmt
            .query_map([gt_id], |r| Ok((r.get(0)?, r.get(1)?)))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();
        assert_eq!(topics.len(), 6, "Graph Theory should have 6 topics");
        for (tid, name) in &topics {
            let qs = get_questions(&conn, *tid, 10).unwrap();
            assert!(!qs.is_empty(), "Topic '{}' should have quiz questions", name);
        }
    }
}
