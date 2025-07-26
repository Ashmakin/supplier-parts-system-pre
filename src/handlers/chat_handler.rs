// src/handlers/chat_handler.rs

use actix::{Actor, Addr};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use querystring::querify;
use sqlx::MySqlPool;

use crate::utils::auth_utils;
use crate::services::{
    chat_server::ChatServer,
    chat_session::ChatSession,
};

// HTTP端点，用于将HTTP连接升级为WebSocket连接
pub async fn start_chat_session(
    req: HttpRequest,
    stream: web::Payload,
    rfq_id: web::Path<i32>,
    chat_server_addr: web::Data<Addr<ChatServer>>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse, Error> {
    // 从查询参数中手动提取token
    let qs = querify(req.query_string());
    let token = qs.iter().find(|(k, _)| k == &"token").map(|(_, v)| *v);

    if token.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    // 手动验证token
    let claims = match auth_utils::validate_jwt(token.unwrap()) {
        Ok(c) => c,
        Err(_) => return Ok(HttpResponse::Unauthorized().finish())
    };
    let session = ChatSession::new(
        rfq_id.into_inner(),
        claims,
        chat_server_addr.get_ref().clone(),
        pool.get_ref().clone()
    );
    ws::start(session, &req, stream)
}