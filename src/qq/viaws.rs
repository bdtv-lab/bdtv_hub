mod utils;
mod wrapper;

use std::sync::Arc;

use onebot_v11::{
    connect::ws::{WsConfig, WsConnect},
    event::{message::Message, meta::Meta},
};
use tracing::{debug, error, info};

use anyhow::Result;

use crate::app;

/// 包装了 ws 连接的请求器
pub struct WsReq {
    pub conn: Arc<WsConnect>,
    group_id: i64,
}

impl WsReq {
    pub async fn new(
        host: String,
        port: u16,
        token: Option<String>,
        group_id: i64,
    ) -> Result<Self> {
        // 构造 ws 并连接
        let conn = WsConnect::new(WsConfig {
            host: host,
            port: port,
            access_token: token,
            ..Default::default()
        })
        .await?;

        Ok(Self { conn, group_id })
    }

    /// 处理来自服务器的事件
    pub async fn handle_server_event(&self, event: app::Event) {
        match event {
            app::Event::PlayerJoined(player) => {
                if let Err(e) = self.send_player_join(player).await {
                    error!("Send Player joined failed: {e:?}")
                }
            }
            app::Event::PlayerLeft(player) => {
                if let Err(e) = self.send_player_left(player).await {
                    error!("Send Player left failed: {e:?}")
                }
            }
            app::Event::PlayerCountChanged(count) => {
                if let Err(e) = self.send_player_count_change(count).await {
                    error!("Send Player count changed failed: {e:?}")
                }
            }
        }
    }

    /// 处理来自 QQ 的事件
    pub fn handle_qq_event(&self, event: onebot_v11::Event) {
        match event {
            onebot_v11::Event::Meta(meta) => match meta {
                Meta::Lifecycle(lifecycle) => {
                    // 对于生命周期事件，目前只在与 ws 服务器建立连接时打印日志
                    match (
                        lifecycle.meta_event_type.as_str(),
                        lifecycle.sub_type.as_str(),
                    ) {
                        ("lifecycle", "connect") => {
                            info!("qq connected to user: {}", lifecycle.self_id)
                        }
                        _ => {}
                    }
                }
                _ => {}
            },
            onebot_v11::Event::Message(message) => match message {
                Message::GroupMessage(group_message) => {
                    // 用作调试以及接口保留，处理特定 qq 群收到的消息
                    if group_message.group_id == self.group_id {
                        debug!(
                            "message from {}: {}",
                            self.group_id, group_message.raw_message
                        )
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}
