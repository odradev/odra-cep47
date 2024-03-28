use core::cmp::Ordering;
use core::ops::{Add, Sub};

use odra::casper_types::U256;
use odra::prelude::*;
use odra::{Address, Mapping, UnwrapOrRevert};

use crate::cep47::{Meta, TokenId};

#[odra::module]
pub struct Owners {
    pub owners: Mapping<TokenId, Option<Address>>,
}

#[odra::module]
impl Owners {
    pub fn init(&mut self) {}

    pub fn get(&self, token_id: TokenId) -> Option<Address> {
        self.owners.get_or_default(&token_id)
    }

    pub fn set(&mut self, token_id: TokenId, owner: Address) {
        self.owners.set(&token_id, Some(owner));
    }

    pub fn remove(&mut self, token_id: TokenId) {
        self.owners.set(&token_id, None);
    }
}

#[odra::module]
pub struct Metadata {
    pub metadata: Mapping<TokenId, Option<Meta>>,
}

#[odra::module]
impl Metadata {
    pub fn init(&mut self) {}

    pub fn get(&self, token_id: TokenId) -> Option<Meta> {
        self.metadata.get_or_default(&token_id)
    }

    pub fn set(&mut self, token_id: TokenId, value: Meta) {
        self.metadata.set(&token_id, Some(value));
    }

    pub fn remove(&mut self, token_id: TokenId) {
        self.metadata.set(&token_id, None);
    }
}

#[odra::module]
pub struct OwnedTokens {
    pub tokens: Mapping<(Address, U256), Option<TokenId>>,
    pub indexes: Mapping<(Address, TokenId), Option<U256>>,
    pub balances: Mapping<Address, U256>,
}

#[odra::module]
impl OwnedTokens {
    pub fn init(&mut self) {}

    pub fn get_token_by_index(&self, owner: Address, index: U256) -> Option<TokenId> {
        self.tokens.get_or_default(&(owner, index))
    }

    pub fn get_index_by_token(&self, owner: Address, value: TokenId) -> Option<U256> {
        self.indexes.get_or_default(&(owner, value))
    }

    pub fn get_balances(&self, owner: Address) -> U256 {
        self.balances.get_or_default(&owner)
    }

    pub fn set_balances(&mut self, owner: Address, value: U256) {
        self.balances.set(&owner, value)
    }

    pub fn set_token(&mut self, owner: Address, value: TokenId) {
        let length = self.get_balances(owner.clone());
        self.indexes.set(&(owner.clone(), value), Some(length));
        self.tokens.set(&(owner.clone(), length), Some(value));
        self.set_balances(owner, length.add(1));
    }

    pub fn remove_token(&mut self, owner: Address, value: TokenId) {
        let length = self.get_balances(owner.clone());
        let index = self
            .get_index_by_token(owner.clone(), value)
            .unwrap_or_revert(&self.env());
        match length.cmp(&(index + 1)) {
            Ordering::Equal => {
                self.tokens.set(&(owner.clone(), index), None);
                self.set_token(owner, length.sub(1))
            }
            Ordering::Greater => {
                let last = self
                    .get_token_by_index(owner.clone(), length.sub(1))
                    .unwrap_or_revert(&self.env());
                self.indexes.set(&(owner.clone(), last), Some(index));
                self.tokens.set(&(owner.clone(), index), Some(last));
                self.tokens.set(&(owner.clone(), length.sub(1)), None);
                self.set_balances(owner, length.sub(1));
            }
            Ordering::Less => {}
        }

        self.indexes.set(&(owner.clone(), value), None);
    }
}

#[odra::module]
pub struct Allowances {
    pub allowances: Mapping<(Address, TokenId), Option<Address>>,
}

#[odra::module]
impl Allowances {
    pub fn init(&mut self) {}

    pub fn get(&self, owner: Address, token_id: TokenId) -> Option<Address> {
        self.allowances.get_or_default(&(owner, token_id))
    }

    pub fn set(&mut self, owner: Address, token_id: TokenId, value: Address) {
        self.allowances.set(&(owner, token_id), Some(value));
    }

    pub fn remove(&mut self, owner: Address, token_id: TokenId) {
        self.allowances.set(&(owner, token_id), None);
    }
}
