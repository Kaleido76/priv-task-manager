use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub project_id: i64,
    pub title: String,
    pub status: String,
    pub priority: String,
    pub recipient: Option<String>,
    pub deadline: Option<String>,
    pub create_time: String,
    pub update_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskContent {
    pub task_id: i64,
    pub note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskLog {
    pub id: i64,
    pub task_id: i64,
    pub content: String,
    pub create_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCard {
    pub id: i64,
    pub task_id: i64,
    pub card_type: String,
    pub sort_order: i64,
    pub data: String,
    pub create_time: String,
    pub update_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCardResponse {
    pub id: i64,
    pub task_id: i64,
    pub card_type: String,
    pub sort_order: i64,
    pub data: serde_json::Value,
    pub create_time: String,
    pub update_time: String,
}

impl From<TaskCard> for TaskCardResponse {
    fn from(card: TaskCard) -> Self {
        TaskCardResponse {
            id: card.id,
            task_id: card.task_id,
            card_type: card.card_type,
            sort_order: card.sort_order,
            data: serde_json::from_str(&card.data).unwrap_or(serde_json::Value::Null),
            create_time: card.create_time,
            update_time: card.update_time,
        }
    }
}
