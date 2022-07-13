use lazy_static::lazy_static;
use std::net::IpAddr;
use std::path::PathBuf;

lazy_static! {
    pub static ref HOST: IpAddr = std::env::var("HOST")
        .unwrap_or_else(|_| "0.0.0.0".to_owned())
        .parse()
        .unwrap();
    pub static ref PORT: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8000".to_owned())
        .parse()
        .unwrap();
    pub static ref DATABASE_URL: String =
        std::env::var("DATABASE_URL").expect("DATABASE_URL is required");
    pub static ref ETHEREUM_URL: String =
        std::env::var("ETHEREUM_URL").expect("ETHEREUM_URL is required");
    pub static ref WEBAPP_DIR: PathBuf = std::env::var("WEBAPP_DIR")
        .expect("WEBAPP_DIR is required")
        .parse()
        .unwrap();
}
