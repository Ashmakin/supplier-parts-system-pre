// src/services/company_service.rs

use crate::{
    errors::AppError,
    models::{company::{CompanyProfile, UpdateCompanyDto}, user::Claims},
};
use sqlx::MySqlPool;

pub async fn get_company_by_id(pool: &MySqlPool, company_id: i32) -> Result<CompanyProfile, AppError> {
    let profile = sqlx::query_as("SELECT id, name, company_type, city, description, is_verified, created_at FROM companies WHERE id = ?")
        .bind(company_id)
        .fetch_one(pool)
        .await?;
    Ok(profile)
}

pub async fn update_company_profile(
    pool: &MySqlPool,
    company_id: i32,
    dto: UpdateCompanyDto,
    claims: &Claims,
) -> Result<u64, AppError> {
    // 权限检查：确保操作者属于他们正试图修改的公司
    if claims.company_id != company_id {
        return Err(AppError::BadRequest("You are not authorized to edit this company profile.".to_string()));
    }

    let result = sqlx::query("UPDATE companies SET description = ? WHERE id = ?")
        .bind(dto.description)
        .bind(company_id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}