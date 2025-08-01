// src/services/quote_service.rs
use crate::{
    errors::AppError,
    models::{quote::{CreateQuoteDto, Quote}, user::Claims},
};
use sqlx::{types::Decimal, MySqlPool, Row};
use std::str::FromStr;
use actix::Addr;
use crate::models::order::PurchaseOrder;
use crate::services::chat_server::ChatServer;
use crate::services::notification_service;
use crate::services::notification_service::NotificationBuilder;

pub async fn create_quote(
    pool: &MySqlPool,
    chat_server: &Addr<ChatServer>,
    rfq_id: i32,
    dto: CreateQuoteDto,
    claims: &Claims,
) -> Result<u64, AppError> {
    if claims.company_type != "SUPPLIER" {
        return Err(AppError::BadRequest("Only suppliers can create quotes".to_string()));
    }

    let _rfq: (i32,) = sqlx::query_as("SELECT id FROM rfqs WHERE id = ? AND status = 'OPEN'")
        .bind(rfq_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::BadRequest("RFQ not found or is not open for quotes".to_string()))?;

    let price_decimal = Decimal::from_str(&dto.price.to_string())
        .map_err(|_| AppError::BadRequest("Invalid price format".to_string()))?;

    let result = sqlx::query(
        "INSERT INTO quotes (rfq_id, supplier_company_id, price, lead_time_days, notes) VALUES (?, ?, ?, ?, ?)",
    )
        .bind(rfq_id)
        .bind(claims.company_id)
        .bind(price_decimal)
        .bind(dto.lead_time_days)
        .bind(dto.notes)
        .execute(pool)
        .await?;

    let quote_id = result.last_insert_id();

    // --- 【关键修改】同时触发两种通知 ---

    // 1. 查询需要通知的用户ID、邮箱和RFQ标题
    // 我们假设一个公司只有一个用户，实际应用中这里可能更复杂
    // --- 【THE FIX】Handle notification results with `if let` ---

    // 1. Fetch the necessary info for notifications
    let rfq_owner_info: Result<(i32, String, String), _> = sqlx::query_as(
        "SELECT u.id, u.email, r.title FROM rfqs r JOIN users u ON r.buyer_company_id = u.company_id WHERE r.id = ?"
    )
        .bind(rfq_id)
        .fetch_one(pool)
        .await;

    if let Ok((buyer_user_id, buyer_email, rfq_title)) = rfq_owner_info {
        // 2. Try to send the in-app notification
        let in_app_result = NotificationBuilder::new(
            buyer_user_id,
            format!("You received a new quote for '{}'", &rfq_title)
        )
            .with_link(format!("/rfqs/{}", rfq_id))
            .send(pool, chat_server)
            .await;

        if let Err(e) = in_app_result {
            log::error!("Failed to send in-app notification: {:?}", e);
        }

        // 3. Try to send the email notification
        let subject = format!("New Quote Received: {}", &rfq_title);
        let body = format!(
            "Hello,\n\nA new quote has been submitted for your RFQ '{}'.\n\nPlease log in to your SCCP account to review it.",
            &rfq_title
        );

        let email_result = notification_service::send_email(buyer_email, subject, body).await;
        if let Err(e) = email_result {
            log::error!("Failed to send email notification: {:?}", e);
        }
    } else {
        log::error!("Failed to fetch RFQ owner info for notifications for RFQ ID: {}", rfq_id);
    }

    // The main function still succeeds and returns the quote_id
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

pub async fn accept_quote(
    pool: &MySqlPool,
    chat_server: &Addr<ChatServer>,
    quote_id: i32,
    claims: &Claims,
) -> Result<u64, AppError> {
    let mut tx = pool.begin().await?;

    let quote_info = sqlx::query(
        "SELECT q.rfq_id, q.supplier_company_id, q.price, r.buyer_company_id, r.status as rfq_status, r.title as rfq_title
         FROM quotes q JOIN rfqs r ON q.rfq_id = r.id WHERE q.id = ? FOR UPDATE",
    )
        .bind(quote_id)
        .fetch_one(&mut *tx)
        .await?;

    let rfq_id: i32 = quote_info.try_get("rfq_id")?;
    let supplier_company_id: i32 = quote_info.try_get("supplier_company_id")?;
    let price: Decimal = quote_info.try_get("price")?;
    let buyer_company_id: i32 = quote_info.try_get("buyer_company_id")?;
    let rfq_status: String = quote_info.try_get("rfq_status")?;
    let rfq_title: String = quote_info.try_get("rfq_title")?;

    if buyer_company_id != claims.company_id || rfq_status != "OPEN" {
        return Err(AppError::BadRequest(
            "Not authorized to accept this quote or RFQ is not open.".to_string(),
        ));
    }

    sqlx::query("UPDATE rfqs SET status = 'AWARDED' WHERE id = ?").bind(rfq_id).execute(&mut *tx).await?;
    sqlx::query("UPDATE quotes SET status = 'ACCEPTED' WHERE id = ?").bind(quote_id).execute(&mut *tx).await?;

    let po_result = sqlx::query(
        "INSERT INTO purchase_orders (quote_id, rfq_id, buyer_company_id, supplier_company_id, total_amount) VALUES (?, ?, ?, ?, ?)",
    )
        .bind(quote_id).bind(rfq_id).bind(buyer_company_id).bind(supplier_company_id).bind(price)
        .execute(&mut *tx)
        .await?;

    let po_id = po_result.last_insert_id();
    tx.commit().await?;

    // --- 【THE FIX】Handle notification Results with `if let` ---
    let supplier_user: Result<(i32, String), _> =
        sqlx::query_as("SELECT id, email FROM users WHERE company_id = ? LIMIT 1")
            .bind(supplier_company_id)
            .fetch_one(pool)
            .await;

    if let Ok((supplier_user_id, supplier_email)) = supplier_user {
        // Try to send in-app notification
        let in_app_result = NotificationBuilder::new(
            supplier_user_id,
            format!("Congratulations! Your quote for '{}' has been accepted.", &rfq_title),
        )
            .with_link(format!("/orders"))
            .send(pool, chat_server)
            .await;

        if let Err(e) = in_app_result {
            log::error!("Failed to send in-app notification to supplier: {:?}", e);
        }

        // Try to send email notification
        let subject = format!("Your Quote for '{}' has been Accepted!", &rfq_title);
        let body = "Congratulations! Your quote has been accepted and a new Purchase Order has been generated. Please log in to view your orders.".to_string();

        let email_result = notification_service::send_email(supplier_email, subject, body).await;
        if let Err(e) = email_result {
            log::error!("Failed to send email notification to supplier: {:?}", e);
        }
    }

    Ok(po_id)
}
// src/services/order_service.rs
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