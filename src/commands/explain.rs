use colored::*;
use rusqlite::Connection;
use crate::display;
use crate::engine::adaptive;

struct ExplanationRow {
    topic_id: i64,
    concept: String,
    explanation: String,
    analogy: Option<String>,
    follow_up: Option<String>,
}

pub fn run(conn: &Connection, concept: &str) -> Result<(), Box<dyn std::error::Error>> {
    let result = conn.query_row(
        "SELECT e.topic_id, e.concept, e.explanation, e.analogy, e.follow_up_question
         FROM explanations e
         WHERE LOWER(e.concept) LIKE '%' || LOWER(?1) || '%'",
        [concept],
        |r| Ok(ExplanationRow {
            topic_id: r.get(0)?,
            concept: r.get(1)?,
            explanation: r.get(2)?,
            analogy: r.get(3)?,
            follow_up: r.get(4)?,
        }),
    );

    match result {
        Ok(row) => {
            display::print_header(&format!("Explain: {}", row.concept));

            display::print_section("What is it?");
            display::print_content(&row.explanation);

            if let Some(analogy) = &row.analogy {
                display::print_section("Think of it this way:");
                display::print_content(analogy);
            }

            if let Some(question) = &row.follow_up {
                display::print_section("Think about this:");
                println!("    🤔 {}", question.italic().bright_yellow());
                println!();
            }

            adaptive::log_activity(conn, row.topic_id, "explain", None)?;

            display::print_info(&format!("Dive deeper: {}",
                "opentutor learn <subject>".bright_cyan()
            ));
        }
        Err(_) => {
            display::print_error(&format!("No explanation found for '{}'.", concept));

            let mut stmt = conn.prepare("SELECT concept FROM explanations")?;
            let concepts: Vec<String> = stmt.query_map([], |r| r.get(0))?
                .filter_map(|r| r.ok())
                .collect();

            if !concepts.is_empty() {
                display::print_info("Available concepts:");
                for c in &concepts {
                    println!("    • {}", c);
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_explain_existing_concept() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "photosynthesis").unwrap();
    }

    #[test]
    fn test_explain_partial_match() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "photo").unwrap();
    }

    #[test]
    fn test_explain_missing_concept() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "quantum_entanglement").unwrap();
    }
}
