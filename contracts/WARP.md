# WARP.md

This file provides guidance to WARP (warp.dev) when working with the smart contracts workspace.

## Workspace Overview

Concordium blockchain smart contracts for forest project tokenization, carbon credit management, P2P trading, and identity & compliance systems. All contracts are written in Rust using the Concordium smart contract framework.

## Development Environment

### Prerequisites

- Rust toolchain with Concordium extensions
- Node.js for build orchestration via yarn workspaces
- Concordium CLI tools installed locally
- Concordium wallet file for deployments

### Local Setup

```bash
# Install Rust and Concordium toolchain
cargo install --locked concordium-smart-contract-cli
# Install Node.js dependencies
yarn install
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

### Recommended Contract Structure

Based on Concordium best practices (following signature-verifier example), each contract should follow this directory structure:

```
contract-name/
├── Cargo.toml
├── src/
│   └── lib.rs          # Main contract implementation
└── tests/
    └── tests.rs        # Comprehensive contract tests
```

### Unit Testing

- Each contract has comprehensive unit tests in `tests/` directories
- Uses Concordium smart contract testing framework (`concordium-smart-contract-testing`)
- Tests cover initialization, state changes, and error conditions
- Test structure should include:
  - Contract deployment and initialization
  - Function calls with valid and invalid parameters
  - State verification after operations
  - Error condition testing

### Testing Framework Dependencies

In each contract's `Cargo.toml`, include the following dev-dependencies:

```toml
[dev-dependencies]
concordium-smart-contract-testing = { version = "4.4.0" }
# Add other test-specific dependencies as needed (e.g., rand for test data)
```

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
# Ensure wallet file is configured for deployment
# Default location: ./default_account.export
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

- **Network**: Set to testnet for development (`CONCORDIUM_NETWORK=testnet`)
- **Node URI**: Configure for Concordium testnet (`CONCORDIUM_NODE_URI=https://grpc.testnet.concordium.com:20000`)
- **Wallet**: Configure wallet file path for deployments (`CONCORDIUM_WALLET_PATH=./default_account.export`)

## Testing Patterns

### Test Structure Example

Based on the signature-verifier pattern, each contract test should:

```rust
use concordium_smart_contract_testing::*;
use your_contract::*;

const ALICE: AccountAddress = account_address!("2xBpaHottqhwFZURMZW4uZduQvpxNDSy46iXMYs9kceNGaPpZX");
const ALICE_ADDR: Address = Address::Account(ALICE);
const SIGNER: Signer = Signer::with_one_key();

#[test]
fn test_contract_functionality() {
    let mut chain = Chain::new();
    
    // Create an account
    chain.create_account(Account::new(ALICE, Amount::from_ccd(1000)));
    
    // Load and deploy the module
    let module = module_load_v1("concordium-out/module.wasm.v1").expect("Module exists.");
    let deployment = chain.module_deploy_v1(SIGNER, ALICE, module).expect("Module deploys.");
    
    // Initialize contract
    let init = chain.contract_init(SIGNER, ALICE, Energy::from(10_000), InitContractPayload {
        amount: Amount::zero(),
        mod_ref: deployment.module_reference,
        init_name: OwnedContractName::new_unchecked("init_contract_name".to_string()),
        param: OwnedParameter::empty(),
    }).expect("Initialize contract");
    
    // Test contract functions with both valid and invalid parameters
    // Assert expected behaviors
}
```

### Key Testing Principles

1. **Test both success and failure cases** - Always test valid parameters and invalid/edge cases
2. **Use realistic test data** - Create meaningful test scenarios that reflect real usage
3. **Verify state changes** - Check that contract state updates correctly after operations
4. **Test energy consumption** - Ensure operations complete within reasonable energy limits
5. **Module loading** - Tests should load the compiled WASM module from `concordium-out/`

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
