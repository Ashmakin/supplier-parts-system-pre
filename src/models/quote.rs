// src/models/quote.rs
use serde::{Deserialize, Serialize};
use sqlx::{types::Decimal, FromRow};
use chrono::{DateTime, Utc};

//sqlx竟然没有decimal序列化，只能自己实现一个
mod decimal_as_string {
    use super::*;
    pub fn serialize<S>(value: &Decimal, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }
}

#[derive(Debug, Serialize, FromRow)]
pub struct Quote {
    pub id: i32,
    pub rfq_id: i32,
    pub supplier_company_id: i32,
    #[serde(with = "decimal_as_string")]
    pub price: Decimal,
    pub lead_time_days: i32,
    pub notes: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    // 这个字段通过JOIN查询得到
    #[sqlx(default)]
    pub supplier_company_name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateQuoteDto {
    pub price: f64,
    pub lead_time_days: i32,
    pub notes: Option<String>,
}