// src/services/analytics_service.rs

use crate::{
    errors::AppError,
    models::{analytics::{BuyerStats, SpendingBySupplier, }, user::Claims},
};
use sqlx::MySqlPool;
use crate::models::analytics::SupplierStats;

pub async fn get_buyer_dashboard_stats(pool: &MySqlPool, claims: &Claims) -> Result<BuyerStats, AppError> {
    // 权限检查
    if claims.company_type != "BUYER" {
        return Err(AppError::BadRequest("Analytics are only available for buyers.".to_string()));
    }

    // 使用SQL聚合函数 COUNT, SUM, COUNT(DISTINCT)
    let stats = sqlx::query_as(
        "SELECT
            COUNT(*) as total_orders,
            COALESCE(SUM(total_amount), 0) as total_spent,
            COUNT(DISTINCT supplier_company_id) as distinct_suppliers
         FROM purchase_orders
         WHERE buyer_company_id = ?"
    )
        .bind(claims.company_id)
        .fetch_one(pool)
        .await?;

    Ok(stats)
}

pub async fn get_buyer_spending_by_supplier(pool: &MySqlPool, claims: &Claims) -> Result<Vec<SpendingBySupplier>, AppError> {
    if claims.company_type != "BUYER" {
        return Err(AppError::BadRequest("Analytics are only available for buyers.".to_string()));
    }

    // 使用 GROUP BY 和 JOIN 来按供应商分组统计支出
    let spending_data = sqlx::query_as(
        "SELECT
            c.name as supplier_name,
            SUM(po.total_amount) as total
         FROM purchase_orders po
         JOIN companies c ON po.supplier_company_id = c.id
         WHERE po.buyer_company_id = ?
         GROUP BY po.supplier_company_id, c.name
         ORDER BY total DESC"
    )
        .bind(claims.company_id)
        .fetch_all(pool)
        .await?;

    Ok(spending_data)
}

pub async fn get_supplier_dashboard_stats(pool: &MySqlPool, claims: &Claims) -> Result<SupplierStats, AppError> {
    if claims.company_type != "SUPPLIER" {
        return Err(AppError::BadRequest("Analytics are only available for suppliers.".to_string()));
    }

    // 【关键修复】重写SQL查询，使其始终返回一行
    let stats: SupplierStats = sqlx::query_as(
        "SELECT
            (SELECT COUNT(*) FROM quotes WHERE supplier_company_id = ?) as total_quotes_submitted,
            (SELECT COUNT(*) FROM quotes WHERE supplier_company_id = ? AND status = 'ACCEPTED') as accepted_quotes,
            (SELECT COALESCE(SUM(po.total_amount), 0) FROM purchase_orders po WHERE po.supplier_company_id = ? AND po.status = 'COMPLETED') as total_revenue
        "
    )
        .bind(claims.company_id)
        .bind(claims.company_id)
        .bind(claims.company_id)
        .fetch_one(pool)
        .await?;

    Ok(stats)
}