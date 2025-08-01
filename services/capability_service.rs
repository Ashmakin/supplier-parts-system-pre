// src/services/capability_service.rs

use crate::{
    errors::AppError,
    models::{capability::Capability, user::Claims},
};
use sqlx::MySqlPool;

// 获取所有可用的能力标签
pub async fn get_all_capabilities(pool: &MySqlPool) -> Result<Vec<Capability>, AppError> {
    let capabilities = sqlx::query_as("SELECT * FROM capabilities ORDER BY category, name")
        .fetch_all(pool)
        .await?;
    Ok(capabilities)
}

// 获取某个特定公司已拥有的能力标签
pub async fn get_company_capabilities(pool: &MySqlPool, company_id: i32) -> Result<Vec<Capability>, AppError> {
    let capabilities = sqlx::query_as(
        "SELECT c.* FROM capabilities c JOIN company_capabilities cc ON c.id = cc.capability_id WHERE cc.company_id = ?"
    )
        .bind(company_id)
        .fetch_all(pool)
        .await?;
    Ok(capabilities)
}

// 为公司添加一个能力标签
pub async fn add_capability_to_company(
    pool: &MySqlPool,
    claims: &Claims,
    capability_id: i32,
) -> Result<(), AppError> {
    // 权限检查：只有供应商才能为自己添加能力
    if claims.company_type != "SUPPLIER" {
        return Err(AppError::BadRequest("Only suppliers can add capabilities.".to_string()));
    }

    sqlx::query("INSERT INTO company_capabilities (company_id, capability_id) VALUES (?, ?)")
        .bind(claims.company_id)
        .bind(capability_id)
        .execute(pool)
        .await?;

    Ok(())
}

// 从公司移除一个能力标签
pub async fn remove_capability_from_company(
    pool: &MySqlPool,
    claims: &Claims,
    capability_id: i32,
) -> Result<(), AppError> {
    if claims.company_type != "SUPPLIER" {
        return Err(AppError::BadRequest("Only suppliers can remove capabilities.".to_string()));
    }

    sqlx::query("DELETE FROM company_capabilities WHERE company_id = ? AND capability_id = ?")
        .bind(claims.company_id)
        .bind(capability_id)
        .execute(pool)
        .await?;

    Ok(())
}