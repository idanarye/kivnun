mod main_actor;
mod relay;

use axum::extract::{State, ws};

pub use main_actor::{MainActor, MainActorRef};

use self::main_actor::StartRelay;

pub async fn upgrade_websocket(
    state: State<MainActorRef>,
    socket: ws::WebSocketUpgrade,
) -> axum::response::Response {
    socket.on_upgrade(async move |socket| {
        if let Err(err) = state.0.ask(StartRelay { socket }).await {
            tracing::error!("Cannot start WebSocket relay: {err}");
        }
        // match socket.send(ws::Message::Text(format!("Welcome! You are {num:?}"))).await {
        // Ok(()) => tracing::info!("Send was successful"),
        // Err(err) => tracing::error!("Send fail: {err:?}"),
        // }
        // match socket.recv().await {
        // Some(Ok(data)) => tracing::info!("Got {data:?} from socket"),
        // Some(Err(err)) => tracing::error!("Recv failed: {err:?}"),
        // None => tracing::info!("Socket was closed"),
        // }
    })
}
