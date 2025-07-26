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