use std::str::FromStr;

use odra::casper_types::U256;
use odra::host::{Deployer, HostEnv, HostRef, HostRefLoader};
use odra::Address;

use odra_cep47::cep47::{Cep47HostRef, Cep47InitArgs, Meta};

fn main() {
    let env = odra_casper_livenet_env::env();

    let owner = env.caller();
    let recipient: &str = "hash-1856e4a0b23c70b64e4509987680de0d99145fa0cdc71ad9b78760e18ff0deec";
    let recipient = Address::from_str(recipient).unwrap();

    // Deploy new contract.
    let mut cep47 = deploy_new(&env);
    println!("contract address: {}", cep47.address().to_string());

    // // Uncomment to load existing contract.
    // let mut cep47 = _load(&env);

    // Mint a couple of tokens.
    env.set_gas(6_000_000_000u64);
    let token_ids = vec![U256::from(1), U256::from(2), U256::from(3)];
    let token_metas = token_metas();
    let tokens = cep47.mint(owner, token_ids, token_metas);
    println!("Minted tokens: {:?}", tokens.unwrap_or_default());

    // Transfer tokens.
    env.set_gas(3_000_000_000u64);
    let _ = cep47.transfer(recipient, vec![U256::from(1)]);
}

fn token_metas() -> Vec<Meta> {
    let mut token_metas = Vec::new();
    let mut token_meta = Meta::new();
    token_meta.insert("name".to_string(), "Odra big logo #1".to_string());
    token_meta.insert(
        "description".to_string(),
        "NFT of Odra framework logo".to_string(),
    );
    token_meta.insert(
        "image".to_string(),
        "https://avatars.githubusercontent.com/u/101402547".to_string(),
    );
    token_metas.push(token_meta.clone());

    token_meta.insert("name".to_string(), "Odra big logo #2".to_string());
    token_metas.push(token_meta.clone());

    token_meta.insert("name".to_string(), "Odra big logo #3".to_string());
    token_metas.push(token_meta.clone());

    token_metas
}

fn deploy_new(env: &HostEnv) -> Cep47HostRef {
    env.set_gas(450_000_000_000u64);
    let args = Cep47InitArgs {
        name: "PlasNFT".to_string(),
        symbol: "PLS".to_string(),
        meta: Default::default(),
    };
    Cep47HostRef::deploy(env, args)
}

fn _load(env: &HostEnv) -> Cep47HostRef {
    let address = "hash-8964243df4742ba8d811691f2b046dbc2f05fefa19f8675146f534080dcfc167";
    let address = Address::from_str(address).unwrap();
    Cep47HostRef::load(env, address)
}
