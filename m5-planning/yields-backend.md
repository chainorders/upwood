# Yields Backend Planning

This document defines the yields backend planning, combining API endpoints and database schema for yield distribution management.

## Background

Yields backend handles FR-BT-2 Interest rate configuration and FR-BT-4 Yield distribution:

- Yield configuration management (Fixed and Variable types)
- Yield payment processing and distribution in PLT tokens
- Bond token holder snapshot with holding period calculation
- Integration with KYC third-party service for yield eligibility
- Yield calculation using fixed and variable interest formulas with holding period

**FR-BT-4 Yield Distribution Key Feature:**

- Security-sft-multi contract uses token IDs as timestamps (days since Unix epoch)
- Holding period calculated per token ID: `interest_period_end - max(token_mint_date, interest_period_start)`
- Enables precise yield calculation based on actual holding period and amount

Note:

- Blacklist management is handled on-chain via the identity registry contract with read functions in identity-registry API
- Trading fee functionality is handled by security-sft-multi blockchain component
- Yields are paid in Concordium PLT Token to bond token holders

## API Layer

### File Structure

- New file: backend/upwood/src/api/yields.rs

### Endpoints

#### POST /admin/yields/configure (Admin)

Configure yield parameters for a bond (after subscription period ends with >= minimum amount raised)

Input:

- `bond_metadata_id` (UUID, required) - Bond metadata ID
- `yield_type` (string, required) - "fixed" or "variable"
- `plt_token_address` (string, required) - PLT token contract address for payments
- `yield_time` (ISO timestamp, required) - When yield becomes eligible for triggering
- `face_value` (decimal, required) - Face value per token (F)
- `coupon_rate` (decimal, required) - Coupon rate (c)
- `payment_frequency` (integer, required) - 1=annual, 2=semi-annual, 4=quarterly (p)
- `fiscal_year_end` (string, optional) - MM-DD format for variable yields (e.g. "12-31")
- `profits_before_tax` (decimal, optional) - Required for variable yields (PBTevi)
- `variable_rate` (decimal, optional) - Required for variable yields (c)

Output:

- `yield_config_id` (string)
- `success` (boolean)
- `created_at` (ISO timestamp)

#### PUT /admin/yields/{yield_config_id} (Admin)

Update existing yield configuration

Input: Same as POST configure
Output: Same as POST configure with `updated_at`

#### POST /admin/yields/{yield_config_id}/trigger (Admin)

Trigger yield distribution (after yield_time has passed)

Output:

- `distribution_job_id` (string)
- `total_recipients` (integer) - Number of bond token holders
- `estimated_total_amount` (decimal)
- `status` (string) - "initiated"

#### GET /admin/yields/bond/{bond_metadata_id} (Admin)

Get yield configurations for a bond

Output:

- `yield_configs` (array of yield configuration objects)
- `bond_metadata_id` (string)

#### GET /admin/yields/distribution/{distribution_job_id} (Admin)

Get yield distribution job status

Output:

- `job_id` (string)
- `status` (string) - "pending", "processing", "completed", "failed"
- `total_recipients` (integer)
- `processed_recipients` (integer)
- `failed_recipients` (integer)
- `started_at` (ISO timestamp)
- `completed_at` (ISO timestamp, nullable)
- `error_message` (string, nullable)

#### GET /admin/yields/history/{bond_metadata_id} (Admin)

Get yield payment history for a bond

Query parameters:

- `limit` (number, optional)
- `offset` (number, optional)

Output:

- `payments` (array of payment records)
- `total_count` (number)


### Security

- Admin auth via Cognito
- Integration with third-party KYC service for verification

## Database Layer

### Yield Configuration

#### yield_configs

- id (string, PK)
- bond_metadata_id (string) # Bond metadata ID
- yield_type (string) # "fixed" or "variable"
- plt_token_address (string) # PLT token for payments
- yield_time (timestamp) # When yield becomes eligible
- face_value (decimal) # F - face value per token
- coupon_rate (decimal) # c - coupon rate
- payment_frequency (integer) # p - 1=annual, 2=semi-annual, 4=quarterly
- fiscal_year_end (string, nullable) # MM-DD format for variable yields
- profits_before_tax (decimal, nullable) # PBTevi for variable yields
- variable_rate (decimal, nullable) # c for variable yields
- created_at (timestamp)
- updated_at (timestamp)
- created_by (string) # Admin user ID

Indexes:

- idx_yield_configs_bond_contract
- idx_yield_configs_yield_time
- idx_yield_configs_yield_type

### Yield Distribution

### Yield Distribution

#### yield_payments

- id (string, PK)
- yield_config_id (string) # FK to yield_configs
- distribution_job_id (string) # FK to yield_distribution_jobs
- investor_address (string)
- bond_tokens_held (decimal) # Number of tokens held
- payment_amount (decimal) # Calculated yield amount in PLT
- payment_date (timestamp) # Dn
- previous_payment_date (timestamp, nullable) # Dn-1
- payment_sequence (integer) # n - payment sequence number
- plt_transaction_hash (string, nullable)
- status (string) # scheduled, processing, completed, failed
- calculation_details (jsonb) # Store calculation breakdown
- created_at (timestamp)
- updated_at (timestamp)

Indexes:

- idx_yield_payments_yield_config
- idx_yield_payments_distribution_job
- idx_yield_payments_investor
- idx_yield_payments_payment_date
- idx_yield_payments_status

#### yield_distribution_jobs

- id (string, PK)
- yield_config_id (string) # FK to yield_configs
- bond_contract_address (string)
- total_bond_tokens (decimal) # Total tokens eligible for yield
- total_plt_amount (decimal) # Total PLT to distribute
- total_recipients (integer)
- processed_recipients (integer)
- failed_recipients (integer)
- status (string) # pending, processing, completed, failed
- triggered_by (string) # Admin user ID
- started_at (timestamp)
- completed_at (timestamp, nullable)
- error_message (text, nullable)

Indexes:

- idx_yield_distribution_jobs_yield_config
- idx_yield_distribution_jobs_bond_contract
- idx_yield_distribution_jobs_status
- idx_yield_distribution_jobs_started_at

#### investor_yield_history

- id (string, PK)
- investor_address (string)
- bond_contract_address (string)
- yield_config_id (string)
- payment_id (string) # FK to yield_payments
- amount_paid (decimal)
- payment_date (timestamp)
- yield_type (string) # "fixed" or "variable"
- payment_sequence (integer)

Indexes:

- idx_investor_yield_history_investor
- idx_investor_yield_history_bond_contract
- idx_investor_yield_history_payment_date


### Yield Calculation Formulas

#### Fixed Interest Calculation (per token)

```rust
// FIp = (F * c) / (Y / p) * (Dn – Dn-1)
// Where:
// F = face_value, c = coupon_rate, p = payment_frequency
// Y = days_in_year (365 or 366), Dn = payment_date, Dn-1 = previous_payment_date

fn calculate_fixed_interest(
    face_value: BigDecimal,
    coupon_rate: BigDecimal,
    payment_frequency: i32,
    days_in_year: i32,
    payment_date: chrono::NaiveDate,
    previous_payment_date: chrono::NaiveDate,
) -> BigDecimal {
    let days_diff = (payment_date - previous_payment_date).num_days();
    let annual_interest = face_value * coupon_rate;
    let period_fraction = BigDecimal::from(days_diff) / BigDecimal::from(days_in_year / payment_frequency);
    annual_interest * period_fraction
}
```

#### Variable Interest Calculation (per token)

```rust
// VIp = (PBTevi * c * Ti) / T
// Where:
// PBTevi = profits_before_tax, c = variable_rate
// Ti = interest_period_for_token, T = total_interest_period

fn calculate_variable_interest(
    profits_before_tax: BigDecimal,
    variable_rate: BigDecimal,
    token_interest_period: i32, // Days token was held
    total_interest_period: i32, // Total days in fiscal period
) -> BigDecimal {
    let base_interest = profits_before_tax * variable_rate;
    let period_fraction = BigDecimal::from(token_interest_period) / BigDecimal::from(total_interest_period);
    base_interest * period_fraction
}
```

#### FR-BT-4: Token ID to Date Conversion

```rust
// Convert token ID (u64 days since Unix epoch) to mint date
fn token_id_to_mint_date(token_id: u64) -> chrono::NaiveDate {
    let unix_epoch = chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
    unix_epoch + chrono::Duration::days(token_id as i64)
}
```

#### FR-BT-4: Holding Period Calculation

```rust
// Calculate holding period for yield distribution
// holding_period = interest_period_end - max(token_mint_date, interest_period_start)
fn calculate_holding_period(
    token_mint_date: chrono::NaiveDate,
    interest_period_start: chrono::NaiveDate,
    interest_period_end: chrono::NaiveDate,
) -> i32 {
    let effective_start = std::cmp::max(token_mint_date, interest_period_start);
    std::cmp::max(0, (interest_period_end - effective_start).num_days() as i32)
}
```

#### FR-BT-4: Fixed Interest with Holding Period

```rust
// Fixed interest calculation using actual holding period per token
// FIp = (F * c * holding_period_days) / days_in_year
fn calculate_fixed_interest_with_holding_period(
    face_value: BigDecimal,
    coupon_rate: BigDecimal,
    holding_period_days: i32,
    days_in_year: i32,
) -> BigDecimal {
    if holding_period_days <= 0 {
        return BigDecimal::from(0);
    }
    
    let annual_interest = face_value * coupon_rate;
    let holding_period_fraction = BigDecimal::from(holding_period_days) / BigDecimal::from(days_in_year);
    annual_interest * holding_period_fraction
}
```

#### FR-BT-4: Variable Interest with Holding Period

```rust
// Variable interest calculation using actual holding period per token
// VIp = (PBTevi * c * holding_period_days) / total_interest_period_days
fn calculate_variable_interest_with_holding_period(
    profits_before_tax: BigDecimal,
    variable_rate: BigDecimal,
    holding_period_days: i32,
    total_interest_period_days: i32,
) -> BigDecimal {
    if holding_period_days <= 0 || total_interest_period_days <= 0 {
        return BigDecimal::from(0);
    }
    
    let base_interest = profits_before_tax * variable_rate;
    let period_fraction = BigDecimal::from(holding_period_days) / BigDecimal::from(total_interest_period_days);
    base_interest * period_fraction
}
```

### Diesel Models

```rust
#[derive(Queryable, Insertable, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = yield_configs)]
pub struct YieldConfig {
    pub id: String,
    pub bond_contract_address: String,
    pub yield_type: String, // "fixed" or "variable"
    pub plt_token_address: String,
    pub yield_time: chrono::NaiveDateTime,
    pub face_value: BigDecimal,
    pub coupon_rate: BigDecimal,
    pub payment_frequency: i32,
    pub fiscal_year_end: Option<String>,
    pub profits_before_tax: Option<BigDecimal>,
    pub variable_rate: Option<BigDecimal>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub created_by: String,
}


#[derive(Queryable, Insertable, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = yield_payments)]
pub struct YieldPayment {
    pub id: String,
    pub yield_config_id: String,
    pub distribution_job_id: String,
    pub investor_address: String,
    pub bond_tokens_held: BigDecimal,
    pub payment_amount: BigDecimal,
    pub payment_date: chrono::NaiveDateTime,
    pub previous_payment_date: Option<chrono::NaiveDateTime>,
    pub payment_sequence: i32,
    pub plt_transaction_hash: Option<String>,
    pub status: String, // scheduled, processing, completed, failed
    pub calculation_details: Option<serde_json::Value>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Insertable, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = yield_distribution_jobs)]
pub struct YieldDistributionJob {
    pub id: String,
    pub yield_config_id: String,
    pub bond_contract_address: String,
    pub total_bond_tokens: BigDecimal,
    pub total_plt_amount: BigDecimal,
    pub total_recipients: i32,
    pub processed_recipients: i32,
    pub failed_recipients: i32,
    pub status: String, // pending, processing, completed, failed
    pub triggered_by: String,
    pub started_at: chrono::NaiveDateTime,
    pub completed_at: Option<chrono::NaiveDateTime>,
    pub error_message: Option<String>,
}

#[derive(Queryable, Insertable, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = investor_yield_history)]
pub struct InvestorYieldHistory {
    pub id: String,
    pub investor_address: String,
    pub bond_contract_address: String,
    pub yield_config_id: String,
    pub payment_id: String,
    pub amount_paid: BigDecimal,
    pub payment_date: chrono::NaiveDateTime,
    pub yield_type: String,
    pub payment_sequence: i32,
}
```

## Integration Notes

### Bond Integration

- Yield configuration only available after bond subscription period ends with >= minimum amount raised
- Bond post-sale token contract address used as primary identifier
- **Queries bond processor database** (managed by bond-blockchain.md) for current token holder data
- **No bond token snapshots stored locally** - yields backend queries bond processor database in real-time
- Token IDs and holder balances retrieved via bond processor database queries
- FR-BT-4 holding period calculations performed using token IDs from bond processor data

### PLT Token Integration

- All yield payments distributed in Concordium PLT Token
- Each yield config specifies PLT token contract address
- Cloud wallet integration for batch PLT token transfers
- Transaction chunking for large distribution batches

### Blacklist & KYC Integration

- Blacklist status read from identity-registry API endpoints
- KYC status checked via third-party service integration
- Blacklisted wallets excluded from yield distribution
- Failed payments logged with reason (blacklisted/kyc_failed)

### Calculation Engine

- Fixed interest: FIp = (F *c) / (Y / p)* (Dn – Dn-1)
- Variable interest: VIp = (PBTevi *c* Ti) / T
- Leap year handling for accurate day calculations
- Payment sequence tracking for multiple distributions

### FR-BT-4 Distribution Workflow

1. Admin configures yield parameters after bond subscription ends
2. When yield_time passes, admin triggers distribution
3. **Token Snapshot by Token ID**: System queries security-sft-multi contract for all token holders by specific token ID
4. **Convert Token IDs to Mint Dates**: Each token ID (u64) represents days since Unix epoch
5. **Calculate Holding Period per Token**: For each token ID: `holding_period = interest_period_end - max(token_mint_date, interest_period_start)`
6. **Calculate Yield per Token ID**: Apply FR-BT-4 formulas individually for each token ID held by each investor
7. **Sum Yields per Investor**: Total all individual token yields for each investor address
8. **PLT Token Distribution**: Cloud wallet processes calculated total amounts per investor in batches
9. **Status Updates**: Record payment status, transaction hashes, and detailed calculation breakdown
10. **Audit Trail**: Maintain complete history with per-token calculation details in investor_yield_history

### Error Handling

- Failed transfers retry with exponential backoff
- Detailed error logging for failed distributions
- Manual retry capabilities for failed payments
- Comprehensive status tracking throughout process
