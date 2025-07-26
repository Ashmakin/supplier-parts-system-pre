use crate::errors::AppError;
use crate::models::user::{LoginDto, LoginResponse, RegisterDto};
use crate::services::auth_service;
use actix_web::{web, HttpResponse, Responder};
use sqlx::MySqlPool;

/// 处理注册请求的API端点
pub async fn register(
    pool: web::Data<MySqlPool>,
    dto: web::Json<RegisterDto>,
) -> Result<impl Responder, AppError> {
    auth_service::register_user_and_company(pool.get_ref(), dto.into_inner()).await?;
    Ok(HttpResponse::Created().json("Registration successful"))
}

/// 处理登录请求的API端点
pub async fn login(
    pool: web::Data<MySqlPool>,
    dto: web::Json<LoginDto>,
) -> Result<impl Responder, AppError> {
    let token = auth_service::login_user(pool.get_ref(), dto.into_inner()).await?;
    Ok(HttpResponse::Ok().json(LoginResponse::new(token)))
}