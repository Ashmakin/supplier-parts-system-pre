use actix::{Actor, ActorContext, Addr, AsyncContext, Handler, Running, StreamHandler};
use actix_web::{web, Error, HttpMessage, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::time::{Duration, Instant};

use crate::{
    errors::AppError,
    models::user::Claims,
    services::{
        chat_server::{self, ChatServer, ClientMessage, JoinRoom, LeaveRoom, ServerMessage},
        notification_service::NotificationBuilder,
    },
    utils::auth_utils,
};
use querystring::querify;
use sqlx::MySqlPool;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// WsSession 代表一个单一的、持久的 WebSocket 连接
pub struct WsSession {
    user_id: i32,
    // 当前加入的聊天室ID, 如果有的话
    current_rfq_id: Option<i32>,
    hb: Instant,
    chat_server_addr: Addr<ChatServer>,
    db_pool: MySqlPool,
}

impl WsSession {
    pub fn new(user_id: i32, chat_server_addr: Addr<ChatServer>, db_pool: MySqlPool) -> Self {
        Self {
            user_id,
            current_rfq_id: None,
            hb: Instant::now(),
            chat_server_addr,
            db_pool,
        }
    }

    // 心跳检查函数
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                log::warn!("Websocket client heartbeat failed, disconnecting user #{}", act.user_id);
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    // 当Actor启动时 (即WebSocket连接建立)
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        let addr = ctx.address().recipient();
        // 向ChatServer注册自己
        self.chat_server_addr.do_send(chat_server::Connect {
            user_id: self.user_id,
            addr,
        });
    }

    // 当Actor停止时 (即WebSocket连接断开)
    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        // 向ChatServer注销自己
        self.chat_server_addr.do_send(chat_server::Disconnect { user_id: self.user_id });
        Running::Stop
    }
}

// 接收来自ChatServer的消息，并将其发送给客户端浏览器
impl Handler<ServerMessage> for WsSession {
    type Result = ();
    fn handle(&mut self, msg: ServerMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

// 处理从客户端浏览器发来的WebSocket消息
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                let text = text.to_string();
                log::debug!("WebSocket received text from user {}: {}", self.user_id, text);

                // 解析前端发来的 "COMMAND|VALUE" 格式的消息
                if let Some((command, value)) = text.split_once('|') {
                    match command {
                        "JOIN" => {
                            if let Ok(rfq_id) = value.parse() {
                                self.current_rfq_id = Some(rfq_id);
                                self.chat_server_addr.do_send(JoinRoom { rfq_id, addr: ctx.address().recipient() });
                            }
                        }
                        "LEAVE" => {
                            if let Ok(rfq_id) = value.parse() {
                                if self.current_rfq_id == Some(rfq_id) {
                                    self.current_rfq_id = None;
                                }
                                self.chat_server_addr.do_send(LeaveRoom { rfq_id, addr: ctx.address().recipient() });
                            }
                        }
                        "CHAT" => {
                            // 解析 "rfqId|message_text" 格式
                            if let Some((rfq_id_str, msg_text)) = value.split_once('|') {
                                if let Ok(rfq_id) = rfq_id_str.parse::<i32>() {
                                    if msg_text.trim().is_empty() { return; }

                                    let pool = self.db_pool.clone();
                                    let chat_server_addr = self.chat_server_addr.clone();
                                    let current_user_id = self.user_id;
                                    let message_to_save = msg_text.to_string();

                                    // 异步执行数据库操作和通知，避免阻塞Actor
                                    actix::spawn(async move {
                                        // 1. 查询发送者信息
                                        let user_info: Result<(String, String), _> = sqlx::query_as(
                                            "SELECT u.full_name, c.name as company_name FROM users u JOIN companies c ON u.company_id = c.id WHERE u.id = ?"
                                        )
                                            .bind(current_user_id)
                                            .fetch_one(&pool)
                                            .await;

                                        if let Ok((user_full_name, company_name)) = user_info {
                                            // 2. 将消息存入数据库
                                            if let Err(e) = sqlx::query(
                                                "INSERT INTO chat_messages (rfq_id, user_id, user_full_name, company_name, message_text) VALUES (?, ?, ?, ?, ?)"
                                            )
                                                .bind(rfq_id)
                                                .bind(current_user_id)
                                                .bind(&user_full_name)
                                                .bind(&company_name)
                                                .bind(&message_to_save)
                                                .execute(&pool)
                                                .await {
                                                log::error!("Failed to save chat message to DB: {:?}", e);
                                                return; // 保存失败则不继续
                                            }

                                            // 3. 将消息广播到聊天室
                                            chat_server_addr.do_send(ClientMessage {
                                                rfq_id,
                                                user_id: current_user_id,
                                                user_full_name: user_full_name.clone(),
                                                company_name,
                                                msg: message_to_save,
                                            });

                                            // 4. 为房间内其他用户创建通知 (简化实现：只通知RFQ所有者)
                                            let rfq_owner_id: Result<(i32,), _> = sqlx::query_as("SELECT u.id FROM rfqs r JOIN users u ON r.buyer_company_id = u.company_id WHERE r.id = ? LIMIT 1")
                                                .bind(rfq_id).fetch_one(&pool).await;

                                            if let Ok((owner_id,)) = rfq_owner_id {
                                                if owner_id != current_user_id { // 不给自己发通知
                                                    let _ = NotificationBuilder::new(
                                                        owner_id,
                                                        format!("New message from {} in RFQ #{}", &user_full_name, rfq_id)
                                                    )
                                                        .with_link(format!("/rfqs/{}", rfq_id))
                                                        .send(&pool, &chat_server_addr)
                                                        .await;
                                                }
                                            }
                                        } else {
                                            log::error!("Could not find user info for user_id: {}", current_user_id);
                                        }
                                    });
                                }
                            }
                        }
                        _ => log::warn!("Unknown WebSocket command: {}", command),
                    }
                }
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

/// HTTP端点，用于将HTTP连接升级为全局WebSocket连接
pub async fn start_global_session(
    req: HttpRequest,
    stream: web::Payload,
    chat_server_addr: web::Data<Addr<ChatServer>>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse, Error> {
    // 从查询参数中手动提取和验证token
    let qs = querify(req.query_string());
    let token = qs
        .iter()
        .find(|(k, _)| k == &"token")
        .map(|(_, v)| *v)
        .ok_or(AppError::AuthError)?;

    let claims = auth_utils::validate_jwt(token).map_err(|_| AppError::AuthError)?;

    let session = WsSession::new(claims.sub, chat_server_addr.get_ref().clone(), pool.get_ref().clone());
    ws::start(session, &req, stream)
}