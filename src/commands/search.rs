use colored::*;
use rusqlite::Connection;
use crate::display;

pub fn run(conn: &Connection, query: &str) -> Result<(), Box<dyn std::error::Error>> {
    let pattern = format!("%{}%", query);

    display::print_header(&format!("Search: {}", query));

    let mut found = false;

    // Search lessons
    let mut lesson_stmt = conn.prepare(
        "SELECT l.title, l.content, t.name, s.name
         FROM lessons l
         JOIN topics t ON t.id = l.topic_id
         JOIN subjects s ON s.id = t.subject_id
         WHERE LOWER(l.title) LIKE LOWER(?1) OR LOWER(l.content) LIKE LOWER(?1)
         LIMIT 10"
    )?;
    let lessons: Vec<(String, String, String, String)> = lesson_stmt.query_map([&pattern], |r| {
        Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?))
    })?.collect::<Result<Vec<_>, _>>()?;

    if !lessons.is_empty() {
        found = true;
        display::print_section(&format!("Lessons ({})", lessons.len()));
        for (title, content, topic, subject) in &lessons {
            println!("    📖 {} ({} → {})", title.bold(), subject.dimmed(), topic.dimmed());
            // Show first 2 lines of content as preview
            let preview: String = content.lines().take(2).collect::<Vec<_>>().join(" ");
            let preview = if preview.len() > 120 {
                format!("{}...", &preview[..120])
            } else {
                preview
            };
            println!("       {}", preview.dimmed());
            println!();
        }
    }

    // Search quiz questions
    let mut quiz_stmt = conn.prepare(
        "SELECT q.question, t.name, s.name
         FROM quiz_questions q
         JOIN topics t ON t.id = q.topic_id
         JOIN subjects s ON s.id = t.subject_id
         WHERE LOWER(q.question) LIKE LOWER(?1) OR LOWER(q.explanation) LIKE LOWER(?1)
         LIMIT 10"
    )?;
    let quizzes: Vec<(String, String, String)> = quiz_stmt.query_map([&pattern], |r| {
        Ok((r.get(0)?, r.get(1)?, r.get(2)?))
    })?.collect::<Result<Vec<_>, _>>()?;

    if !quizzes.is_empty() {
        found = true;
        display::print_section(&format!("Quiz Questions ({})", quizzes.len()));
        for (question, topic, subject) in &quizzes {
            println!("    ❓ {} ({} → {})", question, subject.dimmed(), topic.dimmed());
        }
        println!();
    }

    // Search explanations
    let mut expl_stmt = conn.prepare(
        "SELECT e.concept, e.explanation, t.name, s.name
         FROM explanations e
         JOIN topics t ON t.id = e.topic_id
         JOIN subjects s ON s.id = t.subject_id
         WHERE LOWER(e.concept) LIKE LOWER(?1)
            OR LOWER(e.explanation) LIKE LOWER(?1)
            OR LOWER(COALESCE(e.analogy, '')) LIKE LOWER(?1)
         LIMIT 10"
    )?;
    let expls: Vec<(String, String, String, String)> = expl_stmt.query_map([&pattern], |r| {
        Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?))
    })?.collect::<Result<Vec<_>, _>>()?;

    if !expls.is_empty() {
        found = true;
        display::print_section(&format!("Explanations ({})", expls.len()));
        for (concept, explanation, _topic, subject) in &expls {
            println!("    💡 {} ({})", concept.bold(), subject.dimmed());
            let preview = if explanation.len() > 120 {
                format!("{}...", &explanation[..120])
            } else {
                explanation.clone()
            };
            println!("       {}", preview.dimmed());
            println!();
        }
    }

    // Search topics
    let mut topic_stmt = conn.prepare(
        "SELECT t.name, t.difficulty, s.name
         FROM topics t
         JOIN subjects s ON s.id = t.subject_id
         WHERE LOWER(t.name) LIKE LOWER(?1)
         LIMIT 10"
    )?;
    let topics: Vec<(String, String, String)> = topic_stmt.query_map([&pattern], |r| {
        Ok((r.get(0)?, r.get(1)?, r.get(2)?))
    })?.collect::<Result<Vec<_>, _>>()?;

    if !topics.is_empty() {
        found = true;
        display::print_section(&format!("Topics ({})", topics.len()));
        for (name, difficulty, subject) in &topics {
            let diff_badge = match difficulty.as_str() {
                "intermediate" => "🟡",
                "advanced" => "🔴",
                _ => "🟢",
            };
            println!("    {} {} ({} — {})", diff_badge, name.bold(), subject.dimmed(), difficulty);
        }
        println!();
    }

    if !found {
        display::print_info(&format!("No results found for '{}'.", query));
        display::print_hint("Try broader terms or use 'opentutor subjects' to browse.");
    } else {
        display::print_info(&format!(
            "Dive deeper: {} or {}",
            "opentutor learn <subject>".bright_cyan(),
            "opentutor explain <concept>".bright_cyan()
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_search_finds_lessons() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "photosynthesis").unwrap();
    }

    #[test]
    fn test_search_finds_quiz() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "gravity").unwrap();
    }

    #[test]
    fn test_search_no_results() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "xylophoneZZZnonexistent").unwrap();
    }

    #[test]
    fn test_search_broad_term() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "math").unwrap();
    }

    #[test]
    fn test_search_philosophy() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "Socrates").unwrap();
    }

    #[test]
    fn test_search_economics() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "supply").unwrap();
    }
}
