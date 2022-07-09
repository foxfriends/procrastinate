use super::{Connection, ConnectionNode, ConnectionResult, Cursor};
use juniper::FieldResult;

#[async_trait::async_trait]
pub(crate) trait Connector: Sync + Send {
    type Node: ConnectionNode;

    async fn len(&self) -> FieldResult<u64>;
    async fn first(&self, count: u64, after: Cursor) -> FieldResult<Connection<Self::Node>>;
    async fn last(&self, count: u64, before: Cursor) -> FieldResult<Connection<Self::Node>>;

    async fn get(
        self,
        first: Option<i32>,
        after: Option<Cursor>,
        last: Option<i32>,
        before: Option<Cursor>,
    ) -> FieldResult<ConnectionResult<Self>>
    where
        Self: Sized,
    {
        let connection = if let Some(count) = first {
            let count = if count < 0 { 0 } else { count as u64 };
            self.first(count, after.unwrap_or(Cursor::Start)).await?
        } else if let Some(count) = last {
            let count = if count < 0 { 0 } else { count as u64 };
            self.last(count, before.unwrap_or(Cursor::End)).await?
        } else {
            self.initial().await?
        };
        Ok(ConnectionResult::new(self, connection))
    }

    async fn initial(&self) -> FieldResult<Connection<Self::Node>> {
        self.first(30, Cursor::Start).await
    }
}
