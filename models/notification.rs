// src/models/notification.rs
use serde::Serialize;
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, FromRow, Clone)]
pub struct Notification {
    pub id: i32,
    pub recipient_user_id: i32,
    pub message: String,
    pub link_url: Option<String>,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
}