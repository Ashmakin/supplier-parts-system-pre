// src/handlers/notification_handler.rs
use crate::{errors::AppError, models::user::Claims, services::notification_service};
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use sqlx::MySqlPool;

pub async fn get_notifications(pool: web::Data<MySqlPool>, req: HttpRequest) -> Result<impl Responder, AppError> {
    let claims = req.extensions().get::<Claims>().cloned().ok_or(AppError::AuthError)?;
    let notifications = notification_service::get_notifications_for_user(pool.get_ref(), &claims).await?;
    Ok(HttpResponse::Ok().json(notifications))
}
// 新增：处理标记已读的请求
pub async fn put_mark_as_read(
    pool: web::Data<MySqlPool>,
    notification_id: web::Path<i32>,
    req: HttpRequest
) -> Result<impl Responder, AppError> {
    let claims = req.extensions().get::<Claims>().cloned().ok_or(AppError::AuthError)?;
    notification_service::mark_notification_as_read(pool.get_ref(), &claims, notification_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "message": "Notification marked as read" })))
}

pub async fn mark_notification_as_read(pool: &MySqlPool, claims: &Claims, notification_id: i32) -> Result<u64, AppError> {
    // 增加一个额外的安全检查，确保用户只能标记自己的通知
    let result = sqlx::query(
        "UPDATE notifications SET is_read = TRUE WHERE id = ? AND recipient_user_id = ?"
    )
        .bind(notification_id)
        .bind(claims.sub) // claims.sub is the user_id
        .execute(pool)
        .await?;

    // --- 新增详细日志 ---
    if result.rows_affected() == 0 {
        // 如果没有行被更新，记录一条警告。这可能是因为通知ID错误，或者该通知不属于当前用户。
        log::warn!(
            "User #{} attempted to mark notification #{} as read, but no rows were affected. (Not found or permission denied)",
            claims.sub,
            notification_id
        );
    } else {
        log::info!("Successfully marked notification #{} as read for user #{}.", notification_id, claims.sub);
    }

    Ok(result.rows_affected())
}

/// --- ADD THIS MISSING FUNCTION ---
/// Handles request to mark all notifications as read
pub async fn put_mark_all_as_read(
    pool: web::Data<MySqlPool>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let claims = req.extensions().get::<Claims>().cloned().ok_or(AppError::AuthError)?;
    notification_service::mark_all_as_read(pool.get_ref(), &claims).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "message": "All notifications marked as read" })))
}
