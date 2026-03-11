use colored::*;
use rusqlite::Connection;
use crate::display;

pub fn run(conn: &Connection, goal: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare(
        "SELECT lp.step_order, t.name, lp.description, t.difficulty
         FROM learning_paths lp
         JOIN topics t ON t.id = lp.topic_id
         WHERE LOWER(lp.goal) LIKE '%' || LOWER(?1) || '%'
         ORDER BY lp.step_order"
    )?;

    let steps: Vec<(i64, String, String, String)> = stmt.query_map([goal], |r| {
        Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?))
    })?.collect::<Result<Vec<_>, _>>()?;

    if steps.is_empty() {
        display::print_error(&format!("No learning path found for '{}'.", goal));

        // Show available paths
        let mut path_stmt = conn.prepare("SELECT DISTINCT goal FROM learning_paths ORDER BY goal")?;
        let goals: Vec<String> = path_stmt.query_map([], |r| r.get(0))?
            .filter_map(|r| r.ok())
            .collect();

        if !goals.is_empty() {
            display::print_info("Available learning paths:");
            for g in &goals {
                println!("    • {}", g);
            }
        }
        return Ok(());
    }

    display::print_header(&format!("Learning Path: {}", goal));
    println!("  Follow these steps in order to master {}:\n", goal.bright_white().bold());

    for (order, topic_name, desc, difficulty) in &steps {
        let diff_badge = match difficulty.as_str() {
            "intermediate" => "🟡".to_string(),
            "advanced" => "🔴".to_string(),
            _ => "🟢".to_string(),
        };

        // Check if user has progress on this topic
        let studied: bool = conn.query_row(
            "SELECT COUNT(*) > 0 FROM user_progress p JOIN topics t ON t.id = p.topic_id WHERE LOWER(t.name) = LOWER(?1)",
            [topic_name],
            |r| r.get(0),
        ).unwrap_or(false);

        let status = if studied { "✅" } else { "⬜" };

        println!("  {} Step {}: {} {} {}",
            status,
            order.to_string().bold(),
            diff_badge,
            topic_name.bold().bright_white(),
            format!("[{}]", difficulty).dimmed()
        );
        println!("     {}", desc.dimmed());
        println!();
    }

    display::print_info("Start with the first uncompleted step!");
    display::print_info(&format!("Begin: {}",
        "opentutor learn <subject>".bright_cyan()
    ));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_path_algebra() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "algebra").unwrap();
    }

    #[test]
    fn test_path_healthy_living() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "healthy").unwrap();
    }

    #[test]
    fn test_path_not_found() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "nonexistent_path").unwrap();
    }

    #[test]
    fn test_path_cells() {
        let conn = db::init_memory_db().unwrap();
        run(&conn, "cells").unwrap();
    }
}
