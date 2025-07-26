// src/models/rfq.rs
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, FromRow)]
pub struct Rfq {
    pub id: i32,
    pub buyer_company_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub quantity: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
    // 这个字段通过JOIN查询得到
    #[sqlx(default)]
    pub buyer_company_name: String,
    #[sqlx(default)] // <-- 新增
    pub city: Option<String>, // 公司城市可能为空
}

#[derive(Debug, Deserialize)]
pub struct CreateRfqDto {
    pub title: String,
    pub description: Option<String>,
    pub quantity: i32,
}
#[derive(Debug, Serialize, FromRow)]
pub struct RfqAttachment {
    pub id: i32,
    pub rfq_id: i32,
    pub original_filename: String,
    pub stored_path: String,
}