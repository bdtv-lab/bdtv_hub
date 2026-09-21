mod chat;
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
use tracing::{error, info, trace, warn};

use crate::{
    app,
    server::ws::{chat::PlayerChat, heartbeat::HeartBeat},
};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "action", content = "data")]
/// 来自客户端的事件
pub enum EventFromClient {
    Heartbeat(HeartBeat),
    PlayerChat(PlayerChat),
}

pub(super) async fn ws_connect(
    ws: WebSocketUpgrade,
    State(state): State<Arc<app::State>>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

use tokio::sync::broadcast::error::RecvError;

async fn handle_socket(mut socket: WebSocket, state: Arc<app::State>) {
    let mut event_to_client_rx = state.get_event_to_client_rx();

    loop {
        tokio::select! {
            // ws 接收端
            // 接受来自 ws 客户端的事件
            msg = socket.recv() => {
                let Some(Ok(msg)) = msg else { break };
                match msg {
                    Message::Text(text) => {
                        if let Ok(event) = serde_json::from_str::<EventFromClient>(&text)
                            && let Err(e) = handle_event_from_client(event, Arc::clone(&state)).await
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
            event = event_to_client_rx.recv() => {
                match event {
                    Ok(event) => {
                        todo!()
                    }
                    Err(RecvError::Lagged(n)) => warn!("ws client lagged, dropped {n} events"),
                    Err(RecvError::Closed) => break,
                }
            }
        }
    }
}

/// 处理
async fn handle_event_from_client(event: EventFromClient, state: Arc<app::State>) -> Result<()> {
    match event {
        EventFromClient::Heartbeat(heart_beat) => {
            trace!(
                "Heartbeat received for server {}, with {} players",
                heart_beat.server.slug,
                heart_beat.players.len()
            );
            heartbeat::beat(state, heart_beat).await
        }
        EventFromClient::PlayerChat(chat) => {
            info!(
                "{} in {} said: {}",
                chat.player.nickname, chat.server.slug, chat.content
            );
            chat::received_player_chat(state, chat).await?
        }
    }

    Ok(())
}
