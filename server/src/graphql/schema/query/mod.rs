use super::connection::{ConnectionResult, Connector, Cursor};
use super::Context;
use juniper::FieldResult;
use sea_orm::prelude::*;
use uuid::Uuid;

mod messages;
use messages::{Message, MessagesConnector};

pub(crate) struct Query;

#[juniper::graphql_object(context = Context)]
impl Query {
    /// Currently running version of Battlefield server
    fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    /// Lists messages
    async fn messages_connection<'a>(
        &'a self,
        context: &'a Context,
        thread: Option<Uuid>,
        first: Option<i32>,
        after: Option<Cursor>,
        last: Option<i32>,
        before: Option<Cursor>,
    ) -> FieldResult<ConnectionResult<MessagesConnector<'a>>> {
        MessagesConnector::new(context)
            .get(first.map(Into::into), after, last.map(Into::into), before)
            .await
    }

    /// Gets a specific message
    async fn message<'a>(
        &'a self,
        context: &'a Context,
        id: Uuid,
    ) -> FieldResult<Option<Message<'a>>> {
        let message = entity::messages::Entity::find_by_id(id)
            .one(context.db())
            .await?;
        Ok(message.map(Message::from))
    }
}