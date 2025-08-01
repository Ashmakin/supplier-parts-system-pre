// src/models/chat.rs
use serde::Serialize;
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, FromRow)]
pub struct ChatMessage {
    pub id: i32,
    pub rfq_id: i32,
    pub user_id: i32,
    pub user_full_name: String,
    pub company_name: String,
    pub message_text: String,
    pub created_at: DateTime<Utc>,
}