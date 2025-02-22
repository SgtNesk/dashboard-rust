use rusqlite::{Connection, Result};
use std::sync::Mutex;
use std::path::Path;

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new() -> Result<Self> {
        let path = Path::new("db/students.db");
        let conn = Connection::open(path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS courses (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                hours_attended REAL DEFAULT 0
            );",
            [],
        )?;
        Ok(Self { conn: Mutex::new(conn) })
    }
}
