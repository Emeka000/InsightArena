// Integration tests for issue #1023:
// Verify join_event correctly transfers the entry fee and grows prize_pool.

use creator_event_manager::{CreatorEventManagerContract, CreatorEventManagerContractClient};
use soroban_sdk::{
    testutils::Address as _,
    token::{Client as TokenClient, StellarAssetClient},
    Address, Env, String,
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

fn make_env() -> (
    Env,
    Address, // contract_id
    CreatorEventManagerContractClient<'static>,
    Address, // token_address
    StellarAssetClient<'static>,
    TokenClient<'static>,
) {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let (token_address, token_admin, token_client) = create_token_contract(&env, &admin);

    let contract_id = env.register(CreatorEventManagerContract, ());
    let client = CreatorEventManagerContractClient::new(&env, &contract_id);
    client.initialize(&token_address, &5u32);

    (env, contract_id, client, token_address, token_admin, token_client)
}

// ─── Test 1: happy path ───────────────────────────────────────────────────────
// User has enough XLM → fee is transferred once, prize_pool grows.

#[test]
fn test_join_happy_path() {
    let (env, contract_id, client, _token_address, token_admin, token_client) = make_env();

    let creator = Address::generate(&env);
    let user = Address::generate(&env);

    let entry_fee = 100i128;
    let seed_pool = 500i128;

    token_admin.mint(&creator, &seed_pool);
    token_admin.mint(&user, &1_000i128);

    // Creator approves contract to pull seed pool
    token_client.approve(
        &creator,
        &contract_id,
        &seed_pool,
        &(env.ledger().sequence() + 200),
    );

    let end_time = env.ledger().timestamp() + 86_400;
    let event_id = client.create_oracle_event(
        &creator,
        &String::from_str(&env, "Paid Event"),
        &String::from_str(&env, "Desc"),
        &seed_pool,
        &entry_fee,
        &end_time,
    );

    // Prize pool before join
    assert_eq!(client.get_event_prize_pool(&event_id), seed_pool);

    let user_balance_before = token_client.balance(&user);

    // User approves and joins
    token_client.approve(
        &user,
        &contract_id,
        &entry_fee,
        &(env.ledger().sequence() + 200),
    );
    client.join_event(&user, &event_id);

    // Fee transferred exactly once
    assert_eq!(
        user_balance_before - token_client.balance(&user),
        entry_fee
    );
    // Prize pool grew by exactly entry_fee
    assert_eq!(
        client.get_event_prize_pool(&event_id),
        seed_pool + entry_fee
    );
}

// ─── Test 2: insufficient balance → join fails ────────────────────────────────

#[test]
fn test_join_insufficient_funds() {
    let (env, contract_id, client, _token_address, token_admin, token_client) = make_env();

    let creator = Address::generate(&env);
    let user = Address::generate(&env);

    let entry_fee = 100i128;

    token_admin.mint(&creator, &0i128); // creator funds nothing (no seed)
    // Fund user with one token less than required
    token_admin.mint(&user, &(entry_fee - 1));

    let end_time = env.ledger().timestamp() + 86_400;
    let event_id = client.create_oracle_event(
        &creator,
        &String::from_str(&env, "Paid Event"),
        &String::from_str(&env, "Desc"),
        &0i128,      // no seed pool
        &entry_fee,
        &end_time,
    );

    // Attempt to join without sufficient funds – must fail
    let result = client.try_join_event(&user, &event_id);
    assert!(result.is_err(), "join with insufficient balance should fail");

    // Prize pool must remain unchanged
    assert_eq!(client.get_event_prize_pool(&event_id), 0i128);
}

// ─── Test 3: zero entry fee → no token transfer, balances unchanged ───────────

#[test]
fn test_join_zero_fee() {
    let (env, _contract_id, client, _token_address, token_admin, token_client) = make_env();

    let creator = Address::generate(&env);
    let user = Address::generate(&env);

    let initial_balance = 500i128;
    token_admin.mint(&user, &initial_balance);

    let end_time = env.ledger().timestamp() + 86_400;
    let event_id = client.create_oracle_event(
        &creator,
        &String::from_str(&env, "Free Event"),
        &String::from_str(&env, "Desc"),
        &0i128, // no seed pool
        &0i128, // zero fee
        &end_time,
    );

    client.join_event(&user, &event_id);

    // User balance must be unchanged (no token transfer)
    assert_eq!(token_client.balance(&user), initial_balance);
    // Prize pool stays at 0
    assert_eq!(client.get_event_prize_pool(&event_id), 0i128);
}
