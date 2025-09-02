# Forest Project Planning (Database)

This document defines database planning for forest projects and linkage to bonds.

## Tables

### forest_projects
- id (string, PK)
- name (string)
- description (text)
- location (string)
- area_hectares (decimal)
- carbon_credits_estimated (integer)
- project_start_date (timestamp)
- project_end_date (timestamp)
- certification_body (string, nullable)
- metadata (JSON, nullable)
- status (string: active, archived)
- created_at (timestamp)
- updated_at (timestamp)
- archived_at (timestamp, nullable)

Indexes:
- idx_forest_projects_status
- idx_forest_projects_dates

### forest_project_bonds
- id (string, PK)
- forest_project_id (FK → forest_projects.id)
- postsale_token_contract_address (string)
- linked_at (timestamp)

Indexes:
- UNIQUE(forest_project_id, postsale_token_contract_address)
- idx_forest_project_bonds_project

