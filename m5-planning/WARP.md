# Warp Agent Rules for M5 Planning

## Important Rules

**DO NOT GENERATE COMPLETION REPORTS AUTOMATICALLY**

- Do not generate summary reports or completion summaries unless explicitly requested
- The user will ask for a complete report at the end when needed
- Focus on implementation and planning tasks only
- Wait for explicit request before providing any summary or completion reports

**ALWAYS REFER TO INDIVIDUAL WORKSPACE DOCUMENTATION**

- Always reference individual workspace WARP.md and README.md files for detailed implementation guidance
- Key workspace documentation:
  - [contracts/README.md](../contracts/README.md) and [contracts/WARP.md](../contracts/WARP.md)
  - [backend/README.md](../backend/README.md) and [backend/WARP.md](../backend/WARP.md)
  - Additional workspace documentation as needed
- Use these files to understand existing patterns, conventions, and implementation approaches

**PLANNING FILE STRUCTURE**

The planning files are now organized into two main categories:

**1. Blockchain Components** (Contract + Processor + Database):

- <FUNCTIONALITY>-blockchain.md naming, e.g.:
  - identity-registry-blockchain.md (contract enhancements, event processing, processor database schema)
  - security-sft-multi-blockchain.md (contract enhancements, event processing, processor database schema)
  - bond-blockchain.md (contract functions, event processing, processor database schema)
- Each file contains three parts:
  - PART I: Smart contract (functions, access control, events, state structure)
  - PART II: Event processor (event handling, database operations)
  - PART III: Database schema (processor-managed tables, indexes, Diesel models)

**2. API Components** (REST API endpoints only):

- <FUNCTIONALITY>-api.md naming for blockchain-related APIs:
  - identity-registry-api.md (REST API endpoints for blacklist queries)
  - bond-api.md (REST API endpoints for bond management)
- API components use processor database tables in read-only mode
- Only used when API queries blockchain processor databases

**3. Backend Components** (API + Database for off-chain functionality):

- <FUNCTIONALITY>-backend.md naming, e.g.:
  - user-auth-backend.md (API endpoints + database schema for user registration and authentication)
  - bonds-backend.md (API endpoints + database schema for bond metadata management)
  - yields-backend.md (API endpoints + database schema for yield distribution)
- Each file contains two parts:
  - API Layer: REST endpoints, authentication, validation
  - Database Layer: Schema, indexes, Diesel models
- Used for components that manage their own off-chain data

**4. Workflow Files** (role-specific processes):

- admin-workflow.md (workflow steps with API/contract references)
- investor-workflow.md (workflow steps with API/contract references)
- compliance-officer-workflow.md (to be added)

**PLANNING CONTENT RULES**

**Blockchain Component Files:**

- Contract section: Method descriptions, access control, input/output structs, events, state structure
- Processor section: Event handling logic, database operations, CONTRACT_NAME: EVENT_NAME format
- Database section: Processor-managed table schemas, indexes, Diesel models (no migrations)
- Two database layers: processor-managed (write) and API-managed (read-only)

**API Component Files:**

- Endpoint definitions with simple parameter lists (no complex TypeScript interfaces)
- Read-only access to processor database tables
- Authentication, validation, caching, error handling

**Workflow Files:**

- Step-by-step processes with references to API/blockchain component files
- No code implementations, only process descriptions

**General Rules:**

- Use postsale_token_contract as bond identifier (BIGINT database, Decimal API responses)
- Keep blockchain and API concerns clearly separated
- **Blockchain State Optimization**: Avoid including timestamp fields (created_at, updated_at) in smart contract structs unless specifically required for business logic. These fields increase blockchain state size and storage costs. Use blockchain transaction timestamps or block information when temporal data is needed.

**RUST POEM FRAMEWORK API TYPE RULES:**

**API Layer Type Mapping:**

- **Contract Indices**: Use `Decimal` type for path parameters and request/response bodies in Rust Poem framework
  - Avoids JavaScript precision issues with large integers
  - Prevents framework compatibility problems with u64 types
  - JSON serialization: Contract indices as strings (e.g., `"1234"` not `1234`)
- **Native Token IDs**: Use `String` type for PLT tokens and other native CCD token identifiers
- **All Numeric Values**: Serialize as strings in JSON responses to avoid client-side precision issues

**Database Layer:**

- **Contract Indices**: Store as `BIGINT` for efficient indexing and foreign key relationships
- **All Amounts**: Use `DECIMAL(78, 0)` for high-precision arithmetic
- **Conversion**: Diesel handles automatic type conversion between Rust types and database types

**Internal Rust Types:**

- **Contract Indices**: Use `u64` for internal processing and blockchain operations
- **Type Conversion**: `Decimal::from(u64_value)` and `decimal_value.to_u64()` between layers
- **Smart Contract Layer**: Continue using `ContractAddress` for on-chain operations

**EDIT CONSISTENCY RULE**

- When a new requirement impacts multiple areas (contract, API, DB, processor, workflows), apply edits to all affected planning files together in one change.
- Keep cross-references updated (e.g., when adding fields in bond-db.md, update bond-processor.md event mapping and bond-api.md response models).

**MAINTAIN TECHNICAL PRECISION**

- This is a technical document - everything should have clear, specific meaning
- Avoid vague language and generic statements
- Remove non-specific phrases like "enhanced", "improved", "better" without concrete details
- Every statement should be actionable and technically precise
- Focus on specific functions, files, data structures, and implementation details

**BLOCKCHAIN EVENT OPTIMIZATION**

- Every event depending on its size adds to the blockchain transaction charges so they should be kept to a minimum size
- Events should only contain essential fields needed for database updates and audit trails
- Avoid redundant fields like previous/new balance pairs - use direct values instead
- Prefer single fields over multiple related fields where possible

**PLANNING DOCUMENT STRUCTURE**

- Diesel struct implementations should be removed from planning documents to reduce file size
- Detailed implementations will be handled during the actual coding phase
- Focus on data structures, schemas, and high-level logic in planning documents

**CODE BLOCK FORMATTING FOR READABILITY**

- All planning files should use proper code blocks for technical content:
  - Rust code blocks for smart contract functions, structs, and types
  - TypeScript/JavaScript for API interfaces and frontend code  
  - SQL for database schemas, queries, and operations
  - HTTP for API endpoint examples and request/response formats
  - JSON for data structures and configuration examples
- Use \`\`\`language path=null start=null for planning code blocks
- Wrap parameters, field names, function names, and technical terms in backticks
- Structure technical information using code blocks improves readability during implementation phase

## Planning Development Guidelines

### Clarification Process

- **ALWAYS ask clarifying questions** when business requirements or technical specifications are ambiguous or potentially conflicting
- Questions help ensure accurate implementation planning and prevent misunderstandings
- Better to ask multiple questions upfront than make incorrect assumptions
- Focus on edge cases, state transitions, error handling, and user experience implications

### Identity Registry Rules

- **No "Registered" State**: Only `Whitelisted` and `Blacklisted` states exist in the identity registry
- **Default Behavior**: Addresses not in the registry have no trading restrictions
- **Transfer Logic**: Transfers are blocked only if the recipient is explicitly `Blacklisted`
- **Payment Logic**: Yield and maturity payments go only to explicitly `Whitelisted` addresses
- **Database Storage**: Only store whitelisted and blacklisted entries, not "not in registry" entries
- **API Response**: `getAddressState` returns `Option<AddressState>` where `None` means not in registry

### Business Logic Consistency

- Always verify that planning documents align with business requirements
- When updating one component, check for impacts on related components
- Maintain consistency across blockchain contracts, event processors, APIs, and UI specifications
- Document state transitions clearly for complex workflows

### Technical Architecture Principles

- Event-driven synchronization between blockchain and backend database
- Read-only backend APIs for blockchain state queries
- Direct blockchain transactions for state changes via frontend wallet integration
- Proper error handling and audit trails for all operations
- Use existing database processor patterns for consistency
