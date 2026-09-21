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
use tracing::{error, warn};

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

use tokio::sync::broadcast::error::RecvError;

async fn handle_socket(mut socket: WebSocket, state: Arc<app::State>) {
    let mut server_event_rx = state.get_server_event_rx();

    loop {
        tokio::select! {
            // ws 接收端
            // 接受来自 ws 客户端的事件
            msg = socket.recv() => {
                let Some(Ok(msg)) = msg else { break };
                match msg {
                    Message::Text(text) => {
                        if let Ok(action) = serde_json::from_str::<Action>(&text)
                            && let Err(e) = handle_action(action, Arc::clone(&state)).await
                        {
                            error!("{}", e);
                        }
                    }
                    Message::Close(_) => break,
                    _ => {}
                }
            }

            // ws 发送端
            // 主动发送 ws 到客户端
            event = server_event_rx.recv() => {
                match event {
                    Ok(event) => {
                    }
                    Err(RecvError::Lagged(n)) => warn!("ws client lagged, dropped {n} events"),
                    Err(RecvError::Closed) => break,
                }
            }
        }
    }
}

async fn handle_action(action: Action, state: Arc<app::State>) -> Result<()> {
    match action {
        Action::Heartbeat(heart_beat) => heartbeat::beat(state, heart_beat).await,
    }

    Ok(())
}
