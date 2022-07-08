use procrastinate::{Id, Message};

use super::SocketHandler;
use actix::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Message)]
#[rtype(result = "()")]
#[serde(transparent)]
pub struct Recv(Payload);

#[derive(Deserialize)]
enum Payload {
    Reply { reply_to: Id, content: Message },
}

impl Handler<Recv> for SocketHandler {
    type Result = ();

    fn handle(&mut self, Recv(message): Recv, ctx: &mut Self::Context) {}
}
