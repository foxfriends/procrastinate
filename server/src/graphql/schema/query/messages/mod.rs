use super::User;
use crate::graphql::schema::connection::{connection_edge, ConnectionNode};
use crate::graphql::schema::Context;
use juniper::FieldResult;
use sea_orm::prelude::*;
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
impl<'a> Message<'a> {
    fn id(&self) -> Uuid {
        self.0.id
    }

    async fn author(&self, context: &Context) -> FieldResult<User<'static>> {
        let user = entity::users::Entity::find_by_id(self.0.author_id)
            .one(context.db())
            .await?
            .unwrap();
        Ok(User::from(user))
    }
}

impl ConnectionNode for Message<'_> {
    fn cursor(&self) -> String {
        serde_json::to_string(&self.0.created_at).unwrap()
    }
}

connection_edge!(impl<'a> for Message<'a> as "MessageEdge");
