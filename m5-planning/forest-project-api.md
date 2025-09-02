# Forest Project Planning (API)

This document defines the forest project API planning (backend/upwood/src/api).

## Endpoints

### POST /forest-projects (Admin)

Create project

Input:

- name, description, location
- area_hectares (decimal)
- carbon_credits_estimated (integer)
- project_start_date, project_end_date (ISO timestamps)
- certification_body (string, optional)
- metadata (JSON object, optional)

Output:

- project_id (string)

### PUT /forest-projects/{id}/bonds (Admin)

Link bond to project

Input:

- postsale_token_contract_address (string)

Output:

- success (boolean)

### PUT /forest-projects/{id}/archive (Admin)

Archive project (hides from Active Projects)

Output:

- success (boolean)
- archived_at (ISO timestamp)

### GET /forest-projects

List projects (filters: active/archived)

Query parameters:

- status (string, optional: active, archived)
- limit (number, optional)
- offset (number, optional)

Output:

- projects (array of project summaries)
- total (number)

### GET /forest-projects/{id}

Get project details including linked bonds

Output:

- project (project details object)
- bonds (array of bond summaries)

## Data Objects

Forest Project Summary:

- id, name, location
- area_hectares, carbon_credits_estimated
- project_start_date, project_end_date
- status, bond_count

Forest Project Details (extends Summary):

- description, certification_body, metadata
- created_at, updated_at, archived_at

## Security

- Admin auth via Cognito

## Indexer Integration

- Read-only endpoints query DB for bond linkage and project metadata
