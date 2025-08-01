// src/services/order_service.rs

use crate::{
    errors::AppError,
    models::{order::{PurchaseOrder, UpdateOrderStatusDto}, user::Claims},
};
use sqlx::MySqlPool;
pub async fn get_orders_for_user(pool: &MySqlPool, claims: &Claims) -> Result<Vec<PurchaseOrder>, AppError> {
    let sql_query = if claims.company_type == "BUYER" {
        "SELECT po.*, r.title as rfq_title, b.name as buyer_name, s.name as supplier_name
         FROM purchase_orders po
         JOIN rfqs r ON po.rfq_id = r.id
         JOIN companies b ON po.buyer_company_id = b.id
         JOIN companies s ON po.supplier_company_id = s.id
         WHERE po.buyer_company_id = ? ORDER BY po.created_at DESC"
    } else {
        "SELECT po.*, r.title as rfq_title, b.name as buyer_name, s.name as supplier_name
         FROM purchase_orders po
         JOIN rfqs r ON po.rfq_id = r.id
         JOIN companies b ON po.buyer_company_id = b.id
         JOIN companies s ON po.supplier_company_id = s.id
         WHERE po.supplier_company_id = ? ORDER BY po.created_at DESC"
    };

    let orders = sqlx::query_as(sql_query)
        .bind(claims.company_id)
        .fetch_all(pool)
        .await?;

    Ok(orders)
}

pub async fn update_order_status(
    pool: &MySqlPool,
    order_id: i32,
    dto: UpdateOrderStatusDto,
    claims: &Claims,
) -> Result<u64, AppError> {
    // 权限检查：只有供应商才能更新订单状态
    if claims.company_type != "SUPPLIER" {
        return Err(AppError::BadRequest("Only suppliers can update order status.".to_string()));
    }

    // 验证新状态是否合法
    let new_status = dto.status.as_str();
    if !matches!(new_status, "IN_PRODUCTION" | "SHIPPED" | "COMPLETED") {
        return Err(AppError::BadRequest("Invalid status provided.".to_string()));
    }

    // 执行更新，并确保该供应商确实是此订单的供应商
    let result = sqlx::query(
        "UPDATE purchase_orders SET status = ? WHERE id = ? AND supplier_company_id = ?"
    )
        .bind(new_status)
        .bind(order_id)
        .bind(claims.company_id)
        .execute(pool)
        .await?;

    // 如果没有行被影响，说明订单不存在或该供应商无权修改
    if result.rows_affected() == 0 {
        return Err(AppError::BadRequest("Order not found or you are not authorized to update it.".to_string()));
    }

    Ok(result.rows_affected())
}