use crate::graphql::schema::connection::{connection_edge, ConnectionNode, Cursor};
use crate::graphql::schema::Context;
use std::borrow::Cow;
use uuid::Uuid;

mod messages_connector;
pub(crate) use messages_connector::MessagesConnector;

pub(crate) struct Message<'a>(pub(super) Cow<'a, entity::messages::Model>);

impl From<entity::messages::Model> for Message<'_> {
    fn from(message: entity::messages::Model) -> Self {
        Self(Cow::Owned(message))
    }
}

impl<'a> From<&'a entity::messages::Model> for Message<'a> {
    fn from(message: &'a entity::messages::Model) -> Self {
        Self(Cow::Borrowed(message))
    }
}

#[juniper::graphql_object(context = Context)]
impl Message<'_> {
    fn id(&self) -> Uuid {
        self.0.id
    }
}

impl ConnectionNode for Message<'_> {
    fn cursor(&self) -> Cursor {
        Cursor::Node(self.0.id.to_string())
    }
}

connection_edge!(impl<'a> for Message<'a> as "MessageEdge");
