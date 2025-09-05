# Admin Workflow for Bonds

This document lists admin-facing workflows as discrete tasks. Each section states exactly how to perform the task and which interface (on-chain or API) is used. More tasks will be added later.

## 1) Bond Creation

Steps:

### 1.1 Create Bond Metadata (API)

Admin creates comprehensive bond metadata using the `POST /admin/bonds/metadata` endpoint with parameters defined in bonds-backend.md:

- Bond name, location, display picture
- Market analysis (timber market trends, carbon credit market)
- Competitive analysis (market position, growth potential, risk level)
- Project financials (investment goal, token price, revenue streams)
- Prospectus documents (project prospectus, investment memorandum, financial projections, risk assessment)
- Geo-spatial info (coordinates, total area)
- Forest composition and environmental impact data

Purpose: Register comprehensive bond information in backend for investor UI; does not create on-chain contracts.

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

### 1.5 Link Contracts to Bond Metadata (API)

Admin links blockchain contracts to bond metadata using `PUT /admin/bonds/metadata/{bond_metadata_id}/contracts` with:

- `presale_token_contract` from step 1.2
- `postsale_token_contract` from step 1.4

Purpose: Connect on-chain contracts to off-chain metadata for comprehensive bond information

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

Admin initiates batch operations using bond API endpoints defined in bond-backend.md:

- For successful bonds: `POST /admin/bonds/{bond_metadata_id}/claim` with list of account_addresses
- For failed bonds: `POST /admin/bonds/{bond_metadata_id}/refund` with list of account_addresses

### Backend Processing

- Build batch list of account addresses that hold pre-sale token balances
- Call appropriate on-chain batch function as defined in bond-contract.md:
  - Success: `claim` function (Agent role: Operator)
  - Failed: `refund` function (Agent role: Operator)
- Control batch size to avoid transaction failures; retry remaining addresses as needed
- Events listener records Claim/Refund events; database and UI update accordingly

## 4) Managing Bond Metadata

Admin uses bond metadata API endpoints defined in bonds-backend.md:

### Update Bond Metadata

- Endpoint: `PUT /admin/bonds/metadata/{bond_metadata_id}`
- Input: updated bond information (market analysis, financial projections, documents)
- Purpose: Modify bond information displayed to investors

### Upload Documents (SIMPLIFIED APPROACH)

**Steps for all document types:**

1. **Upload Any Document**: `POST /admin/documents/upload`
   - Input: document_type ("prospectus", "memorandum", "projection", "assessment", "subscription_agreement", "bond_image"), file_extension
   - Output: presigned S3 URL for direct file upload
   - Files stored in S3 with unique names

2. **Include in Bond Metadata**: Use document URLs in bond creation/update
   - **Prospectus documents**: Include URLs in `prospectus_documents` field
   - **Subscription agreements**: Include URL and details in `subscription_agreement` field
   - **Bond images**: Include URL in `bond_display_picture_url` field

### Manage Subscription Agreements (FR-DS-1)

**NEW FEATURE**: Admin can include subscription agreement directly in bond metadata

**Workflow:**

1. **Upload Agreement PDF**: `POST /admin/documents/upload` with `document_type`: "subscription_agreement"
2. **Create/Update Bond**: Include subscription agreement details in bond metadata:
   ```json
   {
     "subscription_agreement": {
       "agreement_name": "Baltic Pine Forest Investment Agreement",
       "document_url": "https://s3.bucket/documents/agreement_123.pdf",
       "document_hash": "a665a459...",
       "file_size_bytes": 1048576
     }
   }
   ```

**Benefits:**
- Single workflow for all documents
- Subscription agreement managed as part of bond metadata
- No separate agreement versioning complexity
- Simpler API surface

### Generate Token Metadata

- Endpoint: `POST /admin/bonds/metadata/{bond_metadata_id}/generate-metadata-json`
- Purpose: Generate CIS-2 compliant token metadata JSON for blockchain contracts
- Effect: Creates public S3 URL for use in token contracts

## 5) Yield Configuration and Distribution (FR-BT-2)

### 5.1 Configure Bond Yield Parameters

After bond subscription period ends with >= minimum amount raised, admin configures yield parameters:

**Prerequisites:**

- Bond status = Success
- Subscription period has ended
- Minimum funding threshold was met

**Steps:**

- Admin uses yield configuration API endpoints defined in yields-backend.md
- Configure yield type (fixed or variable), PLT token address, payment timing and rates
- Can update configuration parameters before first distribution if needed

### 5.2 Trigger Yield Distribution

When yield time arrives, admin triggers distribution:

**Prerequisites:**

- Yield configuration exists for bond
- Current time >= configured yield_time
- Bond tokens have been minted to investors

**Steps:**

- Admin triggers distribution using yield API endpoints defined in yields-backend.md
- System automatically snapshots token holders and calculates individual amounts
- Cloud wallet processes PLT token transfers in batches to eligible investors
- Admin can monitor progress and review completed distributions

**Notes:**

- Detailed API endpoints, calculation formulas, and database schema defined in yields-backend.md
- Investors receive PLT tokens automatically with no action required
- Failed payments (blacklisted/KYC issues) are logged and can be retried

## 5.3 Monitor Subscription Agreement Signatures (FR-DS-1)

**NEW FEATURE**: Admin can monitor which investors have signed subscription agreements

**View Agreement Signatures:**

- **List Bond Agreements**: `GET /admin/bonds/{bond_metadata_id}/subscription-agreements`
  - Shows signature_count per agreement
  - Displays agreement versions and status

- **View Signature Details**: Admin can access detailed signature information
  - Which investors have signed vs. haven't signed
  - Signature timestamps and verification status
  - Audit trail for compliance reporting

**Investment Flow Impact:**

- When bonds require subscription agreements, the investment flow automatically validates signatures
- Investment API returns `SUBSCRIPTION_AGREEMENT_NOT_SIGNED` error for unsigned investors
- Investors are redirected to signing flow before investment can proceed
- Admin can track conversion rates from signature to actual investment

## 6) Investor Blacklist Management (FR-BT-3)

### Blacklist Wallet

Admin can blacklist investor wallets to restrict yield/maturity payments using API endpoints defined in investor-blacklist-api.md:

- Endpoint: `POST /admin/investors/blacklist`
- Input: wallet_address, reason (optional)
- Effect: Wallet cannot receive yield or maturity payments

### Whitelist Wallet (Remove from Blacklist)

Admin can restore wallet to whitelist status:

- Endpoint: `POST /admin/investors/whitelist`  
- Input: wallet_address, reason (optional)
- Effect: Restores wallet access to yield/maturity payments

### View Blacklist Status

- List all blacklisted wallets: `GET /admin/investors/blacklist`
- Check specific wallet: `GET /admin/investors/{wallet_address}/status`
- Shows KYC status (from third party) and blacklist status

Notes:

- All wallets are whitelisted by default after KYC verification
- Blacklist overrides KYC whitelist status
- Audit trail maintained for all blacklist operations

## 7) Trading Fee Configuration (FR-BT-3)

### Configure Trading Fees

Admin configures secondary market trading fees on security-sft-multi contracts:

- Function: `set_trading_fee_config` (on-chain)
- Parameters: treasury_account, default_fee_rate, enabled
- Effect: Sets global fee configuration

### Set Contract-Specific Fees

Admin can set different fee rates for different trading platforms:

- Function: `set_contract_trading_fee` (on-chain)
- Parameters: contract_address (DEX/platform), fee_rate
- Effect: Platform-specific fees when users trade through that contract

### Fee Collection

- Fees automatically deducted from transfers initiated by smart contracts
- Direct wallet-to-wallet transfers have no fees
- All collected fees sent to configured treasury account

## 8) Bond Maturity Handling (FR-BT-5)

### 8.1 Trigger Maturity Payments

When a bond reaches maturity, admin can trigger maturity payments to return face value to investors:

**Prerequisites:**

- Bond has reached its maturity_date
- Investors have been verified and whitelisted via identity registry
- Cloud wallet has sufficient PLT balance for all payments

**Steps:**

1. Admin uses bond maturity API endpoints defined in bond-blockchain.md
2. Trigger maturity payment: `POST /admin/bonds/{bond_contract_address}/maturity/trigger`
3. System performs two-phase transaction:
   - **Phase 1**: Burns all bond tokens held by whitelisted investors
   - **Phase 2**: Transfers PLT tokens equivalent to face value
4. Admin monitors progress: `GET /admin/bonds/maturity/{maturity_job_id}/status`

**Liquidity Management:**

- System checks cloud wallet PLT balance before initiating
- All-or-none payment policy: sufficient liquidity required for all payments
- If insufficient liquidity, job status set to "insufficient_liquidity"
- Admin must ensure adequate PLT balance before triggering

### 8.2 Whitelist Management for Maturity Payments

Only whitelisted investors are eligible for maturity payments:

**Whitelist Investor:**

- Admin uses identity registry API: `POST /admin/identity/addresses/{address}/whitelist`
- Effect: Investor becomes eligible for maturity payments
- Investors remain on whitelist until explicitly blacklisted

**Blacklist Investor:**

- Admin uses identity registry API: `POST /admin/identity/addresses/{address}/blacklist`
- Effect: Investor excluded from maturity payments (tokens remain but no PLT transferred)
- Used for compliance or regulatory reasons

**Check Investor Status:**

- Admin can check status: `GET /admin/identity/addresses/{address}/status`
- Returns: registered, whitelisted, or blacklisted

### 8.3 Monitor Maturity Payment Progress

Admin can track maturity payment execution:

**View Job Status:**

- Monitor overall progress: `GET /admin/bonds/maturity/{maturity_job_id}/status`
- Shows: total recipients, tokens burned, PLT transferred, transaction details
- Job statuses: pending → burning_tokens → transferring_plt → completed

**Review Payment History:**

- View historical payments: `GET /admin/bonds/{bond_contract_address}/maturity/history`
- Shows all past maturity payment jobs with detailed audit trail
- Includes transaction hashes for both burn and PLT transfer phases

**Error Handling:**

- Failed transactions retry automatically with exponential backoff
- Admin can manually retry failed payments if needed
- Comprehensive error logging for troubleshooting

**Notes:**

- Detailed API endpoints, database schema, and integration flows defined in bond-blockchain.md
- Maturity payments are final - no reversals once completed
- Complete audit trail maintained for all maturity operations
