use std::{collections::HashMap, sync::Arc};

use kyori_component_json::{ClickEvent, Color, Component, HoverEvent, NamedColor, UuidRepr};

use crate::app;

/// 根据 State 构建 玩家列表
pub async fn list(state: &Arc<app::State>) -> Vec<Component> {
    let mut list = Vec::new();

    let online_players = state.online_players.lock().await;

    // 同一玩家可能同时存在于多个服务器
    // 只保留心跳最新的那一条
    let mut latest = HashMap::new();
    // 遍历服务器
    for (server, players) in online_players.iter() {
        // 遍历玩家
        for (uuid, (player, last_heartbeat)) in players.iter() {
            let entry = latest
                .entry(uuid)
                .or_insert((server, player, last_heartbeat));
            // 如果心跳更新，则替换之前的
            if last_heartbeat > entry.2 {
                *entry = (server, player, last_heartbeat);
            }
        }
    }

    // 按去重后的归属服务器重新分组
    let mut grouped = HashMap::new();
    for (server, player, _) in latest.into_values() {
        grouped.entry(server).or_insert_with(Vec::new).push(player);
    }

    // 给服务器排个序
    let mut servers: Vec<_> = grouped.into_iter().collect();
    servers.sort_by(|(a, _), (b, _)| a.slug.cmp(&b.slug));

    // 采用索引计数以绘制分隔符
    for (index, (server, mut server_players)) in servers.into_iter().enumerate() {
        if index > 0 {
            list.push(Component::from("\n"));
        }

        // 服务器名称与命令建议
        list.push(
            Component::text(&server.nickname)
                .color(Some(Color::Named(NamedColor::Green)))
                .click_event(Some(ClickEvent::SuggestCommand {
                    command: format!("!!goto {}", server.slug),
                }))
                .hover_event(Some(HoverEvent::ShowText {
                    value: Component::text("点击前往"),
                })),
        );
        // 服务器名称与玩家之间的空格
        list.push(Component::from(" "));

        // 给玩家也排序
        server_players.sort_by_key(|player| player.nickname.to_lowercase());

        for (index, player) in server_players.into_iter().enumerate() {
            if index > 0 {
                list.push(Component::text(", ").color(Some(Color::Named(NamedColor::White))));
            }
            // 玩家名称与实体显示 hover
            list.push(
                Component::text(&player.nickname)
                    .color(Some(Color::Named(NamedColor::Yellow)))
                    .hover_event(Some(HoverEvent::ShowEntity {
                        name: Some(Component::text(&player.nickname)),
                        id: "minecraft:player".to_string(),
                        uuid: UuidRepr::String(player.uuid.to_string()),
                    })),
            );
        }
    }

    list
}
