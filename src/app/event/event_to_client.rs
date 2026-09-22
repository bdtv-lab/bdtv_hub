use serde::Serialize;

use crate::types::{Player, Server};

#[derive(Serialize, Debug, Clone)]
pub struct GroupMemberSentMsg {
    /// 发送者 QQ 号
    pub sender_id: i64,
    /// 发送者昵称
    pub sender_nickname: String,
    /// 消息（未处理）
    pub message: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct ClientSentMsg {
    /// 来源服务器
    pub source: Server,
    /// 发送者
    pub sender: Player,
    /// 消息
    pub message: String,
}
