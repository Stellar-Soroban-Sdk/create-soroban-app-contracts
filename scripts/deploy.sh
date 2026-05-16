#!/usr/bin/env bash
set -euo pipefail

NETWORK="testnet"
RPC_URL="https://soroban-testnet.stellar.org"
PASSPHRASE="Test SDF Network ; September 2015"

echo "==> Building optimized WASM..."
cargo build --release --target wasm32-unknown-unknown

echo "==> Generating deployer keypair..."
stellar keys generate deployer --overwrite
stellar keys fund deployer --network "$NETWORK"

DEPLOYER=$(stellar keys address deployer)
echo "Deployer: $DEPLOYER"

deploy_contract() {
  local name="$1"
  local wasm="target/wasm32-unknown-unknown/release/${name//-/_}.wasm"
  echo "==> Deploying $name..."
  local address
  address=$(stellar contract deploy \
    --wasm "$wasm" \
    --source deployer \
    --network "$NETWORK" \
    2>&1 | tail -1)
  echo "$name: $address"
  # Update DEPLOYMENTS.md
  sed -i "s|_run \`./scripts/deploy.sh\` to populate_.*\(.*$name.*\)|\1 $address|" DEPLOYMENTS.md || true
  echo "$address"
}

BASIC_TOKEN=$(deploy_contract "basic-token")
ESCROW=$(deploy_contract "escrow")
MULTISIG=$(deploy_contract "multisig")

echo ""
echo "==> Deployment complete"
echo "basic-token : $BASIC_TOKEN"
echo "escrow      : $ESCROW"
echo "multisig    : $MULTISIG"
