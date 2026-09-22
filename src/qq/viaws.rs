mod handler;
mod utils;
mod wrapper;
use std::sync::Arc;

use onebot_v11::{
    Event,
    connect::ws::{WsConfig, WsConnect},
    event::{message::Message, meta::Meta},
};
use tracing::{error, info};

use anyhow::Result;

use crate::{
    app::{EventToQQ, State},
    config::QqConfig,
};

/// 包装了 ws 连接的请求器
pub struct WsReq {
    state: Arc<State>,
    pub conn: Arc<WsConnect>,
    group_id: i64,
}

impl WsReq {
    pub async fn new(state: Arc<State>, qq: QqConfig) -> Result<Self> {
        // 构造 ws 并连接
        let conn = WsConnect::new(WsConfig {
            host: qq.host,
            port: qq.port,
            access_token: qq.token,
            ..Default::default()
        })
        .await?;

        Ok(Self {
            state,
            conn,
            group_id: qq.group_id,
        })
    }

    /// 处理发往 qq 的事件
    pub async fn handle_event_to_qq(&self, event: EventToQQ) {
        match event {
            EventToQQ::PlayerJoined(player) => {
                if let Err(e) = self.send_player_join(player).await {
                    error!("send player joined failed: {e:?}")
                }
            }
            EventToQQ::PlayerLeft(player) => {
                if let Err(e) = self.send_player_left(player).await {
                    error!("send player left failed: {e:?}")
                }
            }
            EventToQQ::PlayerCountChanged(count) => {
                if let Err(e) = self.send_player_count_change(count).await {
                    error!("send player count changed failed: {e:?}")
                }
            }
            EventToQQ::PlayerSentChat(player, content) => {
                if let Err(e) = self
                    .send_group_msg(self.group_id, format!("{}: {}", player.nickname, content))
                    .await
                {
                    error!("send player chat failed: {e:?}")
                }
            }
        }
    }

    /// 处理来自 QQ 的事件
    pub fn handle_event_from_qq(&self, event: Event) {
        match event {
            Event::Meta(Meta::Lifecycle(lifecycle)) => {
                // 对于生命周期事件，目前只在与 ws 服务器建立连接时打印日志
                if let ("lifecycle", "connect") = (
                    lifecycle.meta_event_type.as_str(),
                    lifecycle.sub_type.as_str(),
                ) {
                    info!("qq connected to user: {}", lifecycle.self_id)
                }
            }

            Event::Message(Message::GroupMessage(group_message))
                if group_message.group_id == self.group_id =>
            {
                self.handle_special_group_msg(group_message);
            }
            _ => {}
        }
    }
}
