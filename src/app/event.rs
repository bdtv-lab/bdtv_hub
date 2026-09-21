use tokio::sync::broadcast;

use crate::{app::State, types::Player};

#[derive(Debug, Clone)]
pub enum QQEvent {
    PlayerJoined(Player),
    PlayerLeft(Player),
    PlayerCountChanged(usize),
}

#[derive(Debug, Clone)]
pub enum ServerEvent {}

impl State {
    pub fn get_server_event_rx(&self) -> broadcast::Receiver<ServerEvent> {
        self.server_event_tx.subscribe()
    }
}
