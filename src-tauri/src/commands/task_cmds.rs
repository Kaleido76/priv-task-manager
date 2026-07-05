use crate::database::Database;
use crate::models::task::{Task, TaskContent, TaskLog};
use crate::services::task_service::{self, UpdateTaskRequest};

#[tauri::command]
pub fn get_tasks(db: tauri::State<Database>, project_id: i64) -> Result<Vec<Task>, String> {
    println!("[CMD] get_tasks: project_id={}", project_id);
    let r = task_service::list_tasks(db.inner(), project_id);
    println!("[CMD] get_tasks: result={:?}", r.as_ref().map(|v| v.len()));
    r
}

#[tauri::command]
pub fn create_task(db: tauri::State<Database>, project_id: i64, title: String) -> Result<Task, String> {
    println!("[CMD] create_task: project_id={}, title={:?}", project_id, title);
    let r = task_service::create_task(db.inner(), project_id, &title);
    println!("[CMD] create_task: result={:?}", r.as_ref().map(|t| t.id));
    r
}

#[tauri::command]
pub fn update_task(db: tauri::State<Database>, req: UpdateTaskRequest) -> Result<Task, String> {
    println!("[CMD] update_task: id={}", req.id);
    let r = task_service::update_task(db.inner(), &req);
    println!("[CMD] update_task: result={:?}", r.as_ref().map(|t| t.id));
    r
}

#[tauri::command]
pub fn delete_task(db: tauri::State<Database>, id: i64) -> Result<(), String> {
    println!("[CMD] delete_task: id={}", id);
    let r = task_service::delete_task(db.inner(), id);
    println!("[CMD] delete_task: result={:?}", r);
    r
}

#[tauri::command]
pub fn search_tasks(
    db: tauri::State<Database>,
    project_id: i64,
    keyword: Option<String>,
    status: Option<String>,
    priority: Option<String>,
) -> Result<Vec<Task>, String> {
    println!("[CMD] search_tasks: project_id={}, keyword={:?}, status={:?}, priority={:?}",
        project_id, keyword, status, priority);
    let r = task_service::search_tasks(db.inner(), project_id, keyword.as_deref(), status.as_deref(), priority.as_deref());
    println!("[CMD] search_tasks: count={:?}", r.as_ref().map(|v| v.len()));
    r
}

#[tauri::command]
pub fn get_task_content(db: tauri::State<Database>, task_id: i64) -> Result<Option<TaskContent>, String> {
    println!("[CMD] get_task_content: task_id={}", task_id);
    let r = task_service::get_task_content(db.inner(), task_id);
    println!("[CMD] get_task_content: has_content={}", r.as_ref().ok().and_then(|c| c.as_ref()).is_some());
    r
}

#[tauri::command]
pub fn update_task_content(db: tauri::State<Database>, task_id: i64, note: String) -> Result<(), String> {
    println!("[CMD] update_task_content: task_id={}, note_len={}", task_id, note.len());
    let r = task_service::update_task_content(db.inner(), task_id, &note);
    println!("[CMD] update_task_content: result={:?}", r);
    r
}

#[tauri::command]
pub fn get_task_logs(db: tauri::State<Database>, task_id: i64) -> Result<Vec<TaskLog>, String> {
    println!("[CMD] get_task_logs: task_id={}", task_id);
    let r = task_service::get_task_logs(db.inner(), task_id);
    println!("[CMD] get_task_logs: count={:?}", r.as_ref().map(|v| v.len()));
    r
}

#[tauri::command]
pub fn add_task_log(db: tauri::State<Database>, task_id: i64, content: String) -> Result<TaskLog, String> {
    println!("[CMD] add_task_log: task_id={}, content_len={}", task_id, content.len());
    let r = task_service::add_task_log(db.inner(), task_id, &content);
    println!("[CMD] add_task_log: result={:?}", r.as_ref().map(|l| l.id));
    r
}

#[tauri::command]
pub fn delete_task_log(db: tauri::State<Database>, log_id: i64) -> Result<(), String> {
    println!("[CMD] delete_task_log: log_id={}", log_id);
    let r = task_service::delete_task_log(db.inner(), log_id);
    println!("[CMD] delete_task_log: result={:?}", r);
    r
}

#[tauri::command]
pub fn delete_tasks(db: tauri::State<Database>, ids: Vec<i64>) -> Result<(), String> {
    println!("[CMD] delete_tasks: ids={:?}", ids);
    let r = task_service::delete_tasks(db.inner(), &ids);
    println!("[CMD] delete_tasks: result={:?}", r);
    r
}

#[tauri::command]
pub fn move_tasks(db: tauri::State<Database>, ids: Vec<i64>, project_id: i64) -> Result<(), String> {
    println!("[CMD] move_tasks: ids={:?}, project_id={}", ids, project_id);
    let r = task_service::move_tasks(db.inner(), &ids, project_id);
    println!("[CMD] move_tasks: result={:?}", r);
    r
}
