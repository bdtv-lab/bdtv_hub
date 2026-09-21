mod heartbeat;

use std::{collections::HashMap, sync::atomic::AtomicUsize};

use smaragdine::Printer;
use tokio::{
    sync::{Mutex, broadcast, mpsc},
    time::Instant,
};
use uuid::Uuid;

use crate::{
    app::event::{EventToClient, EventToQQ},
    types::{Player, Server},
};

type OnlineServers = HashMap<Server, Instant>;
type OnlinePlayers = HashMap<Uuid, (Player, Instant)>;

#[derive(Debug)]
pub struct State {
    pub online_players: Mutex<HashMap<Server, OnlinePlayers>>,
    pub online_servers: Mutex<OnlineServers>,
    pub printer: Printer,

    pub event_to_qq_tx: mpsc::Sender<EventToQQ>,
    pub event_to_client_tx: broadcast::Sender<EventToClient>,
    /// 上次上报的去重在线人数
    last_reported_count: AtomicUsize,
}

impl State {
    pub fn new(event_to_qq_tx: mpsc::Sender<EventToQQ>) -> Self {
        Self {
            online_players: Mutex::new(HashMap::new()),
            online_servers: Mutex::new(HashMap::new()),
            printer: Printer::new(),
            event_to_qq_tx,
            event_to_client_tx: broadcast::Sender::new(100),
            last_reported_count: AtomicUsize::new(0),
        }
    }
}
