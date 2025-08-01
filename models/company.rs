// src/models/company.rs

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, FromRow)]
pub struct CompanyProfile {
    pub id: i32,
    pub name: String,
    pub company_type: String,
    pub city: Option<String>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub is_verified: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCompanyDto {
    // 目前只允许更新简介
    pub description: String,
}