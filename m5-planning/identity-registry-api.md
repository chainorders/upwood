# Identity Registry API Planning for Address State Management (FR-BT-3 & FR-BT-5)

This document defines the API endpoints needed for the identity-registry address state management system to implement FR-BT-3 investor blacklist functionality and FR-BT-5 maturity handling whitelist requirements.

## File Structure

- New file: `backend/upwood/src/api/identity_registry.rs`
- Update: `backend/upwood/src/api/mod.rs` to include identity_registry module

## Background

FR-BT-3 & FR-BT-5 require: "Address state management for transfer restrictions and maturity payments"

- Admin agents can set address states (whitelist/blacklist) via blockchain transactions
- API provides read-only access to address status and lists
- All address state modifications happen on-chain via smart contract calls
- API serves as query interface for address state data

**Three Address States:**

- **Registered** (default): Address exists but no special status
- **Whitelisted**: Address can receive maturity payments (FR-BT-5)
- **Blacklisted**: Address cannot receive any payments (overrides whitelist)

## API Endpoints

### GET /identity-registry/addresses/{address}/status (Admin/Public)

**NEW** - Primary endpoint for checking address status in the three-state system

**Path Parameters:**

- `address` (string, required) - Wallet address to check

**Response:**

```json
{
  "address": "4UC8o4m8AgTxt5VBFMdLwMJwHhVmr5CqzXMTfUP8PU5t3oN6vT",
  "status": "whitelisted", // "registered" | "whitelisted" | "blacklisted"
  "state_changed_at": "2024-03-15T10:30:00Z", // null for registered
  "contract_address": "1000,0"
}
```

### GET /identity-registry/whitelisted (Admin)

List all whitelisted addresses from the identity-registry contract

**Query Parameters:**

- `limit` (number, optional, default: 50, max: 1000)
- `offset` (number, optional, default: 0)

**Response:**

```json
{
  "whitelisted_addresses": [
    {
      "address": "4UC8o4m8AgTxt5VBFMdLwMJwHhVmr5CqzXMTfUP8PU5t3oN6vT",
      "whitelisted_at": "2024-03-15T10:30:00Z",
      "contract_address": "1000,0"
    }
  ],
  "total_count": 89,
  "contract_info": {
    "contract_address": "1000,0",
    "contract_name": "rwa_identity_registry"
  }
}
```

### GET /identity-registry/blacklisted (Admin)

List all blacklisted addresses from the identity-registry contract

**Query Parameters:**

- `limit` (number, optional, default: 50, max: 1000) - Number of records to return
- `offset` (number, optional, default: 0) - Number of records to skip

**Response:**

```json
{
  "blacklisted_addresses": [
    {
      "address": "4UC8o4m8AgTxt5VBFMdLwMJwHhVmr5CqzXMTfUP8PU5t3oN6vT",
      "blacklisted_at": "2024-03-15T10:30:00Z",
      "contract_address": "1000,0"
    }
  ],
  "total_count": 123,
  "page_count": 3,
  "current_page": 1,
  "contract_info": {
    "contract_address": "1000,0",
    "contract_name": "rwa_identity_registry"
  }
}
```

### GET /identity-registry/blacklisted/{address} (Admin/Public)

Check if a specific address is blacklisted

**Path Parameters:**

- `address` (string, required) - Wallet address to check

**Response:**

```json
{
  "address": "4UC8o4m8AgTxt5VBFMdLwMJwHhVmr5CqzXMTfUP8PU5t3oN6vT",
  "is_blacklisted": true,
  "blacklisted_at": "2024-03-15T10:30:00Z",
  "contract_address": "1000,0",
  "verification_status": {
    "is_verified": false,
    "has_identity": true,
    "blacklist_reason": "blacklisted"
  }
}
```

**Response (Not Blacklisted):**

```json
{
  "address": "4UC8o4m8AgTxt5VBFMdLwMJwHhVmr5CqzXMTfUP8PU5t3oN6vT",
  "is_blacklisted": false,
  "blacklisted_at": null,
  "contract_address": "1000,0",
  "verification_status": {
    "is_verified": true,
    "has_identity": true,
    "blacklist_reason": null
  }
}
```

## Implementation Details

### Database Queries

The API will query the blacklisted_addresses table created by the processor:

```rust path=null start=null
// List blacklisted addresses with pagination
pub async fn get_blacklisted_addresses(
    conn: &mut DbConnection,
    limit: u32,
    offset: u32,
) -> Result<(Vec<BlacklistedAddress>, u64)> {
    // Implementation using Diesel ORM
    // Query single configured identity-registry contract
    // Apply pagination with limit/offset
    // Return addresses and total count
}

// Check if specific address is blacklisted
pub async fn is_address_blacklisted(
    conn: &mut DbConnection,
    address: &str,
) -> Result<Option<BlacklistedAddress>> {
    // Query blacklisted_addresses table for configured contract
    // Return blacklist record if found
}
```

### Error Handling

```rust path=null start=null
#[derive(Debug, Serialize)]
pub enum IdentityRegistryApiError {
    InvalidAddress,
    ContractNotFound,
    DatabaseError,
    InvalidPagination,
}

impl IntoResponse for IdentityRegistryApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            IdentityRegistryApiError::InvalidAddress => {
                (StatusCode::BAD_REQUEST, "Invalid wallet address format")
            }
            IdentityRegistryApiError::ContractNotFound => {
                (StatusCode::NOT_FOUND, "Identity registry contract not found")
            }
            IdentityRegistryApiError::DatabaseError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Database query failed")
            }
            IdentityRegistryApiError::InvalidPagination => {
                (StatusCode::BAD_REQUEST, "Invalid pagination parameters")
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
```

### Authentication & Authorization

- **Admin Endpoints:** Require valid Cognito JWT token with admin role
- **Public Endpoints:** Address check endpoint can be public for transparency
- **Rate Limiting:** Apply rate limits to prevent abuse of public endpoints

### Validation

```rust path=null start=null
// Address validation
pub fn validate_concordium_address(address: &str) -> Result<(), ValidationError> {
    // Validate Concordium address format
    // Support both account and contract addresses
    // Return descriptive error messages
}

// Pagination validation
pub fn validate_pagination(limit: u32, offset: u32) -> Result<(), ValidationError> {
    if limit > 1000 {
        return Err(ValidationError::LimitTooLarge);
    }
    if limit == 0 {
        return Err(ValidationError::InvalidLimit);
    }
    Ok(())
}
```

## Security Considerations

### Data Privacy

- Blacklist information is considered public for compliance transparency
- No sensitive personal information exposed through API
- Only wallet addresses and timestamps are returned

### Access Control

- Admin endpoints require proper authentication
- Public endpoints have rate limiting
- Audit logging for all admin access

### Input Validation

- Strict validation of wallet addresses
- Sanitization of all input parameters
- Prevention of SQL injection through ORM usage

## Performance Optimization

### Database Indexes

- Ensure proper indexing on blacklisted_addresses table
- Optimize queries for large datasets
- Consider caching for frequently accessed data

### Response Caching

```rust path=null start=null
// Cache blacklist status for short periods
pub async fn get_cached_blacklist_status(
    address: &str,
) -> Result<BlacklistStatus> {
    // Check cache first
    // Query database if cache miss
    // Cache result for 5 minutes
}
```

### Rate Limiting

- Apply rate limits to public endpoints
- Different limits for authenticated vs anonymous users
- Implement proper error responses for rate limit exceeded

## Testing Strategy

### Unit Tests

```rust path=null start=null
#[tokio::test]
async fn test_get_blacklisted_addresses() {
    // Test pagination
    // Test empty results
    // Test large datasets
}

#[tokio::test]
async fn test_address_blacklist_check() {
    // Test blacklisted address
    // Test non-blacklisted address
    // Test invalid address format
}
```

### Integration Tests

- Test with real database
- Test authentication flows
- Test error handling scenarios
- Performance tests with large datasets

## API Documentation

### OpenAPI Specification

- Complete OpenAPI 3.0 specification
- Generated TypeScript client for frontend
- Interactive documentation via Swagger UI

### Response Examples

- Comprehensive examples for all endpoints
- Error response examples
- Different scenario coverage

## Monitoring and Metrics

### Key Metrics

- API response times per endpoint
- Request volume and patterns
- Error rates by endpoint type
- Database query performance

### Alerting

- High error rates on blacklist endpoints
- Slow database queries
- Authentication failures
- Unusual access patterns

## Integration with Frontend

### Generated Client

- TypeScript client auto-generated from OpenAPI spec
- Strongly typed interfaces for all responses
- Built-in error handling and retry logic

### Frontend Usage

```typescript path=null start=null
// Example frontend usage
const blacklistedAddresses = await apiClient.identityRegistry.getBlacklistedAddresses({
  limit: 50,
  offset: 0
});

const isBlacklisted = await apiClient.identityRegistry.checkAddressBlacklist(
  '4UC8o4m8AgTxt5VBFMdLwMJwHhVmr5CqzXMTfUP8PU5t3oN6vT'
);
```
