mod viaws;

use tokio::sync::mpsc::Receiver;
use tokio_util::sync::CancellationToken;

use crate::{app::EventToQQ, envconf::Config, qq::viaws::WsReq};

pub async fn get_ws_client(config: Config) -> Option<WsReq> {
    if let Some(host) = config.qq_ws_host
        && let Some(port) = config.qq_ws_port
        && let Some(group_id) = config.qq_notice_group_id
    {
        WsReq::new(host, port, config.qq_ws_token, group_id)
            .await
            .ok()
    } else {
        None
    }
}

pub async fn qq_connector(
    ws_client: WsReq,
    mut event_to_qq_rx: Receiver<EventToQQ>,
    token: CancellationToken,
) {
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
