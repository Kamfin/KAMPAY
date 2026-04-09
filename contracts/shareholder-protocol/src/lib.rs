#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Map, Symbol};

#[contracttype]
pub enum DataKey {
    Admin,
    Token,           // USDC payment token
    KpayToken,       // KPAY share token
    TotalSupply,
    Balances,
    Treasury,
    TotalDistributed,
}

#[contract]
pub struct ShareholderProtocol;

#[contractimpl]
impl ShareholderProtocol {
    pub fn initialize(env: Env, admin: Address, token: Address, kpay_token: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialised");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::KpayToken, &kpay_token);
        env.storage().instance().set(&DataKey::TotalSupply, &0_i128);
        env.storage().instance().set(&DataKey::Treasury, &0_i128);
        env.storage().instance().set(&DataKey::TotalDistributed, &0_i128);
        env.storage()
            .instance()
            .set(&DataKey::Balances, &Map::<Address, i128>::new(&env));
    }

    /// Purchase KPAY shares by sending USDC.
    pub fn purchase_shares(env: Env, buyer: Address, amount: i128) {
        buyer.require_auth();
        let token: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        // Transfer USDC from buyer to contract (treasury)
        token::Client::new(&env, &token).transfer(
            &buyer,
            &env.current_contract_address(),
            &amount,
        );

        let treasury: i128 = env.storage().instance().get(&DataKey::Treasury).unwrap();
        env.storage().instance().set(&DataKey::Treasury, &(treasury + amount));

        // 1:1 KPAY minted per USDC for simplicity — pricing logic lives off-chain
        let mut balances: Map<Address, i128> =
            env.storage().instance().get(&DataKey::Balances).unwrap();
        let current = balances.get(buyer.clone()).unwrap_or(0);
        balances.set(buyer.clone(), current + amount);
        env.storage().instance().set(&DataKey::Balances, &balances);

        let supply: i128 = env.storage().instance().get(&DataKey::TotalSupply).unwrap();
        env.storage().instance().set(&DataKey::TotalSupply, &(supply + amount));

        env.events()
            .publish((Symbol::new(&env, "shares_purchased"),), (buyer, amount));
    }

    /// Distribute dividends proportionally to all shareholders.
    /// Called by admin (or cron agent) monthly.
    pub fn distribute_dividends(env: Env, caller: Address, distributable: i128) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        assert!(caller == admin, "unauthorised");
        caller.require_auth();

        let total_supply: i128 = env.storage().instance().get(&DataKey::TotalSupply).unwrap();
        assert!(total_supply > 0, "no shareholders");

        let balances: Map<Address, i128> =
            env.storage().instance().get(&DataKey::Balances).unwrap();
        let token: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let token_client = token::Client::new(&env, &token);

        for (holder, shares) in balances.iter() {
            let dividend = (shares * distributable) / total_supply;
            if dividend > 0 {
                token_client.transfer(&env.current_contract_address(), &holder, &dividend);
            }
        }

        let distributed: i128 = env.storage().instance().get(&DataKey::TotalDistributed).unwrap();
        env.storage()
            .instance()
            .set(&DataKey::TotalDistributed, &(distributed + distributable));

        env.events()
            .publish((Symbol::new(&env, "dividends_distributed"),), (distributable,));
    }

    pub fn get_shares(env: Env, holder: Address) -> i128 {
        let balances: Map<Address, i128> =
            env.storage().instance().get(&DataKey::Balances).unwrap();
        balances.get(holder).unwrap_or(0)
    }

    pub fn get_total_supply(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0)
    }
}
