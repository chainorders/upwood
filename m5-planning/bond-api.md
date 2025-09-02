# Bonds Planning (API)

This document defines the bonds API planning (backend/upwood/src/api).

## File Structure

- New file: backend/upwood/src/api/bonds.rs

## Endpoints (initial)

### GET /bonds

List bonds (filters: project, status)

Query parameters:

- project (string, optional)
- status (string, optional: Active, Paused, Matured, Success, Failed)
- limit (number, optional)
- offset (number, optional)

Output:

- bonds (array of bond summaries)
- total (number)

### GET /bonds/{postsale_token_contract_address}

Get bond details

Output:

- bond (bond details object)
- investors (array of investor summaries, optional)

### POST /bonds/{postsale_token_contract_address}/claim (Operator)

Start batch claim job

Input:

- account_addresses (array of strings)

Output:

- job_id (string)
- status (string: pending, processing, completed, failed)

### POST /bonds/{postsale_token_contract_address}/refund (Operator)

Start batch refund job

Input:

- account_addresses (array of strings)

Output:

- job_id (string)
- status (string: pending, processing, completed, failed)

Notes:

- Bonds are created entirely on-chain via smart contract `add_bond` function
- Bond data is populated in database by events processor from on-chain events
- Investors invest directly on-chain with payment proof
- Admin updates status directly on-chain

## Security

- Admin / Operator auth via Cognito
