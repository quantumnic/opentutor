use colored::*;
use rusqlite::Connection;
use crate::display;
use crate::engine::spaced;

/// Achievement definitions: (id, name, description, category)
const ACHIEVEMENT_DEFS: &[(&str, &str, &str, &str)] = &[
    // Exploration
    ("first_lesson", "First Steps 👣", "Complete your first learning session", "exploration"),
    ("five_subjects", "Renaissance Learner 🎨", "Study topics in 5 different subjects", "exploration"),
    ("ten_subjects", "Polymath 🧠", "Study topics in 10 different subjects", "exploration"),
    ("all_subjects", "Universal Scholar 🌍", "Study at least one topic in every subject", "exploration"),
    // Mastery
    ("first_mastery", "Memory Master 💪", "Master your first topic (21+ day interval)", "mastery"),
    ("ten_mastery", "Knowledge Keeper 📚", "Master 10 topics", "mastery"),
    ("perfect_quiz", "Perfect Score 💯", "Get 100% on a quiz with 5+ questions", "mastery"),
    // Consistency
    ("streak_3", "Three-Day Streak 🔥", "Learn for 3 days in a row", "consistency"),
    ("streak_7", "Week Warrior 🗓️", "Learn for 7 days in a row", "consistency"),
    ("streak_14", "Two-Week Titan ⚡", "Learn for 14 days in a row", "consistency"),
    ("streak_30", "Monthly Legend 👑", "Learn for 30 days in a row", "consistency"),
    // Effort
    ("fifty_sessions", "Dedicated Learner 📝", "Complete 50 learning sessions", "effort"),
    ("hundred_sessions", "Century Club 🏅", "Complete 100 learning sessions", "effort"),
    ("fifty_reviews", "Review Champion 🔄", "Complete 50 review sessions", "effort"),
    // Challenge
    ("first_challenge", "Challenge Accepted ⚔️", "Complete your first cross-topic challenge", "challenge"),
    ("challenge_streak_5", "Challenge Master 🏆", "Complete 5 cross-topic challenges", "challenge"),
];

/// Check all achievements and unlock any newly earned ones.
/// Returns the list of newly unlocked achievement names.
pub fn check_achievements(conn: &Connection) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // Ensure all achievement rows exist
    for (id, name, desc, cat) in ACHIEVEMENT_DEFS {
        conn.execute(
            "INSERT OR IGNORE INTO achievements (id, name, description, category) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![id, name, desc, cat],
        )?;
    }

    let mut newly_unlocked = Vec::new();

    // Helper: check and unlock
    let mut try_unlock = |id: &str, condition: bool| -> Result<(), Box<dyn std::error::Error>> {
        if !condition {
            return Ok(());
        }
        let already: bool = conn.query_row(
            "SELECT unlocked_at IS NOT NULL FROM achievements WHERE id = ?1",
            [id],
            |r| r.get(0),
        ).unwrap_or(false);
        if !already {
            conn.execute(
                "UPDATE achievements SET unlocked_at = datetime('now') WHERE id = ?1",
                [id],
            )?;
            let name: String = conn.query_row(
                "SELECT name FROM achievements WHERE id = ?1",
                [id],
                |r| r.get(0),
            )?;
            newly_unlocked.push(name);
        }
        Ok(())
    };

    // Gather stats
    let total_sessions: i64 = conn.query_row(
        "SELECT COUNT(*) FROM session_log", [], |r| r.get(0)
    ).unwrap_or(0);
    let learn_sessions: i64 = conn.query_row(
        "SELECT COUNT(*) FROM session_log WHERE activity_type = 'learn'", [], |r| r.get(0)
    ).unwrap_or(0);
    let review_sessions: i64 = conn.query_row(
        "SELECT COUNT(*) FROM session_log WHERE activity_type = 'review'", [], |r| r.get(0)
    ).unwrap_or(0);
    let challenge_sessions: i64 = conn.query_row(
        "SELECT COUNT(*) FROM session_log WHERE activity_type = 'challenge'", [], |r| r.get(0)
    ).unwrap_or(0);
    let subjects_studied: i64 = conn.query_row(
        "SELECT COUNT(DISTINCT t.subject_id) FROM user_progress p JOIN topics t ON t.id = p.topic_id",
        [], |r| r.get(0)
    ).unwrap_or(0);
    let total_subjects: i64 = conn.query_row(
        "SELECT COUNT(*) FROM subjects", [], |r| r.get(0)
    ).unwrap_or(0);
    let mastered: i64 = conn.query_row(
        "SELECT COUNT(*) FROM user_progress WHERE interval_days >= 21 AND ease_factor >= 2.0",
        [], |r| r.get(0)
    ).unwrap_or(0);
    let streak = spaced::calculate_streak(conn);

    // Check conditions
    try_unlock("first_lesson", learn_sessions >= 1 || total_sessions >= 1)?;
    try_unlock("five_subjects", subjects_studied >= 5)?;
    try_unlock("ten_subjects", subjects_studied >= 10)?;
    try_unlock("all_subjects", subjects_studied >= total_subjects && total_subjects > 0)?;
    try_unlock("first_mastery", mastered >= 1)?;
    try_unlock("ten_mastery", mastered >= 10)?;
    try_unlock("streak_3", streak >= 3)?;
    try_unlock("streak_7", streak >= 7)?;
    try_unlock("streak_14", streak >= 14)?;
    try_unlock("streak_30", streak >= 30)?;
    try_unlock("fifty_sessions", total_sessions >= 50)?;
    try_unlock("hundred_sessions", total_sessions >= 100)?;
    try_unlock("fifty_reviews", review_sessions >= 50)?;
    try_unlock("first_challenge", challenge_sessions >= 1)?;
    try_unlock("challenge_streak_5", challenge_sessions >= 5)?;

    // perfect_quiz is checked elsewhere (in quiz/challenge commands)

    Ok(newly_unlocked)
}

/// Unlock the perfect_quiz achievement.
pub fn unlock_perfect_quiz(conn: &Connection) -> Result<Option<String>, Box<dyn std::error::Error>> {
    conn.execute(
        "INSERT OR IGNORE INTO achievements (id, name, description, category) VALUES ('perfect_quiz', 'Perfect Score 💯', 'Get 100% on a quiz with 5+ questions', 'mastery')",
        [],
    )?;
    let already: bool = conn.query_row(
        "SELECT unlocked_at IS NOT NULL FROM achievements WHERE id = 'perfect_quiz'",
        [],
        |r| r.get(0),
    ).unwrap_or(false);
    if !already {
        conn.execute(
            "UPDATE achievements SET unlocked_at = datetime('now') WHERE id = 'perfect_quiz'",
            [],
        )?;
        return Ok(Some("Perfect Score 💯".to_string()));
    }
    Ok(None)
}

/// Display all achievements.
pub fn run(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    // Run a check first to unlock any pending
    let newly = check_achievements(conn)?;

    display::print_header("Achievements");

    if !newly.is_empty() {
        display::print_section("🎉 Newly Unlocked!");
        for name in &newly {
            println!("    🏆 {}", name.bold().bright_yellow());
        }
        println!();
    }

    let categories = ["exploration", "mastery", "consistency", "effort", "challenge"];
    let category_titles = ["🗺️  Exploration", "⭐ Mastery", "📅 Consistency", "💪 Effort", "⚔️  Challenge"];

    let mut total_unlocked = 0;
    let mut total_count = 0;

    for (cat, title) in categories.iter().zip(category_titles.iter()) {
        display::print_section(title);

        let mut stmt = conn.prepare(
            "SELECT name, description, unlocked_at FROM achievements WHERE category = ?1 ORDER BY id",
        )?;
        let rows: Vec<(String, String, Option<String>)> = stmt.query_map([cat], |r| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?))
        })?.collect::<Result<Vec<_>, _>>()?;

        for (name, desc, unlocked) in &rows {
            total_count += 1;
            if let Some(date) = unlocked {
                total_unlocked += 1;
                println!("    ✅ {} — {}",
                    name.bold().bright_green(),
                    format!("unlocked {}", &date[..10]).dimmed()
                );
            } else {
                println!("    🔒 {} — {}",
                    name.dimmed(),
                    desc.dimmed()
                );
            }
        }
        println!();
    }

    display::print_divider();
    display::print_progress_bar("Achievement progress", total_unlocked as f64, total_count as f64);
    println!();

    if total_unlocked == total_count && total_count > 0 {
        display::print_success("🏆 You've unlocked ALL achievements! Incredible! 🏆");
    } else {
        let remaining = total_count - total_unlocked;
        display::print_info(&format!("{} achievements remaining. Keep learning!", remaining));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use crate::engine::adaptive;

    #[test]
    fn test_achievements_empty() {
        let conn = db::init_memory_db().unwrap();
        run(&conn).unwrap();
    }

    #[test]
    fn test_check_achievements_first_lesson() {
        let conn = db::init_memory_db().unwrap();
        adaptive::log_activity(&conn, 1, "learn", Some(100.0)).unwrap();
        let newly = check_achievements(&conn).unwrap();
        assert!(newly.iter().any(|n| n.contains("First Steps")),
            "Should unlock First Steps, got: {:?}", newly);
    }

    #[test]
    fn test_unlock_perfect_quiz() {
        let conn = db::init_memory_db().unwrap();
        let result = unlock_perfect_quiz(&conn).unwrap();
        assert!(result.is_some());
        // Second time should return None
        let result2 = unlock_perfect_quiz(&conn).unwrap();
        assert!(result2.is_none());
    }

    #[test]
    fn test_achievements_idempotent() {
        let conn = db::init_memory_db().unwrap();
        adaptive::log_activity(&conn, 1, "learn", Some(100.0)).unwrap();
        let first = check_achievements(&conn).unwrap();
        let second = check_achievements(&conn).unwrap();
        assert!(!first.is_empty());
        assert!(second.is_empty(), "Second check should not re-unlock");
    }
}
