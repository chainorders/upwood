# Blockchain Transaction Processing and Contract Data Backend (FR-AD-5)

This document defines the backend API for:

- Viewing and exporting blockchain transactions processed by the system
- Read-only access to contract data (identity registry, token contracts, carbon credits)
- All endpoints use data from existing event processors and processor-managed database tables

## API Layer

### Transaction Viewing and Export Endpoints

#### List All Processed Transactions

```http path=null start=null
GET /admin/transactions
Authorization: Bearer <admin_jwt_token>
Query Parameters:
  - page (optional): Page number (default: 0)
  - page_size (optional): Results per page (default: 50, max: 200)
  - start_date (optional): ISO datetime filter
  - end_date (optional): ISO datetime filter
  - transaction_type (optional): "all" | "token_transfer" | "bond_purchase" | "maturity_payment" | "carbon_credit" | "identity_registry"
  - contract_address (optional): Filter by specific contract
  - status (optional): "success" | "failed" | "all"
```

**Purpose**: Display all blockchain transactions processed by the system for admin monitoring

**Authentication**: Admin role required

**Response**: `PagedResponse<ProcessedTransaction>`

```typescript path=null start=null
{
  "data": [
    {
      "transaction_hash": "0xabc123...",
      "block_height": 123456,
      "transaction_date": "2024-03-10T14:30:00Z",
      "transaction_type": "token_transfer",
      "contract_address": "1000,0",
      "contract_name": "Forest Bond Series A",
      "from_address": "4owvMHZAXXX...",
      "to_address": "4owvMHYAXXX...",
      "token_id": "0",
      "amount": "1000.00",
      "status": "success",
      "gas_used": 1250,
      "processing_date": "2024-03-10T14:31:15Z",
      "processor_name": "security_sft_multi_processor"
    }
  ],
  "total_count": 15420,
  "page": 0,
  "page_count": 309
}
```

#### Export Blockchain Transactions CSV

```http path=null start=null
GET /admin/transactions/export
Authorization: Bearer <admin_jwt_token>
Query Parameters:
  - format: "csv" (default)
  - start_date: ISO datetime (optional)
  - end_date: ISO datetime (optional)
  - transaction_type: "all" | "token_transfer" | "bond_purchase" | "maturity_payment" | "carbon_credit" | "identity_registry" (optional)
  - contract_address: string (optional - filter by specific contract)
```

**Purpose**: Download CSV file with all blockchain transactions processed by the system

**Authentication**: Admin role required

**Response**: CSV file download with headers:

- Content-Type: text/csv
- Content-Disposition: attachment; filename="transactions_export_YYYY-MM-DD.csv"

**CSV Columns**:

```csv path=null start=null
transaction_hash,block_height,transaction_date,transaction_type,contract_address,contract_name,from_address,to_address,token_id,amount,status,gas_used,processing_date,processor_name
0xabc123...,123456,2024-03-10T14:30:00Z,token_transfer,1000,Forest Bond Series A,4owvMHZAXXX...,4owvMHYAXXX...,0,1000.00,success,1250,2024-03-10T14:31:15Z,security_sft_multi_processor
```

#### Get Export Status

```http path=null start=null
GET /admin/transactions/export/status/{export_id}
Authorization: Bearer <admin_jwt_token>
```

**Purpose**: Check status of large transaction export jobs (for async processing)

**Authentication**: Admin role required

**Response**:

```typescript path=null start=null
{
  "export_id": "exp-12345",
  "status": "completed", // "pending" | "processing" | "completed" | "failed"
  "export_type": "transactions",
  "created_at": "2024-03-10T15:00:00Z",
  "completed_at": "2024-03-10T15:02:30Z",
  "record_count": 15420,
  "download_url": "/admin/transactions/export/download/exp-12345",
  "expires_at": "2024-03-11T15:02:30Z"
}
```

#### Download Transaction Export File

```http path=null start=null
GET /admin/transactions/export/download/{export_id}
Authorization: Bearer <admin_jwt_token>
```

**Purpose**: Download completed transaction export file

**Authentication**: Admin role required

**Response**: File download (CSV format)

### Contract Information Endpoints

#### Get Identity Registry Contract Info

```http path=null start=null
GET /admin/contracts/identity-registry
Authorization: Bearer <admin_jwt_token>
```

**Purpose**: Provide contract information for frontend to make blockchain calls

**Authentication**: Admin or Compliance Officer role required

**Response**: `IdentityRegistryContract`

```typescript path=null start=null
{
  "contract_address": "1000,0",
  "contract_name": "rwa_identity_registry",
  "network": "testnet" // or "mainnet"
}
```

#### Get Token Contract Details

```http path=null start=null
GET /admin/contracts/token-contract/{token_contract}
Authorization: Bearer <admin_jwt_token>
```

**Purpose**: Get comprehensive information about a specific token contract including tokens, agents, and metadata

**Authentication**: Admin role required

**Path Parameters**:

- `token_contract`: Contract address (e.g., "1234,0")

**Response**: `TokenContractDetails`

```typescript path=null start=null
{
  "contract_address": "1234,0",
  "contract_name": "Forest Bond Series A",
  "network": "testnet", // or "mainnet"
  "token_count": 12,
  "created_at": "2024-01-15T10:00:00Z",
  "last_activity": "2024-03-10T14:30:00Z",
  
  "tokens": [
    {
      "token_id": "1640995200", // Subscription period end timestamp
      "supply": "50000000000000", // Total supply in base units
      "holders_count": 25,
      "is_paused": false,
      "metadata_url": "https://ipfs.io/ipfs/QmXXX...",
      "created_at": "2024-02-01T10:00:00Z"
    },
    {
      "token_id": "1672531200",
      "supply": "30000000000000",
      "holders_count": 18,
      "is_paused": false,
      "metadata_url": "https://ipfs.io/ipfs/QmYYY...",
      "created_at": "2024-03-01T10:00:00Z"
    }
  ],
  
  "agents": [
    {
      "address": "4owvMHZAXXX...",
      "roles": ["Operator", "Pause"],
      "added_at": "2024-01-15T10:30:00Z"
    },
    {
      "address": "4owvMHZBBBB...",
      "roles": ["Freeze", "UnFreeze"],
      "added_at": "2024-02-01T09:15:00Z"
    }
  ]
}
```

#### Get Carbon Credit Contract Overview

```http path=null start=null
GET /admin/contracts/carbon-credits
Authorization: Bearer <admin_jwt_token>
```

**Purpose**: Display carbon credit system overview (single contract/token from system config)

**Authentication**: Admin role required

**Response**: `CarbonCreditOverview`

```typescript path=null start=null
{
  "contract_address": "2000,0",
  "contract_name": "rwa_carbon_credits",
  "network": "testnet", // or "mainnet"
  "token_id": "0", // Always 0 for security-sft-single
  "total_supply": "10000.00", // Total tons CO2
  "total_burned": "2500.00",
  "available_supply": "7500.00",
  "metadata_url": "https://ipfs.io/ipfs/QmXXX...",
  "holders_count": 45,
  "bond_holders_count": 8, // How many bonds hold carbon credits
  "created_at": "2024-01-15T10:00:00Z",
  "last_activity": "2024-03-10T14:30:00Z",
  "recent_activity_count": {
    "transfers_last_30_days": 12,
    "burns_last_30_days": 3
  }
}
```

#### List Carbon Credit Holders

```http path=null start=null
GET /admin/contracts/carbon-credits/holders
Authorization: Bearer <admin_jwt_token>
Query Parameters:
  - page (optional): Page number (default: 0)
  - page_size (optional): Results per page (default: 50, max: 200)
  - holder_type (optional): "account" | "contract" | "all"
  - min_balance (optional): Minimum balance filter
  - sort_by (optional): "balance" | "address" | "last_activity"
  - sort_order (optional): "asc" | "desc"
```

**Purpose**: List all holders of the carbon credit token

**Authentication**: Admin role required

**Response**: `PagedResponse<CarbonCreditHolder>`

```typescript path=null start=null
{
  "data": [
    {
      "holder_address": "4owvMHZAXXX...",
      "balance": "1500.00",
      "frozen_balance": "0.00", // Individual accounts can have frozen carbon credits
      "holder_type": "account",
      "bond_name": null, // null for account addresses
      "first_received": "2024-02-01T10:00:00Z",
      "last_activity": "2024-03-01T15:30:00Z",
      "transaction_count": 8
    },
    {
      "holder_address": "1234,0",
      "balance": "2000.00",
      "frozen_balance": "0.00", // Bonds cannot have frozen carbon credits
      "holder_type": "contract",
      "bond_name": "Forest Bond Series A", // From bonds table
      "first_received": "2024-01-20T12:00:00Z",
      "last_activity": "2024-02-28T09:15:00Z",
      "transaction_count": 3
    }
  ],
  "total_count": 45,
  "page": 0,
  "page_count": 1
}
```

#### Get Identity Registry Address Status

```http path=null start=null
GET /admin/contracts/identity-registry/addresses/{address}
Authorization: Bearer <admin_jwt_token>
```

**Purpose**: Check specific address status in the identity registry (whitelisted, blacklisted, or not in registry)

**Authentication**: Admin or Compliance Officer role required

**Path Parameters**:

- `address`: Concordium address to check

**Response**: `AddressStatusResponse`

```typescript path=null start=null
{
  "address": "4UC8o4m8AgTxt5VBFMdLwMJwHhVmr5CqzXMTfUP8PU5t3oN6vT",
  "status": "whitelisted", // "whitelisted" | "blacklisted" | "not_in_registry"
  "state_changed_at": "2024-03-15T10:30:00Z", // null for not_in_registry
  "contract_address": "1000,0"
}
```

#### List Whitelisted Addresses

```http path=null start=null
GET /admin/contracts/identity-registry/whitelisted
Authorization: Bearer <admin_jwt_token>
Query Parameters:
  - page (optional): Page number (default: 0)
  - page_size (optional): Results per page (default: 50, max: 200)
```

**Purpose**: List all whitelisted addresses from the identity registry

**Authentication**: Admin or Compliance Officer role required

**Response**: `PagedResponse<WhitelistedAddress>`

```typescript path=null start=null
{
  "data": [
    {
      "address": "4UC8o4m8AgTxt5VBFMdLwMJwHhVmr5CqzXMTfUP8PU5t3oN6vT",
      "whitelisted_at": "2024-03-15T10:30:00Z",
      "contract_address": "1000,0"
    }
  ],
  "total_count": 89,
  "page": 0,
  "page_count": 2
}
```

#### List Blacklisted Addresses

```http path=null start=null
GET /admin/contracts/identity-registry/blacklisted
Authorization: Bearer <admin_jwt_token>
Query Parameters:
  - page (optional): Page number (default: 0)
  - page_size (optional): Results per page (default: 50, max: 200)
```

**Purpose**: List all blacklisted addresses from the identity registry

**Authentication**: Admin or Compliance Officer role required

**Response**: `PagedResponse<BlacklistedAddress>`

```typescript path=null start=null
{
  "data": [
    {
      "address": "4UC8o4m8AgTxt5VBFMdLwMJwHhVmr5CqzXMTfUP8PU5t3oN6vT",
      "blacklisted_at": "2024-03-15T10:30:00Z",
      "contract_address": "1000,0"
    }
  ],
  "total_count": 123,
  "page": 0,
  "page_count": 3
}
```

#### Check Address Blacklist Status

```http path=null start=null
GET /admin/contracts/identity-registry/blacklisted/{address}
Authorization: Bearer <admin_jwt_token>
```

**Purpose**: Check if a specific address is blacklisted (quick check endpoint)

**Authentication**: Admin or Compliance Officer role required

**Path Parameters**:

- `address`: Concordium address to check

**Response**: `BlacklistCheckResponse`

```typescript path=null start=null
{
  "address": "4UC8o4m8AgTxt5VBFMdLwMJwHhVmr5CqzXMTfUP8PU5t3oN6vT",
  "is_blacklisted": true,
  "blacklisted_at": "2024-03-15T10:30:00Z", // null if not blacklisted
  "contract_address": "1000,0"
}
```

## Data Sources and Processing

### Event Processor Data Sources

**Existing Transaction Tables** (confirmed from processor code review):

- `listener_transactions` - All processed transaction metadata (block height, hash, timestamp, index)
- `listener_contract_calls` - All contract calls with entrypoint, amounts, sender/instigator info
- `cis2_token_holder_balance_updates` - All CIS2 token transfer, mint, burn events with amounts and addresses
- `security_mint_fund_investment_records` - Bond investment transaction records
- `security_p2p_exchange_records` - P2P trading transaction records
- `offchain_reward_claims` - Reward claim transaction records
- Various processor-specific event tables with transaction metadata

**Transaction Types**:

- **token_transfer**: CIS2 Transfer events from all contracts
- **bond_purchase**: Bond token minting events
- **maturity_payment**: Yield and maturity payment processing
- **carbon_credit**: Carbon credit minting, burning, transfers
- **identity_registry**: Address whitelist/blacklist changes

### Data Aggregation Requirements

**Processor Requirements**:

- Each processor should save processed transactions to a consolidated table
- Include transaction metadata (hash, block height, gas used)
- Store contract information and human-readable names
- Track processing status and timestamps
- Link to specific event records for detailed analysis

## Backend Implementation Notes

### Transaction Export Service

```typescript path=null start=null
interface TransactionExportService {
  // List transactions with filters
  list_transactions(filters: TransactionFilters, page: i64, page_size: i64) -> PagedResponse<ProcessedTransaction>;
  
  // Synchronous export for small datasets
  export_transactions_sync(filters: TransactionFilters) -> CSV;
  
  // Asynchronous export for large datasets  
  export_transactions_async(filters: TransactionFilters) -> ExportJob;
  
  // Export job management
  get_export_status(export_id: string) -> ExportStatus;
  download_export_file(export_id: string) -> FileStream;
}

interface TransactionFilters {
  start_date?: DateTime;
  end_date?: DateTime;
  transaction_type?: TransactionType;
  contract_address?: string;
  status?: TransactionStatus;
}
```

### Database Operations

**Transaction Data Access**:

- Query consolidated transaction table if exists
- Otherwise aggregate across processor event tables
- Apply date range filters on transaction_date
- Filter by transaction type and contract address
- Join with contract metadata for readable names
- Apply pagination and sorting

**Export Processing**:

- Synchronous exports for < 50k records  
- Asynchronous background processing for larger exports
- Export file caching with 24-hour expiry
- Progress tracking for async jobs

### Performance Considerations

**Query Optimization**:

- Indexes on transaction_date across transaction tables
- Materialized views for frequently accessed aggregations
- Partitioning by date ranges for large transaction tables
- Connection pooling for export operations

**Export Strategy**:

- Streaming CSV generation for large exports
- Background job processing using existing task queue
- File compression for large exports
- Cleanup of expired export files

## Frontend Integration

### Admin Portal Transaction Management

**Blockchain Transactions Page**:

- Dedicated page listing all processed transactions
- Filter controls (date range, transaction type, contract)
- Search by transaction hash or addresses  
- Export button with progress tracking

**Transaction List Table**:

- Transaction hash (clickable for blockchain explorer)
- Date/time with timezone
- Transaction type with color coding
- Contract name and addresses
- Amount with proper formatting
- Processing status indicators

**Export Functionality**:

- Export options modal (filters, date ranges)
- Progress indicators for large exports
- Download status tracking
- Export history with re-download links

### UI Components

**Transaction Filters**:

- Date range picker components
- Transaction type dropdown
- Contract address autocomplete
- Status filter checkboxes

**Export Controls**:

- Export button with loading states
- Progress bars for async exports
- Export status cards
- Download ready notifications

**Transaction Details**:

- Expandable rows with full transaction data
- Links to blockchain explorer
- Contract interaction details
- Processing metadata display

This provides comprehensive blockchain transaction monitoring and export capabilities for administrative oversight and compliance reporting.
