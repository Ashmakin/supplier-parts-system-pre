// src/handlers/capability_handler.rs

use crate::{
    errors::AppError,
    models::{capability::AddCompanyCapabilityDto, user::Claims},
    services::capability_service,
};
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder, delete};
use sqlx::MySqlPool;

pub async fn get_capabilities(pool: web::Data<MySqlPool>) -> Result<impl Responder, AppError> {
    let capabilities = capability_service::get_all_capabilities(pool.get_ref()).await?;
    Ok(HttpResponse::Ok().json(capabilities))
}

pub async fn get_company_caps(
    pool: web::Data<MySqlPool>,
    company_id: web::Path<i32>,
) -> Result<impl Responder, AppError> {
    let capabilities = capability_service::get_company_capabilities(pool.get_ref(), company_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(capabilities))
}

pub async fn post_company_cap(
    pool: web::Data<MySqlPool>,
    dto: web::Json<AddCompanyCapabilityDto>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let claims = req.extensions().get::<Claims>().cloned().ok_or(AppError::AuthError)?;
    capability_service::add_capability_to_company(pool.get_ref(), &claims, dto.capability_id).await?;
    Ok(HttpResponse::Created().json("Capability added"))
}

#[delete("/my-company/{capability_id}")]
pub async fn delete_company_cap(
    pool: web::Data<MySqlPool>,
    capability_id: web::Path<i32>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let claims = req.extensions().get::<Claims>().cloned().ok_or(AppError::AuthError)?;
    capability_service::remove_capability_from_company(pool.get_ref(), &claims, capability_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json("Capability removed"))
}