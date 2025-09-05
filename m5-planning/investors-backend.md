# Investors Backend API (FR-ID-2, FR-ID-3 & Portfolio Management)

This document defines the comprehensive backend API and database layer for investor management functionality, including admin investor management, user portfolio dashboards, transaction history, and PDF generation capabilities.

## API Layer

### Admin Investor Management Endpoints

#### List All Registered Investors

```http path=null start=null
GET /admin/investors
Authorization: Bearer <admin_jwt_token>
Query Parameters:
  - page (optional): Page number (default: 0)
  - page_size (optional): Results per page (default: 50, max: 200)
  - status_filter (optional): "whitelisted" | "blacklisted" | "not_in_registry" | "all"
  - search (optional): Search by email, name, or account address
```

**Purpose**: Display comprehensive investor list for admin "Investors" section in frontend UI

**Authentication**: Admin or Compliance Officer role required

**Response**: `PagedResponse<InvestorSummary>`

```typescript path=null start=null
{
  "data": [
    {
      "account_address": "4owvMHZAXXX...",
      "email": "investor@example.com",
      "first_name": "John",
      "last_name": "Doe",
      "nationality": "DK",
      "identity_status": "whitelisted", // "not_in_registry" | "whitelisted" | "blacklisted"
      "kyc_verified": true,
      "total_investment_value": "25000.00", // EUR equivalent
      "active_bond_count": 3,
      "total_yield_received": "1250.00", // Total dividends paid
      "created_at": "2024-01-15T10:30:00Z",
      "last_activity": "2024-03-15T14:22:00Z"
    }
  ],
  "page": 0,
  "page_count": 12,
  "total_investors": 234
}
```

**Database Query Integration**:

- Join `users` table with `identities` table for identity registry status
- Aggregate bond holdings from `bond_token_holders` table
- Sum yield payments from `investor_yield_history` table
- Include search functionality across name, email, account address fields

#### Export Investors CSV

```http path=null start=null
GET /admin/investors/export
Authorization: Bearer <admin_jwt_token>
Query Parameters:
  - format: "csv" (default)
  - include_inactive: boolean (default: false)
  - registry_status: "all" | "whitelisted" | "blacklisted" | "not_in_registry" (optional)
```

**Purpose**: Download CSV file with all investor details from the same investor list page

**Authentication**: Admin role required

**Response**: CSV file download with headers:

- Content-Type: text/csv
- Content-Disposition: attachment; filename="investors_export_YYYY-MM-DD.csv"

**CSV Columns**:

```csv path=null start=null
investor_id,email,first_name,last_name,nationality,phone,address,country,registration_date,wallet_address,identity_registry_status,kyc_status,total_bond_investments,total_investment_amount,total_yield_received,last_activity,is_active
INV-001,john@example.com,John,Doe,DK,+1234567890,"123 Main St",Denmark,2024-01-15T10:00:00Z,4owvMHZAXXX...,whitelisted,approved,3,50000.00,2500.00,2024-03-10T14:30:00Z,true
```

**Data Sources**:

- Primary: `users`, `identities` tables
- Join with `bond_token_holders` for investment totals
- Join with `investor_yield_history` for yield totals
- Include KYC status from `kyc_records`
- Apply same filters as investor list view

#### Get Investor Details

```http path=null start=null
GET /admin/investors/{account_address}
Authorization: Bearer <admin_jwt_token>
```

**Purpose**: Detailed investor view when admin clicks on specific investor in the list

**Authentication**: Admin or Compliance Officer role required

**Response**: `InvestorDetail`

```typescript path=null start=null
{
  "account_address": "4owvMHZAXXX...",
  "email": "investor@example.com",
  "first_name": "John",
  "last_name": "Doe",
  "nationality": "DK",
  "created_at": "2024-01-15T10:30:00Z",
  
  // Identity Registry Status (read-only)
  "identity_status": {
    "current_status": "whitelisted",
    "status_changed_at": "2024-02-01T09:15:00Z",
    "registry_contract": "1000,0"
  },
  
  // KYC Information
  "kyc_details": {
    "verified": true,
    "verification_date": "2024-01-15T10:30:00Z",
    "provider": "digital_trust_solutions"
  },
  
  // Portfolio Summary
  "portfolio_summary": {
    "total_investment_value": "25000.00",
    "total_tokens_held": "25000000000000", // In token base units
    "total_frozen_amount": "0", // In token base units
    "active_bonds_count": 3,
    "yield_payments_received": "1250.00",
    "total_carbon_credits": "125.50", // Allocated based on ownership
    "portfolio_roi": "12.5" // Percentage ROI across all investments
  },
  
  // Recent Activity
  "recent_transactions": [
    {
      "date": "2024-03-10T11:30:00Z",
      "type": "investment",
      "bond_name": "Forest Bond Series A",
      "amount": "5000.00",
      "status": "completed",
      "transaction_hash": "abc123...",
      "isin": "US12345678901"
    }
  ]
}
```

#### Get Investor Bond Holdings

```http path=null start=null
GET /admin/investors/{account_address}/bonds
Authorization: Bearer <admin_jwt_token>
```

**Purpose**: List all bond holdings for compliance officer to view and initiate freeze/unfreeze operations

**Authentication**: Admin or Compliance Officer role required

**Response**: `InvestorBondHoldings`

```typescript path=null start=null
{
  "account_address": "4owvMHZAXXX...",
  "bond_holdings": [
    {
      "postsale_token_contract": "1234,0",
      "bond_name": "Forest Bond Series A",
      "token_id": "1640995200", // Subscription period end timestamp
      "total_amount": "10000000000000", // Total tokens held
      "available_amount": "10000000000000", // Non-frozen tokens
      "frozen_amount": "0", // Frozen tokens
      "investment_value": "10000.00", // EUR equivalent
      "investment_date": "2024-02-15T10:00:00Z",
      "maturity_date": "2025-02-15T10:00:00Z",
      "yield_rate": "8.5",
      "next_yield_date": "2024-12-15T10:00:00Z",
      "isin": "US12345678901",
      "agents": [
        "4owvMHZAXXX...", // List of agents configured on this bond contract
        "4owvMHZBBBB..."
      ]
    }
  ],
  "total_portfolio_value": "25000.00",
  "total_frozen_value": "0.00",
  "identity_registry_contract": "1000,0" // Contract address for frontend to call
}
```

### User Portfolio Dashboard Endpoints

#### Get User Portfolio Dashboard

```http path=null start=null
GET /portfolio/dashboard
Authorization: Bearer <user_jwt_token>
```

**Purpose**: Comprehensive portfolio overview for logged-in investor including bonds, yields, carbon credits, and ROI

**Authentication**: Valid user JWT token required

**Response**: `UserPortfolioDashboard`

```typescript path=null start=null
{
  "account_address": "4owvMHZAXXX...",
  "portfolio_summary": {
    "total_investment_value": "25000.00", // EUR
    "current_portfolio_value": "28125.00", // Including yield payments
    "total_roi": "12.5", // Percentage
    "total_yield_received": "3125.00", // EUR
    "active_bonds_count": 3,
    "matured_bonds_count": 1,
    "total_carbon_credits": "125.50", // Tons CO2
    "carbon_credits_burned": "25.0" // Tons CO2 user has burned
  },
  
  "bond_investments": [
    {
      "postsale_token_contract": "1234,0",
      "bond_name": "Forest Bond Series A",
      "isin": "US12345678901",
      "forest_project": {
        "name": "Amazon Rainforest Conservation",
        "location": "Brazil, Para State",
        "area_hectares": 10000
      },
      "investment_details": {
        "total_invested": "10000.00", // EUR
        "current_value": "11250.00", // Including yields
        "tokens_held": "10.00",
        "yield_rate": "8.5",
        "investment_date": "2024-02-15T10:00:00Z",
        "maturity_date": "2025-02-15T10:00:00Z",
        "status": "active", // "active", "matured", "paused"
        "roi": "12.5", // Individual bond ROI percentage
        "next_yield_date": "2024-12-15T10:00:00Z"
      },
      "carbon_credits": {
        "allocated_credits": "50.25", // Based on ownership percentage
        "credits_burned": "10.0" // Credits user has burned from this bond
      }
    }
  ],
  
  "yield_history": {
    "total_received": "3125.00",
    "payments_count": 5,
    "last_payment": {
      "amount": "625.00",
      "date": "2024-03-01T10:00:00Z",
      "bond_name": "Forest Bond Series A"
    },
    "upcoming_payments": [
      {
        "estimated_amount": "650.00",
        "date": "2024-04-01T10:00:00Z",
        "bond_name": "Forest Bond Series A",
        "status": "scheduled"
      }
    ]
  },
  
  "carbon_credits_summary": {
    "total_allocated": "125.50",
    "total_burned": "25.0",
    "available_to_burn": "100.50",
    "breakdown_by_bond": [
      {
        "bond_name": "Forest Bond Series A",
        "allocated": "50.25",
        "burned": "10.0",
        "available": "40.25"
      }
    ]
  }
}
```

**Database Operations**:

- Join bond holdings with current market values
- Calculate ROI based on investment amount vs current value + yields received
- Aggregate carbon credits allocation based on bond ownership percentages
- Retrieve yield payment history from `investor_yield_history`
- Calculate upcoming yield estimates based on `yield_configs`

#### Get User Bond Details

```http path=null start=null
GET /portfolio/bonds/{bond_contract}
Authorization: Bearer <user_jwt_token>
```

**Purpose**: Detailed view of specific bond investment with performance metrics

**Authentication**: Valid user JWT token required

**Path Parameters**:

- `bond_contract`: Contract address (e.g., "1234,0")

**Response**: `UserBondDetail`

```typescript path=null start=null
{
  "bond": {
    "postsale_token_contract": "1234,0",
    "bond_name": "Forest Bond Series A",
    "isin": "US12345678901",
    "status": "active",
    "forest_project": {
      "name": "Amazon Rainforest Conservation",
      "description": "Protecting 10,000 hectares of Amazon rainforest",
      "location": "Brazil, Para State",
      "area_hectares": 10000,
      "estimated_carbon_credits": 50000,
      "certification_body": "Verra"
    }
  },
  
  "investment_performance": {
    "total_invested": "10000.00", // EUR
    "current_value": "11250.00", // Including yields
    "tokens_held": "10.00",
    "ownership_percentage": "1.25", // Percentage of total bond
    "investment_date": "2024-02-15T10:00:00Z",
    "maturity_date": "2025-02-15T10:00:00Z",
    "yield_rate": "8.5",
    "roi": "12.5", // Current ROI percentage
    "roi_annualized": "9.2" // Annualized ROI
  },
  
  "yield_details": {
    "total_received": "1250.00",
    "payments_received": 3,
    "next_payment": {
      "estimated_amount": "425.00",
      "date": "2024-04-01T10:00:00Z"
    },
    "payment_history": [
      {
        "amount": "416.67",
        "date": "2024-03-01T10:00:00Z",
        "status": "completed",
        "transaction_hash": "xyz789..."
      }
    ]
  },
  
  "carbon_credits": {
    "total_allocated": "50.25", // Based on ownership percentage
    "total_burned": "10.0",
    "available_to_burn": "40.25",
    "allocation_rate": "5.025", // Credits per token held
    "burn_history": [
      {
        "amount": "5.0",
        "date": "2024-02-28T14:00:00Z",
        "transaction_hash": "burn123..."
      }
    ]
  }
}
```

### Transaction History & PDF Generation

#### Get User Transaction History

```http path=null start=null
GET /portfolio/transactions
Authorization: Bearer <user_jwt_token>
Query Parameters:
  - page (optional): Page number (default: 0)
  - page_size (optional): Results per page (default: 50, max: 200)
  - transaction_type (optional): "investment" | "yield_payment" | "carbon_burn" | "transfer" | "all"
  - bond_contract (optional): Filter by specific bond contract
  - date_from (optional): Start date filter (ISO 8601)
  - date_to (optional): End date filter (ISO 8601)
  - sort_order (optional): "asc" | "desc" (default: "desc")
```

**Purpose**: Complete transaction history for investor with filtering and pagination

**Authentication**: Valid user JWT token required

**Response**: `PagedResponse<UserTransaction>`

```typescript path=null start=null
{
  "data": [
    {
      "transaction_id": "txn_001",
      "transaction_hash": "abc123...",
      "date": "2024-03-10T11:30:00Z",
      "type": "investment",
      "description": "Investment in Forest Bond Series A",
      "bond_name": "Forest Bond Series A",
      "bond_contract": "1234,0",
      "isin": "US12345678901",
      "amount": "5000.00", // EUR
      "token_amount": "5.00", // Tokens received/transferred
      "status": "completed",
      "details": {
        "token_id": "1640995200",
        "token_price": "1000.00", // EUR per token
        "transaction_fee": "25.00"
      }
    },
    {
      "transaction_id": "txn_002",
      "transaction_hash": "def456...",
      "date": "2024-03-01T10:00:00Z",
      "type": "yield_payment",
      "description": "Quarterly yield payment",
      "bond_name": "Forest Bond Series A",
      "bond_contract": "1234,0",
      "isin": "US12345678901",
      "amount": "416.67", // EUR
      "token_amount": null, // N/A for yield payments
      "status": "completed",
      "details": {
        "payment_sequence": 2,
        "tokens_held_at_payment": "10.00",
        "yield_rate": "8.5",
        "payment_period": "Q1 2024"
      }
    },
    {
      "transaction_id": "txn_003",
      "transaction_hash": "ghi789...",
      "date": "2024-02-28T14:00:00Z",
      "type": "carbon_burn",
      "description": "Carbon credit burn for offset",
      "bond_name": "Forest Bond Series A",
      "bond_contract": "1234,0",
      "isin": null, // Carbon credits don't have ISIN
      "amount": "5.0", // Tons CO2
      "token_amount": null,
      "status": "completed",
      "details": {
        "carbon_credit_contract": "2000,0",
        "burn_reason": "personal_offset",
        "certification": "VCS_verified"
      }
    }
  ],
  "page": 0,
  "page_count": 5,
  "total_count": 47,
  "summary": {
    "total_investments": "25000.00",
    "total_yields_received": "3125.00",
    "total_carbon_burned": "25.0"
  }
}
```

#### Download Transaction Confirmation PDF

```http path=null start=null
GET /portfolio/transactions/{transaction_id}/pdf
Authorization: Bearer <user_jwt_token>
```

**Purpose**: Generate and download PDF confirmation for specific transaction including ISIN and all regulatory details

**Authentication**: Valid user JWT token required

**Path Parameters**:

- `transaction_id`: Unique transaction identifier

**Response**: PDF file download

- Content-Type: application/pdf
- Content-Disposition: attachment; filename="transaction_confirmation_{transaction_id}.pdf"

**PDF Content**:

```text path=null start=null
UPWOOD FOREST INVESTMENTS
Transaction Confirmation

Transaction ID: txn_001
Date: March 10, 2024
Transaction Hash: abc123def456...

INVESTOR DETAILS
Name: John Doe
Email: investor@example.com
Account Address: 4owvMHZAXXX...

INVESTMENT DETAILS
Bond Name: Forest Bond Series A
ISIN: US12345678901
Contract Address: 1234,0
Forest Project: Amazon Rainforest Conservation
Location: Brazil, Para State

TRANSACTION SUMMARY
Type: Bond Investment
Amount Invested: €5,000.00
Tokens Purchased: 5.00
Token Price: €1,000.00 per token
Transaction Fee: €25.00
Token ID: 1640995200
Maturity Date: February 15, 2025
Yield Rate: 8.5% per annum

CARBON CREDITS
Allocated Credits: 25.125 tons CO2
Credit Rate: 5.025 tons per token

REGULATORY INFORMATION
This investment complies with EU regulatory requirements.
KYC Status: Verified
Identity Registry Status: Whitelisted

---
Generated on: March 10, 2024 11:30:00 UTC
Document Hash: pdf_hash_123...
```

#### Export User Transaction History CSV

```http path=null start=null
GET /portfolio/transactions/export
Authorization: Bearer <user_jwt_token>
Query Parameters:
  - format: "csv" (default)
  - date_from (optional): Start date filter (ISO 8601)
  - date_to (optional): End date filter (ISO 8601)
  - transaction_type (optional): Filter by transaction type
```

**Purpose**: Export complete transaction history as CSV for investor's records

**Authentication**: Valid user JWT token required

**Response**: CSV file download

- Content-Type: text/csv
- Content-Disposition: attachment; filename="transaction_history_YYYY-MM-DD.csv"

**CSV Columns**:

```csv path=null start=null
transaction_id,date,type,bond_name,isin,amount_eur,token_amount,transaction_hash,status,description
txn_001,2024-03-10T11:30:00Z,investment,"Forest Bond Series A",US12345678901,5000.00,5.00,abc123...,completed,"Investment in Forest Bond Series A"
txn_002,2024-03-01T10:00:00Z,yield_payment,"Forest Bond Series A",US12345678901,416.67,,def456...,completed,"Quarterly yield payment"
```

## Database Layer

### Enhanced Schema Extensions

```sql path=null start=null
-- Extend existing bonds table to include ISIN numbers
ALTER TABLE bonds ADD COLUMN isin VARCHAR(12); -- Standard ISIN format
CREATE INDEX idx_bonds_isin ON bonds(isin);

-- Add user transaction history table
CREATE TABLE user_transactions (
    id BIGSERIAL PRIMARY KEY,
    transaction_id VARCHAR(50) UNIQUE NOT NULL,
    user_address TEXT NOT NULL,
    transaction_hash TEXT,
    transaction_type VARCHAR(20) NOT NULL, -- 'investment', 'yield_payment', 'carbon_burn', 'transfer'
    bond_contract TEXT,
    token_id TEXT,
    amount_eur DECIMAL(20, 2),
    token_amount DECIMAL(78, 0),
    status VARCHAR(20) DEFAULT 'pending', -- 'pending', 'completed', 'failed'
    description TEXT,
    metadata JSONB, -- Additional transaction-specific data
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    completed_at TIMESTAMP
);

CREATE INDEX idx_user_transactions_address ON user_transactions(user_address);
CREATE INDEX idx_user_transactions_type ON user_transactions(transaction_type);
CREATE INDEX idx_user_transactions_bond ON user_transactions(bond_contract);
CREATE INDEX idx_user_transactions_date ON user_transactions(created_at);

-- Add carbon credit allocations tracking
CREATE TABLE carbon_credit_allocations (
    id BIGSERIAL PRIMARY KEY,
    bond_contract TEXT NOT NULL,
    user_address TEXT NOT NULL,
    allocated_credits DECIMAL(20, 2) NOT NULL,
    credits_burned DECIMAL(20, 2) DEFAULT 0,
    last_calculated TIMESTAMP NOT NULL DEFAULT now(),
    UNIQUE(bond_contract, user_address)
);

CREATE INDEX idx_carbon_allocations_bond ON carbon_credit_allocations(bond_contract);
CREATE INDEX idx_carbon_allocations_user ON carbon_credit_allocations(user_address);

-- Add PDF generation tracking
CREATE TABLE transaction_pdfs (
    id BIGSERIAL PRIMARY KEY,
    transaction_id VARCHAR(50) NOT NULL,
    user_address TEXT NOT NULL,
    pdf_hash VARCHAR(64), -- SHA-256 hash of generated PDF
    generated_at TIMESTAMP NOT NULL DEFAULT now(),
    download_count INTEGER DEFAULT 0,
    last_downloaded TIMESTAMP,
    FOREIGN KEY (transaction_id) REFERENCES user_transactions(transaction_id)
);

CREATE INDEX idx_transaction_pdfs_transaction ON transaction_pdfs(transaction_id);

-- Extend existing compliance_action_log table (from original file)
CREATE TABLE IF NOT EXISTS compliance_action_log (
    id BIGSERIAL PRIMARY KEY,
    account_address TEXT NOT NULL,
    action_type TEXT NOT NULL, -- 'whitelist_requested', 'blacklist_requested', 'freeze_requested', 'unfreeze_requested'
    contract_address TEXT,     -- Contract involved in action
    token_id TEXT,            -- For specific token operations
    amount DECIMAL(78, 0),    -- For freeze/unfreeze amounts
    operator_email TEXT NOT NULL,
    transaction_hash TEXT,    -- Added after blockchain transaction completes
    status TEXT NOT NULL DEFAULT 'pending', -- 'pending', 'completed', 'failed'
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    completed_at TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_compliance_action_log_address ON compliance_action_log(account_address);
CREATE INDEX IF NOT EXISTS idx_compliance_action_log_type ON compliance_action_log(action_type);
CREATE INDEX IF NOT EXISTS idx_compliance_action_log_status ON compliance_action_log(status);
CREATE INDEX IF NOT EXISTS idx_compliance_action_log_created_at ON compliance_action_log(created_at);
```

### Diesel Models

```rust path=null start=null
// Extended from original InvestorSummary
#[derive(Queryable, Serialize, Deserialize, Clone, Debug)]
pub struct InvestorSummary {
    pub account_address: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub nationality: String,
    pub identity_status: String,        // From identity registry processor
    pub kyc_verified: bool,
    pub total_investment_value: rust_decimal::Decimal,
    pub active_bond_count: i32,
    pub total_yield_received: rust_decimal::Decimal, // Added field
    pub created_at: chrono::NaiveDateTime,
    pub last_activity: Option<chrono::NaiveDateTime>,
}

// Enhanced bond holding with ISIN
#[derive(Queryable, Serialize, Deserialize, Clone, Debug)]
pub struct InvestorBondHolding {
    pub postsale_token_contract: String,
    pub bond_name: String,
    pub token_id: String,
    pub total_amount: rust_decimal::Decimal,
    pub available_amount: rust_decimal::Decimal,
    pub frozen_amount: rust_decimal::Decimal,
    pub investment_value: rust_decimal::Decimal,
    pub investment_date: chrono::NaiveDateTime,
    pub maturity_date: chrono::NaiveDateTime,
    pub yield_rate: rust_decimal::Decimal,
    pub next_yield_date: Option<chrono::NaiveDateTime>,
    pub isin: Option<String>, // Added ISIN field
    pub agents: Vec<String>, // Agent addresses for this contract
}

// New user transaction model
#[derive(Queryable, Serialize, Deserialize, Clone, Debug)]
pub struct UserTransaction {
    pub transaction_id: String,
    pub transaction_hash: Option<String>,
    pub transaction_type: String,
    pub bond_contract: Option<String>,
    pub token_id: Option<String>,
    pub amount_eur: Option<rust_decimal::Decimal>,
    pub token_amount: Option<rust_decimal::Decimal>,
    pub status: String,
    pub description: String,
    pub metadata: Option<serde_json::Value>,
    pub created_at: chrono::NaiveDateTime,
    pub completed_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = user_transactions)]
pub struct NewUserTransaction {
    pub transaction_id: String,
    pub user_address: String,
    pub transaction_type: String,
    pub bond_contract: Option<String>,
    pub token_id: Option<String>,
    pub amount_eur: Option<rust_decimal::Decimal>,
    pub token_amount: Option<rust_decimal::Decimal>,
    pub description: String,
    pub metadata: Option<serde_json::Value>,
}

// Carbon credit allocation model
#[derive(Queryable, Serialize, Deserialize, Clone, Debug)]
pub struct CarbonCreditAllocation {
    pub bond_contract: String,
    pub user_address: String,
    pub allocated_credits: rust_decimal::Decimal,
    pub credits_burned: rust_decimal::Decimal,
    pub last_calculated: chrono::NaiveDateTime,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = carbon_credit_allocations)]
pub struct CarbonCreditAllocationUpdate {
    pub bond_contract: String,
    pub user_address: String,
    pub allocated_credits: rust_decimal::Decimal,
    pub credits_burned: rust_decimal::Decimal,
}

// Portfolio dashboard response models
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserPortfolioDashboard {
    pub account_address: String,
    pub portfolio_summary: PortfolioSummary,
    pub bond_investments: Vec<BondInvestmentSummary>,
    pub yield_history: YieldHistorySummary,
    pub carbon_credits_summary: CarbonCreditsSummary,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PortfolioSummary {
    pub total_investment_value: rust_decimal::Decimal,
    pub current_portfolio_value: rust_decimal::Decimal,
    pub total_roi: rust_decimal::Decimal,
    pub total_yield_received: rust_decimal::Decimal,
    pub active_bonds_count: i32,
    pub matured_bonds_count: i32,
    pub total_carbon_credits: rust_decimal::Decimal,
    pub carbon_credits_burned: rust_decimal::Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BondInvestmentSummary {
    pub postsale_token_contract: String,
    pub bond_name: String,
    pub isin: Option<String>,
    pub forest_project: ForestProjectInfo,
    pub investment_details: InvestmentDetails,
    pub carbon_credits: CarbonCreditInfo,
}
```

### Database Operations

```rust path=null start=null
impl UserTransaction {
    pub fn list_by_user(
        conn: &mut PgConnection,
        user_address: &str,
        page: i64,
        page_size: i64,
        transaction_type: Option<&str>,
        bond_contract: Option<&str>,
        date_from: Option<chrono::NaiveDateTime>,
        date_to: Option<chrono::NaiveDateTime>
    ) -> QueryResult<(Vec<Self>, i64)> {
        // Query user_transactions with filtering and pagination
        // Join with bonds table for ISIN and bond names
        // Apply date range and type filters
        // Return paginated results with total count
    }
    
    pub fn create_investment_transaction(
        conn: &mut PgConnection,
        user_address: &str,
        bond_contract: &str,
        token_id: &str,
        amount_eur: rust_decimal::Decimal,
        token_amount: rust_decimal::Decimal
    ) -> QueryResult<Self> {
        // Create new investment transaction record
        // Generate unique transaction_id
        // Set status to pending initially
    }
    
    pub fn create_yield_transaction(
        conn: &mut PgConnection,
        user_address: &str,
        bond_contract: &str,
        amount_eur: rust_decimal::Decimal,
        payment_sequence: i32
    ) -> QueryResult<Self> {
        // Create yield payment transaction record
        // Include payment metadata (sequence, period, etc.)
    }
}

impl CarbonCreditAllocation {
    pub fn calculate_user_allocation(
        conn: &mut PgConnection,
        user_address: &str,
        bond_contract: &str
    ) -> QueryResult<rust_decimal::Decimal> {
        // Calculate carbon credit allocation based on bond ownership percentage
        // Query bond_token_holders for user's token balance
        // Query carbon_credit_holders for bond's total carbon credits
        // Return allocated amount = (user_tokens / total_tokens) * bond_carbon_credits
    }
    
    pub fn update_allocation(
        conn: &mut PgConnection,
        user_address: &str,
        bond_contract: &str,
        allocated_credits: rust_decimal::Decimal
    ) -> QueryResult<Self> {
        // Upsert carbon credit allocation record
        // Update last_calculated timestamp
    }
    
    pub fn get_user_carbon_summary(
        conn: &mut PgConnection,
        user_address: &str
    ) -> QueryResult<Vec<Self>> {
        // Get all carbon credit allocations for user across all bonds
        // Include burned amounts from transaction history
    }
}

impl UserPortfolioDashboard {
    pub fn build_dashboard(
        conn: &mut PgConnection,
        user_address: &str
    ) -> QueryResult<Self> {
        // Aggregate all portfolio data:
        // 1. Bond holdings with current values
        // 2. Yield payment history and totals
        // 3. Carbon credit allocations
        // 4. ROI calculations
        // 5. Upcoming payment estimates
        
        // Join multiple tables:
        // - bond_token_holders (current holdings)
        // - investor_yield_history (yield payments)
        // - carbon_credit_allocations (carbon credits)
        // - bonds (bond metadata, ISIN)
        // - forest_projects (project info)
    }
}
```

## PDF Generation Service

### PDF Service Integration

```rust path=null start=null
use pdf_create::PDFDocument;
use chrono::{DateTime, Utc};

pub struct TransactionPDFService;

impl TransactionPDFService {
    pub fn generate_confirmation_pdf(
        transaction: &UserTransaction,
        user_details: &UserProfile,
        bond_details: Option<&BondDetail>
    ) -> Result<Vec<u8>, PDFError> {
        let mut doc = PDFDocument::new();
        
        // Header with logo and company info
        doc.add_header("UPWOOD FOREST INVESTMENTS");
        doc.add_subheader("Transaction Confirmation");
        
        // Transaction details section
        doc.add_section("Transaction Details", vec![
            ("Transaction ID", &transaction.transaction_id),
            ("Date", &transaction.created_at.to_string()),
            ("Transaction Hash", transaction.transaction_hash.as_deref().unwrap_or("Pending")),
        ]);
        
        // Investor details section
        doc.add_section("Investor Details", vec![
            ("Name", &format!("{} {}", user_details.first_name, user_details.last_name)),
            ("Email", &user_details.email),
            ("Account Address", &user_details.account_address),
        ]);
        
        // Investment details (if bond transaction)
        if let Some(bond) = bond_details {
            doc.add_section("Investment Details", vec![
                ("Bond Name", &bond.bond_name),
                ("ISIN", bond.isin.as_deref().unwrap_or("N/A")),
                ("Contract Address", &bond.contract_address),
                ("Forest Project", &bond.forest_project.name),
                ("Location", &bond.forest_project.location),
            ]);
            
            if transaction.transaction_type == "investment" {
                doc.add_section("Transaction Summary", vec![
                    ("Type", "Bond Investment"),
                    ("Amount Invested", &format!("€{:.2}", transaction.amount_eur.unwrap_or_default())),
                    ("Tokens Purchased", &format!("{:.2}", transaction.token_amount.unwrap_or_default())),
                    ("Maturity Date", &bond.maturity_date.to_string()),
                    ("Yield Rate", &format!("{:.1}% per annum", bond.yield_rate)),
                ]);
            }
        }
        
        // Carbon credits section (if applicable)
        if let Some(metadata) = &transaction.metadata {
            if let Ok(carbon_info) = serde_json::from_value::<CarbonCreditInfo>(metadata.clone()) {
                doc.add_section("Carbon Credits", vec![
                    ("Allocated Credits", &format!("{:.3} tons CO2", carbon_info.allocated_credits)),
                    ("Credit Rate", &format!("{:.3} tons per token", carbon_info.credit_rate)),
                ]);
            }
        }
        
        // Regulatory information
        doc.add_section("Regulatory Information", vec![
            ("Compliance Status", "EU Regulatory Compliant"),
            ("KYC Status", "Verified"),
            ("Identity Registry", "Whitelisted"),
        ]);
        
        // Footer with generation info
        let generation_time = Utc::now();
        doc.add_footer(&format!(
            "Generated on: {}\nDocument Hash: {}",
            generation_time.format("%B %d, %Y %H:%M:%S UTC"),
            calculate_pdf_hash(&doc.content())
        ));
        
        doc.build()
    }
    
    pub fn store_pdf_record(
        conn: &mut PgConnection,
        transaction_id: &str,
        user_address: &str,
        pdf_hash: &str
    ) -> QueryResult<()> {
        use crate::schema::transaction_pdfs;
        
        diesel::insert_into(transaction_pdfs::table)
            .values((
                transaction_pdfs::transaction_id.eq(transaction_id),
                transaction_pdfs::user_address.eq(user_address),
                transaction_pdfs::pdf_hash.eq(pdf_hash),
            ))
            .execute(conn)?;
        
        Ok(())
    }
}

fn calculate_pdf_hash(content: &[u8]) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(content);
    format!("{:x}", hasher.finalize())
}
```

## Authentication & Authorization

### Enhanced Role-Based Access Control

```rust path=null start=null
// User portfolio access
pub fn ensure_user_access(claims: &BearerAuthorization, requested_address: &str) -> Result<(), Error> {
    // Users can only access their own portfolio data
    if claims.user_address != requested_address {
        return Err(Error::UnAuthorized(PlainText(
            "Access denied: can only access own portfolio data".to_string()
        )));
    }
    Ok(())
}

// Admin investor management access
pub fn ensure_compliance_officer_or_admin(claims: &BearerAuthorization) -> Result<(), Error> {
    if !claims.groups.contains(&"admin".to_string()) 
        && !claims.groups.contains(&"compliance_officer".to_string()) {
        return Err(Error::UnAuthorized(PlainText(
            "Compliance officer or admin role required".to_string()
        )));
    }
    Ok(())
}

// PDF generation access control
pub fn ensure_pdf_access(claims: &BearerAuthorization, transaction: &UserTransaction) -> Result<(), Error> {
    // Users can only generate PDFs for their own transactions
    if claims.user_address != transaction.user_address {
        return Err(Error::UnAuthorized(PlainText(
            "Access denied: can only generate PDFs for own transactions".to_string()
        )));
    }
    Ok(())
}
```

## Integration Points

### Frontend Integration

**User Portfolio Dashboard**:

- `/portfolio/dashboard` provides comprehensive overview for main portfolio page
- Real-time ROI calculations and carbon credit allocations
- Interactive bond performance charts with yield projections

**Transaction Management**:

- `/portfolio/transactions` with filtering for transaction history table
- PDF download buttons for each transaction
- CSV export for complete transaction history

**Admin Investor Management**:

- Existing admin endpoints enhanced with yield and carbon credit data
- Unchanged freeze/unfreeze blockchain transaction flows

### Blockchain Integration

**Carbon Credit Allocation**:

- Periodic jobs to recalculate carbon credit allocations
- Query carbon credit contract for bond holdings
- Update allocations based on ownership percentages

**Transaction Recording**:

- Blockchain event processors create `user_transactions` records
- Link transaction hashes to user-facing transaction IDs
- Automatic PDF generation triggers for completed investments

### Yield Integration

**ROI Calculations**:

- Join with `investor_yield_history` for received payments
- Calculate current portfolio value including accrued yields
- Provide annualized ROI calculations

## Error Handling

### API Error Patterns

```rust path=null start=null
// Portfolio access errors
Error::UnAuthorized(PlainText("Access denied: can only access own portfolio data"))

// PDF generation errors
Error::BadRequest(PlainText("Transaction not found or access denied"))
Error::InternalServerError(PlainText("PDF generation failed"))

// Transaction history errors
Error::BadRequest(PlainText("Invalid date range specified"))
Error::BadRequest(PlainText("Invalid transaction type filter"))

// Carbon credit calculation errors
Error::InternalServerError(PlainText("Carbon credit allocation calculation failed"))
```

### Data Consistency

- Transaction records maintain referential integrity with bond contracts
- Carbon credit allocations recalculated on bond balance changes
- PDF generation includes hash verification for document integrity
- Periodic data validation jobs to ensure portfolio calculations accuracy

## Security Considerations

### Data Access Security

- User portfolio data access restricted to account owner only
- Admin investor data requires compliance officer role
- Transaction PDF downloads include access logging
- Rate limiting on portfolio and transaction endpoints

### PDF Security

- Generated PDFs include cryptographic hash for verification
- PDF content includes blockchain transaction hashes for audit
- Download tracking with timestamp logging
- Secure PDF storage with user access controls

### Compliance & Audit

- Complete audit trail for all investor data access
- Transaction history maintains immutable blockchain references
- PDF generation events logged for regulatory compliance
- Regular compliance reporting capabilities for regulatory authorities
