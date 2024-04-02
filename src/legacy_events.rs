use odra::casper_types::URef;
use odra::odra_casper_wasm_env::casper_contract::contract_api::storage;
use odra::{ContractEnv, UnwrapOrRevert};
use crate::events::Mint;
use odra::prelude::*;
use crate::cep47::Error::WrongArguments;

const CONTRACT_PACKAGE_HASH: &str = "contract_package_hash";

impl Mint {
    pub fn emit(&self, env: &ContractEnv) {
        let self_address = env.self_address();
        let package = self_address.as_contract_package_hash().unwrap_or_revert_with(env, WrongArguments);
        let mut events = Vec::new();
        for token_id in &self.token_ids {
            let mut param = BTreeMap::new();
            param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
            param.insert("event_type", "cep47_mint_one".to_string());
            param.insert("recipient", self.recipient.to_string());
            param.insert("token_id", token_id.to_string());
            events.push(param);
        }

        for param in events {
            let _: URef = storage::new_uref(param);
        }
    }
}