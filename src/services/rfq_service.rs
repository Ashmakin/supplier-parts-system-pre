// src/services/rfq_service.rs
use crate::{
    errors::AppError,
    models::{rfq::{CreateRfqDto, Rfq}, user::Claims},
};
use sqlx::{MySql, MySqlPool, QueryBuilder};
use actix_multipart::Field;
use futures_util::stream::StreamExt;
use std::fs;
use std::io::Write;
use actix_web::web;
use uuid::Uuid;
use crate::models::rfq::RfqAttachment;
use crate::models::chat::ChatMessage;

// This is the new, simple function for creating an RFQ with JSON
pub async fn create_rfq(pool: &MySqlPool, dto: CreateRfqDto, claims: &Claims) -> Result<u64, AppError> {
    if claims.company_type != "BUYER" {
        return Err(AppError::BadRequest("Only buyers can create RFQs".to_string()));
    }

    let result = sqlx::query(
        "INSERT INTO rfqs (buyer_company_id, title, description, quantity) VALUES (?, ?, ?, ?)"
    )
        .bind(claims.company_id)
        .bind(dto.title)
        .bind(dto.description)
        .bind(dto.quantity)
        .execute(pool)
        .await?;

    Ok(result.last_insert_id())
}
/////////////////
pub async fn create_rfq_with_attachment(
    pool: &MySqlPool,
    claims: &Claims,
    mut payload: actix_multipart::Multipart,
) -> Result<u64, AppError> {
    if claims.company_type != "BUYER" {
        return Err(AppError::BadRequest("Only buyers can create RFQs".to_string()));
    }

    let mut title = String::new();
    let mut description = String::new();
    let mut quantity = String::new();
    let mut attachment_path: Option<String> = None;
    let mut original_filename: Option<String> = None;

    while let Some(field_result) = payload.next().await {
        let mut field = field_result?;

        // --- 这是关键的修改 ---
        // 我们获取字段名，并立即使用 .to_string() 将其转换为一个拥有所有权的 String
        // 这样，对 `field` 的不可变借用在这行代码结束后就立即释放了。
        let field_name = field
            .content_disposition().expect("REASON")
            .get_name()
            .unwrap_or_default()
            .to_string();

        match field_name.as_str() {
            "title" | "description" | "quantity" => {
                let mut data = Vec::new();
                while let Some(chunk) = field.next().await {
                    data.extend_from_slice(&chunk?);
                }
                let value = String::from_utf8(data).map_err(|_| AppError::BadRequest("Invalid UTF-8 in form fields".to_string()))?;
                match field_name.as_str() {
                    "title" => title = value,
                    "description" => description = value,
                    "quantity" => quantity = value,
                    _ => (),
                }
            }
            "attachment" => {
                // 这里也需要获取 content_disposition，但同样只在需要时获取
                if let Some(filename) = field.content_disposition().expect("REASON").get_filename() {
                    let unique_filename = format!("{}-{}", Uuid::new_v4(), filename);
                    let filepath = format!("./uploads/{}", unique_filename);

                    original_filename = Some(filename.to_string());
                    attachment_path = Some(filepath.clone());

                    let file_create_result = web::block(move || fs::File::create(filepath)).await?;
                    let mut f = file_create_result?;

                    while let Some(chunk) = field.next().await {
                        let data = chunk?;
                        let write_result = web::block(move || f.write_all(&data).map(|_| f)).await?;
                        f = write_result?;
                    }
                }
            }
            _ => (),
        }
    }

    // 开始数据库事务
    let mut tx = pool.begin().await?;

    let quantity_num: i32 = quantity.parse().map_err(|_| AppError::BadRequest("Invalid quantity".to_string()))?;
    let rfq_result = sqlx::query(
        "INSERT INTO rfqs (buyer_company_id, title, description, quantity) VALUES (?, ?, ?, ?)"
    )
        .bind(claims.company_id)
        .bind(title)
        .bind(description)
        .bind(quantity_num)
        .execute(&mut *tx)
        .await?;

    let rfq_id = rfq_result.last_insert_id();

    if let (Some(path), Some(orig_name)) = (attachment_path, original_filename) {
        sqlx::query(
            "INSERT INTO rfq_attachments (rfq_id, original_filename, stored_path) VALUES (?, ?, ?)"
        )
            .bind(rfq_id)
            .bind(orig_name)
            .bind(path)
            .execute(&mut *tx)
            .await?;
    }

    tx.commit().await?;

    Ok(rfq_id)
}
/////////////////
pub async fn get_all_open_rfqs(
    pool: &MySqlPool,
    search: Option<String>,
    city: Option<String>,
) -> Result<Vec<Rfq>, AppError> {
    // 基础查询语句
    let base_query = "SELECT r.*, c.name as buyer_company_name, c.city FROM rfqs r JOIN companies c ON r.buyer_company_id = c.id WHERE r.status = 'OPEN'";

    // 使用QueryBuilder来安全地构建动态查询
    let mut qb: QueryBuilder<MySql> = QueryBuilder::new(base_query);

    // 如果有搜索关键词
    if let Some(term) = search {
        if !term.trim().is_empty() {
            let search_pattern = format!("%{}%", term);
            // AND (r.title LIKE ? OR r.description LIKE ?)
            qb.push(" AND (r.title LIKE ")
                .push_bind(search_pattern.clone())
                .push(" OR r.description LIKE ")
                .push_bind(search_pattern)
                .push(")");
        }
    }

    // 如果有城市筛选
    if let Some(city_name) = city {
        if !city_name.trim().is_empty() {
            // AND c.city = ?
            qb.push(" AND c.city = ")
                .push_bind(city_name);
        }
    }

    // 添加排序
    qb.push(" ORDER BY r.created_at DESC");

    // 执行查询
    let query = qb.build_query_as::<Rfq>();
    let rfqs = query.fetch_all(pool).await?;

    Ok(rfqs)
}
/////////////////
pub async fn get_rfq_by_id(pool: &MySqlPool, rfq_id: i32) -> Result<Rfq, AppError> {
    let rfq = sqlx::query_as::<_, Rfq>(
        "SELECT r.*, c.name as buyer_company_name FROM rfqs r JOIN companies c ON r.buyer_company_id = c.id WHERE r.id = ?"
    )
        .bind(rfq_id)
        .fetch_one(pool)
        .await?;
    Ok(rfq)
}
/////////////////
pub async fn get_attachments_for_rfq(pool: &MySqlPool, rfq_id: i32) -> Result<Vec<RfqAttachment>, AppError> {
    let attachments = sqlx::query_as("SELECT * FROM rfq_attachments WHERE rfq_id = ?")
        .bind(rfq_id)
        .fetch_all(pool)
        .await?;
    Ok(attachments)
}
/////////////////
pub async fn get_messages_for_rfq(pool: &MySqlPool, rfq_id: i32) -> Result<Vec<ChatMessage>, AppError> {
    let messages = sqlx::query_as("SELECT * FROM chat_messages WHERE rfq_id = ? ORDER BY created_at ASC")
        .bind(rfq_id)
        .fetch_all(pool)
        .await?;
    Ok(messages)
}

// This function is now specifically for attachments. We rename it for clarity.
// Note: We won't test this multipart function right now, but we'll keep the logic.
pub async fn upload_attachment_for_rfq(
    pool: &MySqlPool,
    claims: &Claims,
    rfq_id: i32,
    mut payload: actix_multipart::Multipart,
) -> Result<(), AppError> {
    // Security check remains the same
    let rfq_owner: (i32,) = sqlx::query_as("SELECT buyer_company_id FROM rfqs WHERE id = ?")
        .bind(rfq_id)
        .fetch_one(pool)
        .await?;

    if rfq_owner.0 != claims.company_id {
        return Err(AppError::BadRequest("You are not the owner of this RFQ.".to_string()));
    }

    while let Some(field_result) = payload.next().await {
        let mut field = field_result?;

        // --- FIX for Error #1 (Borrow Checker) ---
        // Get the filename as an owned String immediately to release the borrow on `field`.
        // We do this by mapping the inner `&str` to a `String`.
        let filename_opt: Option<String> = field
            .content_disposition()
            .and_then(|cd| cd.get_filename().map(|s| s.to_string()));

        if let Some(filename) = filename_opt {
            let unique_filename = format!("{}-{}", Uuid::new_v4(), &filename);
            let filepath = format!("./uploads/{}", unique_filename);

            // --- FIX for Error #2 (Moved Value) ---
            // Clone filepath. The clone will be moved into the closure,
            // while the original remains available for the database query below.
            let filepath_clone = filepath.clone();

            let mut f = web::block(move || fs::File::create(filepath_clone)).await??;
            while let Some(chunk) = field.next().await {
                let data = chunk?;
                f = web::block(move || f.write_all(&data).map(|_| f)).await??;
            }

            // Save attachment info to DB
            sqlx::query(
                "INSERT INTO rfq_attachments (rfq_id, original_filename, stored_path) VALUES (?, ?, ?)"
            )
                .bind(rfq_id)
                .bind(filename) // Now using the owned String
                .bind(filepath) // Now using the original `filepath` which was not moved
                .execute(pool)
                .await?;
        }
    }

    Ok(())
}