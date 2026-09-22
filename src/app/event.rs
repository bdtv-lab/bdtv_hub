pub mod event_to_client;

use serde::Serialize;
use tokio::sync::broadcast;
use tracing::error;

use crate::{
    app::{State, event::event_to_client::GroupMemberSentMsg},
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
}

type SlugFilter = fn(&str) -> bool;

#[derive(Debug, Clone)]
pub struct EventToClientWrapper {
    pub event: EventToClient,
    pub slug_filter: Option<SlugFilter>,
}

impl State {
    pub fn get_event_to_client_rx(&self) -> broadcast::Receiver<EventToClientWrapper> {
        self.event_to_client_tx.subscribe()
    }

    pub fn send_event_to_client(&self, event: EventToClient, slug_filter: Option<SlugFilter>) {
        if let Err(e) = self
            .event_to_client_tx
            .send(EventToClientWrapper { event, slug_filter })
        {
            error!("can not send event to client: {}", e);
        }
    }
}
