mod heartbeat;

use std::sync::Arc;

use anyhow::Result;
use axum::{
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::Response,
};
use serde::Deserialize;
use tracing::error;

use crate::{app, server::ws::heartbeat::HeartBeat};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "action", content = "data")]
pub enum Action {
    Heartbeat(HeartBeat),
}

pub(super) async fn ws_connect(
    ws: WebSocketUpgrade,
    State(state): State<Arc<app::State>>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<app::State>) {
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(utf8_bytes) => {
                if let Ok(action) = serde_json::from_str::<Action>(&utf8_bytes)
                    && let Err(e) = handle_action(action, Arc::clone(&state)).await {
                        error!("{}", e)
                    }
            }

            Message::Close(_) => break,
            _ => continue,
        }
    }
}

async fn handle_action(action: Action, state: Arc<app::State>) -> Result<()> {
    match action {
        Action::Heartbeat(heart_beat) => heartbeat::beat(state, heart_beat).await,
    }

    Ok(())
}
