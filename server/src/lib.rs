use actix_files::{Files, NamedFile};
use actix_web::web::{self, ServiceConfig};
use sea_orm::{Database, DatabaseConnection};
use std::path::PathBuf;
use web3::{transports::Http, Web3};

mod extractor;
mod graphql;
mod rest;
mod websocket;

#[derive(Clone)]
pub struct Server {
    database: DatabaseConnection,
    webapp_dir: PathBuf,
    web3: Web3<Http>,
}

impl Server {
    pub async fn new(
        database_url: &str,
        ethereum_url: &str,
        webapp_dir: PathBuf,
    ) -> anyhow::Result<Self> {
        let database = Database::connect(database_url).await?;
        let web3 = Web3::new(Http::new(ethereum_url)?);
        Ok(Self {
            database,
            web3,
            webapp_dir,
        })
    }

    pub fn configure(&self, config: &mut ServiceConfig) {
        let index_file = self.webapp_dir.join("index.html");
        config
            .app_data(web::Data::new(self.web3.clone()))
            .app_data(web::Data::new(self.database.clone()))
            .configure(rest::configure)
            .configure(graphql::configure)
            .configure(websocket::configure)
            .service(Files::new("/", &self.webapp_dir).index_file("index.html"))
            .default_service(web::get().to(move || NamedFile::open_async(index_file.clone())));
    }
}
