use soroban_sdk::{contracttype, Env};

use crate::types::JobConfig;


#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Job(u64),
}


#[contracttype]
#[derive(Clone)]
pub enum TempKey {
    ExecLock(u64),
}


pub fn write_job(env: &Env, job_id: u64, config: &JobConfig) {
    env.storage().persistent().set(&DataKey::Job(job_id), config);
}

pub fn read_job(env: &Env, job_id: u64) -> Option<JobConfig> {
    env.storage().persistent().get(&DataKey::Job(job_id))
}

pub fn remove_job(env: &Env, job_id: u64) {
    env.storage().persistent().remove(&DataKey::Job(job_id));
}


pub fn set_exec_lock(env: &Env, job_id: u64, ttl_ledgers: u32) {
    env.storage()
        .temporary()
        .set(&TempKey::ExecLock(job_id), &true);
    env.storage()
        .temporary()
        .extend_ttl(&TempKey::ExecLock(job_id), ttl_ledgers, ttl_ledgers);
}

pub fn has_exec_lock(env: &Env, job_id: u64) -> bool {
    env.storage()
        .temporary()
        .has(&TempKey::ExecLock(job_id))
}
