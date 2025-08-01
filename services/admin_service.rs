// src/services/admin_service.rs
use crate::{errors::AppError, models::company::CompanyProfile};
use sqlx::MySqlPool;
use crate::models::user::UserProfileResponse;

pub async fn list_all_companies(pool: &MySqlPool) -> Result<Vec<CompanyProfile>, AppError> {
    let companies = sqlx::query_as("SELECT id, name, company_type, city, description, created_at, is_verified FROM companies ORDER BY created_at DESC")
        .fetch_all(pool)
        .await?;
    Ok(companies)
}

pub async fn verify_company(pool: &MySqlPool, company_id: i32) -> Result<u64, AppError> {
    let result = sqlx::query("UPDATE companies SET is_verified = TRUE WHERE id = ?")
        .bind(company_id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}

pub async fn list_all_users(pool: &MySqlPool) -> Result<Vec<UserProfileResponse>, AppError> {
    let users = sqlx::query_as(
        "SELECT u.id, u.full_name, u.email, u.company_id, u.is_active, c.name as company_name
         FROM users u JOIN companies c ON u.company_id = c.id ORDER BY u.created_at DESC"
    )
        .fetch_all(pool)
        .await?;
    Ok(users)
}

pub async fn update_user_status(pool: &MySqlPool, user_id: i32, is_active: bool) -> Result<u64, AppError> {
    let result = sqlx::query("UPDATE users SET is_active = ? WHERE id = ?")
        .bind(is_active)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}