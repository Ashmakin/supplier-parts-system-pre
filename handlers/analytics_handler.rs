// src/handlers/analytics_handler.rs

use crate::{
    errors::AppError,
    models::user::Claims,
    services::analytics_service,
};
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use sqlx::MySqlPool;

pub async fn get_stats(
    pool: web::Data<MySqlPool>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let claims = req.extensions().get::<Claims>().cloned().ok_or(AppError::AuthError)?;
    let stats = analytics_service::get_buyer_dashboard_stats(pool.get_ref(), &claims).await?;
    Ok(HttpResponse::Ok().json(stats))
}

pub async fn get_spending_breakdown(
    pool: web::Data<MySqlPool>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let claims = req.extensions().get::<Claims>().cloned().ok_or(AppError::AuthError)?;
    let breakdown = analytics_service::get_buyer_spending_by_supplier(pool.get_ref(), &claims).await?;
    Ok(HttpResponse::Ok().json(breakdown))
}

/// 处理获取供应方(Supplier)核心统计数据的API请求
pub async fn get_supplier_stats(
    pool: web::Data<MySqlPool>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    // 提取用户信息
    let claims = req.extensions().get::<Claims>().cloned().ok_or(AppError::AuthError)?;

    // 调用service层函数
    let stats = analytics_service::get_supplier_dashboard_stats(pool.get_ref(), &claims).await?;

    // 返回200 OK和JSON数据
    Ok(HttpResponse::Ok().json(stats))
}