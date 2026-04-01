#![no_std]

use soroban_sdk::{
    contract, contractimpl, symbol_short,
    Address, Env, Map, Vec, String
};

#[contract]
pub struct TodoContract;

#[contractimpl]
impl TodoContract {

    // 🧩 Create task
    pub fn create_task(env: Env, user: Address, content: String) {
        user.require_auth();

        let key_tasks = symbol_short!("TASKS");
        let key_count = symbol_short!("COUNT");

        let mut tasks: Map<u32, (u32, String, bool)> =
            env.storage().instance().get(&key_tasks).unwrap_or(Map::new(&env));

        let mut count: u32 =
            env.storage().instance().get(&key_count).unwrap_or(0);

        count += 1;

        let task = (count, content, false);

        tasks.set(count, task);

        env.storage().instance().set(&key_tasks, &tasks);
        env.storage().instance().set(&key_count, &count);
    }

    // ✅ Complete task
    pub fn complete_task(env: Env, user: Address, task_id: u32) {
        user.require_auth();

        let key_tasks = symbol_short!("TASKS");

        let mut tasks: Map<u32, (u32, String, bool)> =
            env.storage().instance().get(&key_tasks).unwrap();

        let mut task = tasks.get(task_id).unwrap();

        task.2 = true; // completed = true

        tasks.set(task_id, task);

        env.storage().instance().set(&key_tasks, &tasks);
    }

    // 📋 Get tasks
    pub fn get_tasks(env: Env) -> Vec<(u32, String, bool)> {
        let key_tasks = symbol_short!("TASKS");

        let tasks: Map<u32, (u32, String, bool)> =
            env.storage().instance().get(&key_tasks).unwrap_or(Map::new(&env));

        let mut list: Vec<(u32, String, bool)> = Vec::new(&env);

        for (_, task) in tasks.iter() {
            list.push_back(task);
        }

        list
    }
}