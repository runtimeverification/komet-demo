#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct AdderContract;

#[contractimpl]
impl AdderContract {
    pub fn add(_env: Env, first: u32, second: u32) -> u64 {
       first as u64 + second as u64
    }
}
