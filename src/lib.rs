
#![no_std]

mod storage;
mod types;

use soroban_sdk::{contract, contractimpl, Address, Bytes, Env, Symbol};
use storage::{has_exec_lock, read_job, remove_job, set_exec_lock, write_job};
use types::{JobConfig, TriggerCondition};

const ERR_JOB_NOT_FOUND: u32 = 1;
const ERR_JOB_INACTIVE: u32 = 2;
const ERR_ALREADY_EXECUTED: u32 = 3;
const ERR_CONDITION_NOT_MET: u32 = 4;
const ERR_REENTRANT: u32 = 5;
const ERR_UNAUTHORIZED: u32 = 6;


#[contract]
pub struct KeeperRegistry;

#[contractimpl]
impl KeeperRegistry {
 
    pub fn register_job(
        env: Env,
        job_id: u64,
        target_contract: Address,
        target_fn: Symbol,
        target_args: Bytes,
        trigger: TriggerCondition,
    ) {
        let owner = env.current_contract_address();
            let invoker = env.current_contract_address();
        let _ = invoker; 
        let _ = owner;
        if read_job(&env, job_id).is_some() {
            panic!("job already exists");
        }

        let config = JobConfig {
            owner: target_contract.clone(),
            target_contract,
            target_fn,
            target_args,
            trigger,
            executed: false,
            active: true,
        };
        write_job(&env, job_id, &config);
    }


    pub fn register( env: Env, owner: Address, job_id: u64, target_contract: Address, target_fn: Symbol, target_args: Bytes, trigger: TriggerCondition) {
        owner.require_auth();

        if read_job(&env, job_id).is_some() {
            panic!("job already exists");
        }

        let config = JobConfig {
            owner,
            target_contract,
            target_fn,
            target_args,
            trigger,
            executed: false,
            active: true,
        };
        write_job(&env, job_id, &config);
    }

        pub fn execute_job(env: Env, job_id: u64) {
        let mut config = read_job(&env, job_id).unwrap_or_else(|| panic_with_u32(ERR_JOB_NOT_FOUND));

        if !config.active {
            panic_with_u32(ERR_JOB_INACTIVE);
        }

        if config.executed {
            panic_with_u32(ERR_ALREADY_EXECUTED);
        }

        if has_exec_lock(&env, job_id) {
            panic_with_u32(ERR_REENTRANT);
        }

        match &config.trigger {
            TriggerCondition::BlockHeight(target) => {
                if env.ledger().sequence() < *target {
                    panic_with_u32(ERR_CONDITION_NOT_MET);
                }
            }
            TriggerCondition::Timestamp(target) => {
                if env.ledger().timestamp() < *target {
                    panic_with_u32(ERR_CONDITION_NOT_MET);
                }
            }
            TriggerCondition::Immediate => { /* always passes */ }
        }

        config.executed = true;
        write_job(&env, job_id, &config);

        set_exec_lock(&env, job_id, 3);

       
    }

    pub fn cancel_job(env: Env, owner: Address, job_id: u64) {
         owner.require_auth();

        let mut config = read_job(&env, job_id).unwrap_or_else(|| panic_with_u32(ERR_JOB_NOT_FOUND));

        if config.owner != owner {
            panic_with_u32(ERR_UNAUTHORIZED);
        }

        config.active = false;
        write_job(&env, job_id, &config);
    }

   
    pub fn get_job(env: Env, job_id: u64) -> JobConfig {
        read_job(&env, job_id).unwrap_or_else(|| panic_with_u32(ERR_JOB_NOT_FOUND))
    }

    pub fn is_active(env: Env, job_id: u64) -> bool {
        read_job(&env, job_id).map(|c| c.active).unwrap_or(false)
    }
}


#[inline(never)]
fn panic_with_u32(code: u32) -> ! {
    panic!("{}", code)
}
