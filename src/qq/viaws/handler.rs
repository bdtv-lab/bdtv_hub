use onebot_v11::event::message::GroupMessage;
use tracing::debug;

use crate::{
    app::{EventToClient, event::event_to_client::GroupMemberSentMsg},
    qq::viaws::WsReq,
};

/// 从群消息中提取出用户昵称
fn get_sender_name(group_message: GroupMessage) -> String {
    let sender = group_message.sender;
    // 检查群名片是否可用
    if let Some(card) = sender.card
        && card.len() != 0
    {
        return card;
    }
    // 检查用户昵称是否可用
    else if let Some(nickname) = sender.nickname
        && nickname.len() != 0
    {
        return nickname;
    }
    // 实在不行采用 QQ 号
    else {
        return group_message.user_id.to_string();
    }
}

impl WsReq {
    /// 处理配置文件中设定群中的消息
    pub(super) fn handle_special_group_msg(&self, group_message: GroupMessage) {
        let sender = &group_message.sender;
        debug!(
            "qq message received from group {}: {}({}): {}",
            group_message.group_id,
            get_sender_name(group_message.clone()),
            sender.user_id.unwrap_or_default(),
            group_message.raw_message
        );

        self.state.send_event_to_client(
            EventToClient::GroupMemberSentMsg(GroupMemberSentMsg {
                sender_id: group_message.user_id,
                sender_nickname: get_sender_name(group_message.clone()),
                message: group_message.raw_message,
            }),
            None,
        );
    }
}
