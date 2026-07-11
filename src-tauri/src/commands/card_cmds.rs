use crate::database::Database;
use crate::models::task::TaskCardResponse;
use crate::services::card_service;

#[tauri::command]
pub fn get_task_cards(db: tauri::State<Database>, task_id: i64) -> Result<Vec<TaskCardResponse>, String> {
    card_service::list_cards(db.inner(), task_id)
        .map(|cards| cards.into_iter().map(|c| c.into()).collect())
}

#[tauri::command]
pub fn create_task_card(db: tauri::State<Database>, task_id: i64, card_type: String, data: serde_json::Value) -> Result<TaskCardResponse, String> {
    let data_str = data.to_string();
    card_service::create_card(db.inner(), task_id, &card_type, &data_str)
        .map(|c| c.into())
}

#[tauri::command]
pub fn update_task_card(db: tauri::State<Database>, id: i64, data: serde_json::Value) -> Result<TaskCardResponse, String> {
    let data_str = data.to_string();
    card_service::update_card(db.inner(), id, &data_str)
        .map(|c| c.into())
}

#[tauri::command]
pub fn delete_task_card(db: tauri::State<Database>, id: i64) -> Result<(), String> {
    card_service::delete_card(db.inner(), id)
}

#[tauri::command]
pub fn reorder_task_cards(db: tauri::State<Database>, ids: Vec<i64>) -> Result<(), String> {
    card_service::reorder_cards(db.inner(), &ids)
}

#[tauri::command]
pub fn open_file_location(path: String) -> Result<(), String> {
    std::process::Command::new("explorer")
        .arg(format!("/select,{}", path))
        .spawn()
        .map_err(|e| format!("Failed to open file location: {}", e))?;
    Ok(())
}

#[tauri::command]
pub fn open_url(url: String) -> Result<(), String> {
    let normalized = if url.starts_with("http://") || url.starts_with("https://") {
        url
    } else {
        format!("https://{}", url)
    };
    std::process::Command::new("cmd")
        .args(["/c", "start", "", &normalized])
        .spawn()
        .map_err(|e| format!("Failed to open URL: {}", e))?;
    Ok(())
}

#[tauri::command]
pub fn check_path_exists(path: String) -> Result<bool, String> {
    let p = std::path::Path::new(&path);
    Ok(p.exists())
}
