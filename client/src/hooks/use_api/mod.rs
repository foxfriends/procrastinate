use super::use_account;
use yew::prelude::*;

mod api;

pub(crate) use api::{Api, Error};

pub(crate) fn use_api() -> Api {
    let account = use_account();
    Api::new(account)
}
