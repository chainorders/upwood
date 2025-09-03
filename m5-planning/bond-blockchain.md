# Bond Blockchain Component

## File Locations

### Smart Contract

- Contract: `contracts/security-mint-fund/src/lib.rs`
- State: `contracts/security-mint-fund/src/state.rs`
- Types: `contracts/security-mint-fund/src/types.rs`

### Event Processor

- Processor: `backend/events_listener/src/processors/bonds.rs`

### Database Layer (Processor-Managed)

- Database Models: `backend/shared/src/db/bonds.rs`
- Schema: `backend/shared/src/schema.rs`

---

# PART I: SMART CONTRACT

## Access Control and Agent Roles

Agent roles match contracts/security-mint-fund/src/types.rs (AgentRole):

- AddFund → required for add_bond
- RemoveFund → required for remove_bond
- UpdateFundState → required for update_bond_status
- Operator → required for claim and refund

Public access:

- invest → callable by any investor (subject to identity registry policies and payment-proof verification)

## State Structure

```rust path=null start=null
struct State {
    bonds: StateMap<ContractAddress, Bond>,
    investor_nonces: StateMap<AccountAddress, u64>,
}

struct Bond {
    maturity_date: Timestamp,
    interest_rate_type: InterestRateType,
    maximum_supply: Amount,
    minimum_raise_amount: Amount,
    lockup_period_duration: Duration,
    subscription_period_end: Timestamp,
    bond_price: Amount,
    presale_token_contract_address: ContractAddress,
    postsale_token_contract_address: ContractAddress,
    current_supply: Amount,
    status: BondStatus,
}
```

## Core Functions

### add_bond

**Agent Role:** AddFund

**Input Parameters:**

```rust path=null start=null
struct AddBondParams {
    maturity_date: Timestamp,
    interest_rate_type: InterestRateType,
    maximum_supply: Amount,
    minimum_raise_amount: Amount,
    lockup_period_duration: Duration,
    subscription_period_end: Timestamp,
    bond_price: Amount,
    presale_token_contract_address: ContractAddress,
    postsale_token_contract_address: ContractAddress,
}
```

### remove_bond

**Agent Role:** RemoveFund

**Input Parameters:**

```rust path=null start=null
struct RemoveBondParams {
    postsale_token_contract_address: ContractAddress,
}
```

### invest

**Agent Role:** Public (any investor with valid payment proof)

**Input Parameters:**

```rust path=null start=null
struct InvestParams {
    postsale_token_contract_address: ContractAddress,
    amount: Amount,
    payment_proof: PaymentProof,
}
```

### claim (batch)

**Agent Role:** Operator

**Input Parameters:**

```rust path=null start=null
struct ClaimParams {
    postsale_token_contract_address: ContractAddress,
    account_addresses: Vec<AccountAddress>,
}
```

### refund (batch)

**Agent Role:** Operator

**Input Parameters:**

```rust path=null start=null
struct RefundParams {
    postsale_token_contract_address: ContractAddress,
    account_addresses: Vec<AccountAddress>,
}
```

### update_bond_status

**Agent Role:** UpdateFundState

**Input Parameters:**

```rust path=null start=null
struct UpdateBondStatusParams {
    postsale_token_contract_address: ContractAddress,
    new_status: BondStatus,
}
```

## Events

```rust path=null start=null
struct BondAddedEvent {
    bond_address: ContractAddress,
    maturity_date: Timestamp,
    subscription_period_end: Timestamp,
}

struct BondRemovedEvent {
    bond_address: ContractAddress,
}

struct BondInvestmentEvent {
    bond_address: ContractAddress,
    investor: AccountAddress,
    amount: Amount,
    reward_id: Vec<u8>,
    token_type: String, // "presale" or "postsale"
}

struct BondClaimedEvent {
    bond_address: ContractAddress,
    investor: AccountAddress,
    amount: Amount,
}

struct BondRefundedEvent {
    bond_address: ContractAddress,
    investor: AccountAddress,
    amount: Amount,
}

struct BondStatusUpdatedEvent {
    bond_address: ContractAddress,
    old_status: BondStatus,
    new_status: BondStatus,
}
```

## Payment Proof Verification

```rust path=null start=null
struct PaymentProof {
    reward_id: Vec<u8>,
    nonce: u64,
    signer: AccountAddress,
    signature: Signature,
}

fn expected_message(investor, bond, amount, reward_id, nonce) -> Vec<u8>
fn verify_proof(proof: &PaymentProof, message: &[u8]) -> ContractResult<bool>
```

---

# PART II: EVENT PROCESSOR

```rust path=null start=null
pub fn process_events(
    conn: &mut DbConn,
    _block_height: Decimal,
    block_time: NaiveDateTime,
    _txn_index: Decimal,
    _txn_sender: &str,
    _txn_instigator: &str,
    contract: &ContractAddress,
    events: &[ContractEvent],
) -> Result<(), ProcessorError> {
    for event in events {
        let parsed_event = event.parse::<Event>().expect("Failed to parse event");
        
        match parsed_event {
            Event::BondAdded(e) => {
                let new_bond = Bond::new(
                    e.bond_address,
                    e.maturity_date,
                    e.subscription_period_end,
                    block_time
                );
                Bond::insert(new_bond, conn)?;
            }
            Event::BondRemoved(e) => {
                Bond::update_status(
                    e.bond_address.to_decimal(),
                    "Archived".to_string(),
                    block_time,
                    conn
                )?;
            }
            Event::BondInvestment(e) => {
                BondInvestor::upsert_investment(
                    e.bond_address.to_decimal(),
                    e.investor,
                    e.amount,
                    &e.token_type,
                    conn
                )?;
                
                BondInvestmentRecord::insert_investment(
                    e.bond_address.to_decimal(),
                    e.investor,
                    e.amount,
                    &e.token_type,
                    hex::encode(&e.reward_id),
                    block_time,
                    conn
                )?;
            }
            Event::BondStatusUpdated(e) => {
                Bond::update_status(
                    e.bond_address.to_decimal(),
                    e.new_status.to_string(),
                    block_time,
                    conn
                )?;
            }
            Event::BondClaimed(e) => {
                BondInvestor::process_claim(
                    e.bond_address.to_decimal(),
                    e.investor,
                    e.amount,
                    conn
                )?;
                
                BondInvestmentRecord::insert_claim(
                    e.bond_address.to_decimal(),
                    e.investor,
                    e.amount,
                    block_time,
                    conn
                )?;
            }
            Event::BondRefunded(e) => {
                BondInvestor::process_refund(
                    e.bond_address.to_decimal(),
                    e.investor,
                    e.amount,
                    conn
                )?;
                
                BondInvestmentRecord::insert_refund(
                    e.bond_address.to_decimal(),
                    e.investor,
                    e.amount,
                    block_time,
                    conn
                )?;
            }
        }
    }

    Ok(())
}
```

---

# PART III: DATABASE SCHEMA

### bonds

```sql path=null start=null
CREATE TABLE bonds (
    postsale_token_contract_address TEXT PRIMARY KEY,
    maturity_date TIMESTAMP WITH TIME ZONE NOT NULL,
    interest_rate_type TEXT NOT NULL,
    maximum_supply DECIMAL(78, 0) NOT NULL,
    minimum_raise_amount DECIMAL(78, 0) NOT NULL,
    lockup_period_duration INTERVAL NOT NULL,
    subscription_period_end TIMESTAMP WITH TIME ZONE NOT NULL,
    bond_price DECIMAL(78, 0) NOT NULL,
    presale_token_contract_address TEXT NOT NULL,
    current_supply DECIMAL(78, 0) NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Active' CHECK (status IN ('Active', 'Paused', 'Matured', 'Success', 'Failed', 'Archived')),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_bonds_status ON bonds(status);
CREATE INDEX idx_bonds_subscription_period_end ON bonds(subscription_period_end);
```

### bond_investors

```sql path=null start=null
CREATE TABLE bond_investors (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    bond_id TEXT NOT NULL REFERENCES bonds(postsale_token_contract_address),
    account_address TEXT NOT NULL,
    total_invested DECIMAL(78, 0) NOT NULL DEFAULT 0,
    presale_balance DECIMAL(78, 0) NOT NULL DEFAULT 0,
    postsale_balance DECIMAL(78, 0) NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(bond_id, account_address)
);

CREATE INDEX idx_bond_investors_account ON bond_investors(account_address);
```

### bond_investment_records

```sql path=null start=null
CREATE TABLE bond_investment_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    bond_id TEXT NOT NULL REFERENCES bonds(postsale_token_contract_address),
    investor_address TEXT NOT NULL,
    reward_id TEXT,
    record_type TEXT NOT NULL CHECK (record_type IN ('invest', 'divest', 'claim', 'refund')),
    amount DECIMAL(78, 0) NOT NULL,
    token_type TEXT CHECK (token_type IN ('presale', 'postsale')),
    token_id TEXT,
    transaction_hash BYTEA,
    processed_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_bond_investment_records_reward_id ON bond_investment_records(reward_id);
CREATE INDEX idx_bond_investment_records_bond ON bond_investment_records(bond_id, processed_at DESC);
```

## Diesel Models

### Bond Model

```rust path=null start=null
#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = bonds)]
pub struct Bond {
    pub postsale_token_contract_address: String,
    pub maturity_date: DateTime<Utc>,
    pub interest_rate_type: String,
    pub maximum_supply: Decimal,
    pub minimum_raise_amount: Decimal,
    pub lockup_period_duration: String, // Interval as string
    pub subscription_period_end: DateTime<Utc>,
    pub bond_price: Decimal,
    pub presale_token_contract_address: String,
    pub current_supply: Decimal,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Bond {
    pub fn new(
        bond_address: ContractAddress,
        maturity_date: Timestamp,
        subscription_period_end: Timestamp,
        processed_at: NaiveDateTime,
    ) -> NewBond {
        // Implementation details for creating new bond
    }

    pub fn insert(new_bond: NewBond, conn: &mut DbConn) -> QueryResult<Self> {
        diesel::insert_into(bonds::table)
            .values(&new_bond)
            .get_result(conn)
    }

    pub fn update_status(
        bond_address: Decimal,
        new_status: String,
        updated_at: NaiveDateTime,
        conn: &mut DbConn,
    ) -> QueryResult<Self> {
        diesel::update(bonds::table.filter(bonds::postsale_token_contract_address.eq(bond_address.to_string())))
            .set((
                bonds::status.eq(new_status),
                bonds::updated_at.eq(DateTime::from_utc(updated_at, Utc)),
            ))
            .get_result(conn)
    }
}
```

### BondInvestor Model

```rust path=null start=null
#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = bond_investors)]
pub struct BondInvestor {
    pub id: Uuid,
    pub bond_id: String,
    pub account_address: String,
    pub total_invested: Decimal,
    pub presale_balance: Decimal,
    pub postsale_balance: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl BondInvestor {
    pub fn upsert_investment(
        bond_id: Decimal,
        investor: AccountAddress,
        amount: Amount,
        token_type: &str,
        conn: &mut DbConn,
    ) -> QueryResult<Self> {
        // Implementation for upsert logic based on token_type
    }

    pub fn process_claim(
        bond_id: Decimal,
        investor: AccountAddress,
        amount: Amount,
        conn: &mut DbConn,
    ) -> QueryResult<Self> {
        // Implementation for claim processing
    }

    pub fn process_refund(
        bond_id: Decimal,
        investor: AccountAddress,
        amount: Amount,
        conn: &mut DbConn,
    ) -> QueryResult<Self> {
        // Implementation for refund processing
    }
}
```

### BondInvestmentRecord Model

```rust path=null start=null
#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = bond_investment_records)]
pub struct BondInvestmentRecord {
    pub id: Uuid,
    pub bond_id: String,
    pub investor_address: String,
    pub reward_id: Option<String>,
    pub record_type: String,
    pub amount: Decimal,
    pub token_type: Option<String>,
    pub token_id: Option<String>,
    pub transaction_hash: Option<Vec<u8>>,
    pub processed_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl BondInvestmentRecord {
    pub fn insert_investment(
        bond_id: Decimal,
        investor: AccountAddress,
        amount: Amount,
        token_type: &str,
        reward_id: String,
        processed_at: NaiveDateTime,
        conn: &mut DbConn,
    ) -> QueryResult<Self> {
        // Implementation for investment record
    }

    pub fn insert_claim(
        bond_id: Decimal,
        investor: AccountAddress,
        amount: Amount,
        processed_at: NaiveDateTime,
        conn: &mut DbConn,
    ) -> QueryResult<Self> {
        // Implementation for claim record
    }

    pub fn insert_refund(
        bond_id: Decimal,
        investor: AccountAddress,
        amount: Amount,
        processed_at: NaiveDateTime,
        conn: &mut DbConn,
    ) -> QueryResult<Self> {
        // Implementation for refund record
    }
    
    pub fn get_postsale_token_holders(
        bond_id: &str,
        conn: &mut DbConn,
    ) -> QueryResult<Vec<(String, Decimal)>> {
        // Query to get all investors with postsale_balance > 0 for yield distribution
        // Returns Vec<(account_address, postsale_balance)>
        // Used by yields-backend.md for token holder snapshots
        bond_investors::table
            .filter(
                bond_investors::bond_id.eq(bond_id)
                .and(bond_investors::postsale_balance.gt(Decimal::ZERO))
            )
            .select((bond_investors::account_address, bond_investors::postsale_balance))
            .load::<(String, Decimal)>(conn)
    }
}

---

# PART IV: FR-BT-5 MATURITY HANDLING

## API Layer

### File Structure
- New file: backend/upwood/src/api/bond_maturity.rs

### Endpoints

#### POST /admin/bonds/{bond_contract_address}/maturity/trigger (Admin)

**NEW** - Trigger maturity payments for a bond (two-phase transaction process)

Input:
- `plt_token_address` (string, required) - PLT token contract address for payments
- `face_value_per_token` (decimal, required) - Amount to pay per bond token

Output:
- `maturity_job_id` (string)
- `total_recipients` (integer) - Number of whitelisted bond token holders
- `total_plt_required` (decimal) - Total PLT needed for all payments
- `cloud_wallet_balance` (decimal) - Current PLT balance in cloud wallet
- `sufficient_liquidity` (boolean)
- `status` (string) - "initiated" | "insufficient_liquidity"

#### GET /admin/bonds/maturity/{maturity_job_id}/status (Admin)

Get maturity payment job status

Output:
- `job_id` (string)
- `bond_contract_address` (string)
- `status` (string) - "initiated", "processing", "completed", "failed", "insufficient_liquidity"
- `total_recipients` (integer)
- `processed_recipients` (integer)
- `failed_recipients` (integer)
- `total_tokens_burned` (decimal)
- `total_plt_transferred` (decimal)
- `started_at` (ISO timestamp)
- `completed_at` (ISO timestamp, nullable)
- `error_message` (string, nullable)
- `transaction_details` (object with burn/transfer transaction info)

#### GET /admin/bonds/{bond_contract_address}/maturity/history (Admin)

Get maturity payment history for a bond

Query parameters:
- `limit` (number, optional)
- `offset` (number, optional)

Output:
- `maturity_payments` (array of payment records)
- `total_count` (number)

### Security
- Admin auth via Cognito
- Integration with identity registry for whitelist/blacklist checks
- Cloud wallet operator permissions for token burning

## Database Schema

### maturity_jobs

```sql path=null start=null
CREATE TABLE maturity_jobs (
    id TEXT PRIMARY KEY,
    bond_contract_address TEXT NOT NULL REFERENCES bonds(postsale_token_contract_address),
    plt_token_address TEXT NOT NULL,
    face_value_per_token DECIMAL(78, 0) NOT NULL,
    total_recipients INTEGER NOT NULL,
    total_bond_tokens DECIMAL(78, 0) NOT NULL,
    total_plt_required DECIMAL(78, 0) NOT NULL,
    cloud_wallet_balance DECIMAL(78, 0) NOT NULL,
    sufficient_liquidity BOOLEAN NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('initiated', 'processing', 'completed', 'failed', 'insufficient_liquidity')),
    triggered_by TEXT NOT NULL,
    started_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMP WITH TIME ZONE,
    error_message TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_maturity_jobs_bond_contract ON maturity_jobs(bond_contract_address);
CREATE INDEX idx_maturity_jobs_status ON maturity_jobs(status);
CREATE INDEX idx_maturity_jobs_started_at ON maturity_jobs(started_at);
```

### maturity_payments

```sql path=null start=null
CREATE TABLE maturity_payments (
    id TEXT PRIMARY KEY,
    maturity_job_id TEXT NOT NULL REFERENCES maturity_jobs(id),
    investor_address TEXT NOT NULL,
    bond_tokens_held DECIMAL(78, 0) NOT NULL,
    tokens_burned DECIMAL(78, 0),
    plt_payment_amount DECIMAL(78, 0) NOT NULL,
    burn_transaction_hash TEXT,
    plt_transaction_hash TEXT,
    status TEXT NOT NULL CHECK (status IN ('pending', 'burning', 'burned', 'transferring', 'completed', 'failed')),
    whitelist_status TEXT NOT NULL CHECK (whitelist_status IN ('whitelisted', 'not_whitelisted', 'blacklisted')),
    burn_completed_at TIMESTAMP WITH TIME ZONE,
    plt_completed_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_maturity_payments_job ON maturity_payments(maturity_job_id);
CREATE INDEX idx_maturity_payments_investor ON maturity_payments(investor_address);
CREATE INDEX idx_maturity_payments_status ON maturity_payments(status);
CREATE INDEX idx_maturity_payments_whitelist_status ON maturity_payments(whitelist_status);
```

## Diesel Models

```rust path=null start=null
#[derive(Queryable, Insertable, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = maturity_jobs)]
pub struct MaturityJob {
    pub id: String,
    pub bond_contract_address: String,
    pub plt_token_address: String,
    pub face_value_per_token: Decimal,
    pub total_recipients: i32,
    pub total_bond_tokens: Decimal,
    pub total_plt_required: Decimal,
    pub cloud_wallet_balance: Decimal,
    pub sufficient_liquidity: bool,
    pub status: String, // initiated, processing, completed, failed, insufficient_liquidity
    pub triggered_by: String,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Queryable, Insertable, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = maturity_payments)]
pub struct MaturityPayment {
    pub id: String,
    pub maturity_job_id: String,
    pub investor_address: String,
    pub bond_tokens_held: Decimal,
    pub tokens_burned: Option<Decimal>,
    pub plt_payment_amount: Decimal,
    pub burn_transaction_hash: Option<String>,
    pub plt_transaction_hash: Option<String>,
    pub status: String, // pending, burning, burned, transferring, completed, failed
    pub whitelist_status: String, // whitelisted, not_whitelisted, blacklisted
    pub burn_completed_at: Option<DateTime<Utc>>,
    pub plt_completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

## Integration Notes

### Two-Phase Transaction Process

1. **Phase 1: Token Burning**
   - Query bond_investors table for all postsale_balance > 0
   - Check whitelist status via identity-registry API for each investor
   - Cloud wallet burns bond tokens for whitelisted investors only
   - Update maturity_payments with burn transaction hashes and status

2. **Phase 2: PLT Transfer**
   - After successful token burning, transfer PLT tokens to investors
   - Amount = face_value_per_token * tokens_burned
   - Update maturity_payments with PLT transaction hashes and completion status

### Liquidity Management

- Check cloud wallet PLT balance before initiating maturity job
- All-or-none payment policy: sufficient liquidity required for all payments
- If insufficient liquidity, job status set to "insufficient_liquidity"
- Admin must ensure adequate PLT balance before triggering maturity

### Whitelist Integration

- Query identity-registry API for address states before processing
- Only whitelisted addresses eligible for maturity payments
- Blacklisted addresses excluded from both phases
- Not-whitelisted addresses excluded but tracked for audit purposes

### Error Handling

- Failed token burns retry with exponential backoff
- Detailed error logging for failed transactions
- Manual retry capabilities for failed payments
- Comprehensive status tracking for both burn and transfer phases
- Transaction rollback capabilities if partial failures occur

### Maturity Workflow

1. Admin triggers maturity payment for bond contract
2. System queries bond_investors for all postsale token holders
3. Check cloud wallet PLT balance against total required
4. If sufficient liquidity:
   - Create maturity_job record with status "initiated"
   - Create maturity_payment records for each eligible investor
   - Check whitelist status for each investor via identity-registry API
5. **Processing**: Execute two-phase transaction
   - Update job status to "processing"
   - **Phase 1**: Process token burns for whitelisted investors in batches
   - Update individual payment records with burn transaction hashes and status
   - **Phase 2**: Transfer PLT tokens to investors whose tokens were successfully burned
   - Update payment records with PLT transaction hashes and status
6. **Completion**: Update job status to "completed"
   - Record completion timestamp
   - Generate audit trail for all transactions
