// src/handlers/company_handler.rs

use crate::{
    errors::AppError,
    models::{company::UpdateCompanyDto, user::Claims},
    services::company_service,
};
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use sqlx::MySqlPool;

pub async fn get_profile(
    pool: web::Data<MySqlPool>,
    company_id: web::Path<i32>,
) -> Result<impl Responder, AppError> {
    let profile = company_service::get_company_by_id(pool.get_ref(), company_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(profile))
}

pub async fn update_profile(
    pool: web::Data<MySqlPool>,
    company_id: web::Path<i32>,
    dto: web::Json<UpdateCompanyDto>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let claims = req.extensions().get::<Claims>().cloned().ok_or(AppError::AuthError)?;
    let id = company_id.into_inner();
    company_service::update_company_profile(pool.get_ref(), id, dto.into_inner(), &claims).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "message": "Profile updated successfully" })))
}