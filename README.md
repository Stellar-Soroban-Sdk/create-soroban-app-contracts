# create-soroban-app-contracts

Official Soroban contract templates for [create-soroban-app](https://github.com/Stellar-Soroban-Sdk/create-soroban-app) — token, escrow, and multisig.

These are the canonical, production-ready reference implementations that the CLI templates are based on. Each contract is deployed to the **Stellar Testnet** and built against the [Soroban SDK](https://developers.stellar.org/docs/smart-contracts) — Stellar's smart contract platform.

## How These Contracts Fit Into Stellar

Soroban is Stellar's smart contract environment, running on the Stellar network alongside its native payment rails. These contracts integrate with Stellar in the following ways:

- **Addresses** — every participant (`admin`, `buyer`, `owner`, etc.) is a Stellar account (`G...`) or contract address (`C...`), authenticated via Stellar's Ed25519 keypair system through `require_auth()`
- **Tokens** — `basic-token` implements the [SEP-41 token interface](https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0041.md), the Soroban-native fungible token standard. `escrow` and `multisig` accept any SEP-41 token address, including the native XLM Stellar Asset Contract
- **Storage** — contract state lives in Soroban's ledger storage (instance, persistent, temporary), subject to Stellar's ledger entry TTL and rent model. These contracts extend TTLs on reads to keep entries alive
- **Events** — all state-changing functions emit Soroban contract events, consumable by Stellar Horizon or RPC subscribers
- **Testnet** — contracts are deployed to the Stellar Testnet via the [Stellar CLI](https://developers.stellar.org/docs/tools/stellar-cli). See [DEPLOYMENTS.md](DEPLOYMENTS.md) for live addresses

## Architecture

```
Stellar Network
└── Soroban Runtime
    ├── basic-token        SEP-41 fungible token
    │   ├── instance storage: admin, decimals, name, symbol, total_supply
    │   └── persistent storage: balances, allowances (with TTL bumping)
    │
    ├── escrow             Trustless escrow with arbiter resolution
    │   ├── instance storage: EscrowConfig, EscrowState
    │   └── integrates with any SEP-41 token via token::Client
    │
    └── multisig           M-of-N multisig wallet
        ├── instance storage: owners[], threshold, next_id
        └── persistent storage: Proposal{} entries
```

## Contracts

| Contract | Description | Docs |
|----------|-------------|------|
| [`basic-token`](contracts/basic-token/src/lib.rs) | SEP-41 fungible token — mint, burn, burn_from, transfer, approve | [docs/basic-token.md](docs/basic-token.md) |
| [`escrow`](contracts/escrow/src/lib.rs) | Trustless escrow with arbiter dispute resolution | [docs/escrow.md](docs/escrow.md) |
| [`multisig`](contracts/multisig/src/lib.rs) | M-of-N multisignature wallet | [docs/multisig.md](docs/multisig.md) |

## Quick Start

```bash
# Build all contracts
cargo build --release --target wasm32-unknown-unknown

# Run all tests
cargo test

# Deploy to Stellar Testnet
./scripts/deploy.sh
```

## Requirements

- Rust 1.74+
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- Stellar CLI: `cargo install --locked stellar-cli`

## Stellar Network Configuration

```bash
# Add Stellar Testnet to your CLI config
stellar network add testnet \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015"

# Fund a new keypair via Friendbot
stellar keys generate my-key
stellar keys fund my-key --network testnet
```

## Testnet Deployments

See [DEPLOYMENTS.md](DEPLOYMENTS.md) for live testnet contract addresses.

## Sister Repos

- CLI: https://github.com/Stellar-Soroban-Sdk/create-soroban-app
- Web UI: https://github.com/Stellar-Soroban-Sdk/create-soroban-app-web

## Further Reading

- [Soroban Developer Docs](https://developers.stellar.org/docs/smart-contracts)
- [SEP-41 Token Standard](https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0041.md)
- [Stellar CLI Reference](https://developers.stellar.org/docs/tools/stellar-cli)
- [Soroban SDK Crate](https://crates.io/crates/soroban-sdk)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Apache-2.0
