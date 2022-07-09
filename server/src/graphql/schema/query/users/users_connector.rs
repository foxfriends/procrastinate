use super::User;
use crate::graphql::schema::connection::{connection, DatabaseConnector};
use crate::graphql::schema::Context;
use chrono::{DateTime, Utc};
use juniper::FieldResult;
use sea_orm::prelude::*;

pub(crate) struct UsersConnector<'a> {
    context: &'a Context,
}

impl<'a> UsersConnector<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self { context }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
enum QueryAs {
    Count,
}

#[async_trait::async_trait]
impl<'a> DatabaseConnector for UsersConnector<'a> {
    type Node = User<'a>;
    type Entity = entity::users::Entity;
    const ORDER_COLUMN: entity::users::Column = entity::users::Column::CreatedAt;
    const COUNT_COLUMN: entity::users::Column = entity::users::Column::Id;

    fn database(&self) -> &DatabaseConnection {
        self.context.db()
    }

    fn parse_cursor(&self, cursor: &str) -> FieldResult<sea_orm::Value> {
        Ok(serde_json::from_str::<DateTime<Utc>>(cursor)?.into())
    }
}

connection!(impl<'a> for UsersConnector<'a> as "UsersConnection");
