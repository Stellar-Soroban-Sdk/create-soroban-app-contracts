# multisig

An M-of-N multisignature wallet contract for Soroban. Owners propose token transfers; execution requires `threshold` approvals.

## Types

### `Proposal`
| Field | Type | Description |
|-------|------|-------------|
| `id` | `u64` | Auto-incremented proposal ID |
| `to` | `Address` | Transfer recipient |
| `token` | `Address` | Token contract address |
| `amount` | `i128` | Transfer amount |
| `approvals` | `Vec<Address>` | Owners who have approved |
| `executed` | `bool` | Whether the transfer has been sent |
| `created_at` | `u64` | Ledger timestamp at creation |

## Functions

### `initialize(env, owners, threshold)`
One-time setup. `threshold` must be `> 0` and `<= owners.len()`.

---

### `propose(env, proposer, to, token, amount) → u64`
Creates a new proposal. Proposer is auto-approved. Returns proposal ID. Proposer must be an owner. Proposer auth required.

---

### `approve(env, owner, proposal_id)`
Owner approves a proposal. Panics if already approved or proposal executed. Owner auth required.

---

### `revoke(env, owner, proposal_id)`
Owner removes their approval. Panics if not previously approved or already executed. Owner auth required.

---

### `execute(env, proposal_id)`
Executes the token transfer if `approvals.len() >= threshold`. Panics if already executed or insufficient approvals.

---

### `get_proposal(env, proposal_id) → Proposal`
Returns proposal by ID.

### `get_owners(env) → Vec<Address>`
Returns current owner list.

### `get_threshold(env) → u32`
Returns current approval threshold.

### `is_owner(env, address) → bool`
Returns whether `address` is an owner.

---

### `add_owner(env, caller, new_owner)`
Adds a new owner. Caller must be existing owner. Caller auth required.

---

### `remove_owner(env, caller, owner)`
Removes an owner. Cannot remove the last owner. If threshold exceeds new owner count, threshold is reduced automatically. Caller auth required.

---

### `change_threshold(env, caller, new_threshold)`
Updates the approval threshold. Must be `> 0` and `<= owners.len()`. Caller must be owner. Caller auth required.

## Usage Example

```rust
// 2-of-3 multisig
client.initialize(&vec![&env, alice, bob, carol], &2u32);

// Fund the contract with tokens first, then propose
let id = client.propose(&alice, &recipient, &token, &500_i128);

// Second owner approves
client.approve(&bob, &id);

// Execute (2 approvals >= threshold of 2)
client.execute(&id);

// Governance
client.add_owner(&alice, &dave);
client.change_threshold(&alice, &3u32);
client.remove_owner(&alice, &carol);
```
