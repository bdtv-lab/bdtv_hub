use std::{sync::Arc, time::Duration};

use tokio::time;
use tokio_util::sync::CancellationToken;

use crate::{app, config::AppConfig};

pub async fn warden(config: AppConfig, state: Arc<app::State>, token: CancellationToken) {
    // 创建定时器
    let mut ticker = time::interval(Duration::from_secs(config.check.interval));
    // 第一次 tick 立即触发
    // 也就是跳过第一次的触发逻辑
    ticker.tick().await;

    loop {
        tokio::select! {
            _ = ticker.tick() => {
                check(&state, config.check.timeout).await;
            }

            _ = token.cancelled() => {
                break;
            }
        }
    }
}

async fn check(state: &app::State, timeout: u64) {
    state.check_player_timeouts(timeout).await;
    state.check_server_timeouts(timeout).await;
    state.report_player_count().await;
}
