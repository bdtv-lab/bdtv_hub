use std::{
    collections::{HashMap, HashSet},
    sync::atomic::Ordering,
};

use tokio::time::Instant;
use tracing::{debug, info};

use uuid::Uuid;

use crate::{
    app::{QQEvent, State},
    types::{Player, Server},
};

impl State {
    /// 统计所有服务器内不重复的在线玩家数量
    async fn unique_player_count(&self) -> usize {
        self.online_players
            .lock()
            .await
            .values()
            .flat_map(|players| players.keys())
            .collect::<HashSet<_>>()
            .len()
    }

    /// 上报去重后的在线人数
    pub async fn report_player_count(&self) {
        let count = self.unique_player_count().await;

        // 与上次上报的值相同则不重复发送
        if self.last_reported_count.swap(count, Ordering::Relaxed) == count {
            return;
        }

        debug!("Unique player count changed to {count}");
        let _ = self.qq_event_tx.send(QQEvent::PlayerCountChanged(count)).await;
    }

    /// 标记玩家为在线状态
    pub async fn mark_player_as_online(&self, server: &Server, player: &Player) {
        let (is_new, is_unique_new) = {
            // 获取在线玩家的可变引用
            let mut online_players = self.online_players.lock().await;

            // 检查所在服务器是否不包含该玩家
            let is_new = !online_players
                .entry(server.clone())
                .or_default()
                .contains_key(&player.uuid);

            // 检查是否任何服务器内都不包含该玩家
            let is_unique_new = !online_players
                .values()
                .any(|players| players.contains_key(&player.uuid));

            // 更新玩家的心跳时间
            online_players
                .entry(server.clone())
                .or_insert_with(HashMap::new)
                .insert(player.uuid, (player.clone(), Instant::now()));

            (is_new, is_unique_new)
        };

        // 如果是所有服务器中的新玩家则发送事件
        if is_unique_new {
            info!("Player {} joined server {}", player.nickname, server.slug);
            let _ = self
                .qq_event_tx
                .send(QQEvent::PlayerJoined(player.clone()))
                .await;
        }
        // 否则检查是否是已有玩家加入服务器
        else if is_new {
            info!(
                "Player {} joined another server {}",
                player.nickname, server.slug
            );
        }
    }

    /// 检查玩家是否超时
    ///
    /// 玩家可能同时在多个服务器上
    ///
    /// 只有从所有服务器上都消失才算离开
    pub async fn check_player_timeouts(&self, timeout: u64) {
        // 限制锁的作用域
        let left_players = {
            let mut online_players = self.online_players.lock().await;
            let now = Instant::now();

            // 剔除所有服务器上的超时玩家
            // 同一名玩家可能在多个服务器上超时
            let mut timed_out: HashMap<Uuid, (Server, Player)> = HashMap::new();
            for (server, players) in online_players.iter_mut() {
                players.retain(|uuid, (player, last_heartbeat)| {
                    let alive = now.duration_since(*last_heartbeat).as_secs() <= timeout;
                    if !alive {
                        timed_out.insert(*uuid, (server.clone(), player.clone()));
                    }
                    alive
                });
            }

            // 仍然存在于其他服务器的玩家为切换服务器的玩家
            let mut left_players = Vec::new();
            for (uuid, (server, player)) in timed_out {
                let still_online = online_players
                    .values()
                    .any(|players| players.contains_key(&uuid));

                if still_online {
                    info!("Player {} left server {}", player.nickname, server.slug);
                } else {
                    left_players.push((server, player));
                }
            }

            left_players
        };

        // 发送玩家离开的事件
        for (server, player) in left_players {
            info!(
                "Player {} left server {} as last",
                player.nickname, server.slug
            );
            let _ = self.qq_event_tx.send(QQEvent::PlayerLeft(player)).await;
        }
    }

    pub async fn mark_server_as_online(&self, server: &Server) {
        let mut online_servers = self.online_servers.lock().await;
        let is_new = online_servers
            .insert(server.clone(), Instant::now())
            .is_none();

        if is_new {
            info!("Server {} is online", server.slug);
        }
    }

    pub async fn check_server_timeouts(&self, timeout: u64) {
        let mut timeout_servers = Vec::new();

        let mut online_servers = self.online_servers.lock().await;
        let now = Instant::now();

        for (server, last_heartbeat) in online_servers.iter() {
            if now.duration_since(*last_heartbeat).as_secs() > timeout {
                timeout_servers.push(server.clone());
            }
        }

        for server in timeout_servers {
            online_servers.remove(&server);
            info!("Server {} is offline", server.slug);
        }
    }
}
