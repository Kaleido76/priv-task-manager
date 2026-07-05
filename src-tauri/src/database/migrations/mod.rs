use rusqlite::Connection;
use std::collections::HashSet;

const MIGRATIONS: &[(&str, &str)] = &[
    ("0001_init", include_str!("0001_init.sql")),
    ("0002_add_project_fields", include_str!("0002_add_project_fields.sql")),
    ("0003_rename_owner_to_recipient", include_str!("0003_rename_owner_to_recipient.sql")),
];

pub fn run_migrations(conn: &Connection) -> Result<(), String> {
    println!("[DB] run_migrations: start");
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS _migrations (version TEXT PRIMARY KEY, applied_at TEXT NOT NULL);"
    ).map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare("SELECT version FROM _migrations ORDER BY version")
        .map_err(|e| e.to_string())?;
    let applied: HashSet<String> = stmt.query_map([], |row| row.get::<_, String>(0))
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    println!("[DB] run_migrations: already applied={:?}", applied);

    for (version, sql) in MIGRATIONS {
        if applied.contains(*version) {
            println!("[DB] run_migrations: skip {}", version);
            continue;
        }

        println!("[DB] run_migrations: applying {}", version);
        let tx = conn.unchecked_transaction().map_err(|e| e.to_string())?;
        tx.execute_batch(sql).map_err(|e| e.to_string())?;
        let now = chrono::Utc::now().to_rfc3339();
        tx.execute(
            "INSERT INTO _migrations (version, applied_at) VALUES (?1, ?2)",
            rusqlite::params![version, now],
        ).map_err(|e| e.to_string())?;
        tx.commit().map_err(|e| e.to_string())?;
        println!("[DB] run_migrations: applied {} OK", version);
    }

    println!("[DB] run_migrations: done");
    Ok(())
}
