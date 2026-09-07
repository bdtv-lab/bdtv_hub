mod list;

use std::sync::Arc;

use anyhow::Result;
use smaragdine::{AsyncConsole, Exit, Source, brigadier::prelude::*};
use tokio::runtime::Handle;
use tokio_util::sync::CancellationToken;
use tracing::error;

use crate::app;

type Src = Source<Arc<app::State>>;

pub async fn console(state: Arc<app::State>, token: CancellationToken) {
    let console = AsyncConsole::builder(Handle::current())
        .printer(state.printer.clone())
        .prompt("/ ")
        .multiline_prompt("| ")
        .completion_prompt("? ")
        // 以 Minecraft 风格注册命令
        .command(
            literal("ping").executes(|ctx: &CommandContext<Src>| -> Result<i32> {
                ctx.source.printer().print("pong!");
                Ok(1)
            }),
        )
        .commands(list::register)
        .build(state);

    let source = console.source();
    let mut task = tokio::task::spawn_blocking(move || console.run());

    tokio::select! {
        result = &mut task => match result {
            // 正常退出
            Ok(Exit::Quit(()) | Exit::Interrupted) => token.cancel(),
            // 无可用终端
            Ok(Exit::NoTerminal) => {}
            // 一般错误
            Ok(Exit::Failed(error)) => error!("console error: {error}"),
            // 致命错误
            Err(error) => {
                error!("console task failed: {error}");
                token.cancel();
            }
        },
        _ = token.cancelled() => {
            source.request_quit();
            if let Err(error) = task.await {
                error!("console task failed: {error}");
            }
        }
    }
}
