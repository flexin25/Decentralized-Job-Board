#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, Env, Symbol, Vec, String
};

#[contract]
pub struct JobBoard;

#[derive(Clone)]
#[contracttype] // 🔥 REQUIRED for storage
pub struct Job {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub employer: String,
}

#[contractimpl]
impl JobBoard {
    pub fn post_job(env: Env, title: String, description: String, employer: String) -> u64 {
        let key = Symbol::short("COUNT");

        let mut count: u64 = env.storage().instance().get(&key).unwrap_or(0);
        count += 1;

        let job = Job {
            id: count,
            title,
            description,
            employer,
        };

        env.storage().instance().set(&count, &job);
        env.storage().instance().set(&key, &count);

        count
    }

    pub fn get_job(env: Env, id: u64) -> Job {
        env.storage().instance().get(&id).unwrap()
    }

    pub fn get_all_jobs(env: Env) -> Vec<Job> {
        let key = Symbol::short("COUNT");
        let count: u64 = env.storage().instance().get(&key).unwrap_or(0);

        let mut jobs = Vec::new(&env);

        let mut i = 1;
        while i <= count {
            let job: Job = env.storage().instance().get(&i).unwrap();
            jobs.push_back(job);
            i += 1;
        }

        jobs
    }
}