mod viaws;

use tokio::sync::mpsc::Receiver;
use tokio_util::sync::CancellationToken;
use tracing::error;

use crate::{app::EventToQQ, envconf::Config, qq::viaws::WsReq};

pub async fn get_ws_client(config: Config) -> Option<WsReq> {
    if let Some(host) = config.qq_ws_host
        && let Some(port) = config.qq_ws_port
        && let Some(group_id) = config.qq_notice_group_id
    {
        let ws_client = WsReq::new(host, port, config.qq_ws_token, group_id).await;

        if let Err(e) = &ws_client {
            error!("can not connect to qq: {}", e)
        }

        ws_client.ok()
    } else {
        None
    }
}

pub async fn qq_connector(
    ws_client: Option<WsReq>,
    mut event_to_qq_rx: Receiver<EventToQQ>,
    token: CancellationToken,
) {
    // 如果 QQ 不可用，把 rx 发送到销毁器
    let Some(ws_client) = ws_client else {
        exhaust_event(event_to_qq_rx, token).await;
        return;
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
                // 通道已关闭，不会再有事件
                let Ok(event) = event else {
                    break;
                };

                ws_client.handle_event_from_qq(event);
            }
        }
    }
}

/// QQ 事件销毁器
async fn exhaust_event(mut event_to_qq_rx: Receiver<EventToQQ>, token: CancellationToken) {
    while let Some(_) = tokio::select! {
        _ = token.cancelled() => None,
        e = event_to_qq_rx.recv() => e,
    } {}
}
