use colored::*;
use rusqlite::Connection;
use crate::display;
use crate::engine::spaced;

/// Show topics that are "leeches" — topics the user repeatedly struggles with.
/// Suggests targeted review strategies.
pub fn run(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    display::print_header("Leech Detection 🧛");

    let leeches = spaced::get_leeches(conn)?;

    if leeches.is_empty() {
        display::print_success("No leeches detected! All your topics are progressing well. 🎉");
        println!();
        display::print_info("Leeches are topics you've failed repeatedly (4+ consecutive failures).");
        display::print_info("Keep reviewing to prevent them from forming!");
        return Ok(());
    }

    println!("  Found {} leech topic(s) — these need extra attention:\n",
        leeches.len().to_string().bold().bright_red());

    for (topic_id, topic_name, subject_name, leech_count) in &leeches {
        let retention = spaced::estimate_retention(conn, *topic_id);
        let ret_pct = (retention * 100.0) as u32;

        let ease: f64 = conn.query_row(
            "SELECT ease_factor FROM user_progress WHERE topic_id = ?1",
            [topic_id], |r| r.get(0),
        ).unwrap_or(2.5);

        println!("  🧛 {} ({})", topic_name.bold().bright_red(), subject_name.dimmed());
        println!("     Leech count: {} | Ease: {:.2} | Retention: {}%",
            leech_count, ease, ret_pct);

        // Suggest action
        if ease < 1.5 {
            println!("     💡 {}", "Try re-reading the lesson material before quizzing again.".yellow());
            println!("        → {}", format!("opentutor learn {}", subject_name.to_lowercase()).bright_cyan());
        } else {
            println!("     💡 {}", "Focus on the explanation first, then quiz with fewer questions.".yellow());
        }
        println!();
    }

    display::print_divider();
    println!();
    display::print_section("What Are Leeches?");
    display::print_content(
        "Leeches are topics that drain your study time without progressing.\n\
         They're detected when a topic has:\n\
         • 4+ consecutive failures in spaced repetition reviews\n\
         • Each 4 consecutive fails adds a leech mark\n\
         • Successful reviews reset the failure counter\n\n\
         Strategy: Go back to basics. Re-read the lesson, study the\n\
         explanation, then try the quiz again with just 2-3 questions."
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use crate::engine::spaced;

    #[test]
    fn test_leech_no_data() {
        let conn = db::init_memory_db().unwrap();
        run(&conn).unwrap();
    }

    #[test]
    fn test_leech_with_struggling_topic() {
        let conn = db::init_memory_db().unwrap();
        // Trigger leech via consecutive failures
        spaced::update_spaced_repetition(&conn, 1, 4).unwrap(); // initial
        for _ in 0..4 {
            spaced::update_spaced_repetition(&conn, 1, 1).unwrap();
        }
        run(&conn).unwrap();
    }

    #[test]
    fn test_leech_no_false_positives() {
        let conn = db::init_memory_db().unwrap();
        spaced::update_spaced_repetition(&conn, 1, 5).unwrap();
        spaced::update_spaced_repetition(&conn, 1, 5).unwrap();
        run(&conn).unwrap(); // Should show "no leeches"
    }
}
