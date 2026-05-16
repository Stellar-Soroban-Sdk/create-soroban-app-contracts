# create-soroban-app-contracts

Official Soroban contract templates for [create-soroban-app](https://github.com/Stellar-Soroban-Sdk/create-soroban-app) — token, escrow, and multisig.

These are the canonical, production-ready reference implementations that the CLI templates are based on.

## Contracts

| Contract | Description | Docs |
|----------|-------------|------|
| [`basic-token`](contracts/basic-token/src/lib.rs) | Fungible token (mint, burn, transfer, approve) | [docs/basic-token.md](docs/basic-token.md) |
| [`escrow`](contracts/escrow/src/lib.rs) | Trustless escrow with arbiter dispute resolution | [docs/escrow.md](docs/escrow.md) |
| [`multisig`](contracts/multisig/src/lib.rs) | M-of-N multisignature wallet | [docs/multisig.md](docs/multisig.md) |

## Quick Start

```bash
# Build all contracts
cargo build --release --target wasm32-unknown-unknown

# Run all tests
cargo test

# Deploy to testnet
./scripts/deploy.sh
```

## Requirements

- Rust 1.74+
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- Stellar CLI: `cargo install --locked stellar-cli`

## Testnet Deployments

See [DEPLOYMENTS.md](DEPLOYMENTS.md) for live testnet contract addresses.

## Sister Repos

- CLI: https://github.com/Stellar-Soroban-Sdk/create-soroban-app
- Web UI: https://github.com/Stellar-Soroban-Sdk/create-soroban-app-web

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Apache-2.0
