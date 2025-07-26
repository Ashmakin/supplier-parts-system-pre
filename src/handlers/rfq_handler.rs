// src/handlers/rfq_handler.rs
use crate::{
    errors::AppError,
    models::user::Claims,
    services::rfq_service,
};
use serde::Deserialize;
use actix_multipart::Multipart;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use sqlx::MySqlPool;
use crate::models::rfq::CreateRfqDto;

#[derive(Debug, Deserialize)]
pub struct RfqFilterParams {
    search: Option<String>,
    city: Option<String>,
}

// Replace the multipart version of post_rfq with this JSON version
pub async fn post_rfq(
    pool: web::Data<MySqlPool>,
    dto: web::Json<CreateRfqDto>, // <-- Back to using web::Json
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let claims = req.extensions().get::<Claims>().cloned().ok_or(AppError::AuthError)?;
    // We will create a new, simpler service function for this
    let rfq_id = rfq_service::create_rfq(pool.get_ref(), dto.into_inner(), &claims).await?;
    Ok(HttpResponse::Created().json(serde_json::json!({ "rfq_id": rfq_id })))
}

pub async fn get_rfqs(
    pool: web::Data<MySqlPool>,
    params: web::Query<RfqFilterParams>, // <-- 将参数绑定到我们的Struct
) -> Result<impl Responder, AppError> {
    let rfqs = rfq_service::get_all_open_rfqs(
        pool.get_ref(),
        params.search.clone(),
        params.city.clone(),
    )
        .await?;
    Ok(HttpResponse::Ok().json(rfqs))
}

pub async fn get_rfq_detail(
    pool: web::Data<MySqlPool>,
    rfq_id: web::Path<i32>,
) -> Result<impl Responder, AppError> {
    let rfq = rfq_service::get_rfq_by_id(pool.get_ref(), rfq_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(rfq))
}

pub async fn get_attachments(
    pool: web::Data<MySqlPool>,
    rfq_id: web::Path<i32>,
) -> Result<impl Responder, AppError> {
    let attachments = rfq_service::get_attachments_for_rfq(pool.get_ref(), rfq_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(attachments))
}

pub async fn get_messages_for_rfq(
    pool: web::Data<MySqlPool>,
    rfq_id: web::Path<i32>,
) -> Result<impl Responder, AppError> {
    let messages = rfq_service::get_messages_for_rfq(pool.get_ref(), rfq_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(messages))
}