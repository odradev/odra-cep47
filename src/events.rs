use alloc::vec::Vec;
use odra::prelude::*;
use odra::{Address, Event};

use crate::cep47::TokenId;

#[derive(Event, PartialEq, Eq, Debug)]
pub struct Mint {
    pub recipient: Address,
    pub token_ids: Vec<TokenId>,
}

#[derive(Event, PartialEq, Eq, Debug)]
pub struct Burn {
    pub owner: Address,
    pub token_ids: Vec<TokenId>,
}

#[derive(Event, PartialEq, Eq, Debug)]
pub struct Approve {
    pub owner: Address,
    pub spender: Address,
    pub token_ids: Vec<TokenId>,
}

#[derive(Event, PartialEq, Eq, Debug)]
pub struct Revoke {
    pub owner: Address,
    pub token_ids: Vec<TokenId>,
}

#[derive(Event, PartialEq, Eq, Debug)]
pub struct Transfer {
    pub from: Address,
    pub to: Address,
    pub token_ids: Vec<TokenId>,
}

#[derive(Event, PartialEq, Eq, Debug)]
pub struct MetadataUpdate {
    pub token_id: TokenId,
}
