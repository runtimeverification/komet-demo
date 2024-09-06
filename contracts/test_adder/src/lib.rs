#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Address, Bytes, Env, Symbol};

mod komet;

mod adder_contract {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/adder.wasm"
    );
}

const ADDR_KEY: Symbol = symbol_short!("adder");

#[contract]
pub struct TestAdderContract;

#[contractimpl]
impl TestAdderContract {
    pub fn init(env: Env, adder_hash: Bytes) {
        let addr_bytes = b"adder_ctr_______________________";
        let adder = komet::create_contract(&env, &Bytes::from_array(&env, addr_bytes), &adder_hash);
        env.storage().instance().set(&ADDR_KEY, &adder);
    }

  
    pub fn test_add(env: Env, x: u32, y: u32) -> bool {
        // Retrieve the address of the `adder` contract from storage
        let adder: Address = env.storage().instance().get(&ADDR_KEY).unwrap();
      
        // Create a client for interacting with the `adder` contract
        let adder_client = adder_contract::Client::new(&env, &adder);


        // Call the `add` endpoint of the `adder` contract with the provided numbers
        let sum = adder_client.add(&x, &y);
      
        // Check if the returned sum matches the expected result
        sum == x as u64 + y as u64
    }
}

