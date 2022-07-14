use super::Account;
use crate::hooks::use_web3::Web3;
use web3::types::Address;

#[derive(Clone)]
pub(crate) struct Accounts {
    accounts: Vec<Account>,
    web3: Web3,
}

impl PartialEq for Accounts {
    fn eq(&self, other: &Self) -> bool {
        self.accounts == other.accounts
    }
}

impl Accounts {
    pub fn new(addresses: Vec<Address>, web3: Web3) -> Self {
        Self {
            accounts: addresses
                .into_iter()
                .map(|address| Account::new(address, web3.clone()))
                .collect(),
            web3,
        }
    }

    pub fn empty(web3: Web3) -> Self {
        Self {
            accounts: vec![],
            web3,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.accounts.is_empty()
    }

    pub async fn connect(&self) -> web3::error::Result<Vec<Address>> {
        self.web3.eth().request_accounts().await
    }
}
