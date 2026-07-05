use crate::database::Database;
use crate::models::project::Project;
use crate::services::project_service;

#[tauri::command]
pub fn get_projects(db: tauri::State<Database>) -> Result<Vec<Project>, String> {
    println!("[CMD] get_projects");
    let r = project_service::list_projects(db.inner());
    println!("[CMD] get_projects: count={:?}", r.as_ref().map(|v| v.len()));
    r
}

#[tauri::command]
pub fn create_project(db: tauri::State<Database>, name: String) -> Result<Project, String> {
    println!("[CMD] create_project: name={:?}", name);
    let r = project_service::create_project(db.inner(), &name);
    println!("[CMD] create_project: result={:?}", r.as_ref().map(|p| p.id));
    r
}

#[tauri::command]
pub fn rename_project(db: tauri::State<Database>, id: i64, name: String) -> Result<Project, String> {
    println!("[CMD] rename_project: id={}, name={:?}", id, name);
    let r = project_service::rename_project(db.inner(), id, &name);
    println!("[CMD] rename_project: result={:?}", r.as_ref().map(|p| p.id));
    r
}

#[tauri::command]
pub fn update_project_description(db: tauri::State<Database>, id: i64, description: String) -> Result<Project, String> {
    println!("[CMD] update_project_description: id={}, desc_len={}", id, description.len());
    let r = project_service::update_project_description(db.inner(), id, &description);
    println!("[CMD] update_project_description: result={:?}", r.as_ref().map(|p| p.id));
    r
}

#[tauri::command]
pub fn update_project_color(db: tauri::State<Database>, id: i64, color: Option<String>) -> Result<Project, String> {
    println!("[CMD] update_project_color: id={}, color={:?}", id, color);
    let r = project_service::update_project_color(db.inner(), id, color);
    println!("[CMD] update_project_color: result={:?}", r.as_ref().map(|p| p.id));
    r
}

#[tauri::command]
pub fn delete_project(db: tauri::State<Database>, id: i64) -> Result<(), String> {
    println!("[CMD] delete_project: id={}", id);
    let r = project_service::delete_project(db.inner(), id);
    println!("[CMD] delete_project: result={:?}", r);
    r
}

#[tauri::command]
pub fn reorder_projects(db: tauri::State<Database>, ids: Vec<i64>) -> Result<(), String> {
    println!("[CMD] reorder_projects: ids={:?}", ids);
    let r = project_service::reorder_projects(db.inner(), &ids);
    println!("[CMD] reorder_projects: result={:?}", r);
    r
}
