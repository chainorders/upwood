# M5 Planning Rules - Concordium RWA Platform

## Planning System Rules for WARP

When working with M5 planning documents in this directory (`m5-planning/`), ALWAYS follow these rules:

### 🤖 WARP AI Assistant Behavior Rules

1. **Requirement Analysis Mode**: When user mentions "business requirement" or "requirement document", automatically:
   - Examine current codebase implementation in relevant files
   - Create/update appropriate requirement chunk file (bonds.md, yield.md, etc.)
   - Use the `_template_requirement.md` as the base structure
   - Document specific todos with exact file paths and function names

2. **Todo Implementation Mode**: When user references "todo #X in m5-planning/[file].md":
   - Read the specific todo item from the planning document
   - Examine the current implementation in the specified files
   - Execute the change with full context understanding
   - Mark the todo as complete after successful implementation

3. **Planning Document Creation**: Always use this structure for new requirement chunk files:
   - Copy from `_template_requirement.md`
   - Replace all [PLACEHOLDER] values with specific content
   - Include exact file paths: `contracts/[contract-name]/src/lib.rs`
   - Include specific function/struct names from actual codebase
   - Provide detailed change descriptions, not just high-level summaries

4. **Code Analysis Requirements**: Before creating todos, ALWAYS:
   - Use `read_files` to examine current implementation
   - Use `grep` or `search_codebase` to understand existing patterns
   - Identify specific functions, structs, and code sections that need changes
   - Understand dependencies between different parts of the system

5. **Progress Tracking**: After completing any implementation:
   - Update the relevant planning document with completed status
   - Update `overview.md` with progress information
   - Document any issues or deviations encountered

## 📋 Purpose

This planning system is designed to:
- **Break down business requirements** into manageable development chunks
- **Document code changes** required for each requirement
- **Create actionable todo lists** for implementation
- **Enable targeted development** by pointing to specific todo items
- **Maintain traceability** between business requirements and code changes

## 📁 Directory Structure

```
m5-planning/
├── WARP.md                      # This file - planning rules and workflow
├── _template_requirement.md     # Template for new requirement chunks
├── bonds.md                     # Bond-related requirements and changes
├── yield.md                     # Yield distribution requirements and changes  
├── trading.md                   # P2P trading requirements and changes
├── compliance.md                # Identity & compliance requirements and changes
├── frontend.md                  # Frontend/UI requirements and changes
├── infrastructure.md            # Infrastructure & deployment requirements
└── overview.md                  # High-level overview of all M5 changes
```

## 🔄 Workflow Process

### Phase 1: Requirement Analysis
1. **Review business requirement document** section by section
2. **Identify the requirement chunk** (e.g., bonds, yield, trading)
3. **Create or update** the corresponding `requirement_chunk.md` file
4. **Document requirement IDs** being addressed
5. **Analyze impacted code areas** (contracts, backend, frontend)

### Phase 2: Code Change Documentation
1. **Examine current implementation** in relevant files
2. **Document specific changes needed** in todo list format
3. **Include file paths and function/struct names** for precision
4. **Estimate complexity** and dependencies between changes
5. **Update the overview document** with cross-references

### Phase 3: Implementation Execution  
1. **Point to specific todo item** in specific requirement document
2. **AI/Developer reads the todo** and understands context
3. **Execute the change** with full context of requirement and codebase
4. **Mark todo as complete** and update documentation

## 📝 Document Format Standards

Each requirement chunk document follows this structure:

### Header Section
- **Requirement IDs**: List of business requirement IDs addressed
- **Summary**: Brief overview of the changes
- **Affected Areas**: Which parts of codebase are impacted

### Todo List Sections
Organized by codebase area:
- **Smart Contracts Changes**
- **Backend API Changes** 
- **Frontend UI Changes**
- **Database Schema Changes**
- **Infrastructure Changes**

### Implementation Notes
- **Dependencies**: What must be completed first
- **Risks**: Potential issues or complications
- **Testing Strategy**: How to verify the changes work

## 🎯 Usage Examples

### Starting Development on a Specific Feature
```bash
# Example: Work on bond yield calculation improvements
# 1. Open: m5-planning/bonds.md
# 2. Find: "Update yield calculation algorithm in security-mint-fund contract"
# 3. Execute: Follow the detailed todo with file paths and changes needed
```

### Tracking Progress
```bash
# Check overall progress
cat m5-planning/overview.md

# Check specific area progress  
grep -E "^- \[x\]|^- \[ \]" m5-planning/bonds.md
```

## 🔗 Integration with Development

### With AI Development Assistant
- Point to specific todo: "Implement todo #3 in m5-planning/bonds.md"
- AI reads context and implements with full understanding
- AI can update documentation after completion

### With Manual Development
- Each todo contains enough detail for independent implementation
- File paths, function names, and change descriptions are specific
- Cross-references help understand impact on other areas

## 📊 Tracking and Metrics

### Completion Tracking
- [ ] bonds.md requirements
- [ ] yield.md requirements  
- [ ] trading.md requirements
- [ ] compliance.md requirements
- [ ] frontend.md requirements
- [ ] infrastructure.md requirements

### Priority Levels
- **P0**: Critical for M5 release
- **P1**: Important for M5 release
- **P2**: Nice to have for M5 release
- **P3**: Future consideration

## 🚀 Getting Started

1. **Review this README** to understand the workflow
2. **Examine the template** at `_template_requirement.md`
3. **Start with your first requirement chunk**
4. **Use the template** to create structured documentation
5. **Begin implementation** using the todo lists

---

**Next Steps**: Begin by discussing your business requirement document sections and we'll populate the appropriate requirement chunk files with detailed implementation plans.
