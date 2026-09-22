mod goto;
mod motd;
mod ws;

use std::sync::Arc;

use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use crate::{app, config::AppConfig};

/// 启动 HTTP 服务器
///
/// 这个 HTTP 服务器会接受来自 MCDR 插件发出的请求，并给予相应回复
///
/// MCDR 有能力通过这个中心服务器获取整个服务器集群的状态
pub async fn http_server(config: AppConfig, state: Arc<app::State>, token: CancellationToken) {
    let app = Router::new()
        .route("/motd", get(motd::get_motd))
        .route("/servers", get(goto::get_servers))
        .route("/ws", get(ws::ws_connect))
        .with_state(state);

    // 绑定 TCP 监听器
    let listener = match TcpListener::bind(&config.listen).await {
        Ok(l) => l,
        Err(e) => {
            error!("can not bind to {}: {}", config.listen, e);
            token.cancel();
            return;
        }
    };

    info!("listening on http://{}", listener.local_addr().unwrap());

    // 启动 HTTP 服务器
    if let Err(e) = axum::serve(listener, app)
        .with_graceful_shutdown(token.clone().cancelled_owned())
        .await
    {
        error!("server error: {e}");
        token.cancel();
    }
}
