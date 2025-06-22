#![no_std]

use soroban_sdk::{contract, contractimpl, log, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct CounterContract;

#[contractimpl]
impl CounterContract {
    pub fn read(env: Env) -> u32 {
        env.storage().instance().get(&COUNTER).unwrap_or(0)
    }

    pub fn increment(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        log!(&env, "count before: {}", count);

        count += 1;
        log!(&env, "count after: {}", count);

        env.storage().instance().set(&COUNTER, &count);
        env.storage().instance().extend_ttl(50, 100);

        count
    }

    pub fn decrement(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        log!(&env, "count before: {}", count);

        count = count.saturating_sub(1);
        log!(&env, "count after: {}", count);

        env.storage().instance().set(&COUNTER, &count);
        env.storage().instance().extend_ttl(50, 100);

        count
    }
}

mod test;
