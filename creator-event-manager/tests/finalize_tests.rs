// Integration test for issue #1027:
// Verify that entry-fee contributions accumulate in prize_pool before finalize_event
// distributes it, and that integer-division remainder goes to the creator.

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

// ─── Main accumulation + finalize test ───────────────────────────────────────
//
// Setup:
//   seed_pool  = 500 XLM
//   entry_fee  = 100 XLM
//   participants = 3 → prize_pool = 500 + 300 = 800 XLM
//
// Scoring:
//   user1 – predicts exact score  → 300 pts (rank 1)
//   user2 – predicts correct result → 100 pts (rank 2)
//   user3 – wrong prediction      →   0 pts (rank 3)
//
// Distribution: [60, 40]
//   user1 → 800 * 60 / 100 = 480
//   user2 → 800 * 40 / 100 = 320
//   remainder = 800 - 480 - 320 = 0  (sent to creator when > 0)

#[test]
fn test_entry_fee_grows_prize_pool_before_finalize() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let (token_address, token_admin, token_client) = create_token_contract(&env, &admin);

    let contract_id = env.register(CreatorEventManagerContract, ());
    let client = CreatorEventManagerContractClient::new(&env, &contract_id);
    client.initialize(&token_address, &5u32);

    let creator = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let user3 = Address::generate(&env);

    let seed_pool: i128 = 500;
    let entry_fee: i128 = 100;
    let initial_balance: i128 = 10_000;

    token_admin.mint(&creator, &(initial_balance + seed_pool));
    token_admin.mint(&user1, &initial_balance);
    token_admin.mint(&user2, &initial_balance);
    token_admin.mint(&user3, &initial_balance);

    // Creator approves the contract to pull the seed pool
    token_client.approve(
        &creator,
        &contract_id,
        &seed_pool,
        &(env.ledger().sequence() + 1000),
    );

    let end_time = env.ledger().timestamp() + 7_200;

    let event_id = client.create_oracle_event(
        &creator,
        &String::from_str(&env, "Championship"),
        &String::from_str(&env, "Predict the winner"),
        &seed_pool,
        &entry_fee,
        &end_time,
    );

    // ── 3 participants join ──────────────────────────────────────────────────
    for user in [&user1, &user2, &user3] {
        token_client.approve(user, &contract_id, &entry_fee, &(env.ledger().sequence() + 1000));
        client.join_event(user, &event_id);
    }

    // Prize pool must now be seed + 3 × entry_fee = 800
    let accumulated = client.get_event_prize_pool(&event_id);
    assert_eq!(accumulated, seed_pool + 3 * entry_fee, "prize_pool should equal seed + N * entry_fee");

    // ── Create one match ─────────────────────────────────────────────────────
    let match_id = client.create_oracle_match(
        &creator,
        &event_id,
        &String::from_str(&env, "Team A"),
        &String::from_str(&env, "Team B"),
        &(env.ledger().timestamp() + 3600),
        &1u32,
    );

    // user1 predicts exact: Team A wins 2-1
    client.submit_prediction(&user1, &event_id, &match_id, &0u32, &2u32, &1u32);
    // user2 predicts correct result, different score
    client.submit_prediction(&user2, &event_id, &match_id, &0u32, &3u32, &0u32);
    // user3 predicts wrong result
    client.submit_prediction(&user3, &event_id, &match_id, &1u32, &0u32, &3u32);

    // Submit actual result: Team A wins 2-1
    client.submit_oracle_result(&creator, &match_id, &0u32, &2u32, &1u32);

    // Advance ledger time past end_time
    env.ledger().with_mut(|l| {
        l.timestamp = end_time + 1;
    });

    // Verify scores
    assert_eq!(client.get_user_score(&user1, &event_id), 300i128); // exact
    assert_eq!(client.get_user_score(&user2, &event_id), 100i128); // correct result
    assert_eq!(client.get_user_score(&user3, &event_id), 0i128);   // wrong

    // ── Finalize: top-2 split [60, 40] ──────────────────────────────────────
    let mut winners = Vec::new(&env);
    winners.push_back(user1.clone()); // rank 1 (300 pts)
    winners.push_back(user2.clone()); // rank 2 (100 pts)

    let mut dist = Vec::new(&env);
    dist.push_back(60u32);
    dist.push_back(40u32);

    client.finalize_event(&creator, &event_id, &winners, &dist);

    let total_pool: i128 = 800;

    // user1: paid entry_fee, gets 60 % of 800 = 480
    assert_eq!(
        token_client.balance(&user1),
        initial_balance - entry_fee + (total_pool * 60 / 100),
        "user1 (rank 1) should receive 480"
    );
    // user2: paid entry_fee, gets 40 % of 800 = 320
    assert_eq!(
        token_client.balance(&user2),
        initial_balance - entry_fee + (total_pool * 40 / 100),
        "user2 (rank 2) should receive 320"
    );
    // user3: paid entry_fee, wins nothing
    assert_eq!(
        token_client.balance(&user3),
        initial_balance - entry_fee,
        "user3 (wrong prediction) should receive nothing"
    );

    // 60 + 40 = 100 % → no remainder; prize_pool should be 0 after finalize
    assert_eq!(client.get_event_prize_pool(&event_id), 0i128);

    // Total distributed = 480 + 320 = 800 = seed + 3 × entry_fee  ✓
    let total_distributed = (total_pool * 60 / 100) + (total_pool * 40 / 100);
    assert_eq!(total_distributed, total_pool);
}

// ─── Remainder test: distribution sums to < 100 % → remainder to creator ─────
//
// pool = 100,  dist = [70]  →  winner gets 70,  creator gets 30

#[test]
fn test_remainder_goes_to_creator() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let (token_address, token_admin, token_client) = create_token_contract(&env, &admin);

    let contract_id = env.register(CreatorEventManagerContract, ());
    let client = CreatorEventManagerContractClient::new(&env, &contract_id);
    client.initialize(&token_address, &5u32);

    let creator = Address::generate(&env);
    let user1 = Address::generate(&env);

    let seed_pool: i128 = 100;
    let initial_creator: i128 = 10_000;

    token_admin.mint(&creator, &(initial_creator + seed_pool));
    token_admin.mint(&user1, &1_000i128);

    token_client.approve(
        &creator,
        &contract_id,
        &seed_pool,
        &(env.ledger().sequence() + 1000),
    );

    let end_time = env.ledger().timestamp() + 3_600;

    let event_id = client.create_oracle_event(
        &creator,
        &String::from_str(&env, "Remainder Test"),
        &String::from_str(&env, ""),
        &seed_pool,
        &0i128, // free join
        &end_time,
    );

    client.join_event(&user1, &event_id);

    let match_id = client.create_oracle_match(
        &creator,
        &event_id,
        &String::from_str(&env, "X"),
        &String::from_str(&env, "Y"),
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
    dist.push_back(70u32); // only 70 %, remainder 30 % → creator

    let creator_balance_before = token_client.balance(&creator);
    client.finalize_event(&creator, &event_id, &winners, &dist);

    // user1 gets 70 % of 100 = 70
    assert_eq!(token_client.balance(&user1), 1_000 + 70);
    // creator gets the 30 remainder
    assert_eq!(
        token_client.balance(&creator),
        creator_balance_before + 30,
        "integer-division remainder should go to creator"
    );
}
