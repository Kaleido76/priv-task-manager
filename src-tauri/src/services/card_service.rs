use crate::database::Database;
use crate::models::task::TaskCard;
use crate::repository::card_repo;

pub fn list_cards(db: &Database, task_id: i64) -> Result<Vec<TaskCard>, String> {
    card_repo::find_by_task_id(db, task_id)
}

pub fn create_card(db: &Database, task_id: i64, card_type: &str, data: &str) -> Result<TaskCard, String> {
    let now = chrono::Utc::now().to_rfc3339();
    card_repo::insert(db, task_id, card_type, data, &now)
}

pub fn update_card(db: &Database, id: i64, data: &str) -> Result<TaskCard, String> {
    let now = chrono::Utc::now().to_rfc3339();
    card_repo::update(db, id, data, &now)
}

pub fn delete_card(db: &Database, id: i64) -> Result<(), String> {
    card_repo::delete(db, id)
}

pub fn reorder_cards(db: &Database, ids: &[i64]) -> Result<(), String> {
    card_repo::reorder(db, ids)
}
