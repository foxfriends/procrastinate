use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::future::{ready, Ready};
use std::str::FromStr;
use web3::signing::keccak256;
use web3::types::Address;

#[derive(Debug)]
pub(crate) enum Error {
    Invalid,
    Missing,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) struct EthereumAddress(Address);

impl Serialize for EthereumAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = self.checksum().to_string();
        value.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for EthereumAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(serde::de::Error::custom)
    }
}

impl From<Address> for EthereumAddress {
    fn from(address: Address) -> Self {
        Self(address)
    }
}

impl EthereumAddress {
    pub fn checksum(&self) -> Checksum {
        Checksum(self.0)
    }
}

impl fmt::Display for EthereumAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{}", hex::encode(self.0))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Checksum(Address);

impl fmt::Display for Checksum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hash = keccak256(hex::encode(self.0).as_bytes());
        write!(f, "0x")?;
        for (i, ch) in hex::encode(self.0).chars().enumerate() {
            if (hash[i / 2] >> ((1 - i % 2) * 4)) & 0x0F >= 8 {
                ch.to_ascii_uppercase().fmt(f)?;
            } else {
                ch.to_ascii_lowercase().fmt(f)?;
            }
        }
        Ok(())
    }
}

impl FromStr for EthereumAddress {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 42 || !s.starts_with("0x") {
            return Err(Error::Invalid);
        }
        let bytes = hex::decode(&s[2..]).map_err(|_| Error::Invalid)?;
        Ok(Self(Address::from_slice(&bytes)))
    }
}

impl FromRequest for EthereumAddress {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let headers = req.headers();
        ready(
            headers
                .get("X-Ethereum-Address")
                .ok_or(Error::Missing)
                .and_then(|addr| addr.to_str().map_err(|_| Error::Invalid))
                .and_then(|addr| addr.parse()),
        )
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Invalid => "Invalid value for X-Ethereum-Address header".fmt(f),
            Self::Missing => "Missing X-Ethereum-Address header".fmt(f),
        }
    }
}

impl From<Error> for actix_web::Error {
    fn from(error: Error) -> Self {
        actix_web::error::ErrorBadRequest(error)
    }
}
