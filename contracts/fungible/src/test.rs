#![cfg(test)]

extern crate std;

use soroban_sdk::{ testutils::Address as _, Address, Env, String };

use crate::contract::{ KometToken, KometTokenClient };

#[test]
fn initial_state() {
    let env = Env::default();

    let contract_addr = env.register(KometToken, (Address::generate(&env),));
    let client = KometTokenClient::new(&env, &contract_addr);
    
    let holder = Address::generate(&env);
    env.mock_all_auths();
    client.mint(&holder, &100);
    client.transfer(&holder, &holder, &50);
    assert_eq!(client.name(), String::from_str(&env, "KometToken"));
}

// Add more tests bellow
