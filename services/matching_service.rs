// src/services/matching_service.rs

use crate::{
    errors::AppError,
    models::rfq::Rfq,
    services::{
        chat_server::ChatServer,
        notification_service::NotificationBuilder,
    },
};
use actix::Addr;
use sqlx::{MySqlPool, Row}; // Make sure Row is imported

// extract_keywords function remains the same
fn extract_keywords(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty() && s.len() > 2)
        .map(|s| s.to_string())
        .collect()
}

// Replace the old function with this new version
pub async fn find_and_notify_suppliers(
    pool: &MySqlPool,
    chat_server: &Addr<ChatServer>,
    rfq: &Rfq,
) -> Result<(), AppError> {
    log::info!("Starting supplier matching process for RFQ #{}", rfq.id);

    // 1. Extract keywords
    let search_text = format!("{} {}", rfq.title, rfq.description.as_deref().unwrap_or(""));
    let keywords = extract_keywords(&search_text);

    if keywords.is_empty() {
        log::info!("No keywords extracted from RFQ #{}, skipping matching.", rfq.id);
        return Ok(());
    }
    log::info!("Extracted keywords for RFQ #{}: {:?}", rfq.id, keywords);


    // 2. 【THE FIX】Find matching capabilities using LIKE instead of IN
    // We build a query like: SELECT id FROM capabilities WHERE name LIKE ? OR name LIKE ? ...
    let like_clauses: Vec<String> = keywords.iter().map(|_| "name LIKE ?".to_string()).collect();
    let where_clause = like_clauses.join(" OR ");
    let sql = format!("SELECT id FROM capabilities WHERE {}", where_clause);

    let mut query_builder = sqlx::query(&sql);
    for keyword in &keywords {
        // Add wildcards for partial matching
        query_builder = query_builder.bind(format!("%{}%", keyword));
    }

    let matched_cap_ids: Vec<i32> = query_builder
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|row| row.get("id"))
        .collect();

    if matched_cap_ids.is_empty() {
        log::info!("No capabilities matched for RFQ #{}, skipping notification.", rfq.id);
        return Ok(());
    }
    log::info!("Found matching capability IDs for RFQ #{}: {:?}", rfq.id, matched_cap_ids);

    // 3. Find suppliers with these capabilities (this part remains the same)
    let query = format!(
        "SELECT company_id, COUNT(capability_id) as match_count
         FROM company_capabilities
         WHERE capability_id IN ({})
         GROUP BY company_id
         ORDER BY match_count DESC
         LIMIT 5",
        matched_cap_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",")
    );

    let mut query_builder = sqlx::query(&query);
    for id in &matched_cap_ids {
        query_builder = query_builder.bind(id);
    }

    let matched_suppliers: Vec<(i32,)> = query_builder
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|row| (row.get("company_id"),))
        .collect();

    // 4. Send notifications (this part remains the same)
    if matched_suppliers.is_empty() {
        log::info!("No suppliers found with matched capabilities for RFQ #{}", rfq.id);
        return Ok(());
    }

    // 4. 为匹配到的供应商创建并发送通知
    log::info!("Found {} matched suppliers for RFQ #{}. Sending notifications...", matched_suppliers.len(), rfq.id);
    for (company_id,) in matched_suppliers {
        // 假设一个公司只有一个用户接收通知，实际应用可能更复杂
        let user_result: Result<(i32,), _> = sqlx::query_as("SELECT id FROM users WHERE company_id = ? LIMIT 1")
            .bind(company_id)
            .fetch_one(pool)
            .await;

        if let Ok((user_id,)) = user_result {
            // 不通知RFQ的发布者自己
            if user_id != rfq.buyer_company_id {
                let message = format!("New high-match opportunity: '{}'", &rfq.title);
                let link = format!("/rfqs/{}", rfq.id);

                // 使用我们已有的通知服务
                NotificationBuilder::new(user_id, message)
                    .with_link(link)
                    .send(pool, chat_server)
                    .await?;
            }
        }
    }

    Ok(())
}