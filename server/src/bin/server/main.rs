use actix_web::{middleware, App, HttpServer};
use server::Server;

mod env;
mod filter_middleware;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let host = *env::HOST;
    let port = *env::PORT;

    log::info!("Redlack served on {}:{}", host, port);

    let server = Server::new(
        &*env::DATABASE_URL,
        &*env::ETHEREUM_URL,
        env::WEBAPP_DIR.clone(),
    )
    .await?;

    HttpServer::new(move || {
        let logger = middleware::Logger::default().log_target("[http request]");
        let logger = filter_middleware::Filter::new(middleware::Compat::new(logger), |req| {
            // TODO: if this project doesn't end up using graphql, get rid of this.
            // it might still use graphql though... we'll see
            req.headers()
                .get("Referer")
                .and_then(|s| s.to_str().ok())
                .map(|s| !s.contains("/graphql/playground"))
                .unwrap_or(true)
        });
        App::new()
            .wrap(logger)
            .wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::trim())
            .configure(|config| server.configure(config))
    })
    .bind((host, port))?
    .run()
    .await?;
    Ok(())
}
