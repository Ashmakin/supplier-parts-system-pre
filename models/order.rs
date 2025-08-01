// src/models/order.rs

use serde::{Deserialize, Serialize};
use sqlx::{types::Decimal, FromRow};
use chrono::{DateTime, Utc};

// 用于自定义Decimal的序列化
mod decimal_as_string {
    use super::*;
    pub fn serialize<S>(value: &Decimal, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }
}

#[derive(Debug, Serialize, FromRow)]
pub struct PurchaseOrder {
    pub id: i32,
    pub rfq_id: i32,
    #[sqlx(default)] // 这个字段来自JOIN
    pub rfq_title: String,
    pub buyer_company_id: i32,
    #[sqlx(default)] // 这个字段来自JOIN
    pub buyer_name: String,
    pub supplier_company_id: i32,
    #[sqlx(default)] // 这个字段来自JOIN
    pub supplier_name: String,
    #[serde(with = "decimal_as_string")]
    pub total_amount: Decimal,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub payment_status: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateOrderStatusDto {
    pub status: String,
}