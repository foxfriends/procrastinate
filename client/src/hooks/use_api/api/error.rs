use gloo::net::{self, http::Response};
use gloo::utils::errors::JsError;
use std::fmt;

#[derive(Debug)]
pub(crate) enum Error {
    Http(Response),
    Js(JsError),
    Web3(web3::Error),
    Serde(serde_json::Error),
    Custom(String),
}

impl Error {
    pub fn custom(message: impl fmt::Display) -> Self {
        Self::Custom(message.to_string())
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http(response) => {
                write!(f, "Error {}: {}", response.status(), response.status_text())
            }
            Self::Web3(error) => error.fmt(f),
            Self::Js(error) => error.fmt(f),
            Self::Serde(error) => error.fmt(f),
            Self::Custom(string) => write!(f, "Error: {string}"),
        }
    }
}

impl From<net::Error> for Error {
    fn from(error: net::Error) -> Self {
        match error {
            net::Error::JsError(error) => Self::Js(error),
            net::Error::SerdeError(error) => Self::Serde(error),
        }
    }
}

impl From<Response> for Error {
    fn from(error: Response) -> Self {
        Self::Http(error)
    }
}

impl From<web3::Error> for Error {
    fn from(error: web3::Error) -> Self {
        Self::Web3(error)
    }
}
