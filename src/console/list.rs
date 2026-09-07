use std::sync::Arc;

use anyhow::Result;
use azalea_chat::FormattedText;
use serde::Deserialize;
use smaragdine::{brigadier::prelude::*, commands};
use tracing::info;

use crate::{console::Src, richtext};

pub(super) fn register(d: &mut CommandDispatcher<Src>) {
    // 注册 list
    commands!(d, {
        literal("list") => { run async: list; };
    });
}

async fn list(ctx: Arc<CommandContext<Src>>) -> Result<i32> {
    let state = ctx.source.state();

    let player_list = serde_json::to_value(richtext::list(state).await)?;

    let list_string = FormattedText::deserialize(&player_list)?.to_string();

    info!("Player(s) online:\n{}", list_string);

    Ok(1)
}
