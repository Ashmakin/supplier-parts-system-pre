// src/models/analytics.rs

use serde::Serialize;
use sqlx::{types::Decimal, FromRow};

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
pub struct BuyerStats {
    pub total_orders: i64, // 使用 i64 以防订单数非常多
    #[serde(with = "decimal_as_string")]
    pub total_spent: Decimal,
    pub distinct_suppliers: i64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct SpendingBySupplier {
    pub supplier_name: String,
    #[serde(with = "decimal_as_string")]
    pub total: Decimal,
}