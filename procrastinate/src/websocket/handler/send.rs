use super::SocketHandler;
use crate::{Id, Message};
use actix::prelude::*;
use ron::Value;
use serde::Serialize;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Send(Payload);

#[derive(Serialize)]
enum Payload {
    Reply {
        reply_to: Id,
        content: Message,
    },
    Error {
        code: &'static str,
        message: String,
        context: Value,
    },
}

impl Send {
    pub fn error<M, T>(code: &'static str, message: M, context: T) -> Self
    where
        T: Serialize,
        M: std::fmt::Display,
    {
        let context = ron::to_string(&context).unwrap();
        let context = ron::from_str(&context).unwrap();
        Send(Payload::Error {
            code,
            message: message.to_string(),
            context,
        })
    }
}

impl Handler<Send> for SocketHandler {
    type Result = ();

    fn handle(&mut self, Send(message): Send, ctx: &mut Self::Context) {
        ctx.text(ron::to_string(&message).unwrap());
    }
}
