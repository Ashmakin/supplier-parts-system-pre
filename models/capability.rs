// src/models/capability.rs

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct Capability {
    pub id: i32,
    pub name: String,
    pub category: String,
}

#[derive(Debug, Deserialize)]
pub struct AddCompanyCapabilityDto {
    pub capability_id: i32,
}