# Admin Workflow for Bonds

This document lists admin-facing workflows as discrete tasks. Each section states exactly how to perform the task and which interface (on-chain or API) is used. More tasks will be added later.

## 1) Bond Creation

Steps:

### 1.1 Create Forest Project (API)

Admin creates a forest project using the `POST /forest-projects` endpoint with parameters defined in forest-project-api.md:

- Project name, description, location
- Area in hectares and estimated carbon credits
- Project start and end dates
- Optional certification body

Purpose: Register project in backend and UI; does not create on-chain contracts.

### 1.2 Create Pre-sale Token Contract (on-chain)

Admin deploys security-sft-single contract:

- Initialize with token metadata (name, symbol, description)
- Token ID 0 is created automatically on init
- Contract will hold pre-sale tokens during subscription period

### 1.3 Create Post-sale Token Contract (on-chain)

Admin deploys security-sft-multi contract:

- Initialize with token metadata
- Must support `addToken` function for time-based token IDs
- Will hold post-sale tokens after subscription period ends

### 1.4 Create Bond (on-chain)

Admin calls `add_bond` function on bonds contract as defined in bond-contract.md:

- Function: `add_bond`
- Agent role: AddFund
- Parameters: Bond terms, supply limits, pricing, token contracts
- Storage key: `postsale_token_contract_address` → Bond
- Events processor will automatically populate bond data in database from `BondAdded` event

### 1.5 Link Bond to Forest Project (API)

Admin links bond to project using `PUT /forest-projects/{project_id}/bonds` with:

- `postsale_token_contract_address` from step 1.4

Purpose: Create off-chain mapping forest_project_id ↔ postsale_token_contract_address

Notes:

- Post-sale token IDs will use subscription_period_end as the token_id for minted post-sale tokens.
- security-sft-multi must support initialization with TokenMetadata (see bonds-planning.md).

## 2) Setting the Bond Status

Admin calls `update_bond_status` function on bonds contract as defined in bond-contract.md:

- Function: `update_bond_status`
- Agent role: UpdateFundState
- Parameters: postsale_token_contract_address, new_status
- Allowed statuses: Active, Paused, Matured, Success, Failed

Effects:

- Emits `BondStatusUpdated` event
- Events listener updates database; UI reflects new status

## 3) Starting Bond Claim / Refund Batch Jobs

### Entry points (UI)

- **Claim All** appears when bond status = Success
- **Refund All** appears when bond status = Failed

### Backend API Calls

Admin initiates batch operations using bond API endpoints defined in bond-api.md:

- For successful bonds: `POST /bonds/{postsale_token_contract_address}/claim` with list of account_addresses
- For failed bonds: `POST /bonds/{postsale_token_contract_address}/refund` with list of account_addresses

### Backend Processing

- Build batch list of account addresses that hold pre-sale token balances
- Call appropriate on-chain batch function as defined in bond-contract.md:
  - Success: `claim` function (Agent role: Operator)
  - Failed: `refund` function (Agent role: Operator)
- Control batch size to avoid transaction failures; retry remaining addresses as needed
- Events listener records Claim/Refund events; database and UI update accordingly

## 4) Creating / Archiving Forest Project

Admin uses forest project API endpoints defined in forest-project-api.md:

### Create Project

- Endpoint: `POST /forest-projects`
- Input: project details (name, description, location, area, carbon credits, dates, certification)
- Purpose: Register project for UI and off-chain mapping

### Archive Project  

- Endpoint: `PUT /forest-projects/{project_id}/archive`
- Effect: Remove from Active Projects lists; does not alter on-chain contracts

## 5) Setting Bond Interest Rates

- On-chain parameters:
  - interest_rate_type is set at bond creation via add_bond
- Off-chain configuration:
  - Specific rate schedules and calculations remain off-chain
  - Admin updates rate metadata in backend (API endpoint to be defined), used for display and off-chain calculations
- Future on-chain updates:
  - If needed, an update_bond_params function can be added; currently not required per bonds-planning.md
