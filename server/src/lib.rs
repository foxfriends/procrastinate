use actix_web::web::{self, ServiceConfig};
use sea_orm::{Database, DatabaseConnection};

mod websocket;

#[derive(Clone)]
pub struct Server {
    database: DatabaseConnection,
}

impl Server {
    pub async fn new(database_url: &str) -> anyhow::Result<Self> {
        let database = Database::connect(database_url).await?;
        Ok(Self { database })
    }

    pub fn configure(&self, config: &mut ServiceConfig) {
        config
            .app_data(web::Data::new(self.database.clone()))
            .configure(websocket::configure);
    }
}