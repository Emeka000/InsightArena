// Integration tests for issue #1021:
// Verify get_event_prize_pool returns the correct live pool at every lifecycle stage.

use creator_event_manager::{CreatorEventManagerContract, CreatorEventManagerContractClient};
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token::{Client as TokenClient, StellarAssetClient},
    Address, Env, String, Vec,
};

fn create_token_contract<'a>(
    e: &Env,
    admin: &Address,
) -> (Address, StellarAssetClient<'a>, TokenClient<'a>) {
    let addr = e.register_stellar_asset_contract_v2(admin.clone()).address();
    (
        addr.clone(),
        StellarAssetClient::new(e, &addr),
        TokenClient::new(e, &addr),
    )
}

fn base_setup(env: &Env) -> (Address, Address, CreatorEventManagerContractClient, StellarAssetClient, TokenClient) {
    env.mock_all_auths();
    let admin = Address::generate(env);
    let (token_address, token_admin, token_client) = create_token_contract(env, &admin);
    let contract_id = env.register(CreatorEventManagerContract, ());
    let client = CreatorEventManagerContractClient::new(env, &contract_id);
    client.initialize(&token_address, &5u32);
    (contract_id, token_address, client, token_admin, token_client)
}

// ─── Test 1: pre-join – pool equals the creator seed ─────────────────────────

#[test]
fn test_pre_join_prize_pool() {
    let env = Env::default();
    let (contract_id, _token_address, client, token_admin, token_client) = base_setup(&env);

    let creator = Address::generate(&env);
    let seed_pool: i128 = 1_000;

    token_admin.mint(&creator, &seed_pool);
    token_client.approve(
        &creator,
        &contract_id,
        &seed_pool,
        &(env.ledger().sequence() + 200),
    );

    let event_id = client.create_oracle_event(
        &creator,
        &String::from_str(&env, "Pre-join Event"),
        &String::from_str(&env, ""),
        &seed_pool,
        &0i128,
        &(env.ledger().timestamp() + 86_400),
    );

    assert_eq!(
        client.get_event_prize_pool(&event_id),
        seed_pool,
        "pool before any joins should equal the seed"
    );
}

// ─── Test 2: post-join – pool grows by entry_fee × N ─────────────────────────

#[test]
fn test_post_join_prize_pool_growth() {
    let env = Env::default();
    let (contract_id, _token_address, client, token_admin, token_client) = base_setup(&env);

    let creator = Address::generate(&env);
    let entry_fee: i128 = 50;
    let initial_seed: i128 = 200;
    let n_users: i128 = 3;

    token_admin.mint(&creator, &initial_seed);
    token_client.approve(
        &creator,
        &contract_id,
        &initial_seed,
        &(env.ledger().sequence() + 200),
    );

    let event_id = client.create_oracle_event(
        &creator,
        &String::from_str(&env, "Growth Event"),
        &String::from_str(&env, ""),
        &initial_seed,
        &entry_fee,
        &(env.ledger().timestamp() + 86_400),
    );

    for _ in 0..n_users {
        let user = Address::generate(&env);
        token_admin.mint(&user, &entry_fee);
        token_client.approve(&user, &contract_id, &entry_fee, &(env.ledger().sequence() + 200));
        client.join_event(&user, &event_id);
    }

    assert_eq!(
        client.get_event_prize_pool(&event_id),
        initial_seed + n_users * entry_fee,
        "pool should equal seed + N * entry_fee after joins"
    );
}

// ─── Test 3: post-finalize – pool is 0 after distribution ────────────────────

#[test]
fn test_post_finalize_prize_pool() {
    let env = Env::default();
    let (contract_id, _token_address, client, token_admin, token_client) = base_setup(&env);

    let creator = Address::generate(&env);
    let user1 = Address::generate(&env);
    let seed: i128 = 400;
    let entry_fee: i128 = 100;

    token_admin.mint(&creator, &seed);
    token_admin.mint(&user1, &1_000i128);

    token_client.approve(&creator, &contract_id, &seed, &(env.ledger().sequence() + 500));
    token_client.approve(&user1, &contract_id, &entry_fee, &(env.ledger().sequence() + 500));

    let end_time = env.ledger().timestamp() + 3_600;
    let event_id = client.create_oracle_event(
        &creator,
        &String::from_str(&env, "Finalize Test"),
        &String::from_str(&env, ""),
        &seed,
        &entry_fee,
        &end_time,
    );

    client.join_event(&user1, &event_id);

    // Add a match so get_user_score has something to evaluate
    let match_id = client.create_oracle_match(
        &creator,
        &event_id,
        &String::from_str(&env, "A"),
        &String::from_str(&env, "B"),
        &(env.ledger().timestamp() + 1800),
        &1u32,
    );
    client.submit_prediction(&user1, &event_id, &match_id, &0u32, &1u32, &0u32);
    client.submit_oracle_result(&creator, &match_id, &0u32, &1u32, &0u32);

    env.ledger().with_mut(|l| {
        l.timestamp = end_time + 1;
    });

    let mut winners = Vec::new(&env);
    winners.push_back(user1.clone());
    let mut dist = Vec::new(&env);
    dist.push_back(100u32);

    client.finalize_event(&creator, &event_id, &winners, &dist);

    // After full distribution prize_pool must be 0
    assert_eq!(
        client.get_event_prize_pool(&event_id),
        0i128,
        "prize_pool should be 0 after finalization"
    );
}

// ─── Test 4: non-existent event → error ──────────────────────────────────────

#[test]
fn test_not_found_event_prize_pool() {
    let env = Env::default();
    let (_contract_id, _token_address, client, _token_admin, _token_client) = base_setup(&env);

    let result = client.try_get_event_prize_pool(&9_999u64);
    assert!(
        result.is_err(),
        "get_event_prize_pool with unknown id should return an error"
    );
}
