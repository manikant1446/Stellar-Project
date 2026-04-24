#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, Symbol,
};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Journalist(Address),
}

#[contract]
pub struct JournalistCredential;

#[contractimpl]
impl JournalistCredential {
    // Initialize contract
    pub fn init(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }

        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    // Internal helper
    fn read_admin(env: &Env) -> Address {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap()
    }

    // Issue credential
    pub fn issue(env: Env, admin: Address, journalist: Address, name: Symbol) {
        let stored_admin = Self::read_admin(&env);
        if admin != stored_admin {
            panic!("not authorized");
        }

        admin.require_auth();

        let key = DataKey::Journalist(journalist);
        env.storage().instance().set(&key, &name);
    }

    // Revoke credential
    pub fn revoke(env: Env, admin: Address, journalist: Address) {
        let stored_admin = Self::read_admin(&env);
        if admin != stored_admin {
            panic!("not authorized");
        }

        admin.require_auth();

        let key = DataKey::Journalist(journalist);
        env.storage().instance().remove(&key);
    }

    // Verify credential
    pub fn verify(env: Env, journalist: Address) -> bool {
        let key = DataKey::Journalist(journalist);
        env.storage().instance().has(&key)
    }

    // Get journalist name
    pub fn get_name(env: Env, journalist: Address) -> Option<Symbol> {
        let key = DataKey::Journalist(journalist);
        env.storage().instance().get(&key)
    }
}