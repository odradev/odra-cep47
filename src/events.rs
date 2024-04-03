//! Events that can be emitted by the CEP-47 contract.
use alloc::vec::Vec;

use odra::casper_types::Key;

use crate::cep47::TokenId;

/// Mint event.
pub struct Mint {
    pub recipient: Key,
    pub token_ids: Vec<TokenId>,
}

/// Burn event.
pub struct Burn {
    pub owner: Key,
    pub token_ids: Vec<TokenId>,
}

/// Approve event.
pub struct Approve {
    pub owner: Key,
    pub spender: Key,
    pub token_ids: Vec<TokenId>,
}

/// Revoke event.
pub struct Revoke {
    pub owner: Key,
    pub token_ids: Vec<TokenId>,
}

/// Transfer event.
pub struct Transfer {
    pub from: Key,
    pub to: Key,
    pub token_ids: Vec<TokenId>,
}

/// Metadata update event.
pub struct MetadataUpdate {
    pub token_id: TokenId,
}
