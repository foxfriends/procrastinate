use super::Message;
use crate::graphql::schema::connection::{connection, Connection, Connector, Cursor};
use crate::graphql::schema::Context;
use juniper::FieldResult;

pub(crate) struct MessagesConnector<'a> {
    context: &'a Context,
}

impl<'a> MessagesConnector<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self { context }
    }
}

#[async_trait::async_trait]
impl<'a> Connector for MessagesConnector<'a> {
    type Node = Message<'a>;

    async fn len(&self) -> FieldResult<usize> {
        todo!()
    }

    async fn first(&self, count: usize, after: Cursor) -> FieldResult<Connection<Self::Node>> {
        todo!()
    }

    async fn last(&self, count: usize, before: Cursor) -> FieldResult<Connection<Self::Node>> {
        todo!()
    }
}

connection!(impl<'a> for MessagesConnector<'a> as "MessagesConnection");
