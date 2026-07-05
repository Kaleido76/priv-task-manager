use serde::{Deserialize, Serialize};

use crate::database::Database;
use crate::models::task::{Task, TaskContent, TaskLog};
use crate::repository::{task_log_repo, task_repo};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTaskRequest {
    pub id: i64,
    pub title: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub recipient: Option<String>,
    pub progress: Option<i32>,
    pub deadline: Option<String>,
}

pub fn create_task(db: &Database, project_id: i64, title: &str) -> Result<Task, String> {
    let now = chrono::Utc::now().to_rfc3339();
    let task = Task {
        id: 0,
        project_id,
        title: title.to_string(),
        status: "todo".to_string(),
        priority: "medium".to_string(),
        recipient: None,
        progress: 0,
        deadline: None,
        create_time: now.clone(),
        update_time: now,
    };
    let task = task_repo::insert(db, &task)?;

    let content = TaskContent {
        task_id: task.id,
        note: String::new(),
    };
    task_repo::upsert_content(db, &content)?;

    Ok(task)
}

pub fn get_task(db: &Database, id: i64) -> Result<Option<Task>, String> {
    task_repo::find_by_id(db, id)
}

pub fn update_task(db: &Database, req: &UpdateTaskRequest) -> Result<Task, String> {
    let mut task = task_repo::find_by_id(db, req.id)?
        .ok_or_else(|| "Task not found".to_string())?;

    if let Some(title) = &req.title {
        task.title = title.clone();
    }
    if let Some(status) = &req.status {
        task.status = status.clone();
    }
    if let Some(priority) = &req.priority {
        task.priority = priority.clone();
    }
    if let Some(recipient) = &req.recipient {
        task.recipient = Some(recipient.clone());
    }
    if let Some(progress) = req.progress {
        task.progress = progress;
    }
    if let Some(deadline) = &req.deadline {
        task.deadline = Some(deadline.clone());
    }

    task.update_time = chrono::Utc::now().to_rfc3339();
    task_repo::update(db, &task)
}

pub fn delete_task(db: &Database, id: i64) -> Result<(), String> {
    task_repo::delete(db, id)
}

pub fn list_tasks(db: &Database, project_id: i64) -> Result<Vec<Task>, String> {
    task_repo::find_by_project_id(db, project_id)
}

pub fn search_tasks(
    db: &Database,
    project_id: i64,
    keyword: Option<&str>,
    status: Option<&str>,
    priority: Option<&str>,
) -> Result<Vec<Task>, String> {
    task_repo::search(db, project_id, keyword, status, priority)
}

pub fn get_task_content(db: &Database, task_id: i64) -> Result<Option<TaskContent>, String> {
    task_repo::find_content_by_task_id(db, task_id)
}

pub fn update_task_content(db: &Database, task_id: i64, note: &str) -> Result<(), String> {
    let content = TaskContent {
        task_id,
        note: note.to_string(),
    };
    task_repo::upsert_content(db, &content)?;
    Ok(())
}

pub fn get_task_logs(db: &Database, task_id: i64) -> Result<Vec<TaskLog>, String> {
    task_log_repo::find_by_task_id(db, task_id)
}

pub fn add_task_log(db: &Database, task_id: i64, content: &str) -> Result<TaskLog, String> {
    let log = TaskLog {
        id: 0,
        task_id,
        content: content.to_string(),
        create_time: chrono::Utc::now().to_rfc3339(),
    };
    task_log_repo::insert(db, &log)
}

pub fn delete_task_log(db: &Database, log_id: i64) -> Result<(), String> {
    task_log_repo::delete(db, log_id)
}

pub fn delete_tasks(db: &Database, ids: &[i64]) -> Result<(), String> {
    task_repo::delete_many(db, ids)
}

pub fn move_tasks(db: &Database, ids: &[i64], project_id: i64) -> Result<(), String> {
    task_repo::move_to_project(db, ids, project_id)
}
