# Investor Workflow for Bonds

This document describes the investor-facing workflow for bond investments.

## 1. Login and Dashboard

Investor accesses dashboard showing:

- **Active Projects**: bonds open for investment
- **My Assets**: total portfolio and list of invested bonds

Dashboard data retrieved via:

- Active projects: `GET /bonds?status=Active` (endpoint details in bond-api.md)
- Portfolio data: derived from bond data filtered by investor account

## 2. Select Bond and Investment Amount

Investor workflow:

- Select a bond and click **"Invest Now"**
- Enter EUR amount to invest
- Frontend validates:
  - Amount meets minimum investment requirement
  - Investment doesn't exceed bond maximum supply
  - Subscription period is still active

## 3. Payment Processing

Investor performs off-chain payment:

- Transfer PLT tokens to pre-configured wallet
- Obtain payment proof (signed message) with:
  - `reward_id`: payment transaction hash
  - `nonce`: investor-specific nonce for replay protection
  - `signer_account_address`: authorized signer
  - `signature`: signature over canonical message format

Payment proof structure defined in bond-contract.md.

## 4. On-chain Investment

Investor calls `invest` function on bonds contract:

- Parameters: `postsale_token_contract_address`, `amount`, `payment_proof`
- Contract performs verification and minting as defined in bond-contract.md
- Emits `BondInvestment` event with `reward_id` for correlation

## 5. Database and UI Updates

Events processor handles `BondInvestment` event:

- Updates investor balances in database tables (see bond-db.md)
- Records investment with `reward_id` correlation
- UI automatically reflects updated portfolio balances
- Investor sees updated data in **My Assets** section

## 6. Yield and Maturity Payments

### Yield Distributions (FR-BT-4)

Investors receive automatic yield payments when admin triggers distribution:

- **No action required** - PLT tokens sent directly to investor wallet
- Payments based on actual holding period per token ID
- Only eligible if wallet passes KYC verification and is not blacklisted
- View payment history in **My Assets** section

### Maturity Payments (FR-BT-5)

At bond maturity, investors receive face value payment:

- **No action required** - automated two-phase process
- Phase 1: Bond tokens automatically burned from investor wallet
- Phase 2: PLT tokens equivalent to face value transferred to wallet
- **Whitelist requirement**: Must be whitelisted to receive payment

## 7. Questions and Unknowns

### Whitelist Process (FR-BT-5)

**DOUBT**: How do investors get whitelisted for maturity payments?

- Is whitelist status automatic after KYC verification?
- Do investors need to request whitelist approval separately?
- What triggers admin to whitelist an investor?
- Is there a whitelist application process in the UI?
- Are there additional compliance requirements beyond KYC?

**Current Implementation Gap:**
- Admin workflow includes whitelist management endpoints
- Investor workflow unclear on how to achieve whitelist status
- May require additional UI flow or automatic process definition

### KYC Integration Status

**DOUBT**: Integration with third-party KYC service

- Which KYC provider will be used?
- How does investor complete KYC verification?
- Is KYC verification handled in-app or external redirect?
- What investor data is required for KYC?
- How are KYC status changes communicated to investor?

### Payment Notifications

**DOUBT**: How are investors notified of payments?

- Are email/push notifications sent for yield payments?
- How does investor know maturity payment was processed?
- Is there a payment history/statement feature?
- Are failed payments (due to blacklist/KYC) communicated?
