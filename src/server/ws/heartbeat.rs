use std::sync::Arc;

use serde::Deserialize;

use crate::{
    app,
    types::{Player, Server},
};

#[derive(Debug, Clone, Deserialize)]
pub struct HeartBeat {
    pub server: Server,
    pub players: Vec<Player>,
}

pub(super) async fn beat(state: Arc<app::State>, payload: HeartBeat) {
    let players = payload.players;
    let server = payload.server;

    state.mark_server_as_online(&server).await;

    for player in players {
        state.mark_player_as_online(&server, &player).await;
    }
}
