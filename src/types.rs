use soroban_sdk::{contracttype, Address, Bytes};

/// Defines the condition that must be met before a job can be executed.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum TriggerCondition {
    /// Execute once the ledger sequence number reaches or exceeds the target.
    BlockHeight(u32),
    /// Execute once the ledger timestamp (Unix seconds) reaches or exceeds the target.
    Timestamp(u64),
    /// Execute unconditionally — useful for one-shot manual keeper jobs.
    Immediate,
}

/// Persisted on-chain configuration for a single automation job.
#[contracttype]
#[derive(Clone, Debug)]
pub struct JobConfig {
    /// The address that owns and authorized this job.
    pub owner: Address,
    /// The contract address that will be invoked when the job fires.
    pub target_contract: Address,
    /// The name of the function to call on the target contract.
    pub target_fn: soroban_sdk::Symbol,
    /// ABI-encoded arguments forwarded verbatim to the target function.
    pub target_args: Bytes,
    /// The condition the relayer must verify before submitting execution.
    pub trigger: TriggerCondition,
    /// Set to `true` after the job has been successfully executed.
    /// Written to storage BEFORE the external call to prevent re-entrancy.
    pub executed: bool,
    /// Set to `false` when the owner cancels the job.
    pub active: bool,
}
