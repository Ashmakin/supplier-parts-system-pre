// tests/rfq_test.rs

use crate::{config, api, models::user::{LoginResponse, RegisterDto}};
use actix_web::{test, web, App, http::header, ResponseError}; // Import ResponseError

#[actix_web::test]
async fn test_create_rfq_success_with_auth() {
    let pool = config::configure_test_db().await;
    let (jwt, company_id) = setup_and_login(&pool).await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(api::config)
    ).await;

    // The DTO is now a simple JSON object again
    let rfq_dto = serde_json::json!({
        "title": "Test RFQ from integration test",
        "description": "This is a test description.",
        "quantity": 1000
    });

    let req = test::TestRequest::post()
        .uri("/api/rfqs")
        .insert_header((header::AUTHORIZATION, format!("Bearer {}", jwt)))
        .set_json(&rfq_dto)
        .to_request();

    let resp = test::call_service(&app, req).await;

    // This assertion will now pass because the endpoint expects JSON
    assert_eq!(resp.status(), 201, "Status code should be 201 Created");

    let saved_rfq: Result<(String,), sqlx::Error> = sqlx::query_as("SELECT title FROM rfqs WHERE buyer_company_id = ?")
        .bind(company_id)
        .fetch_one(&pool)
        .await;

    assert!(saved_rfq.is_ok(), "RFQ should be saved to the database");
    assert_eq!(saved_rfq.unwrap().0, "Test RFQ from integration test");
}


#[actix_web::test]
async fn test_create_rfq_unauthorized_without_token() {
    // 1. Setup
    let pool = config::configure_test_db().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(api::config)
    ).await;

    // 2. Prepare request data
    let rfq_dto = serde_json::json!({
        "title": "Unauthorized RFQ",
        "description": "This should fail.",
        "quantity": 50
    });

    // 3. Send request
    let req = test::TestRequest::post()
        .uri("/api/rfqs")
        .set_json(&rfq_dto)
        .to_request();

    // 4. Use `try_call_service` which returns a Result
    let result = test::try_call_service(&app, req).await;

    // 5. Assert that the result is an error and check the response it generates
    match result {
        Ok(_) => panic!("Request should have failed but it succeeded"),
        Err(e) => {
            // The 'e' variable is an actix_web::Error which implements ResponseError.
            // We can call .error_response() directly on it.
            let response = e.error_response();
            assert_eq!(response.status(), 401, "Error response status should be 401");
        }
    }
}


// --- Helper Function (add serde_json and sqlx::Row imports if needed) ---
use serde_json::json;
use sqlx::Row;
use crate::errors::AppError;

async fn setup_and_login(pool: &sqlx::MySqlPool) -> (String, i32) {
    let register_dto = RegisterDto {
        company_name: "Test Buyer Corp".to_string(),
        company_type: "BUYER".to_string(),
        city: "City".to_string(),
        email: "buyer_for_rfq_test@example.com".to_string(),
        password: "password123".to_string(),
        full_name: "Buyer Test User".to_string(),
    };

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(api::config)
    ).await;

    let req = test::TestRequest::post().uri("/api/auth/register").set_json(&register_dto).to_request();
    test::call_service(&app, req).await;

    let login_dto = json!({
        "email": "buyer_for_rfq_test@example.com",
        "password": "password123"
    });
    let req = test::TestRequest::post().uri("/api/auth/login").set_json(&login_dto).to_request();
    let resp = test::call_service(&app, req).await;

    let login_resp: LoginResponse = test::read_body_json(resp).await;
    let token = login_resp.token;

    let company_row = sqlx::query("SELECT id FROM companies WHERE name = 'Test Buyer Corp'")
        .fetch_one(pool)
        .await
        .unwrap();
    let company_id: i32 = company_row.get("id");

    (token, company_id)
}