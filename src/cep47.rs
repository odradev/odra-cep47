use odra::{Address, SubModule, Var};
use odra::casper_types::U256;
use odra::prelude::*;
use crate::data::{Allowances, Metadata, OwnedTokens, Owners};
use crate::events::*;

pub type Meta = BTreeMap<String, String>;
pub type TokenId = U256;

#[odra::odra_type]
pub enum Error {
    PermissionDenied = 1,
    WrongArguments = 2,
    TokenIdAlreadyExists = 3,
    TokenIdDoesNotExist = 4,
    SymbolNotSet = 10,
    NameNotSet = 11,
}
impl From<Error> for odra::OdraError { fn from(error: Error) -> Self { odra::OdraError::user(error as u16) } }

#[odra::module(events = [Mint], errors = Error)]
pub struct Cep47 {
    pub name: Var<String>,
    pub symbol: Var<String>,
    pub meta: Var<BTreeMap<String, String>>,
    pub total_supply: Var<U256>,
    pub owners: SubModule<Owners>,
    pub owned_tokens: SubModule<OwnedTokens>,
    pub metadata: SubModule<Metadata>,
    pub allowances: SubModule<Allowances>,
}

#[odra::module]
impl Cep47 {
    pub fn init(&mut self, name: String, symbol: String, meta: Meta) {
        self.name.set(name);
        self.symbol.set(symbol);
        self.meta.set(meta);
        self.total_supply.set(U256::zero());
        self.owners.init();
        self.owned_tokens.init();
        self.metadata.init();
        self.allowances.init();
    }

    pub fn name(&self) -> String {
        self.name.get_or_revert_with(Error::NameNotSet)
    }
    pub fn symbol(&self) -> String {
        self.symbol.get_or_revert_with(Error::SymbolNotSet)
    }
    pub fn meta(&self) -> BTreeMap<String, String> {
        self.meta.get_or_default()
    }
    pub fn total_supply(&self) -> U256 {
        self.total_supply.get_or_default()
    }
    pub fn balance_of(&self, owner: Address) -> U256 {
        self.owned_tokens.get_balances(owner)
    }
    pub fn owner_of(&self, token_id: TokenId) -> Option<Address> {
        self.owners.get(token_id)
    }
    pub fn token_meta(&self, token_id: TokenId) -> Option<Meta> {
        self.metadata.get(token_id)
    }
    pub fn set_token_meta(&mut self, token_id: TokenId, meta: Meta) -> Result<(), Error> {
        if self.owner_of(token_id).is_none() {
            return Err(Error::TokenIdDoesNotExist);
        }

        self.metadata.set(token_id, meta);

        self.env().emit_event(MetadataUpdate { token_id });
        Ok(())
    }

    pub fn get_token_by_index(&self, owner: Address, index: U256) -> Option<TokenId> {
        self.owned_tokens.get_token_by_index(owner, index)
    }

    pub fn validate_token_ids(&self, token_ids: Vec<TokenId>) -> bool {
        token_ids
            .iter()
            .all(|token_id| self.owner_of(*token_id).is_some())
    }

    pub fn mint(
        &mut self,
        recipient: Address,
        token_ids: Vec<TokenId>,
        token_metas: Vec<Meta>,
    ) -> Result<Vec<TokenId>, Error> {
        if token_ids.len() != token_metas.len() {
            self.env().revert(Error::WrongArguments)
        }

        for token_id in &token_ids {
            if self.owner_of(*token_id).is_some() {
                self.env().revert(Error::TokenIdAlreadyExists)
            }
        }

        for (token_id, token_meta) in token_ids.iter().zip(&token_metas) {
            self.metadata.set(*token_id, token_meta.clone());
            self.owners.set(*token_id, recipient);
            self.owned_tokens.set_token(recipient, *token_id);
        }

        let minted_tokens_count: U256 = From::<u64>::from(token_ids.len().try_into().unwrap());
        self.total_supply.add(minted_tokens_count);

        let mint_event = Mint {
            recipient,
            token_ids: token_ids.clone(),
        };

        self.env().emit_event(&mint_event);

        #[cfg(target_arch = "wasm32")]
        {
            mint_event.emit(&self.env());
        }

        Ok(token_ids)
    }
    pub fn mint_copies(
        &mut self,
        recipient: Address,
        token_ids: Vec<TokenId>,
        token_meta: Meta,
        count: u32,
    ) -> Result<Vec<TokenId>, Error> {
        let token_metas = vec![token_meta; count as usize];
        self.mint(recipient, token_ids, token_metas)
    }
    pub fn burn(&mut self, owner: Address, token_ids: Vec<TokenId>) -> Result<(), Error> {
        let spender = self.env().caller();
        if spender != owner {
            for token_id in &token_ids {
                if !self.is_approved(owner, *token_id, spender) {
                    return Err(Error::PermissionDenied);
                }
            }
        }
        self.burn_internal(owner, token_ids)
    }

    pub fn approve(&mut self, spender: Address, token_ids: Vec<TokenId>) -> Result<(), Error> {
        let caller = self.env().caller();
        for token_id in &token_ids {
            match self.owner_of(*token_id) {
                None => return Err(Error::WrongArguments),
                Some(owner) if owner != caller => return Err(Error::PermissionDenied),
                Some(_) => self.allowances.set(caller, *token_id, spender),
            }
        }

        self.env().emit_event(Approve {
            owner: caller,
            spender,
            token_ids,
        });

        Ok(())
    }

    pub fn revoke(&mut self, token_ids: Vec<TokenId>) -> Result<(), Error> {
        let caller = self.env().caller();
        for token_id in &token_ids {
            match self.owner_of(*token_id) {
                None => return Err(Error::WrongArguments),
                Some(owner) if owner != caller => return Err(Error::PermissionDenied),
                Some(_) => self.allowances.remove(caller, *token_id),
            }
        }

        self.env().emit_event(Revoke {
            owner: caller,
            token_ids,
        });

        Ok(())
    }

    pub fn get_approved(&self, owner: Address, token_id: TokenId) -> Option<Address> {
        self.allowances.get(owner, token_id)
    }

    pub fn transfer(&mut self, recipient: Address, token_ids: Vec<TokenId>) -> Result<(), Error> {
        self.transfer_from(self.env().caller(), recipient, token_ids)
    }

    pub fn transfer_from(
        &mut self,
        owner: Address,
        recipient: Address,
        token_ids: Vec<TokenId>,
    ) -> Result<(), Error> {
        let spender = self.env().caller();
        if owner != spender {
            for token_id in &token_ids {
                if !self.is_approved(owner, *token_id, spender) {
                    return Err(Error::PermissionDenied);
                }
                self.allowances.remove(owner, *token_id);
            }
        }

        self.transfer_from_internal(owner, recipient, token_ids)
    }

    pub fn is_approved(&self, owner: Address, token_id: TokenId, spender: Address) -> bool {
        if let Some(spender_of) = self.allowances.get(owner, token_id) {
            return spender_of == spender;
        }
        false
    }
}

impl Cep47 {
    fn burn_internal(&mut self, owner: Address, token_ids: Vec<TokenId>) -> Result<(), Error> {
        self.validate_tokens_ownership(owner, token_ids.clone())?;

        for token_id in &token_ids {
            self.owned_tokens.remove_token(owner, *token_id);
            self.metadata.remove(*token_id);
            self.owners.remove(*token_id);
            self.allowances.remove(owner, *token_id);
        }

        let burnt_tokens_count = U256::from(token_ids.len() as u64);
        self.total_supply.subtract(burnt_tokens_count);
        self.env().emit_event(Burn { owner, token_ids });
        Ok(())
    }

    fn transfer_from_internal(
        &mut self,
        owner: Address,
        recipient: Address,
        token_ids: Vec<TokenId>,
    ) -> Result<(), Error> {
        self.validate_tokens_ownership(owner, token_ids.clone())?;

        for token_id in &token_ids {
            self.owned_tokens.remove_token(owner, *token_id);
            self.owned_tokens.set_token(recipient, *token_id);
            self.owners.set(*token_id, recipient);
        }

        self.env().emit_event(Transfer {
            from: owner,
            to: recipient,
            token_ids,
        });
        Ok(())
    }

    fn validate_tokens_ownership(
        &self,
        owner: Address,
        token_ids: Vec<TokenId>,
    ) -> Result<(), Error> {
        for token_id in &token_ids {
            match self.owners.get(*token_id) {
                Some(owner_of_key) => {
                    if owner_of_key != owner {
                        return Err(Error::PermissionDenied);
                    }
                }
                None => {
                    return Err(Error::TokenIdDoesNotExist);
                }
            }
        }

        Ok(())
    }
}
