mod viaws;

use tokio::sync::mpsc::Receiver;
use tokio_util::sync::CancellationToken;

use crate::{app, envconf::Config, qq::viaws::WsReq};

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
    mut event_rx: Receiver<app::Event>,
    token: CancellationToken,
) {
    let mut ws_rx = ws_client.conn.subscribe().await;

    loop {
        tokio::select! {
            _ = token.cancelled() => {
                break;
            }

            // 服务器事件
            event = event_rx.recv() => {
                // 通道已关闭，不会再有事件
                let Some(event) = event else {
                    break;
                };

                ws_client.handle_server_event(event).await;
            }

            // qq 事件
            event = ws_rx.recv() => {
                // 通道已关闭，不会再有事件
                let Ok(event) = event else {
                    break;
                };

                ws_client.handle_qq_event(event);
            }
        }
    }
}
