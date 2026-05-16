# Contributing

## Local Dev Setup

1. Install Rust:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Add the wasm32 target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

3. Install Stellar CLI:
   ```bash
   cargo install --locked stellar-cli
   ```

4. Configure testnet:
   ```bash
   stellar network add testnet \
     --rpc-url https://soroban-testnet.stellar.org \
     --network-passphrase "Test SDF Network ; September 2015"
   ```

## Build

```bash
cargo build                                              # dev build
cargo build --release --target wasm32-unknown-unknown   # optimized wasm
```

## Test

```bash
cargo test                    # all contracts
cargo test -p basic-token     # single contract
cargo test -- --nocapture     # show println output
```

## Deploy to Testnet

```bash
./scripts/deploy.sh
```

The script generates a funded keypair, deploys all three contracts, and appends addresses to `DEPLOYMENTS.md`.

## Adding a New Contract Template

1. Create the contract directory:
   ```bash
   mkdir -p contracts/my-contract/src
   ```

2. Add `contracts/my-contract/Cargo.toml`:
   ```toml
   [package]
   name = "my-contract"
   version = "0.1.0"
   edition = "2021"

   [lib]
   crate-type = ["cdylib", "rlib"]

   [dependencies]
   soroban-sdk = { workspace = true }
   ```

3. Add `"contracts/my-contract"` to the `members` array in the root `Cargo.toml`.

4. Implement `contracts/my-contract/src/lib.rs` with a `#[contract]` struct and `#[contractimpl]` block.

5. Add at least 8 tests in a `#[cfg(test)]` module covering happy paths, auth rejections, and edge cases.

6. Write `docs/my-contract.md` with full function reference and usage examples.

7. Add a row to the contracts table in `README.md`.

8. Deploy and record the testnet address in `DEPLOYMENTS.md`.

## Code Standards

- No `todo!()` macros — all functions must be fully implemented
- All admin/privileged functions must call `require_auth()`
- Positive amount assertions on all financial operations
- One-time initialization guarded by a storage existence check

## Sister Repos

- CLI: https://github.com/Stellar-Soroban-Sdk/create-soroban-app
- Web UI: https://github.com/Stellar-Soroban-Sdk/create-soroban-app-web
