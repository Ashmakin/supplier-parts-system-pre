// src/services/chat_server.rs

use actix::{Actor, Context, Handler, Message, Recipient};
use std::collections::{HashMap, HashSet};

// --- Actor之间传递的消息类型 ---


/// 【新增】加入聊天室
#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinRoom {
    pub rfq_id: i32,
    pub addr: Recipient<ServerMessage>,
}

/// 【新增】离开聊天室
#[derive(Message)]
#[rtype(result = "()")]
pub struct LeaveRoom {
    pub rfq_id: i32,
    pub addr: Recipient<ServerMessage>,
}



/// 客户端发送给服务器的聊天消息 (保持不变)
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub rfq_id: i32,
    pub user_id: i32,
    pub user_full_name: String,
    pub company_name: String,
    pub msg: String,
}

/// 【新增】服务器内部发送给特定用户的通知消息
#[derive(Message)]
#[rtype(result = "()")]
pub struct DirectMessage {
    pub recipient_user_id: i32,
    pub content: String, // JSON格式的通知内容
}

/// 新用户连接 (新增 user_id)
// Connect 和 Disconnect 现在变得更简单
#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub user_id: i32,
    pub addr: Recipient<ServerMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub user_id: i32,
}


/// 服务器发送给客户端的消息 (保持不变)
#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct ServerMessage(pub String);


// --- ChatServer Actor 定义 ---

// ... ChatServer Actor ...
#[derive(Default)]
pub struct ChatServer {
    rooms: HashMap<i32, HashSet<Recipient<ServerMessage>>>,
    sessions: HashMap<i32, Recipient<ServerMessage>>,
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: Connect, _: &mut Self::Context) {
        self.sessions.insert(msg.user_id, msg.addr);
        log::info!("User #{} connected.", msg.user_id);
    }
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) {
        // 当用户断开连接时，我们需要将他们从所有可能加入的房间中移除
        if let Some(addr) = self.sessions.remove(&msg.user_id) {
            for room in self.rooms.values_mut() {
                room.remove(&addr);
            }
        }
        log::info!("User #{} disconnected.", msg.user_id);
    }
}

// 【新增】处理 JoinRoom
impl Handler<JoinRoom> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: JoinRoom, _: &mut Self::Context) {
        self.rooms.entry(msg.rfq_id).or_default().insert(msg.addr);
        log::info!("A user joined RFQ room #{}.", msg.rfq_id);
    }
}

// 【新增】处理 LeaveRoom
impl Handler<LeaveRoom> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: LeaveRoom, _: &mut Self::Context) {
        if let Some(room) = self.rooms.get_mut(&msg.rfq_id) {
            room.remove(&msg.addr);
            log::info!("A user left RFQ room #{}.", msg.rfq_id);
        }
    }
}


// 处理 ClientMessage (聊天消息)
impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Self::Context) {
        let server_msg = format!("chat|{} ({}): {}", msg.user_full_name, msg.company_name, msg.msg);

        if let Some(room) = self.rooms.get(&msg.rfq_id) {
            for addr in room.iter() {
                addr.do_send(ServerMessage(server_msg.clone()));
            }
        }
    }
}

// 【新增】处理 DirectMessage (直接通知)
impl Handler<DirectMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: DirectMessage, _: &mut Self::Context) {
        // 查找在线用户并发送消息
        if let Some(recipient_addr) = self.sessions.get(&msg.recipient_user_id) {
            // 我们在消息前加上一个前缀，方便前端区分
            let full_msg = format!("notification|{}", msg.content);
            recipient_addr.do_send(ServerMessage(full_msg));
            log::info!("Sent direct notification to online user #{}.", msg.recipient_user_id);
        } else {
            log::info!("User #{} is offline. Notification was saved to DB but not pushed.", msg.recipient_user_id);
        }
    }
}