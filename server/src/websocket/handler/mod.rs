use actix::prelude::*;
use actix_web_actors::ws;

mod recv;
mod send;

use recv::Recv;
use send::Send;

pub struct SocketHandler;

impl SocketHandler {
    pub fn new() -> Self {
        Self
    }
}

impl Actor for SocketHandler {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for SocketHandler {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => match ron::from_str::<Recv>(&text) {
                Ok(event) => ctx.notify(event),
                Err(error) => ctx.notify(Send::error(
                    "BadRequest",
                    error.to_string(),
                    text.to_string(),
                )),
            },
            _ => {}
        }
    }
}
