use std::sync::Arc;

use anyhow::Result;
use smaragdine::{brigadier::prelude::*, commands};

use crate::{
    app::{EventToClient, event::Audience},
    console::Src,
};

pub(super) fn register(d: &mut CommandDispatcher<Src>) {
    // 注册 say
    commands!(d, {
        literal("say") => {
            greedy_string("msg") => { run async: say; };
        };
    });
}

async fn say(ctx: Arc<CommandContext<Src>>) -> Result<i32> {
    let state = ctx.source.state();

    let msg = get_string(&ctx, "msg").unwrap_or_default();
    state.send_event_to_client(EventToClient::ConsoleSentMsg(msg.clone()), Audience::All);

    Ok(1)
}
