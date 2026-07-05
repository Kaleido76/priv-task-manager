use rusqlite::Connection;

pub fn run_migrations(conn: &Connection) -> Result<(), String> {
    println!("[DB] run: init schema");
    conn.execute_batch(include_str!("0001_init.sql"))
        .map_err(|e| e.to_string())?;
    println!("[DB] run: done");
    Ok(())
}
