#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum TriggerCondition {
    BlockHeight(u32),
    Timestamp(u64),
    Immediate,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct JobConfig {
    pub owner: Address,
    pub target_contract: Address,
    pub target_fn: Symbol,
    pub target_args: Bytes,
    pub trigger: TriggerCondition,
    pub executed: bool,
    pub active: bool,
}
