# WARP.md

This file provides guidance to WARP (warp.dev) when working with the backend services workspace.

## Workspace Overview

Rust-based backend services for blockchain event processing, REST APIs, and database management. Built with Poem web framework, Diesel ORM, and PostgreSQL, providing the core data layer for the Concordium RWA platform.

## Development Environment

### Prerequisites

- Docker Compose for local development
- AWS credentials configured on host under `~/.aws` (default profile)
- Node.js for OpenAPI client generation (optional, for running scripts on host)

### Local Setup

```bash
# One-time
cp .env.example .env
cp backend/upwood/.secure.env.sample backend/upwood/.secure.env  # add secrets

# Run services
docker compose up -d postgres
# In separate terminals
docker compose up --build backend-listener
docker compose up --build backend-api
```

## Core Development Commands

### Building & Testing

```bash
yarn build                      # Clean release build
yarn test                       # Run all tests
yarn format                     # Format with cargo +nightly fmt
```

### Services Management

#### Event Listener Service

```bash
yarn debug:listener             # Run blockchain event processor
yarn watch:listener             # Auto-restart on changes (cargo watch)
```

#### API Service

```bash
yarn debug:app-api              # Run REST API server (port 3000)
yarn watch:app-api              # Auto-restart on changes (cargo watch)
```

### API Client Generation

```bash
yarn generate:client            # Generate TypeScript client → ../frontend-app/src/apiClient
yarn generate:spec              # Generate OpenAPI spec only
yarn generate:app-api-spec      # Generate API specs from upwood_api_specs binary
yarn generate:app-api-client    # Generate and output client to frontend-app
```

## Database Management

### Database Operations (Diesel ORM)

```bash
diesel database reset          # Reset and migrate database
diesel migration run           # Run pending migrations
diesel migration generate      # Create new migration
```

### Database Connection

- **Host**: localhost:5432 (via Docker Compose)
- **Database**: concordium_rwa_dev
- **User**: concordium_rwa_dev_user
- **Connection**: Pre-configured via `DATABASE_URL` environment variable

## Workspace Architecture

### Core Services

#### events_listener/

- **Purpose**: Processes Concordium blockchain events in real-time
- **Executables**: `listener_server` - main event processing daemon
- **Processors**: Contract-specific event handlers in `src/processors/`
- **Key Features**: Blockchain state sync, reorganization handling, event recovery

#### upwood/

- **Purpose**: REST API service for frontend applications
- **Executables**:
  - `upwood_api_server` - main REST API server
  - `upwood_api_specs` - OpenAPI specification generator
- **API Endpoints**: Forest projects, portfolios, transactions, user management
- **Framework**: Poem web framework with OpenAPI support

#### shared/

- **Purpose**: Common database models, migrations, and utilities
- **Database**: Diesel ORM models and schemas in `src/db/`
- **Migrations**: Database schema evolution in `migrations/`
- **Utilities**: Shared business logic and data structures

#### shared_tests/

- **Purpose**: Integration test utilities and framework
- **Usage**: Shared testing infrastructure for cross-service testing

## Development Patterns

### Event Processing Architecture

```rust
// Event processor pattern
pub struct ContractProcessor {
    db_pool: DbPool,
    // processor-specific state
}

impl EventProcessor for ContractProcessor {
    async fn process_event(&self, event: Event) -> Result<()> {
        // Parse blockchain event
        // Update database state
        // Handle business logic
    }
}
```

### API Endpoint Pattern

```rust
// Poem API endpoint pattern
#[OpenApi]
impl ApiEndpoints {
    #[oai(path = "/api/endpoint", method = "get")]
    async fn endpoint(&self, query: Query<Params>) -> Result<Response> {
        // Validate input
        // Query database
        // Return structured response
    }
}
```

### Database Model Pattern

```rust
// Diesel model pattern
#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = table_name)]
pub struct Model {
    pub id: Uuid,
    // model fields
}
```

## Testing Strategy

### Unit Testing

```bash
# Test specific module
cd events_listener/
cargo test processor_tests

# Test with output
cargo test -- --nocapture

# Test specific API endpoints
cd upwood/
cargo test api_tests
```

### Integration Testing

```bash
# Run shared integration tests
cd shared_tests/
cargo test

# Test database migrations
diesel migration redo
```

## Environment Configuration

### Blockchain Configuration

```bash
# Testnet (default for development)
CONCORDIUM_NODE_URI=https://grpc.testnet.concordium.com:20000
NETWORK=testnet

# Mainnet (production)
CONCORDIUM_NODE_URI=https://grpc.mainnet.concordium.software:20000
NETWORK=mainnet
```

### Service Configuration

```bash
WEB_SERVER_ADDR=0.0.0.0:3000    # API server bind address
DATABASE_URL=postgres://...      # PostgreSQL connection (auto-configured)
DEFAULT_BLOCK_HEIGHT=0           # Starting block height for listener
```

## Common Development Tasks

### Adding New Event Processor

1. Create processor in `events_listener/src/processors/`
2. Implement `EventProcessor` trait
3. Register processor in listener server
4. Add corresponding database models in `shared/src/db/`
5. Create migration for new database tables

### Adding New API Endpoint

1. Define endpoint in `upwood/src/api/`
2. Add OpenAPI documentation
3. Implement request/response types
4. Add endpoint to main API router
5. Regenerate TypeScript client for frontend

### Database Schema Changes

```bash
# Create new migration
diesel migration generate migration_name

# Edit migration files (up.sql and down.sql)
# Apply migration
diesel migration run

# Test migration rollback
diesel migration redo
```

### Running Individual Services

```bash
# Run event listener only
cargo run --bin listener_server

# Run API server only  
cargo run --bin upwood_api_server

# Generate API specs
cargo run --bin upwood_api_specs
```

## Debugging & Monitoring

### Logging

```bash
# Run with debug logging
RUST_LOG=debug yarn debug:listener
RUST_LOG=debug yarn debug:app-api

# Service-specific logging
RUST_LOG=events_listener=debug,upwood=info yarn debug:listener
```

### Database Debugging

```bash
# Connect to database directly
psql $DATABASE_URL

# Check migration status
diesel migration pending

# View database tables
\dt
```

## API Development

### OpenAPI Workflow

1. Define endpoints with Poem OpenAPI macros
2. Run `yarn generate:spec` to create specification
3. Run `yarn generate:client` to update frontend TypeScript client
4. Frontend imports generated client from `src/apiClient/`

### Testing API Endpoints

```bash
# Start API server
yarn debug:app-api

# Test endpoints (in another terminal)
curl http://localhost:3000/api/health
curl http://localhost:3000/api/swagger-ui/  # API documentation
```
