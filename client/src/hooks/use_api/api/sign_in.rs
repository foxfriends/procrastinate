use super::*;
use gloo::net::http::{Request, Response};

impl Api {
    pub async fn sign_in(&self) -> Result<(), Error> {
        let account = self
            .account
            .as_ref()
            .ok_or_else(|| Error::custom("Account is not connected"))?;
        let message = Request::post("/api/auth/challenge")
            .header("X-Ethereum-Address", &account.to_string())
            .send()
            .await?
            .try_ok()?
            .text()
            .await?;
        let signature = account.sign(&message).await?;
        Request::post("/api/auth/submit")
            .header("X-Ethereum-Address", &account.to_string())
            .body(hex::encode(signature))
            .send()
            .await?
            .try_ok()?;
        Ok(())
    }
}
