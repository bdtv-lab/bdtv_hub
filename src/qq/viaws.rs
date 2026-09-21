mod handler;
mod utils;
mod wrapper;
use std::sync::Arc;

use onebot_v11::{
    connect::ws::{WsConfig, WsConnect},
    event::{message::Message, meta::Meta},
};
use tracing::{error, info};

use anyhow::Result;

use crate::app::EventToQQ;

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
            host,
            port,
            access_token: token,
            ..Default::default()
        })
        .await?;

        Ok(Self { conn, group_id })
    }

    /// 处理发往 qq 的事件
    pub async fn handle_event_to_qq(&self, event: EventToQQ) {
        match event {
            EventToQQ::PlayerJoined(player) => {
                if let Err(e) = self.send_player_join(player).await {
                    error!("Send Player joined failed: {e:?}")
                }
            }
            EventToQQ::PlayerLeft(player) => {
                if let Err(e) = self.send_player_left(player).await {
                    error!("Send Player left failed: {e:?}")
                }
            }
            EventToQQ::PlayerCountChanged(count) => {
                if let Err(e) = self.send_player_count_change(count).await {
                    error!("Send Player count changed failed: {e:?}")
                }
            }
        }
    }

    /// 处理来自 QQ 的事件
    pub fn handle_event_from_qq(&self, event: onebot_v11::Event) {
        match event {
            onebot_v11::Event::Meta(meta) => {
                if let Meta::Lifecycle(lifecycle) = meta {
                    // 对于生命周期事件，目前只在与 ws 服务器建立连接时打印日志
                    if let ("lifecycle", "connect") = (
                        lifecycle.meta_event_type.as_str(),
                        lifecycle.sub_type.as_str(),
                    ) {
                        info!("qq connected to user: {}", lifecycle.self_id)
                    }
                }
            }

            onebot_v11::Event::Message(message) => match message {
                Message::GroupMessage(group_message) => {
                    // 用作调试以及接口保留，处理特定 qq 群收到的消息
                    if group_message.group_id == self.group_id {
                        self.handle_special_group_msg(group_message);
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}
