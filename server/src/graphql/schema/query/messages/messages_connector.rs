use super::Message;
use crate::graphql::schema::connection::{connection, DatabaseConnector};
use crate::graphql::schema::Context;
use chrono::{DateTime, Utc};
use juniper::FieldResult;
use sea_orm::prelude::*;

pub(crate) struct MessagesConnector<'a> {
    context: &'a Context,
}

impl<'a> MessagesConnector<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self { context }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
enum QueryAs {
    Count,
}

#[async_trait::async_trait]
impl<'a> DatabaseConnector for MessagesConnector<'a> {
    type Node = Message<'a>;
    type Entity = entity::messages::Entity;
    const ORDER_COLUMN: entity::messages::Column = entity::messages::Column::CreatedAt;
    const COUNT_COLUMN: entity::messages::Column = entity::messages::Column::Id;

    fn database(&self) -> &DatabaseConnection {
        self.context.db()
    }

    fn parse_cursor(&self, cursor: &str) -> FieldResult<sea_orm::Value> {
        Ok(serde_json::from_str::<DateTime<Utc>>(cursor)?.into())
    }
}

connection!(impl<'a> for MessagesConnector<'a> as "MessagesConnection");
