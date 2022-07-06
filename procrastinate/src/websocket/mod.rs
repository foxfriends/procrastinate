use actix_web::web::{self, ServiceConfig};

mod handler;
mod notifications;

use notifications::notifications;

pub fn configure(config: &mut ServiceConfig) {
    config.route("/ws", web::get().to(notifications));
}
