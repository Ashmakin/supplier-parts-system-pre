use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// --- 数据库模型 ---

#[derive(Debug, FromRow)]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub company_type: String,
}

#[derive(Debug, FromRow)]
pub struct User {
    pub id: i32,
    pub company_id: i32,
    pub email: String,
    pub password_hash: String,
    pub full_name: Option<String>,
    pub is_admin: bool,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct ChangePasswordDto {
    pub current_password: String,
    pub new_password: String,
}


// --- API 请求体 (DTOs) ---

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterDto {
    pub company_name: String,
    pub company_type: String, // "BUYER" or "SUPPLIER"
    pub city: String, // <-- 新增此字段
    pub email: String,
    pub password: String,
    pub full_name: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginDto {
    pub email: String,
    pub password: String,
}

// --- API 响应体 ---

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub token_type: String,
}

impl LoginResponse {
    pub fn new(token: String) -> Self {
        Self {
            token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[derive(Debug, Serialize, FromRow)]
pub struct UserProfileResponse {
    pub id: i32,
    pub full_name: Option<String>,
    pub email: String,
    pub company_id: i32,
    #[sqlx(default)]
    pub company_name: String,
    pub is_active: bool, // <-- 新增
}


// --- JWT Claims ---
// 这是嵌入在JWT中的数据

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Claims {
    pub sub: i32, // User ID
    pub company_id: i32,
    pub company_type: String,
    pub is_admin: bool, // <-- 新增
    pub exp: usize, // Expiration timestamp
}