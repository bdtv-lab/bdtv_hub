use onebot_v11::event::message::GroupMessage;
use tracing::debug;

use crate::qq::viaws::WsReq;

impl WsReq {
    pub(super) fn handle_special_group_msg(&self, group_message: GroupMessage) {
        let sender = group_message.sender;
        debug!(
            "qq message received from group {}: {}({}): {}",
            group_message.group_id,
            sender.card.unwrap_or_default(),
            sender.user_id.unwrap_or_default(),
            group_message.raw_message
        )
    }
}
