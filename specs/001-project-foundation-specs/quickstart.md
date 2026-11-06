# Quickstart: Project Foundation Validation

**Generated**: 2026-11-06
**Feature**: Project Foundation Specs (specs/001-project-foundation-specs)
**Purpose**: Runnable validation scenarios to prove the foundation infrastructure works end-to-end

## Prerequisites

- Git installed
- Rust 1.75+ installed (`rustup toolchain install 1.75 && rustup default 1.75`)
- Node.js + npm installed
- Docker + Docker Compose installed
- PostgreSQL installed and running (local or Docker)

## Setup Validation Scenarios

### Scenario 1: Repository Initialization

**Validate** the Rust workspace is properly structured.

**Commands**:

```bash
# Clone or ensure the repository exists
git clone <repository-url>
cd biosouk

# Verify cargo workspace structure
cargo check

# Expected outcome: `cargo check` completes successfully without errors
```

**Expected outcome**: Rust workspace compiles successfully; no compilation errors.

---

### Scenario 2: Bootstrap Backend and Frontend

**Validate** both the Rust backend and React frontend start successfully.

**Commands**:

```bash
# Start the backend (Axum server)
cargo run

# In a separate terminal, start the frontend
npm run dev

# Expected outcome: 
# - Backend server starts and listens on configured address (default: 127.0.0.1:3000)
# - Frontend dev server starts and is accessible at http://localhost:5173
# - Both servers report healthy status
```

**Expected outcome**: Both backend and frontend servers are running and responsive.

---

### Scenario 3: Database Connectivity

**Validate** PostgreSQL connection is established from the backend.

**Commands**:

```bash
# Verify the database connection via health check
curl http://127.0.0.1:3000/health

# Expected outcome: Health endpoint returns HTTP 200 with database connectivity status
```

**Expected outcome**: Health check reports successful PostgreSQL connection.

---

### Scenario 4: CI Pipeline Execution

**Validate** the CI pipeline executes formatting, linting, and initial tests.

**Commands**:

```bash
# Run the CI pipeline locally (equivalent to GitHub Actions)
cargo fmt --check
cargo clippy -- -D warnings
cargo test

# Expected outcome: All checks pass without errors
```

**Expected outcome**: Formatting, linting, and tests all pass.

---

### Scenario 5: Secrets Governance

**Validate** no secrets are present in Git history.

**Commands**:

```bash
# Check git history for common secret patterns
git log --oneline --all | head -5

# Expected outcome: No API keys, passwords, or tokens in commit messages or history
```

**Expected outcome**: Git history is clean of secrets (per constitution: forbidden).

## Validation Summary

All 5 scenarios must pass for the Project Foundation to be considered successfully set up. Each scenario maps to a functional requirement from the specification:

| Scenario | Mapped FR |
|----------|-----------|
| 1 — Repository Initialization | FR-001, FR-002 |
| 2 — Backend/Frontend Bootstrap | FR-005, FR-008 |
| 3 — Database Connectivity | FR-004 |
| 4 — CI Pipeline Execution | FR-006 |
| 5 — Secrets Governance | FR-007 |

## Notes

- These validation scenarios are the **minimum** to verify the foundation is properly set up
- Additional scenario-specific prerequisites may apply based on individual environment
- Full feature implementation validation will be covered in subsequent milestone quickstart guides
- References to contracts and data model details are intentionally omitted from this foundation-level guide (see `data-model.md` for infrastructure entities only)