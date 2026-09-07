use std::sync::Arc;

use kyori_component_json::{Color, Component, NamedColor};

use crate::{app, richtext::list::list};

/// 根据 State 构建 MOTD
pub async fn motd(state: Arc<app::State>) -> Vec<Component> {
    let mut motd = vec![
        Component::text("欢迎来到").color(Some(Color::Named(NamedColor::White))),
        Component::from(" "),
        Component::text("BDTV").color(Some(Color::Named(NamedColor::White))),
        // .decoration(TextDecoration::Bold, Some(true)),
        Component::from("\n\n"),
        Component::text("当前在线玩家:").color(Some(Color::Named(NamedColor::White))),
        Component::from("\n"),
    ];

    motd.extend(list(&state).await);

    motd
}
