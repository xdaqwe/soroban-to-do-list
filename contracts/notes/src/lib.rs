#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, vec, Env, String, Symbol, Vec, Address,
};

// ─── Data Types ───────────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

// Storage keys
const TASKS: Symbol = symbol_short!("TASKS");
const COUNTER: Symbol = symbol_short!("COUNTER");

// ─── Contract ─────────────────────────────────────────────────────────────────

#[contract]
pub struct TodoContract;

#[contractimpl]
impl TodoContract {

    /// Tambah task baru
    pub fn add_task(env: Env, user: Address, title: String) -> u32 {
        user.require_auth();

        // Ambil counter ID terakhir
        let mut counter: u32 = env
            .storage()
            .persistent()
            .get(&(COUNTER, user.clone()))
            .unwrap_or(0);

        counter += 1;

        let task = Task {
            id: counter,
            title,
            completed: false,
        };

        // Ambil list tasks user, tambahkan task baru
        let mut tasks: Vec<Task> = env
            .storage()
            .persistent()
            .get(&(TASKS, user.clone()))
            .unwrap_or(vec![&env]);

        tasks.push_back(task);

        // Simpan kembali
        env.storage().persistent().set(&(TASKS, user.clone()), &tasks);
        env.storage().persistent().set(&(COUNTER, user.clone()), &counter);

        counter // return ID task baru
    }

    /// Ambil semua tasks milik user
    pub fn get_tasks(env: Env, user: Address) -> Vec<Task> {
        env.storage()
            .persistent()
            .get(&(TASKS, user))
            .unwrap_or(vec![&env])
    }

    /// Tandai task sebagai selesai/belum
    pub fn complete_task(env: Env, user: Address, task_id: u32) -> bool {
        user.require_auth();

        let mut tasks: Vec<Task> = env
            .storage()
            .persistent()
            .get(&(TASKS, user.clone()))
            .unwrap_or(vec![&env]);

        let mut found = false;
        let mut updated_tasks: Vec<Task> = vec![&env];

        for i in 0..tasks.len() {
            let mut task = tasks.get(i).unwrap();
            if task.id == task_id {
                task.completed = !task.completed; // toggle
                found = true;
            }
            updated_tasks.push_back(task);
        }

        if found {
            env.storage().persistent().set(&(TASKS, user), &updated_tasks);
        }

        found
    }

    /// Hapus task berdasarkan ID
    pub fn delete_task(env: Env, user: Address, task_id: u32) -> bool {
        user.require_auth();

        let tasks: Vec<Task> = env
            .storage()
            .persistent()
            .get(&(TASKS, user.clone()))
            .unwrap_or(vec![&env]);

        let mut updated_tasks: Vec<Task> = vec![&env];
        let mut found = false;

        for i in 0..tasks.len() {
            let task = tasks.get(i).unwrap();
            if task.id == task_id {
                found = true; // skip / hapus
            } else {
                updated_tasks.push_back(task);
            }
        }

        if found {
            env.storage().persistent().set(&(TASKS, user), &updated_tasks);
        }

        found
    }

    /// Hapus semua tasks milik user
    pub fn clear_tasks(env: Env, user: Address) {
        user.require_auth();
        env.storage().persistent().remove(&(TASKS, user.clone()));
        env.storage().persistent().remove(&(COUNTER, user));
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_add_and_get() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, TodoContract);
        let client = TodoContractClient::new(&env, &contract_id);

        let user = Address::generate(&env);
        let title = String::from_str(&env, "Belajar Soroban");

        let id = client.add_task(&user, &title);
        assert_eq!(id, 1);

        let tasks = client.get_tasks(&user);
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks.get(0).unwrap().title, title);
        assert_eq!(tasks.get(0).unwrap().completed, false);
    }

    #[test]
    fn test_complete_task() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, TodoContract);
        let client = TodoContractClient::new(&env, &contract_id);

        let user = Address::generate(&env);
        client.add_task(&user, &String::from_str(&env, "Deploy contract"));

        let result = client.complete_task(&user, &1);
        assert!(result);

        let tasks = client.get_tasks(&user);
        assert_eq!(tasks.get(0).unwrap().completed, true);
    }

    #[test]
    fn test_delete_task() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, TodoContract);
        let client = TodoContractClient::new(&env, &contract_id);

        let user = Address::generate(&env);
        client.add_task(&user, &String::from_str(&env, "Task 1"));
        client.add_task(&user, &String::from_str(&env, "Task 2"));

        client.delete_task(&user, &1);

        let tasks = client.get_tasks(&user);
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks.get(0).unwrap().id, 2);
    }
}