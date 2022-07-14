use crate::hooks::use_web3::Web3;
use web3::types::Address;

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
}
