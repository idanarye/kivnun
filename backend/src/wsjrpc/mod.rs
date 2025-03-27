use axum;
use axum::extract::ws;

pub async fn upgrade_websocket(socket: ws::WebSocketUpgrade) -> axum::response::Response {
    socket.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: ws::WebSocket) {
    socket.send(ws::Message::Text("Welcome!".to_owned())).await.unwrap();
    socket.close().await.unwrap();
}
