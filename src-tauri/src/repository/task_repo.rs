use crate::database::Database;
use crate::models::task::{Task, TaskContent};

pub fn find_by_project_id(db: &Database, project_id: i64) -> Result<Vec<Task>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT id, project_id, title, status, priority, recipient, progress, deadline, create_time, update_time \
             FROM tasks WHERE project_id = ?1 ORDER BY update_time DESC",
        )
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map(rusqlite::params![project_id], |row| {
            Ok(Task {
                id: row.get(0)?,
                project_id: row.get(1)?,
                title: row.get(2)?,
                status: row.get(3)?,
                priority: row.get(4)?,
                recipient: row.get(5)?,
                progress: row.get(6)?,
                deadline: row.get(7)?,
                create_time: row.get(8)?,
                update_time: row.get(9)?,
            })
        })
        .map_err(|e| e.to_string())?;
    let mut tasks = Vec::new();
    for row in rows {
        tasks.push(row.map_err(|e| e.to_string())?);
    }
    Ok(tasks)
}

pub fn find_by_id(db: &Database, id: i64) -> Result<Option<Task>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT id, project_id, title, status, priority, recipient, progress, deadline, create_time, update_time \
             FROM tasks WHERE id = ?1",
        )
        .map_err(|e| e.to_string())?;
    let mut rows = stmt
        .query_map(rusqlite::params![id], |row| {
            Ok(Task {
                id: row.get(0)?,
                project_id: row.get(1)?,
                title: row.get(2)?,
                status: row.get(3)?,
                priority: row.get(4)?,
                recipient: row.get(5)?,
                progress: row.get(6)?,
                deadline: row.get(7)?,
                create_time: row.get(8)?,
                update_time: row.get(9)?,
            })
        })
        .map_err(|e| e.to_string())?;
    match rows.next() {
        Some(Ok(task)) => Ok(Some(task)),
        Some(Err(e)) => Err(e.to_string()),
        None => Ok(None),
    }
}

pub fn insert(db: &Database, task: &Task) -> Result<Task, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO tasks (project_id, title, status, priority, recipient, progress, deadline, create_time, update_time) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        rusqlite::params![
            task.project_id,
            task.title,
            task.status,
            task.priority,
            task.recipient,
            task.progress,
            task.deadline,
            task.create_time,
            task.update_time,
        ],
    )
    .map_err(|e| e.to_string())?;
    let id = conn.last_insert_rowid();
    Ok(Task {
        id,
        project_id: task.project_id,
        title: task.title.clone(),
        status: task.status.clone(),
        priority: task.priority.clone(),
        recipient: task.recipient.clone(),
        progress: task.progress,
        deadline: task.deadline.clone(),
        create_time: task.create_time.clone(),
        update_time: task.update_time.clone(),
    })
}

pub fn update(db: &Database, task: &Task) -> Result<Task, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE tasks SET project_id = ?1, title = ?2, status = ?3, priority = ?4, recipient = ?5, \
         progress = ?6, deadline = ?7, update_time = ?8 WHERE id = ?9",
        rusqlite::params![
            task.project_id,
            task.title,
            task.status,
            task.priority,
            task.recipient,
            task.progress,
            task.deadline,
            task.update_time,
            task.id,
        ],
    )
    .map_err(|e| e.to_string())?;
    Ok(task.clone())
}

pub fn delete(db: &Database, id: i64) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM tasks WHERE id = ?1", rusqlite::params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn search(
    db: &Database,
    project_id: i64,
    keyword: Option<&str>,
    status: Option<&str>,
    priority: Option<&str>,
) -> Result<Vec<Task>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut sql = String::from(
        "SELECT id, project_id, title, status, priority, recipient, progress, deadline, create_time, update_time \
         FROM tasks WHERE project_id = ?1",
    );
    let mut param_index = 2;

    if keyword.is_some() {
        sql.push_str(&format!(" AND title LIKE ?{}", param_index));
        param_index += 1;
    }
    if status.is_some() {
        sql.push_str(&format!(" AND status = ?{}", param_index));
        param_index += 1;
    }
    if priority.is_some() {
        sql.push_str(&format!(" AND priority = ?{}", param_index));
    }

    sql.push_str(" ORDER BY update_time DESC");

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;

    let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> =
        vec![Box::new(project_id)];

    if let Some(kw) = keyword {
        param_values.push(Box::new(format!("%{}%", kw)));
    }
    if let Some(s) = status {
        param_values.push(Box::new(s.to_string()));
    }
    if let Some(p) = priority {
        param_values.push(Box::new(p.to_string()));
    }

    let params_ref: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();

    let rows = stmt
        .query_map(params_ref.as_slice(), |row| {
            Ok(Task {
                id: row.get(0)?,
                project_id: row.get(1)?,
                title: row.get(2)?,
                status: row.get(3)?,
                priority: row.get(4)?,
                recipient: row.get(5)?,
                progress: row.get(6)?,
                deadline: row.get(7)?,
                create_time: row.get(8)?,
                update_time: row.get(9)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut tasks = Vec::new();
    for row in rows {
        tasks.push(row.map_err(|e| e.to_string())?);
    }
    Ok(tasks)
}

pub fn find_content_by_task_id(db: &Database, task_id: i64) -> Result<Option<TaskContent>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT task_id, note FROM task_contents WHERE task_id = ?1")
        .map_err(|e| e.to_string())?;
    let mut rows = stmt
        .query_map(rusqlite::params![task_id], |row| {
            Ok(TaskContent {
                task_id: row.get(0)?,
                note: row.get(1)?,
            })
        })
        .map_err(|e| e.to_string())?;
    match rows.next() {
        Some(Ok(content)) => Ok(Some(content)),
        Some(Err(e)) => Err(e.to_string()),
        None => Ok(None),
    }
}

pub fn upsert_content(db: &Database, content: &TaskContent) -> Result<TaskContent, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO task_contents (task_id, note) VALUES (?1, ?2) \
         ON CONFLICT(task_id) DO UPDATE SET note = excluded.note",
        rusqlite::params![content.task_id, content.note],
    )
    .map_err(|e| e.to_string())?;
    Ok(content.clone())
}

pub fn delete_many(db: &Database, ids: &[i64]) -> Result<(), String> {
    if ids.is_empty() { return Ok(()); }
    let placeholders: Vec<String> = ids.iter().map(|_| "?".to_string()).collect();
    let sql = format!("DELETE FROM tasks WHERE id IN ({})", placeholders.join(","));
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let params: Vec<&dyn rusqlite::types::ToSql> = ids.iter().map(|id| id as &dyn rusqlite::types::ToSql).collect();
    conn.execute(&sql, params.as_slice()).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn move_to_project(db: &Database, ids: &[i64], project_id: i64) -> Result<(), String> {
    if ids.is_empty() { return Ok(()); }
    let now = chrono::Utc::now().to_rfc3339();
    let placeholders: Vec<String> = ids.iter().map(|_| "?".to_string()).collect();
    let sql = format!(
        "UPDATE tasks SET project_id = ?1, update_time = ?2 WHERE id IN ({})",
        placeholders.join(",")
    );
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = vec![
        Box::new(project_id),
        Box::new(now),
    ];
    for id in ids {
        params.push(Box::new(*id));
    }
    let params_ref: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    conn.execute(&sql, params_ref.as_slice()).map_err(|e| e.to_string())?;
    Ok(())
}
