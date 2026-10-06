#![cfg(test)]

extern crate std;

use soroban_sdk::{ Address, Bytes, Env, String, testutils::Address as _ };

use crate::{ TestToken, TestTokenClient };

#[test]
fn test_fixed() {
    let env = Env::default();

    let token_wasm_hash = env.deployer().upload_contract_wasm(*include_bytes!("../../../target/wasm32v1-none/release/fungible_contract.wasm"));

    let contract_addr = env.register(TestToken, ());
    
    let client = TestTokenClient::new(&env, &contract_addr);

    env.mock_all_auths();

    client.init(&token_wasm_hash.to_bytes());    
    client.test_self_transfer_keeps_balance(&928124, &4297881);
}
