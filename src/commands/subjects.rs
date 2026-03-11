use colored::*;
use rusqlite::Connection;
use crate::display;

pub fn run(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    display::print_header("Available Subjects");

    let mut stmt = conn.prepare(
        "SELECT s.name, s.description, COUNT(t.id) as topic_count
         FROM subjects s
         LEFT JOIN topics t ON t.subject_id = s.id
         GROUP BY s.id ORDER BY s.name"
    )?;

    let rows = stmt.query_map([], |r| {
        Ok((
            r.get::<_, String>(0)?,
            r.get::<_, String>(1)?,
            r.get::<_, i64>(2)?,
        ))
    })?;

    for row in rows {
        let (name, desc, count) = row?;
        println!("  \u{1F4D8} {} ({})",
            name.bold().bright_white(),
            format!("{} topics", count).dimmed()
        );
        println!("     {}", desc.dimmed());
        println!();
    }

    display::print_info(&format!("Start learning: {} or {}",
        "opentutor learn <subject>".bright_cyan(),
        "opentutor quiz <topic>".bright_cyan()
    ));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_subjects_runs() {
        let conn = db::init_memory_db().unwrap();
        run(&conn).unwrap();
    }
}
