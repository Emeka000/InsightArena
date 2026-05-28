#![no_std]

pub mod admin;
pub mod storage;
pub mod storage_types;
mod token;

use soroban_sdk::{contract, contractimpl, Address, Env};

use admin::AdminError;

// ---------------------------------------------------------------------------
// Contract entry point
// ---------------------------------------------------------------------------

/// The CreatorEventManager contract.
///
/// Call [`CreatorEventManagerContract::initialize`] exactly once after
/// deployment to configure the contract.  All other functions will panic
/// (or return an error) if called before initialization.
#[contract]
pub struct CreatorEventManagerContract;

#[contractimpl]
impl CreatorEventManagerContract {
    /// Initialise the contract for first use.
    ///
    /// Must be called exactly once after deployment.  Stores the admin,
    /// AI agent, treasury, XLM token address, and creation fee in persistent
    /// storage, resets all counters to zero, and emits an `initialized` event.
    ///
    /// # Panics
    /// * `"already_initialized"` — called more than once.
    /// * `"invalid_address"` — one of the addresses equals the contract itself.
    /// * `"invalid_creation_fee"` — `initial_creation_fee` ≤ 0.
    pub fn initialize(
        env: Env,
        admin: Address,
        ai_agent: Address,
        treasury: Address,
        xlm_token: Address,
        initial_creation_fee: i128,
    ) {
        match admin::initialize(
            &env,
            admin,
            ai_agent,
            treasury,
            xlm_token,
            initial_creation_fee,
        ) {
            Ok(()) => {}
            Err(AdminError::AlreadyInitialized) => {
                panic!("already_initialized")
            }
            Err(AdminError::InvalidAddress) => {
                panic!("invalid_address")
            }
            Err(AdminError::InvalidCreationFee) => {
                panic!("invalid_creation_fee")
            }
            Err(_) => panic!("unexpected_error"),
        }
    }

    /// Update the treasury address where collected fees are sent.
    ///
    /// Only the admin may call this. `new_treasury` must not be the contract itself.
    ///
    /// # Panics
    /// * `"unauthorized"` — caller is not the admin.
    /// * `"invalid_address"` — `new_treasury` equals the contract address.
    pub fn set_treasury(env: Env, caller: Address, new_treasury: Address) {
        match admin::set_treasury(&env, caller, new_treasury) {
            Ok(()) => {}
            Err(AdminError::Unauthorized) => panic!("unauthorized"),
            Err(AdminError::InvalidAddress) => panic!("invalid_address"),
            Err(_) => panic!("unexpected_error"),
        }
    }

    /// Update the AI oracle agent address authorised to submit match results.
    ///
    /// Only the admin may call this. `new_agent` must not be the contract itself.
    ///
    /// # Panics
    /// * `"unauthorized"` — caller is not the admin.
    /// * `"invalid_address"` — `new_agent` equals the contract address.
    pub fn set_ai_agent(env: Env, caller: Address, new_agent: Address) {
        match admin::set_ai_agent(&env, caller, new_agent) {
            Ok(()) => {}
            Err(AdminError::Unauthorized) => panic!("unauthorized"),
            Err(AdminError::InvalidAddress) => panic!("invalid_address"),
            Err(_) => panic!("unexpected_error"),
        }
    }

    /// Halt contract operations in an emergency.
    ///
    /// Only the admin may call this. Panics if the contract is already paused.
    ///
    /// # Panics
    /// * `"unauthorized"` — caller is not the admin.
    /// * `"already_paused"` — contract is already paused.
    pub fn pause(env: Env, caller: Address) {
        match admin::pause(&env, caller) {
            Ok(()) => {}
            Err(AdminError::Unauthorized) => panic!("unauthorized"),
            Err(AdminError::AlreadyPaused) => panic!("already_paused"),
            Err(_) => panic!("unexpected_error"),
        }
    }

    /// Resume contract operations after a pause.
    ///
    /// Only the admin may call this. Panics if the contract is not currently paused.
    ///
    /// # Panics
    /// * `"unauthorized"` — caller is not the admin.
    /// * `"not_paused"` — contract is not currently paused.
    pub fn unpause(env: Env, caller: Address) {
        match admin::unpause(&env, caller) {
            Ok(()) => {}
            Err(AdminError::Unauthorized) => panic!("unauthorized"),
            Err(AdminError::NotPaused) => panic!("not_paused"),
            Err(_) => panic!("unexpected_error"),
        }
    }

    /// Returns `true` if the contract has been initialised.
    pub fn is_initialized(env: Env) -> bool {
        admin::is_initialized(&env)
    }

    /// Returns the current creation fee in stroops, or 0 if not initialised.
    pub fn get_creation_fee(env: Env) -> i128 {
        admin::get_creation_fee(&env).unwrap_or(0)
    }

    /// Returns `true` if the contract is currently paused.
    pub fn is_paused(env: Env) -> bool {
        admin::is_paused(&env)
    }

    /// Returns the current treasury address, or panics if not initialised.
    pub fn get_treasury(env: Env) -> Address {
        admin::get_treasury(&env).unwrap_or_else(|| panic!("not_initialized"))
    }

    /// Returns the current AI agent address, or panics if not initialised.
    pub fn get_ai_agent(env: Env) -> Address {
        admin::get_ai_agent(&env).unwrap_or_else(|| panic!("not_initialized"))
    }
}
