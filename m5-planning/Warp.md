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

- Use <FUNCTIONALITY>-<WORKSPACE>.md naming, e.g.:
  - bond-contract.md (smart contract functions, access control, payment proof)
  - bond-api.md (REST API endpoints, simple input/output parameters)
  - bond-db.md (database tables, indexes - no queries/migrations)
  - bond-processor.md (event-to-database mapping, references other files)
  - forest-project-api.md (forest project API endpoints)
  - forest-project-db.md (forest project database schema)
- Workflows are role-specific markdown files:
  - admin-workflow.md (workflow steps with API/contract references)
  - investor-workflow.md (workflow steps with API/contract references)
  - compliance-officer-workflow.md (to be added)
- bonds-planning.md serves as an index referencing these files.

**PLANNING CONTENT RULES**

- Contract files: Method descriptions, access control, input/output structs, events, functionality
- API files: Endpoint definitions with simple parameter lists (no complex TypeScript interfaces)
- Database files: Table schemas and indexes only (no queries, migrations, or event mapping)
- Processor files: CONTRACT_NAME: EVENT_NAME format with database operation descriptions
- Workflow files: Step-by-step processes with references to API/contract files (no code implementations)
- Use postsale_token_contract_address as bond identifier (immutable, unique, idempotent)

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
