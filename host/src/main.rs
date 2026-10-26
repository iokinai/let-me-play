use std::error::Error;

use enigo::{Enigo, Settings};
use futures_util::{SinkExt, StreamExt};
use let_me_play_models::{Event, NewRoomResponse};
use log::{error, info};
use reqwest::Client;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Bytes, Message},
};

use crate::{
    error::{LmpError, ServerError},
    handler::Handler,
};

mod error;
mod handler;
mod mappings;
mod requests;

#[cfg(debug_assertions)]
fn server_url(proto: &str) -> String {
    format!("{proto}://127.0.0.1:8080")
}

#[cfg(not(debug_assertions))]
fn server_url(proto: &str) -> String {
    format!("{proto}s://176.222.55.14:8090")
}

async fn run() -> Result<(), LmpError> {
    let handler = Handler::new(Enigo::new(&Settings::default()).unwrap(), 500);
    let http_server_url = server_url("http");
    let ws_url = server_url("ws");

    let client = Client::new();
    let response = client
        .post(format!("{}/api/rooms", http_server_url))
        .send()
        .await
        .inspect_err(|e| error!("Error while creating room: {}", e))
        .map_err(|_| LmpError::ServerError(ServerError::CouldNotCreateRoom))?
        .json::<NewRoomResponse>()
        .await
        .inspect_err(|e| error!("Error parsing create room response: {}", e))
        .map_err(|_| LmpError::ServerError(ServerError::CouldNotCreateRoom))?;

    info!("Created room {}", response.id());

    let (ws_stream, _) = connect_async(format!("{}/ws?code={}", ws_url, response.id()))
        .await
        .inspect_err(|e| {
            error!("WebSocket connection error: {}", e);

            if let tokio_tungstenite::tungstenite::Error::Http(he) = e {
                error!(
                    "Http error: {}",
                    String::from_utf8(he.body().clone().unwrap()).unwrap()
                );
            }
        })
        .map_err(|_| LmpError::ServerError(ServerError::WebSocketConnectError))?;

    let (mut write, mut read) = ws_stream.split();

    while let Some(msg_result) = read.next().await {
        let msg = match msg_result {
            Ok(m) => m,
            Err(e) => {
                error!("Message read error: {}", e);
                continue;
            }
        };

        match msg {
            Message::Text(text) => {
                let event = match serde_json::from_str::<Event>(&text) {
                    Ok(ev) => ev,
                    Err(e) => {
                        error!("Parse event error: {}", e);
                        continue;
                    }
                };

                if let Err(e) = handler.handle_event(&event).await {
                    error!("Error handling event: {}", e)
                }
            }
            Message::Ping(_) => {
                if let Err(e) = write.send(Message::Pong(Bytes::from(vec![]))).await {
                    error!("Error sending PONG: {}", e)
                }
            }
            _ => {}
        }
    }

    Ok(())
}

fn exit_with_error(e: LmpError) -> ! {
    println!("\x1b[31m[FATAL ERROR]\x1b[0m {}", e.to_string());

    std::process::exit(-1)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_target(false).init();

    match run().await {
        Ok(_) => std::process::exit(0),
        Err(e) => exit_with_error(e),
    }
}
