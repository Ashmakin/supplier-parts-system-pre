// src/handlers/payment_handler.rs

use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use sqlx::MySqlPool;
use crate::{
    errors::AppError,
    models::{user::Claims, payment::CheckoutSessionResponse},
    services::payment_service,
};
use std::env;
use stripe::{EventObject, EventType, Webhook};

// create_session 函数保持不变
pub async fn create_session(
    pool: web::Data<MySqlPool>,
    order_id: web::Path<i32>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let claims = req.extensions().get::<Claims>().cloned().ok_or(AppError::AuthError)?;
    let session_id = payment_service::create_checkout_session(pool.get_ref(), order_id.into_inner(), &claims).await?;
    Ok(HttpResponse::Ok().json(CheckoutSessionResponse { session_id }))
}

// 【关键修复】使用 async-stripe 的API处理Webhook
pub async fn handle_webhook(
    pool: web::Data<MySqlPool>,
    payload: String,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let signature = req.headers().get("Stripe-Signature").and_then(|h| h.to_str().ok()).unwrap_or_default();
    let webhook_secret = env::var("STRIPE_WEBHOOK_SECRET").expect("STRIPE_WEBHOOK_SECRET must be set");

    let event = Webhook::construct_event(&payload, signature, &webhook_secret)
        .map_err(|e| AppError::BadRequest(format!("Invalid Stripe signature: {}", e)))?;

    // 检查事件类型
    if event.type_ == EventType::CheckoutSessionCompleted {
        // 使用 `if let` 模式匹配来安全地提取会话对象
        if let EventObject::CheckoutSession(session) = event.data.object {
            log::info!("Checkout session {} completed!", session.id);

            let session_id = session.id.to_string();

            // 更新数据库
            sqlx::query("UPDATE purchase_orders SET payment_status = 'PAID' WHERE stripe_session_id = ?")
                .bind(session_id)
                .execute(pool.get_ref())
                .await?;
        }
    }

    Ok(HttpResponse::Ok())
}