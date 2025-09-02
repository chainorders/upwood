# Bonds Planning (Processor / Indexer)

This document defines the bonds events processor (backend/events_listener) planning.

## Module

- File: backend/events_listener/src/processors/bonds.rs
- Register in processors::mod.rs with module_ref and contract_name

## Event Processing

### BONDS: BondAdded
Upon receiving this event, following database entries will be added:
- Insert new row into `bonds` table using postsale_token_contract_address as PK as defined in bond-db.md
- Populate bond parameters from event data
- Set initial status and created_at timestamp

### BONDS: BondRemoved
Upon receiving this event, following database entries will be updated:
- Update `bonds` table status to archived/deleted as defined in bond-db.md
- Preserve bond row for historical data

### BONDS: BondInvestment
Upon receiving this event, following database entries will be added/updated:
- Upsert row in `bond_investors` table as defined in bond-db.md (total_invested, presale_balance, postsale_balance)
- Insert new row in `bond_investment_records` table with reward_id correlation
- Handle presale/postsale token type distinction

### BONDS: BondStatusUpdated
Upon receiving this event, following database entries will be updated:
- Update `bonds` table status field as defined in bond-db.md
- Update updated_at timestamp

### BONDS: BondClaimed
Upon receiving this event, following database entries will be updated/added:
- Update `bond_investors` table: decrease presale_balance, increase postsale_balance as defined in bond-db.md
- Insert new row in `bond_investment_records` table with record_type='claim'

### BONDS: BondRefunded
Upon receiving this event, following database entries will be updated/added:
- Update `bond_investors` table: decrease presale_balance only as defined in bond-db.md
- Insert new row in `bond_investment_records` table with record_type='refund'

## Processing Notes

- Batch events (Claimed/Refunded) correspond to multiple database operations
- Ensure idempotency using transaction hash
- Process events in single database transaction per block
- Reference table structures and constraints from bond-db.md
