use axum::extract::ws;
use futures::SinkExt;
use futures::stream::SplitSink;
use kameo::Actor;
use kameo::actor::ActorRef;
use kameo::message::StreamMessage;
use kameo::prelude::*;

#[derive(Actor)]
pub struct Relay {
    socket_sink: SplitSink<ws::WebSocket, ws::Message>,
}

impl Relay {
    pub fn spawn(socket_sink: SplitSink<ws::WebSocket, ws::Message>) -> ActorRef<Self> {
        kameo::spawn(Self { socket_sink })
    }
}

impl Message<StreamMessage<Result<ws::Message, axum::Error>, (), ()>> for Relay {
    type Reply = ();

    async fn handle(
        &mut self,
        msg: StreamMessage<Result<ws::Message, axum::Error>, (), ()>,
        ctx: &mut Context<Self, Self::Reply>,
    ) {
        let msg = match msg {
            StreamMessage::Next(Ok(msg)) => msg,
            StreamMessage::Next(Err(err)) => {
                tracing::error!("Got error: {err}");
                return;
            }
            StreamMessage::Started(_) => return,
            StreamMessage::Finished(_) => return,
        };
        let message_text;
        let request = match msg {
            ws::Message::Text(text) => {
                message_text = text;
                // TODO: handle notifications too
                let request: jsonrpsee_types::Request = match serde_json::from_str(&message_text) {
                    Ok(request) => request,
                    Err(err) => {
                        let _ = self
                            .socket_sink
                            .send(ws::Message::Text(format!("Invalid request: {err:?}!")))
                            .await;
                        return;
                    }
                };
                let _ = self
                    .socket_sink
                    .send(ws::Message::Text("Thanks for your message!".to_owned()))
                    .await;
                request
            }
            ws::Message::Binary(_) => {
                let _ = self
                    .socket_sink
                    .send(ws::Message::Text("Cannot handle binary".to_owned()))
                    .await;
                return;
            }
            ws::Message::Ping(ping) => {
                let _ = self.socket_sink.send(ws::Message::Pong(ping)).await;
                return;
            }
            ws::Message::Pong(_) => {
                // TODO: handle?
                return;
            }
            ws::Message::Close(_) => {
                let _ = ctx.actor_ref().stop_gracefully().await;
                return;
            }
        };

        match request.method.as_ref() {
            "Register" => {
                let _ = self
                    .socket_sink
                    .send(ws::Message::Text("Will register".to_owned()))
                    .await;
            }
            method => {
                let response = jsonrpsee_types::Response::<()>::new(
                    jsonrpsee_types::ResponsePayload::Error(jsonrpsee_types::ErrorObject::owned(
                        -32601,
                        format!("Unknown method {method:?}"),
                        Option::<usize>::None,
                    )),
                    request.id,
                );
                let _ = self
                    .socket_sink
                    .send(ws::Message::Text(serde_json::to_string(&response).unwrap()))
                    .await;
            }
        }
    }
}
