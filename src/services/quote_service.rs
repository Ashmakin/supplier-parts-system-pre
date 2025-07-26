// src/services/quote_service.rs
use crate::{
    errors::AppError,
    models::{quote::{CreateQuoteDto, Quote}, user::Claims},
};
use sqlx::{types::Decimal, MySqlPool, Row};
use std::str::FromStr;
use crate::services::notification_service;

pub async fn create_quote(pool: &MySqlPool, rfq_id: i32, dto: CreateQuoteDto, claims: &Claims) -> Result<u64, AppError> {
    // 权限检查：只有供应方(SUPPLIER)才能报价
    if claims.company_type != "SUPPLIER" {
        return Err(AppError::BadRequest("Only suppliers can create quotes".to_string()));
    }

    // 检查RFQ是否存在且开放
    let rfq: (i32,) = sqlx::query_as("SELECT id FROM rfqs WHERE id = ? AND status = 'OPEN'")
        .bind(rfq_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::BadRequest("RFQ not found or is not open for quotes".to_string()))?;

    let price_decimal = Decimal::from_str(&dto.price.to_string())
        .map_err(|_| AppError::BadRequest("Invalid price format".to_string()))?;



    let result = sqlx::query(
        "INSERT INTO quotes (rfq_id, supplier_company_id, price, lead_time_days, notes) VALUES (?, ?, ?, ?, ?)"
    )
        .bind(rfq_id)
        .bind(claims.company_id)
        .bind(price_decimal)
        .bind(dto.lead_time_days)
        .bind(dto.notes)
        .execute(pool)
        .await?;
    let quote_id = result.last_insert_id();

    // --- 新增：发送邮件通知 ---
    // 查询RFQ的创建者信息（邮箱）和RFQ的标题
    let rfq_info: Result<(String, String), _> = sqlx::query_as(
        "SELECT u.email, r.title FROM rfqs r JOIN users u ON r.buyer_company_id = u.company_id WHERE r.id = ?"
    )
        .bind(rfq_id)
        .fetch_one(pool)
        .await;

    if let Ok((buyer_email, rfq_title)) = rfq_info {
        let subject = format!("You've received a new quote for your RFQ: {}", rfq_title);
        let body = format!(
            "Hello,\n\nA new quote has been submitted for your RFQ '{}'.\n\nPlease log in to your SCCP account to review it.\n\nThank you,\nThe SCCP Team",
            rfq_title
        );

        // 调用邮件服务，它会在后台发送
        notification_service::send_email(buyer_email, subject, body).await?;
    } else {
        log::error!("Failed to fetch RFQ info for email notification for RFQ ID: {}", rfq_id);
    }

    Ok(quote_id)

}

pub async fn get_quotes_for_rfq(pool: &MySqlPool, rfq_id: i32, claims: &Claims) -> Result<Vec<Quote>, AppError> {
    // 权限检查：只有创建该RFQ的采购方才能查看报价
    let rfq_owner: (i32,) = sqlx::query_as("SELECT buyer_company_id FROM rfqs WHERE id = ?")
        .bind(rfq_id)
        .fetch_one(pool)
        .await?;

    if rfq_owner.0 != claims.company_id {
        return Err(AppError::BadRequest("You are not authorized to view quotes for this RFQ".to_string()));
    }

    let quotes = sqlx::query_as::<_, Quote>(
        "SELECT q.*, c.name as supplier_company_name FROM quotes q JOIN companies c ON q.supplier_company_id = c.id WHERE q.rfq_id = ? ORDER BY q.price ASC"
    )
        .bind(rfq_id)
        .fetch_all(pool)
        .await?;

    Ok(quotes)
}

pub async fn accept_quote(pool: &MySqlPool, quote_id: i32, claims: &Claims) -> Result<u64, AppError> {
    let mut tx = pool.begin().await?;

    // 1. 获取报价和其关联的RFQ信息，并加锁以防并发问题
    let quote_info = sqlx::query(
        "SELECT q.rfq_id, q.supplier_company_id, q.price, r.buyer_company_id, r.status as rfq_status
         FROM quotes q JOIN rfqs r ON q.rfq_id = r.id WHERE q.id = ? FOR UPDATE"
    )
        .bind(quote_id)
        .fetch_one(&mut *tx)
        .await?;

    let rfq_id: i32 = quote_info.try_get("rfq_id")?;
    let supplier_company_id: i32 = quote_info.try_get("supplier_company_id")?;
    let price: Decimal = quote_info.try_get("price")?;
    let buyer_company_id: i32 = quote_info.try_get("buyer_company_id")?;
    let rfq_status: String = quote_info.try_get("rfq_status")?;

    // 2. 权限检查：必须是RFQ的创建者，且RFQ必须是OPEN状态
    if buyer_company_id != claims.company_id {
        return Err(AppError::BadRequest("Not authorized to accept this quote".to_string()));
    }
    if rfq_status != "OPEN" {
        return Err(AppError::BadRequest("This RFQ is no longer open".to_string()));
    }

    // 3. 更新RFQ状态为AWARDED
    sqlx::query("UPDATE rfqs SET status = 'AWARDED' WHERE id = ?")
        .bind(rfq_id)
        .execute(&mut *tx)
        .await?;

    // 4. 更新被接受的报价状态为ACCEPTED
    sqlx::query("UPDATE quotes SET status = 'ACCEPTED' WHERE id = ?")
        .bind(quote_id)
        .execute(&mut *tx)
        .await?;

    // 5. 创建采购订单 (PO)
    let po_result = sqlx::query(
        "INSERT INTO purchase_orders (quote_id, rfq_id, buyer_company_id, supplier_company_id, total_amount) VALUES (?, ?, ?, ?, ?)"
    )
        .bind(quote_id)
        .bind(rfq_id)
        .bind(buyer_company_id)
        .bind(supplier_company_id)
        .bind(price)
        .execute(&mut *tx)
        .await?;

    // 提交事务
    tx.commit().await?;

    Ok(po_result.last_insert_id())
}