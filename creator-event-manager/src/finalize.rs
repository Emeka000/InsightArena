use soroban_sdk::{Address, Env, Vec};

use crate::storage_types::OracleEvent;
use crate::token::TokenHelper;
use crate::DataKey;

/// Distribute `event.prize_pool` to `winners` according to `reward_distribution` percentages.
///
/// `winners` must be pre-sorted highest-score-first by the caller.
/// Any integer-division remainder is sent to the event creator.
pub(crate) fn do_finalize(
    env: &Env,
    creator: &Address,
    event_id: u64,
    winners: &Vec<Address>,
    reward_distribution: &Vec<u32>,
) {
    let mut event: OracleEvent = env
        .storage()
        .persistent()
        .get(&DataKey::OracleEvent(event_id))
        .expect("event_not_found");

    assert!(event.creator == *creator, "Only creator can finalize");
    assert!(!event.is_finalized, "already_finalized");
    assert!(
        env.ledger().timestamp() >= event.end_time,
        "event_not_ended"
    );

    let token_address: Address = env
        .storage()
        .instance()
        .get(&DataKey::TokenAddress)
        .expect("Token not set");

    let total_pool = event.prize_pool;
    let n_dist = reward_distribution.len();
    let n_win = winners.len();
    let pairs = if n_dist < n_win { n_dist } else { n_win };

    let mut distributed: i128 = 0;
    for i in 0..pairs {
        let pct = reward_distribution.get(i).unwrap() as i128;
        let amount = (total_pool * pct) / 100;
        let winner = winners.get(i).unwrap();
        if amount > 0 {
            TokenHelper::distribute_winnings(env, &token_address, &winner, amount)
                .expect("distribute failed");
        }
        distributed += amount;
    }

    let remainder = total_pool - distributed;
    if remainder > 0 {
        TokenHelper::distribute_winnings(env, &token_address, creator, remainder)
            .expect("remainder to creator failed");
    }

    event.is_finalized = true;
    event.prize_pool = 0;
    env.storage()
        .persistent()
        .set(&DataKey::OracleEvent(event_id), &event);
}
