use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use crate::models::user::Claims;

/// 哈希密码
pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

/// 验证密码
pub fn verify_password(password: &str, hash_str: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash_str)
}

/// 创建JWT
pub fn create_jwt(
    user_id: i32,
    company_id: i32,
    company_type: &str,
    is_admin: bool, // <-- 新增参数
) -> Result<String, jsonwebtoken::errors::Error> {
    // 从环境变量获取JWT密钥
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    // 设置过期时间为7天后
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(7))
        .expect("Failed to create valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id,
        company_id,
        company_type: company_type.to_string(),
        is_admin,
        exp: expiration as usize,
    };

    // 编码JWT
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}
pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let validation = Validation::default();

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )
        .map(|data| data.claims)
}


// --- 新增：测试模块 ---
// `#[cfg(test)]` 宏告诉Rust编译器，只有在运行 `cargo test` 命令时才编译和运行这段代码。
#[cfg(test)]
mod tests {
    use super::*; // 导入父模块（auth_utils）的所有内容

    #[test]
    fn test_password_hashing_and_verification() {
        // 1. 定义一个简单的密码
        let password = "mySecurePassword123";

        // 2. 哈希密码
        // .unwrap() 在测试中是可接受的，因为如果这里失败了，我们希望测试立即恐慌(panic)
        let hashed_password = hash_password(password).unwrap();

        // 3. 验证
        // 断言(assert!)：验证正确的密码应该返回 true
        let is_valid = verify_password(password, &hashed_password).unwrap();
        assert!(is_valid, "Password should be valid");

        // 断言(assert!)：验证错误的密码应该返回 false
        let is_invalid = verify_password("wrongPassword", &hashed_password).unwrap();
        assert!(!is_invalid, "Wrong password should be invalid");
    }
}