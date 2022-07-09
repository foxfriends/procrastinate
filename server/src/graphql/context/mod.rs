use actix_web::dev::Payload;
use actix_web::web;
use actix_web::FromRequest;
use actix_web::HttpRequest;
use sea_orm::DatabaseConnection;
use std::future::Future;
use std::pin::Pin;

pub(crate) struct Context {
    database: DatabaseConnection,
}

impl juniper::Context for Context {}

impl Context {
    pub fn db(&self) -> &DatabaseConnection {
        &self.database
    }
}

impl FromRequest for Context {
    type Error = <web::Data<DatabaseConnection> as FromRequest>::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let database = web::Data::<DatabaseConnection>::from_request(req, payload);
        Box::pin(async move {
            let database = (**database.await?).clone();
            Ok(Self { database })
        })
    }
}
