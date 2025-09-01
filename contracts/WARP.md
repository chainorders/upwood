# WARP.md

This file provides guidance to WARP (warp.dev) when working with the smart contracts workspace.

## Workspace Overview

Concordium blockchain smart contracts for forest project tokenization, carbon credit management, P2P trading, and identity & compliance systems. All contracts are written in Rust using the Concordium smart contract framework.

## Development Environment

### Prerequisites
- Uses VS Code dev container (`.devcontainer/contracts/`)
- Rust toolchain with Concordium extensions
- Node.js for build orchestration via yarn workspaces
- Concordium wallet file: `.devcontainer/contracts/default_account.export`

### Container Setup
```bash
# Open in VS Code dev container
# Dev Containers: Reopen in Container → Select "contracts"
# Container auto-installs Rust, Concordium toolchain, and dependencies
```

## Core Development Commands

### Building & Cleaning
```bash
yarn build          # Build all contracts in workspace
yarn clean          # Clean all build artifacts
yarn format         # Format all Rust code with nightly formatter
```

### Testing
```bash
yarn test           # Run all contract tests across workspace
```

### Deployment
```bash
yarn deploy         # Deploy all contracts to testnet
```

## Individual Contract Development

Each contract is a yarn workspace with its own package.json:

```bash
# Work on specific contract
cd security-sft-multi/
yarn build          # Build this contract only
yarn test           # Test this contract only  
yarn deploy         # Deploy this contract only
yarn format         # Format this contract's Rust code
```

## Smart Contract Architecture

### Core Contracts
- **identity-registry** - Access control and user verification (whitelist/blacklist)
- **security-sft-multi** - Main forest project token representation (CIS-2)
- **security-sft-single** - Carbon credit tokenization (CIS-2)
- **security-p2p-trading** - Direct trading marketplace
- **security-mint-fund** - Investment fund and bond management

### Contract Dependencies
- **concordium-protocols/** - Shared protocol library used across contracts
- **integration-tests/** - Cross-contract testing framework
- **euroe/** - EUROe stablecoin integration

### Deprecated Contracts
- **compliance/** - Complex compliance system (being removed)
- **offchain-rewards/** - Off-chain rewards (deprecated)
- **security-sft-multi-yielder/** - Yield distribution (deprecated)
- **sponsor/** - Transaction sponsorship (legacy)

## Testing Strategy

### Unit Testing
- Each contract has comprehensive unit tests in `tests/` directories
- Uses Concordium smart contract testing framework
- Tests cover initialization, state changes, and error conditions

### Integration Testing
- Cross-contract interactions tested in `integration-tests/`
- Simulates real-world contract interaction flows
- Tests identity verification → tokenization → trading workflows

## Development Patterns

### Contract Structure
```rust
// Standard Concordium contract structure
#[derive(Serialize, SchemaType)]
pub struct InitParameter { /* ... */ }

#[derive(Serialize, SchemaType)]  
pub struct State { /* ... */ }

#[init(contract = "contract_name")]
fn contract_init(/* ... */) -> InitResult<State> { /* ... */ }

#[receive(contract = "contract_name", name = "function_name")]
fn contract_function(/* ... */) -> ReceiveResult<ActionsTree> { /* ... */ }
```

### Error Handling
- Custom error types for each contract
- Comprehensive error messages for debugging
- Proper error propagation through contract calls

### State Management
- Efficient state serialization/deserialization
- Minimal state modifications for gas optimization
- State validation in all receive functions

## Blockchain Configuration

### Testnet Development
```bash
# Contracts automatically deploy to testnet
# Node: https://grpc.testnet.concordium.com:20000
# Uses wallet from .devcontainer/contracts/default_account.export
```

### Contract Verification
```bash
# Verify deployment
concordium-client contract show <contract-address>

# Check contract schema
concordium-client contract invoke <contract-address> --entrypoint view --schema schema.bin
```

## Common Development Tasks

### Adding New Contract
1. Create new directory with Cargo.toml
2. Add to workspace in root package.json
3. Implement contract using Concordium patterns
4. Add comprehensive tests
5. Update integration tests if needed

### Debugging Contracts
```bash
# Run with verbose logging
RUST_LOG=debug yarn test

# Check contract events
concordium-client transaction status <tx-hash> --show-events
```

### Schema Generation
```bash
# Generate contract schema (automatically done in build)
cargo concordium build --schema-out schema.bin
```

## Environment Variables

- **Network**: Automatically set to testnet in dev container
- **Node URI**: Pre-configured for Concordium testnet
- **Wallet**: Auto-loaded from dev container wallet file

## Testing Individual Functions

```bash
# Test specific function
cd security-sft-multi/
cargo test test_function_name

# Test with output
cargo test test_function_name -- --nocapture

# Run integration tests
cd integration-tests/  
cargo test
```
