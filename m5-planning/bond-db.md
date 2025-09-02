# Bonds Planning (Database)

This document defines database planning for bonds-related data.

## Tables

### bonds

- postsale_token_contract_address (string, PK)
- maturity_date (timestamp)
- interest_rate_type (string)
- maximum_supply (decimal)
- minimum_raise_amount (decimal)
- lockup_period_duration (interval)
- subscription_period_end (timestamp)
- bond_price (decimal)
- presale_token_contract_address (string)
- current_supply (decimal)
- status (string: Active, Paused, Matured, Success, Failed)
- created_at (timestamp)
- updated_at (timestamp)

Indexes:

- idx_bonds_status
- idx_bonds_subscription_period_end

### bond_investors

- id (string, PK)
- bond_id (FK → bonds.postsale_token_contract_address)
- account_address (string)
- total_invested (decimal)
- presale_balance (decimal)
- postsale_balance (decimal)
- created_at (timestamp)
- updated_at (timestamp)
- UNIQUE(bond_id, account_address)

Indexes:

- idx_bond_investors_account

### bond_investment_records

- id (string, PK)
- bond_id (FK → bonds.postsale_token_contract_address)
- investor_id (FK → bond_investors.id)
- reward_id (string)  # correlates off-chain payment and on-chain invest
- record_type (string: invest, divest, claim, refund)
- amount (decimal)
- token_type (string: presale, postsale)
- token_id (string nullable)
- transaction_hash (string)
- processed_at (timestamp)

Indexes:

- idx_bond_investment_records_reward_id
- idx_bond_investment_records_bond
