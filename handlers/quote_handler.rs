use actix::Addr;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use sqlx::MySqlPool;

use crate::{
    errors::AppError,
    models::{quote::CreateQuoteDto, user::Claims},
    services::{chat_server::ChatServer, quote_service},
};

/// 处理供应方(Supplier)为某个RFQ提交新报价的请求
/// POST /api/rfqs/{rfq_id}/quotes
pub async fn post_quote(
    pool: web::Data<MySqlPool>,
    chat_server: web::Data<Addr<ChatServer>>, // <-- 新增：获取WebSocket服务器地址
    rfq_id: web::Path<i32>,
    dto: web::Json<CreateQuoteDto>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    // 从请求中提取经过Auth中间件验证后的用户信息
    let claims = req
        .extensions()
        .get::<Claims>()
        .cloned()
        .ok_or(AppError::AuthError)?;

    // 调用service层函数，并传入 chat_server 地址以触发实时通知
    let quote_id = quote_service::create_quote(
        pool.get_ref(),
        chat_server.get_ref(),
        rfq_id.into_inner(),
        dto.into_inner(),
        &claims,
    )
        .await?;

    // 返回 201 Created 和新创建的报价ID
    Ok(HttpResponse::Created().json(serde_json::json!({ "quote_id": quote_id })))
}

/// 处理采购方(Buyer)获取其RFQ收到的所有报价列表的请求
/// GET /api/rfqs/{rfq_id}/quotes
pub async fn get_quotes(
    pool: web::Data<MySqlPool>,
    rfq_id: web::Path<i32>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    // 提取用户信息
    let claims = req
        .extensions()
        .get::<Claims>()
        .cloned()
        .ok_or(AppError::AuthError)?;

    // 调用service层函数获取数据
    let quotes =
        quote_service::get_quotes_for_rfq(pool.get_ref(), rfq_id.into_inner(), &claims).await?;

    // 返回 200 OK 和报价列表
    Ok(HttpResponse::Ok().json(quotes))
}

/// 处理采购方(Buyer)接受某个报价的请求
/// POST /api/quotes/{quote_id}/accept
pub async fn post_accept_quote(
    pool: web::Data<MySqlPool>,
    chat_server:  web::Data<Addr<ChatServer>>,
    quote_id: web::Path<i32>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    // 提取用户信息
    let claims = req
        .extensions()
        .get::<Claims>()
        .cloned()
        .ok_or(AppError::AuthError)?;

    // 调用service层函数，该函数会创建PO并更新相关状态
    let po_id =
        quote_service::accept_quote(
            pool.get_ref(),
            chat_server.get_ref(),
            quote_id.into_inner(),
            &claims).await?;

    // 返回 200 OK 和新创建的采购订单ID
    Ok(HttpResponse::Ok().json(serde_json::json!({ "purchase_order_id": po_id })))
}