#[cfg(test)]
mod tests {
    const NAME: &str = "DragonsNFT";
    const SYMBOL: &str = "DGNFT";

    use crate::cep47::Error::PermissionDenied;
    use crate::cep47::{Cep47HostRef, Cep47InitArgs, TokenId};
    use odra::casper_types::U256;
    use odra::host::{Deployer, HostEnv};
    use odra::prelude::*;
    use odra::Address;

    #[test]
    fn test_deploy() {
        let (_, token, _) = deploy();
        assert_eq!(token.name(), NAME);
        assert_eq!(token.symbol(), SYMBOL);
        assert_eq!(token.meta(), meta::contract_meta());
        assert_eq!(token.total_supply(), U256::zero());
    }

    #[test]
    fn test_token_meta() {
        let (env, mut token, _) = deploy();
        let user = env.get_account(1);
        let token_id = TokenId::zero();
        let token_meta = meta::red_dragon();

        let _ = token.mint(user, vec![token_id], vec![token_meta.clone()]);

        let user_token_meta = token.token_meta(token_id);
        assert_eq!(user_token_meta.unwrap(), token_meta);

        let first_user_token = token.get_token_by_index(user, U256::zero());
        assert_eq!(first_user_token, Some(token_id));
    }

    #[test]
    fn test_mint_one() {
        let (env, mut token, _) = deploy();
        let user = env.get_account(1);
        let token_id = TokenId::zero();
        let token_meta = meta::red_dragon();

        let _ = token.mint(user, vec![token_id], vec![token_meta]);
        let first_user_token = token.get_token_by_index(user, U256::from(0));
        let second_user_token = token.get_token_by_index(user, U256::from(1));
        assert_eq!(first_user_token, Some(token_id));
        assert_eq!(token.total_supply(), U256::one());
        assert_eq!(token.balance_of(user), U256::one());
        assert_eq!(second_user_token, None);
        assert_eq!(token.owner_of(token_id).unwrap(), user);
    }

    #[test]
    fn test_mint_copies() {
        let (env, mut token, _) = deploy();
        let user = env.get_account(1);
        let token_meta = meta::red_dragon();
        let token_ids = vec![TokenId::zero(), TokenId::one(), TokenId::from(2)];
        let _ = token.mint_copies(user, token_ids.clone(), token_meta, 3);
        let first_user_token = token.get_token_by_index(user, U256::from(0));
        let second_user_token = token.get_token_by_index(user, U256::from(1));
        let third_user_token = token.get_token_by_index(user, U256::from(2));
        let fourth_user_token = token.get_token_by_index(user, U256::from(3));
        assert_eq!(token.total_supply(), U256::from(3));
        assert_eq!(token.balance_of(user), U256::from(3));
        assert_eq!(token.owner_of(first_user_token.unwrap()).unwrap(), user);
        assert_eq!(token.owner_of(second_user_token.unwrap()).unwrap(), user);
        assert_eq!(token.owner_of(third_user_token.unwrap()).unwrap(), user);
        assert_eq!(first_user_token, Some(token_ids[0]));
        assert_eq!(second_user_token, Some(token_ids[1]));
        assert_eq!(third_user_token, Some(token_ids[2]));
        assert_eq!(fourth_user_token, None);
    }

    #[test]
    fn test_mint_many() {
        let (env, mut token, _) = deploy();
        let user = env.get_account(1);
        let token_metas = vec![meta::red_dragon(), meta::gold_dragon()];
        let token_ids = vec![TokenId::zero(), TokenId::one()];
        let _ = token.mint(user, token_ids.clone(), token_metas);
        let first_user_token = token.get_token_by_index(user, U256::from(0));
        let second_user_token = token.get_token_by_index(user, U256::from(1));
        let third_user_token = token.get_token_by_index(user, U256::from(2));
        assert_eq!(token.total_supply(), U256::from(2));
        assert_eq!(token.balance_of(user), U256::from(2));
        assert_eq!(token.owner_of(first_user_token.unwrap()).unwrap(), user);
        assert_eq!(token.owner_of(second_user_token.unwrap()).unwrap(), user);
        assert_eq!(first_user_token, Some(token_ids[0]));
        assert_eq!(second_user_token, Some(token_ids[1]));
        assert_eq!(third_user_token, None);
    }

    #[test]
    fn test_burn_many() {
        let (env, mut token, _) = deploy();
        let user = env.get_account(1);
        let token_metas = vec![
            meta::red_dragon(),
            meta::blue_dragon(),
            meta::black_dragon(),
            meta::gold_dragon(),
        ];
        let token_ids = vec![
            TokenId::zero(),
            TokenId::one(),
            TokenId::from(2),
            TokenId::from(3),
        ];

        let _ = token.mint(user, token_ids.clone(), token_metas);

        env.set_caller(user);
        let _ = token.burn(user, vec![token_ids[0], token_ids[3]]);
        assert_eq!(token.total_supply(), U256::from(2));
        assert_eq!(token.balance_of(user), U256::from(2));

        let new_first_user_token = token.get_token_by_index(user, U256::from(0));
        let new_second_user_token = token.get_token_by_index(user, U256::from(1));
        let new_third_user_token = token.get_token_by_index(user, U256::from(2));
        let new_fourth_user_token = token.get_token_by_index(user, U256::from(3));
        assert_eq!(new_first_user_token, Some(token_ids[2]));
        assert_eq!(new_second_user_token, Some(token_ids[1]));
        assert_eq!(new_third_user_token, None);
        assert_eq!(new_fourth_user_token, None);
    }

    #[test]
    fn test_burn_many_from_allowance_with_approve() {
        let (env, mut token, owner) = deploy();
        let user = env.get_account(1);
        let token_metas = vec![
            meta::red_dragon(),
            meta::blue_dragon(),
            meta::black_dragon(),
            meta::gold_dragon(),
        ];
        let token_ids = vec![
            TokenId::zero(),
            TokenId::one(),
            TokenId::from(2),
            TokenId::from(3),
        ];

        let _ = token.mint(user, token_ids.clone(), token_metas);

        env.set_caller(user);
        let _ = token.approve(owner, vec![token_ids[0], token_ids[2]]);
        env.set_caller(owner);
        assert_eq!(token.total_supply(), U256::from(4));
        let _ = token.burn(user, vec![token_ids[0], token_ids[2]]);
        assert_eq!(token.total_supply(), U256::from(2));
        assert_eq!(token.balance_of(user), U256::from(2));

        let new_first_user_token = token.get_token_by_index(user, U256::from(0));
        let new_second_user_token = token.get_token_by_index(user, U256::from(1));
        let new_third_user_token = token.get_token_by_index(user, U256::from(2));
        let new_fourth_user_token = token.get_token_by_index(user, U256::from(3));
        assert_eq!(new_first_user_token, Some(token_ids[3]));
        assert_eq!(new_second_user_token, Some(token_ids[1]));
        assert_eq!(new_third_user_token, None);
        assert_eq!(new_fourth_user_token, None);
    }

    #[test]
    fn test_burn_many_from_allowance_without_approve() {
        let (env, mut token, _) = deploy();
        let user = env.get_account(1);
        let token_metas = vec![
            meta::red_dragon(),
            meta::blue_dragon(),
            meta::black_dragon(),
            meta::gold_dragon(),
        ];
        let token_ids = vec![
            TokenId::zero(),
            TokenId::one(),
            TokenId::from(2),
            TokenId::from(3),
        ];

        assert_eq!(
            token.mint(user, token_ids.clone(), token_metas),
            Ok(token_ids.clone())
        );

        assert_eq!(
            token.burn(user, vec![token_ids[0], token_ids[1]]),
            Err(PermissionDenied)
        );
    }

    #[test]
    fn test_burn_one() {
        let (env, mut token, _) = deploy();
        let user = env.get_account(1);
        let token_metas = vec![meta::red_dragon(), meta::gold_dragon()];
        let token_ids = vec![TokenId::zero(), TokenId::one()];
        let _ = token.mint(user, token_ids.clone(), token_metas);
        env.set_caller(user);
        let _ = token.burn(user, vec![token_ids[0]]);
        assert_eq!(token.total_supply(), U256::from(1));
        assert_eq!(token.balance_of(user), U256::from(1));

        let new_first_user_token = token.get_token_by_index(user, U256::from(0));
        let new_second_user_token = token.get_token_by_index(user, U256::from(1));
        assert_eq!(new_first_user_token, Some(token_ids[1]));
        assert_eq!(new_second_user_token, None);
    }

    #[test]
    fn test_transfer_token() {
        let (env, mut token, _) = deploy();
        let ali = env.get_account(1);
        let bob = env.get_account(2);
        let token_metas = vec![meta::red_dragon(), meta::gold_dragon()];
        let token_ids = vec![TokenId::zero(), TokenId::one()];

        let _ = token.mint(ali, token_ids.clone(), token_metas);

        assert_eq!(token.total_supply(), U256::from(2));
        assert_eq!(token.balance_of(ali), U256::from(2));
        assert_eq!(token.owner_of(token_ids[0]).unwrap(), ali);
        assert_eq!(token.owner_of(token_ids[1]).unwrap(), ali);

        env.set_caller(ali);
        let _ = token.transfer(bob, vec![token_ids[0]]);
        let new_first_ali_token = token.get_token_by_index(ali, U256::from(0));
        let new_second_ali_token = token.get_token_by_index(ali, U256::from(1));
        let new_first_bob_token = token.get_token_by_index(bob, U256::from(0));
        let new_second_bob_token = token.get_token_by_index(bob, U256::from(1));

        assert_eq!(token.total_supply(), U256::from(2));
        assert_eq!(token.balance_of(ali), U256::from(1));
        assert_eq!(token.balance_of(bob), U256::from(1));
        assert_eq!(token.owner_of(new_first_ali_token.unwrap()).unwrap(), ali);
        assert_eq!(token.owner_of(new_first_bob_token.unwrap()).unwrap(), bob);
        assert_eq!(new_second_ali_token, None);
        assert_eq!(new_second_bob_token, None);
    }

    #[test]
    fn test_transfer_from_tokens_with_approve() {
        let (env, mut token, owner) = deploy();
        let ali = env.get_account(1);
        let bob = env.get_account(2);
        let token_metas = vec![meta::red_dragon(), meta::gold_dragon()];
        let token_ids = vec![TokenId::zero(), TokenId::one()];

        let _ = token.mint(ali, token_ids.clone(), token_metas);
        assert_eq!(token.total_supply(), U256::from(2));
        assert_eq!(token.balance_of(ali), U256::from(2));
        assert_eq!(token.owner_of(token_ids[0]).unwrap(), ali);
        assert_eq!(token.owner_of(token_ids[1]).unwrap(), ali);
        env.set_caller(ali);
        let _ = token.approve(owner, vec![TokenId::one()]);
        env.set_caller(owner);
        let _ = token.transfer_from(ali, bob, vec![TokenId::one()]);
        let new_first_ali_token = token.get_token_by_index(ali, U256::from(0));
        let new_second_ali_token = token.get_token_by_index(ali, U256::from(1));
        let new_first_bob_token = token.get_token_by_index(bob, U256::from(0));
        let new_second_bob_token = token.get_token_by_index(bob, U256::from(1));
        assert_eq!(token.total_supply(), U256::from(2));
        assert_eq!(token.balance_of(ali), U256::from(1));
        assert_eq!(token.balance_of(bob), U256::from(1));
        assert_eq!(token.owner_of(new_first_ali_token.unwrap()).unwrap(), ali);
        assert_eq!(token.owner_of(new_first_bob_token.unwrap()).unwrap(), bob);
        assert_eq!(new_second_ali_token, None);
        assert_eq!(new_second_bob_token, None);
    }

    #[test]
    fn test_transfer_from_tokens_without_approve() {
        let (env, mut token, _) = deploy();
        let ali = env.get_account(1);
        let bob = env.get_account(2);
        let token_metas = vec![meta::red_dragon(), meta::gold_dragon()];
        let token_ids = vec![TokenId::zero(), TokenId::one()];

        let _ = token.mint(ali, token_ids.clone(), token_metas);

        assert_eq!(token.total_supply(), U256::from(2));
        assert_eq!(token.balance_of(ali), U256::from(2));
        assert_eq!(token.owner_of(token_ids[0]).unwrap(), ali);
        assert_eq!(token.owner_of(token_ids[1]).unwrap(), ali);
        assert_eq!(
            token.transfer_from(ali, bob, vec![token_ids[0]]),
            Err(PermissionDenied)
        );
    }

    #[test]
    fn test_approve() {
        let (env, mut token, owner) = deploy();
        let user = env.get_account(1);
        let token_metas = vec![
            meta::red_dragon(),
            meta::blue_dragon(),
            meta::black_dragon(),
            meta::gold_dragon(),
        ];
        let token_ids = vec![
            TokenId::zero(),
            TokenId::one(),
            TokenId::from(1),
            TokenId::from(2),
        ];

        let _ = token.mint(user, token_ids.clone(), token_metas);

        env.set_caller(user);
        let _ = token.approve(owner, vec![token_ids[0], token_ids[3]]);
        assert_eq!(token.get_approved(user, token_ids[0]).unwrap(), owner);
        assert_eq!(token.get_approved(user, token_ids[3]).unwrap(), owner);
    }

    #[test]
    fn test_token_metadata_update() {
        let (env, mut token, _) = deploy();
        let user = env.get_account(1);
        let token_id = TokenId::zero();

        let _ = token.mint(user, vec![token_id], vec![meta::red_dragon()]);

        let _ = token.set_token_meta(token_id, meta::gold_dragon());
        assert_eq!(token.token_meta(token_id).unwrap(), meta::gold_dragon());
    }

    #[test]
    fn test_transfer_from_tokens_after_revoked_approval() {
        let (env, mut token, owner) = deploy();
        let ali = env.get_account(1);
        let bob = env.get_account(2);
        let token_metas = vec![meta::red_dragon(), meta::gold_dragon()];
        let token_ids = vec![TokenId::zero(), TokenId::one()];

        let _ = token.mint(ali, token_ids.clone(), token_metas);
        assert_eq!(token.total_supply(), U256::from(2));
        assert_eq!(token.balance_of(ali), U256::from(2));
        assert_eq!(token.owner_of(token_ids[0]).unwrap(), ali);
        assert_eq!(token.owner_of(token_ids[1]).unwrap(), ali);
        env.set_caller(ali);
        let _ = token.approve(bob, vec![TokenId::one()]);
        assert_eq!(token.get_approved(ali, token_ids[1]).unwrap(), bob);
        let _ = token.revoke(vec![TokenId::one()]);
        env.set_caller(bob);
        assert_eq!(
            token.transfer_from(ali, owner, vec![TokenId::one()]),
            Err(PermissionDenied)
        );
    }

    fn deploy() -> (HostEnv, Cep47HostRef, Address) {
        let env = odra_test::env();
        let owner = env.get_account(0);
        let token = Cep47HostRef::deploy(
            &env,
            Cep47InitArgs {
                name: NAME.to_string(),
                symbol: SYMBOL.to_string(),
                meta: meta::contract_meta(),
            },
        );

        (env, token, owner)
    }

    mod meta {
        use alloc::collections::BTreeMap;
        use alloc::string::ToString;

        use crate::cep47::Meta;

        pub fn contract_meta() -> Meta {
            let mut meta = BTreeMap::new();
            meta.insert("origin".to_string(), "fire".to_string());
            meta
        }

        pub fn red_dragon() -> Meta {
            let mut meta = BTreeMap::new();
            meta.insert("color".to_string(), "red".to_string());
            meta
        }

        pub fn blue_dragon() -> Meta {
            let mut meta = BTreeMap::new();
            meta.insert("color".to_string(), "blue".to_string());
            meta
        }

        pub fn black_dragon() -> Meta {
            let mut meta = BTreeMap::new();
            meta.insert("color".to_string(), "black".to_string());
            meta
        }

        pub fn gold_dragon() -> Meta {
            let mut meta = BTreeMap::new();
            meta.insert("color".to_string(), "gold".to_string());
            meta
        }
    }
}
