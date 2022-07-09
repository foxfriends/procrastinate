use super::Message;
use crate::graphql::schema::connection::{
    connection, Connection, Connector, Cursor, Edge, PageInfo,
};
use crate::graphql::schema::Context;
use chrono::{DateTime, Utc};
use juniper::FieldResult;
use sea_orm::prelude::*;
use sea_orm::{Order, QueryOrder, QuerySelect};

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
impl<'a> Connector for MessagesConnector<'a> {
    type Node = Message<'a>;

    async fn len(&self) -> FieldResult<u64> {
        Ok(entity::messages::Entity::find()
            .select_only()
            .column_as(entity::messages::Column::Id.count(), QueryAs::Count)
            .into_values::<_, QueryAs>()
            .one(self.context.db())
            .await?
            .unwrap_or(0u64))
    }

    async fn first(&self, count: u64, after: Cursor) -> FieldResult<Connection<Self::Node>> {
        let entities = entity::messages::Entity::find();

        let has_previous_page = match &after {
            Cursor::Start => false,
            Cursor::End => self.len().await? > 0,
            Cursor::Node(..) => true,
        };

        let entities = match &after {
            Cursor::End => {
                return Ok(Connection::empty_end(has_previous_page));
            }
            Cursor::Start => entities,
            Cursor::Node(after) => {
                let date = serde_json::from_str::<DateTime<Utc>>(after)?;
                entities.filter(entity::messages::Column::CreatedAt.gt(date))
            }
        };

        let entities: Vec<_> = entities
            .order_by(entity::messages::Column::CreatedAt, Order::Asc)
            .limit(count)
            .all(self.context.db())
            .await?;
        if entities.is_empty() {
            return Ok(Connection::empty_end(has_previous_page));
        }

        let has_next_page = entity::messages::Entity::find()
            .select_only()
            .column_as(entity::messages::Column::Id.count(), QueryAs::Count)
            .filter(entity::messages::Column::CreatedAt.gt(entities.last().unwrap().created_at))
            .into_values::<_, QueryAs>()
            .one(self.context.db())
            .await?
            .map(|count: u64| count > 0)
            .unwrap_or(false);

        let edges: Vec<_> = entities
            .into_iter()
            .map(Message::from)
            .map(Edge)
            .collect();

        let page_info = PageInfo {
            has_previous_page,
            has_next_page,
            start_cursor: edges.first().unwrap().cursor(),
            end_cursor: edges.last().unwrap().cursor(),
        };
        Ok(Connection::new(edges, page_info))
    }

    async fn last(&self, count: u64, before: Cursor) -> FieldResult<Connection<Self::Node>> {
        let entities = entity::messages::Entity::find();

        let has_next_page = match &before {
            Cursor::Start => false,
            Cursor::End => self.len().await? > 0,
            Cursor::Node(..) => true,
        };

        let entities = match &before {
            Cursor::Start => {
                return Ok(Connection::empty_start(has_next_page));
            }
            Cursor::End => entities,
            Cursor::Node(after) => {
                let date = serde_json::from_str::<DateTime<Utc>>(after)?;
                entities.filter(entity::messages::Column::CreatedAt.lt(date))
            }
        };

        let mut entities: Vec<_> = entities
            .order_by(entity::messages::Column::CreatedAt, Order::Desc)
            .limit(count)
            .all(self.context.db())
            .await?;
        entities.reverse();
        if entities.is_empty() {
            return Ok(Connection::empty_start(has_next_page));
        }

        let has_previous_page = entity::messages::Entity::find()
            .select_only()
            .column_as(entity::messages::Column::Id.count(), QueryAs::Count)
            .filter(entity::messages::Column::CreatedAt.lt(entities.first().unwrap().created_at))
            .into_values::<_, QueryAs>()
            .one(self.context.db())
            .await?
            .map(|count: u64| count > 0)
            .unwrap_or(false);

        let edges: Vec<_> = entities
            .into_iter()
            .map(Message::from)
            .map(Edge)
            .collect();

        let page_info = PageInfo {
            has_previous_page,
            has_next_page,
            start_cursor: edges.first().unwrap().cursor(),
            end_cursor: edges.last().unwrap().cursor(),
        };
        Ok(Connection::new(edges, page_info))
    }
}

connection!(impl<'a> for MessagesConnector<'a> as "MessagesConnection");
