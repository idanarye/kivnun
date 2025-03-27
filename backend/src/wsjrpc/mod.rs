use axum;
use axum::extract::ws;

pub async fn upgrade_websocket(socket: ws::WebSocketUpgrade) -> axum::response::Response {
    socket.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: ws::WebSocket) {
    match socket.send(ws::Message::Text("Welcome!".to_owned())).await {
        Ok(()) => tracing::info!("Send was successful"),
        Err(err) => tracing::error!("Send fail: {err:?}"),
    }
    match socket.recv().await {
        Some(Ok(data)) => tracing::info!("Got {data:?} from socket"),
        Some(Err(err)) => tracing::error!("Recv failed: {err:?}"),
        None => tracing::info!("Socket was closed"),
    }
}
