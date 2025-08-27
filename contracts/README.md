# 📦 Smart Contracts Workspace

**Concordium blockchain smart contracts for Upwood's Real World Asset (RWA) platform, specializing in forest project tokenization and environmental asset management.**

## 🌟 Overview

This workspace contains all smart contracts written in Rust for the Concordium blockchain. The contracts enable forest project tokenization, carbon credit management, P2P trading, investment fund management, and comprehensive identity & compliance systems.

## 🏗️ Directory Structure

```
contracts/
├── compliance/                    # Regulatory compliance (deprecated)
├── concordium-protocols/          # Shared protocol library
├── euroe/                         # EUROe stablecoin integration
├── identity-registry/             # Identity & access control
├── integration-tests/             # Cross-contract integration tests
├── nft-multi-rewarded/           # Authentication NFTs
├── offchain-rewards/             # Off-chain rewards (deprecated)
├── security-mint-fund/           # Investment fund & bonds
├── security-p2p-trading/         # P2P marketplace
├── security-sft-multi/           # Core forest project representation
├── security-sft-multi-yielder/   # Yield distribution (deprecated)
├── security-sft-single/          # Carbon credit tokenization
└── sponsor/                      # Transaction sponsorship (legacy)
```

## 🚀 Development Environment Setup

### Using VS Code Dev Containers

1. **Open VS Code in repository root**

   ```bash
   cd /path/to/concordium-rwa
   code .
   ```

2. **Open in Dev Container**
   - Press `F1` or `Ctrl+Shift+P`
   - Type: `Dev Containers: Reopen in Container`
   - Select: **contracts**

3. **Container Setup**
   - The container will automatically install Rust, Concordium toolchain, and Node.js
   - Wait for the setup to complete
   - Terminal will show available yarn scripts upon completion

### Wallet Setup

```bash
# Export your Concordium testnet wallet and place it in:
.devcontainer/contracts/default_account.export

# The container automatically uses this wallet for deployments
```

## 🛠️ Available Scripts

All scripts are defined in `package.json` and can be run with `yarn <script>`:

### Building

```bash
yarn build    # Build all contracts in workspace
yarn clean    # Clean all build artifacts
yarn format   # Format all Rust code
```

### Testing

```bash
yarn test     # Run all contract tests
```

### Deployment

```bash
yarn deploy   # Deploy all contracts to testnet
```

### Individual Contract Operations

Each contract subdirectory has its own `package.json` with specific scripts:

```bash
# Navigate to specific contract
cd security-sft-multi/
yarn build    # Build this contract only
yarn test     # Test this contract only
yarn deploy   # Deploy this contract only
```

## Contracts

# [rwa_compliance](./compliance/src/compliance/mod.rs)

## Description

**COMPLIANCE MANAGEMENT CONTRACT** - Previously designed to manage and enforce complex compliance rules for token transactions with modular compliance system and agent management.

**Status**: ❌ **REMOVED** - Will be removed in next version

**Reason for Removal**:

- **Simplified Requirements** - Complex compliance functionality not needed for forest projects
- **Whitelist/Blacklist Approach** - Simple identity whitelisting/blacklisting through identity registry is sufficient
- **Reduced Complexity** - Eliminates need for modular compliance system and multiple validation layers
- **Part of Core RWA Protocol** - While part of the broader RWA protocol, not required for current forest project use case

**Contract is initiated with** compliance modules and initial agents

**Functions (DEPRECATED):**

- `supports` - Checks if the contract supports a given standard
- `isAgent` - Returns true if the given address is an agent
- `agents` - Returns the list of agents
- `addAgent` - Adds the given address as an agent (owner only)
- `removeAgent` - Removes the given address as an agent (owner only)
- `addModule` - Adds a new compliance module to the contract
- `removeModule` - Removes a compliance module from the contract
- `modules` - Returns the list of compliance modules
- `burned` - Handles token burn events and propagates to all modules
- `minted` - Handles token mint events and propagates to all modules
- `transferred` - Handles token transfer events and propagates to all modules
- `canTransfer` - Checks if a transfer can be made according to all compliance modules

# [rwa_compliance_module_allowed_nationalities](./compliance/src/compliance_modules/allowed_nationalities/mod.rs)

## Description

**NATIONALITY COMPLIANCE MODULE** - Previously validated token transfers based on allowed nationalities by checking identity registry attributes.

**Status**: ❌ **REMOVED** - Will be removed in next version

**Reason for Removal**:

- **Over-engineered** - Nationality-based compliance too complex for current forest project needs
- **Simple Identity Management** - Basic account address whitelisting/blacklisting through identity registry provides sufficient access control
- **Streamlined Approach** - Eliminates complex attribute checking in favor of straightforward identity verification

**Contract is initiated with** identity registry address and allowed nationalities list

**Functions (DEPRECATED):**

- `burned` - Handles token burn events (no-op implementation)
- `canTransfer` - Validates if a transfer can be made based on recipient's nationality
- `minted` - Handles token mint events (no-op implementation)
- `transferred` - Handles token transfer events (no-op implementation)

# [rwa_identity_registry](./identity-registry/src/lib.rs)

## Description

**📝 IDENTITY & ACCESS CONTROL CONTRACT** - A comprehensive solution for managing forest project access control through identity registration and verification. Currently provides whitelisting functionality for approved participants and will be enhanced with blacklisting capabilities.

**Current Functionality**: Identity whitelisting through registration and verification system
**Planned Enhancement**: Addition of blacklisting functionality for comprehensive access control

**Status**: ✅ **RETAINED & ENHANCED** - Will be updated with blacklisting functionality

**Forest Project Access Control**: Manages who can participate in forest projects by maintaining verified identity registrations (whitelist) and will add the ability to explicitly block problematic addresses (blacklist).

**Contract is initiated with** initial list of agents (owner becomes default agent)

**Current Functions:**

- `supports` - Checks if the contract supports a given standard
- `isAgent` - Returns true if the given address is an agent
- `agents` - Returns the list of agents
- `addAgent` - Adds the given address as an agent (owner only)
- `removeAgent` - Removes the given address as an agent (owner only)
- `registerIdentity` - Registers a new identity for an address (whitelist - agents only)
- `isVerified` - Checks if an identity is verified by checking all issuer credentials
- `deleteIdentity` - Removes an identity from the registry (agents only)
- `hasIdentity` - Returns true if the address has a registered identity (whitelisted)
- `getIdentity` - Returns the identity details for a specific address
- `isIssuer` - Returns true if the given address is an issuer
- `issuers` - Returns the list of issuers
- `addIssuer` - Adds a new issuer to the contract (owner only)
- `removeIssuer` - Removes an issuer from the contract (owner only)

**Planned Blacklist Functions** (Next Version):

- `blacklistAddress` - Explicitly blocks an address from forest project participation
- `unblacklistAddress` - Removes an address from the blacklist
- `isBlacklisted` - Checks if an address is blacklisted
- `getBlacklistedAddresses` - Returns list of blacklisted addresses

# [security_sft_single](./security-sft-single/src/contract.rs)

## Description

**🍃 CARBON CREDIT REPRESENTATION CONTRACT** - A CIS-2 compatible contract specifically designed to represent carbon credits on-chain. This contract manages single security tokens with comprehensive features including freezing, pausing, compliance integration, and identity registry support for carbon credit tokenization.

**Carbon Credit Use Case**: Each instance of this contract represents a specific type of carbon credit, enabling transparent and compliant trading of environmental assets within the forest project ecosystem.

**Status**: ✅ **RETAINED** - No changes planned for next version

**Contract is initiated with** security parameters (compliance and identity registry), agents, and token metadata

**Functions:**

- `identityRegistry` - Returns the address of the identity registry contract
- `setIdentityRegistry` - Sets the identity registry contract address
- `compliance` - Returns the address of the compliance contract
- `setCompliance` - Sets the compliance contract address
- `addAgent` - Adds a new agent with specific roles
- `removeAgent` - Removes an agent from the contract
- `isAgent` - Checks if an address is an agent with specific roles
- `freeze` - Freezes a specific amount of tokens for a holder
- `unFreeze` - Unfreezes a specific amount of tokens for a holder
- `balanceOfFrozen` - Returns the frozen balance for given addresses
- `balanceOfUnFrozen` - Returns the unfrozen balance for given addresses
- `pause` - Pauses operations for specific tokens
- `unPause` - Unpauses operations for specific tokens
- `isPaused` - Returns if tokens are paused
- `recover` - Facilitates recovery of a lost account
- `recoveryAddress` - Returns the recovery address for an account
- `updateOperator` - Updates operator permissions for token management
- `operatorOf` - Checks if an address is an operator for a token owner
- `tokenMetadata` - Returns metadata for tokens
- `setTokenMetadata` - Updates token metadata (agents only)
- `supports` - Checks if contract supports specific standards
- `mint` - Creates new tokens and adds them to total supply
- `transfer` - Executes compliant token transfers
- `burn` - Burns tokens from holder's account
- `balanceOf` - Returns total token balances for addresses

# [security_sft_multi](./security-sft-multi/src/contract.rs)

## Description

**🌳 FOREST PROJECT REPRESENTATION CONTRACT** - This is the core contract that represents forest projects on-chain. Each instance of this contract represents one specific forest project. Each token within the contract represents a version of the forest project, allowing for proper yield management and project state tracking.

**Forest Project Versioning**: Tokens are versioned to distinguish project states over time (e.g., Version 1: yield payment pending → Version 2: yields paid). This enables seamless management of forest project lifecycle and investor rewards.

**Status**: ✅ **RETAINED** - No changes planned for next version

**Contract is initiated with** security parameters (compliance and identity registry) and agents

**Functions:**

- `identityRegistry` - Returns the address of the identity registry contract
- `setIdentityRegistry` - Sets the identity registry contract address
- `compliance` - Returns the address of the compliance contract
- `setCompliance` - Sets the compliance contract address
- `isAgent` - Checks if an address is an agent with specific roles
- `addAgent` - Adds a new agent with specific roles (owner only)
- `removeAgent` - Removes an agent from the contract (owner only)
- `freeze` - Freezes a specific amount of tokens for a holder
- `unFreeze` - Unfreezes a specific amount of tokens for a holder
- `balanceOfFrozen` - Returns the frozen balance for given addresses and tokens
- `balanceOfUnFrozen` - Returns the unfrozen balance for given addresses and tokens
- `pause` - Pauses operations for specific tokens
- `unPause` - Unpauses operations for specific tokens
- `isPaused` - Returns if tokens are paused
- `recover` - Facilitates recovery of a lost account
- `addToken` - Adds a new token to the contract with metadata
- `updateOperator` - Updates operator permissions for token management
- `operatorOf` - Checks if an address is an operator for a token owner
- `tokenMetadata` - Returns metadata for tokens
- `supports` - Checks if contract supports specific standards
- `mint` - Creates new tokens and adds them to total supply
- `transfer` - Executes compliant token transfers
- `burn` - Burns tokens from holder's account
- `balanceOf` - Returns total token balances for addresses

# [security_sft_multi_yielder](./security-sft-multi-yielder/src/lib.rs)

## Description

**YIELD DISTRIBUTION CONTRACT** - Previously managed yield distribution across multiple forest project tokens with configurable yield calculations and treasury management.

**Status**: ❌ **REMOVED** - Will be removed in next version

**Replacement System**:

- **Yields** will now be paid in **Concordium Protocol Level Tokens (PLT)**
- **Distribution** will be handled by the **Cloud Wallet** (programmatic backend-controlled wallet)
- **Calculation Logic** will be migrated to the backend system
- **Benefits**: Simplified architecture, better scalability, protocol-level token support

**Contract is initiated with** treasury address and initial agents

**Functions (DEPRECATED):**

- `setTreasury` - Sets the treasury address for yield distribution
- `getTreasury` - Returns the current treasury address
- `addAgent` - Adds a new agent with specific roles (owner only)
- `removeAgent` - Removes an agent from the contract (owner only)
- `upsertYield` - Adds or updates yield configuration for security tokens
- `removeYield` - Removes yield configuration for specific tokens
- `yieldFor` - Calculates and distributes yields to token holders

# [security_p2p_trading](./security-p2p-trading/src/lib.rs)

## Description

**🔄 FOREST PROJECT P2P TRADING CONTRACT** - A peer-to-peer trading contract for forest project security tokens supporting both mint and transfer markets with configurable rates and liquidity providers. Enables direct trading of forest project tokens and carbon credits between participants.

**Status**: ✅ **RETAINED & UPDATED** - Updated for PLT payment integration

**Payment System Changes**:

- **🔄 PLT Payment Integration** - Users pay PLT to predefined accounts; indexer detects payments
- **Transaction Hash Verification** - Users present PLT payment transaction hash to Cloud Wallet API for verification
- **Non-Payable Methods** - Previously payable functions now non-payable and restricted to authorized agents
- **Cloud Wallet Control** - Only the Cloud Wallet can execute trading operations after verifying PLT payments

**Forest Project Trading Use Case**: Facilitates peer-to-peer trading of forest project tokens and carbon credits, allowing investors to buy/sell directly through liquidity providers with transparent pricing mechanisms managed through the Cloud Wallet.

**Contract is initiated with** currency token address and initial agents

**Functions:**

- `addAgent` - Adds a new agent with specific roles (owner only)
- `removeAgent` - Removes an agent from the contract (owner only)
- `addMarket` - Creates a new market for trading (mint or transfer type)
- `removeMarket` - Removes an existing market
- `getMarket` - Returns market details for a specific token contract
- `sell` - **UPDATED** - Agent-only (Cloud Wallet) function to process forest project token sales at buy rate
- `buy` - **UPDATED** - Agent-only (Cloud Wallet) function to process forest project token purchases at sell rate
- `mint` - **UPDATED** - Agent-only (Cloud Wallet) function to mint tokens for mint markets

# [security_mint_fund](./security-mint-fund/src/lib.rs)

## Description

**🌳 FOREST PROJECT BOND & INVESTMENT CONTRACT** - A contract for managing forest project bonds and investment funds with support for bond tranches. Converts currency tokens to security tokens representing forest project bonds at specified rates with agent-controlled fund lifecycle.

**Status**: ✅ **RETAINED & ENHANCED** - Will be updated with bond and bond tranche functionality

**Bond & Tranche Updates**:

- **Bond Parameters** - Enhanced parameters to support forest project bond characteristics
- **Bond Tranches** - Support for different bond tranche types with varying risk/return profiles
- **Forest Project Focus** - Specialized for forest project bond issuance and management
- **🔄 PLT Payment Integration** - Users pay PLT to predefined accounts; indexer detects payments
- **Transaction Hash Verification** - Users present PLT payment transaction hash to Cloud Wallet API for verification
- **Non-Payable Methods** - Previously payable functions now non-payable and restricted to authorized agents

**Forest Project Bond Use Case**: Enables issuing bonds for forest projects where investors can purchase different tranches based on their risk appetite and return expectations.

**Contract is initiated with** currency token address and initial agents

**Current Functions:**

- `addAgent` - Adds a new agent with specific roles (owner only)
- `removeAgent` - Removes an agent from the contract (owner only)
- `addFund` - Creates a new investment fund with conversion rate
- `removeFund` - Removes an existing fund (if no active investments)
- `updateFundState` - Updates fund state (open/success/fail)
- `transferInvest` - **UPDATED** - Agent-only (Cloud Wallet) function to initiate investment after PLT payment verification
- `invest` - **UPDATED** - Non-payable, agent-only function to process investments, record amounts, convert to security tokens, and mint frozen tokens
- `claimInvestment` - **UPDATED** - Agent-only function to process investment claims based on fund outcome:
  - **Success**: Transfers currency to fund receiver, then unfreezes/burns/mints tokens as needed
  - **Fail**: Returns invested currency to investor and burns initially minted frozen tokens

**Enhanced Bond Functions** (Next Version):

- `createBond` - Creates a new forest project bond with tranche specifications
- `addTranche` - Adds a new tranche to an existing bond
- `updateBondParams` - Updates bond parameters for forest project specifics
- `getTranche` - Returns tranche details for a specific bond
- `calculateTrancheReturns` - Calculates returns based on tranche type and performance

# [nft_multi_rewarded](./nft-multi-rewarded/src/lib.rs) → [nft_multi]

## Description

**🔐 AUTHENTICATION NFT CONTRACT** - A CIS-2 compatible NFT contract designed for authentication purposes. This contract will be renamed to `nft_multi` and repurposed from reward distribution to authentication NFT management.

**Status**: 🔄 **RENAMED & MODIFIED** - Will become `nft_multi` with authentication focus

**Changes in Next Version**:

- **Contract Name**: `nft_multi_rewarded` → `nft_multi`
- **Purpose**: Reward NFTs → Authentication NFTs
- **Removed**: All yield/reward distribution functionality
- **Focus**: Pure NFT management for authentication use cases

**Contract is initiated with** ~~reward token configuration~~ basic NFT configuration

**Functions:**

- `isAgent` - Checks if an address is an agent
- `addAgent` - Adds a new agent to the contract (owner only)
- `removeAgent` - Removes an agent from the contract (owner only)
- `updateOperator` - Updates operator permissions for NFT management
- `operatorOf` - Checks if an address is an operator for an NFT owner
- `balanceOf` - Returns NFT balances for addresses
- `transfer` - Transfers NFTs between addresses
- `mint` - Mints new authentication NFTs to specified addresses
- `burn` - Burns NFTs from holder's account
- ~~`distributeReward`~~ - **REMOVED** - Reward distribution functionality removed

# [offchain_rewards](./offchain-rewards/src/lib.rs)

## Description

**OFF-CHAIN REWARD CLAIMS CONTRACT** - Previously managed off-chain reward claims with cryptographic signature verification and nonce-based replay protection.

**Status**: ❌ **REMOVED** - Will be removed in next version

**Reason for Removal**:

- **PLT Yields Only** - All yields will now be distributed using **Concordium Protocol Level Tokens (PLT)**
- **No Off-chain Claims** - The new system eliminates the need for off-chain reward claiming mechanisms
- **Simplified Architecture** - Direct PLT distribution through Cloud Wallet removes complexity of signature verification and nonce management

**Contract is initiated with** treasury address

**Functions (DEPRECATED):**

- `isAgent` - Checks if an address is an agent
- `addAgent` - Adds a new agent to the contract (owner only)
- `removeAgent` - Removes an agent from the contract (owner only)
- `claimReward` - Claims rewards with cryptographic signature verification
