# M5 Planning Overview - Concordium RWA Platform

## 📊 Current Codebase Analysis

### Smart Contracts Architecture
The platform currently includes the following smart contracts:

#### Core Asset Management Contracts
- **`security-sft-multi`**: Multi-token security (forest project) contract - main asset tokenization
- **`security-sft-single`**: Single token security contract - carbon credits and individual assets
- **`security-mint-fund`**: Investment fund and bond management contract
- **`security-sft-multi-yielder`**: Yield distribution and reward management for multi-token securities

#### Trading and Compliance Contracts  
- **`security-p2p-trading`**: Peer-to-peer trading marketplace
- **`identity-registry`**: User access control and compliance management
- **`compliance`**: Advanced compliance rules and regulations
- **`nft-multi-rewarded`**: NFT with reward distribution capabilities
- **`offchain-rewards`**: Off-chain reward calculation and distribution

#### Supporting Infrastructure
- **`concordium-protocols`**: Protocol definitions and utilities
- **`integration-tests`**: Cross-contract integration testing

### Backend Services Architecture
The backend consists of event-driven services built with Rust and the Poem framework:

#### Event Processing Layer
- **`events_listener`**: Main blockchain event processor
  - **Processors**: Contract-specific event handlers for each smart contract
  - **Listener**: Core blockchain monitoring and event distribution

#### API Services Layer  
- **`app_api`**: REST API service for frontend applications
- **Database Layer**: PostgreSQL with Diesel ORM
  - **Contract DB Models**: Mirror blockchain state in database
  - **Application DB Models**: Business logic data (portfolios, projects, etc.)

### Frontend Application Architecture
React 18 + TypeScript + Vite application with:

#### User Interface
- **Main App**: User-facing forest project investment interface
- **Admin Section**: Comprehensive admin panel for platform management

#### Key Admin Components
- **Token Management**: Token creation, metadata, pause/unpause controls
- **User Management**: Investor tables, agent management, balance controls
- **Trading Management**: P2P trading oversight, market monitoring
- **Yield Management**: Distribution tracking, yield calculations

#### Infrastructure Integration
- **Wallet Integration**: Concordium browser wallet support
- **API Client**: Auto-generated from backend OpenAPI specs
- **State Management**: React hooks and context for blockchain state

## 📋 M5 Requirement Areas Identified

Based on the codebase analysis, the following requirement chunk areas are anticipated:

### 🔗 Primary Requirement Chunks
1. **`bonds.md`** - Investment fund and bond management enhancements
2. **`yield.md`** - Yield distribution and reward calculation improvements  
3. **`trading.md`** - P2P trading platform enhancements
4. **`compliance.md`** - Identity registry and regulatory compliance updates
5. **`frontend.md`** - User interface and admin panel improvements
6. **`infrastructure.md`** - Deployment and AWS infrastructure changes

### 🔄 Cross-Cutting Concerns
- **Database Schema Evolution**: New tables/columns for enhanced features
- **API Client Regeneration**: Updates required after backend changes
- **Event Processing Updates**: New blockchain events to capture
- **Testing Coverage**: Unit, integration, and E2E tests for new features

## 🎯 Planning Workflow Status

### Completed Setup Tasks
- [x] Planning directory structure created
- [x] README.md with workflow documentation  
- [x] Template for requirement chunks (`_template_requirement.md`)
- [x] Codebase analysis and architecture overview
- [x] Overview document with current state analysis

### Next Steps
- [ ] Review business requirement document with stakeholder
- [ ] Create specific requirement chunk files (bonds.md, yield.md, etc.)
- [ ] Analyze each requirement against current implementation
- [ ] Document specific code changes needed in todo format
- [ ] Prioritize and estimate effort for each change
- [ ] Begin implementation phase using documented todos

## 📁 File Navigation Quick Reference

### Smart Contract Files
- **Main Contracts**: `contracts/[contract-name]/src/lib.rs`
- **Contract Types**: `contracts/[contract-name]/src/types.rs` (if exists)
- **Integration Tests**: `contracts/integration-tests/src/[contract-name].rs`

### Backend Files
- **Event Processors**: `backend/events_listener/src/processors/[contract-name].rs`
- **API Handlers**: `backend/app_api/src/handlers/[domain].rs`
- **Database Models**: `backend/shared/src/db/[contract-name].rs`
- **Database Migrations**: `backend/shared/migrations/[timestamp]_[description].sql`

### Frontend Files
- **Main Components**: `frontend-app/src/[section]/[ComponentName].tsx`
- **Admin Components**: `frontend-app/src/adminSection/components/[ComponentName].tsx`
- **API Client**: `frontend-app/src/apiClient/[generated-files]`
- **Types**: `frontend-app/src/types/[domain].d.ts`

### Infrastructure Files
- **CDK Stacks**: `cdk-deployment/lib/[stack-name].ts`
- **Environment Configs**: `cdk-deployment/env/[environment].ts`

## 🚀 Ready for Requirements Discussion

The planning system is now set up and ready for detailed requirement analysis. 

**To proceed:**
1. Share your business requirement document sections
2. We'll create specific requirement chunk files using the template
3. Each chunk will contain detailed implementation todos
4. You'll be able to point to specific todos for targeted development

The system is designed to maintain full traceability from business requirements to specific code changes, making implementation efficient and well-documented.

---

**Last Updated**: 2025-09-01  
**Status**: Setup Complete - Ready for Requirement Analysis
