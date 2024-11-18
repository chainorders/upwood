# Frontend

## Admin Sections

### Identity Registry

#### Identities list (Users list)

- Fields
  - Cognito Id
  - Account Address
  - Email
  - Creation Date
  - KYC Done
  - Is Admin
  - Is Blockchain Admin
- Actions

  - [Mark KYC](./contracts/identity-registry/src/identities.rs) `rwa_identity_registry.registerIdentity` :heavy_check_mark: (only when KYC not done)
  - [Un Mark KYC](./contracts/identity-registry/src/identities.rs) `rwa_identity_registry.deleteIdentity` :heavy_check_mark: (only when KYC done)
  - [Delete](./backend/upwood/src/api/user.rs) :heavy_check_mark: (Only when KYC not done)

#### Identity Registry Details

- API `/admin/identity_registry/contract` :heavy_check_mark:
- Fields
  - Contract Owner
    - Api `/admin/users/account_address/:account_address` :heavy_check_mark:
    - [Popup](#user-details-component)
  - Contract Address
  - Identity Count
  - Agents List
    - Fields
      - Agent Address
        - Api `/admin/users/account_address/:account_address` :heavy_check_mark:
        - [Popup](#user-details-component)
- Actions
  - Link to [Identities List](#identities-list-users-list)
  - CCD Scan Contract Link

### Carbon Credit Token

#### Carbon Credit Details

- Api `/admin/carbon_credits/contract` :heavy_check_mark:
- [Page](#rewardable-token-details-component)

### EuroE Token

#### EuroE Details

- Fields
  - Contract Address
  - Token Id
- Actions
  - Transfer As Reward
    - [Popup](#transfer-reward-component)

### Tree FT (Fungible Token)

#### Tree FT Details

- Api `GET` `/admin/tree_ft/contract` :heavy_check_mark:
- [Page](#rewardable-token-details-component)

### Tree NFT

#### Tree NFT Details

- Api `GET` `/admin/tree_nft/contract` :heavy_check_mark:
- Fields
  - Contract Address
  - Tokens Count
  - Unique Metadata Count
- Actions
  - Link to Tokens List (CCD Scan)

### Tree NFT Metadata

### Forest Project

## User Sections

### Active Projects

- List of Forest Projects
  - API `/forest_projects/list_by_state/funding/:page` :heavy_check_mark:
  - List of [Forest Project Card Component](#forest-project-card-component)
  - Actions
    - Details
    - Invest
      - [Popup](#invest-component)

### Funded Projects

### Investment Portfolio

### Legal Forest Project Contracts

### Wallet Management

### News & Updates

### Support

### Settings

### Logout

## Reusable Components

### Rewardable Token Details Component

- Fields
  - Total Supply
  - Unique Holder Count
  - Contract Address (Link to Contract page on CCD Scan)
  - Token Id (Link to Tokens Page on CCD Scan)
  - List of Agents
    - Fields
    - Agent Address
    - Agent Role CSV
- Actions
  - Transfer As Reward
    - [Popup](#transfer-reward-component)
  - Mint (Only if current account address is mint agent)
    - [Popup](#fungible-token-mint-component)

### Transfer Reward Component

- Fields
  - Amount
  - Dropdown (Select Forest Project) #api
- Actions
  - Transfer #txn

### Fungible Token Mint Component

- Fields
  - Amount
  - To Address
- Actions
  - Mint #txn

### User Details Component

### Forest Project Card Component

- Details available on UI Figma
