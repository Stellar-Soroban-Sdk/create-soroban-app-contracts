#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, token, vec, Address, Env, Vec,
};

#[contracttype]
#[derive(Clone)]
pub struct Proposal {
    pub id: u64,
    pub to: Address,
    pub token: Address,
    pub amount: i128,
    pub approvals: Vec<Address>,
    pub executed: bool,
    pub created_at: u64,
}

#[contracttype]
enum DataKey {
    Owners,
    Threshold,
    NextId,
    Proposal(u64),
}

fn get_owners(env: &Env) -> Vec<Address> {
    env.storage().instance().extend_ttl(518400, 518400);
    env.storage().instance().get(&DataKey::Owners).unwrap()
}

fn get_threshold(env: &Env) -> u32 {
    env.storage().instance().extend_ttl(518400, 518400);
    env.storage().instance().get(&DataKey::Threshold).unwrap()
}

fn require_owner(env: &Env, addr: &Address) {
    let owners = get_owners(env);
    assert!(owners.contains(addr), "not an owner");
}

fn get_proposal(env: &Env, id: u64) -> Proposal {
    env.storage()
        .persistent()
        .get(&DataKey::Proposal(id))
        .expect("proposal not found")
}

fn save_proposal(env: &Env, proposal: &Proposal) {
    env.storage()
        .persistent()
        .set(&DataKey::Proposal(proposal.id), proposal);
}

#[contract]
pub struct MultisigContract;

#[contractimpl]
impl MultisigContract {
    pub fn initialize(env: Env, owners: Vec<Address>, threshold: u32) {
        assert!(
            !env.storage().instance().has(&DataKey::Owners),
            "already initialized"
        );
        assert!(!owners.is_empty(), "owners required");
        assert!(
            threshold > 0 && threshold <= owners.len() as u32,
            "invalid threshold"
        );
        env.storage().instance().set(&DataKey::Owners, &owners);
        env.storage()
            .instance()
            .set(&DataKey::Threshold, &threshold);
        env.storage().instance().set(&DataKey::NextId, &0_u64);
        env.events()
            .publish((symbol_short!("init"),), (owners, threshold));
    }

    pub fn propose(
        env: Env,
        proposer: Address,
        to: Address,
        token: Address,
        amount: i128,
    ) -> u64 {
        assert!(amount > 0, "amount must be positive");
        require_owner(&env, &proposer);
        proposer.require_auth();
        let id: u64 = env.storage().instance().get(&DataKey::NextId).unwrap();
        let proposal = Proposal {
            id,
            to,
            token,
            amount,
            approvals: vec![&env, proposer.clone()],
            executed: false,
            created_at: env.ledger().timestamp(),
        };
        save_proposal(&env, &proposal);
        env.storage()
            .instance()
            .set(&DataKey::NextId, &(id + 1));
        env.events()
            .publish((symbol_short!("propose"),), (id, proposer, amount));
        id
    }

    pub fn approve(env: Env, owner: Address, proposal_id: u64) {
        require_owner(&env, &owner);
        owner.require_auth();
        let mut proposal = get_proposal(&env, proposal_id);
        assert!(!proposal.executed, "already executed");
        assert!(
            !proposal.approvals.contains(&owner),
            "already approved"
        );
        proposal.approvals.push_back(owner.clone());
        save_proposal(&env, &proposal);
        env.events()
            .publish((symbol_short!("approve"),), (proposal_id, owner));
    }

    pub fn revoke(env: Env, owner: Address, proposal_id: u64) {
        require_owner(&env, &owner);
        owner.require_auth();
        let mut proposal = get_proposal(&env, proposal_id);
        assert!(!proposal.executed, "already executed");
        let pos = proposal
            .approvals
            .iter()
            .position(|a| a == owner)
            .expect("approval not found");
        proposal.approvals.remove(pos as u32);
        save_proposal(&env, &proposal);
        env.events()
            .publish((symbol_short!("revoke"),), (proposal_id, owner));
    }

    pub fn execute(env: Env, proposal_id: u64) {
        let mut proposal = get_proposal(&env, proposal_id);
        assert!(!proposal.executed, "already executed");
        let threshold = get_threshold(&env);
        assert!(
            proposal.approvals.len() as u32 >= threshold,
            "insufficient approvals"
        );
        proposal.executed = true;
        save_proposal(&env, &proposal);
        let client = token::Client::new(&env, &proposal.token);
        client.transfer(
            &env.current_contract_address(),
            &proposal.to,
            &proposal.amount,
        );
        env.events()
            .publish((symbol_short!("execute"),), (proposal_id, proposal.amount));
    }

    pub fn get_proposal(env: Env, proposal_id: u64) -> Proposal {
        get_proposal(&env, proposal_id)
    }

    pub fn get_owners(env: Env) -> Vec<Address> {
        get_owners(&env)
    }

    pub fn get_threshold(env: Env) -> u32 {
        get_threshold(&env)
    }

    pub fn is_owner(env: Env, address: Address) -> bool {
        get_owners(&env).contains(&address)
    }

    pub fn add_owner(env: Env, caller: Address, new_owner: Address) {
        require_owner(&env, &caller);
        caller.require_auth();
        let mut owners = get_owners(&env);
        assert!(!owners.contains(&new_owner), "already an owner");
        owners.push_back(new_owner.clone());
        env.storage().instance().set(&DataKey::Owners, &owners);
        env.events()
            .publish((symbol_short!("add_owner"),), (new_owner,));
    }

    pub fn remove_owner(env: Env, caller: Address, owner: Address) {
        require_owner(&env, &caller);
        caller.require_auth();
        let mut owners = get_owners(&env);
        assert!(owners.len() > 1, "cannot remove last owner");
        let pos = owners
            .iter()
            .position(|a| a == owner)
            .expect("owner not found");
        owners.remove(pos as u32);
        let threshold = get_threshold(&env);
        if threshold > owners.len() as u32 {
            env.storage()
                .instance()
                .set(&DataKey::Threshold, &(owners.len() as u32));
        }
        env.storage().instance().set(&DataKey::Owners, &owners);
        env.events()
            .publish((symbol_short!("rm_owner"),), (owner,));
    }

    pub fn change_threshold(env: Env, caller: Address, new_threshold: u32) {
        require_owner(&env, &caller);
        caller.require_auth();
        let owners = get_owners(&env);
        assert!(
            new_threshold > 0 && new_threshold <= owners.len() as u32,
            "invalid threshold"
        );
        env.storage()
            .instance()
            .set(&DataKey::Threshold, &new_threshold);
        env.events()
            .publish((symbol_short!("chg_thr"),), (new_threshold,));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{
        testutils::Address as _,
        token::{Client as TokenClient, StellarAssetClient},
        Env,
    };

    fn setup_3of2() -> (
        Env,
        MultisigContractClient<'static>,
        Address,
        Address,
        Address,
        Address,
        Address,
    ) {
        let env = Env::default();
        env.mock_all_auths();

        let owner1 = Address::generate(&env);
        let owner2 = Address::generate(&env);
        let owner3 = Address::generate(&env);

        let token_admin = Address::generate(&env);
        let token_id = env.register_stellar_asset_contract_v2(token_admin.clone());
        let token_address = token_id.address();

        let contract_id = env.register(MultisigContract, ());
        let client = MultisigContractClient::new(&env, &contract_id);

        // Fund the multisig contract
        let sac = StellarAssetClient::new(&env, &token_address);
        sac.mint(&contract_id, &100_000);

        client.initialize(
            &vec![&env, owner1.clone(), owner2.clone(), owner3.clone()],
            &2_u32,
        );

        (env, client, owner1, owner2, owner3, token_address, contract_id)
    }

    #[test]
    fn test_initialize() {
        let (_, client, owner1, owner2, owner3, _, _) = setup_3of2();
        assert_eq!(client.get_threshold(), 2);
        assert!(client.is_owner(&owner1));
        assert!(client.is_owner(&owner2));
        assert!(client.is_owner(&owner3));
    }

    #[test]
    fn test_propose_and_approve_and_execute() {
        let (env, client, owner1, owner2, _, token_address, _) = setup_3of2();
        let recipient = Address::generate(&env);
        let id = client.propose(&owner1, &recipient, &token_address, &500_i128);
        assert_eq!(id, 0);
        client.approve(&owner2, &id);
        let token = TokenClient::new(&env, &token_address);
        let before = token.balance(&recipient);
        client.execute(&id);
        assert_eq!(token.balance(&recipient), before + 500);
        assert!(client.get_proposal(&id).executed);
    }

    #[test]
    #[should_panic(expected = "insufficient approvals")]
    fn test_execute_without_enough_approvals() {
        let (env, client, owner1, _, _, token_address, _) = setup_3of2();
        let recipient = Address::generate(&env);
        let id = client.propose(&owner1, &recipient, &token_address, &500_i128);
        client.execute(&id); // only 1 approval, threshold is 2
    }

    #[test]
    #[should_panic(expected = "already approved")]
    fn test_duplicate_approval() {
        let (env, client, owner1, _, _, token_address, _) = setup_3of2();
        let recipient = Address::generate(&env);
        let id = client.propose(&owner1, &recipient, &token_address, &100_i128);
        client.approve(&owner1, &id); // owner1 already approved via propose
    }

    #[test]
    fn test_revoke_approval() {
        let (env, client, owner1, owner2, _, token_address, _) = setup_3of2();
        let recipient = Address::generate(&env);
        let id = client.propose(&owner1, &recipient, &token_address, &100_i128);
        client.approve(&owner2, &id);
        client.revoke(&owner2, &id);
        let proposal = client.get_proposal(&id);
        assert_eq!(proposal.approvals.len(), 1);
    }

    #[test]
    fn test_add_and_remove_owner() {
        let (env, client, owner1, _, _, _, _) = setup_3of2();
        let new_owner = Address::generate(&env);
        client.add_owner(&owner1, &new_owner);
        assert!(client.is_owner(&new_owner));
        client.remove_owner(&owner1, &new_owner);
        assert!(!client.is_owner(&new_owner));
    }

    #[test]
    fn test_change_threshold() {
        let (_, client, owner1, _, _, _, _) = setup_3of2();
        client.change_threshold(&owner1, &3_u32);
        assert_eq!(client.get_threshold(), 3);
    }

    #[test]
    #[should_panic(expected = "not an owner")]
    fn test_non_owner_cannot_propose() {
        let (env, client, _, _, _, token_address, _) = setup_3of2();
        let stranger = Address::generate(&env);
        let recipient = Address::generate(&env);
        client.propose(&stranger, &recipient, &token_address, &100_i128);
    }

    #[test]
    #[should_panic(expected = "already executed")]
    fn test_execute_twice_panics() {
        let (env, client, owner1, owner2, _, token_address, _) = setup_3of2();
        let recipient = Address::generate(&env);
        let id = client.propose(&owner1, &recipient, &token_address, &100_i128);
        client.approve(&owner2, &id);
        client.execute(&id);
        client.execute(&id);
    }

    #[test]
    #[should_panic(expected = "invalid threshold")]
    fn test_threshold_exceeds_owners() {
        let (_, client, owner1, _, _, _, _) = setup_3of2();
        client.change_threshold(&owner1, &10_u32);
    }

    #[test]
    #[should_panic(expected = "already an owner")]
    fn test_add_duplicate_owner_panics() {
        let (_, client, owner1, owner2, _, _, _) = setup_3of2();
        client.add_owner(&owner1, &owner2);
    }

    #[test]
    #[should_panic(expected = "cannot remove last owner")]
    fn test_remove_last_owner_panics() {
        let (env, client, owner1, owner2, owner3, _, _) = setup_3of2();
        client.remove_owner(&owner1, &owner2);
        client.remove_owner(&owner1, &owner3);
        client.remove_owner(&owner1, &owner1); // last one
    }
}
