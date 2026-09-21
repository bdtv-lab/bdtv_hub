mod heartbeat;

use std::{collections::HashMap, sync::atomic::AtomicUsize};

use smaragdine::Printer;
use tokio::{
    sync::{Mutex, broadcast, mpsc},
    time::Instant,
};
use uuid::Uuid;

use crate::{
    app::event::{QQEvent, ServerEvent},
    types::{Player, Server},
};

type OnlineServers = HashMap<Server, Instant>;
type OnlinePlayers = HashMap<Uuid, (Player, Instant)>;

#[derive(Debug)]
pub struct State {
    pub online_players: Mutex<HashMap<Server, OnlinePlayers>>,
    pub online_servers: Mutex<OnlineServers>,
    pub printer: Printer,

    pub qq_event_tx: mpsc::Sender<QQEvent>,
    pub server_event_tx: broadcast::Sender<ServerEvent>,
    /// 上次上报的去重在线人数
    last_reported_count: AtomicUsize,
}

impl State {
    pub fn new(qq_tx: mpsc::Sender<QQEvent>) -> Self {
        Self {
            online_players: Mutex::new(HashMap::new()),
            online_servers: Mutex::new(HashMap::new()),
            printer: Printer::new(),
            qq_event_tx: qq_tx,
            server_event_tx: broadcast::Sender::new(100),
            last_reported_count: AtomicUsize::new(0),
        }
    }
}
