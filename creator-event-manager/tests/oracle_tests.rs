// Integration tests for issue #1024:
// Verify that Prediction::grade multiplies base points by the match's points_multiplier
// and that get_user_score returns the summed total across all matches.

use creator_event_manager::{CreatorEventManagerContract, CreatorEventManagerContractClient};
use soroban_sdk::{
    testutils::Address as _,
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

/// Spin up a contract with the given token; returns (contract_id, client).
fn setup(env: &Env) -> (Address, Address, CreatorEventManagerContractClient) {
    let admin = Address::generate(env);
    let (token_address, _, _) = create_token_contract(env, &admin);
    let contract_id = env.register(CreatorEventManagerContract, ());
    let client = CreatorEventManagerContractClient::new(env, &contract_id);
    env.mock_all_auths();
    client.initialize(&token_address, &5u32);
    (contract_id, token_address, client)
}

/// Create a free oracle event (no seed pool, no entry fee) ending in 24 h.
fn create_free_event(
    env: &Env,
    client: &CreatorEventManagerContractClient,
    creator: &Address,
) -> u64 {
    client.create_oracle_event(
        creator,
        &String::from_str(env, "Test Event"),
        &String::from_str(env, "Desc"),
        &0i128,
        &0i128,
        &(env.ledger().timestamp() + 86400),
    )
}

// ─── Test 1: correct result with 2× multiplier → base(100) × 2 = 200 ─────────

#[test]
fn test_double_result_points() {
    let env = Env::default();
    let (_contract_id, _token_address, client) = setup(&env);
    let creator = Address::generate(&env);
    let user = Address::generate(&env);

    let event_id = create_free_event(&env, &client, &creator);

    let match_id = client.create_oracle_match(
        &creator,
        &event_id,
        &String::from_str(&env, "Team A"),
        &String::from_str(&env, "Team B"),
        &(env.ledger().timestamp() + 3600),
        &2u32, // points_multiplier = 2
    );

    // User joins (free) and predicts Team A wins 2-0
    client.join_event(&user, &event_id);
    client.submit_prediction(&user, &event_id, &match_id, &0u32, &2u32, &0u32);

    // Actual result: Team A wins 3-1 (correct result, different score)
    client.submit_oracle_result(&creator, &match_id, &0u32, &3u32, &1u32);

    let score = client.get_user_score(&user, &event_id);
    // Correct result (not exact) → 100 × 2 = 200
    assert_eq!(score, 200i128);
}

// ─── Test 2: exact score with 2× multiplier → base(300) × 2 = 600 ────────────

#[test]
fn test_double_exact_score_points() {
    let env = Env::default();
    let (_contract_id, _token_address, client) = setup(&env);
    let creator = Address::generate(&env);
    let user = Address::generate(&env);

    let event_id = create_free_event(&env, &client, &creator);

    let match_id = client.create_oracle_match(
        &creator,
        &event_id,
        &String::from_str(&env, "Team A"),
        &String::from_str(&env, "Team B"),
        &(env.ledger().timestamp() + 3600),
        &2u32,
    );

    client.join_event(&user, &event_id);
    // User predicts exact: Team A wins 2-1
    client.submit_prediction(&user, &event_id, &match_id, &0u32, &2u32, &1u32);

    // Actual result: Team A wins 2-1 (exact match)
    client.submit_oracle_result(&creator, &match_id, &0u32, &2u32, &1u32);

    let score = client.get_user_score(&user, &event_id);
    // Exact score → 300 × 2 = 600
    assert_eq!(score, 600i128);
}

// ─── Test 3: exact score with 3× multiplier → 300 × 3 = 900 ─────────────────

#[test]
fn test_triple_exact_score_points() {
    let env = Env::default();
    let (_contract_id, _token_address, client) = setup(&env);
    let creator = Address::generate(&env);
    let user = Address::generate(&env);

    let event_id = create_free_event(&env, &client, &creator);

    let match_id = client.create_oracle_match(
        &creator,
        &event_id,
        &String::from_str(&env, "Team A"),
        &String::from_str(&env, "Team B"),
        &(env.ledger().timestamp() + 3600),
        &3u32, // points_multiplier = 3
    );

    client.join_event(&user, &event_id);
    client.submit_prediction(&user, &event_id, &match_id, &1u32, &0u32, &2u32);

    // Actual: Team B wins 0-2 (exact)
    client.submit_oracle_result(&creator, &match_id, &1u32, &0u32, &2u32);

    let score = client.get_user_score(&user, &event_id);
    // 300 × 3 = 900
    assert_eq!(score, 900i128);
}

// ─── Test 4: wrong prediction → 0 regardless of multiplier ───────────────────

#[test]
fn test_wrong_prediction_with_multiplier() {
    let env = Env::default();
    let (_contract_id, _token_address, client) = setup(&env);
    let creator = Address::generate(&env);
    let user = Address::generate(&env);

    let event_id = create_free_event(&env, &client, &creator);

    let match_id = client.create_oracle_match(
        &creator,
        &event_id,
        &String::from_str(&env, "Team A"),
        &String::from_str(&env, "Team B"),
        &(env.ledger().timestamp() + 3600),
        &2u32,
    );

    client.join_event(&user, &event_id);
    // User predicts Team B wins
    client.submit_prediction(&user, &event_id, &match_id, &1u32, &0u32, &3u32);

    // Actual: Team A wins
    client.submit_oracle_result(&creator, &match_id, &0u32, &2u32, &0u32);

    let score = client.get_user_score(&user, &event_id);
    assert_eq!(score, 0i128);
}

// ─── Test 5: two matches with multipliers 1 and 2; user correct on both ───────
// Expected total = 100×1 + 100×2 = 300

#[test]
fn test_mixed_multiplier_event() {
    let env = Env::default();
    let (_contract_id, _token_address, client) = setup(&env);
    let creator = Address::generate(&env);
    let user = Address::generate(&env);

    let event_id = create_free_event(&env, &client, &creator);
    let now = env.ledger().timestamp();

    let match_id1 = client.create_oracle_match(
        &creator,
        &event_id,
        &String::from_str(&env, "Team A"),
        &String::from_str(&env, "Team B"),
        &(now + 3600),
        &1u32, // multiplier 1
    );
    let match_id2 = client.create_oracle_match(
        &creator,
        &event_id,
        &String::from_str(&env, "Team C"),
        &String::from_str(&env, "Team D"),
        &(now + 7200),
        &2u32, // multiplier 2
    );

    client.join_event(&user, &event_id);

    // Predict correct result (not exact) for both
    client.submit_prediction(&user, &event_id, &match_id1, &0u32, &1u32, &0u32); // Team A wins
    client.submit_prediction(&user, &event_id, &match_id2, &1u32, &2u32, &0u32); // Team D wins

    // Actual results
    client.submit_oracle_result(&creator, &match_id1, &0u32, &2u32, &1u32); // Team A wins 2-1
    client.submit_oracle_result(&creator, &match_id2, &1u32, &3u32, &0u32); // Team D wins 3-0

    let score = client.get_user_score(&user, &event_id);
    // 100×1 + 100×2 = 300
    assert_eq!(score, 300i128);
}
