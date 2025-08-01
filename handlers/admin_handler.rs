// src/handlers/admin_handler.rs
use crate::{errors::AppError, models::user::Claims, services::admin_service};
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use sqlx::MySqlPool;
use serde::Deserialize; // <-- 导入
// ...
#[derive(Deserialize)]
pub struct UserStatusUpdate {
    is_active: bool,
}

// 权限检查辅助函数
fn check_admin(req: &HttpRequest) -> Result<Claims, AppError> {
    let claims = req.extensions().get::<Claims>().cloned().ok_or(AppError::AuthError)?;
    if !claims.is_admin {
        return Err(AppError::BadRequest("Administrator privileges required.".to_string()));
    }
    Ok(claims)
}

pub async fn get_all_companies(pool: web::Data<MySqlPool>, req: HttpRequest) -> Result<impl Responder, AppError> {
    check_admin(&req)?;
    let companies = admin_service::list_all_companies(pool.get_ref()).await?;
    Ok(HttpResponse::Ok().json(companies))
}

pub async fn put_verify_company(pool: web::Data<MySqlPool>, company_id: web::Path<i32>, req: HttpRequest) -> Result<impl Responder, AppError> {
    check_admin(&req)?;
    admin_service::verify_company(pool.get_ref(), company_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "message": "Company verified successfully" })))
}

pub async fn get_all_users(pool: web::Data<MySqlPool>, req: HttpRequest) -> Result<impl Responder, AppError> {
    check_admin(&req)?;
    let users = admin_service::list_all_users(pool.get_ref()).await?;
    Ok(HttpResponse::Ok().json(users))
}

pub async fn put_update_user_status(
    pool: web::Data<MySqlPool>,
    user_id: web::Path<i32>,
    dto: web::Json<UserStatusUpdate>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    check_admin(&req)?;
    admin_service::update_user_status(pool.get_ref(), user_id.into_inner(), dto.is_active).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "message": "User status updated successfully" })))
}