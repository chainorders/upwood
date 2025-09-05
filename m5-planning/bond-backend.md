# Bond Backend Component

This document defines the comprehensive bond backend component including API endpoints and database schema for managing off-chain bond metadata, investor bond queries, subscription agreement management (FR-DS-1), and integration with blockchain processor data.

## File Locations

### API Layer

- Bond APIs: `backend/upwood/src/api/bonds.rs`
- Update: `backend/upwood/src/api/mod.rs` to include bonds module

### Database Layer (API-Managed)

- Bond Metadata Models: `backend/shared/src/db/bonds_metadata.rs`
- Subscription Signature Models: `backend/shared/src/db/user_subscription_signatures.rs` (FR-DS-1)
- Schema: `backend/shared/src/schema.rs`

## Bond Metadata Structure Reference

The `bond_metadata` object contains comprehensive bond information including:

- Basic information: `name`, `location`, `bond_display_picture_url`
- Market analysis: `market_analysis` (JSONB with timber trends, carbon credit market data)
- Competitive analysis: `competitive_analysis` (JSONB with market position, growth potential, risk levels)
- Financial projections: `project_financials` with investment goals, progress, revenue streams
- Legal documents: `prospectus_documents` (JSONB with prospectus, memorandum, projections, assessments)
- Subscription agreement: `subscription_agreement` (JSONB with agreement name, document URL, hash, file size) (FR-DS-1)
  - **Signature Implementation**: When present in metadata, requires investor signature via dedicated signing endpoint
  - **Signature Storage**: User signatures stored in `bond_subscription_agreement_user_signatures` table
  - **Verification**: Uses same pattern as forest project legal contract signatures with Concordium wallet verification
- Geographic data: `geo_spatial_info` with coordinates and area details
- Environmental data: `forest_composition`, `environmental_impact` with carbon sequestration and biodiversity

For complete structure details, see the database schema section.

---

# PART I: API ENDPOINTS

## Bond Metadata Management (Admin)

### POST /admin/bonds/metadata (Admin)

Create new bond metadata entry

**Headers:**

- `Authorization: Bearer <admin_jwt_token>` (required)

**Input Parameters:**

```json
{
  "name": "EU Taxonomy aligned Green Bonds",
  "location": "Latvia, EU",
  "bond_display_picture_url": "https://s3.bucket/bond_pictures/bond_123.jpg",
  "market_analysis": {
    "timber_market_trends": "The timber market in the Baltic region has shown consistent growth over the past decade, with prices increasing by an average of 5-7% annually.",
    "carbon_credit_market": "The voluntary carbon market has experienced significant growth, with prices for forest-based carbon credits increasing by over 140% in the last year."
  },
  "competitive_analysis": {
    "market_position": {
      "status": "Strong",
      "indicator": 80
    },
    "growth_potential": {
      "status": "Very High",
      "indicator": 90
    },
    "risk_level": {
      "status": "Low",
      "indicator": 30
    }
  },
  "project_financials": {
    "total_investment_goal": "1000000.00",
    "progress": 97.0,
    "token_price": "100.00",
    "revenue_streams": [
      {
        "name": "Timber Revenue",
        "description": "Projected annual timber revenue based on sustainable harvesting practices and current market prices.",
        "estimated_annual_yield": "150000.00"
      }
    ]
  },
  "prospectus_documents": {
    "project_prospectus": "https://s3.bucket/documents/prospectus_bond_123.pdf",
    "investment_memorandum": "https://s3.bucket/documents/memorandum_bond_123.pdf",
    "financial_projection": "https://s3.bucket/documents/financials_bond_123.pdf",
    "risk_assessment": "https://s3.bucket/documents/risk_bond_123.pdf"
  },
  "geo_spatial_info": {
    "latitude": 56.9496,
    "longitude": 24.1052,
    "total_area_hectares": 250.0
  },
  "forest_composition": {
    "pine_percentage": 60.0,
    "spruce_percentage": 30.0,
    "birch_percentage": 10.0
  },
  "environmental_impact": {
    "carbon_sequestration_tons": 5325.0,
    "biodiversity_score": 8.5
  },
  "subscription_agreement": {
    "agreement_name": "Baltic Pine Forest Investment Agreement",
    "document_url": "https://s3.bucket/subscription-agreements/agreement_123.pdf",
    "document_hash": "a665a45920422f9d417e4867efdc4fb8a04a1f3fff1fa07e998e86f7f7a27ae3",
    "file_size_bytes": 1048576
  }
}
```

**Response:**

```json
{
  "bond_metadata_id": "bmd_123e4567-e89b-12d3-a456-426614174000",
  "created_at": "2024-03-15T10:30:00Z"
}
```

### PUT /admin/bonds/metadata/{bond_metadata_id}/contracts (Admin)

Link blockchain contracts to bond metadata after on-chain initialization

**Path Parameters:**

- `bond_metadata_id` (UUID, required) - Bond metadata ID

**Input Parameters:**

```json
{
  "presale_token_contract": "1233",
  "postsale_token_contract": "1234"
}
```

**Validation:**

- Verify contracts exist in bonds processor database
- Return error if contracts not found in blockchain processor tables

**Response:**

```json
{
  "success": true,
  "contracts_linked_at": "2024-03-15T10:30:00Z"
}
```

### POST /admin/documents/upload (Admin)

**SIMPLIFIED ENDPOINT**: Generate presigned URL for uploading any bond-related documents to S3

**Headers:**
- `Authorization: Bearer <admin_jwt_token>` (required)

**Input Parameters:**
```json
{
  "document_type": "subscription_agreement",
  "file_extension": "pdf"
}
```

**Validation:**
- `document_type` must be one of: "prospectus" | "memorandum" | "projection" | "assessment" | "subscription_agreement" | "bond_image"
- `file_extension` must be "pdf" (or "jpg", "png" for bond_image)

**Response:**
```json
{
  "presigned_url": "https://s3.amazonaws.com/bucket/documents/doc_123e4567-e89b-12d3-a456-426614174000.pdf?AWSAccessKeyId=...",
  "file_name": "documents/doc_123e4567-e89b-12d3-a456-426614174000.pdf",
  "expires_at": "2024-03-15T11:30:00Z"
}
```

**Usage Flow:**
1. Admin uploads any document to get S3 URL
2. Admin includes document URL in bond metadata creation/update
3. Backend validates document exists when processing metadata

### PUT /admin/bonds/metadata/{bond_metadata_id} (Admin)

Update existing bond metadata

**Headers:**
- `Authorization: Bearer <admin_jwt_token>` (required)

**Path Parameters:**
- `bond_metadata_id` (UUID, required) - Bond metadata ID

**Input Parameters:** Same structure as POST /admin/bonds/metadata (including subscription_agreement)

**Response:**
```json
{
  "bond_metadata_id": "bmd_123e4567-e89b-12d3-a456-426614174000",
  "updated_at": "2024-03-15T10:30:00Z"
}
```


## Investor Bond Endpoints

### GET /bonds (Investor)

List all bonds with bond metadata and user ownership data for investment dashboard

**Query parameters:**

- `status` (string, optional) - Filter by bond status: Active, Paused, Matured, Success, Failed
- `user_balance_filter` (string, optional) - "owned_only" returns only bonds where user has balance > 0
- `limit` (number, optional, default: 20) - Number of results per page
- `offset` (number, optional, default: 0) - Pagination offset
- `sort_by` (string, optional, default: "created_at") - Sort field
- `sort_order` (string, optional, default: "desc") - Sort direction: "asc", "desc"

**Headers:**

- `Authorization: Bearer <jwt_token>` (required for user ownership data)

**Output:**

```json
{
  "bonds": [
    {
      "bond_metadata_id": "bmd_123e4567-e89b-12d3-a456-426614174000",
      "postsale_token_contract": "1234",
      "presale_token_contract": "1233",
      "status": "Active",
      "maturity_date": "2025-12-31T23:59:59Z",
      "subscription_period_end": "2024-06-30T23:59:59Z",
      "bond_price": "100.00",
      "maximum_supply": "10000.00",
      "current_supply": "7500.00",
      "minimum_raise_amount": "5000.00",
      "lockup_period_duration": "P1Y",
      "created_at": "2024-01-15T10:00:00Z",
      "bond_metadata": {
        "name": "EU Taxonomy aligned Green Bonds",
        "location": "Latvia, EU",
        "bond_display_picture_url": "https://s3.bucket/bond_pictures/bond_123.jpg"
      },
      "user_ownership": {
        "total_invested_plt": "500.00",
        "presale_balance": "0.00",
        "postsale_balance": "5.00"
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

**Note:** This endpoint returns minimal bond_metadata for list view. Complete bond metadata structure is available in the single bond endpoint.

### GET /bonds/{bond_metadata_id} (Investor)

Get detailed information for a single bond including comprehensive metadata and user ownership data

**Path parameters:**

- `bond_metadata_id` (UUID, required) - Bond metadata ID

**Headers:**

- `Authorization: Bearer <jwt_token>` (required for user ownership data)

**Output:**

```json
{
  "bond": {
    "bond_metadata_id": "bmd_123e4567-e89b-12d3-a456-426614174000",
    "postsale_token_contract": "1234",
    "presale_token_contract": "1233",
    "status": "Active",
    "maturity_date": "2025-12-31T23:59:59Z",
    "subscription_period_end": "2024-06-30T23:59:59Z",
    "bond_price": "100.00",
    "maximum_supply": "10000.00",
    "current_supply": "7500.00",
    "minimum_raise_amount": "5000.00",
    "lockup_period_duration": "P1Y",
    "total_supply_all_contracts": "8250.00",
    "created_at": "2024-01-15T10:00:00Z",
    "updated_at": "2024-01-20T14:30:00Z",
    "bond_metadata": {
      "name": "EU Taxonomy aligned Green Bonds",
      "location": "Latvia, EU",
      "bond_display_picture_url": "https://s3.bucket/bond_pictures/bond_123.jpg",
      "market_analysis": {
        "timber_market_trends": "The timber market in the Baltic region has shown consistent growth...",
        "carbon_credit_market": "The voluntary carbon market has experienced significant growth..."
      },
      "competitive_analysis": {
        "market_position": {
          "status": "Strong",
          "indicator": 80
        },
        "growth_potential": {
          "status": "Very High",
          "indicator": 90
        },
        "risk_level": {
          "status": "Low",
          "indicator": 30
        }
      },
      "project_financials": {
        "total_investment_goal": "1000000.00",
        "progress": 97.0,
        "token_price": "100.00",
        "revenue_streams": [
          {
            "name": "Timber Revenue",
            "description": "Projected annual timber revenue...",
            "estimated_annual_yield": "150000.00"
          }
        ]
      },
      "prospectus_documents": {
        "project_prospectus": "https://s3.bucket/documents/prospectus_bond_123.pdf",
        "investment_memorandum": "https://s3.bucket/documents/memorandum_bond_123.pdf",
        "financial_projection": "https://s3.bucket/documents/financials_bond_123.pdf",
        "risk_assessment": "https://s3.bucket/documents/risk_bond_123.pdf"
      },
      "geo_spatial_info": {
        "latitude": 56.9496,
        "longitude": 24.1052,
        "total_area_hectares": 250.0
      },
      "forest_composition": {
        "pine_percentage": 60.0,
        "spruce_percentage": 30.0,
        "birch_percentage": 10.0
      },
      "environmental_impact": {
        "carbon_sequestration_tons": 5325.0,
        "biodiversity_score": 8.5
      }
    },
    "user_ownership": {
      "total_invested_plt": "500.00",
      "presale_balance": "0.00",
      "postsale_balance": "5.00"
    },
    "investment_summary": {
      "total_investors": 150,
      "funding_percentage": 75.0,
      "days_until_subscription_end": 45
    }
  }
}
```

**Note:**

- Complete bond_metadata structure is returned (see reference section above for full details)
- `total_supply_all_contracts` provides aggregate supply across both presale and postsale contracts
- For detailed contract token information, use the bond processor API endpoints

### GET /bonds/{bond_metadata_id}/subscription-agreement/status (Investor)

**NEW FOR FR-DS-1**: Check if user has signed the required subscription agreement for a bond

**Pattern**: Checks bond metadata for subscription agreement presence and queries signature table for user signature status

**Headers:**
- `Authorization: Bearer <jwt_token>` (required)

**Path Parameters:**
- `bond_metadata_id` (UUID, required) - Bond metadata ID

**Process:**
1. Extract user account from JWT claims
2. Query bond_metadata for subscription_agreement field
3. Query bond_subscription_agreement_user_signatures table for user signature record
4. Return combined status and agreement information

**Response (Signed):**
```json
{
  "requires_agreement": true,
  "is_signed": true,
  "signed_at": "2024-01-15T14:30:00Z",
  "agreement": {
    "agreement_name": "Baltic Pine Forest Investment Agreement",
    "document_url": "https://s3.amazonaws.com/bucket/subscription-agreements/...",
    "document_hash": "a665a45920422f9d417e4867efdc4fb8a04a1f3fff1fa07e998e86f7f7a27ae3"
  }
}
```

**Response (Not Signed):**
```json
{
  "requires_agreement": true,
  "is_signed": false,
  "agreement": {
    "agreement_name": "Baltic Pine Forest Investment Agreement",
    "document_url": "https://s3.amazonaws.com/bucket/subscription-agreements/...",
    "document_hash": "a665a45920422f9d417e4867efdc4fb8a04a1f3fff1fa07e998e86f7f7a27ae3"
  }
}
```

**Response (No Agreement Required):**
```json
{
  "requires_agreement": false,
  "is_signed": null,
  "agreement": null
}
```

### POST /bonds/{bond_metadata_id}/subscription-agreement/sign (Investor)

**NEW FOR FR-DS-1**: Submit digital signature for bond's subscription agreement

**Pattern**: Follows the same signature verification pattern as forest project legal contract signing

**Headers:**
- `Authorization: Bearer <jwt_token>` (required)

**Path Parameters:**
- `bond_metadata_id` (UUID, required) - Bond metadata ID

**Input Parameters:**
```json
{
  "0": {
    "0": "304502210089abcdef01234567890abcdef..."
  }
}
```

**Note**: Input follows Concordium `AccountSignatures` format (BTreeMap<u8, CredentialSignatures>)

**Process:**
1. Extract user account address from JWT claims
2. Verify bond has subscription agreement in metadata
3. Verify signature using `verify_account_signature()` with `bond_metadata_id.to_string()` as message
4. Store signature in `bond_subscription_agreement_user_signatures` table
5. Return stored signature record

**Response (Success):**
```json
{
  "bond_metadata_id": "bmd_123e4567-e89b-12d3-a456-426614174000",
  "user_account": "4owvMHZAXXX...",
  "user_signature": "{\"0\":{\"0\":\"304502210089abcdef...\"}}" ,
  "created_at": "2024-01-15T14:30:00Z",
  "updated_at": "2024-01-15T14:30:00Z"
}
```

### GET /user/subscription-agreements (Investor)

**NEW FOR FR-DS-1**: List all subscription agreements signed by the authenticated user (for Contracts dashboard)

**Headers:**
- `Authorization: Bearer <jwt_token>` (required)

**Query Parameters:**
- `limit` (number, optional, default: 20)
- `offset` (number, optional, default: 0)
- `sort_by` (string, optional, default: "signed_at")
- `sort_order` (string, optional, default: "desc")

**Response:**
```json
{
  "signed_agreements": [
    {
      "signature_id": "sig_123e4567-e89b-12d3-a456-426614174000",
      "agreement_name": "Baltic Pine Forest Investment Agreement",
      "signed_at": "2024-01-15T14:30:00Z",
      "bond": {
        "bond_metadata_id": "bmd_123e4567-e89b-12d3-a456-426614174000",
        "bond_name": "Baltic Pine Forest",
        "bond_status": "Active"
      },
      "agreement": {
        "subscription_agreement_id": "sub_123e4567-e89b-12d3-a456-426614174000",
        "document_url": "https://s3.amazonaws.com/bucket/subscription-agreements/...",
        "version": 1,
        "file_size_bytes": 1048576
      }
    }
  ],
  "total_count": 3,
  "pagination": {
    "limit": 20,
    "offset": 0,
    "has_next": false
  }
}
```

## PLT Payment Integration Endpoints (FR-PM-1)

### POST /bonds/{bond_metadata_id}/payment-proof/generate (Investor)

**NEW FOR FR-PM-1**: Generate signed payment proof for bond investment after PLT payment

**Pattern**: Core endpoint for PLT integration - converts PLT transaction to smart contract payment proof

**Headers:**
- `Authorization: Bearer <jwt_token>` (required)

**Path Parameters:**
- `bond_metadata_id` (UUID, required) - Bond metadata ID

**Input Parameters:**
```json
{
  "plt_transaction_hash": "c8b7e8d9a1f2e3b4c5a6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7",
  "plt_amount": "1000.00",
  "investment_amount": "100.00"
}
```

**Process:**
1. Validate transaction hash exists on-chain and was sent to platform wallet
2. Verify transaction amount matches requested PLT amount
3. Check bond investment limits and subscription period
4. Query current investor nonce from `investor_nonces` table for the bond contract
5. Create canonical message (reward_id + nonce + signer_account)
6. Generate platform signature over canonical message
7. Store payment proof record with nonce for audit trail
8. Return signed payment proof for smart contract use

**Validation:**
- Bond must be in active subscription period or open for investment
- PLT transaction must be confirmed on blockchain
- Transaction recipient must be configured platform wallet
- Investment amount must meet bond minimum requirements
- User must have signed subscription agreement (if required)

**Response (Success):**
```json
{
  "payment_proof": {
    "reward_id": "c8b7e8d9a1f2e3b4c5a6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7",
    "nonce": "1",
    "signer_account_address": "4owvMHZAXXX...",
    "signature": "a1b2c3d4e5f6..."
  },
  "bond_contract_address": "1234",
  "investment_details": {
    "token_contract": "presale",
    "expected_tokens": "100.00",
    "token_price": "1.00"
  }
}
```

**Error Responses:**
- `400 Bad Request`: Invalid transaction hash or amount mismatch
- `402 Payment Required`: Subscription agreement not signed
- `409 Conflict`: Transaction already used for payment proof
- `422 Unprocessable Entity`: Bond not accepting investments

**Usage Flow:**
1. Investor transfers PLT to platform wallet via Concordium transaction
2. Frontend calls this endpoint with transaction hash and amounts
3. Backend validates transaction and generates payment proof
4. Frontend uses payment proof to call smart contract `invest` function
5. Smart contract verifies proof and mints bond tokens to investor

### GET /bonds/{bond_metadata_id}/payment-info (Investor)

**NEW FOR FR-PM-1**: Get platform wallet address and payment information for bond investment

**Headers:**
- `Authorization: Bearer <jwt_token>` (required)

**Path Parameters:**
- `bond_metadata_id` (UUID, required) - Bond metadata ID

**Response:**
```json
{
  "platform_wallet_address": "4owvMHZAXXX...",
  "plt_token_info": {
    "token_type": "native",
    "token_name": "CCD",
    "decimals": 6
  },
  "bond_info": {
    "current_contract": "presale",
    "token_price": "1.00",
    "minimum_investment": "100.00",
    "subscription_period_active": true
  }
}
```

### FR-PM-1 Implementation Summary

**Complete PLT Integration Flow:**

1. **Payment Info**: Investor calls `GET /bonds/{id}/payment-info` to get platform wallet address
2. **PLT Transfer**: Investor transfers PLT tokens to platform wallet via Concordium transaction
3. **Proof Generation**: Investor calls `POST /bonds/{id}/payment-proof/generate` with transaction hash
4. **Backend Process**:
   - Validates PLT transaction on-chain
   - Queries current investor nonce from `investor_nonces` table
   - Creates signed payment proof with platform signer
   - Stores proof record in `plt_payment_proofs` table
5. **Smart Contract**: Investor uses payment proof to call bond contract `invest` function
6. **Contract Verification**: Contract verifies proof signature and nonce, mints tokens, emits event
7. **Event Processing**: Backend processes `BondInvestment` event and updates investor nonce

**Key Benefits of PLT Integration:**
- **Decoupled Payments**: Separates PLT transactions from smart contract calls
- **Replay Protection**: Nonce mechanism prevents double-spending of payment proofs
- **Audit Trail**: Complete tracking of PLT payments to bond investments
- **Flexible Pricing**: Allows dynamic EUR/PLT conversion rates
- **Legacy Compatibility**: Maintains existing smart contract interface while enabling PLT payments

## Admin/Operator Endpoints

### POST /admin/bonds/{bond_metadata_id}/maturity/trigger (Admin)

Trigger maturity payments for a bond (two-phase transaction process)

**Path parameters:**

- `bond_metadata_id` (UUID, required) - Bond metadata ID

**Headers:**

- `Authorization: Bearer <admin_jwt_token>` (required)

**Input:**

```json
{
  "plt_token_id": "native_token",
  "face_value_per_token": "1000.00"
}
```

**Output:**

```json
{
  "maturity_job_id": "mat_job_001",
  "total_recipients": 45,
  "total_plt_required": "45000.00",
  "cloud_wallet_balance": "50000.00",
  "sufficient_liquidity": true,
  "status": "initiated"
}
```

---

# PART II: DATABASE SCHEMA

### bonds_metadata

```sql path=null start=null
CREATE TABLE bonds_metadata (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    location TEXT NOT NULL,
    bond_display_picture_url TEXT,
    
    -- Market Analysis (JSONB for structured storage)
    market_analysis JSONB NOT NULL DEFAULT '{}'::jsonb,
    
    -- Competitive Analysis  
    competitive_analysis JSONB NOT NULL DEFAULT '{}'::jsonb,
    
    -- Project Financials
    total_investment_goal DECIMAL(20, 2) NOT NULL,
    progress DECIMAL(5, 2) DEFAULT 0.0,
    token_price DECIMAL(20, 2) NOT NULL,
    revenue_streams JSONB NOT NULL DEFAULT '[]'::jsonb,
    
    -- Prospectus Documents
    prospectus_documents JSONB NOT NULL DEFAULT '{}'::jsonb,
    
    -- Subscription Agreement (FR-DS-1)
    subscription_agreement JSONB DEFAULT NULL, -- Contains agreement_name, document_url, document_hash, file_size_bytes
    
    -- Geo Spatial Information
    latitude DECIMAL(10, 7),
    longitude DECIMAL(10, 7), 
    total_area_hectares DECIMAL(10, 4),
    
    -- Forest Composition
    pine_percentage DECIMAL(5, 2) DEFAULT 0.0,
    spruce_percentage DECIMAL(5, 2) DEFAULT 0.0,
    birch_percentage DECIMAL(5, 2) DEFAULT 0.0,
    
    -- Environmental Impact
    carbon_sequestration_tons DECIMAL(10, 2),
    biodiversity_score DECIMAL(3, 1),
    
    -- Contract Linkage
    presale_token_contract BIGINT,
    postsale_token_contract BIGINT,
    contracts_linked_at TIMESTAMP WITH TIME ZONE,
    
    -- Token Metadata
    metadata_url TEXT, -- S3 URL for CIS-2 token metadata JSON
    
    -- Status Management
    status TEXT NOT NULL DEFAULT 'draft' CHECK (status IN ('draft', 'published', 'archived')),
    
    -- Timestamps
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    -- Constraints
    UNIQUE(presale_token_contract),
    UNIQUE(postsale_token_contract)
);

-- Indexes for efficient queries
CREATE INDEX idx_bonds_metadata_status ON bonds_metadata(status);
CREATE INDEX idx_bonds_metadata_contracts ON bonds_metadata(presale_token_contract, postsale_token_contract);
CREATE INDEX idx_bonds_metadata_location ON bonds_metadata USING gin (to_tsvector('english', location));
CREATE INDEX idx_bonds_metadata_created_at ON bonds_metadata(created_at);

-- Geospatial index for location queries
CREATE INDEX idx_bonds_metadata_geo ON bonds_metadata(latitude, longitude) WHERE latitude IS NOT NULL AND longitude IS NOT NULL;

-- JSON indexes for efficient JSONB queries
CREATE INDEX idx_bonds_metadata_market_analysis ON bonds_metadata USING gin (market_analysis);
CREATE INDEX idx_bonds_metadata_revenue_streams ON bonds_metadata USING gin (revenue_streams);
```

### bond_subscription_agreement_user_signatures (FR-DS-1)

**Pattern**: Follows same structure as `forest_project_legal_contract_user_signatures` table

```sql path=null start=null
CREATE TABLE bond_subscription_agreement_user_signatures (
    bond_metadata_id UUID NOT NULL REFERENCES bonds_metadata(id) ON DELETE CASCADE,
    user_account TEXT NOT NULL, -- Concordium account address (primary user identifier)
    user_signature TEXT NOT NULL, -- JSON serialized AccountSignatures from Concordium wallet
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (bond_metadata_id, user_account)
);

-- Indexes for efficient queries
CREATE INDEX idx_bond_subscription_signatures_account ON bond_subscription_agreement_user_signatures(user_account);
CREATE INDEX idx_bond_subscription_signatures_created ON bond_subscription_agreement_user_signatures(created_at);
```

**Note**: 
- Uses composite primary key (bond_metadata_id, user_account) for unique signature per bond per account
- Stores Concordium `AccountSignatures` as JSON text (verified via `verify_account_signature()` utility)
- Uses only `user_account` (Concordium address) as primary user identifier
- References bond_metadata via UUID instead of SecurityMintFund contract addresses

### plt_payment_proofs (FR-PM-1)

**Pattern**: Tracks PLT payment transactions and generated proofs for bond investments with nonce-based replay protection

```sql path=null start=null
CREATE TABLE plt_payment_proofs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    bond_metadata_id UUID NOT NULL REFERENCES bonds_metadata(id) ON DELETE CASCADE,
    user_account TEXT NOT NULL, -- Concordium account address (primary user identifier)
    
    -- PLT Transaction Details
    plt_transaction_hash TEXT NOT NULL UNIQUE, -- Original PLT transaction hash (becomes reward_id)
    plt_amount DECIMAL(20, 6) NOT NULL, -- PLT amount sent to platform wallet
    investment_amount DECIMAL(20, 2) NOT NULL, -- EUR equivalent investment amount
    platform_wallet_address TEXT NOT NULL, -- Platform wallet that received PLT
    
    -- Payment Proof Details with Nonce Mechanism
    investor_nonce INTEGER NOT NULL, -- Current nonce from bonds contract state (for replay protection)
    signer_account_address TEXT NOT NULL, -- Platform signer account
    proof_signature TEXT NOT NULL, -- Signature over canonical message (reward_id + nonce + signer)
    
    -- Bond Investment Context
    bond_contract_address TEXT, -- Which contract to invest in (presale/postsale)
    expected_token_amount DECIMAL(20, 2), -- Expected bond tokens to be minted
    token_price DECIMAL(20, 2), -- Price per token at time of proof generation
    
    -- Status Tracking
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'used', 'expired', 'invalid')),
    blockchain_transaction_hash TEXT, -- Transaction hash when proof is used on-chain
    used_at TIMESTAMP WITH TIME ZONE, -- When proof was used on smart contract
    
    -- Timestamps
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT (NOW() + INTERVAL '1 hour'), -- Proof expiry
    
    -- Constraints
    UNIQUE(bond_metadata_id, plt_transaction_hash), -- One proof per PLT transaction per bond
    UNIQUE(user_account, investor_nonce, bond_contract_address) -- Unique nonce per investor per bond contract
);

-- Indexes for efficient queries
CREATE INDEX idx_plt_payment_proofs_user ON plt_payment_proofs(user_account, created_at DESC);
CREATE INDEX idx_plt_payment_proofs_bond ON plt_payment_proofs(bond_metadata_id, status);
CREATE INDEX idx_plt_payment_proofs_transaction ON plt_payment_proofs(plt_transaction_hash);
CREATE INDEX idx_plt_payment_proofs_status ON plt_payment_proofs(status, created_at);
CREATE INDEX idx_plt_payment_proofs_nonce ON plt_payment_proofs(user_account, investor_nonce, bond_contract_address);
```

### investor_nonces (FR-PM-1)

**Pattern**: Tracks current nonce state per investor per bond contract (synced from blockchain events)

```sql path=null start=null
CREATE TABLE investor_nonces (
    user_account TEXT NOT NULL, -- Concordium account address
    bond_contract_address TEXT NOT NULL, -- Bond contract address
    current_nonce INTEGER NOT NULL DEFAULT 0, -- Current nonce from contract state
    last_updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    last_event_block_height BIGINT, -- Block height of last processed nonce event
    
    PRIMARY KEY (user_account, bond_contract_address)
);

-- Index for efficient nonce lookups during proof generation
CREATE INDEX idx_investor_nonces_account ON investor_nonces(user_account);
CREATE INDEX idx_investor_nonces_updated ON investor_nonces(last_updated_at);
```

**Note**:
- **Nonce Flow**: Backend queries `investor_nonces` table for current nonce when generating proof
- **Replay Protection**: Smart contract rejects proofs with nonce <= current contract state nonce
- **State Sync**: Events processor updates `investor_nonces` when processing `BondInvestment` events
- **Single User ID**: Uses only `user_account` (Concordium address) as primary identifier
- **Contract-Specific**: Nonces are per bond contract, allowing parallel investments

## Diesel Models

### BondMetadata Model

```rust path=null start=null
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = bonds_metadata)]
pub struct BondMetadata {
    pub id: Uuid,
    pub name: String,
    pub location: String,
    pub bond_display_picture_url: Option<String>,
    pub market_analysis: Value,
    pub competitive_analysis: Value,
    pub total_investment_goal: rust_decimal::Decimal,
    pub progress: Option<rust_decimal::Decimal>,
    pub token_price: rust_decimal::Decimal,
    pub revenue_streams: Value,
    pub prospectus_documents: Value,
    pub latitude: Option<rust_decimal::Decimal>,
    pub longitude: Option<rust_decimal::Decimal>,
    pub total_area_hectares: Option<rust_decimal::Decimal>,
    pub pine_percentage: Option<rust_decimal::Decimal>,
    pub spruce_percentage: Option<rust_decimal::Decimal>,
    pub birch_percentage: Option<rust_decimal::Decimal>,
    pub carbon_sequestration_tons: Option<rust_decimal::Decimal>,
    pub biodiversity_score: Option<rust_decimal::Decimal>,
    pub presale_token_contract: Option<i64>,
    pub postsale_token_contract: Option<i64>,
    pub contracts_linked_at: Option<DateTime<Utc>>,
    pub metadata_url: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Implementation details removed per WARP rules - will be implemented during coding phase
```

## Integration Notes

### Data Sources and Integration

#### Bond Data

- Source: `bonds` table (managed by bond-blockchain.md events processor)
- Primary key: `postsale_token_contract` (BIGINT)
- Includes: basic bond information, status, dates, supply limits

#### Bond Metadata

- Source: `bonds_metadata` table (managed by bond-backend.md)
- Joined via: postsale_token_contract as primary key
- Includes: comprehensive bond information, market analysis, financial projections, documents

#### User Ownership Data

- Source: `bond_investors` and `bond_investment_records` tables
- Requires: JWT token authentication to identify current user
- Aggregated data:
  - `total_invested_plt`: Sum of PLT amounts from investment records
  - `presale_balance`: Current presale token balance
  - `postsale_balance`: Current postsale token balance

#### PLT Payment Integration Data (FR-PM-1)

- Source: `plt_payment_proofs` and `investor_nonces` tables (managed by bond-backend.md)
- Integration: Correlates with bond blockchain processor via `reward_id` (transaction hash)
- Flow:
  - Backend generates payment proof and stores in `plt_payment_proofs` table
  - Investor uses proof in smart contract `invest` function
  - Events processor handles `BondInvestment` event with `reward_id` correlation
  - Backend updates `investor_nonces` table with new nonce from event
  - Backend marks payment proof as 'used' in `plt_payment_proofs` table

### Base Query Structure

```sql path=null start=null
SELECT 
  bm.id as bond_metadata_id,
  b.*,
  bm.*,
  COALESCE(bi.total_invested, 0) as total_invested_plt,
  COALESCE(bi.presale_balance, 0) as presale_balance,
  COALESCE(bi.postsale_balance, 0) as postsale_balance,
  COALESCE(presale_supply.total_supply, 0) + COALESCE(postsale_supply.total_supply, 0) as total_supply_all_contracts
FROM bonds_metadata bm
LEFT JOIN bonds b ON b.postsale_token_contract = bm.postsale_token_contract
LEFT JOIN bond_investors bi ON (bm.id = bi.bond_metadata_id AND bi.account_address = ?)
LEFT JOIN (SELECT contract, SUM(total_supply) as total_supply FROM sft_tokens GROUP BY contract) presale_supply ON presale_supply.contract = bm.presale_token_contract
LEFT JOIN (SELECT contract, SUM(total_supply) as total_supply FROM sft_tokens GROUP BY contract) postsale_supply ON postsale_supply.contract = bm.postsale_token_contract
WHERE bm.status = 'published'
```

### AWS S3 Integration

**Document Storage:**

- Prospectus documents stored in `/documents/` prefix
- Bond display pictures stored in `/bond_pictures/` prefix  
- Token metadata JSON stored in `/token_metadata/` prefix
- All URLs returned are public S3 URLs

### Security Considerations

- Admin-only access to metadata management endpoints
- Public read access to investor-facing metadata
- File upload restricted to authenticated admins
- Input sanitization for all text fields
- File type validation (PDF, JPG, PNG only)
- File size limits (10MB for documents, 5MB for images)

### Performance Considerations

#### Caching Strategy

- Cache bond metadata with TTL (updated when admin modifies metadata)
- Cache bond basic data with TTL (updates from blockchain events)
- Never cache user ownership data (real-time accuracy required)

#### Database Indexes

- `bonds_metadata.status` for published bonds filtering
- `bonds_metadata.contracts` for joining with blockchain data
- `bonds.status` for bond status filtering
- `bond_investors.account_address` for user ownership lookups
