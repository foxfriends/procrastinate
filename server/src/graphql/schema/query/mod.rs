use super::connection::{ConnectionResult, Connector, Cursor};
use super::Context;
use juniper::FieldResult;
use sea_orm::prelude::*;
use uuid::Uuid;

mod messages;
use messages::{Message, MessagesConnector};

mod users;
use users::{User, UsersConnector};

pub(crate) struct Query;

#[juniper::graphql_object(context = Context)]
impl Query {
    /// Currently server version
    fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    /// Lists messages
    async fn messages_connection<'a>(
        &'a self,
        context: &'a Context,
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
    async fn message(&self, context: &Context, id: Uuid) -> FieldResult<Option<Message<'static>>> {
        let message = entity::messages::Entity::find_by_id(id)
            .one(context.db())
            .await?;
        Ok(message.map(Message::from))
    }

    /// Lists users
    async fn users_connection<'a>(
        &'a self,
        context: &'a Context,
        first: Option<i32>,
        after: Option<Cursor>,
        last: Option<i32>,
        before: Option<Cursor>,
    ) -> FieldResult<ConnectionResult<UsersConnector<'a>>> {
        UsersConnector::new(context)
            .get(first.map(Into::into), after, last.map(Into::into), before)
            .await
    }

    /// Gets a specific user
    async fn user(&self, context: &Context, id: Uuid) -> FieldResult<Option<User<'static>>> {
        let user = entity::users::Entity::find_by_id(id)
            .one(context.db())
            .await?;
        Ok(user.map(User::from))
    }
}
