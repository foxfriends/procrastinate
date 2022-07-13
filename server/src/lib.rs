use actix_files::Files;
use actix_web::web::{self, ServiceConfig};
use sea_orm::{Database, DatabaseConnection};
use std::path::PathBuf;

mod graphql;
mod websocket;

#[derive(Clone)]
pub struct Server {
    database: DatabaseConnection,
    webapp_dir: PathBuf,
}

impl Server {
    pub async fn new(database_url: &str, webapp_dir: PathBuf) -> anyhow::Result<Self> {
        let database = Database::connect(database_url).await?;
        Ok(Self {
            database,
            webapp_dir,
        })
    }

    pub fn configure(&self, config: &mut ServiceConfig) {
        config
            .app_data(web::Data::new(self.database.clone()))
            .configure(rest::configure)
            .configure(graphql::configure)
            .configure(websocket::configure)
            .service(Files::new("/", &self.webapp_dir).index_file("index.html"));
    }
}
