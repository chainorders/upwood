# Bond Requirements - M5 Planning

## 📋 Current Bond System Documentation

### Current Bond Functionality Overview

This document serves as the reference for all bond-related functionality in the Concordium RWA platform. It will be updated as changes are made during M5 development.

## 🏗️ Current Bond Architecture

### Security Versioned Token Creation (`security-sft-multi`)

**Contract**: `contracts/security-sft-multi/src/contract.rs`

- **Token Representation**: Versioned security tokens are represented by the `security-sft-multi` contract
- **Initialization**: 
  - Call the `init` function via blockchain transaction
  - Returns Smart Contract Address: `<SMART_CONTRACT_INDEX, SMART_CONTRACT_SUB_INDEX>` (SUB_INDEX always 0)
- **Token Creation**: 
  - Call `add_token` function in `contracts/security-sft-multi/src/contract.rs`
  - Emits `Cis2Event::TokenMetadata` event
  - Event processed by `backend/events_listener/src/processors/cis2_security.rs`

### Forest Project & Forest Project Token Initialization

**API Endpoints**: `backend/upwood/src/api/forest_project.rs`

1. **Project Creation**: 
   - Use API endpoint `admin_create_forest_project`
   
2. **Forest Project Token Contract**:
   - Create blockchain transaction for "forest project token contract" (same process as Security Versioned Token Creation)
   - Forest Project tokens represented by `security-sft-multi`
   - Update via `forest_project_token_contract_create` API endpoint
   - Request params include `SecurityTokenContractType` of type `Property`
   
3. **Forest Project Token**:
   - Create blockchain transaction for "forest project token" (same process as Security Versioned Token Creation)

### Bond Initialization Process

**Bond Tokens**: Represented by `security-sft-multi`

**Required Blockchain Transactions**:
1. Create Bond Pre-Sale Token Contract (Security Versioned Token Creation process)
2. Create Bond Pre-Sale Token (Security Versioned Token Creation process)
3. Create Bond Token Contract (Security Versioned Token Creation process)
4. Create Bond Token (Security Versioned Token Creation process)

**Fund Addition**:
- **Contract**: `contracts/security-mint-fund/src/lib.rs`
- **Function**: `add_fund` for `security_mint_fund` contract
- **Parameters** (`AddFundParams`):
  - `token`: Bond Pre-Sale Token Contract & Bond Pre-Sale Token
  - `security_token`: Bond Token Contract & Bond Token
  - `rate`: Conversion rate from token to security token
- **Events**: Emits `FundAddedEvent`
- **Processing**: Event processed by `backend/events_listener/src/processors/security_mint_fund.rs`
- **Database**: New "fund" record added to database

### Bond Maturity Process

**Contract**: `contracts/security-mint-fund/src/lib.rs`
**Function**: `update_fund_state`

**Parameters**:
- `security_token`: Bond Token Contract & Bond Token
- `state`: `Success` or `Fail`
  - `Success`: `Receiver` = Treasury Account Address (receives initially invested funds)

**Events**: Emits `FundStateUpdated` event
**Processing**: Event processed by `backend/events_listener/src/processors/security_mint_fund.rs`

### Bond Investment Process

**Flow**:
1. Call `transferInvest` function in `security-mint-fund` contract with `TransferInvestParams`
2. Contract calls Currency Contract `transfer` to move funds to itself
3. Contract calls its own `invest` function via internal transfer
4. `invest` function mints new "Bond Pre-Sale Token" in frozen state using `TokenAmountSecurity::new_frozen()`
5. Emits `Invested` event with investment details

### Bond Claims by Investors

**Function**: `claimInvestment` in `security-mint-fund` contract
**Parameters**: `ClaimInvestmentParams` with list of investments to claim

**Detailed Claim Logic**:
- **If `FundState::Open`**: Transaction fails (invalid state)
- **If `FundState::Fail`**: 
  - Returns original invested currency amount to investor
  - Burns the "Bond Pre-Sale Token"
  - Emits `InvestmentCancelled` event
- **If `FundState::Success`**: 
  - Transfers currency amount to the `funds_receiver` (Treasury)
  - **If security_token == initial token**: Only unfreezes the tokens (same token scenario)
  - **If security_token != initial token**: Burns "Bond Pre-Sale Token" and mints "Bond Token" in unfrozen state
  - Emits `InvestmentClaimed` event

**Authorization**: Investors can claim their own investments, or agents with `AgentRole::Operator` can claim on behalf of others

## 📝 Business Requirement IDs Addressed
*To be populated when specific M5 requirements are discussed*

- [ ] REQ-BONDS-[ID-1]: [Brief description]
- [ ] REQ-BONDS-[ID-2]: [Brief description]
- [ ] REQ-BONDS-[ID-3]: [Brief description]

## 🎯 Affected Codebase Areas

### Current Bond-Related Files
- [ ] **Smart Contracts**: 
  - `security-sft-multi` (token representation)
  - `security-mint-fund` (fund management)
- [ ] **Backend Services**: 
  - `backend/upwood/src/api/forest_project.rs` (project APIs)
  - `backend/events_listener/src/processors/cis2_security.rs` (token events)
  - `backend/events_listener/src/processors/security_mint_fund.rs` (fund events)
- [ ] **Frontend Application**: [To be documented when requirements are discussed]
- [ ] **Database Schema**: Fund-related tables
- [ ] **Infrastructure**: [To be documented when requirements are discussed]

## 🔧 Smart Contracts Changes

### security-sft-multi Contract Changes
*To be populated based on M5 requirements*

### security-mint-fund Contract Changes
*To be populated based on M5 requirements*

## 🖥️ Backend API Changes

### Event Processing Changes
*To be populated based on M5 requirements*

### REST API Changes
*To be populated based on M5 requirements*

### Database Schema Changes
*To be populated based on M5 requirements*

## 🎨 Frontend UI Changes

### New Components/Pages
*To be populated based on M5 requirements*

### Existing Component Updates
*To be populated based on M5 requirements*

## ☁️ Infrastructure Changes
*To be populated based on M5 requirements*

## 📝 Implementation Notes

### Current System Dependencies
1. **Currency Contract**: Required for investment transfers
2. **Treasury Account**: Required for bond maturity success scenarios
3. **Event Processing Chain**: cis2_security.rs → security_mint_fund.rs processors

### Known System Flows
1. **Bond Creation Flow**: Contract creation → Token creation → Fund addition
2. **Investment Flow**: Currency transfer → Investment processing → Token minting
3. **Maturity Flow**: Admin state update → Event processing → Database update
4. **Claim Flow**: User claim → State check → Token/fund distribution

### Current Risks and Considerations
- **Risk 1**: Bond state management depends on admin manual updates
- **Risk 2**: Investment/claim timing dependencies on blockchain event processing

## 🔄 Progress Tracking

### Overall Progress
- **Documentation**: ✅ Current system documented
- **M5 Requirements**: ⏳ Pending business requirement analysis
- **Implementation Todos**: ⏳ Pending requirement breakdown

### Last Updated
- **Date**: 2025-09-01
- **Updated By**: WARP AI Assistant
- **Changes Made**: Initial documentation of current bond system architecture and flows

---

**Status**: Current system documented - Ready for M5 business requirement analysis
