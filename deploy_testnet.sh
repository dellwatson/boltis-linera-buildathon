#!/bin/bash

# Deploy game contracts to Linera testnet

set -e

echo "=== Building WASM files directly with cargo ==="

# Add wasm32 target if not already added
rustup target add wasm32-unknown-unknown

echo "=== Building shared-value contract ==="
cd contracts/shared-value
cargo build --release --target wasm32-unknown-unknown
cd ../..

echo "=== Building game-engine contract ==="
cd contracts/game-engine
cargo build --release --target wasm32-unknown-unknown
cd ../..

echo "=== Building leaderboard contract ==="
cd contracts/leaderboard
cargo build --release --target wasm32-unknown-unknown
cd ../..

echo "=== Copying WASM files to deployment directory ==="
mkdir -p wasm-output

# Copy shared-counter WASM files (package name is shared-counter)
cp contracts/shared-value/target/wasm32-unknown-unknown/release/shared_counter_{contract,service}.wasm wasm-output/

# Copy game-engine WASM files
cp contracts/game-engine/target/wasm32-unknown-unknown/release/boltis_game_engine_contract.wasm wasm-output/

# Copy leaderboard WASM files
cp contracts/leaderboard/target/wasm32-unknown-unknown/release/boltis_leaderboard_{contract,service}.wasm wasm-output/

echo "✅ WASM files built successfully!"
ls -lh wasm-output/

echo "=== Setting up Linera wallet for testnet ==="

FAUCET_URL=https://faucet.testnet-conway.linera.net/

# Use existing wallet (already initialized)
echo "Using existing wallet..."
linera wallet show

echo "=== Deploying shared-counter contract ==="
SHARED_COUNTER_APP_ID=$(linera publish-and-create \
  wasm-output/shared_counter_{contract,service}.wasm \
  --json-argument "0")

echo "✅ Shared-counter deployed!"
echo "Application ID: $SHARED_COUNTER_APP_ID"
echo ""

echo "=== Skipping game-engine contract (contract-only, needs service binary) ==="
echo ""

echo "=== Deploying leaderboard contract ==="
LEADERBOARD_APP_ID=$(linera publish-and-create \
  wasm-output/boltis_leaderboard_{contract,service}.wasm \
  --json-argument "{}")

echo "✅ Leaderboard deployed!"
echo "Application ID: $LEADERBOARD_APP_ID"
echo ""

echo "=== Deployment Summary ==="
echo "Shared-counter App ID: $SHARED_COUNTER_APP_ID"
echo "Leaderboard App ID: $LEADERBOARD_APP_ID"
echo ""
echo "Note: Game-engine contract skipped (needs service binary for deployment)"
echo ""
echo "To start GraphQL service:"
echo "linera service --port 8080"
