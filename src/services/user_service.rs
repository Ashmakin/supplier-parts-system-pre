// src/services/user_service.rs

use crate::{
    errors::AppError,
    models::user::{ChangePasswordDto, Claims, User, UserProfileResponse},
    utils::auth_utils,
};
use sqlx::MySqlPool;

pub async fn get_my_profile(pool: &MySqlPool, claims: &Claims) -> Result<UserProfileResponse, AppError> {
    // 使用JOIN查询来同时获取公司名称
    let profile = sqlx::query_as(
        "SELECT u.id, u.full_name, u.email, u.company_id, c.name as company_name
         FROM users u
         JOIN companies c ON u.company_id = c.id
         WHERE u.id = ?"
    )
        .bind(claims.sub) // claims.sub 就是 user_id
        .fetch_one(pool)
        .await?;

    Ok(profile)
}

pub async fn change_password(
    pool: &MySqlPool,
    claims: &Claims,
    dto: ChangePasswordDto,
) -> Result<(), AppError> {
    // 1. 获取用户当前的密码哈希
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(claims.sub)
        .fetch_one(pool)
        .await?;

    // 2. 验证当前密码是否正确
    let valid_password = auth_utils::verify_password(&dto.current_password, &user.password_hash)
        .map_err(|_| AppError::InternalServerError("Password verification error".to_string()))?;

    if !valid_password {
        return Err(AppError::BadRequest("Incorrect current password.".to_string()));
    }

    // 3. 验证新密码强度（简单示例：不少于6位）
    if dto.new_password.len() < 6 {
        return Err(AppError::BadRequest("New password must be at least 6 characters long.".to_string()));
    }

    // 4. 哈希新密码并更新数据库
    let new_password_hash = auth_utils::hash_password(&dto.new_password)
        .map_err(|_| AppError::InternalServerError("Failed to hash new password".to_string()))?;

    sqlx::query("UPDATE users SET password_hash = ? WHERE id = ?")
        .bind(new_password_hash)
        .bind(claims.sub)
        .execute(pool)
        .await?;

    Ok(())
}