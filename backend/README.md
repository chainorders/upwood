# 🔧 Backend Services Workspace

**Rust-based backend services for Upwood's Concordium RWA platform, providing blockchain event processing, REST APIs, and database management.**

## 🌟 Overview

This workspace contains backend services written in Rust that process Concordium blockchain events, provide REST APIs for frontend applications, and manage data persistence. The services are built using the Poem web framework and integrate with PostgreSQL databases.

## 🏗️ Directory Structure

```
backend/
├── events_listener/               # Concordium blockchain event processor
│   ├── src/
│   │   ├── bin/                # Event listener executables
│   │   └── processors/         # Contract-specific event processors
│   └── Cargo.toml
├── shared/                       # Shared database models and utilities
│   ├── migrations/            # Database migration files
│   ├── src/
│   │   ├── db/                 # Database connection and models
│   │   └── db_app/             # Application-specific database logic
│   └── Cargo.toml
├── shared_tests/                # Integration test utilities
│   └── src/
├── upwood/                      # Main API service
│   ├── src/
│   │   ├── api/                # REST API endpoints
│   │   ├── bin/                # API server executables
│   │   └── utils/              # Utility functions
│   ├── tests/                 # API integration tests
│   └── Cargo.toml
└── Cargo.toml                   # Workspace configuration
```

## 🚀 Development Environment Setup

### Using VS Code Dev Containers

1. **Open VS Code in repository root**

   ```bash
   cd /path/to/concordium-rwa
   code .
   ```

2. **Open in Dev Container**
   - Press `F1` or `Ctrl+Shift+P`
   - Type: `Dev Containers: Reopen in Container`
   - Select: **backend**

3. **Container Setup**
   - The container automatically installs Rust, PostgreSQL, and Node.js
   - Database starts automatically on port 5432
   - Terminal shows available yarn scripts upon completion

## 🛠️ Available Scripts

All scripts are defined in `package.json` and can be run with `yarn <script>`:

### Development & Building

```bash
yarn build              # Clean build release version
yarn test               # Run all tests
yarn format             # Format Rust code with nightly formatter
```

### Event Listener Service

```bash
yarn debug:listener     # Run blockchain event listener (cargo run --bin listener_server)
yarn watch:listener     # Watch and auto-restart listener on code changes
```

### API Service

```bash
yarn debug:app-api      # Run REST API server (cargo run --bin upwood_api_server)
yarn watch:app-api      # Watch and auto-restart API server on code changes
```

### API Client Generation

```bash
yarn generate:spec         # Generate OpenAPI specification (app-api-specs.json)
yarn generate:client       # Generate TypeScript client for frontend-app
yarn generate:app-api-spec # Generate API specs from upwood_api_specs binary
yarn generate:app-api-client # Generate and output client to ../frontend-app/src/apiClient
```

## 📊 Services Overview

### 📡 Events Listener (`events_listener/`)

**Purpose**: Processes Concordium blockchain events and updates the database

**Key Features**:

- Monitors smart contract events in real-time
- Processes identity registry, trading, and fund events
- Maintains database state synchronization with blockchain
- Handles blockchain reorganizations and recovery

**Executables**:

- `listener_server` - Main event processing daemon

### 🔗 API Service (`upwood/`)

**Purpose**: Provides REST API endpoints for frontend applications

**Key Features**:

- RESTful API using Poem web framework
- Authentication and authorization
- Forest project management endpoints
- Portfolio and transaction history APIs
- OpenAPI specification generation

**Executables**:

- `upwood_api_server` - Main REST API server
- `upwood_api_specs` - OpenAPI specification generator

### 📚 Shared Library (`shared/`)

**Purpose**: Common database models, migrations, and utilities

**Key Features**:

- Database connection management
- Diesel ORM models and schemas
- Database migrations for all services
- Common data structures and utilities

### 🧪 Integration Tests (`shared_tests/`)

**Purpose**: Shared testing utilities and integration test framework

## 📦 Database Setup

The backend uses PostgreSQL with Diesel ORM:

```bash
# Database runs automatically in dev container on localhost:5432
# DATABASE_URL is pre-configured in .devcontainer/backend/.env

# Run migrations (DATABASE_URL already set)
diesel migration run

# Reset database (DATABASE_URL already set)
diesel database reset
```

### Database Connection Details

```bash
# Database runs automatically in backend dev container
# Configuration from .devcontainer/backend/.env:
# Host: localhost
# Port: 5432
# Database: concordium_rwa_dev
# Username: concordium_rwa_dev_user
# Password: concordium_rwa_dev_pswd
# Full Connection String: postgres://concordium_rwa_dev_user:concordium_rwa_dev_pswd@localhost:5432/concordium_rwa_dev

# DATABASE_URL environment variable is automatically set by the dev container
# No need to specify --database-url parameter for diesel commands
```

## ⚙️ Configuration

### Blockchain Configuration

Configure the backend services to connect to different Concordium networks:

```bash
# Testnet configuration (development)
CONCORDIUM_NODE_URI=https://grpc.testnet.concordium.com:20000
NETWORK=testnet

# Mainnet configuration (production)
CONCORDIUM_NODE_URI=https://grpc.mainnet.concordium.software:20000
NETWORK=mainnet
```

## Environment Variables

### General Variables

| Variable             | Description                                        |
| -------------------- | -------------------------------------------------- |
| CONCORDIUM_NODE_URI  | The URI of the Concordium node.                    |
| MONGODB_URI          | The URI of the MongoDB database.                   |
| WEB_SERVER_ADDR      | The address and port the web server is running on. |
| DEFAULT_BLOCK_HEIGHT | The starting block height for the blockchain.      |
| NETWORK              | The network the application is running on.         |

### Module Refs

| Variable                         | Description                                         |
| -------------------------------- | --------------------------------------------------- |
| RWA_COMPLIANCE_MODULE_REF        | The reference for the RWA Compliance module.        |
| RWA_IDENTITY_REGISTRY_MODULE_REF | The reference for the RWA Identity Registry module. |
| RWA_MARKET_MODULE_REF            | The reference for the RWA Market module.            |
| RWA_SECURITY_NFT_MODULE_REF      | The reference for the RWA Security NFT module.      |
| RWA_SECURITY_SFT_MODULE_REF      | The reference for the RWA Security SFT module.      |
| RWA_SPONSOR_MODULE_REF           | The reference for the RWA Sponsor module.           |

### Contract Names

| Variable                            | Description                                     |
| ----------------------------------- | ----------------------------------------------- |
| RWA_IDENTITY_REGISTRY_CONTRACT_NAME | The name of the RWA Identity Registry contract. |
| RWA_COMPLIANCE_CONTRACT_NAME        | The name of the RWA Compliance contract.        |
| RWA_MARKET_CONTRACT_NAME            | The name of the RWA Market contract.            |
| RWA_SECURITY_NFT_CONTRACT_NAME      | The name of the RWA Security NFT contract.      |
| RWA_SPONSOR_CONTRACT_NAME           | The name of the RWA Sponsor contract.           |

### Verifier

| Variable                 | Description                                                 |
| ------------------------ | ----------------------------------------------------------- |
| IDENTITY_REGISTRY        | The identity registry.                                      |
| AGENT_WALLET_PATH        | The path to the agent's wallet.                             |
| VERIFIER_WEB_SERVER_ADDR | The address and port the verifier web server is running on. |

### Sponsor

| Variable                | Description                                                |
| ----------------------- | ---------------------------------------------------------- |
| SPONSOR_WALLET_PATH     | The path to the sponsor's wallet.                          |
| SPONSOR_WEB_SERVER_ADDR | The address and port the sponsor web server is running on. |
| SPONSOR_CONTRACT        | The sponsor contract.                                      |
