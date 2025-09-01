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

### M5 Enhanced Bond System Requirements

- [ ] **FR-BT-1**: Bond token issuance - Admin can create new bond offerings with configurable parameters
- [ ] **FR-BT-2**: Interest rate configuration - Admin can set fixed/variable interest rates and yield payment dates
- [ ] **FR-BT-3**: Bond token trading controls - Admin can set lock-up periods, trading restrictions, and fees
- [ ] **FR-BT-4**: Yield distribution - Admin can configure yield calculations and automated distribution
- [ ] **FR-BT-5**: Maturity handling - Automated maturity processing, token burning, and face value payments

### Key System Changes Required

- [ ] **REMOVE**: Complete fund-based functionality (security-mint-fund approach)
- [ ] **ENHANCE**: Bond token system with advanced parameters and controls
- [ ] **ADD**: Sophisticated yield calculation and distribution system
- [ ] **ADD**: Advanced trading controls and restrictions
- [ ] **ADD**: Automated maturity and redemption handling

## 🎯 M5 Bond Requirements Detailed Specifications

### FR-BT-1: Bond Token Issuance
**Admin can create new bond offerings with configurable parameters:**

- **Maturity Period**: Admin-defined specific date
- **Interest Rate Type**: Fixed or Fixed+Variable
- **Maximum Supply**: Total bonds that can be issued
- **Minimum Raise Amount**: Threshold for successful bond offering
- **Lock-up Period Duration**: Time before transfers are allowed
- **Bond Tranche Amount**: Admin-defined, manually triggered tranches
  - Custom price per tranche
  - Custom maturity period per tranche
- **Bond Price**: Price to reference currency
- **Unique Identifier**: Each bond offering has unique ID
- **Metadata Management**: 
  - Add metadata at any point
  - Environmental impact reports
  - ISIN numbers (stored as PDF files on IPFS)

### FR-BT-2: Interest Rate Configuration
**Admin can configure interest rates and yield payment schedules:**

- **Fixed Interest Rate**: Set in yearly percentage (APY)
- **Yield Payment Dates**: Admin-defined schedule
- **Variable Interest Bonds**:
  - Fixed component (e.g., 5% base rate)
  - Variable component based on manual admin input
  - Variable amount in € based on underlying asset performance
  - Coupon payment periods (typically 1 year)

### FR-BT-3: Bond Token Trading Controls
**Admin can control bond trading and transfers:**

- **Initial Lock-up Period**: Prevents transfers until conditions met
  - Triggered when tranche amount sold > minimum threshold
  - Time-based: Days, Months, Years
- **Manual Trading Enablement**: Admin function to enable free trading after successful capital raise
- **Refund Mechanism**: Admin function to refund if minimum raise not met
- **Transfer Restrictions**: 
  - Yield/maturity payments to whitelisted wallets only
  - Force transfer for law enforcement purposes
- **Secondary Market Fees**: Admin can impose trading fees

### FR-BT-4: Yield Distribution
**Admin can configure and manage yield payments:**

- **Interest Calculation**: Based on provided formulas (page 12-13 of requirements)
- **Annual Coupon Payments**: In configurable stablecoins
- **Manual Trigger**: Admin approves yield payments before execution date
- **Automated Distribution**: Using cloud wallets for whitelisted addresses
- **Yield Calculation**: Based on holding period and amount held

### FR-BT-5: Maturity Handling
**Automated maturity processing and redemption:**

- **Face Value Payment Approval**: Admin can approve before payment date
- **Maturity Date Execution**: Face value payment at maturity date
- **Automatic Token Burning**: Upon maturity payment and last coupon payment
- **Pre-Maturity Freeze**: Tokens freeze one day before maturity
- **Redemption Period**: Automatic burning after redemption period for non-whitelisted wallets
- **Obligation Time Bar**: XX years for non-whitelisted wallets

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

### security-sft-multi Contract Enhancement

- [ ] **TODO #1**: Add enhanced bond token parameters to SecurityTokenState struct
  - **File**: `contracts/security-sft-multi/src/state.rs`
  - **Struct**: `SecurityTokenState`
  - **Change Details**: Add fields for maturity_date, interest_rate_type, max_supply, min_raise_amount, lockup_period, tranche_config, bond_price, isin_metadata_url
  - **Priority**: P0
  - **Estimated Effort**: M

- [ ] **TODO #2**: Create new AddBondTokenParams type for enhanced bond creation
  - **File**: `contracts/security-sft-multi/src/types.rs`
  - **Function/Struct**: New `AddBondTokenParams` struct
  - **Change Details**: Create comprehensive parameter struct with all FR-BT-1 fields
  - **Priority**: P0
  - **Estimated Effort**: S

- [ ] **TODO #3**: Enhance addToken function to support bond-specific parameters
  - **File**: `contracts/security-sft-multi/src/contract.rs`
  - **Function**: `add_token` (line 900)
  - **Change Details**: Add bond-specific validation and parameter handling for maturity dates, interest rates, supply limits
  - **Priority**: P0
  - **Estimated Effort**: L

- [ ] **TODO #4**: Add trading control functions for lock-up periods
  - **File**: `contracts/security-sft-multi/src/contract.rs`
  - **Function/Struct**: New `setLockupPeriod`, `enableFreeTrading`, `setTransferRestrictions` functions
  - **Change Details**: Implement FR-BT-3 trading controls with time-based lockups and whitelist restrictions
  - **Priority**: P0
  - **Estimated Effort**: XL

- [ ] **TODO #5**: Add yield configuration and distribution functions
  - **File**: `contracts/security-sft-multi/src/contract.rs`
  - **Function/Struct**: New `configureYield`, `distributeYield`, `calculateInterest` functions
  - **Change Details**: Implement FR-BT-4 yield management with fixed/variable rates and automated distribution
  - **Priority**: P0
  - **Estimated Effort**: XL

- [ ] **TODO #6**: Add maturity handling and automatic burning functions
  - **File**: `contracts/security-sft-multi/src/contract.rs`
  - **Function/Struct**: New `handleMaturity`, `autoRedeemTokens`, `freezePreMaturity` functions
  - **Change Details**: Implement FR-BT-5 automated maturity processing and redemption
  - **Priority**: P0
  - **Estimated Effort**: XL

- [ ] **TODO #7**: Add new agent roles for bond management
  - **File**: `contracts/security-sft-multi/src/types.rs`
  - **Function/Struct**: `AgentRole` enum (line 17)
  - **Change Details**: Add roles: ManageBonds, ConfigureYield, HandleMaturity, SetTradingControls
  - **Priority**: P0
  - **Estimated Effort**: S

### security-mint-fund Contract - REMOVAL

- [ ] **TODO #8**: Remove security-mint-fund contract functionality
  - **File**: `contracts/security-mint-fund/src/lib.rs`
  - **Change Details**: Mark contract as deprecated, remove from build process, migrate any required functionality to security-sft-multi
  - **Priority**: P1
  - **Estimated Effort**: L

### New Bond Management Contract (Optional)

- [ ] **TODO #9**: Consider creating dedicated bond management contract
  - **File**: `contracts/security-bond-manager/src/lib.rs` (new)
  - **Change Details**: Extract complex bond logic from security-sft-multi into dedicated contract for better separation of concerns
  - **Priority**: P2
  - **Estimated Effort**: XL

## 🖥️ Backend API Changes

### Event Processing Changes

- [ ] **TODO #10**: Remove security-mint-fund event processor
  - **File**: `backend/events_listener/src/processors/security_mint_fund.rs`
  - **Change Details**: Remove processor and associated event handling for fund-based system
  - **Priority**: P1
  - **Estimated Effort**: S

- [ ] **TODO #11**: Enhance cis2_security processor for bond events
  - **File**: `backend/events_listener/src/processors/cis2_security.rs`
  - **Function**: Event processing functions
  - **Change Details**: Add processing for new bond-specific events: YieldDistributed, MaturityHandled, TradingControlsUpdated, BondConfigured
  - **Priority**: P0
  - **Estimated Effort**: L

### REST API Changes

- [ ] **TODO #12**: Create bond management API endpoints
  - **File**: `backend/app_api/src/handlers/bonds.rs` (new)
  - **Endpoint**: `POST /api/v1/admin/bonds/create`
  - **Change Details**: Create endpoint for FR-BT-1 bond token issuance with all configurable parameters
  - **Priority**: P0
  - **Estimated Effort**: M

- [ ] **TODO #13**: Add yield configuration API endpoints
  - **File**: `backend/app_api/src/handlers/bonds.rs`
  - **Endpoint**: `POST /api/v1/admin/bonds/{id}/yield/configure`
  - **Change Details**: Create endpoints for FR-BT-2 interest rate configuration and payment schedules
  - **Priority**: P0
  - **Estimated Effort**: M

- [ ] **TODO #14**: Add trading controls API endpoints
  - **File**: `backend/app_api/src/handlers/bonds.rs`
  - **Endpoint**: `POST /api/v1/admin/bonds/{id}/trading-controls`
  - **Change Details**: Create endpoints for FR-BT-3 lock-up periods, trading restrictions, and fees
  - **Priority**: P0
  - **Estimated Effort**: M

- [ ] **TODO #15**: Add yield distribution API endpoints
  - **File**: `backend/app_api/src/handlers/bonds.rs`
  - **Endpoint**: `POST /api/v1/admin/bonds/{id}/yield/distribute`
  - **Change Details**: Create endpoints for FR-BT-4 manual yield triggering and automated distribution
  - **Priority**: P0
  - **Estimated Effort**: L

- [ ] **TODO #16**: Add maturity handling API endpoints
  - **File**: `backend/app_api/src/handlers/bonds.rs`
  - **Endpoint**: `POST /api/v1/admin/bonds/{id}/maturity/approve`
  - **Change Details**: Create endpoints for FR-BT-5 maturity approval and face value payments
  - **Priority**: P0
  - **Estimated Effort**: M

- [ ] **TODO #17**: Remove fund-related API endpoints
  - **File**: `backend/upwood/src/api/forest_project.rs`
  - **Function**: Fund-related endpoints
  - **Change Details**: Remove or refactor endpoints that depend on security-mint-fund functionality
  - **Priority**: P1
  - **Estimated Effort**: M

### Database Schema Changes

- [ ] **TODO #18**: Create bond_offerings table
  - **Migration File**: `backend/shared/migrations/[timestamp]_create_bond_offerings.sql`
  - **Tables Affected**: New bond_offerings table
  - **Change Details**: Create table with columns for maturity_date, interest_rate_type, max_supply, min_raise_amount, lockup_period, bond_price, isin_metadata_url
  - **Priority**: P0
  - **Estimated Effort**: S

- [ ] **TODO #19**: Create bond_tranches table
  - **Migration File**: `backend/shared/migrations/[timestamp]_create_bond_tranches.sql`
  - **Tables Affected**: New bond_tranches table
  - **Change Details**: Create table for tracking individual bond tranches with custom prices and maturity periods
  - **Priority**: P0
  - **Estimated Effort**: S

- [ ] **TODO #20**: Create yield_configurations table
  - **Migration File**: `backend/shared/migrations/[timestamp]_create_yield_configurations.sql`
  - **Tables Affected**: New yield_configurations table
  - **Change Details**: Create table for fixed/variable interest rates and payment schedules
  - **Priority**: P0
  - **Estimated Effort**: S

- [ ] **TODO #21**: Create yield_distributions table
  - **Migration File**: `backend/shared/migrations/[timestamp]_create_yield_distributions.sql`
  - **Tables Affected**: New yield_distributions table
  - **Change Details**: Create table for tracking yield payments and distribution history
  - **Priority**: P0
  - **Estimated Effort**: S

- [ ] **TODO #22**: Create trading_controls table
  - **Migration File**: `backend/shared/migrations/[timestamp]_create_trading_controls.sql`
  - **Tables Affected**: New trading_controls table
  - **Change Details**: Create table for lock-up periods, transfer restrictions, and trading fees
  - **Priority**: P0
  - **Estimated Effort**: S

- [ ] **TODO #23**: Remove fund-related tables
  - **Migration File**: `backend/shared/migrations/[timestamp]_remove_fund_tables.sql`
  - **Tables Affected**: funds, fund_investments, fund_states tables
  - **Change Details**: Remove tables related to security-mint-fund functionality
  - **Priority**: P1
  - **Estimated Effort**: S

- [ ] **TODO #24**: Create database model structs for bond system
  - **File**: `backend/shared/src/db/bonds.rs` (new)
  - **Change Details**: Create Diesel model structs for all new bond-related tables
  - **Priority**: P0
  - **Estimated Effort**: M

## 🎨 Frontend UI Changes

### New Components/Pages

- [ ] **TODO #25**: Create BondCreationForm component
  - **File**: `frontend-app/src/adminSection/components/BondCreationForm.tsx`
  - **Component Type**: Admin Form Component
  - **Change Details**: Create form for FR-BT-1 bond token issuance with all configurable parameters (maturity, interest rate, supply limits, tranches)
  - **Priority**: P0
  - **Estimated Effort**: L

- [ ] **TODO #26**: Create YieldConfigurationPanel component
  - **File**: `frontend-app/src/adminSection/components/YieldConfigurationPanel.tsx`
  - **Component Type**: Admin Panel Component
  - **Change Details**: Create interface for FR-BT-2 interest rate configuration (fixed/variable rates, payment schedules)
  - **Priority**: P0
  - **Estimated Effort**: M

- [ ] **TODO #27**: Create TradingControlsManager component
  - **File**: `frontend-app/src/adminSection/components/TradingControlsManager.tsx`
  - **Component Type**: Admin Management Component
  - **Change Details**: Create interface for FR-BT-3 trading controls (lock-up periods, restrictions, fees)
  - **Priority**: P0
  - **Estimated Effort**: M

- [ ] **TODO #28**: Create YieldDistributionCenter component
  - **File**: `frontend-app/src/adminSection/components/YieldDistributionCenter.tsx`
  - **Component Type**: Admin Action Component
  - **Change Details**: Create interface for FR-BT-4 yield distribution management and manual triggering
  - **Priority**: P0
  - **Estimated Effort**: L

- [ ] **TODO #29**: Create MaturityManagementPanel component
  - **File**: `frontend-app/src/adminSection/components/MaturityManagementPanel.tsx`
  - **Component Type**: Admin Panel Component
  - **Change Details**: Create interface for FR-BT-5 maturity handling (approval, face value payments, redemption)
  - **Priority**: P0
  - **Estimated Effort**: M

- [ ] **TODO #30**: Create BondOverviewDashboard component
  - **File**: `frontend-app/src/adminSection/components/BondOverviewDashboard.tsx`
  - **Component Type**: Dashboard Component
  - **Change Details**: Create comprehensive dashboard showing all bond offerings, their status, and key metrics
  - **Priority**: P0
  - **Estimated Effort**: L

### Existing Component Updates

- [ ] **TODO #31**: Update FundsTable component to BondsTable
  - **File**: `frontend-app/src/adminSection/components/FundsTable.tsx`
  - **Function**: Component logic and data handling
  - **Change Details**: Refactor from fund-based display to bond-based display with new columns and functionality
  - **Priority**: P1
  - **Estimated Effort**: M

- [ ] **TODO #32**: Remove fund-related admin components
  - **File**: `frontend-app/src/adminSection/components/` (multiple files)
  - **Components**: Fund management related components
  - **Change Details**: Remove components that depend on security-mint-fund functionality
  - **Priority**: P1
  - **Estimated Effort**: S

- [ ] **TODO #33**: Update YieldsTab component for new bond yield system
  - **File**: `frontend-app/src/adminSection/components/YieldsTab.tsx`
  - **Function**: Yield display and management logic
  - **Change Details**: Update to work with new bond yield system instead of fund-based yields
  - **Priority**: P0
  - **Estimated Effort**: L

### API Integration Changes

- [ ] **TODO #34**: Update API client for bond endpoints
  - **Generated Client**: `frontend-app/src/apiClient/` (auto-generated)
  - **Change Details**: Regenerate API client after backend bond endpoints are implemented
  - **Priority**: P0
  - **Estimated Effort**: S (auto-generated)

- [ ] **TODO #35**: Create bond management hooks
  - **File**: `frontend-app/src/hooks/useBondManagement.ts` (new)
  - **Hook Type**: Custom React Hook
  - **Change Details**: Create hooks for bond creation, yield configuration, trading controls, and maturity management
  - **Priority**: P0
  - **Estimated Effort**: M

## ☁️ Infrastructure Changes

### AWS CDK Changes

- [ ] **TODO #36**: Update database configuration for new bond tables
  - **File**: `cdk-deployment/lib/database-stack.ts`
  - **Resource Type**: RDS Database Configuration
  - **Change Details**: Ensure database has sufficient capacity and configuration for new bond-related tables
  - **Priority**: P1
  - **Estimated Effort**: S

- [ ] **TODO #37**: Add IPFS integration for bond metadata storage
  - **File**: `cdk-deployment/lib/storage-stack.ts`
  - **Resource Type**: S3 + IPFS Gateway
  - **Change Details**: Add infrastructure for storing ISIN documents and environmental impact reports on IPFS
  - **Priority**: P1
  - **Estimated Effort**: M

- [ ] **TODO #38**: Configure automated yield distribution cloud functions
  - **File**: `cdk-deployment/lib/lambda-stack.ts`
  - **Resource Type**: Lambda Functions + CloudWatch Events
  - **Change Details**: Add Lambda functions for automated yield distribution and maturity processing with scheduled triggers
  - **Priority**: P0
  - **Estimated Effort**: L

### Environment Configuration

- [ ] **TODO #39**: Add bond-specific environment variables
  - **File**: `cdk-deployment/env/` (environment configs)
  - **Change Details**: Add environment variables for yield calculation parameters, IPFS endpoints, automated distribution settings
  - **Priority**: P0
  - **Estimated Effort**: S

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
- **M5 Requirements**: ✅ FR-BT-1 through FR-BT-5 documented and analyzed
- **Implementation Todos**: ✅ 39 detailed todos created across all system layers
- **Code Analysis**: ✅ Current implementation examined and change points identified
- **Impact Assessment**: ✅ Complete system impact analyzed (contracts, backend, frontend, infrastructure)

### Implementation Summary
- **Total Todos**: 39 implementation tasks
- **P0 (Critical)**: 22 tasks - Core bond functionality
- **P1 (Important)**: 8 tasks - Fund system removal and cleanup
- **P2 (Nice to have)**: 1 task - Optional dedicated bond contract
- **Smart Contract Changes**: 9 todos
- **Backend Changes**: 15 todos
- **Frontend Changes**: 11 todos
- **Infrastructure Changes**: 4 todos

### Dependencies
1. **Contract Enhancement** must be completed before backend integration
2. **Backend API Development** must be completed before frontend integration
3. **Database Schema Changes** must be implemented before backend services
4. **Fund System Removal** can be done in parallel with new bond system development

### Risks and Mitigation
- **Risk 1**: Complex yield calculation logic - Mitigate with thorough testing and formula validation
- **Risk 2**: Bond state management complexity - Mitigate with state machine pattern and comprehensive event logging
- **Risk 3**: Automated distribution failures - Mitigate with retry mechanisms and manual override capabilities
- **Risk 4**: Migration from fund-based system - Mitigate with parallel development and feature flags

### Next Steps for Implementation
1. **Start with Database Schema** (TODOs #18-#24) - Foundation for all other changes
2. **Begin Contract Enhancement** (TODOs #1-#7) - Core bond functionality
3. **Develop Backend APIs** (TODOs #10-#17) - Integration layer
4. **Build Frontend Components** (TODOs #25-#35) - User interface
5. **Setup Infrastructure** (TODOs #36-#39) - Automated processes
6. **Remove Legacy Fund System** (TODOs #8, #10, #17, #23, #32) - Cleanup

### Last Updated
- **Date**: 2025-09-01
- **Updated By**: WARP AI Assistant
- **Changes Made**: Comprehensive M5 bond system planning with 39 detailed implementation todos

---

**Status**: ✅ M5 Bond Planning Complete - Ready for Implementation

**Ready for Development**: You can now point to any specific todo (e.g., "Implement TODO #1 in m5-planning/bonds.md") to start targeted development with full context and implementation details.
