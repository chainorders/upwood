# Bonds Planning (API)

This document defines the bonds API planning (backend/upwood/src/api).

## File Structure

- New file: backend/upwood/src/api/bonds.rs

## Endpoints

### GET /bonds (Investor)

List all bonds with forest project information and user ownership data for investment dashboard

Query parameters:

- `status` (string, optional) - Filter by bond status: Active, Paused, Matured, Success, Failed
- `forest_project_id` (string, optional) - Filter by specific forest project
- `user_balance_filter` (string, optional) - "owned_only" returns only bonds where user has balance > 0
- `limit` (number, optional, default: 20) - Number of results per page
- `offset` (number, optional, default: 0) - Pagination offset
- `sort_by` (string, optional, default: "created_at") - Sort field: "created_at", "maturity_date", "subscription_period_end", "bond_price"
- `sort_order` (string, optional, default: "desc") - Sort direction: "asc", "desc"

Headers:

- `Authorization: Bearer <jwt_token>` (required for user ownership data)

Output:

```json
{
  "bonds": [
    {
      "postsale_token_contract_address": "string",
      "presale_token_contract_address": "string",
      "status": "Active",
      "maturity_date": "2025-12-31T23:59:59Z",
      "subscription_period_end": "2024-06-30T23:59:59Z",
      "bond_price": "100.00",
      "maximum_supply": "10000.00",
      "current_supply": "7500.00",
      "minimum_raise_amount": "5000.00",
      "interest_rate_type": "Fixed",
      "lockup_period_duration": "P1Y",
      "created_at": "2024-01-15T10:00:00Z",
      "forest_project": {
        "id": "proj-123",
        "name": "Amazon Rainforest Conservation",
        "description": "Protecting 10,000 hectares of Amazon rainforest",
        "location": "Brazil, Para State",
        "area_hectares": 10000,
        "estimated_carbon_credits": 50000,
        "project_start_date": "2024-01-01T00:00:00Z",
        "project_end_date": "2034-12-31T23:59:59Z",
        "certification_body": "Verra"
      },
      "user_ownership": {
        "total_invested_plt": "500.00",
        "presale_balance": "0.00",
        "postsale_balance": "5.00",
        "tokens": [
          {
            "token_id": "18628",
            "balance": "3.00",
            "invested_plt_amount": "300.00"
          },
          {
            "token_id": "18650",
            "balance": "2.00",
            "invested_plt_amount": "200.00"
          }
        ]
      }
    }
  ],
  "total_count": 15,
  "page_info": {
    "has_next_page": true,
    "has_previous_page": false,
    "current_page": 1,
    "total_pages": 3
  }
}
```

### GET /bonds/{postsale_token_contract_address} (Investor)

Get detailed information for a single bond including forest project and user ownership data

Path parameters:

- `postsale_token_contract_address` (string, required) - Bond contract address

Headers:

- `Authorization: Bearer <jwt_token>` (required for user ownership data)

Output:

```json
{
  "bond": {
    "postsale_token_contract_address": "string",
    "presale_token_contract_address": "string",
    "status": "Active",
    "maturity_date": "2025-12-31T23:59:59Z",
    "subscription_period_end": "2024-06-30T23:59:59Z",
    "bond_price": "100.00",
    "maximum_supply": "10000.00",
    "current_supply": "7500.00",
    "minimum_raise_amount": "5000.00",
    "interest_rate_type": "Fixed",
    "lockup_period_duration": "P1Y",
    "created_at": "2024-01-15T10:00:00Z",
    "updated_at": "2024-01-20T14:30:00Z",
    "forest_project": {
      "id": "proj-123",
      "name": "Amazon Rainforest Conservation",
      "description": "Protecting 10,000 hectares of Amazon rainforest",
      "location": "Brazil, Para State",
      "area_hectares": 10000,
      "estimated_carbon_credits": 50000,
      "project_start_date": "2024-01-01T00:00:00Z",
      "project_end_date": "2034-12-31T23:59:59Z",
      "certification_body": "Verra"
    },
    "user_ownership": {
      "total_invested_plt": "500.00",
      "presale_balance": "0.00",
      "postsale_balance": "5.00",
      "tokens": [
        {
          "token_id": "18628",
          "balance": "3.00",
          "invested_plt_amount": "300.00",
          "mint_date": "2024-02-15T12:00:00Z"
        },
        {
          "token_id": "18650",
          "balance": "2.00",
          "invested_plt_amount": "200.00",
          "mint_date": "2024-03-10T09:30:00Z"
        }
      ]
    },
    "investment_summary": {
      "total_investors": 150,
      "funding_percentage": 75.0,
      "days_until_subscription_end": 45
    }
  }
}
```

## Admin/Operator Endpoints

### GET /admin/bonds (Admin)

Admin list bonds (filters: project, status) - includes investor summaries

Query parameters:

- project (string, optional)
- status (string, optional: Active, Paused, Matured, Success, Failed)
- limit (number, optional)
- offset (number, optional)

Output:

- bonds (array of bond summaries with investor data)
- total (number)

### GET /admin/bonds/{postsale_token_contract_address} (Admin)

Get detailed bond information for admin dashboard

Output:

- bond (bond details object)
- investors (array of investor summaries)

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

## Implementation Notes

### Data Sources and Integration

#### Bond Data

- Source: `bonds` table (managed by bond-blockchain.md events processor)
- Primary key: `postsale_token_contract_address`
- Includes: basic bond information, status, dates, supply limits

#### Forest Project Data  

- Source: `forest_projects` table (managed by forest-project-api.md)
- Joined via: bond-to-project mapping table
- Includes: project details, location, carbon credits, certification

#### User Ownership Data

- Source: `bond_investors` and `bond_investment_records` tables
- Requires: JWT token authentication to identify current user
- Aggregated data:
  - `total_invested_plt`: Sum of PLT amounts from investment records
  - `presale_balance`: Current presale token balance
  - `postsale_balance`: Current postsale token balance
  - `tokens[]`: Array of individual token holdings with amounts

#### Token ID to Date Conversion

- Token IDs represent days since Unix epoch (u64)
- Convert to `mint_date` using: `Unix Epoch + token_id days`
- Used for: detailed token information and yield calculations

### Query Implementation

#### Base Query Structure

```sql
SELECT 
  b.*,
  fp.*,
  COALESCE(bi.total_invested, 0) as total_invested_plt,
  COALESCE(bi.presale_balance, 0) as presale_balance,
  COALESCE(bi.postsale_balance, 0) as postsale_balance
FROM bonds b
LEFT JOIN forest_project_bonds fpb ON b.postsale_token_contract_address = fpb.bond_address
LEFT JOIN forest_projects fp ON fpb.forest_project_id = fp.id
LEFT JOIN bond_investors bi ON (b.postsale_token_contract_address = bi.bond_id AND bi.account_address = ?)
```

#### Token Details Subquery

```sql
SELECT 
  bir.token_id,
  SUM(bir.amount) as balance,
  SUM(CASE WHEN bir.record_type = 'invest' THEN bir.amount ELSE 0 END) as invested_plt_amount
FROM bond_investment_records bir
WHERE bir.bond_id = ? AND bir.investor_address = ?
GROUP BY bir.token_id
HAVING SUM(bir.amount) > 0
```

### Filtering and Sorting

#### Status Filter

- Apply WHERE clause on `bonds.status`
- Values: Active, Paused, Matured, Success, Failed

#### Forest Project Filter

- Apply WHERE clause on `forest_projects.id`
- Requires JOIN through mapping table

#### User Balance Filter ("owned_only")

- Apply WHERE clause: `bi.presale_balance > 0 OR bi.postsale_balance > 0`
- Only returns bonds where current user has token holdings

#### Sorting Options

- `created_at`: Bond creation date (default DESC)
- `maturity_date`: Bond maturity date
- `subscription_period_end`: Investment deadline
- `bond_price`: Price per token

### Pagination

- Standard LIMIT/OFFSET implementation
- Return `page_info` object with navigation metadata
- Calculate `total_pages` from `total_count` and `limit`

### Performance Considerations

#### Database Indexes

- `bonds.status` for status filtering
- `bonds.created_at` for default sorting
- `bond_investors.account_address` for user ownership lookups
- `bond_investment_records(bond_id, investor_address)` for token details

#### Caching Strategy

- Cache forest project data (rarely changes)
- Cache bond basic data with TTL (updates from blockchain events)
- Never cache user ownership data (real-time accuracy required)

## Security

### Authentication

- Investor endpoints: JWT Bearer token required for user ownership data
- Admin endpoints: Cognito admin role required
- Anonymous access: Bond and forest project data only (no ownership info)

### Authorization

- User ownership data filtered by authenticated account address
- Admin endpoints restricted to authorized roles
- Rate limiting applied to prevent abuse
