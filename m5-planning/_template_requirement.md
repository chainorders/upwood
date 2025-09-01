# [REQUIREMENT_CHUNK_NAME] Requirements - M5 Planning

> **Template Usage**: Copy this template and replace [REQUIREMENT_CHUNK_NAME] with the actual requirement area (e.g., Bonds, Yield, Trading, etc.)

## 📋 Requirement Overview

### Business Requirement IDs Addressed
- [ ] REQ-[ID-1]: [Brief description]
- [ ] REQ-[ID-2]: [Brief description]
- [ ] REQ-[ID-3]: [Brief description]

### Summary
[Provide a 2-3 sentence summary of what this requirement chunk encompasses and its business impact]

### Affected Codebase Areas
- [ ] **Smart Contracts**: [contract-name-1, contract-name-2]
- [ ] **Backend Services**: [service-name-1, service-name-2]  
- [ ] **Frontend Application**: [component/page areas]
- [ ] **Database Schema**: [table/migration changes]
- [ ] **Infrastructure**: [deployment/config changes]

## 🔧 Smart Contracts Changes

### [contract-name] Contract Changes
- [ ] **TODO #1**: [Specific change description]
  - **File**: `contracts/[contract-name]/src/lib.rs`
  - **Function/Struct**: `[specific_function_or_struct_name]`
  - **Change Details**: [Detailed description of what needs to be changed]
  - **Priority**: P[0-3]
  - **Estimated Effort**: [S/M/L/XL]

- [ ] **TODO #2**: [Another specific change]
  - **File**: `contracts/[contract-name]/src/[specific-file].rs`
  - **Function/Struct**: `[specific_function_or_struct_name]`
  - **Change Details**: [Detailed description]
  - **Priority**: P[0-3]
  - **Estimated Effort**: [S/M/L/XL]

## 🖥️ Backend API Changes

### Event Processing Changes
- [ ] **TODO #[N]**: [Specific processor change]
  - **File**: `backend/events_listener/src/processors/[contract_name].rs`
  - **Function**: `[process_function_name]`
  - **Change Details**: [What needs to be updated in event processing]
  - **Priority**: P[0-3]
  - **Estimated Effort**: [S/M/L/XL]

### REST API Changes
- [ ] **TODO #[N+1]**: [API endpoint change]
  - **File**: `backend/app_api/src/handlers/[handler_name].rs`
  - **Endpoint**: `[HTTP_METHOD] /api/v1/[endpoint]`
  - **Change Details**: [What needs to be added/modified in the API]
  - **Priority**: P[0-3]
  - **Estimated Effort**: [S/M/L/XL]

### Database Schema Changes
- [ ] **TODO #[N+2]**: [Database change]
  - **Migration File**: `backend/shared/migrations/[timestamp]_[description].sql`
  - **Tables Affected**: [table_name_1, table_name_2]
  - **Change Details**: [Schema changes needed]
  - **Priority**: P[0-3]  
  - **Estimated Effort**: [S/M/L/XL]

## 🎨 Frontend UI Changes

### New Components/Pages
- [ ] **TODO #[N+3]**: [New component/page]
  - **File**: `frontend-app/src/[path]/[ComponentName].tsx`
  - **Component Type**: [Page/Component/Hook/Utility]
  - **Change Details**: [What UI functionality needs to be built]
  - **Priority**: P[0-3]
  - **Estimated Effort**: [S/M/L/XL]

### Existing Component Updates  
- [ ] **TODO #[N+4]**: [Existing component update]
  - **File**: `frontend-app/src/[existing-path]/[ExistingComponent].tsx`
  - **Function/Hook**: `[specific_function_name]`
  - **Change Details**: [What needs to be modified]
  - **Priority**: P[0-3]
  - **Estimated Effort**: [S/M/L/XL]

### API Integration Changes
- [ ] **TODO #[N+5]**: [API client updates]
  - **Generated Client**: `frontend-app/src/apiClient/[generated-files]`
  - **Custom Hooks**: `frontend-app/src/hooks/[hook-files]`  
  - **Change Details**: [What API integrations need to be added/updated]
  - **Priority**: P[0-3]
  - **Estimated Effort**: [S/M/L/XL]

## ☁️ Infrastructure Changes

### AWS CDK Changes
- [ ] **TODO #[N+6]**: [Infrastructure change]
  - **File**: `cdk-deployment/lib/[stack-name].ts`
  - **Resource Type**: [AWS resource type]
  - **Change Details**: [What infrastructure needs to be added/modified]
  - **Priority**: P[0-3]
  - **Estimated Effort**: [S/M/L/XL]

## 📝 Implementation Notes

### Dependencies
- [List any todos that must be completed before others can start]
- [Cross-references to other requirement chunk dependencies]

### Risks and Considerations
- **Risk 1**: [Description and mitigation strategy]
- **Risk 2**: [Description and mitigation strategy]

### Testing Strategy
- **Unit Tests**: [What needs unit test coverage]
- **Integration Tests**: [What needs integration test coverage]  
- **End-to-End Tests**: [What user journeys need E2E coverage]

### Definition of Done
- [ ] All todos marked complete
- [ ] Code changes implemented and tested
- [ ] API client regenerated (if backend changes made)
- [ ] Documentation updated
- [ ] Deployment tested in staging environment

## 🔄 Progress Tracking

### Overall Progress
- **Total Todos**: [N]
- **Completed**: [X] 
- **Remaining**: [N-X]
- **Blocked**: [Y]

### Last Updated
- **Date**: [YYYY-MM-DD]
- **Updated By**: [Name/AI]
- **Changes Made**: [Brief description of updates]

---

**Usage Instructions**:
1. Copy this template to create a new requirement chunk file
2. Replace all [PLACEHOLDER] values with actual content
3. Fill in specific file paths, function names, and detailed change descriptions
4. Use this for planning conversations and implementation tracking
