# Investor Workflow for Bonds

This document describes the complete investor-facing workflow for bond investments, from initial registration through maturity payouts.

## Authentication Flow (FR-ID-1)

Before accessing the investment platform, users must complete registration and authentication using their Concordium account and KYC verification.

### Registration Methods

**Invitation-Based Registration**:

- Users receive referral links: `/register?invite={uuid}` or `/register/{affiliate_account_address}`
- Frontend captures invite parameter for affiliate tracking
- 10% default affiliate commission applied to inviter

**Self-Registration**:

- Users visit `/register` and submit registration request
- API: `POST /user/registration-request` (defined in user-auth-backend.md)
- Users receive email with temp password and setup instructions

### Wallet Connection & KYC Verification

**Prerequisites**: User must have Concordium Wallet with KYC-verified identity credentials

**Technical Flow**:

1. **Wallet Detection**: `detectConcordiumProvider()` from `@concordium/browser-wallet-api-helpers`
2. **Account Connection**: `walletApi.getMostRecentlySelectedAccount()` or `requestAccounts()`
3. **Challenge Generation**: Frontend hashes email with `sha256(email)`
4. **KYC Presentation**: Request verifiable presentation

```typescript path=null start=null
// KYC presentation request
const identityResponse = await walletApi.requestVerifiablePresentation(
  emailHash,
  new Web3StatementBuilder()
    .addForIdentityCredentials([0, 1, 2], (b) =>
      b.revealAttribute("firstName")
       .revealAttribute("lastName")
       .revealAttribute("nationality")
    )
    .getStatements()
);
```

### Registration Completion

**API Endpoint**: `POST /user/register` (defined in user-auth-backend.md)

**Request Structure**:

```typescript path=null start=null
{
  "account_address": "4owvMHZAXXX...",  // From connected wallet
  "email": "user@example.com",
  "temp_password": "temp123456",       // From email invitation
  "password": "newPassword123",       // User sets permanent password
  "proof": identityResponse,           // Web3Id presentation
  "desired_investment_amount": 50000   // Optional, EUR cents
}
```

**Backend Processing**:

1. Validate temp password via Cognito `InitiateAuth`
2. Verify Web3Id presentation using `concordium::identity::verify_presentation`
3. Extract KYC attributes (`firstName`, `lastName`, `nationality`)
4. Create permanent Cognito user account
5. Store user in database with KYC attributes
6. Return user model with verification status

### Sign-In Process

**Frontend Implementation**: Client-side via AWS Cognito JavaScript SDK

1. User enters credentials on `/login` page
2. `CognitoUser.authenticateUser()` with email/password
3. Cognito returns `CognitoUserSession` with JWT tokens
4. JWT included in `Authorization: Bearer <token>` for API calls

### Sign-Out Process

1. User clicks logout button
2. Frontend calls `CognitoUser.signOut()`
3. Clear local session storage
4. Redirect to login page

## Dashboard & Portfolio Overview

After successful authentication, investors access their dashboard showing:

- **Available Bonds**: List of active bonds open for investment
- **My Portfolio**: Total investment value and individual bond holdings
- **Payment History**: Record of yield distributions and transactions

Dashboard data retrieved via:

- Active bonds: `GET /bonds?status=Active` (endpoint details in bond-api.md)
- Portfolio data: Aggregated from bond holdings filtered by investor account

## Investment Flow

### Bond Selection & Investment Amount

**User Interface**:

1. Investor browses available bonds on dashboard
2. Clicks **"Invest Now"** on selected bond
3. Reviews bond details (terms, minimum investment, subscription period)
4. Enters EUR investment amount

**Frontend Validation**:

- Amount meets minimum investment requirement
- Investment doesn't exceed bond maximum supply
- Subscription period is still active
- User has sufficient PLT token balance

### Subscription Agreement Signing (FR-DS-1)

**NEW REQUIREMENT**: Before investment proceeds, investor must sign subscription agreement if required by bond

**Pre-Investment Check**:

**API Call**: `GET /bonds/{bond_metadata_id}/subscription-agreement/status`

```typescript path=null start=null
// Frontend pre-investment validation
async function handleInvestButtonClick(bondMetadataId: string, amount: number) {
  try {
    // Check if subscription agreement signature is required
    const signatureStatus = await apiClient.bonds.getSubscriptionAgreementStatus(bondMetadataId);
    
    if (signatureStatus.requires_agreement && !signatureStatus.is_signed) {
      // Redirect to subscription agreement signing flow
      navigate(`/bonds/${bondMetadataId}/subscription-agreement/sign`);
      return;
    }
    
    // Proceed with normal investment flow
    await processInvestment(bondMetadataId, amount);
    
  } catch (error) {
    if (error.code === 'SUBSCRIPTION_AGREEMENT_NOT_SIGNED') {
      // Handle server-side validation error
      navigate(`/bonds/${bondMetadataId}/subscription-agreement/sign`);
    } else {
      showErrorMessage(error.message);
    }
  }
}
```

**Document Review & Signing Flow**:

1. **Document Display**: Investor views subscription agreement PDF in browser
2. **Hash Calculation**: Frontend calculates SHA256 hash of document content
3. **Wallet Signature**: Investor signs document hash using Concordium wallet
4. **Signature Submission**: Frontend submits signature to backend for verification
5. **Investment Continuation**: After successful signing, investment flow proceeds

**Digital Signature Process**:

```typescript path=null start=null
// Document signing implementation
async function signSubscriptionAgreement(agreementId: string, documentHash: string) {
  try {
    // Get connected Concordium wallet
    const walletApi = await detectConcordiumProvider();
    const account = await walletApi.getMostRecentlySelectedAccount();
    
    // Create signature over document hash
    const message = hexToUint8Array(documentHash);
    const signature = await walletApi.signMessage(account, {
      data: message,
      schema: undefined // Raw bytes
    });
    
    // Submit signature to backend
    const signatureResponse = await apiClient.subscriptionAgreements.submitSignature(agreementId, {
      document_hash: documentHash,
      signature_hex: uint8ArrayToHex(signature.signature),
      signing_account_address: account
    });
    
    // Redirect back to investment flow
    navigate(`/bonds/${bondMetadataId}/invest`);
    
  } catch (error) {
    showErrorMessage('Failed to sign subscription agreement: ' + error.message);
  }
}
```

**User Experience Flow**:

1. User clicks "Invest" on bond
2. System checks if subscription agreement is required
3. If unsigned agreement exists:
   - User redirected to `/bonds/{id}/subscription-agreement/sign`
   - Document displayed with "View Document" and "Download Document" buttons
   - User reviews agreement content
   - User clicks "Sign Agreement" button
   - Browser wallet prompts for signature
   - After successful signing, user redirected back to investment
4. If no agreement required or already signed:
   - Investment flow proceeds normally

### Payment Processing

**Off-Chain Payment Flow**:

1. Investor transfers PLT tokens to pre-configured platform wallet
2. Platform generates payment proof containing:
   - `reward_id`: Payment transaction hash
   - `nonce`: Investor-specific nonce for replay protection
   - `signer_account_address`: Authorized platform signer
   - `signature`: Signature over canonical message format

**Payment Proof Structure** (defined in bond-blockchain.md):

```typescript path=null start=null
{
  "reward_id": "transaction_hash",
  "nonce": "investor_nonce", 
  "signer_account_address": "platform_signer",
  "signature": "signed_proof"
}
```

### On-Chain Investment Execution

**Smart Contract Interaction**:

- Investor calls `invest` function on bonds contract
- Parameters: `postsale_token_contract_address`, `amount`, `payment_proof`
- Contract performs verification and mints bond tokens
- Emits `BondInvestment` event with `reward_id` for correlation

**Processing Flow** (defined in bond-blockchain.md):

1. Verify payment proof signature and nonce
2. Check bond subscription period and limits
3. Mint bond tokens to investor wallet
4. Update contract state with new investment
5. Emit event for backend processing

### Investment Confirmation

**Backend Processing**:

- Events processor handles `BondInvestment` event
- Updates investor balances in database tables
- Records investment with `reward_id` correlation
- Triggers UI updates

**User Experience**:

- Real-time portfolio balance updates
- Investment confirmation displayed
- Updated bond tokens visible in **My Assets** section
- Transaction history updated with new investment

## Yield Distribution Flow (FR-BT-4)

### Automatic Yield Payments

**Investor Experience**:

- **No action required** from investor
- PLT tokens automatically sent to investor wallet
- Payments triggered by admin via yield distribution system
- Notifications sent via email/platform alerts

**Payment Calculation**:

- Based on actual holding period per token ID
- Proportional to investment amount and time held
- Only eligible if wallet passes KYC verification
- Blacklisted wallets excluded from payments

**Payment History**:

- View detailed payment history in **My Assets** section
- Shows payment date, amount, bond reference
- Displays payment status (completed, pending, failed)
- Export functionality for tax reporting

## Maturity & Exit Flow (FR-BT-5)

### Bond Maturity Process

**Automated Process**:

- **No action required** from investor
- Triggered automatically when bond reaches maturity date
- Two-phase execution managed by platform

**Phase 1 - Token Burning**:

- Bond tokens automatically burned from investor wallet
- Ensures investor cannot trade tokens after maturity
- Creates definitive record of eligible investors

**Phase 2 - Face Value Payment**:

- PLT tokens equivalent to face value transferred to wallet
- Payment amount based on original investment principal
- Only whitelisted investors receive payments

### Whitelist Requirements

**Eligibility Criteria**:

- Must maintain KYC verification status
- Cannot be blacklisted for compliance reasons
- Wallet must be whitelisted by admin before maturity
- Identity registry verification required

**Payment Process**:

- Automatic execution via cloud wallet system
- Batch processing for multiple investors
- Transaction confirmation via blockchain events
- Payment notifications sent to investors

## Compliance & Support

### Notifications & Communication

**Payment Notifications**:

- Email alerts for yield distribution payments
- Push notifications for maturity payment processing
- SMS notifications for critical account status changes
- In-app notification center with payment history

**Platform Updates**:

- Bond status changes (active, paused, matured)
- New investment opportunities
- Regulatory compliance updates
- Platform maintenance notifications

### Support & Documentation

**Help Resources**:

- FAQ section covering common investor questions
- Step-by-step guides for wallet setup and KYC
- Video tutorials for investment process
- Troubleshooting guides for common issues

**Contact Support**:

- In-platform support ticket system
- Live chat for immediate assistance
- Email support for complex issues
- Phone support for high-value investors

## Contracts Dashboard (FR-DS-1)

**NEW FEATURE**: Dedicated tab for viewing and managing signed subscription agreements

### Contracts Tab Navigation

**Navigation Location**: Main dashboard navigation bar - "Contracts" tab

**Page Access**: `/contracts` or `/user/subscription-agreements`

### Signed Agreements List

**API Endpoint**: `GET /user/subscription-agreements`

**Display Format**: List of subscription agreements with the following information per agreement:

```
Agreement Name: Baltic Pine Forest Investment Agreement
Date Signed: 1/15/2024
Bond Name: Baltic Pine Forest
Bond Status: Active
[View Document] [Download Document]
```

**User Experience Features**:

- **View Document**: Opens PDF in new browser tab for immediate viewing
- **Download Document**: Triggers direct PDF download to user's device
- Sort by date signed (newest first by default)
- Sort by agreement name (alphabetical)
- Filter by bond status (Active, Matured, etc.)
- Search by agreement name or bond name
- Pagination: 20 agreements per page
- Empty state message when no contracts signed yet
- Document access validation (authenticated users only)
- Secure S3 URL handling with appropriate permissions

## Implementation Questions & Open Items

### Whitelist Process Clarification (FR-BT-5)

**Outstanding Questions**:

- Is whitelist status automatic after KYC verification completion?
- Do investors need to request whitelist approval separately?
- What specific triggers cause admin to whitelist an investor?
- Should there be a whitelist application UI flow for investors?
- Are there additional compliance requirements beyond basic KYC?

**Current Implementation Gap**:

- Investor-facing process for achieving whitelist status undefined
- May require additional UI components or automatic workflow

### KYC Provider Integration Details (UNCLEAR IMPLEMENTATION)

**Partnership with Digital Trust Solutions**:

- Upwood partnered with Digital Trust Solutions (<https://www.digitaltrustsolutions.nl/>)
- DTS is already an authorized Concordium Identity Provider
- Goal: Single KYC process for both Concordium account creation and platform registration

**Outstanding Technical Questions**:

- How to ensure users select DTS (not other Identity Providers) during account creation?
- How to communicate to DTS that a user is registering for Upwood platform?
- How to associate Concordium account address with DTS user record?
- What is the exact DTS API structure for fetching comprehensive KYC data?
- How to verify that a Concordium account was created via DTS specifically?

**Current Problem**:

- Existing Web3Id presentations provide limited data (`firstName`, `lastName`, `nationality`)
- Insufficient for regulatory compliance requirements
- Missing: full address, date of birth, identification documents, sanctions screening

**Proposed Flow (Implementation Unclear)**:

1. User downloads Concordium Browser Wallet
2. Creates account specifically through DTS Identity Provider
3. Platform verifies account was DTS-created
4. Backend fetches comprehensive KYC data from DTS API
5. User registered with full compliance information

## M5 Milestone Interim Approach

### Current Development Strategy

Due to unclear external KYC integration details, M5 development continues with:

**Admin-Created Users**:

- Admin uses `POST /admin/user/register` endpoint (defined in user-auth-backend.md)
- Bypasses KYC presentation verification entirely
- Admin provides user details directly: `firstName`, `lastName`, `nationality`
- Users immediately whitelisted in identity registry
- Users can login and access all platform functionality

**User Experience**:

1. Admin creates user account with Concordium address and basic details
2. User receives login credentials (email and password)
3. User accesses platform immediately without registration flow
4. Full investment, yield, and maturity workflows available

**Benefits for Development**:

- Allows testing of complete investment workflows
- Frontend and backend integration validation
- Bond lifecycle testing with real users
- Performance and user experience optimization

**Limitations**:

- No self-service registration
- Limited KYC data collection
- Manual user creation process
- No external Identity Provider integration

### Migration to Full KYC Integration

**Phase 1 (Current M5)**:

- Admin-managed user creation
- Basic user information storage
- Core platform functionality testing

**Phase 2 (Future Milestone)**:

- DTS API integration implementation
- Self-service registration workflow
- Comprehensive KYC data collection
- Automated compliance verification

**Phase 3 (Production Ready)**:

- Real-time KYC monitoring
- Advanced compliance features
- Regulatory reporting capabilities
- Complete external Identity Provider integration

### Enhanced User Experience Features

**Notification System**:

- Email/push notification implementation for yield payments?
- How do investors receive confirmation of maturity payment processing?
- Is there a comprehensive payment history and statement feature?
- How are failed payments (due to blacklist/KYC issues) communicated?

**Portfolio Management**:

- Advanced portfolio analytics and reporting
- Tax document generation and export
- Investment performance tracking over time
- Secondary market trading interface (if implemented)
