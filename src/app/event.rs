use tokio::sync::broadcast;

use crate::{app::State, types::Player};

#[derive(Debug, Clone)]
pub enum EventToQQ {
    PlayerJoined(Player),
    PlayerLeft(Player),
    PlayerCountChanged(usize),
}

#[derive(Debug, Clone)]
pub enum EventToClient {}

impl State {
    pub fn get_event_to_client_rx(&self) -> broadcast::Receiver<EventToClient> {
        self.event_to_client_tx.subscribe()
    }
}
