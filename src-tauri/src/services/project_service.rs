use crate::database::Database;
use crate::models::project::Project;
use crate::repository::project_repo;

pub fn create_project(db: &Database, name: &str) -> Result<Project, String> {
    let now = chrono::Utc::now().to_rfc3339();
    let max_order = project_repo::get_max_sort_order(db)?;
    let project = Project {
        id: 0,
        name: name.to_string(),
        description: String::new(),
        color: None,
        sort_order: max_order + 1,
        create_time: now.clone(),
        update_time: Some(now),
    };
    project_repo::insert(db, &project)
}

pub fn rename_project(db: &Database, id: i64, name: &str) -> Result<Project, String> {
    let mut project = project_repo::find_by_id(db, id)?
        .ok_or_else(|| "Project not found".to_string())?;
    project.name = name.to_string();
    project.update_time = Some(chrono::Utc::now().to_rfc3339());
    project_repo::update(db, &project)
}

pub fn update_project_description(db: &Database, id: i64, description: &str) -> Result<Project, String> {
    let mut project = project_repo::find_by_id(db, id)?
        .ok_or_else(|| "Project not found".to_string())?;
    project.description = description.to_string();
    project.update_time = Some(chrono::Utc::now().to_rfc3339());
    project_repo::update(db, &project)
}

pub fn update_project_color(db: &Database, id: i64, color: Option<String>) -> Result<Project, String> {
    let mut project = project_repo::find_by_id(db, id)?
        .ok_or_else(|| "Project not found".to_string())?;
    project.color = color;
    project.update_time = Some(chrono::Utc::now().to_rfc3339());
    project_repo::update(db, &project)
}

pub fn delete_project(db: &Database, id: i64) -> Result<(), String> {
    project_repo::delete(db, id)
}

pub fn list_projects(db: &Database) -> Result<Vec<Project>, String> {
    project_repo::find_all(db)
}

pub fn reorder_projects(db: &Database, ids: &[i64]) -> Result<(), String> {
    project_repo::update_sort_order(db, ids)
}
