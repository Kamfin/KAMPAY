#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Symbol};

#[contracttype]
pub enum DataKey {
    Worker,
    Token,
    RatePerSecond,
    StartTime,
    EndTime,
    Withdrawn,
    Paused,
}

#[contract]
pub struct StreamingPayment;

#[contractimpl]
impl StreamingPayment {
    pub fn initialize(
        env: Env,
        worker: Address,
        token: Address,
        rate_per_second: i128,
        start_time: u64,
        end_time: u64,
    ) {
        if env.storage().instance().has(&DataKey::Worker) {
            panic!("already initialised");
        }
        env.storage().instance().set(&DataKey::Worker, &worker);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::RatePerSecond, &rate_per_second);
        env.storage().instance().set(&DataKey::StartTime, &start_time);
        env.storage().instance().set(&DataKey::EndTime, &end_time);
        env.storage().instance().set(&DataKey::Withdrawn, &0_i128);
        env.storage().instance().set(&DataKey::Paused, &false);
    }

    pub fn earned(env: Env) -> i128 {
        let start: u64 = env.storage().instance().get(&DataKey::StartTime).unwrap();
        let end: u64 = env.storage().instance().get(&DataKey::EndTime).unwrap();
        let rate: i128 = env.storage().instance().get(&DataKey::RatePerSecond).unwrap();
        let now = env.ledger().timestamp();
        let elapsed = now.min(end).saturating_sub(start) as i128;
        elapsed * rate
    }

    pub fn withdraw(env: Env) {
        let worker: Address = env.storage().instance().get(&DataKey::Worker).unwrap();
        worker.require_auth();
        let paused: bool = env.storage().instance().get(&DataKey::Paused).unwrap();
        assert!(!paused, "stream is paused");

        let withdrawn: i128 = env.storage().instance().get(&DataKey::Withdrawn).unwrap();
        let claimable = Self::earned(env.clone()) - withdrawn;
        assert!(claimable > 0, "nothing to withdraw");

        env.storage().instance().set(&DataKey::Withdrawn, &(withdrawn + claimable));

        let token: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        token::Client::new(&env, &token).transfer(
            &env.current_contract_address(),
            &worker,
            &claimable,
        );

        env.events().publish((Symbol::new(&env, "withdrawn"),), (worker, claimable));
    }

    pub fn pause(env: Env, caller: Address) {
        caller.require_auth();
        env.storage().instance().set(&DataKey::Paused, &true);
    }

    pub fn resume(env: Env, caller: Address) {
        caller.require_auth();
        env.storage().instance().set(&DataKey::Paused, &false);
    }

    pub fn terminate(env: Env, caller: Address) {
        caller.require_auth();
        let end = env.ledger().timestamp();
        env.storage().instance().set(&DataKey::EndTime, &end);
        env.storage().instance().set(&DataKey::Paused, &true);
        env.events().publish((Symbol::new(&env, "terminated"),), ());
    }
}
