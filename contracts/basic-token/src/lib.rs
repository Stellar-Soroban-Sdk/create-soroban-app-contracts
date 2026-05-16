#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, String};

#[contracttype]
pub enum DataKey {
    Admin,
    Decimals,
    Name,
    Symbol,
    TotalSupply,
    Balance(Address),
    Allowance(Address, Address),
}

fn get_admin(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::Admin).unwrap()
}

const BALANCE_BUMP_AMOUNT: u32 = 518400; // ~30 days in ledgers
const BALANCE_LIFETIME_THRESHOLD: u32 = 259200; // ~15 days

fn get_balance(env: &Env, addr: &Address) -> i128 {
    let key = DataKey::Balance(addr.clone());
    let val = env.storage().persistent().get(&key).unwrap_or(0);
    if val != 0 {
        env.storage()
            .persistent()
            .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
    }
    val
}

fn set_balance(env: &Env, addr: &Address, amount: i128) {
    env.storage()
        .persistent()
        .set(&DataKey::Balance(addr.clone()), &amount);
}

fn get_allowance(env: &Env, from: &Address, spender: &Address) -> i128 {
    env.storage()
        .persistent()
        .get(&DataKey::Allowance(from.clone(), spender.clone()))
        .unwrap_or(0)
}

fn set_allowance(env: &Env, from: &Address, spender: &Address, amount: i128) {
    let key = DataKey::Allowance(from.clone(), spender.clone());
    if amount == 0 {
        env.storage().persistent().remove(&key);
    } else {
        env.storage().persistent().set(&key, &amount);
        env.storage()
            .persistent()
            .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
    }
}

#[contract]
pub struct TokenContract;

#[contractimpl]
impl TokenContract {
    pub fn initialize(env: Env, admin: Address, decimal: u32, name: String, symbol: String) {
        assert!(
            !env.storage().instance().has(&DataKey::Admin),
            "already initialized"
        );
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Decimals, &decimal);
        env.storage().instance().set(&DataKey::Name, &name);
        env.storage().instance().set(&DataKey::Symbol, &symbol);
        env.storage().instance().set(&DataKey::TotalSupply, &0_i128);
        env.events()
            .publish((symbol_short!("init"),), (admin, decimal, name, symbol));
    }

    pub fn mint(env: Env, to: Address, amount: i128) {
        assert!(amount > 0, "amount must be positive");
        let admin = get_admin(&env);
        admin.require_auth();
        let balance = get_balance(&env, &to);
        set_balance(&env, &to, balance + amount);
        let supply: i128 = env
            .storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap();
        env.storage()
            .instance()
            .set(&DataKey::TotalSupply, &(supply + amount));
        env.events()
            .publish((symbol_short!("mint"),), (to, amount));
    }

    pub fn burn(env: Env, from: Address, amount: i128) {
        assert!(amount > 0, "amount must be positive");
        from.require_auth();
        let balance = get_balance(&env, &from);
        assert!(balance >= amount, "insufficient balance");
        set_balance(&env, &from, balance - amount);
        let supply: i128 = env
            .storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap();
        env.storage()
            .instance()
            .set(&DataKey::TotalSupply, &(supply - amount));
        env.events()
            .publish((symbol_short!("burn"),), (from, amount));
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        assert!(amount > 0, "amount must be positive");
        from.require_auth();
        let from_balance = get_balance(&env, &from);
        assert!(from_balance >= amount, "insufficient balance");
        set_balance(&env, &from, from_balance - amount);
        let to_balance = get_balance(&env, &to);
        set_balance(&env, &to, to_balance + amount);
        env.events()
            .publish((symbol_short!("transfer"),), (from, to, amount));
    }

    pub fn approve(env: Env, from: Address, spender: Address, amount: i128) {
        assert!(amount >= 0, "amount must be non-negative");
        from.require_auth();
        set_allowance(&env, &from, &spender, amount);
        env.events()
            .publish((symbol_short!("approve"),), (from, spender, amount));
    }

    pub fn transfer_from(
        env: Env,
        spender: Address,
        from: Address,
        to: Address,
        amount: i128,
    ) {
        assert!(amount > 0, "amount must be positive");
        spender.require_auth();
        let allowance = get_allowance(&env, &from, &spender);
        assert!(allowance >= amount, "insufficient allowance");
        let from_balance = get_balance(&env, &from);
        assert!(from_balance >= amount, "insufficient balance");
        set_allowance(&env, &from, &spender, allowance - amount);
        set_balance(&env, &from, from_balance - amount);
        let to_balance = get_balance(&env, &to);
        set_balance(&env, &to, to_balance + amount);
        env.events()
            .publish((symbol_short!("xfer_from"),), (spender, from, to, amount));
    }

    pub fn balance(env: Env, id: Address) -> i128 {
        get_balance(&env, &id)
    }

    pub fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        get_allowance(&env, &from, &spender)
    }

    pub fn total_supply(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0)
    }

    pub fn decimals(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::Decimals).unwrap()
    }

    pub fn name(env: Env) -> String {
        env.storage().instance().get(&DataKey::Name).unwrap()
    }

    pub fn symbol(env: Env) -> String {
        env.storage().instance().get(&DataKey::Symbol).unwrap()
    }

    pub fn set_admin(env: Env, new_admin: Address) {
        let admin = get_admin(&env);
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &new_admin);
        env.events()
            .publish((symbol_short!("set_admin"),), (new_admin,));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    fn setup() -> (Env, TokenContractClient<'static>, Address) {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(TokenContract, ());
        let client = TokenContractClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        client.initialize(
            &admin,
            &7_u32,
            &String::from_str(&env, "Test Token"),
            &String::from_str(&env, "TTK"),
        );
        (env, client, admin)
    }

    #[test]
    fn test_initialize() {
        let (env, client, _) = setup();
        assert_eq!(client.decimals(), 7);
        assert_eq!(client.name(), String::from_str(&env, "Test Token"));
        assert_eq!(client.symbol(), String::from_str(&env, "TTK"));
        assert_eq!(client.total_supply(), 0);
    }

    #[test]
    #[should_panic(expected = "already initialized")]
    fn test_initialize_twice_panics() {
        let (env, client, admin) = setup();
        client.initialize(
            &admin,
            &7_u32,
            &String::from_str(&env, "Test Token"),
            &String::from_str(&env, "TTK"),
        );
    }

    #[test]
    fn test_mint_and_balance() {
        let (env, client, _) = setup();
        let user = Address::generate(&env);
        client.mint(&user, &1000);
        assert_eq!(client.balance(&user), 1000);
        assert_eq!(client.total_supply(), 1000);
    }

    #[test]
    fn test_burn() {
        let (env, client, _) = setup();
        let user = Address::generate(&env);
        client.mint(&user, &500);
        client.burn(&user, &200);
        assert_eq!(client.balance(&user), 300);
        assert_eq!(client.total_supply(), 300);
    }

    #[test]
    fn test_transfer() {
        let (env, client, _) = setup();
        let alice = Address::generate(&env);
        let bob = Address::generate(&env);
        client.mint(&alice, &1000);
        client.transfer(&alice, &bob, &400);
        assert_eq!(client.balance(&alice), 600);
        assert_eq!(client.balance(&bob), 400);
    }

    #[test]
    fn test_approve_and_transfer_from() {
        let (env, client, _) = setup();
        let alice = Address::generate(&env);
        let bob = Address::generate(&env);
        let carol = Address::generate(&env);
        client.mint(&alice, &1000);
        client.approve(&alice, &bob, &300);
        assert_eq!(client.allowance(&alice, &bob), 300);
        client.transfer_from(&bob, &alice, &carol, &200);
        assert_eq!(client.balance(&alice), 800);
        assert_eq!(client.balance(&carol), 200);
        assert_eq!(client.allowance(&alice, &bob), 100);
    }

    #[test]
    #[should_panic(expected = "insufficient balance")]
    fn test_transfer_insufficient_balance() {
        let (env, client, _) = setup();
        let alice = Address::generate(&env);
        let bob = Address::generate(&env);
        client.mint(&alice, &100);
        client.transfer(&alice, &bob, &200);
    }

    #[test]
    #[should_panic(expected = "insufficient allowance")]
    fn test_transfer_from_exceeds_allowance() {
        let (env, client, _) = setup();
        let alice = Address::generate(&env);
        let bob = Address::generate(&env);
        let carol = Address::generate(&env);
        client.mint(&alice, &1000);
        client.approve(&alice, &bob, &50);
        client.transfer_from(&bob, &alice, &carol, &100);
    }

    #[test]
    fn test_set_admin() {
        let (env, client, _) = setup();
        let new_admin = Address::generate(&env);
        client.set_admin(&new_admin);
        // new admin can mint
        client.mint(&new_admin, &100);
        assert_eq!(client.balance(&new_admin), 100);
    }

    #[test]
    #[should_panic(expected = "amount must be positive")]
    fn test_mint_zero_panics() {
        let (env, client, _) = setup();
        let user = Address::generate(&env);
        client.mint(&user, &0);
    }
}
