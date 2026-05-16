# Testnet Deployments

Network: **Stellar Testnet** (`Test SDF Network ; September 2015`)  
RPC: `https://soroban-testnet.stellar.org`

## Contracts

| Contract | Address | Deployed At |
|----------|---------|-------------|
| basic-token | _run `./scripts/deploy.sh` to populate_ | — |
| escrow | _run `./scripts/deploy.sh` to populate_ | — |
| multisig | _run `./scripts/deploy.sh` to populate_ | — |

## Deploying

```bash
./scripts/deploy.sh
```

The script will:
1. Generate a new keypair and fund it via Friendbot
2. Build optimized WASM for all contracts
3. Upload and instantiate each contract
4. Print and record the contract addresses here

## Verifying

```bash
stellar contract invoke \
  --id <CONTRACT_ADDRESS> \
  --network testnet \
  -- \
  total_supply
```
