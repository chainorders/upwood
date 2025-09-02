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
