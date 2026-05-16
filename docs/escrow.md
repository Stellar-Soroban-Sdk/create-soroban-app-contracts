# escrow

A trustless escrow contract with arbiter-mediated dispute resolution.

## State Machine

```
Initialized → Deposited → Released
                       ↘ Refunded
                       → Disputed → Released
                                  ↘ Refunded
Deposited (after deadline) → Refunded (via claim_expired)
```

## Types

### `EscrowState`
`Initialized | Deposited | Released | Refunded | Disputed`

### `EscrowConfig`
| Field | Type | Description |
|-------|------|-------------|
| `buyer` | `Address` | Funds depositor |
| `seller` | `Address` | Funds recipient on release |
| `arbiter` | `Address` | Trusted third-party resolver |
| `token` | `Address` | SEP-41 token contract |
| `amount` | `i128` | Locked amount |
| `deadline` | `u64` | Unix timestamp; buyer can reclaim after this |

## Functions

### `initialize(env, buyer, seller, arbiter, token, amount, deadline)`
One-time setup. Panics if already initialized or `amount <= 0`.

---

### `deposit(env)`
Buyer transfers `amount` tokens into the contract. Requires `Initialized` state. Buyer auth required.

---

### `release(env)`
Sends funds to seller. Requires `Deposited` state. Arbiter auth required.

---

### `refund(env)`
Returns funds to buyer. Requires `Deposited` or `Disputed` state. Arbiter auth required.

---

### `dispute(env)`
Buyer flags the escrow for arbiter resolution. Requires `Deposited` state. Buyer auth required.

---

### `resolve(env, release: bool)`
Arbiter resolves a dispute. `release=true` → seller gets funds; `false` → buyer gets refund. Requires `Disputed` state. Arbiter auth required.

---

### `claim_expired(env)`
Buyer reclaims funds after deadline passes. Requires `Deposited` state and `ledger.timestamp >= deadline`. Buyer auth required.

---

### `get_state(env) → EscrowState`
Returns current escrow state.

### `get_config(env) → EscrowConfig`
Returns escrow configuration.

## Usage Example

```rust
// Setup
client.initialize(&buyer, &seller, &arbiter, &token, &1_000_0000000_i128, &deadline);

// Buyer deposits
client.deposit();

// Happy path: arbiter releases to seller
client.release();

// Dispute path
client.dispute();
client.resolve(&true);  // release to seller
client.resolve(&false); // refund to buyer

// Expired path (after deadline)
client.claim_expired();
```
