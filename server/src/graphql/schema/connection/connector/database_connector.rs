use super::super::{Connection, ConnectionNode, Cursor, Edge, PageInfo};
use super::Connector;
use async_trait::async_trait;
use juniper::FieldResult;
use sea_orm::prelude::*;
use sea_orm::{Order, QueryOrder, QuerySelect};

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
enum QueryAs {
    Count,
}

#[async_trait]
pub(crate) trait DatabaseConnector {
    type Entity: EntityTrait;
    type Node: ConnectionNode + From<<Self::Entity as EntityTrait>::Model> + Sync + Send;
    const ORDER_COLUMN: <Self::Entity as EntityTrait>::Column;
    const COUNT_COLUMN: <Self::Entity as EntityTrait>::Column;

    fn database(&self) -> &DatabaseConnection;
    fn parse_cursor(&self, cursor: &str) -> FieldResult<sea_orm::Value>;

    async fn len(&self) -> FieldResult<u64> {
        Ok(Self::Entity::find()
            .select_only()
            .column_as(Self::COUNT_COLUMN.count(), QueryAs::Count)
            .into_values::<_, QueryAs>()
            .one(self.database())
            .await?
            .unwrap_or(0u64))
    }

    async fn page_info(&self, page: &[Edge<Self::Node>]) -> FieldResult<PageInfo> {
        let start_cursor = page.first().unwrap().0.cursor();
        let end_cursor = page.last().unwrap().0.cursor();

        let has_previous_page = Self::Entity::find()
            .select_only()
            .column_as(Self::COUNT_COLUMN.count(), QueryAs::Count)
            .filter(Self::ORDER_COLUMN.lt(self.parse_cursor(&start_cursor)?))
            .into_values::<_, QueryAs>()
            .one(self.database())
            .await?
            .map(|n: u64| n > 0)
            .unwrap_or(false);

        let has_next_page = Self::Entity::find()
            .select_only()
            .column_as(Self::COUNT_COLUMN.count(), QueryAs::Count)
            .filter(Self::ORDER_COLUMN.gt(self.parse_cursor(&end_cursor)?))
            .into_values::<_, QueryAs>()
            .one(self.database())
            .await?
            .map(|n: u64| n > 0)
            .unwrap_or(false);

        Ok(PageInfo {
            has_previous_page,
            has_next_page,
            start_cursor: Cursor::Node(start_cursor),
            end_cursor: Cursor::Node(end_cursor),
        })
    }

    fn before(&self, cursor: Option<Value>) -> Select<Self::Entity> {
        let entities = match cursor {
            None => Self::Entity::find(),
            Some(value) => Self::Entity::find().filter(Self::ORDER_COLUMN.lt(value)),
        };

        entities.order_by(Self::ORDER_COLUMN, Order::Desc)
    }

    fn after(&self, cursor: Option<Value>) -> Select<Self::Entity> {
        let entities = match cursor {
            None => Self::Entity::find(),
            Some(value) => Self::Entity::find().filter(Self::ORDER_COLUMN.gt(value)),
        };

        entities.order_by(Self::ORDER_COLUMN, Order::Asc)
    }
}

#[async_trait]
impl<T: DatabaseConnector + Send + Sync> Connector for T {
    type Node = <Self as DatabaseConnector>::Node;

    async fn len(&self) -> FieldResult<u64> {
        DatabaseConnector::len(self).await
    }

    async fn first(&self, count: u64, after: Cursor) -> FieldResult<Connection<Self::Node>> {
        let has_previous_page = match &after {
            Cursor::Start => false,
            Cursor::End => self.len().await? > 0,
            Cursor::Node(..) => true,
        };

        let entities = match &after {
            Cursor::End => {
                return Ok(Connection::empty_end(has_previous_page));
            }
            Cursor::Start => self.after(None),
            Cursor::Node(after) => {
                let value = self.parse_cursor(after)?;
                self.after(Some(value))
            }
        };

        let entities: Vec<_> = entities.limit(count).all(self.database()).await?;
        if entities.is_empty() {
            return Ok(Connection::empty_end(has_previous_page));
        }

        let edges: Vec<_> = entities.into_iter().map(From::from).map(Edge).collect();
        let page_info = self.page_info(&edges[..]).await?;
        Ok(Connection::new(edges, page_info))
    }

    async fn last(&self, count: u64, before: Cursor) -> FieldResult<Connection<Self::Node>> {
        let has_next_page = match &before {
            Cursor::Start => false,
            Cursor::End => self.len().await? > 0,
            Cursor::Node(..) => true,
        };

        let entities = match &before {
            Cursor::Start => {
                return Ok(Connection::empty_start(has_next_page));
            }
            Cursor::End => self.before(None),
            Cursor::Node(before) => {
                let value = self.parse_cursor(before)?;
                self.before(Some(value))
            }
        };

        let mut entities: Vec<_> = entities.limit(count).all(self.database()).await?;
        entities.reverse();
        if entities.is_empty() {
            return Ok(Connection::empty_start(has_next_page));
        }

        let edges: Vec<_> = entities.into_iter().map(From::from).map(Edge).collect();
        let page_info = self.page_info(&edges[..]).await?;
        Ok(Connection::new(edges, page_info))
    }
}
