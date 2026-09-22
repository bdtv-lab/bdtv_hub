mod viaws;

use std::sync::Arc;

use tokio::sync::{broadcast::error::RecvError, mpsc::Receiver};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

use crate::{
    app::{EventToQQ, State},
    config::QqConfig,
    qq::viaws::WsReq,
};

pub async fn qq_connector(
    qq: QqConfig,
    state: Arc<State>,
    mut event_to_qq_rx: Receiver<EventToQQ>,
    token: CancellationToken,
) {
    info!("waiting for QQ ws connection to {}:{}", qq.host, qq.port);
    let ws_client = tokio::select! {
        _ = token.cancelled() => return,
        c = WsReq::new(state, qq) => match c {
            Ok(c) => c,
            Err(e) => { error!("can not connect to qq: {e}"); return; }
        },
    };

    let mut event_from_qq_rx = ws_client.conn.subscribe().await;

    loop {
        tokio::select! {
            _ = token.cancelled() => {
                break;
            }

            // 服务器发往 qq 的事件
            event = event_to_qq_rx.recv() => {
                // 通道已关闭，不会再有事件
                let Some(event) = event else {
                    break;
                };

                ws_client.handle_event_to_qq(event).await;
            }

            // 来自 qq 的事件
            event = event_from_qq_rx.recv() => {
                match event {
                    Ok(event) => ws_client.handle_event_from_qq(event),
                    // 通道已关闭，不会再有事件
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(n)) => warn!("QQ ws connection lagged, dropped {n} events"),
                }
            }
        }
    }
}
