use std::{collections::HashMap, sync::Arc};

use axum::{
    Router,
    http::header,
    routing::{any, get, post},
};
use tokio::{net::TcpListener, sync::RwLock};
use tower_http::cors::CorsLayer;

use crate::{
    app_state::AppState,
    environment::Environment,
    handlers::{new_room_handler, push_new_event_handler, try_room_handler, ws_handler},
};

mod app_state;
mod environment;
mod error;
mod handlers;
mod push_event_request;
mod room;

pub fn cooldown_s() -> u32 {
    15
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_target(false).init();
    let env = Environment::parse().unwrap();

    let state = AppState::new(
        Arc::new(RwLock::new(HashMap::new())),
        Arc::new(RwLock::new(HashMap::new())),
    );

    let cors = CorsLayer::new()
        .allow_origin(
            env.frontend_url()
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
        )
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
        .allow_headers([
            header::CONTENT_TYPE,
            header::UPGRADE,
            header::CONNECTION,
            header::SEC_WEBSOCKET_KEY,
            header::SEC_WEBSOCKET_VERSION,
            header::SEC_WEBSOCKET_PROTOCOL,
        ])
        .allow_credentials(true);

    let api_rounter = Router::new()
        .route("/rooms", post(new_room_handler))
        .route("/room", get(try_room_handler))
        .route("/room", post(push_new_event_handler));

    let app = Router::new()
        .nest("/api", api_rounter)
        .route("/ws", any(ws_handler))
        .with_state(state)
        .layer(cors);

    let listener = TcpListener::bind(env.backend_addr()).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
