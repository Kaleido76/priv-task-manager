use crate::database::Database;
use crate::models::project::Project;

const SELECT_COLS: &str = "id, name, description, color, sort_order, create_time, update_time";

pub fn find_all(db: &Database) -> Result<Vec<Project>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(&format!("SELECT {} FROM projects ORDER BY sort_order", SELECT_COLS))
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                color: row.get(3)?,
                sort_order: row.get(4)?,
                create_time: row.get(5)?,
                update_time: row.get(6)?,
            })
        })
        .map_err(|e| e.to_string())?;
    let mut projects = Vec::new();
    for row in rows {
        projects.push(row.map_err(|e| e.to_string())?);
    }
    Ok(projects)
}

pub fn find_by_id(db: &Database, id: i64) -> Result<Option<Project>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(&format!("SELECT {} FROM projects WHERE id = ?1", SELECT_COLS))
        .map_err(|e| e.to_string())?;
    let mut rows = stmt
        .query_map(rusqlite::params![id], |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                color: row.get(3)?,
                sort_order: row.get(4)?,
                create_time: row.get(5)?,
                update_time: row.get(6)?,
            })
        })
        .map_err(|e| e.to_string())?;
    match rows.next() {
        Some(Ok(project)) => Ok(Some(project)),
        Some(Err(e)) => Err(e.to_string()),
        None => Ok(None),
    }
}

pub fn insert(db: &Database, project: &Project) -> Result<Project, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO projects (name, description, color, sort_order, create_time, update_time) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        rusqlite::params![project.name, project.description, project.color, project.sort_order, project.create_time, project.update_time],
    )
    .map_err(|e| e.to_string())?;
    let id = conn.last_insert_rowid();
    Ok(Project {
        id,
        name: project.name.clone(),
        description: project.description.clone(),
        color: project.color.clone(),
        sort_order: project.sort_order,
        create_time: project.create_time.clone(),
        update_time: project.update_time.clone(),
    })
}

pub fn update(db: &Database, project: &Project) -> Result<Project, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE projects SET name = ?1, description = ?2, color = ?3, sort_order = ?4, update_time = ?5 WHERE id = ?6",
        rusqlite::params![project.name, project.description, project.color, project.sort_order, project.update_time, project.id],
    )
    .map_err(|e| e.to_string())?;
    Ok(project.clone())
}

pub fn delete(db: &Database, id: i64) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM projects WHERE id = ?1", rusqlite::params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn update_sort_order(db: &Database, ids: &[i64]) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let tx = conn.unchecked_transaction().map_err(|e| e.to_string())?;
    for (i, id) in ids.iter().enumerate() {
        tx.execute(
            "UPDATE projects SET sort_order = ?1 WHERE id = ?2",
            rusqlite::params![i as i32, id],
        )
        .map_err(|e| e.to_string())?;
    }
    tx.commit().map_err(|e| e.to_string())?;
    Ok(())
}

pub fn get_max_sort_order(db: &Database) -> Result<i32, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let max: Option<i32> = conn
        .query_row("SELECT MAX(sort_order) FROM projects", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    Ok(max.unwrap_or(0))
}
