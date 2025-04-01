use axum::extract::ws;
use futures::StreamExt;
use kameo::actor::ActorRef;
use kameo::{Actor, messages};

use super::relay::Relay;

#[derive(Actor)]
pub struct MainActor {}

pub type MainActorRef = ActorRef<MainActor>;

impl MainActor {
    pub fn spawn() -> MainActorRef {
        kameo::spawn(Self {})
    }
}

#[messages]
impl MainActor {
    #[message]
    pub fn start_relay(&self, socket: ws::WebSocket) {
        let (mut sink, mut stream) = socket.split();

        let relay = Relay::spawn(sink);

        relay.attach_stream(stream, (), ());
    }
}
