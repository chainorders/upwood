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
  - forest-project-backend.md (API endpoints + database schema for forest project management)
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
- Use postsale_token_contract_address as bond identifier (immutable, unique, idempotent)
- Keep blockchain and API concerns clearly separated
- **Blockchain State Optimization**: Avoid including timestamp fields (created_at, updated_at) in smart contract structs unless specifically required for business logic. These fields increase blockchain state size and storage costs. Use blockchain transaction timestamps or block information when temporal data is needed.

**EDIT CONSISTENCY RULE**

- When a new requirement impacts multiple areas (contract, API, DB, processor, workflows), apply edits to all affected planning files together in one change.
- Keep cross-references updated (e.g., when adding fields in bond-db.md, update bond-processor.md event mapping and bond-api.md response models).

**MAINTAIN TECHNICAL PRECISION**

- This is a technical document - everything should have clear, specific meaning
- Avoid vague language and generic statements
- Remove non-specific phrases like "enhanced", "improved", "better" without concrete details
- Every statement should be actionable and technically precise
- Focus on specific functions, files, data structures, and implementation details

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
