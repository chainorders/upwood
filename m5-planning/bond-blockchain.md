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
    maximum_supply: Amount,
    minimum_raise_amount: Amount,
    lockup_period_duration: Duration,
    subscription_period_end: Timestamp,
    bond_price: Amount,
    presale_token_contract_address: ContractAddress,
    postsale_token_contract_address: ContractAddress,
    current_supply: Amount,
    status: BondStatus,
    // Carbon Credits Pool Integration (FR-CC-1, FR-CC-2, FR-CC-3)
    carbon_credit_balance: Amount,
    carbon_credits_burned: Amount,
    carbon_credit_metadata: Option<TokenMetadata>, // CIS-2 TokenMetadata
}
```

## Core Functions

### add_bond

**Agent Role:** AddFund

**Input Parameters:**

```rust path=null start=null
struct AddBondParams {
    maturity_date: Timestamp,
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

## Carbon Credit Functions

### receive_carbon_credits

**Agent Role:** Public (CIS-2 compliant receiver function)

**Function Signature:** Uses CIS-2 standard `onReceivingCIS2` callback

**Input Parameters:**

```rust path=null start=null
// CIS-2 compliant function signature
pub fn on_receiving_cis2(
    ctx: &ReceiveContext,
    host: &Host<State>,
) -> ContractResult<()> {
    let params: OnReceivingCis2DataParams<TokenIdUnit, Amount, ContractAddress> = 
        ctx.parameter_cursor().get()?;
    
    // params.token_id - Carbon credit token ID
    // params.amount - Amount of carbon credits received
    // params.from - Address sending the carbon credits
    // params.data - Bond contract address to credit
}
```

### set_carbon_credit_metadata

**Agent Role:** UpdateFundState

**Input Parameters:**

```rust path=null start=null
struct SetCarbonCreditMetadataParams {
    postsale_token_contract_address: ContractAddress,
    metadata: TokenMetadata, // CIS-2 TokenMetadata
}
```

### burn_carbon_credits

**Agent Role:** UpdateFundState

**Input Parameters:**

```rust path=null start=null
struct BurnCarbonCreditsParams {
    postsale_token_contract_address: ContractAddress,
    amount: Amount,
}
```

## Carbon Credit Events

```rust path=null start=null
// Minimized event sizes to reduce blockchain transaction costs
struct CarbonCreditsReceivedEvent {
    bond_address: ContractAddress,
    balance: Amount, // Total final balance after receiving
}

struct CarbonCreditMetadataUpdatedEvent {
    bond_address: ContractAddress,
    metadata: TokenMetadata,
}

struct CarbonCreditsBurnedEvent {
    bond_address: ContractAddress,
    burned_amount: Amount, // Amount burned in this transaction
}
```

## Bond Events

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
    reward_id: Vec<u8>, // PLT transaction hash used as proof correlation
    nonce: u64, // Investor nonce that was used (for backend sync)
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

## Payment Proof Verification (FR-PM-1)

**PLT Integration**: Payment proofs enable PLT token payments to be converted to smart contract investments

```rust path=null start=null
struct PaymentProof {
    reward_id: Vec<u8>, // PLT transaction hash from off-chain payment
    nonce: u64, // Current investor nonce for replay protection
    signer: AccountAddress, // Platform signer account
    signature: Signature, // Signature over canonical message
}

fn expected_message(investor, bond, amount, reward_id, nonce) -> Vec<u8>
fn verify_proof(proof: &PaymentProof, message: &[u8]) -> ContractResult<bool>
```

**Nonce Mechanism (Replay Protection)**:
1. Contract maintains `investor_nonces: StateMap<AccountAddress, u64>` in state
2. Each investment must provide `nonce == current_investor_nonce + 1`
3. After successful investment, contract increments investor's nonce in state
4. Contract rejects any proof with `nonce <= current_investor_nonce`
5. Backend syncs nonce state via `BondInvestmentEvent` processing

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
                
                // FR-PM-1: Update investor nonce for payment proof sync
                InvestorNonce::update_nonce(
                    e.investor.to_string(),
                    e.bond_address.to_string(),
                    e.nonce,
                    block_height,
                    conn
                )?;
                
                // FR-PM-1: Mark corresponding payment proof as used
                PltPaymentProof::mark_as_used(
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
            // Carbon Credit Event Processing (FR-CC-1, FR-CC-2, FR-CC-3)
            Event::CarbonCreditsReceived(e) => {
                Bond::update_carbon_credit_balance(
                    e.bond_address.to_decimal(),
                    e.balance.to_decimal(), // Direct balance update
                    block_time,
                    conn
                )?;
            }
            Event::CarbonCreditsBurned(e) => {
                // Increment burned amount and calculate new balance
                let current_bond = Bond::find_by_address(e.bond_address.to_decimal(), conn)?;
                let new_burned_total = current_bond.carbon_credits_burned + e.burned_amount.to_decimal();
                let new_balance = current_bond.carbon_credit_balance - e.burned_amount.to_decimal();
                
                Bond::update_carbon_credits_burned(
                    e.bond_address.to_decimal(),
                    new_burned_total,
                    new_balance,
                    block_time,
                    conn
                )?;
            }
            Event::CarbonCreditMetadataUpdated(e) => {
                let metadata_url = e.metadata.url.to_string(); // Extract URL from TokenMetadata
                Bond::update_carbon_credit_metadata(
                    e.bond_address.to_decimal(),
                    metadata_url,
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
    postsale_token_contract BIGINT PRIMARY KEY,
    maturity_date TIMESTAMP WITH TIME ZONE NOT NULL,
    maximum_supply DECIMAL(78, 0) NOT NULL,
    minimum_raise_amount DECIMAL(78, 0) NOT NULL,
    lockup_period_duration INTERVAL NOT NULL,
    subscription_period_end TIMESTAMP WITH TIME ZONE NOT NULL,
    bond_price DECIMAL(78, 0) NOT NULL,
    presale_token_contract BIGINT NOT NULL,
    current_supply DECIMAL(78, 0) NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Active' CHECK (status IN ('Active', 'Paused', 'Matured', 'Success', 'Failed', 'Archived')),
    -- Carbon Credit Pool Integration (FR-CC-1, FR-CC-2, FR-CC-3)
    carbon_credit_balance DECIMAL(78, 0) NOT NULL DEFAULT 0,
    carbon_credits_burned DECIMAL(78, 0) NOT NULL DEFAULT 0,
    carbon_credit_metadata TEXT, -- URL string for CIS-2 TokenMetadata
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_bonds_status ON bonds(status);
CREATE INDEX idx_bonds_subscription_period_end ON bonds(subscription_period_end);
CREATE INDEX idx_bonds_carbon_credit_balance ON bonds(carbon_credit_balance) WHERE carbon_credit_balance > 0;
```

### bond_investors

```sql path=null start=null
CREATE TABLE bond_investors (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    bond_id BIGINT NOT NULL REFERENCES bonds(postsale_token_contract),
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
    bond_id BIGINT NOT NULL REFERENCES bonds(postsale_token_contract),
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
    pub maximum_supply: Decimal,
    pub minimum_raise_amount: Decimal,
    pub lockup_period_duration: String, // Interval as string
    pub subscription_period_end: DateTime<Utc>,
    pub bond_price: Decimal,
    pub presale_token_contract_address: String,
    pub current_supply: Decimal,
    pub status: String,
    // Carbon Credit Pool Integration (FR-CC-1, FR-CC-2, FR-CC-3)
    pub carbon_credit_balance: Decimal,
    pub carbon_credits_burned: Decimal,
    pub carbon_credit_metadata: Option<String>, // URL string for CIS-2 TokenMetadata
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Diesel implementations removed to reduce file size - will be implemented during coding phase
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
```

# PART IV: FR-BT-5 MATURITY HANDLING DATABASE SCHEMA

## Database Schema

### maturity_jobs

```sql path=null start=null
CREATE TABLE maturity_jobs (
    id TEXT PRIMARY KEY,
    bond_contract BIGINT NOT NULL REFERENCES bonds(postsale_token_contract),
    plt_token_id TEXT NOT NULL,
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

CREATE INDEX idx_maturity_jobs_bond_contract ON maturity_jobs(bond_contract);
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
    error_status TEXT, -- NULL if successful, 'non_whitelisted' if not whitelisted, other error messages
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_maturity_payments_job ON maturity_payments(maturity_job_id);
CREATE INDEX idx_maturity_payments_investor ON maturity_payments(investor_address);
CREATE INDEX idx_maturity_payments_error_status ON maturity_payments(error_status);
```

## Diesel Models

```rust path=null start=null
#[derive(Queryable, Insertable, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = maturity_jobs)]
pub struct MaturityJob {
    pub id: String,
    pub bond_contract: u64,
    pub plt_token_id: String,
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
    pub error_status: Option<String>, // NULL if successful, 'non_whitelisted' if not whitelisted, other error messages
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
- Non-whitelisted addresses excluded with error_status 'non_whitelisted'
- Blacklisted addresses excluded with error_status 'blacklisted'

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
