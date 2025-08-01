use crate::errors::AppError;
use crate::models::user::{LoginDto, RegisterDto, User};
use crate::utils::auth_utils;
use sqlx::{FromRow, MySqlPool, Row};

/// 处理新用户和公司的注册逻辑
pub async fn register_user_and_company(
    pool: &MySqlPool,
    dto: RegisterDto,
) -> Result<(), AppError> {
    // 检查邮箱是否已被注册
    let existing_user = sqlx::query("SELECT id FROM users WHERE email = ?")
        .bind(&dto.email)
        .fetch_optional(pool)
        .await?;
    if existing_user.is_some() {
        return Err(AppError::BadRequest("Email already exists".to_string()));
    }

    // 使用数据库事务，确保所有操作要么全部成功，要么全部失败
    let mut tx = pool.begin().await?;
    // --- 诊断步骤 ---
    // 我们只执行第一个INSERT，看看错误是否依然发生
    log::info!("Attempting to insert into companies table...");
    // 1. 创建公司
    let company_result = sqlx::query("INSERT INTO companies (name, company_type,city) VALUES (?, ?,?)")
        .bind(&dto.company_name)
        .bind(&dto.company_type)
        .bind(&dto.city) // <-- 绑定新的 city 数据
        .execute(&mut *tx)
        .await?;
    log::info!("Successfully inserted into companies table. (Transaction not committed)");
    let company_id = company_result.last_insert_id() as i32;

    // 2. 哈希密码
    let password_hash = auth_utils::hash_password(&dto.password)
        .map_err(|_| AppError::InternalServerError("Failed to hash password".to_string()))?;

    // 3. 创建用户
    sqlx::query(
        "INSERT INTO users (company_id, email, password_hash, full_name) VALUES (?, ?, ?, ?)",
    )
        .bind(company_id)
        .bind(&dto.email)
        .bind(password_hash)
        .bind(&dto.full_name)
        .execute(&mut *tx)
        .await?;

    // 提交事务
    tx.commit().await?;

    Ok(())
}

/// 处理用户登录逻辑
pub async fn login_user(pool: &MySqlPool, dto: LoginDto) -> Result<String, AppError> {
    // 修改SQL查询以同时获取 is_admin 字段
    let row = sqlx::query(
        "SELECT u.*, c.company_type FROM users u JOIN companies c ON u.company_id = c.id WHERE u.email = ?"
    )
        .bind(&dto.email)
        .fetch_optional(pool)
        .await?
        .ok_or(AppError::AuthError)?;

    // 从查询结果中提取密码哈希和公司类型
    let password_hash: String = row.try_get("password_hash")?;
    let company_type: String = row.try_get("company_type")?;

    // 2. 验证密码
    let valid_password = auth_utils::verify_password(&dto.password, &password_hash)
        .map_err(|_| AppError::InternalServerError("Password verification error".to_string()))?;

    if !valid_password {
        return Err(AppError::AuthError); // 如果密码不匹配，返回认证失败
    }

    // 3. 创建JWT
    let user: User = User::from_row(&row)?;
    // --- 【关键新增】检查用户是否被禁用 ---
    if !user.is_active {
        return Err(AppError::BadRequest("This account has been disabled.".to_string()));
    }


    let company_type: String = row.try_get("company_type")?;

    let token = auth_utils::create_jwt(user.id, user.company_id, &company_type, user.is_admin) // <-- 传入 is_admin
        .map_err(|_| AppError::InternalServerError("Failed to create token".to_string()))?;

    Ok(token)
}
