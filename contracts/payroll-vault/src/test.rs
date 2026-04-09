#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token::{Client as TokenClient, StellarAssetClient},
    Address, Env,
};

/// Helper: deploy a native Stellar asset and return (token_address, admin_client)
fn create_token(env: &Env, admin: &Address) -> (Address, StellarAssetClient) {
    let token_id = env.register_stellar_asset_contract_v2(admin.clone());
    let asset_client = StellarAssetClient::new(env, &token_id.address());
    (token_id.address(), asset_client)
}

#[test]
fn test_deposit_and_get_balance() {
    let env = Env::default();
    env.mock_all_auths();

    let owner = Address::generate(&env);
    let depositor = Address::generate(&env);

    // Deploy token and mint to depositor
    let (token_addr, asset_client) = create_token(&env, &owner);
    asset_client.mint(&depositor, &10_000);

    // Deploy vault
    let vault_id = env.register(PayrollVault, ());
    let vault = PayrollVaultClient::new(&env, &vault_id);

    vault.initialize(&owner, &token_addr, &500_i128);

    // Deposit 1_000 stroops
    vault.deposit(&depositor, &1_000);

    assert_eq!(vault.get_balance(), 1_000);
}

#[test]
fn test_create_stream_and_withdraw() {
    let env = Env::default();
    env.mock_all_auths();

    let owner = Address::generate(&env);
    let worker = Address::generate(&env);

    let (token_addr, asset_client) = create_token(&env, &owner);
    // Fund the vault via owner deposit
    asset_client.mint(&owner, &100_000);

    let vault_id = env.register(PayrollVault, ());
    let vault = PayrollVaultClient::new(&env, &vault_id);

    vault.initialize(&owner, &token_addr, &100_i128);
    vault.deposit(&owner, &100_000);

    // Stream: 10 stroops/second, runs for 100 seconds
    let start: u64 = 1_000;
    let end: u64 = 1_100;
    env.ledger().set_timestamp(start);

    vault.create_stream(&0_u32, &worker, &10_i128, &start, &end);

    // Advance time by 50 seconds — worker should have earned 500
    env.ledger().set_timestamp(start + 50);

    let earned_before = vault.earned(&0_u32);
    assert_eq!(earned_before, 500);

    let worker_token = TokenClient::new(&env, &token_addr);
    let balance_before = worker_token.balance(&worker);

    vault.withdraw(&0_u32);

    let balance_after = worker_token.balance(&worker);
    assert_eq!(balance_after - balance_before, 500);
    assert_eq!(vault.get_balance(), 100_000 - 500);
}
