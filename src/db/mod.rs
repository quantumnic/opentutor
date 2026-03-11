pub mod schema;
pub mod seed;

use rusqlite::Connection;
use std::path::Path;

/// Initialize the database: create tables and seed if empty.
pub fn init_db(path: &Path) -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open(path)?;
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
    schema::create_tables(&conn)?;
    seed::seed_if_empty(&conn)?;
    Ok(conn)
}

#[cfg(test)]
pub fn init_memory_db() -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open_in_memory()?;
    conn.execute_batch("PRAGMA foreign_keys=ON;")?;
    schema::create_tables(&conn)?;
    seed::seed_if_empty(&conn)?;
    Ok(conn)
}
