use alloc::vec::Vec;

use odra::Address;
use odra::prelude::*;

use crate::cep47::TokenId;

#[odra::event]
pub struct Mint {
    pub recipient: Address,
    pub token_ids: Vec<TokenId>,
}

#[odra::event]
pub struct Burn {
    pub owner: Address,
    pub token_ids: Vec<TokenId>,
}

#[odra::event]
pub struct Approve {
    pub owner: Address,
    pub spender: Address,
    pub token_ids: Vec<TokenId>,
}

#[odra::event]
pub struct Revoke {
    pub owner: Address,
    pub token_ids: Vec<TokenId>,
}

#[odra::event]
pub struct Transfer {
    pub from: Address,
    pub to: Address,
    pub token_ids: Vec<TokenId>,
}

#[odra::event]
pub struct MetadataUpdate {
    pub token_id: TokenId,
}
