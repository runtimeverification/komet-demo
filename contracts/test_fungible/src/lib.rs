#![no_std]

use soroban_sdk::{
    contract, contractimpl, symbol_short, token::StellarAssetClient, Address, Bytes, Env, Symbol,
};

use stellar_tokens::fungible::FungibleTokenClient;

mod komet;

#[contract]
pub struct TestToken;

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

/// Picks a user address from `USER_ADDRS` using a fuzzed index.
fn user(env: &Env, user_index: u32) -> Address {
    let bytes = USER_ADDRS[(user_index as usize) % USER_ADDRS.len()];
    komet::address_from_bytes(env, bytes, false)
}

#[contractimpl]
impl TestToken {
    pub fn init(env: Env, hash_token: Bytes) {
        let self_address = env.current_contract_address();
        let token_addr =
            komet::create_contract(&env, TOKEN_ADDR, hash_token.as_object(), Some((self_address,)));
        env.storage().instance().set(&TOK_ADDR_KEY, &token_addr);
    }

    /// Minting a non-negative amount increases the recipient's balance by exactly that amount.
    pub fn test_mint_adds_exact_amount(env: Env, recipient_index: u32, mint_amount: i128) -> bool {
        if mint_amount < 0 {
            return true;
        }
        let recipient = user(&env, recipient_index);
        let balance_before = token_client(&env).balance(&recipient);

        token_admin_client(&env).mint(&recipient, &mint_amount);

        token_client(&env).balance(&recipient) == balance_before + mint_amount
    }

    /// A valid transfer moves exactly `transfer_amount` and conserves the sum of the two balances.
    pub fn test_valid_transfer(
        env: Env,
        // sender_index: u32,
        receiver_index: u32,
        initial_balance: i128,
        transfer_amount: i128,
    ) -> bool {
        if !( 0 <= transfer_amount && transfer_amount <= initial_balance ) {
            return true;
        }
        // let sender = user(&env, sender_index);
        let sender = env.current_contract_address();
        let receiver = user(&env, receiver_index);
        token_admin_client(&env).mint(&sender, &initial_balance);

        let token = token_client(&env);
        
        let sender_before = token.balance(&sender);
        let receiver_before = token.balance(&receiver);
        let supply_before = token.total_supply();

        token.transfer(&sender, &receiver, &transfer_amount);

        let sender_after = token.balance(&sender);
        let receiver_after = token.balance(&receiver);
        let supply_after = token.total_supply();

        supply_before == supply_after
            && sender_after == sender_before - transfer_amount
            && receiver_after == receiver_before + transfer_amount
    }

    /// Transferring to yourself leaves your balance unchanged.
    pub fn test_self_transfer_keeps_balance(
        env: Env,
        initial_balance: i128,
        transfer_amount: i128,
    ) -> bool {
        if !( 0 <= transfer_amount && transfer_amount <= initial_balance ) {
            return true;
        }
        let holder = env.current_contract_address();
        token_admin_client(&env).mint(&holder, &initial_balance);

        let token = token_client(&env);
        let balance_before = token.balance(&holder);

        token.transfer(&holder, &holder, &transfer_amount);

        token.balance(&holder) == balance_before
    }

    /// Transferring more than the balance fails and leaves both balances untouched.
    pub fn test_transfer_over_balance_fails(
        env: Env,
        receiver_index: u32,
        initial_balance: i128,
        transfer_amount: i128,
    ) -> bool {
        if initial_balance < 0 || transfer_amount <= initial_balance {
            return true;
        }
        let sender = env.current_contract_address();
        let receiver = user(&env, receiver_index);
        token_admin_client(&env).mint(&sender, &initial_balance);

        let token = token_client(&env);
        let receiver_before = token.balance(&receiver);
        let transfer_result = token.try_transfer(&sender, &receiver, &transfer_amount);

        transfer_result.is_err()
            && token.balance(&sender) == initial_balance
            && token.balance(&receiver) == receiver_before
    }

}

mod test;
