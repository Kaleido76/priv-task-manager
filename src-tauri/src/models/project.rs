use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub color: Option<String>,
    pub sort_order: i32,
    pub create_time: String,
    pub update_time: Option<String>,
}
