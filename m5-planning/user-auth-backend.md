# User Registration and Authentication Backend (FR-ID-1)

This document outlines the backend API and database layer for user registration and authentication, integrating with AWS Cognito and External Identity Provider (KYC) through Concordium Web3Id presentation proofs.

## API Layer

### User Registration Request Endpoints

#### Request Registration Invitation

```http path=null start=null
POST /user/registration-request
Content-Type: application/json

{
  "email": "user@example.com",
  "affiliate_account_address": "4owvMHZAXXX..." // Optional
}
```

**Purpose**: Creates a registration request for users without invitation links. Admin approval required.

**Authentication**: None (public endpoint)

**Response**: `UserRegistrationRequest`

```typescript path=null start=null
{
  "id": "uuid",
  "email": "string",
  "affiliate_account_address": "string | null",
  "is_accepted": false,
  "created_at": "timestamp",
  "updated_at": "timestamp"
}
```

**Validation**:

- Email format validation
- Check if user already registered
- Check if registration request already exists

**Error Codes**:

- `400` - User already registered
- `400` - Invalid email format
- `500` - Database error

#### User Registration (Final Step)

```http path=null start=null
POST /user/register
Content-Type: application/json
Authorization: None (validates temp password via Cognito)

{
  "account_address": "4owvMHZAXXX...",
  "email": "user@example.com", 
  "temp_password": "temp123456",
  "password": "newPassword123",
  "proof": {
    // Concordium Web3Id Presentation object
    "verifiableCredential": [...],
    "challenge": "hash",
    "presentationContext": "..."
  },
  "desired_investment_amount": 50000 // Optional, in EUR cents
}
```

**Purpose**: Final registration step after admin approval or invitation. Validates KYC presentation and creates permanent user account.

**Authentication**: Validates `temp_password` against Cognito

**Processing Flow**:

1. Validate `temp_password` via Cognito `InitiateAuth` (expects `NewPasswordRequired` challenge)
2. Verify Web3Id presentation using `concordium::identity::verify_presentation`
3. Extract KYC attributes (`firstName`, `lastName`, `nationality`)  
4. Create permanent Cognito user with confirmed password
5. Store user in database with KYC attributes
6. Return user model with KYC verification status

**Response**: `UserKYCModel`

```typescript path=null start=null
{
  "account_address": "string",
  "cognito_user_id": "string",
  "email": "string",
  "first_name": "string",
  "last_name": "string", 
  "nationality": "string",
  "affiliate_account_address": "string | null",
  "affiliate_commission": "decimal",
  "desired_investment_amount": "decimal | null",
  "kyc_verified": true
}
```

**Error Codes**:

- `400` - Invalid temp password / User not found in Cognito
- `400` - Invalid Web3Id presentation proof
- `400` - Account address already registered with different email
- `400` - Email already registered with different account address
- `500` - Cognito operation failed
- `500` - Database error

### Authentication Endpoints

#### User Login

```http path=null start=null
POST /user/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "userPassword123"
}
```

**Purpose**: Authenticate user and return JWT token.

**Note**: This endpoint is handled client-side via AWS Cognito JavaScript SDK. Backend does not process login directly - JWT validation occurs in protected endpoints via `BearerAuthorization` middleware.

**Frontend Implementation**:

- Uses `CognitoUser.authenticateUser()` from `amazon-cognito-identity-js`
- Returns `CognitoUserSession` containing JWT tokens
- Stores session in browser storage
- Includes JWT in `Authorization: Bearer <token>` header for API calls

#### User Logout

```http path=null start=null
POST /user/logout
Authorization: Bearer <jwt_token>
```

**Purpose**: Invalidate user session.

**Implementation**: Client-side via Cognito SDK `CognitoUser.signOut()`. Server-side JWT validation will fail after logout.

### Admin Registration Endpoints

#### List Registration Requests

```http path=null start=null
GET /admin/registration-request/list?page=0&page_size=20
Authorization: Bearer <admin_jwt_token>
```

**Authentication**: Admin role required (`ensure_is_admin`)

**Response**: `PagedResponse<UserRegistrationRequest>`

#### Get Registration Request

```http path=null start=null
GET /admin/registration-request/:id
Authorization: Bearer <admin_jwt_token>
```

**Authentication**: Admin role required

**Response**: `UserRegistrationRequest`

#### Accept/Reject Registration Request

```http path=null start=null
PUT /admin/registration-request/:id/accept/:is_accepted
Authorization: Bearer <admin_jwt_token>
```

**Parameters**:

- `id`: UUID of registration request
- `is_accepted`: `true` or `false`

**Processing**:

- If accepted: Creates temp Cognito user with generated password, sends email invitation
- If rejected: Deletes registration request
- Both cases: Removes request from database

**Authentication**: Admin role required

#### Admin User Registration (Direct)

```http path=null start=null
POST /admin/user/register
Content-Type: application/json
Authorization: Bearer <admin_jwt_token>

{
  "account_address": "4owvMHZAXXX...",
  "email": "user@example.com",
  "password": "permanentPassword123",
  "first_name": "John",
  "last_name": "Doe", 
  "nationality": "DK"
}
```

**Purpose**: Direct user creation by admin, bypassing KYC presentation verification.

**Processing**:

1. Creates permanent Cognito user immediately
2. Sets user attributes directly (no Web3Id proof required)
3. Stores user in database
4. Returns user model

**Authentication**: Admin role required

**Response**: `UserKYCModel`

## Cognito Integration

### User Attributes

Cognito custom attributes stored for each user:

```rust path=null start=null
// Custom attributes in AWS Cognito User Pool
"custom:con_accnt" -> "4owvMHZAXXX..."           // Concordium account address  
"custom:affiliate_con_accnt" -> "4owvMHZAXXX..."  // Optional referral address
"custom:nationality" -> "DK"                      // From KYC presentation
"custom:company_id" -> "uuid-string"              // Optional company association

// Standard attributes
"email" -> "user@example.com" 
"email_verified" -> "true"
"given_name" -> "John"        // From KYC presentation  
"family_name" -> "Doe"        // From KYC presentation
```

### Operations Mapping

```rust path=null start=null
// Registration request acceptance -> temporary user creation
aws_sdk_cognitoidentityprovider::Client::admin_create_user()
  - UserAttributes: email, email_verified=true
  - TemporaryPassword: auto-generated
  - MessageAction: "SUPPRESS" or "RESEND" (send email)

// User registration -> permanent password setup  
aws_sdk_cognitoidentityprovider::Client::initiate_auth()
  - AuthFlow: "USER_PASSWORD_AUTH" 
  - Expects: ChallengeNameType::NewPasswordRequired

aws_sdk_cognitoidentityprovider::Client::respond_to_auth_challenge()
  - ChallengeName: "NEW_PASSWORD_REQUIRED"
  - ChallengeResponses: NEW_PASSWORD, userAttributes

// Admin direct registration -> immediate permanent user
aws_sdk_cognitoidentityprovider::Client::admin_create_user()
  - TemporaryPassword: none (permanent from start)
  - UserAttributes: all KYC + custom attributes
```

### KYC Presentation Verification

**Current Implementation** (Limited Information):

```rust path=null start=null
// Backend verification using existing concordium utilities
use crate::utils::concordium::identity::{verify_presentation, VerifyPresentationResponse};

let verification_res = verify_presentation(
    &mut concordium_client,
    proof,              // From request JSON
    account_address,    // Parsed from request
    network,            // Testnet/Mainnet
    global_context,     // Concordium global context
    challenge          // Generated from email hash
).await?;

// Extracts KYC attributes (LIMITED DATA):
// - verification_res.first_name
// - verification_res.last_name  
// - verification_res.nationality
// NOTE: Insufficient for regulatory compliance requirements
```

**Proposed External KYC Integration** (Digital Trust Solutions):

```rust path=null start=null
// UNCLEAR IMPLEMENTATION - Integration details undefined
// Proposed flow with Digital Trust Solutions (DTS) as Identity Provider:

// 1. Verify account was created via DTS Identity Provider
let identity_provider = get_identity_provider(&mut concordium_client, account_address).await?;
if identity_provider != "digital_trust_solutions" {
    return Err(Error::BadRequest(PlainText(
        "Account must be created via Digital Trust Solutions KYC process"
    )));
}

// 2. Fetch comprehensive KYC data from DTS API
// UNCLEAR: How to associate Concordium account with DTS user record
let kyc_data = dts_api::get_user_kyc_details(
    account_address,    // How is this mapped to DTS user ID?
    api_key,           // DTS API authentication
).await?;

// 3. Extract comprehensive compliance data
// UNCLEAR: Exact data structure and available fields
// Expected additional data:
// - Full address information
// - Date of birth
// - Identification document details
// - Enhanced due diligence flags
// - Sanctions screening results
```

## Database Layer

### Schema

```sql path=null start=null
-- User registration requests (pending admin approval)
CREATE TABLE user_registration_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT UNIQUE NOT NULL,
    affiliate_account_address TEXT,
    is_accepted BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);

-- Main user records (post-registration)
CREATE TABLE users (
    account_address TEXT PRIMARY KEY,                    -- Concordium account address
    cognito_user_id TEXT UNIQUE NOT NULL,               -- AWS Cognito "sub" attribute  
    email TEXT UNIQUE NOT NULL,
    first_name TEXT NOT NULL,                           -- From KYC presentation
    last_name TEXT NOT NULL,                            -- From KYC presentation
    nationality TEXT NOT NULL,                          -- From KYC presentation
    affiliate_commission DECIMAL(5,4) NOT NULL DEFAULT 0.1000,  -- 10% default
    desired_investment_amount DECIMAL(78, 0),           -- Optional, in EUR cents
    affiliate_account_address TEXT,                     -- Optional referral address
    company_id UUID REFERENCES companies(id),           -- Optional company association
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);

-- Indexes for performance
CREATE INDEX idx_users_cognito_user_id ON users(cognito_user_id);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_company_id ON users(company_id);
CREATE INDEX idx_user_registration_requests_email ON user_registration_requests(email);
```

### Diesel Models

```rust path=null start=null
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Serialize, Deserialize, Clone, Debug)]
#[diesel(table_name = user_registration_requests)]
pub struct UserRegistrationRequest {
    pub id: Uuid,
    pub email: String,
    pub affiliate_account_address: Option<String>,
    pub is_accepted: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = user_registration_requests)]
pub struct NewUserRegistrationRequest {
    pub email: String,
    pub affiliate_account_address: Option<String>,
}

#[derive(Queryable, Serialize, Deserialize, Clone, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub account_address: String,
    pub cognito_user_id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub nationality: String,
    pub affiliate_commission: rust_decimal::Decimal,
    pub desired_investment_amount: Option<rust_decimal::Decimal>,
    pub affiliate_account_address: Option<String>,
    pub company_id: Option<Uuid>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub account_address: String,
    pub cognito_user_id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub nationality: String,
    pub affiliate_commission: rust_decimal::Decimal,
    pub desired_investment_amount: Option<rust_decimal::Decimal>,
    pub affiliate_account_address: Option<String>,
}

// Response model with KYC verification status
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserKYCModel {
    pub account_address: String,
    pub cognito_user_id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub nationality: String,
    pub affiliate_account_address: Option<String>,
    pub affiliate_commission: rust_decimal::Decimal,
    pub desired_investment_amount: Option<rust_decimal::Decimal>,
    pub kyc_verified: bool,  // Checked against identity_registry contract
}
```

### Database Operations

```rust path=null start=null
impl UserRegistrationRequest {
    pub fn find_by_email(
        conn: &mut PgConnection, 
        email: &str
    ) -> QueryResult<Option<Self>> {
        user_registration_requests
            .filter(schema::user_registration_requests::email.eq(email))
            .first(conn)
            .optional()
    }

    pub fn list(
        conn: &mut PgConnection,
        page: i64,
        page_size: i64
    ) -> QueryResult<(Vec<Self>, i64)> {
        let offset = page * page_size;
        let results = user_registration_requests
            .offset(offset)
            .limit(page_size)
            .load(conn)?;
        
        let total_count = user_registration_requests
            .count()
            .get_result::<i64>(conn)?;
        let page_count = (total_count + page_size - 1) / page_size;
        
        Ok((results, page_count))
    }

    pub fn insert(self, conn: &mut PgConnection) -> QueryResult<Self> {
        diesel::insert_into(user_registration_requests)
            .values(&self)
            .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, id: Uuid) -> QueryResult<usize> {
        diesel::delete(
            user_registration_requests.filter(
                schema::user_registration_requests::id.eq(id)
            )
        ).execute(conn)
    }
}

impl User {
    pub fn find_by_email(
        conn: &mut PgConnection, 
        email: &str
    ) -> QueryResult<Option<Self>> {
        users
            .filter(schema::users::email.eq(email))
            .first(conn)
            .optional()
    }

    pub fn find_by_account_address(
        conn: &mut PgConnection, 
        address: &str
    ) -> QueryResult<Option<Self>> {
        users
            .filter(schema::users::account_address.eq(address))
            .first(conn)
            .optional()
    }

    pub fn find_by_cognito_id(
        conn: &mut PgConnection, 
        cognito_id: &str
    ) -> QueryResult<Option<Self>> {
        users
            .filter(schema::users::cognito_user_id.eq(cognito_id))
            .first(conn)
            .optional()
    }

    pub fn upsert(self, conn: &mut PgConnection) -> QueryResult<Self> {
        diesel::insert_into(users)
            .values(&self)
            .on_conflict(schema::users::account_address)
            .do_update()
            .set((
                schema::users::email.eq(&self.email),
                schema::users::first_name.eq(&self.first_name),
                schema::users::last_name.eq(&self.last_name),
                schema::users::nationality.eq(&self.nationality),
                schema::users::updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .get_result(conn)
    }

    pub fn update_company_id(
        conn: &mut PgConnection,
        cognito_id: &str,
        company_id: Option<Uuid>
    ) -> QueryResult<usize> {
        diesel::update(
            users.filter(schema::users::cognito_user_id.eq(cognito_id))
        )
        .set((
            schema::users::company_id.eq(company_id),
            schema::users::updated_at.eq(chrono::Utc::now().naive_utc()),
        ))
        .execute(conn)
    }
}
```

## Two-Path Registration Flows

### Path A: Existing Concordium Account Holder

**Prerequisites**: User has Concordium Wallet with KYC-verified identity credentials

1. **Direct Registration Access** - User visits `/register` or follows invite link `/register?invite={uuid}`
2. **Connect Wallet** - Frontend detects Concordium provider, user connects wallet
3. **KYC Presentation** - Frontend requests verifiable presentation with `firstName`, `lastName`, `nationality`
4. **Submit Registration** - POST `/user/register` with account address, email, temp password, KYC proof
5. **Backend Verification** - Validates temp password (Cognito), verifies Web3Id presentation, creates permanent user
6. **Auto Sign-in** - User redirected to login with new permanent password

### Path B: New User (No Concordium Account)

**Prerequisites**: User needs to create Concordium account and complete KYC with external provider

1. **Registration Request** - User visits `/register`, submits email via POST `/user/registration-request`
2. **Admin Approval** - Admin reviews request via `/admin/registration-request/list`, accepts/rejects
3. **Wallet Creation Instructions** - If approved, user receives email with temporary password and wallet setup instructions
4. **External KYC Process** - User creates Concordium account, completes KYC with configured Identity Provider
5. **Return to Platform** - User clicks email link, returns to `/register` with temp password pre-filled
6. **Follow Path A** - User connects newly created wallet and proceeds with KYC presentation verification

### Invite Link Tracking

Both paths support optional affiliate tracking:

- Invite Link Format: `/register?invite={uuid}` or `/register/{affiliate_account_address}`
- Frontend captures invite parameter, includes in `affiliate_account_address` field
- Backend stores affiliate relationship in user record
- Affiliate commission rate applied from `AffiliateCommission` config (default 10%)

## Error Handling

### Common Error Patterns

```rust path=null start=null
// Email already registered
Error::BadRequest(PlainText("User already registered"))

// Invalid Web3Id presentation
Error::BadRequest(PlainText("Invalid identity proof"))  

// Temp password validation failed  
Error::BadRequest(PlainText("Invalid temp password"))

// Account address conflicts
Error::BadRequest(PlainText(
    "Account address already registered with different email"
))

// KYC presentation verification failed
Error::BadRequest(PlainText("KYC verification failed"))

// Cognito service errors
Error::InternalServer(PlainText("Authentication service unavailable"))

// Database connection/constraint errors
Error::InternalServer(PlainText("Registration failed - please try again"))
```

### Frontend Error Display

- Registration form shows validation errors inline
- Wallet connection errors display as alerts
- KYC presentation failures redirect to wallet setup instructions
- Network/server errors show generic "try again" message with option to contact support

## Security Considerations

### Authentication Flow Security

- Temp passwords single-use, expire after email delivery
- KYC presentations include challenge derived from email hash (prevents replay attacks)
- JWT tokens validated on all protected endpoints
- Account address ownership verified via Web3Id presentation signature

### Data Protection

- PII (first_name, last_name, nationality) encrypted at rest in database
- Email addresses unique constraints prevent duplicate registrations
- Affiliate addresses validated as proper Concordium account format
- Cognito handles password storage/hashing according to AWS security standards

### Admin Access Controls

- Admin endpoints require `ensure_is_admin` middleware check
- Admin role assigned via Cognito user groups
- Registration request approval creates audit trail
- Direct admin registration bypasses KYC for internal testing only

## KYC Integration Status & Outstanding Questions

### Current System Limitations

**Problem with Existing Concordium Web3Id Approach**:

- Limited personal information available through Web3Id presentation proofs
- Only basic attributes: `firstName`, `lastName`, `nationality`
- Insufficient data for comprehensive regulatory compliance requirements
- Missing critical KYC data: full address, date of birth, identification documents, sanctions screening

### Proposed Solution: Digital Trust Solutions Integration

**Partnership Overview**:

- Upwood has partnered with Digital Trust Solutions (DTS): <https://www.digitaltrustsolutions.nl/>
- DTS is already an authorized Identity Provider on the Concordium network
- Goal: Single KYC process for both Concordium account creation and Upwood platform registration

### New Registration Flow (UNCLEAR IMPLEMENTATION)

**Target User**: New user without existing Concordium account

1. **Wallet Setup**:
   - User downloads Concordium Browser Wallet
   - Creates new Concordium account through wallet interface
   - Account creation requires KYC verification via Concordium Identity Provider

2. **KYC Provider Selection**:
   - User must complete KYC specifically through Digital Trust Solutions
   - **UNCLEAR**: How to ensure user selects DTS and not other Identity Providers
   - **UNCLEAR**: How to communicate to DTS backend that this user is an Upwood platform user

3. **Platform Registration**:
   - User connects newly created account to Upwood platform
   - Backend verifies account was created via DTS Identity Provider
   - Backend fetches comprehensive KYC data from DTS API
   - User registered with full compliance data

### Outstanding Technical Questions

#### Identity Provider Verification

**Question**: How to verify Concordium account was created via specific Identity Provider?

**Potential Approaches**:

```rust path=null start=null
// Option 1: Query Concordium node for account creation details
// UNCLEAR: Does Concordium API provide Identity Provider information?
let account_info = concordium_client
    .get_account_info(account_address, BlockIdentifier::LastFinal)
    .await?;

// Option 2: Check identity credentials for specific provider signature
// UNCLEAR: Can we identify DTS-issued credentials cryptographically?
let credentials = concordium_client
    .get_identity_credentials(account_address)
    .await?;

// Option 3: Maintain allowlist of DTS-verified accounts
// UNCLEAR: How would this list be populated and synchronized?
```

#### DTS API Integration

**Question**: How to associate Concordium account address with DTS user record?

**Integration Challenges**:

- DTS KYC process occurs during Concordium account creation
- Upwood backend has no visibility into DTS user ID or session
- Need to map Concordium account address to DTS user record

**Potential Solutions**:

```rust path=null start=null
// Option 1: Account address as primary key
// Assumes DTS stores Concordium address for each user
struct DTSUserRequest {
    concordium_address: String,  // Primary identifier
}

// Option 2: Additional identifier exchange
// Requires coordination between DTS and Upwood during KYC
struct DTSUserRequest {
    concordium_address: String,
    upwood_session_id: String,   // Shared during KYC process
}

// Option 3: Webhook notification
// DTS notifies Upwood when user completes KYC
struct DTSWebhook {
    concordium_address: String,
    kyc_data: DTSUserData,
    timestamp: DateTime<Utc>,
}
```

#### DTS API Specification

**Question**: What is the exact API structure for fetching KYC data?

**Required Information**:

```typescript path=null start=null
// Expected comprehensive KYC data structure
interface DTSUserData {
  // Basic identity
  firstName: string;
  lastName: string;
  dateOfBirth: string;          // UNCLEAR: Format? ISO date?
  nationality: string;
  
  // Address information  
  streetAddress: string;
  city: string;
  postalCode: string;
  country: string;
  
  // Identification documents
  documentType: string;         // passport, driver_license, national_id
  documentNumber: string;
  documentExpiryDate: string;
  documentIssuer: string;
  
  // Compliance flags
  sanctionsScreened: boolean;
  pepScreened: boolean;         // Politically Exposed Person
  enhancedDueDiligence: boolean;
  riskLevel: "low" | "medium" | "high";
  
  // Verification status
  kycCompletedAt: string;
  kycExpiresAt?: string;
  verificationLevel: string;    // basic, enhanced, etc.
}
```

### Interim Development Approach (M5 Milestone)

**Current Implementation Strategy**:

- Continue development using existing Web3Id presentation verification
- Admin creates users directly via `POST /admin/user/register` endpoint
- Users can login and access platform functionality
- External KYC integration implemented in future milestone

**Admin User Creation**:

```rust path=null start=null
// Temporary approach for M5 development
// Admin bypasses KYC verification entirely
let user = User {
    account_address: req.account_address,
    cognito_user_id: cognito_user.cognito_user_id,
    email: req.email,
    first_name: req.first_name,      // Admin provides directly
    last_name: req.last_name,        // Admin provides directly  
    nationality: req.nationality,    // Admin provides directly
    // ... other fields
};

// User immediately whitelisted in identity registry
identity_registry_contract.whitelist_address(user.account_address).await?;
```

### Migration Path to Full KYC Integration

**Phase 1 (Current M5)**:

- Admin-created users with basic information
- Web3Id presentation verification for self-registration
- Limited compliance data collection

**Phase 2 (Future Milestone)**:

- DTS API integration implementation
- Enhanced user registration flow
- Comprehensive KYC data collection
- Automated compliance checking

**Phase 3 (Full Production)**:

- Real-time KYC status monitoring
- Automated sanctions screening
- Regulatory reporting capabilities
- Advanced compliance workflows

### Required Decisions for Implementation

1. **DTS Integration Architecture**: API-based vs webhook vs hybrid approach
2. **Account Verification Method**: How to ensure DTS-created accounts
3. **Data Mapping**: Concordium address to DTS user ID association
4. **Error Handling**: Failed KYC verification scenarios
5. **Data Storage**: Where to store comprehensive KYC data (compliance with GDPR)
6. **User Experience**: How to guide users to specific Identity Provider
7. **Testing Strategy**: Sandbox/testing environment for DTS integration
