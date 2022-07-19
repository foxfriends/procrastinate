use gloo::net::http::Response;

mod check_auth;
mod error;
mod sign_in;

use crate::hooks::use_accounts::Account;

pub(crate) use error::Error;

#[derive(Clone, PartialEq)]
pub(crate) struct Api {
    account: Option<Account>,
}

impl Api {
    pub fn new(account: Option<Account>) -> Self {
        Self { account }
    }
}

trait ResponseExt: Sized {
    fn try_ok(self) -> Result<Self, Self>;
}

impl ResponseExt for Response {
    fn try_ok(self) -> Result<Self, Self> {
        if self.ok() {
            Ok(self)
        } else {
            Err(self)
        }
    }
}
