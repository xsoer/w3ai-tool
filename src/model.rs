use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Chart {
    pub id: i32,
    pub chart_title: String,
    pub chart_tags: String,
}