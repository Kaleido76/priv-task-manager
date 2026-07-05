use crate::database::Database;
use crate::models::task::TaskLog;

pub fn find_by_task_id(db: &Database, task_id: i64) -> Result<Vec<TaskLog>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT id, task_id, content, create_time FROM task_logs WHERE task_id = ?1 ORDER BY create_time ASC",
        )
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map(rusqlite::params![task_id], |row| {
            Ok(TaskLog {
                id: row.get(0)?,
                task_id: row.get(1)?,
                content: row.get(2)?,
                create_time: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?;
    let mut logs = Vec::new();
    for row in rows {
        logs.push(row.map_err(|e| e.to_string())?);
    }
    Ok(logs)
}

pub fn insert(db: &Database, log: &TaskLog) -> Result<TaskLog, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO task_logs (task_id, content, create_time) VALUES (?1, ?2, ?3)",
        rusqlite::params![log.task_id, log.content, log.create_time],
    )
    .map_err(|e| e.to_string())?;
    let id = conn.last_insert_rowid();
    Ok(TaskLog {
        id,
        task_id: log.task_id,
        content: log.content.clone(),
        create_time: log.create_time.clone(),
    })
}

pub fn delete(db: &Database, id: i64) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "DELETE FROM task_logs WHERE id = ?1",
        rusqlite::params![id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn search(db: &Database, task_id: i64, keyword: &str) -> Result<Vec<TaskLog>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT id, task_id, content, create_time FROM task_logs \
             WHERE task_id = ?1 AND content LIKE ?2 ORDER BY create_time ASC",
        )
        .map_err(|e| e.to_string())?;
    let pattern = format!("%{}%", keyword);
    let rows = stmt
        .query_map(rusqlite::params![task_id, pattern], |row| {
            Ok(TaskLog {
                id: row.get(0)?,
                task_id: row.get(1)?,
                content: row.get(2)?,
                create_time: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?;
    let mut logs = Vec::new();
    for row in rows {
        logs.push(row.map_err(|e| e.to_string())?);
    }
    Ok(logs)
}
