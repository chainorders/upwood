# Forest Project Backend Component

This document defines the forest project backend component including API endpoints and database schema for managing forest project metadata and linkage to bonds.

## File Locations

### API Layer

- API endpoints: `backend/upwood/src/api/forest_projects.rs`
- Update: `backend/upwood/src/api/mod.rs` to include forest_projects module

### Database Layer (API-Managed)

- Database Models: `backend/shared/src/db/forest_projects.rs`
- Schema: `backend/shared/src/schema.rs`

---

# PART I: API ENDPOINTS

## API Endpoints

### POST /forest-projects (Admin)

Create new forest project

**Input Parameters:**

- name (string)
- description (text)
- location (string)
- area_hectares (decimal)
- carbon_credits_estimated (integer)
- project_start_date (ISO timestamp)
- project_end_date (ISO timestamp)
- certification_body (string, optional)
- metadata (JSON object, optional)

**Response:**

```json
{
  "project_id": "fp_123e4567-e89b-12d3-a456-426614174000"
}
```

### PUT /forest-projects/{id}/bonds (Admin)

Link bond to forest project

**Path Parameters:**

- id (string) - Forest project ID

**Input Parameters:**

- postsale_token_contract_address (string)

**Response:**

```json
{
  "success": true,
  "linked_at": "2024-03-15T10:30:00Z"
}
```

### PUT /forest-projects/{id}/archive (Admin)

Archive project (hides from active projects list)

**Path Parameters:**

- id (string) - Forest project ID

**Response:**

```json
{
  "success": true,
  "archived_at": "2024-03-15T10:30:00Z"
}
```

### GET /forest-projects

List forest projects with filtering

**Query Parameters:**

- status (string, optional: active, archived)
- limit (number, optional, default: 50, max: 1000)
- offset (number, optional, default: 0)

**Response:**

```json
{
  "projects": [
    {
      "id": "fp_123e4567-e89b-12d3-a456-426614174000",
      "name": "Amazon Reforestation Project",
      "location": "Brazil, Amazon Basin",
      "area_hectares": 1000.5,
      "carbon_credits_estimated": 50000,
      "project_start_date": "2024-01-01T00:00:00Z",
      "project_end_date": "2034-12-31T23:59:59Z",
      "status": "active",
      "bond_count": 2
    }
  ],
  "total": 15,
  "page_count": 1
}
```

### GET /forest-projects/{id}

Get detailed project information including linked bonds

**Path Parameters:**

- id (string) - Forest project ID

**Response:**

```json
{
  "project": {
    "id": "fp_123e4567-e89b-12d3-a456-426614174000",
    "name": "Amazon Reforestation Project",
    "description": "Large-scale reforestation initiative in the Amazon rainforest",
    "location": "Brazil, Amazon Basin",
    "area_hectares": 1000.5,
    "carbon_credits_estimated": 50000,
    "project_start_date": "2024-01-01T00:00:00Z",
    "project_end_date": "2034-12-31T23:59:59Z",
    "certification_body": "Verified Carbon Standard",
    "metadata": {
      "species": ["Mahogany", "Cecropia", "Brazil Nut"],
      "expected_survival_rate": 0.85
    },
    "status": "active",
    "created_at": "2024-01-15T08:00:00Z",
    "updated_at": "2024-02-01T14:30:00Z",
    "archived_at": null
  },
  "bonds": [
    {
      "postsale_token_contract_address": "1000,0",
      "bond_name": "Amazon Forest Bond Series A",
      "status": "Active",
      "linked_at": "2024-01-20T09:15:00Z"
    }
  ]
}
```

## Authentication & Authorization

- **Admin Endpoints:** Require valid Cognito JWT token with admin role
- **Public Endpoints:** Forest project list and details can be public for transparency
- **Rate Limiting:** Apply rate limits to prevent abuse

## Security Considerations

- Input validation for all forest project parameters
- Sanitization of location and description fields
- JSON metadata validation to prevent injection attacks
- Audit logging for all administrative actions

---

# PART II: DATABASE SCHEMA

### forest_projects

```sql path=null start=null
CREATE TABLE forest_projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    description TEXT,
    location TEXT NOT NULL,
    area_hectares DECIMAL(12, 4) NOT NULL,
    carbon_credits_estimated INTEGER NOT NULL,
    project_start_date TIMESTAMP WITH TIME ZONE NOT NULL,
    project_end_date TIMESTAMP WITH TIME ZONE NOT NULL,
    certification_body TEXT,
    metadata JSONB,
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'archived')),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    archived_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_forest_projects_status ON forest_projects(status);
CREATE INDEX idx_forest_projects_dates ON forest_projects(project_start_date, project_end_date);
CREATE INDEX idx_forest_projects_location ON forest_projects USING gin (to_tsvector('english', location));
```

### forest_project_bonds

```sql path=null start=null
CREATE TABLE forest_project_bonds (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    forest_project_id UUID NOT NULL REFERENCES forest_projects(id),
    postsale_token_contract_address TEXT NOT NULL,
    linked_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(forest_project_id, postsale_token_contract_address)
);

CREATE INDEX idx_forest_project_bonds_project ON forest_project_bonds(forest_project_id);
CREATE INDEX idx_forest_project_bonds_contract ON forest_project_bonds(postsale_token_contract_address);
```

## Diesel Models

### ForestProject Model

```rust path=null start=null
use chrono::{DateTime, Utc, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = forest_projects)]
pub struct ForestProject {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub location: String,
    pub area_hectares: rust_decimal::Decimal,
    pub carbon_credits_estimated: i32,
    pub project_start_date: DateTime<Utc>,
    pub project_end_date: DateTime<Utc>,
    pub certification_body: Option<String>,
    pub metadata: Option<Value>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub archived_at: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[diesel(table_name = forest_projects)]
pub struct NewForestProject {
    pub name: String,
    pub description: Option<String>,
    pub location: String,
    pub area_hectares: rust_decimal::Decimal,
    pub carbon_credits_estimated: i32,
    pub project_start_date: DateTime<Utc>,
    pub project_end_date: DateTime<Utc>,
    pub certification_body: Option<String>,
    pub metadata: Option<Value>,
}

impl ForestProject {
    pub fn create(new_project: NewForestProject, conn: &mut DbConn) -> QueryResult<Self> {
        diesel::insert_into(forest_projects::table)
            .values(&new_project)
            .get_result(conn)
    }

    pub fn list(
        status_filter: Option<String>,
        limit: i64,
        offset: i64,
        conn: &mut DbConn,
    ) -> QueryResult<(Vec<Self>, i64)> {
        let mut query = forest_projects::table.into_boxed();

        if let Some(status) = status_filter {
            query = query.filter(forest_projects::status.eq(status));
        }

        let projects = query
            .order(forest_projects::created_at.desc())
            .limit(limit)
            .offset(offset)
            .load::<Self>(conn)?;

        let total = forest_projects::table.count().get_result::<i64>(conn)?;

        Ok((projects, total))
    }

    pub fn find_by_id(project_id: Uuid, conn: &mut DbConn) -> QueryResult<Self> {
        forest_projects::table
            .filter(forest_projects::id.eq(project_id))
            .first(conn)
    }

    pub fn archive(project_id: Uuid, conn: &mut DbConn) -> QueryResult<Self> {
        diesel::update(forest_projects::table.filter(forest_projects::id.eq(project_id)))
            .set((
                forest_projects::status.eq("archived"),
                forest_projects::archived_at.eq(Some(Utc::now())),
                forest_projects::updated_at.eq(Utc::now()),
            ))
            .get_result(conn)
    }
}
```

### ForestProjectBond Model

```rust path=null start=null
#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = forest_project_bonds)]
pub struct ForestProjectBond {
    pub id: Uuid,
    pub forest_project_id: Uuid,
    pub postsale_token_contract_address: String,
    pub linked_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = forest_project_bonds)]
pub struct NewForestProjectBond {
    pub forest_project_id: Uuid,
    pub postsale_token_contract_address: String,
}

impl ForestProjectBond {
    pub fn link_bond(
        forest_project_id: Uuid,
        bond_contract_address: String,
        conn: &mut DbConn,
    ) -> QueryResult<Self> {
        let new_link = NewForestProjectBond {
            forest_project_id,
            postsale_token_contract_address: bond_contract_address,
        };

        diesel::insert_into(forest_project_bonds::table)
            .values(&new_link)
            .get_result(conn)
    }

    pub fn get_project_bonds(
        forest_project_id: Uuid,
        conn: &mut DbConn,
    ) -> QueryResult<Vec<Self>> {
        forest_project_bonds::table
            .filter(forest_project_bonds::forest_project_id.eq(forest_project_id))
            .order(forest_project_bonds::linked_at.desc())
            .load(conn)
    }

    pub fn get_bond_projects(
        bond_contract_address: &str,
        conn: &mut DbConn,
    ) -> QueryResult<Vec<Self>> {
        forest_project_bonds::table
            .filter(forest_project_bonds::postsale_token_contract_address.eq(bond_contract_address))
            .load(conn)
    }
}
```

## Integration Notes

### Bond Integration

- Forest projects can be linked to multiple bonds
- Bond information is retrieved from bond blockchain processor database
- API joins forest project metadata with bond data for comprehensive project details

### Carbon Credit Integration

- Forest project metadata includes estimated carbon credit generation
- Future integration with carbon credit tokenization contracts
- Tracking of actual vs estimated carbon credit production

### Reporting and Analytics

- Project performance metrics based on bond success rates
- Geographic distribution analysis via location data
- Carbon credit efficiency analysis (credits per hectare)

### Search and Filtering

- Full-text search on project names and locations
- Date range filtering for project timelines
- Status-based filtering for active vs archived projects
