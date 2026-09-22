pub mod event_to_client;

use serde::Serialize;
use tokio::sync::broadcast;
use tracing::error;

use crate::{
    app::{
        State,
        event::event_to_client::{ClientSentMsg, GroupMemberSentMsg},
    },
    types::Player,
};

#[derive(Debug, Clone)]
pub enum EventToQQ {
    // 玩家数量事件
    PlayerJoined(Player),
    PlayerLeft(Player),
    PlayerCountChanged(usize),

    // 聊天事件
    PlayerSentChat(Player, String),
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "event", content = "data")]
pub enum EventToClient {
    /// QQ 群消息推送到 MC 事件
    GroupMemberSentMsg(GroupMemberSentMsg),
    /// MC 服务器发送的消息
    ClientSentMsg(ClientSentMsg),
    /// 终端消息
    ConsoleSentMsg(String),
}

#[derive(Debug, Clone)]
/// ClientSentMsg 的接收者选择器
pub enum Audience {
    /// 所有 MC 服务器
    All,
    /// 仅特定服务器
    Only(String),
    /// 排除特定服务器
    Except(String),
}

impl Audience {
    pub fn includes(&self, slug: &str) -> bool {
        match self {
            Audience::All => true,
            Audience::Only(s) => s == slug,
            Audience::Except(s) => s != slug,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EventToClientWrapper {
    pub event: EventToClient,
    pub audience: Audience,
}

impl State {
    pub fn get_event_to_client_rx(&self) -> broadcast::Receiver<EventToClientWrapper> {
        self.event_to_client_tx.subscribe()
    }

    pub fn send_event_to_client(&self, event: EventToClient, audience: Audience) {
        if let Err(e) = self
            .event_to_client_tx
            .send(EventToClientWrapper { event, audience })
        {
            error!("can not send event to client: {}", e);
        }
    }
}
