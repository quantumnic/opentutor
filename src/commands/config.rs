use colored::Colorize;
use rusqlite::Connection;

/// Get a config value, returning the default if not set.
pub fn get_config(conn: &Connection, key: &str, default: &str) -> String {
    conn.query_row(
        "SELECT value FROM user_config WHERE key = ?1",
        [key],
        |r| r.get(0),
    )
    .unwrap_or_else(|_| default.to_string())
}

/// Set a config value.
pub fn set_config(conn: &Connection, key: &str, value: &str) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO user_config (key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value = ?2",
        rusqlite::params![key, value],
    )?;
    Ok(())
}

/// Get the user's desired retention rate (0.5-0.99, default 0.85).
pub fn get_desired_retention(conn: &Connection) -> f64 {
    let val = get_config(conn, "desired_retention", "0.85");
    val.parse::<f64>().unwrap_or(0.85).clamp(0.5, 0.99)
}

pub fn run(conn: &Connection, key: &Option<String>, value: &Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    match (key, value) {
        (Some(k), Some(v)) => {
            // Validate known keys
            match k.as_str() {
                "desired_retention" => {
                    let r: f64 = v.parse().map_err(|_| "Value must be a number between 0.5 and 0.99")?;
                    if !(0.5..=0.99).contains(&r) {
                        return Err("desired_retention must be between 0.50 and 0.99".into());
                    }
                    set_config(conn, k, v)?;
                    println!("{} {} = {}", "✓".green().bold(), k, v);
                    println!(
                        "  {}",
                        format!(
                            "Spaced repetition will now target {}% memory retention.",
                            (r * 100.0).round()
                        )
                        .dimmed()
                    );
                }
                "daily_goal" => {
                    let g: u32 = v.parse().map_err(|_| "Value must be a positive integer")?;
                    if g == 0 || g > 100 {
                        return Err("daily_goal must be between 1 and 100".into());
                    }
                    set_config(conn, k, v)?;
                    println!("{} {} = {}", "✓".green().bold(), k, v);
                    println!("  {}", format!("Your daily goal is now {} topics.", g).dimmed());
                }
                _ => {
                    set_config(conn, k, v)?;
                    println!("{} {} = {}", "✓".green().bold(), k, v);
                }
            }
        }
        (Some(k), None) => {
            // Show a single config value
            let val = get_config(conn, k, "(not set)");
            println!("{}: {}", k.bold(), val);
        }
        (None, None) => {
            // Show all config
            println!("{}", "⚙  Configuration".bold().cyan());
            println!();

            let known = [
                ("desired_retention", "0.85", "Target memory retention for spaced repetition (0.50–0.99)"),
                ("daily_goal", "5", "Number of topics to study per day"),
            ];

            for (key, default, desc) in &known {
                let val = get_config(conn, key, default);
                println!("  {} = {}", key.bold(), val.green());
                println!("    {}", desc.dimmed());
            }

            // Show any custom keys
            let mut stmt = conn.prepare(
                "SELECT key, value FROM user_config WHERE key NOT IN ('desired_retention', 'daily_goal') ORDER BY key",
            )?;
            let custom: Vec<(String, String)> = stmt
                .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
                .filter_map(|r| r.ok())
                .collect();

            if !custom.is_empty() {
                println!();
                println!("  {}", "Custom:".dimmed());
                for (k, v) in &custom {
                    println!("  {} = {}", k.bold(), v.green());
                }
            }

            println!();
            println!("{}", "Usage: opentutor config <key> <value>".dimmed());
        }
        _ => unreachable!(),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_get_set_config() {
        let conn = db::init_memory_db().unwrap();
        assert_eq!(get_config(&conn, "test_key", "default"), "default");
        set_config(&conn, "test_key", "hello").unwrap();
        assert_eq!(get_config(&conn, "test_key", "default"), "hello");
    }

    #[test]
    fn test_desired_retention_default() {
        let conn = db::init_memory_db().unwrap();
        let r = get_desired_retention(&conn);
        assert!((r - 0.85).abs() < 0.001);
    }

    #[test]
    fn test_desired_retention_custom() {
        let conn = db::init_memory_db().unwrap();
        set_config(&conn, "desired_retention", "0.90").unwrap();
        let r = get_desired_retention(&conn);
        assert!((r - 0.90).abs() < 0.001);
    }

    #[test]
    fn test_desired_retention_clamped() {
        let conn = db::init_memory_db().unwrap();
        set_config(&conn, "desired_retention", "0.3").unwrap();
        assert!((get_desired_retention(&conn) - 0.5).abs() < 0.001);
        set_config(&conn, "desired_retention", "1.5").unwrap();
        assert!((get_desired_retention(&conn) - 0.99).abs() < 0.001);
    }
}
