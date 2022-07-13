#[actix_web::post("/api/auth/challenge")]
pub fn challenge(cookies: &CookieJar) -> String {

}

#[actix_web::post("/api/auth/submit")]
pub fn verify() -> String {}
