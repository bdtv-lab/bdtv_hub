mod chat;
mod heartbeat;

use std::sync::Arc;

use anyhow::Result;
use axum::{
    extract::{
        Query, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::Response,
};
use serde::Deserialize;
use tokio::sync::broadcast;
use tracing::{debug, error, info, trace, warn};

use crate::{
    app::{self},
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

#[derive(Debug, Deserialize)]
pub struct WsParams {
    slug: String,
}

pub(super) async fn ws_connect(
    ws: WebSocketUpgrade,
    Query(params): Query<WsParams>,
    State(state): State<Arc<app::State>>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state, params.slug))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<app::State>, server_slug: String) {
    debug!("websocket upgraded with client: {}", server_slug);

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
                            error!("error when handling event from client({}): {}", server_slug, e);
                        }
                    }
                    Message::Close(_) => break,
                    _ => {}
                }
            }

            // ws 发送端
            // 主动发送 ws 到客户端
            event_wrapper = event_to_client_rx.recv() => {
                match event_wrapper {
                    Ok(event_wrapper) => {
                        // 检查是否属于事件接收者
                        if !event_wrapper.audience.includes(&server_slug) {
                            trace!("ignored event to client({})", server_slug);
                            continue;
                        }

                        // 尝试序列化并发送事件
                        let event = event_wrapper.event;
                        if let Err(e) = async || -> Result<()> {
                            let serded_event = serde_json::to_string(&event)?;
                            socket.send(Message::Text(serded_event.into())).await?;

                            Ok(())
                        }().await  {
                            error!("can not send to client({}): {}", server_slug, e)
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(n)) => warn!("MC ws client lagged, dropped {n} events"),
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
        }
    }
}

/// 处理来自 MC 服务器的事件
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
