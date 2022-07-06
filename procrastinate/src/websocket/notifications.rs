use super::handler::SocketHandler;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

pub async fn notifications(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(SocketHandler::new(), &req, stream)
}
