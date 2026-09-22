use tokio_util::sync::CancellationToken;

pub async fn shutdown_signal(token: CancellationToken) {
    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("can not listen signal SIGTERM")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = tokio::signal::ctrl_c() => tracing::info!("received ctrl+c"),
        _ = terminate => tracing::info!("received SIGTERM"),
        _ = token.cancelled() => return,
    }

    token.cancel();
}
