use std::sync::Arc;

use anyhow::Result;
use serde::Deserialize;

use crate::{
    app::{self, EventToQQ},
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
    state
        .event_to_qq_tx
        .send(EventToQQ::PlayerSentChat(payload.player, payload.content))
        .await?;

    Ok(())
}
