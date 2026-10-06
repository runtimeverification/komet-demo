#![no_std]

use soroban_sdk::{
    contract, contractimpl, symbol_short,
    token::{StellarAssetClient},
    Address, Bytes, Env, Symbol,
};

use stellar_tokens::fungible::FungibleTokenClient;

use crate::action::{ACTION_SIZE, Action, MAX_INPUT_LEN};

mod komet;
mod action;

#[contract]
pub struct TestTokenInvariants;

const TOKEN_ADDR: &[u8; 32] = b"kmt_token_______________________";

const USER_ADDRS: [&[u8; 32]; 4] = [
    b"kmt_user_0______________________",
    b"kmt_user_1______________________",
    b"kmt_user_2______________________",
    b"kmt_user_3______________________",
];

const TOK_ADDR_KEY: Symbol = symbol_short!("tok_addr");

fn token_address(env: &Env) -> Address {
    env.storage().instance().get(&TOK_ADDR_KEY).unwrap()
}

fn token_client(env: &Env) -> FungibleTokenClient<'_> {
    FungibleTokenClient::new(env, &token_address(env))
}

fn token_admin_client(env: &Env) -> StellarAssetClient<'_> {
    StellarAssetClient::new(env, &token_address(env))
}

/// Picks an account address from `USER_ADDRS` using a fuzzed index.
fn account(env: &Env, user_index: usize) -> Address {
    let bytes = USER_ADDRS[user_index % USER_ADDRS.len()];
    komet::address_from_bytes(env, bytes, false)
}

const NUM_ACCOUNTS: usize = USER_ADDRS.len() + 1;
const APPROVAL_LIFETIME_LEDGERS: u32 = 100;

fn all_balances(env: &Env) -> [i128; NUM_ACCOUNTS] {
    let token = token_client(env);
    let mut balances = [0; NUM_ACCOUNTS];
    for (account_index, balance) in balances.iter_mut().enumerate() {
        *balance = token.balance(&account(env, account_index));
    }
    balances
}

fn all_allowances(env: &Env) -> [i128; NUM_ACCOUNTS] {
    let token = token_client(env);
    let owner = env.current_contract_address();
    let mut allowances = [0; NUM_ACCOUNTS];
    for (account_index, allowance) in allowances.iter_mut().enumerate() {
        *allowance = token.allowance(&owner, &account(env, account_index));
    }
    allowances
}

#[contractimpl]
impl TestTokenInvariants {
    pub fn init(env: Env, hash_token: Bytes) {
        let self_address = env.current_contract_address();
        let token_addr =
            komet::create_contract(&env, TOKEN_ADDR, hash_token.as_object(), Some((self_address,)));
        env.storage().instance().set(&TOK_ADDR_KEY, &token_addr);
    }

    pub fn test_invariants(env: Env, encoded_actions_bs: Bytes) -> bool {
        let input_len = encoded_actions_bs.len() as usize;
        let mut buffer = [0u8; MAX_INPUT_LEN];
        encoded_actions_bs
            .slice(0..input_len as u32)
            .copy_into_slice(&mut buffer[..input_len]);

        let actions: &[u8] = &buffer[..input_len];

        let token_admin = token_admin_client(&env);
        let token = token_client(&env);
        let test_contract = env.current_contract_address();

        for chunk in actions.chunks_exact(ACTION_SIZE) {
            let action = Action::decode(chunk, NUM_ACCOUNTS);

            // State before
            let total_supply_before = token.total_supply();
            let balances_before = all_balances(&env);
            let allowances_before = all_allowances(&env);
            
            // Execute action
            let succeeded = match action {
                Action::Mint { to, amount } => token_admin
                    .try_mint(&account(&env, to), &amount)
                    .is_ok(),

                Action::Transfer { to, amount } => token
                    .try_transfer(&test_contract, &account(&env, to), &amount)
                    .is_ok(),

                Action::Approve { spender, amount } => {
                    let expiration_ledger = env.ledger().sequence() + APPROVAL_LIFETIME_LEDGERS;
                    token
                        .try_approve(&test_contract, &account(&env, spender), &amount, &expiration_ledger)
                        .is_ok()
                }

                Action::TransferFrom { to, amount } => token
                    .try_transfer_from(&test_contract, &test_contract, &account(&env, to), &amount)
                    .is_ok(),
            };
            
            // State after
            let total_supply_after = token.total_supply();
            let balances_after = all_balances(&env);
            let allowances_after: [i128; 5] = all_allowances(&env);
            

            // All balances and allowances non-zero
            assert!(balances_after.iter().all(|x| *x >= 0));
            assert!(allowances_after.iter().all(|x| *x >= 0));

            if succeeded {
                // TotalSupply' = TotalSupply + mint_amount
                let supply_change = match action {
                    Action::Mint {amount, to: _ } => amount,
                    _ => 0,
                };

                assert_eq!(total_supply_before + supply_change, total_supply_after);
            } else {
                // Total supply, balances, allowances remain constant
                assert_eq!(total_supply_before, total_supply_after);
                assert_eq!(balances_before, balances_after);
                assert_eq!(allowances_before, allowances_after);
            }
        }
        true
    }
}

mod test;