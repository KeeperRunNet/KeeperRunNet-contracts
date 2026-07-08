#![cfg(test)]

use keeperrunnet_registry::{KeeperRegistry, KeeperRegistryClient};
use soroban_sdk::{
    testutils::{Address as _, Ledger, LedgerInfo},
    Address, Bytes, Env, Symbol,
};

use keeperrunnet_registry::types::TriggerCondition;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn setup() -> (Env, KeeperRegistryClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, KeeperRegistry);
    let client = KeeperRegistryClient::new(&env, &contract_id);
    (env, client)
}

fn dummy_args(env: &Env) -> Bytes {
    Bytes::from_slice(env, &[0u8; 4])
}

fn dummy_fn(env: &Env) -> Symbol {
    Symbol::new(env, "do_work")
}

// ---------------------------------------------------------------------------
// register_job
// ---------------------------------------------------------------------------

#[test]
fn test_register_job_succeeds() {
    let (env, client) = setup();
    let owner = Address::generate(&env);
    let target = Address::generate(&env);

    client.register(
        &owner,
        &1u64,
        &target,
        &dummy_fn(&env),
        &dummy_args(&env),
        &TriggerCondition::Immediate,
    );

    let job = client.get_job(&1u64);
    assert_eq!(job.owner, owner);
    assert!(job.active);
    assert!(!job.executed);
}

#[test]
#[should_panic]
fn test_register_duplicate_job_id_panics() {
    let (env, client) = setup();
    let owner = Address::generate(&env);
    let target = Address::generate(&env);

    client.register(
        &owner,
        &42u64,
        &target,
        &dummy_fn(&env),
        &dummy_args(&env),
        &TriggerCondition::Immediate,
    );
    // Second registration with the same ID must panic.
    client.register(
        &owner,
        &42u64,
        &target,
        &dummy_fn(&env),
        &dummy_args(&env),
        &TriggerCondition::Immediate,
    );
}

// ---------------------------------------------------------------------------
// execute_job — Immediate trigger
// ---------------------------------------------------------------------------

#[test]
fn test_execute_immediate_job_succeeds() {
    let (env, client) = setup();
    let owner = Address::generate(&env);
    let target = Address::generate(&env);

    client.register(
        &owner,
        &10u64,
        &target,
        &dummy_fn(&env),
        &dummy_args(&env),
        &TriggerCondition::Immediate,
    );

    client.execute_job(&10u64);

    let job = client.get_job(&10u64);
    assert!(job.executed);
}

#[test]
#[should_panic]
fn test_execute_job_twice_panics() {
    let (env, client) = setup();
    let owner = Address::generate(&env);
    let target = Address::generate(&env);

    client.register(
        &owner,
        &20u64,
        &target,
        &dummy_fn(&env),
        &dummy_args(&env),
        &TriggerCondition::Immediate,
    );

    client.execute_job(&20u64);
    // Second execution must panic with ERR_ALREADY_EXECUTED.
    client.execute_job(&20u64);
}

// ---------------------------------------------------------------------------
// execute_job — BlockHeight trigger
// ---------------------------------------------------------------------------

#[test]
fn test_execute_block_height_trigger_passes_when_ready() {
    let (env, client) = setup();
    let owner = Address::generate(&env);
    let target = Address::generate(&env);

    // Register with a target block height of 100.
    client.register(
        &owner,
        &30u64,
        &target,
        &dummy_fn(&env),
        &dummy_args(&env),
        &TriggerCondition::BlockHeight(100),
    );

    // Advance the ledger past the target.
    env.ledger().set(LedgerInfo {
        sequence_number: 101,
        timestamp: 0,
        protocol_version: 20,
        network_passphrase: soroban_sdk::bytes![&env],
        base_reserve: 5_000_000,
        min_temp_entry_ttl: 16,
        min_persistent_entry_ttl: 4096,
        max_entry_ttl: 6_312_000,
    });

    client.execute_job(&30u64);
    assert!(client.get_job(&30u64).executed);
}

#[test]
#[should_panic]
fn test_execute_block_height_trigger_fails_when_not_ready() {
    let (env, client) = setup();
    let owner = Address::generate(&env);
    let target = Address::generate(&env);

    client.register(
        &owner,
        &31u64,
        &target,
        &dummy_fn(&env),
        &dummy_args(&env),
        &TriggerCondition::BlockHeight(9999),
    );

    // Ledger is at the default sequence (0) — condition not met.
    client.execute_job(&31u64);
}

// ---------------------------------------------------------------------------
// execute_job — Timestamp trigger
// ---------------------------------------------------------------------------

#[test]
fn test_execute_timestamp_trigger_passes_when_ready() {
    let (env, client) = setup();
    let owner = Address::generate(&env);
    let target = Address::generate(&env);

    client.register(
        &owner,
        &40u64,
        &target,
        &dummy_fn(&env),
        &dummy_args(&env),
        &TriggerCondition::Timestamp(1_000_000),
    );

    env.ledger().set(LedgerInfo {
        sequence_number: 1,
        timestamp: 2_000_000,
        protocol_version: 20,
        network_passphrase: soroban_sdk::bytes![&env],
        base_reserve: 5_000_000,
        min_temp_entry_ttl: 16,
        min_persistent_entry_ttl: 4096,
        max_entry_ttl: 6_312_000,
    });

    client.execute_job(&40u64);
    assert!(client.get_job(&40u64).executed);
}

// ---------------------------------------------------------------------------
// cancel_job
// ---------------------------------------------------------------------------

#[test]
fn test_cancel_job_deactivates() {
    let (env, client) = setup();
    let owner = Address::generate(&env);
    let target = Address::generate(&env);

    client.register(
        &owner,
        &50u64,
        &target,
        &dummy_fn(&env),
        &dummy_args(&env),
        &TriggerCondition::Immediate,
    );

    assert!(client.is_active(&50u64));
    client.cancel_job(&owner, &50u64);
    assert!(!client.is_active(&50u64));
}

#[test]
#[should_panic]
fn test_execute_cancelled_job_panics() {
    let (env, client) = setup();
    let owner = Address::generate(&env);
    let target = Address::generate(&env);

    client.register(
        &owner,
        &51u64,
        &target,
        &dummy_fn(&env),
        &dummy_args(&env),
        &TriggerCondition::Immediate,
    );

    client.cancel_job(&owner, &51u64);
    // Must panic with ERR_JOB_INACTIVE.
    client.execute_job(&51u64);
}

#[test]
#[should_panic]
fn test_cancel_job_wrong_owner_panics() {
    let (env, client) = setup();
    let owner = Address::generate(&env);
    let attacker = Address::generate(&env);
    let target = Address::generate(&env);

    client.register(
        &owner,
        &52u64,
        &target,
        &dummy_fn(&env),
        &dummy_args(&env),
        &TriggerCondition::Immediate,
    );

    // Attacker tries to cancel owner's job — must panic with ERR_UNAUTHORIZED.
    client.cancel_job(&attacker, &52u64);
}

// ---------------------------------------------------------------------------
// View helpers
// ---------------------------------------------------------------------------

#[test]
#[should_panic]
fn test_get_nonexistent_job_panics() {
    let (_env, client) = setup();
    client.get_job(&9999u64);
}

#[test]
fn test_is_active_nonexistent_returns_false() {
    let (_env, client) = setup();
    assert!(!client.is_active(&9999u64));
}
