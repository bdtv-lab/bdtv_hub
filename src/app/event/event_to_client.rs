use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct GroupMemberSentMsg {
    /// 发送者 QQ 号
    pub sender_id: i64,
    /// 发送者昵称
    pub sender_nickname: String,
    /// 消息（未处理）
    pub message: String,
}
