// src/services/payment_service.rs

use crate::{
    errors::AppError,
    models::{order::PurchaseOrder, user::Claims},
};
use sqlx::MySqlPool;
use std::env;
// 导入 async_stripe 的相关模块
use stripe::{
    Client,
    CheckoutSession, CheckoutSessionMode, CreateCheckoutSession, CreateCheckoutSessionLineItems, CreateCheckoutSessionLineItemsPriceData,CreateCheckoutSessionLineItemsPriceDataProductData
};
// 导入 ToPrimitive trait 以使用 .to_f64()
use num_traits::ToPrimitive;

pub async fn create_checkout_session(
    pool: &MySqlPool,
    order_id: i32,
    claims: &Claims,
) -> Result<String, AppError> {
    // 1. 验证订单
    let order: PurchaseOrder = sqlx::query_as(
        "SELECT po.*, r.title as rfq_title, b.name as buyer_name, s.name as supplier_name
         FROM purchase_orders po
         JOIN rfqs r ON po.rfq_id = r.id
         JOIN companies b ON po.buyer_company_id = b.id
         JOIN companies s ON po.supplier_company_id = s.id
         WHERE po.id = ? AND po.buyer_company_id = ?"
    )
        .bind(order_id)
        .bind(claims.company_id)
        .fetch_one(pool)
        .await
        .map_err(|_| AppError::BadRequest("Order not found or you are not authorized.".to_string()))?;

    if order.payment_status == "PAID" {
        return Err(AppError::BadRequest("This order has already been paid.".to_string()));
    }

    // 2. 配置Stripe客户端
    let secret_key = env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY must be set");
    let client = Client::new(secret_key);

    // 3. 定义URL
    let frontend_url = env::var("FRONTEND_URL").expect("FRONTEND_URL must be set");
    let success_url = format!("{}/payment/success?session_id={{CHECKOUT_SESSION_ID}}", frontend_url);
    let cancel_url = format!("{}/orders", frontend_url);

    // 4. 【关键修复】使用 async-stripe 的API创建Checkout会话
    let mut params = CreateCheckoutSession::new();
    params.success_url = Some(&*success_url);
    params.cancel_url = Some(&*cancel_url);
    params.mode = Some(CheckoutSessionMode::Payment);
    params.line_items = Some(vec![CreateCheckoutSessionLineItems {
        price_data: Some(CreateCheckoutSessionLineItemsPriceData {
            currency: stripe::Currency::USD, // Or your currency
            product_data: Some(CreateCheckoutSessionLineItemsPriceDataProductData {
                name: order.rfq_title.clone(),
                ..Default::default()
            }),
            unit_amount: Some((order.total_amount.to_f64().unwrap_or(0.0) * 100.0) as i64),
            ..Default::default()
        }),
        quantity: Some(1),
        ..Default::default()
    }]);

    let session = CheckoutSession::create(&client, params).await
        .map_err(|e| AppError::InternalServerError(format!("Stripe error: {}", e)))?;

    // 5. 将会话ID存入数据库
    sqlx::query("UPDATE purchase_orders SET stripe_session_id = ? WHERE id = ?")
        .bind(session.id.to_string()) // 使用 .to_string() 转换ID
        .bind(order_id)
        .execute(pool)
        .await?;

    Ok(session.id.to_string())
}