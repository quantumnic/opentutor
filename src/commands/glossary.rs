use colored::*;
use rusqlite::Connection;
use crate::display;

pub fn run(conn: &Connection, query: &Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    match query {
        Some(q) => search_glossary(conn, q),
        None => list_all_terms(conn),
    }
}

fn search_glossary(conn: &Connection, query: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare(
        "SELECT e.concept, e.explanation, e.analogy, s.name
         FROM explanations e
         JOIN topics t ON t.id = e.topic_id
         JOIN subjects s ON s.id = t.subject_id
         WHERE LOWER(e.concept) LIKE '%' || LOWER(?1) || '%'
         ORDER BY e.concept"
    )?;

    let results: Vec<(String, String, Option<String>, String)> = stmt
        .query_map([query], |r| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    if results.is_empty() {
        display::print_error(&format!("No glossary entries matching '{}'.", query));
        display::print_info("Use 'opentutor glossary' without arguments to see all terms.");
        return Ok(());
    }

    display::print_header(&format!("Glossary: '{}' ({} result{})", query, results.len(), if results.len() == 1 { "" } else { "s" }));

    for (concept, explanation, analogy, subject) in &results {
        println!();
        println!("  {} {}", "•".bright_cyan(), concept.bold());
        println!("    {} {}", "Subject:".dimmed(), subject);
        // Show first 200 chars of explanation as preview
        let preview = if explanation.len() > 200 {
            format!("{}…", &explanation[..200])
        } else {
            explanation.clone()
        };
        for line in preview.lines() {
            println!("    {}", line);
        }
        if let Some(a) = analogy {
            println!("    {} {}", "💡".dimmed(), a.italic());
        }
    }
    println!();

    Ok(())
}

fn list_all_terms(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare(
        "SELECT e.concept, s.name
         FROM explanations e
         JOIN topics t ON t.id = e.topic_id
         JOIN subjects s ON s.id = t.subject_id
         ORDER BY s.name, e.concept"
    )?;

    let terms: Vec<(String, String)> = stmt
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))
        ?
        .collect::<Result<Vec<_>, _>>()?;

    if terms.is_empty() {
        display::print_info("No glossary entries found.");
        return Ok(());
    }

    display::print_header(&format!("Glossary ({} terms)", terms.len()));

    let mut current_subject = String::new();
    for (concept, subject) in &terms {
        if *subject != current_subject {
            current_subject = subject.clone();
            println!();
            display::print_section(subject);
        }
        println!("    • {}", concept);
    }
    println!();
    display::print_info("Use 'opentutor glossary <term>' to see the full definition.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_glossary_list_all() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, &None).unwrap();
    }

    #[test]
    fn test_glossary_search_found() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, &Some("photosynthesis".to_string())).unwrap();
    }

    #[test]
    fn test_glossary_search_not_found() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, &Some("xyznonexistent".to_string())).unwrap();
    }

    #[test]
    fn test_glossary_partial_match() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, &Some("photo".to_string())).unwrap();
    }
}
