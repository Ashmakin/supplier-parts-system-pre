// src/services/chat_session.rs

use actix::{Actor, ActorContext, Addr, AsyncContext, Handler, Running, StreamHandler};
use actix_web_actors::ws;
use sqlx::MySqlPool;
use std::time::{Duration, Instant};

use crate::models::user::{Claims,Company};
use crate::services::chat_server::{self, ChatServer};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct ChatSession {
    rfq_id: i32,
    user_claims: Claims,
    hb: Instant,
    chat_server_addr: Addr<ChatServer>,
    db_pool: MySqlPool,
}

impl ChatSession {
    pub fn new(
        rfq_id: i32,
        user_claims: Claims,
        chat_server_addr: Addr<ChatServer>,
        db_pool: MySqlPool,
    ) -> Self {
        Self {
            rfq_id,
            user_claims,
            hb: Instant::now(),
            chat_server_addr,
            db_pool,
        }
    }

    // 心跳检查函数
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Websocket client heartbeat failed, disconnecting!");
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}

impl Actor for ChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx); // 启动心跳

        let addr = ctx.address().recipient();
        self.chat_server_addr
            .do_send(chat_server::Connect { rfq_id: self.rfq_id, addr });
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        let addr = ctx.address().recipient();
        self.chat_server_addr
            .do_send(chat_server::Disconnect { rfq_id: self.rfq_id, addr });
        Running::Stop
    }
}

// 处理从服务器发来的消息
impl Handler<chat_server::ServerMessage> for ChatSession {
    type Result = ();

    fn handle(&mut self, msg: chat_server::ServerMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

// 处理WebSocket的文本/二进制消息
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
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
                let msg_text = text.to_string();
                if msg_text.trim().is_empty() { return; }

                // 克隆所需数据，以便在异步块中使用
                let rfq_id = self.rfq_id;
                let user_id = self.user_claims.sub;
                let chat_server_addr = self.chat_server_addr.clone();
                let pool = self.db_pool.clone();

                // 【关键修复】查询数据库获取用户名和公司名
                // 然后将消息发送到ChatServer和数据库
                actix::spawn(async move {
                    // 查询发送者的全名和公司名
                    let user_info: Result<(String, String), sqlx::Error> = sqlx::query_as(
                        "SELECT u.full_name, c.name as company_name FROM users u JOIN companies c ON u.company_id = c.id WHERE u.id = ?"
                    )
                        .bind(user_id)
                        .fetch_one(&pool)
                        .await;

                    let (user_full_name, company_name) = match user_info {
                        Ok((name, company)) => (name, company),
                        Err(e) => {
                            println!("Failed to fetch user info for chat: {:?}", e);
                            // 提供一个回退值
                            (format!("User {}", user_id), format!("Company {}", "Unknown"))
                        }
                    };

                    // 1. 将带有完整信息的消息发送到ChatServer进行广播
                    chat_server_addr.do_send(chat_server::ClientMessage {
                        rfq_id,
                        user_id,
                        user_full_name: user_full_name.clone(),
                        company_name: company_name.clone(),
                        msg: msg_text.clone(),
                    });

                    // 2. 将消息存入数据库
                    let res = sqlx::query(
                        "INSERT INTO chat_messages (rfq_id, user_id, user_full_name, company_name, message_text) VALUES (?, ?, ?, ?, ?)"
                    )
                        .bind(rfq_id)
                        .bind(user_id)
                        .bind(user_full_name)
                        .bind(company_name)
                        .bind(msg_text)
                        .execute(&pool)
                        .await;
                    if let Err(e) = res {
                        println!("Failed to save chat message: {:?}", e);
                    }
                });
            }
            _ => ctx.stop(),
        }
    }
}