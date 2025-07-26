// src/handlers/user_handler.rs

use crate::{
    errors::AppError,
    models::user::{ChangePasswordDto, Claims},
    services::user_service,
};
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use sqlx::MySqlPool;

pub async fn get_me(
    pool: web::Data<MySqlPool>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let claims = req.extensions().get::<Claims>().cloned().ok_or(AppError::AuthError)?;
    let profile = user_service::get_my_profile(pool.get_ref(), &claims).await?;
    Ok(HttpResponse::Ok().json(profile))
}

pub async fn update_password(
    pool: web::Data<MySqlPool>,
    dto: web::Json<ChangePasswordDto>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let claims = req.extensions().get::<Claims>().cloned().ok_or(AppError::AuthError)?;
    user_service::change_password(pool.get_ref(), &claims, dto.into_inner()).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "message": "Password updated successfully" })))
}