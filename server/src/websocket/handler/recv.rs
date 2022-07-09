use super::SocketHandler;
use actix::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Message)]
#[rtype(result = "()")]
#[serde(transparent)]
pub struct Recv(Payload);

#[derive(Deserialize)]
enum Payload {
    Reply { reply_to: Uuid, content: String },
}

impl Handler<Recv> for SocketHandler {
    type Result = ();

    fn handle(&mut self, Recv(_message): Recv, _ctx: &mut Self::Context) {}
}
