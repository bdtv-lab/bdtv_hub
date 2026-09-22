use tokio::sync::broadcast;

use crate::{app::State, types::Player};

#[derive(Debug, Clone)]
pub enum EventToQQ {
    // 玩家数量事件
    PlayerJoined(Player),
    PlayerLeft(Player),
    PlayerCountChanged(usize),
    // 聊天事件
    PlayerSentChat(Player, String),
}

#[derive(Debug, Clone)]
pub enum EventToClient {}

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
        self.event_to_client_tx
            .send(EventToClientWrapper { event, slug_filter });
    }
}
