use crate::database::Database;
use crate::models::task::TaskCard;

pub fn find_by_task_id(db: &Database, task_id: i64) -> Result<Vec<TaskCard>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT id, task_id, card_type, sort_order, data, create_time, update_time \
             FROM task_cards WHERE task_id = ?1 ORDER BY sort_order ASC",
        )
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map(rusqlite::params![task_id], |row| {
            Ok(TaskCard {
                id: row.get(0)?,
                task_id: row.get(1)?,
                card_type: row.get(2)?,
                sort_order: row.get(3)?,
                data: row.get(4)?,
                create_time: row.get(5)?,
                update_time: row.get(6)?,
            })
        })
        .map_err(|e| e.to_string())?;
    let mut cards = Vec::new();
    for row in rows {
        cards.push(row.map_err(|e| e.to_string())?);
    }
    Ok(cards)
}

pub fn insert(db: &Database, task_id: i64, card_type: &str, data: &str, now: &str) -> Result<TaskCard, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let max_order: i64 = conn
        .query_row(
            "SELECT COALESCE(MAX(sort_order), -1) FROM task_cards WHERE task_id = ?1",
            rusqlite::params![task_id],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;
    let new_order = max_order + 1;

    conn.execute(
        "INSERT INTO task_cards (task_id, card_type, sort_order, data, create_time, update_time) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        rusqlite::params![task_id, card_type, new_order, data, now, now],
    )
    .map_err(|e| e.to_string())?;
    let id = conn.last_insert_rowid();
    Ok(TaskCard {
        id,
        task_id,
        card_type: card_type.to_string(),
        sort_order: new_order,
        data: data.to_string(),
        create_time: now.to_string(),
        update_time: now.to_string(),
    })
}

pub fn update(db: &Database, id: i64, data: &str, now: &str) -> Result<TaskCard, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE task_cards SET data = ?1, update_time = ?2 WHERE id = ?3",
        rusqlite::params![data, now, id],
    )
    .map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT id, task_id, card_type, sort_order, data, create_time, update_time \
             FROM task_cards WHERE id = ?1",
        )
        .map_err(|e| e.to_string())?;
    let card = stmt
        .query_row(rusqlite::params![id], |row| {
            Ok(TaskCard {
                id: row.get(0)?,
                task_id: row.get(1)?,
                card_type: row.get(2)?,
                sort_order: row.get(3)?,
                data: row.get(4)?,
                create_time: row.get(5)?,
                update_time: row.get(6)?,
            })
        })
        .map_err(|e| e.to_string())?;
    Ok(card)
}

pub fn delete(db: &Database, id: i64) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM task_cards WHERE id = ?1", rusqlite::params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn reorder(db: &Database, ids: &[i64]) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    for (i, id) in ids.iter().enumerate() {
        conn.execute(
            "UPDATE task_cards SET sort_order = ?1 WHERE id = ?2",
            rusqlite::params![i as i64, id],
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}
