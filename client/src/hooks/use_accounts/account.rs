use crate::hooks::use_web3::Web3;
use std::fmt;
use web3::signing::keccak256;
use web3::types::{Address, H520};

#[derive(Clone, Debug)]
pub(crate) struct Account {
    address: Address,
    #[allow(dead_code)]
    web3: Web3,
}

impl PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}

impl Account {
    pub fn new(address: Address, web3: Web3) -> Self {
        Self { address, web3 }
    }

    pub fn checksum(&self) -> Checksum {
        Checksum(self.address)
    }

    pub fn obscured(&self) -> Obscured {
        Obscured(self.address)
    }

    pub async fn sign(&self, message: &str) -> web3::Result<H520> {
        self.web3
            .personal()
            .sign(From::from(message), self.address, "")
            .await
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{}", hex::encode(self.address))
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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Obscured(Address);

impl fmt::Display for Obscured {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let checksum_str = Checksum(self.0).to_string();
        write!(
            f,
            "{}…{}",
            &checksum_str[..5],
            &checksum_str[checksum_str.len() - 3..]
        )
    }
}
