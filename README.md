# Boltis Linera Buildathon Submission

A card game platform built on Linera blockchain with smart contracts for game state management and leaderboard tracking.

## 🎮 Project Overview

Boltis is a multiplayer card game (similar to UNO) with on-chain state management and player progression system. This submission includes three Linera smart contracts deployed to testnet.

## 📦 Contracts

### 1. **Shared Counter** (`shared-value/`)

- Simple counter contract for testing and demonstration
- **Testnet App ID**: `a853136aaf79d91155eff9def2a6002122fac5ef691102179e8cc8ee62290b86`

### 2. **Game Engine** (`game-engine/`)

- Manages game state and player moves
- Stores game sessions on-chain
- Validates and applies game operations

### 3. **Leaderboard** (`leaderboard/`)

- Tracks player scores, XP, and match history
- Maintains global rankings
- Records match results and statistics

## 🚀 Quick Start

### Prerequisites

- Rust 1.86.0 or later
- Linera CLI (`cargo install linera-service`)
- Linera wallet configured for testnet

### Build Contracts

```bash
# Build all contracts
cd contracts/shared-value && cargo build --release --target wasm32-unknown-unknown
cd ../game-engine && cargo build --release --target wasm32-unknown-unknown
cd ../leaderboard && cargo build --release --target wasm32-unknown-unknown
```

### Deploy to Testnet

```bash
# Run the deployment script
./deploy_testnet.sh
```

This will:

1. Build all WASM binaries
2. Deploy contracts to Linera testnet
3. Output application IDs for each contract

## 📁 Project Structure

```
contracts/
├── shared-value/       # Counter contract (deployed ✅)
├── game-engine/        # Game state management
└── leaderboard/        # Player rankings and stats
```

## 🔧 Development

### Local Testing with Docker

```bash
# Start local Linera network
docker compose up --force-recreate
```

Ports:

- 5173: Frontend (optional)
- 8080: Linera faucet
- 9000: GraphQL service
- 9001: Validator proxy
- 13001: Validator

### Key Dependencies

All contracts use:

- `linera-sdk = "0.15.8"`
- `async-graphql = "=7.0.17"`
- `async-graphql-value = "=7.0.17"` (pinned to avoid unstable features)

## 🎯 Features

- **On-chain Game State**: All game moves stored on blockchain
- **Player Progression**: XP system with levels and rankings
- **Match History**: Complete record of all games played
- **Leaderboard**: Global player rankings by XP
- **Testnet Deployment**: Live contracts on Linera Conway testnet
