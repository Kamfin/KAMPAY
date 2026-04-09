
#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, token, Address, Env, Map, Symbol,
};

// ── Storage keys ────────────────────────────────────────────────────────────

#[contracttype]
pub enum DataKey {
    Owner,
    Token,
    Balance,
    Threshold,
    Streams,
}

// ── Stream record ────────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone)]
pub struct Stream {
    pub worker: Address,
    pub rate_per_second: i128,
    pub start_time: u64,
    pub end_time: u64,
    pub withdrawn: i128,
    pub active: bool,
}

// ── Contract ─────────────────────────────────────────────────────────────────

#[contract]
pub struct PayrollVault;

#[contractimpl]
impl PayrollVault {
    /// Initialise the vault. Must be called once after deployment.
    pub fn initialize(env: Env, owner: Address, token: Address, threshold: i128) {
        if env.storage().instance().has(&DataKey::Owner) {
            panic!("already initialised");
        }
        env.storage().instance().set(&DataKey::Owner, &owner);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::Balance, &0_i128);
        env.storage().instance().set(&DataKey::Threshold, &threshold);
        env.storage()
            .instance()
            .set(&DataKey::Streams, &Map::<u32, Stream>::new(&env));
    }

    /// Deposit funds into the vault.
    pub fn deposit(env: Env, from: Address, amount: i128) {
        from.require_auth();
        let token: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        token::Client::new(&env, &token).transfer(&from, &env.current_contract_address(), &amount);
        let balance: i128 = env.storage().instance().get(&DataKey::Balance).unwrap();
        env.storage()
            .instance()
            .set(&DataKey::Balance, &(balance + amount));
        env.events()
            .publish((Symbol::new(&env, "deposit"),), (from, amount));
    }

    /// Create a streaming payment for a worker.
    pub fn create_stream(
        env: Env,
        stream_id: u32,
        worker: Address,
        rate_per_second: i128,
        start_time: u64,
        end_time: u64,
    ) {
        let owner: Address = env.storage().instance().get(&DataKey::Owner).unwrap();
        owner.require_auth();

        let stream = Stream {
            worker: worker.clone(),
            rate_per_second,
            start_time,
            end_time,
            withdrawn: 0,
            active: true,
        };

        let mut streams: Map<u32, Stream> =
            env.storage().instance().get(&DataKey::Streams).unwrap();
        streams.set(stream_id, stream);
        env.storage().instance().set(&DataKey::Streams, &streams);

        env.events()
            .publish((Symbol::new(&env, "stream_created"),), (stream_id, worker));
    }

    /// Worker withdraws their earned balance.
    pub fn withdraw(env: Env, stream_id: u32) {
        let mut streams: Map<u32, Stream> =
            env.storage().instance().get(&DataKey::Streams).unwrap();
        let mut stream = streams.get(stream_id).expect("stream not found");

        stream.worker.require_auth();
        assert!(stream.active, "stream is not active");

        let now = env.ledger().timestamp();
        let elapsed = now.min(stream.end_time).saturating_sub(stream.start_time) as i128;
        let earned = elapsed * stream.rate_per_second;
        let claimable = earned - stream.withdrawn;
        assert!(claimable > 0, "nothing to withdraw");

        let balance: i128 = env.storage().instance().get(&DataKey::Balance).unwrap();
        assert!(balance >= claimable, "insufficient vault balance");

        stream.withdrawn += claimable;
        streams.set(stream_id, stream.clone());
        env.storage().instance().set(&DataKey::Streams, &streams);
        env.storage()
            .instance()
            .set(&DataKey::Balance, &(balance - claimable));

        let token: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        token::Client::new(&env, &token).transfer(
            &env.current_contract_address(),
            &stream.worker,
            &claimable,
        );

        // Emit VaultLow event if balance drops below threshold
        let threshold: i128 = env.storage().instance().get(&DataKey::Threshold).unwrap();
        if balance - claimable < threshold {
            env.events()
                .publish((Symbol::new(&env, "vault_low"),), (balance - claimable,));
        }

        env.events()
            .publish((Symbol::new(&env, "payment_made"),), (stream_id, claimable));
    }

    /// Cancel a stream (owner only).
    pub fn cancel_stream(env: Env, stream_id: u32) {
        let owner: Address = env.storage().instance().get(&DataKey::Owner).unwrap();
        owner.require_auth();

        let mut streams: Map<u32, Stream> =
            env.storage().instance().get(&DataKey::Streams).unwrap();
        let mut stream = streams.get(stream_id).expect("stream not found");
        stream.active = false;
        streams.set(stream_id, stream);
        env.storage().instance().set(&DataKey::Streams, &streams);

        env.events()
            .publish((Symbol::new(&env, "stream_cancelled"),), (stream_id,));
    }

    /// Returns the current vault balance.
    pub fn get_balance(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Balance).unwrap_or(0)
    }

    /// Returns how much a worker has earned so far (including already withdrawn).
    pub fn earned(env: Env, stream_id: u32) -> i128 {
        let streams: Map<u32, Stream> =
            env.storage().instance().get(&DataKey::Streams).unwrap();
        let stream = streams.get(stream_id).expect("stream not found");
        let now = env.ledger().timestamp();
        let elapsed = now.min(stream.end_time).saturating_sub(stream.start_time) as i128;
        elapsed * stream.rate_per_second
    }
}

mod test;
