// tests/auth_test.rs

// 引入我们应用的主crate，并命名为 sccp_backend
use crate::{config, api, models::user::RegisterDto};
use actix_web::{test, web, App};
use serde_json::json;
//use crate::
// `#[actix_web::test]` 宏会自动设置一个tokio运行时
#[actix_web::test]
async fn test_register_user_success() {
    // 1. 设置测试环境
    // 连接到我们的测试数据库
    let pool = config::configure_test_db().await;
    // 初始化一个测试服务器
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(api::config)
    ).await;

    // 2. 准备请求数据
    let register_dto = RegisterDto {
        company_name: "Test Company".to_string(),
        company_type: "BUYER".to_string(),
        city: "".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
        full_name: "Test User".to_string(),
    };

    // 3. 构建并发送API请求
    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(&register_dto)
        .to_request();

    let resp = test::call_service(&app, req).await;

    // 4. 断言HTTP响应
    // 验证响应状态码是否为 201 Created
    assert!(resp.status().is_success(), "Registration request should succeed");
    assert_eq!(resp.status(), 201, "Status code should be 201 Created");

    // 5. 断言数据库状态
    // 直接查询测试数据库，验证数据是否被正确写入
    let saved_user: Result<(String,), sqlx::Error> = sqlx::query_as("SELECT email FROM users WHERE email = ?")
        .bind("test@example.com")
        .fetch_one(&pool)
        .await;

    assert!(saved_user.is_ok(), "User should be saved to the database");
    assert_eq!(saved_user.unwrap().0, "test@example.com", "Saved email should match");

    let saved_company: Result<(String,), sqlx::Error> = sqlx::query_as("SELECT name FROM companies WHERE name = ?")
        .bind("Test Company")
        .fetch_one(&pool)
        .await;

    assert!(saved_company.is_ok(), "Company should be saved to the database");
}