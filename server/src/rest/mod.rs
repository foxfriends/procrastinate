mod auth;

pub(crate) fn configure(config: &mut ServiceConfig) {
    config.service(auth::challenge).service(auth::verify)
}
