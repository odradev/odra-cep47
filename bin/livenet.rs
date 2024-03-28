use dummy_cep47::cep47::Cep47HostRef;
use odra::casper_types::U256;
use odra::host::{Deployer, HostEnv, HostRef, HostRefLoader, NoArgs};
use odra::Address;
use std::str::FromStr;

fn main() {
    let env = odra_casper_livenet_env::env();

    let owner = env.caller();
    // let recipient: &str = "hash-2c4a6ce0da5d175e9638ec0830e01dd6cf5f4b1fbb0724f7d2d9de12b1e0f840";
    // let recipient = Address::from_str(recipient).unwrap();

    // Deploy new contract.
    let mut cep47 = deploy_new(&env);
    println!("contract address: {}", cep47.address().to_string());

    // // Uncomment to load existing contract.
    // let mut cep47 = _load(&env);

    // println!("contract name: {}", cep47.name());

    // env.set_gas(3_000_000_000u64);
    // token.transfer(recipient, U256::from(1000));

    // println!("Owner's balance: {:?}", cep47.balance_of(owner));
    // println!("Recipient's balance: {:?}", cep47.balance_of(recipient));
}

fn deploy_new(env: &HostEnv) -> Cep47HostRef {
    env.set_gas(400_000_000_000u64);
    Cep47HostRef::deploy(env, NoArgs)
}

fn _load(env: &HostEnv) -> Cep47HostRef {
    let address = "hash-d26fcbd2106e37be975d2045c580334a6d7b9d0a241c2358a4db970dfd516945";
    let address = Address::from_str(address).unwrap();
    Cep47HostRef::load(env, address)
}
