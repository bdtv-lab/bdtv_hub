use std::sync::Arc;

use anyhow::Result;
use serde::Deserialize;

use crate::{
    app::{
        self, EventToClient, EventToQQ,
        event::{Audience, event_to_client::ClientSentMsg},
    },
    types::{Player, Server},
};

#[derive(Debug, Clone, Deserialize)]
pub struct PlayerChat {
    pub player: Player,
    pub server: Server,
    pub content: String,
}

pub(super) async fn received_player_chat(
    state: Arc<app::State>,
    payload: PlayerChat,
) -> Result<()> {
    //向 QQ 群发送消息
    let payload_for_qq = payload.clone();
    state.try_send_event_to_qq(EventToQQ::PlayerSentChat(
        payload_for_qq.player,
        payload_for_qq.content,
    ));

    // 向其他 MC 服务器发送消息
    let payload_for_client = payload.clone();
    state.send_event_to_client(
        EventToClient::ClientSentMsg(ClientSentMsg {
            source: payload_for_client.server,
            sender: payload_for_client.player,
            message: payload_for_client.content,
        }),
        Audience::Except(payload.server.slug),
    );

    Ok(())
}
