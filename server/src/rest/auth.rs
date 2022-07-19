use crate::extractor::{EthereumAddress, Session};
use actix_web::cookie::time::Duration;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::http::header::{CacheControl, CacheDirective};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use uuid::Uuid;
use web3::types::Recovery;
use web3::{transports::Http, Web3};

const COOKIE_NAME: &str = "challenge_nonce";

fn make_message(address: &EthereumAddress, nonce: &str) -> String {
    format!("Your Ethereum address ({}) is your account. We require a signature from the address to prove that you are the owner of the address. This will authorize your session to send messages on behalf. This will not provide access to any assets or other applications that use your address.\n\nSignature ID: {}", address.checksum(), nonce)
}

// TODO: Use private cookies here
// TODO: Use cryptographically secure random values

#[post("/api/auth/challenge")]
pub(crate) async fn challenge(address: EthereumAddress) -> actix_web::Result<impl Responder> {
    let challenge_nonce = Uuid::new_v4().to_hyphenated().to_string();
    let cookie = Cookie::build(COOKIE_NAME, &challenge_nonce)
        .secure(true)
        .http_only(true)
        .max_age(Duration::minutes(5))
        .same_site(SameSite::Strict)
        .path("/")
        .finish();
    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .body(make_message(&address, &challenge_nonce)))
}

#[get("/api/auth")]
pub(crate) async fn check(_session: Session) -> actix_web::Result<impl Responder> {
    Ok(HttpResponse::Ok().finish())
}

#[post("/api/auth/submit")]
pub(crate) async fn verify(
    request: HttpRequest,
    web3: web::Data<Web3<Http>>,
    address: EthereumAddress,
    signature: String,
) -> actix_web::Result<impl Responder> {
    let cookie = match request.cookie(COOKIE_NAME) {
        None => {
            return Err(actix_web::error::ErrorUnauthorized(
                "Missing challenge nonce",
            ));
        }
        Some(cookie) => cookie,
    };
    let message = make_message(&address, cookie.value());
    let signature = hex::decode(signature)
        .map_err(|_| actix_web::error::ErrorBadRequest("Signature must be hex encoded"))?;
    let recovery = Recovery::from_raw_signature(message, signature)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid signature"))?;
    let signer = web3
        .accounts()
        .recover(recovery)
        .map_err(|_| actix_web::error::ErrorBadRequest("Failed to recover signing address"))
        .map(EthereumAddress::from)?;
    if signer != address {
        return Err(actix_web::error::ErrorBadRequest(
            "Message not signed with expected address",
        ))?;
    }

    let mut removal = Cookie::build(COOKIE_NAME, "")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Strict)
        .path("/")
        .finish();
    removal.make_removal();
    let session = Session::new(address);
    removal.make_removal();
    Ok(HttpResponse::Ok()
        .cookie(removal)
        .cookie(session.into_cookie())
        .insert_header(CacheControl(vec![CacheDirective::NoStore]))
        .finish())
}
