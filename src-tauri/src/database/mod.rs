pub mod migrations;

use rusqlite::Connection;
use std::sync::Mutex;

pub struct Database {
    pub conn: Mutex<Connection>,
}

pub fn init(base_dir: &std::path::Path) -> Result<Database, String> {
    std::fs::create_dir_all(base_dir).map_err(|e| e.to_string())?;
    let db_path = base_dir.join("taskmanager.db");
    println!("[DB] init: path={}", db_path.display());
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")
        .map_err(|e| e.to_string())?;
    migrations::run_migrations(&conn)?;
    Ok(Database {
        conn: Mutex::new(conn),
    })
}
