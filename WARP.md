# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

Upwood's Concordium RWA (Real World Asset) platform for forest project tokenization and environmental asset management. This is a comprehensive blockchain-based system with 5 specialized development workspaces using VS Code dev containers.

## Architecture

### Multi-Workspace Structure
- **contracts/** - Concordium smart contracts (Rust) - forest project tokenization, carbon credits, P2P trading, compliance
- **backend/** - Event processing & REST APIs (Rust + Poem framework + PostgreSQL)
- **frontend-app/** - Main user interface (React 18 + Vite + TypeScript + Tailwind)
- **cdk-deployment/** - AWS infrastructure (TypeScript + CDK v2)
- **frontend-dapp/** - Legacy admin interface (React)

Each workspace has its own dev container configuration in `.devcontainer/<workspace>/`

### Key Architectural Patterns
- **Event-Driven**: Backend listens to Concordium blockchain events and processes them via specialized processors
- **Workspace Isolation**: Each service runs in dedicated dev containers with specific toolchains
- **Code Generation**: Backend auto-generates TypeScript API clients for frontend consumption
- **Multi-Chain**: Supports both Concordium testnet and mainnet environments
- **Compliance-First**: Built-in identity registry and access control systems

## Development Commands

### Repository Setup
```bash
git submodule update --init --recursive
# Open specific workspace: Dev Containers: Reopen in Container → Select workspace
```

### Smart Contracts (contracts/)
```bash
yarn build          # Build all contract modules in workspace
yarn test           # Run all contract tests
yarn deploy         # Deploy all contracts to testnet
yarn format         # Format Rust code with nightly formatter
yarn clean          # Clean build artifacts

# Individual contract operations
cd security-sft-multi/
yarn build          # Build specific contract
yarn test           # Test specific contract
yarn deploy         # Deploy specific contract
```

### Backend Services (backend/)
```bash
yarn build                      # Clean release build
yarn test                       # Run all tests
yarn format                     # Format with cargo +nightly fmt

# Event Listener Service
yarn debug:listener             # Run blockchain event processor
yarn watch:listener             # Auto-restart on changes

# API Service  
yarn debug:app-api              # Run REST API server (port 3000)
yarn watch:app-api              # Auto-restart on changes

# API Client Generation
yarn generate:client            # Generate TypeScript client → ../frontend-app/src/apiClient
yarn generate:spec              # Generate OpenAPI spec only

# Database Management (Diesel ORM)
diesel database reset          # Reset and migrate database
diesel migration run           # Run pending migrations
```

### Frontend Application (frontend-app/)
```bash
yarn dev                        # Development server with HMR (port 5173)
yarn build                      # Production build (TypeScript + Vite)
yarn preview                    # Preview production build
yarn lint                       # ESLint checks
yarn lint:fix                   # Auto-fix ESLint issues
yarn format                     # Prettier formatting
```

### Infrastructure (cdk-deployment/)
```bash
yarn build                      # Compile TypeScript
yarn watch                      # Watch mode compilation
yarn test                       # Infrastructure tests

# CDK Operations
yarn cdk deploy                 # Deploy to AWS
yarn cdk deploy --all           # Deploy all stacks
yarn cdk diff                   # Show deployment differences
yarn cdk synth                  # Generate CloudFormation templates
yarn cdk bootstrap             # Bootstrap CDK in AWS account
yarn cdk destroy               # Destroy infrastructure
```

## Testing Strategy

### Contract Testing
- Each contract has comprehensive unit tests
- Integration tests in `contracts/integration-tests/`
- Contracts use Concordium smart contract testing framework

### Backend Testing
- Unit tests for individual processors and API endpoints
- Integration tests using shared test utilities in `shared_tests/`
- Database testing with Diesel migrations

### Frontend Testing
- Component testing planned with React Testing Library
- API integration tests against generated client

## Key Development Patterns

### Blockchain Event Processing
- Backend `events_listener` monitors smart contract events
- Contract-specific processors in `backend/events_listener/src/processors/`
- Events update PostgreSQL database to maintain blockchain state synchronization

### API Client Generation Workflow
1. Backend generates OpenAPI spec from Rust API definitions
2. TypeScript client auto-generated using `openapi-typescript-codegen`
3. Frontend imports generated client from `src/apiClient/`

### Multi-Environment Support
- Testnet: `CONCORDIUM_NODE_URI=https://grpc.testnet.concordium.com:20000`
- Mainnet: `CONCORDIUM_NODE_URI=https://grpc.mainnet.concordium.software:20000`
- Environment-specific configurations in each workspace

### Wallet Integration
- Contracts workspace uses `default_account.export` for deployments
- Frontend uses `@concordium/browser-wallet-api-helpers` for user wallets

## Database Architecture

### Backend Database (PostgreSQL)
- Runs automatically in backend dev container (localhost:5432)
- Managed via Diesel ORM with migrations in `backend/shared/migrations/`
- Shared models in `backend/shared/src/db/` used across services
- Connection configured via `DATABASE_URL` environment variable

## Deployment & Infrastructure

### AWS CDK Stack Organization
- **Cognito**: User authentication and management
- **ECS**: Containerized backend services (API + Event Listener)
- **RDS**: PostgreSQL database
- **API Gateway**: REST API proxy with CORS
- **S3 + CloudFront**: Frontend application distribution
- **VPC**: Private networking with service discovery

### Container Architecture
- Each workspace builds its own Docker image
- Production containers optimized for specific service requirements
- Development containers include full toolchain and debugging capabilities

## Configuration Requirements

### Apple Silicon Compatibility
For M1/M2 Macs, update `VARIANT` to `"bullseye"` in:
- `.devcontainer/contracts/devcontainer.json`
- `.devcontainer/backend/docker-compose.yml` 
- `.devcontainer/frontend-dapp/devcontainer.json`

### Required Files
- `.devcontainer/contracts/default_account.export` - Concordium wallet for contract deployments
- `.devcontainer/cdk-deployment/aws_accessKeys.csv` - AWS credentials for infrastructure deployment

## Smart Contract Ecosystem

### Core Contracts
- **identity-registry**: Access control and user verification (whitelist/blacklist)
- **security-sft-multi**: Forest project token representation (main asset contract)
- **security-sft-single**: Carbon credit tokenization
- **security-p2p-trading**: Direct trading marketplace
- **security-mint-fund**: Investment fund and bond management

### Contract Interaction Flow
1. Identity verification through identity-registry
2. Forest project tokenization via security-sft-multi
3. Carbon credit creation using security-sft-single
4. P2P trading through security-p2p-trading
5. Investment fund participation via security-mint-fund

## Environment Variables

### Backend Services
- `CONCORDIUM_NODE_URI`: Blockchain node endpoint
- `DATABASE_URL`: PostgreSQL connection string
- `WEB_SERVER_ADDR`: API server bind address
- `NETWORK`: blockchain network (testnet/mainnet)

### Frontend Application  
- `VITE_API_BASE_URL`: Backend API endpoint
- `VITE_CONCORDIUM_NODE_URL`: Blockchain node endpoint
- `VITE_NETWORK`: Network identifier
- `VITE_COGNITO_USER_POOL_ID`: AWS Cognito configuration
- `VITE_COGNITO_CLIENT_ID`: AWS Cognito client ID
