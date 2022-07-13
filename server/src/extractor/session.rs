use super::EthereumAddress;
use actix_web::cookie::time::Duration;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

const COOKIE_NAME: &str = "session";

pub(crate) struct Error;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub(crate) struct Session {
    address: EthereumAddress,
}

impl FromRequest for Session {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        ready(
            req.cookie(COOKIE_NAME)
                .and_then(|cookie| serde_json::from_str(cookie.value()).ok())
                .ok_or(Error),
        )
    }
}

impl Session {
    pub fn new(address: EthereumAddress) -> Self {
        Self { address }
    }

    pub fn into_cookie(self) -> Cookie<'static> {
        let json = serde_json::to_string(&self).unwrap();
        Cookie::build(COOKIE_NAME, json)
            .secure(true)
            .http_only(true)
            .max_age(Duration::minutes(5))
            .same_site(SameSite::Strict)
            .path("/")
            .finish()
    }
}

impl From<Error> for actix_web::Error {
    fn from(_: Error) -> Self {
        actix_web::error::ErrorForbidden("Session not found")
    }
}
