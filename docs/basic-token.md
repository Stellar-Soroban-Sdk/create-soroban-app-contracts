# basic-token

A standard fungible token contract for the Stellar/Soroban network.

## State

| Key | Type | Description |
|-----|------|-------------|
| `Admin` | `Address` | Contract administrator |
| `Decimals` | `u32` | Token decimal places |
| `Name` | `String` | Token name |
| `Symbol` | `String` | Token symbol |
| `TotalSupply` | `i128` | Total minted supply |
| `Balance(Address)` | `i128` | Per-address balance |
| `Allowance(Address, Address)` | `i128` | Spender allowance |

## Functions

### `initialize(env, admin, decimal, name, symbol)`
One-time setup. Panics if called again.

| Param | Type | Description |
|-------|------|-------------|
| `admin` | `Address` | Initial admin |
| `decimal` | `u32` | Decimal precision (e.g. `7`) |
| `name` | `String` | Token name |
| `symbol` | `String` | Token ticker |

---

### `mint(env, to, amount) → ()`
Mints `amount` tokens to `to`. Admin auth required.

---

### `burn(env, from, amount) → ()`
Burns `amount` from `from`. Caller (`from`) auth required.

---

### `transfer(env, from, to, amount) → ()`
Transfers `amount` from `from` to `to`. Caller (`from`) auth required.

---

### `approve(env, from, spender, amount) → ()`
Sets `spender` allowance on `from`'s balance. Caller (`from`) auth required.

---

### `transfer_from(env, spender, from, to, amount) → ()`
Transfers `amount` from `from` to `to` using spender's allowance. Caller (`spender`) auth required.

---

### `balance(env, id) → i128`
Returns token balance of `id`.

---

### `allowance(env, from, spender) → i128`
Returns how much `spender` can spend from `from`.

---

### `total_supply(env) → i128`
Returns total token supply.

---

### `decimals(env) → u32` / `name(env) → String` / `symbol(env) → String`
Return token metadata.

---

### `set_admin(env, new_admin) → ()`
Transfers admin role. Current admin auth required.

## Usage Example

```rust
// Initialize
client.initialize(&admin, &7u32, &String::from_str(&env, "My Token"), &String::from_str(&env, "MTK"));

// Mint to user
client.mint(&user, &1_000_000_000); // 100 MTK at 7 decimals

// Transfer
client.transfer(&user, &recipient, &500_000_000);

// Approve + transfer_from
client.approve(&user, &spender, &200_000_000);
client.transfer_from(&spender, &user, &recipient, &100_000_000);
```
