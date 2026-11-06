# Implementation Plan: [FEATURE]

**Branch**: `[###-feature-name]` | **Date**: [DATE] | **Spec**: [link]

**Input**: Feature specification from `/specs/[###-feature-name]/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command; its definition describes the execution workflow.

## Summary

[Extract from feature spec: primary requirement + technical approach from research]

## Technical Context

**Language/Version**: Rust 1.75+ (from quickstart.md), Axum 0.7, SQLx 0.7, tokio 1.37, serde 1.0, cors 0.1

**Primary Dependencies**: sqlx (postgres feature), tokio (full feature), axum 0.7, serde 1.0 (derive), cors 0.1

**Storage**: PostgreSQL biosouk_db with schemas platform (users/roles/audit), commerce (products/categories/inventory), traceability (producers/farms/certifications/batches) — SQLx for data access

**Testing**: cargo test, cargo clippy -- -D warnings, cargo fmt --check (from .github/workflows/ci.yml with Rust 1.96)

**Target Platform**: Linux (CI: ubuntu-latest), Docker Compose for local development, local PostgreSQL instance — per quickstart.md prerequisites

**Project Type**: Modular monolith (from docs/08-implementation-plan.md: "BioSouk will be implemented as a modular monolith") — single repository, single backend application, single PostgreSQL database, separate domain schemas

**Performance Goals**: SC-001: Users can complete registration and authentication flow end-to-end in under 2 minutes; SC-002: System supports 100 concurrent users authenticating simultaneously without degradation; SC-003: Product search returns results in under 1 second for datasets up to 1000 products with category/farm/organic filters; SC-004: Inventory concurrency safety — no overselling when 100 concurrent purchase requests are processed for the same product; SC-007: Database migrations execute successfully and can be rolled back safely against a real PostgreSQL instance; SC-008: All five milestones (SM-2.1 through SM-2.5) meet their acceptance criteria before sprint completion

**Constraints**: UUID primary keys for all entities (from database design §3.2); NUMERIC(19,4) for monetary values, NOT floating-point (from database design §3.4 and API specification §301); JWT-based authentication with configurable expiration (from API specification §4.1-4.3); RBAC middleware with roles CUSTOMER, FARMER, SELLER, ADMIN (from API specification §4.2-4.3); Decimal-safe monetary representations, NOT float/double (from API specification §301); Organic certification status derived from validated domain records, NOT trusted from client input (from API specification §303); No sensitive payment credentials stored in application logs (from API specification §495); Cross-context ownership boundaries enforced at application level, NOT solely database constraints (from database design §1276-1278); No direct database access from API handlers — business logic in application/domain layers (from API specification §578-580); Input validation on all incoming requests before executing use cases (from API specification §344-359); Error responses follow standard format, NO internal details exposed to clients (from API specification §367-416); Payment operations MUST support idempotency with Idempotency-Key header (from API specification §464-486); Sensitive information MUST NOT appear in application logs (from API specification §505);

**Scale/Scope**: Sprint 2 = 5 bounded contexts (Platform, Commerce, Traceability per constitution-approved domains: Platform Domain, Commerce Domain, Traceability Domain), 15 functional requirements (FR-001 through FR-015), 8 success criteria (SC-001 through SC-008), 5 user stories (P1-P3 priority), core domain entities: User, Role, Permission (Platform), Product, Category, InventoryItem, InventoryReservation (Commerce), Producer, Farm, Certification, Batch (Traceability), SQLx migrations for schema management, JWT access/refresh tokens, Argon2id/bcrypt password hashing

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Gate 1 — Domain ownership assigned**: PASS. Feature implements entities across three approved bounded contexts: Platform Domain (users, roles, permissions, authentication, authorization, audit per §99-116), Commerce Domain (products, categories, inventory per §127-137), and Traceability Domain (producers, farms, origins, batches, certifications, recalls per §164-178). Each domain explicitly owns its business rules, aggregates, entities, persistence schema, and domain events per §52-59. No feature implemented without explicit domain ownership per §49.

**Gate 2 — Bounded context identified**: PASS. Constitution lists eight approved domains (Platform, Customer, Commerce, Order, Traceability, Operations, Engagement, Analytics per §39-47); feature implements three of them (Platform, Commerce, Traceability) with clear ownership assignment. No feature may be implemented without assigning domain ownership per §49.

**Gate 3 — Clean architecture dependencies**: PASS. Domain layer contains business rules per §63-73; dependencies must point inward per §66-72; domain layer does NOT depend on Axum, SQLx, PostgreSQL, React, or external service implementations per §73. Application layer orchestrates use cases; infrastructure layer (SQLx, PostgreSQL) implements technical adapters.

**Gate 4 — Security requirements met**: PASS. JWT authentication per §228-231; RBAC with Admin/Producer/Customer roles per §236-244; input validation per §251; Argon2id password hashing per §229; secrets management per §253; audit logging per §255; forbidden patterns avoided: no plaintext passwords, no hardcoded credentials, no production secrets in source control, no trusting frontend authorization per §262-269.

**Gate 5 — Modular monolith architecture**: PASS. Single repository (§80), single backend application (§81), single PostgreSQL database (§82), separate domain schemas (§83-84). Modules communicate through public application interfaces, domain events, explicit integration contracts per §87-92. Direct access to another domain's private implementation or database repositories forbidden per §92.

**Gate 6 — Event-driven design readiness**: PASS. Outbox pattern for integration events per §306-332 (minimum fields: event_id, event_type, event_version, aggregate_id, payload, created_at, processed_at, processing_status, retry_count). Domain events for important business activities per §271-305. At-least-once delivery, idempotent processing, retry with backoff per §323-332.

**Complexity Tracking**: No violations detected. All six gates pass without requiring justification or alternative design. No complexity tracking needed.

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)
<!--
  ACTION REQUIRED: Replace the placeholder tree below with the concrete layout
  for this feature. Delete unused options and expand the chosen structure with
  real paths (e.g., apps/admin, packages/something). The delivered plan must
  not include Option labels.
-->

```text
# [REMOVE IF UNUSED] Option 1: Single project (DEFAULT)
src/
├── models/
├── services/
├── cli/
└── lib/

tests/
├── contract/
├── integration/
└── unit/

# [REMOVE IF UNUSED] Option 2: Web application (when "frontend" + "backend" detected)
backend/
├── src/
│   ├── models/
│   ├── services/
│   └── api/
└── tests/

frontend/
├── src/
│   ├── components/
│   ├── pages/
│   └── services/
└── tests/

# [REMOVE IF UNUSED] Option 3: Mobile + API (when "iOS/Android" detected)
api/
└── [same as backend above]

ios/ or android/
└── [platform-specific structure: feature modules, UI flows, platform tests]
```

**Structure Decision**: [Document the selected structure and reference the real
directories captured above]

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
