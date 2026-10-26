use std::time::{Duration, Instant};

use log::{info, warn};
use tokio::{
    sync::mpsc::{self, Receiver},
    time::interval,
};

use axum::{
    Json,
    body::Bytes,
    extract::{
        Query, State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
};
use let_me_play_models::{
    Event, NewRoomResponse, TryRoomRequest, TryRoomResponse, WsConnectRequest,
};
use uuid::Uuid;

use crate::{
    app_state::AppState, cooldown_s, error::LmpServerError, push_event_request::PushEventRequest,
    room::Room,
};

pub async fn new_room_handler(
    State(state): State<AppState>,
) -> Result<Json<NewRoomResponse>, LmpServerError> {
    let (sender, receiver) = mpsc::channel::<Event>(5);

    let id = Uuid::new_v4().to_string();

    let room = Room::new(sender, None);

    state.rooms().write().await.insert(id.clone(), room);
    state.receivers().write().await.insert(id.clone(), receiver);

    Ok(Json(NewRoomResponse::new(id)))
}

pub async fn try_room_handler(
    State(state): State<AppState>,
    Query(query): Query<TryRoomRequest>,
) -> Result<Json<TryRoomResponse>, LmpServerError> {
    let rooms = state.rooms().read().await;

    let exist = match rooms.get(query.id()) {
        Some(_) => true,
        None => false,
    };

    Ok(Json(TryRoomResponse::new(exist)))
}

pub async fn push_new_event_handler(
    State(state): State<AppState>,
    Json(request): Json<PushEventRequest>,
) -> Result<(), LmpServerError> {
    let sender = {
        let mut rooms = state.rooms().write().await;
        let room = rooms
            .get_mut(request.room_id())
            .ok_or(LmpServerError::InvalidRoom)?;

        if let Some(i) = room.last_sent()
            && Instant::now() - *i < Duration::from_secs(cooldown_s() as u64)
        {
            return Err(LmpServerError::Cooldown);
        }

        room.set_last_sent(Some(Instant::now()));
        room.sender().clone()
    };

    sender
        .send(request.event())
        .await
        .map_err(|_| LmpServerError::CouldNotSend)?;

    Ok(())
}

pub async fn ws_handler(
    upgrade: WebSocketUpgrade,
    State(state): State<AppState>,
    Query(query): Query<WsConnectRequest>,
) -> Result<impl IntoResponse, LmpServerError> {
    let receiver = state
        .receivers()
        .write()
        .await
        .remove(query.code())
        .ok_or(LmpServerError::InvalidRoom)?;

    Ok(upgrade.on_upgrade(move |socket| {
        socket_handler(socket, receiver, state.clone(), query.code().clone())
    }))
}

async fn socket_handler(
    mut websocket: WebSocket,
    mut receiver: Receiver<Event>,
    state: AppState,
    room_id: String,
) {
    let mut ping_interval = interval(Duration::from_secs(15));

    loop {
        tokio::select! {
            _ = ping_interval.tick() => {
                if let Err(e) = websocket.send(Message::Ping(Bytes::from(vec![]))).await {
                    warn!("Failed to send ping: {}", e);
                    break;
                }
            }

            Some(next) = receiver.recv() => {
                let Some(s) = serde_json::to_string(&next).ok() else {
                    continue;
                };

                if websocket.send(Message::text(s)).await.is_err() {
                    info!("WebSocket error in room {}, closing...", room_id);
                    break;
                }
            }

            msg = websocket.recv() => {
                match msg {
                    Some(Ok(Message::Close(_))) | Some(Err(_)) | None => {
                        info!("Room {} closed by host!", room_id);
                        break;
                    }
                    _ => {
                    }
                }
            }
        }
    }

    state.rooms().write().await.remove(&room_id);
}
