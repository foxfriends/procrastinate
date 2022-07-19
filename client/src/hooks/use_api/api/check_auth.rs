use super::*;
use gloo::net::http::{Request, Response};

impl Api {
    pub async fn check_auth(&self) -> Result<bool, Error> {
        let account = match &self.account {
            None => return Ok(false),
            Some(account) => account,
        };
        Ok(Request::get("/api/auth")
            .header("X-Ethereum-Address", &account.to_string())
            .send()
            .await?
            .ok())
    }
}
