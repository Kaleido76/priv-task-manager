use rusqlite::Connection;

const MIGRATIONS: &[(i32, &str, &str)] = &[
    (1, "init", include_str!("001_init.sql")),
];

pub fn run_migrations(conn: &Connection) -> Result<(), String> {
    let current_version: i32 = conn
        .pragma_query_value(None, "user_version", |row| row.get(0))
        .map_err(|e| e.to_string())?;

    for (version, name, sql) in MIGRATIONS {
        if *version <= current_version {
            continue;
        }
        println!("[DB] run migration v{}: {}", version, name);
        conn.execute_batch(sql).map_err(|e| e.to_string())?;
        conn.execute_batch(&format!("PRAGMA user_version = {};", version))
            .map_err(|e| e.to_string())?;
        println!("[DB] migration v{} done", version);
    }

    Ok(())
}
