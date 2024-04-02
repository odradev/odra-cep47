use std::str::FromStr;

use odra::Address;
use odra::casper_types::U256;
use odra::host::{Deployer, HostEnv, HostRef, HostRefLoader};

use odra_cep47::cep47::{Cep47HostRef, Cep47InitArgs};

fn main() {
    let env = odra_casper_livenet_env::env();

    let owner = env.caller();
    // let recipient: &str = "hash-2c4a6ce0da5d175e9638ec0830e01dd6cf5f4b1fbb0724f7d2d9de12b1e0f840";
    // let recipient = Address::from_str(recipient).unwrap();

    // Deploy new contract.
    let mut cep47 = deploy_new(&env);
    println!("contract address: {}", cep47.address().to_string());

    // Mint a token.
    let token_id = U256::from(2);
    env.set_gas(3_000_000_000u64);
    let tokens = cep47.mint(owner, vec![token_id], vec![Default::default()]);
    println!("Minted tokens: {:?}", tokens.unwrap_or_default());

    // // Uncomment to load existing contract.
    // let mut cep47 = _load(&env);

    // println!("contract name: {}", cep47.name());

    // env.set_gas(3_000_000_000u64);
    // token.transfer(recipient, U256::from(1000));

    // println!("Owner's balance: {:?}", cep47.balance_of(owner));
    // println!("Recipient's balance: {:?}", cep47.balance_of(recipient));
}

fn deploy_new(env: &HostEnv) -> Cep47HostRef {
    env.set_gas(250_000_000_000u64);
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
