#!/usr/bin/env bash

set -eu

# Load helper that defines linera_spawn and temp dirs
eval "$(linera net helper)"

echo "[shared-counter] Starting local Linera network with faucet on :8080..."
linera_spawn linera net up --with-faucet

export LINERA_FAUCET_URL=http://localhost:8080

echo "[shared-counter] Initializing wallet via faucet $LINERA_FAUCET_URL..."
linera wallet init --faucet="$LINERA_FAUCET_URL"

INFO=($(linera wallet request-chain --faucet="$LINERA_FAUCET_URL"))
CHAIN_ID="${INFO[0]}"
OWNER="${INFO[1]}"
echo "[shared-counter] Default chain: $CHAIN_ID (owner: $OWNER)"

echo "[shared-counter] Building and publishing shared-counter project..."
cd /build/contracts/shared-counter

# Instantiate counter with initial value 0 on the default chain.
# IMPORTANT: The NAME argument must match the generated wasm prefix
# `<name>_{contract,service}.wasm`. For package `shared-counter`, the
# default name is `shared_counter`, so we pass that here.
APP_ID=$(linera project publish-and-create . shared_counter "$CHAIN_ID" --json-argument "0")

echo "[shared-counter] Application deployed"
echo "SHARED_COUNTER_CHAIN_ID=$CHAIN_ID"
echo "SHARED_COUNTER_APP_ID=$APP_ID"

GRAPHQL_ENDPOINT="http://localhost:9000/chains/$CHAIN_ID/applications/$APP_ID"
echo "SHARED_COUNTER_GRAPHQL_ENDPOINT=$GRAPHQL_ENDPOINT"

cd /build

echo "[counter] Building and publishing counter project..."
cd /build/contracts/counter

# Instantiate counter with initial value 0 on the default chain.
# IMPORTANT: The NAME argument must match the generated wasm prefix
# `<name>_{contract,service}.wasm`. For package `counter`, the
# default name is `counter`, so we pass that here.
COUNTER_APP_ID=$(linera project publish-and-create . counter "$CHAIN_ID" --json-argument "0")

echo "[counter] Application deployed"
echo "COUNTER_CHAIN_ID=$CHAIN_ID"
echo "COUNTER_APP_ID=$COUNTER_APP_ID"

COUNTER_GRAPHQL_ENDPOINT="http://localhost:9000/chains/$CHAIN_ID/applications/$COUNTER_APP_ID"
echo "COUNTER_GRAPHQL_ENDPOINT=$COUNTER_GRAPHQL_ENDPOINT"

cd /build

echo "[shared-counter] Starting linera service on :9000 for local GraphQL access..."
echo "[shared-counter] Open $GRAPHQL_ENDPOINT (shared-counter) or $COUNTER_GRAPHQL_ENDPOINT (counter) in GraphiQL or from your frontend."

# Run GraphQL service in the foreground so the container stays alive
linera service --port 9000
