// src/services/chat_server.rs

use actix::{Actor, Context, Handler, Message, Recipient};
use std::collections::{HashMap, HashSet};

// --- Actor之间传递的消息类型 ---

/// 客户端发送给服务器的消息
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub rfq_id: i32,
    pub user_id: i32,
    pub user_full_name: String,
    pub company_name: String,
    pub msg: String,
}

/// 新用户连接
#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub rfq_id: i32,
    pub addr: Recipient<ServerMessage>,
}

/// 用户断开连接
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub rfq_id: i32,
    pub addr: Recipient<ServerMessage>,
}

/// 服务器发送给客户端的消息
#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct ServerMessage(pub String);


// --- ChatServer Actor 定义 ---

#[derive(Default)]
pub struct ChatServer {
    // 房间列表: key是rfq_id, value是该房间所有客户端连接的集合
    rooms: HashMap<i32, HashSet<Recipient<ServerMessage>>>,
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

// 处理 Connect 消息
impl Handler<Connect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) {
        // 获取或创建房间，并添加新成员
        self.rooms.entry(msg.rfq_id).or_default().insert(msg.addr);
        println!("Someone connected to RFQ room {}", msg.rfq_id);
    }
}

// 处理 Disconnect 消息
impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) {
        if let Some(room) = self.rooms.get_mut(&msg.rfq_id) {
            room.remove(&msg.addr);
            println!("Someone disconnected from RFQ room {}", msg.rfq_id);
        }
    }
}

// 处理 ClientMessage 消息
impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Self::Context) {
        // 格式化消息
        let server_msg = format!("{}({}): {}", msg.user_full_name, msg.company_name, msg.msg);

        // 将消息广播给房间里的所有成员
        if let Some(room) = self.rooms.get(&msg.rfq_id) {
            for addr in room.iter() {
                addr.do_send(ServerMessage(server_msg.clone()));
            }
        }
    }
}