# Bonds Contract Planning (Contract)

This document defines the bonds smart contract planning, including access control, core functions, state structure, and payment proof verification.

## Access Control and Agent Roles

Agent roles match contracts/security-mint-fund/src/types.rs (AgentRole):

- AddFund → required for add_bond
- RemoveFund → required for remove_bond
- UpdateFundState → required for update_bond_status
- Operator → required for claim and refund

Public access:

- invest → callable by any investor (subject to identity registry policies and payment-proof verification)

## Core Functions

- add_bond (Admin only)
- remove_bond (Admin only)
- invest (Investor on-chain; payment-proof verified)
- claim (Operator only; batch)
- refund (Operator only; batch)
- update_bond_status (Admin only)

### add_bond

**Agent Role:** AddFund

**Input Parameters:**
```rust path=null start=null
struct AddBondParams {
    maturity_date: Timestamp,
    interest_rate_type: InterestRateType,
    maximum_supply: Amount,
    minimum_raise_amount: Amount,
    lockup_period_duration: Duration,
    subscription_period_end: Timestamp,
    bond_price: Amount,
    presale_token_contract_address: ContractAddress,
    postsale_token_contract_address: ContractAddress,
}
```

**Event Emitted:**
```rust path=null start=null
struct BondAddedEvent {
    bond_address: ContractAddress,
    maturity_date: Timestamp,
    subscription_period_end: Timestamp,
}
```

**Functionality:**
Store Bond under key `postsale_token_contract_address` with initial status Active

### remove_bond

**Agent Role:** RemoveFund

**Input Parameters:**
```rust path=null start=null
struct RemoveBondParams {
    postsale_token_contract_address: ContractAddress,
}
```

**Event Emitted:**
```rust path=null start=null
struct BondRemovedEvent {
    bond_address: ContractAddress,
}
```

**Functionality:**
Remove Bond from state (archive for historical data)

### invest

**Agent Role:** Public (any investor with valid payment proof)

**Input Parameters:**
```rust path=null start=null
struct InvestParams {
    postsale_token_contract_address: ContractAddress,
    amount: Amount,
    payment_proof: PaymentProof,
}
```

**Event Emitted:**
```rust path=null start=null
struct BondInvestmentEvent {
    bond_address: ContractAddress,
    investor: AccountAddress,
    amount: Amount,
    reward_id: Vec<u8>,
    token_type: String, // "presale" or "postsale"
}
```

**Functionality:**
Verify payment proof signature and nonce, mint appropriate tokens based on subscription window

### claim (batch)

**Agent Role:** Operator

**Input Parameters:**
```rust path=null start=null
struct ClaimParams {
    postsale_token_contract_address: ContractAddress,
    account_addresses: Vec<AccountAddress>,
}
```

**Event Emitted:**
```rust path=null start=null
struct BondClaimedEvent {
    bond_address: ContractAddress,
    investor: AccountAddress,
    amount: Amount,
}
```

**Functionality:**
For successful bonds, convert pre-sale tokens to post-sale tokens for each investor

### refund (batch)

**Agent Role:** Operator

**Input Parameters:**
```rust path=null start=null
struct RefundParams {
    postsale_token_contract_address: ContractAddress,
    account_addresses: Vec<AccountAddress>,
}
```

**Event Emitted:**
```rust path=null start=null
struct BondRefundedEvent {
    bond_address: ContractAddress,
    investor: AccountAddress,
    amount: Amount,
}
```

**Functionality:**
For failed bonds, burn pre-sale tokens (refund handled off-chain)

### update_bond_status

**Agent Role:** UpdateFundState

**Input Parameters:**
```rust path=null start=null
struct UpdateBondStatusParams {
    postsale_token_contract_address: ContractAddress,
    new_status: BondStatus,
}
```

**Event Emitted:**
```rust path=null start=null
struct BondStatusUpdatedEvent {
    bond_address: ContractAddress,
    old_status: BondStatus,
    new_status: BondStatus,
}
```

**Functionality:**
Update Bond status (Active, Paused, Matured, Success, Failed)

## Payment Proof Verification (adapted from offchain_rewards::claim_reward)

- `reward_id` = off-chain payment tx hash
- Nonce per investor prevents replay; incremented after success
- Event carries `reward_id` for indexer correlation

Illustrative structure:

```rust path=null start=null
struct PaymentProof {
    reward_id: Vec<u8>,
    nonce: u64,
    signer: AccountAddress,
    signature: Signature,
}
```

Helper functions:
```rust path=null start=null
fn expected_message(investor, bond, amount, reward_id, nonce) -> Vec<u8>
fn verify_proof(proof: &PaymentProof, message: &[u8]) -> ContractResult<bool>
```

## State Structure

- bonds: Map<ContractAddress, Bond> (key = postsale_token_contract_address)
- investor_nonces: Map<AccountAddress, u64>

Bond fields:

- maturity_date, interest_rate_type, maximum_supply, minimum_raise_amount,
  lockup_period_duration, subscription_period_end, bond_price,
  presale_token_contract_address, postsale_token_contract_address,
  current_supply, status

## Enhanced Token Contract Integration

- Pre-sale tokens: security-sft-single (init with metadata; token ID 0 on init)
- Post-sale tokens: security-sft-multi (must support TokenMetadata initialization and later addToken)
- Bond identification: each bond uses two separate token contracts
- Post-sale token ID for bond = subscription_period_end timestamp

## State Structure

```rust path=null start=null
// Contract state uses postsale_token_contract_address as key
struct State {
    bonds: StateMap<ContractAddress, Bond>,
    investor_nonces: StateMap<AccountAddress, u64>,
}

struct Bond {
    maturity_date: Timestamp,
    interest_rate_type: InterestRateType,
    maximum_supply: Amount,
    minimum_raise_amount: Amount,
    lockup_period_duration: Duration,
    subscription_period_end: Timestamp,
    bond_price: Amount,
    presale_token_contract_address: ContractAddress,
    postsale_token_contract_address: ContractAddress,
    current_supply: Amount,
    status: BondStatus,
}
```
