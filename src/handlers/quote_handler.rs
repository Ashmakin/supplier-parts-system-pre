// src/handlers/quote_handler.rs
use crate::{
    errors::AppError,
    models::{quote::CreateQuoteDto, user::Claims},
    services::quote_service,
};
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use sqlx::MySqlPool;

pub async fn post_quote(
    pool: web::Data<MySqlPool>,
    rfq_id: web::Path<i32>,
    dto: web::Json<CreateQuoteDto>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let claims = req.extensions().get::<Claims>().cloned().ok_or(AppError::AuthError)?;
    let quote_id = quote_service::create_quote(pool.get_ref(), rfq_id.into_inner(), dto.into_inner(), &claims).await?;
    Ok(HttpResponse::Created().json(serde_json::json!({ "quote_id": quote_id })))
}

pub async fn get_quotes(
    pool: web::Data<MySqlPool>,
    rfq_id: web::Path<i32>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let claims = req.extensions().get::<Claims>().cloned().ok_or(AppError::AuthError)?;
    let quotes = quote_service::get_quotes_for_rfq(pool.get_ref(), rfq_id.into_inner(), &claims).await?;
    Ok(HttpResponse::Ok().json(quotes))
}

pub async fn post_accept_quote(
    pool: web::Data<MySqlPool>,
    quote_id: web::Path<i32>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let claims = req.extensions().get::<Claims>().cloned().ok_or(AppError::AuthError)?;
    let po_id = quote_service::accept_quote(pool.get_ref(), quote_id.into_inner(), &claims).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "purchase_order_id": po_id })))
}