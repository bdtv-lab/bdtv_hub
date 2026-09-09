use std::{collections::HashMap, sync::Arc};

use axum::{Json, extract::State};

use crate::{app, types::Server};

pub(super) async fn get_servers(
    State(state): State<Arc<app::State>>,
) -> Json<HashMap<String, Server>> {
    let server_map = state.online_servers.lock().await;
    let servers = server_map
        .keys()
        .map(|server| (server.slug.clone(), server.clone()))
        .collect();
    Json(servers)
}
