use crate::graphql::schema::connection::{connection_edge, ConnectionNode};
use crate::graphql::schema::Context;
use chrono::{DateTime, FixedOffset};
use std::borrow::Cow;
use uuid::Uuid;

mod users_connector;
pub(crate) use users_connector::UsersConnector;

pub(crate) struct User<'a>(pub(super) Cow<'a, entity::users::Model>);

impl From<entity::users::Model> for User<'_> {
    fn from(message: entity::users::Model) -> Self {
        Self(Cow::Owned(message))
    }
}

impl<'a> From<&'a entity::users::Model> for User<'a> {
    fn from(message: &'a entity::users::Model) -> Self {
        Self(Cow::Borrowed(message))
    }
}

#[juniper::graphql_object(context = Context)]
impl<'a> User<'a> {
    fn id(&self) -> Uuid {
        self.0.id
    }

    fn display_name(&self) -> &str {
        &self.0.display_name
    }

    fn full_name(&self) -> Option<&str> {
        self.0.full_name.as_deref()
    }

    fn created_at(&self) -> DateTime<FixedOffset> {
        self.0.created_at
    }
}

impl ConnectionNode for User<'_> {
    fn cursor(&self) -> String {
        serde_json::to_string(&self.0.created_at).unwrap()
    }
}

connection_edge!(impl<'a> for User<'a> as "UserEdge");
