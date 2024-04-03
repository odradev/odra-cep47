//! This module contains the implementation of the legacy events for the CEP47 standard.
//! When cspr.live will migrate to CES in their CEP-47 discovery, this module will be removed.
//! As the events are stored in storage in a layout currently not supported by Odra,
//! this module calls casper host directly, and causes the code to be hidden behind the
//! `#[cfg(target_arch = "wasm32")]` attribute.

use crate::cep47::Error::InternalError;
use crate::events::{Approve, Burn, MetadataUpdate, Mint, Revoke, Transfer};
use odra::casper_types::URef;
use odra::odra_casper_wasm_env::casper_contract::contract_api::storage;
use odra::prelude::*;
use odra::{ContractEnv, UnwrapOrRevert};

const CONTRACT_PACKAGE_HASH: &str = "contract_package_hash";

impl Mint {
    /// Emits a `cep47_mint_one` event.
    pub fn emit(&self, env: &ContractEnv) {
        let self_address = env.self_address();
        let package = self_address
            .as_contract_package_hash()
            .unwrap_or_revert_with(env, InternalError);
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

impl Burn {
    /// Emits a `cep47_burn_one` event.
    pub fn emit(&self, env: &ContractEnv) {
        let self_address = env.self_address();
        let package = self_address
            .as_contract_package_hash()
            .unwrap_or_revert_with(env, InternalError);
        let mut events = Vec::new();
        for token_id in &self.token_ids {
            let mut param = BTreeMap::new();
            param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
            param.insert("event_type", "cep47_burn_one".to_string());
            param.insert("owner", self.owner.to_string());
            param.insert("token_id", token_id.to_string());
            events.push(param);
        }

        for param in events {
            let _: URef = storage::new_uref(param);
        }
    }
}

impl Approve {
    /// Emits a `cep47_approve_token` event.
    pub fn emit(&self, env: &ContractEnv) {
        let self_address = env.self_address();
        let package = self_address
            .as_contract_package_hash()
            .unwrap_or_revert_with(env, InternalError);
        let mut events = Vec::new();
        for token_id in &self.token_ids {
            let mut param = BTreeMap::new();
            param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
            param.insert("event_type", "cep47_approve_token".to_string());
            param.insert("owner", self.owner.to_string());
            param.insert("spender", self.spender.to_string());
            param.insert("token_id", token_id.to_string());
            events.push(param);
        }

        for param in events {
            let _: URef = storage::new_uref(param);
        }
    }
}

impl Revoke {
    /// Emits a `cep47_revoke_token` event.
    pub fn emit(&self, env: &ContractEnv) {
        let self_address = env.self_address();
        let package = self_address
            .as_contract_package_hash()
            .unwrap_or_revert_with(env, InternalError);
        let mut events = Vec::new();
        for token_id in &self.token_ids {
            let mut param = BTreeMap::new();
            param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
            param.insert("event_type", "cep47_revoke_token".to_string());
            param.insert("owner", self.owner.to_string());
            param.insert("token_id", token_id.to_string());
            events.push(param);
        }

        for param in events {
            let _: URef = storage::new_uref(param);
        }
    }
}

impl Transfer {
    /// Emits a `cep47_transfer_token` event.
    pub fn emit(&self, env: &ContractEnv) {
        let self_address = env.self_address();
        let package = self_address
            .as_contract_package_hash()
            .unwrap_or_revert_with(env, InternalError);
        let mut events = Vec::new();
        for token_id in &self.token_ids {
            let mut param = BTreeMap::new();
            param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
            param.insert("event_type", "cep47_transfer_token".to_string());
            param.insert("sender", self.from.to_string());
            param.insert("recipient", self.to.to_string());
            param.insert("token_id", token_id.to_string());
            events.push(param);
        }

        for param in events {
            let _: URef = storage::new_uref(param);
        }
    }
}

impl MetadataUpdate {
    /// Emits a `cep47_metadata_update` event.
    pub fn emit(&self, env: &ContractEnv) {
        let self_address = env.self_address();
        let package = self_address
            .as_contract_package_hash()
            .unwrap_or_revert_with(env, InternalError);
        let mut param = BTreeMap::new();
        param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
        param.insert("event_type", "cep47_metadata_update".to_string());
        param.insert("token_id", self.token_id.to_string());
        let _: URef = storage::new_uref(param);
    }
}
