use soroban_sdk::Env;

use crate::storage_types::OracleEvent;
use crate::DataKey;

/// Return the live prize pool for an oracle event.
/// Panics with "event_not_found" when the event does not exist.
pub(crate) fn prize_pool(env: &Env, event_id: u64) -> i128 {
    let event: OracleEvent = env
        .storage()
        .persistent()
        .get(&DataKey::OracleEvent(event_id))
        .expect("event_not_found");
    event.prize_pool
}
