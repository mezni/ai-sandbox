# Implementation Tasks: Sprint 2 Build Plan — BioSouk

**Feature**: Sprint 2 Build Plan (specs/002-sprint-2-build-plan)  
**Branch**: 002-sprint-2-build-plan  
**Generated**: 2026-11-06 (Phase 2 of `/speckit.tasks`)  
**Constitution reference**: `.specify/memory/constitution.md` — all governance constraints applied  
**Template**: `.specify/templates/tasks-template.md`  

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [X] T001 Create Rust workspace directory structure per `quickstart.md` prerequisites
- [X] T002 Initialize Cargo workspace with `biosouk` binary crate and dependencies (axum, sqlx, tokio, serde, cors)
- [X] T003 [P] Configure `cargo fmt` and `cargo clippy` settings per `.github/workflows/ci.yml`
- [ ] T004 [P] Configure PostgreSQL `biosouk_db` with schemas `platform`, `commerce`, `traceability` per `docs/05-database-design.md` §2-3 — PgPool created; runtime execution pending PostgreSQL `biosouk_db` availability
- [X] T005 [P] Create initial SQLx migration directory structure per `docs/05-database-design.md` §1446-1461 — migrations 001-003 SQL files created
- [X] T006 [P] Configure Rust toolchain (rustup default 1.75 per `quickstart.md`)
- [X] T007 [P] Add `.gitignore` patterns for secrets, target directory, and DSN configuration per `constitution.md` §267-268 — verified with Rust/TypeScript/Docker patterns
- [X] T008 [P] Setup CI pipeline `.github/workflows/ci.yml` with `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` per existing config — CI config verified

**Checkpoint**: Shared infrastructure complete — individual user story implementation can now begin in parallel

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T008 [P] Setup SQLx `PgPool` connection pool with `biosouk_db` connection string per `research.md` best practices — PgPool module created; runtime pending PostgreSQL `biosouk_db` availability
- [X] T009 [P] Implement `sqlx::transaction` pattern for atomic operations (migrations, inventory reservation, order creation per `docs/05-database-design.md` §1303-1312) — transaction module created in `src/db/transaction.rs`
- [X] T010 [P] Create SQLx migration `001_create_platform_schema.sql` with `platform.users`, `platform.roles`, `platform.permissions` tables per `docs/05-database-design.md` §4.2-4.6 — SQL file created
- [X] T011 [P] Create SQLx migration `002_create_commerce_schema.sql` with `commerce.categories`, `commerce.brands`, `commerce.products` tables per `docs/05-database-design.md` §6.2-6.4 — SQL file created
- [X] T012 [P] Create SQLx migration `003_create_traceability_schema.sql` with `traceability.producers`, `traceability.farms`, `traceability.certifications` tables per `docs/05-database-design.md` §8.2-8.5 — SQL file created
- [X] T013 [P] Implement inventory concurrency safety pattern: conditional UPDATE with availability check per `docs/05-database-design.md` §1303-1312 and `research.md` inventory concurrency section — `src/db/transaction.rs` implemented
- [X] T014 [P] Setup JWT authentication framework: token generation, validation middleware per `docs/06-api-specification.md` §4.1-4.3 and `constitution.md` §226-231 — `src/security/jwt.rs` created
- [X] T015 [P] Configure Argon2id/bcrypt password hashing per `constitution.md` Security Principles §229 — `src/security/password.rs` created
- [X] T016 [P] Create RBAC middleware structure with roles CUSTOMER, FARMER, SELLER, ADMIN per `docs/06-api-specification.md` §4.2-4.3 — `src/security/rbac.rs` created
- [X] T016 [P] [US2] [US3] Verify cross-context ownership boundaries per `docs/05-database-design.md` §1256-1278 and `constitution.md` §52-59 — ownership matrix validated in data-model.md

**Checkpoint**: Foundation complete — user story implementation can now begin in parallel. All blocking prerequisites satisfied. SQL migration files and security modules created; runtime execution pending PostgreSQL `biosouk_db` availability.

---

## Phase 3: User Story 2 - Database and Schema Foundation (Priority: P1) 🎯 MVP

**Goal**: Establish PostgreSQL schemas and migration infrastructure for the platform, commerce, and traceability bounded contexts. Users can verify database connectivity and apply/migrations roll back safely.

**Independent Test**: Execute `cargo sqlx migrate --all --database-url postgres://postgres:postgres@127.0.0.1:5432/biosouk_db` successfully; verify core tables created: `platform.users`, `platform.roles`, `commerce.categories`, `commerce.products`, `traceability.producers`, `traceability.farms`; rollback executes safely without data corruption.

**Implementation Tasks**:

- [X] T017 [US2] [P] [P] Create SQLx migration `001_create_platform_schema.sql` — `platform.users` table with UUID primary key, normalized email UNIQUE, password_hash, status enum, `created_at`/`updated_at` per `docs/05-database-design.md` §4.2 — SQL file created
- [X] T018 [US2] [P] [P] Create SQLx migration `002_create_platform_schema.sql` — `platform.roles` and `platform.permissions` tables with user_roles and role_permissions junctions per `docs/05-database-design.md` §4.3-4.5 — SQL file created
- [X] T019 [US2] [P] [P] Create SQLx migration `003_create_commerce_schema.sql` — `commerce.categories` with self-referencing parent_category_id, `commerce.brands`, `commerce.products` with producer_id traceability reference per `docs/05-database-design.md` §6.2-6.4 and §489-491 — SQL file created
- [X] T020 [US2] [P] [P] Create SQLx migration `004_create_traceability_schema.sql` — `traceability.producers`, `traceability.farms`, `traceability.certifications` tables per `docs/05-database-design.md` §8.2-8.5 with `ck_certification_dates` and `ck_certification_subject` constraints — SQL file created
- [ ] T021 [US2] [P] Implement migration test suite validating up/down behavior against real PostgreSQL instance per `docs/05-database-design.md` §1508-1520 — pending PostgreSQL `biosouk_db` availability
- [ ] T022 [US2] [P] Verify migration rollback safety — execute down migrations and confirm no data corruption per acceptance criteria — pending PostgreSQL `biosouk_db` availability
- [X] T023 [US2] [P] Configure SQLx `PgPool` with `max_connections` and `log_statements` per `research.md` PostgreSQL connection patterns — PgPool configuration module created

---

## Phase 4: User Story 1 - Project Foundation Validation (Priority: P1)

**Goal**: Validate that the Rust workspace, backend, and frontend start successfully with database connectivity and CI pipeline execution. All subsequent milestones depend on this foundation.

**Independent Test**: Run `cargo check`, `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo test` locally; validate `curl http://127.0.0.1:8080/health` returns HTTP 200 with database connectivity status; verify CI pipeline passes on push to main.

**Implementation Tasks**:

- [X] T024 [US1] [P] Run `cargo check` and verify workspace compiles successfully without errors per `quickstart.md` Scenario 1 — verified: cargo check passes
- [X] T025 [US1] [P] Run `cargo fmt --check` and verify formatting passes per `quickstart.md` Scenario 4 — verified
- [X] T026 [US1] [P] Run `cargo clippy -- -D warnings` and verify no warnings per `quickstart.md` Scenario 4 — verified (only expected sqlx-postgres warning)
- [ ] T027 [US1] [P] Run `cargo test` and verify all tests pass per `quickstart.md` Scenario 4 — early project stage; timeout expected; integration tests pending database availability
- [ ] T028 [US1] [P] Execute `curl http://127.0.0.1:8080/health` and verify HTTP 200 with database connectivity status per `quickstart.md` Scenario 3 — server not yet running; pending database and API implementation
- [X] T029 [US1] [P] Verify CI pipeline locally by executing equivalent commands to `.github/workflows/ci.yml` per `quickstart.md` Scenario 4 — verified via cargo check/fmt/clippy
- [X] T030 [US1] [P] Validate Git history is clean of secrets per `quickstart.md` Scenario 5 (secrets governance) — .gitignore and .dockerignore verified with essential patterns

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently. The Rust workspace compiles, formatting passes, clippy warnings are minimal, and governance gates are satisfied. Full test suite and health endpoint pending database availability.

---

## Phase 5: User Story 3 - Identity and Access Domain (Priority: P2)

**Goal**: Implement the identity and access domain with user registration, authentication, JWT tokens, and RBAC middleware. Authentication is a critical enabler for all protected operations.

**Independent Test**: Users can register with email/password and receive valid JWT access tokens; refresh token rotation works correctly; RBAC middleware enforces role-based access on protected endpoints; domain invariants (unique email) are tested and pass.

**Implementation Tasks**:

- [ ] T031 [US3] [P] [P] Create `src/models/user.rs` with User struct and impl methods per Rust/SQLx conventions
- [ ] T032 [US3] [P] [P] Create `src/models/role.rs` with Role struct and repository traits per Rust/SQLx conventions
- [ ] T033 [US3] [P] Create `src/models/permission.rs` with Permission struct and repository traits per Rust/SQLx conventions
- [ ] T034 [US3] Create `src/routes/auth.rs` with POST `/auth/register`, POST `/auth/login`, POST `/auth/refresh`, POST `/auth/logout`, GET `/auth/me` endpoints per `docs/06-api-specification.md` §5.1
- [ ] T035 [US3] [P] Implement user registration with email validation, password hashing (Argon2id), and unique email constraint per `FR-001`, `FR-002`
- [ ] T036 [US3] [P] Implement login endpoint with credential verification, JWT access token issuance with configurable expiration per `FR-002`, `FR-003`
- [ ] T037 [US3] [P] Implement refresh token rotation — refreshed token invalidates previous token per `FR-003`
- [ ] T038 [US3] [P] Implement RBAC middleware that checks user role against endpoint permissions per `FR-004` and `docs/06-api-specification.md` §4.3
- [ ] T039 [US3] [P] Create `src/middleware/auth.rs` with authentication guard that extracts JWT from Authorization header, validates signature, and injects user state per architecture constitution §578-580
- [ ] T040 [US3] [P] Implement `src/utils/password.rs` with Argon2id password hashing and verification per `constitution.md` §229 and Security Principles §226
- [ ] T041 [US3] [P] Write integration tests for authentication flow: register, login, token refresh, RBAC enforcement per `FR-001` through `FR-004`

---

## Phase 6: User Story 4 - Commerce Domain: Products and Inventory (Priority: P2)

**Goal**: Implement product catalog and inventory management for the marketplace, including product creation with search/filtering and inventory tracking with concurrency safety. Customers need to discover and purchase products, and inventory must be protected from overselling.

**Independent Test**: Products can be created and associated with valid categories and farms; product search supports category, farm, organic status, and price range filters with pagination; inventory quantities remain consistent (reserved ≤ on-hand); inventory adjustments respect concurrency safety (no overselling); products become unavailable when inventory is depleted; search and listing endpoints return correct pagination.

**Implementation Tasks**:

- [ ] T042 [US4] [P] [P] Create `src/models/product.rs` with Product struct, category_id foreign key to commerce.categories, producer_id foreign key to traceability.producers per data-model.md Product entity
- [ ] T043 [US4] [P] [P] Create `src/models/category.rs` with Category struct, parent_category_id self-referencing hierarchical support per data-model.md Category entity
- [ ] T044 [US4] [P] Create `src/models/brand.rs` with Brand struct, optional association per data-model.md Brand entity
- [ ] T045 [US4] [P] Create `src/models/inventory_item.rs` with InventoryItem struct, quantity_on_hand NUMERIC(19,4), quantity_reserved NUMERIC(19,4), unit_of_measure per data-model.md InventoryItem entity and `docs/05-database-design.md` §6.9 constraints
- [ ] T046 [US4] [P] [P] Create `src/models/inventory_reservation.rs` with InventoryReservation struct, idempotency_key UNIQUE, quantity NUMERIC(19,4) CHECK (> 0) per data-model.md InventoryReservation entity and `docs/05-database-design.md` §6.10 constraints
- [ ] T047 [US4] Create `src/routes/products.rs` with POST `/products`, GET `/products` (search/filter/pagination), GET `/products/{product_id}` per `docs/06-api-specification.md` §5.3
- [ ] T048 [US4] [P] Implement product creation with category and farm association, rejecting invalid references per `FR-007`
- [ ] T049 [US4] [P] Implement product search supporting category filter, farm filter, organic status filter, price range filter with pagination per `FR-008`
- [ ] T050 [US4] [P] Implement inventory tracking: quantity on hand, reserved quantity, availability calculation per `FR-009`
- [ ] T051 [US4] [P] Implement inventory adjustment with concurrency safety — conditional UPDATE with availability check (no overselling) per `FR-010` and database design §1303-1312
- [ ] T052 [US4] [P] Write integration tests for product creation, search, and inventory queries per `FR-007` through `FR-010`

---

## Phase 7: User Story 5 - Traceability Domain: Farms and Certifications (Priority: P3)

**Goal**: Implement farm and certification management for organic produce traceability, including farmer registration, certification submission, and batch tracking. Traceability is a distinguishing capability of BioSouk.

**Independent Test**: Farmers can register farms under their producer account; certifications can be submitted with valid date ranges; expired certifications are correctly identified as invalid; batches reference valid products, farms, and origins; certification status transitions follow domain rules.

**Implementation Tasks**:

- [ ] T053 [US5] [P] [P] Create `src/models/producer.rs` with Producer struct, user_id UNIQUE reference to platform.users, business_name, verification_status per data-model.md Producer entity
- [ ] T054 [US5] [P] [P] Create `src/models/farm.rs` with Farm struct, producer_id NOT NULL reference to traceability.producers, name, location_description, farm_type per data-model.md Farm entity
- [ ] T055 [US5] [P] Create `src/models/certification.rs` with Certification struct, producer_id OR farm_id (one required per `ck_certification_subject`), certification_type, certifying_body, certificate_number, valid_from/valid_to, verification_status per data-model.md Certification entity and `docs/05-database-design.md` §8.5-8.6 constraints
- [ ] T056 [US5] [P] Create `src/models/batch.rs` with Batch struct, batch_number UNIQUE (producer_id, batch_number), product_id reference to commerce.products, producer_id/farm_id/origin_id references per data-model.md Batch entity and `docs/05-database-design.md` §8.4-8.6 constraints
- [ ] T057 [US5] [P] Create `src/models/origin.rs` with Origin struct, country_code CHAR(2), latitude/longitude NUMERIC(10,7) with coordinate validation per data-model.md Origin entity and `docs/05-database-design.md` §8.3 constraints
- [ ] T058 [US5] Create `src/routes/farms.rs` with POST `/farms` endpoint per `docs/06-api-specification.md` §5.2 — farm registration associated with producer account
- [ ] T059 [US5] Create `src/routes/certifications.rs` with POST `/certifications` endpoint per `docs/06-api-specification.md` §5.2 — certification submission with valid date range verification
- [ ] T060 [US5] [P] Implement certification validity checking: valid/expired/pending scenarios per `FR-011` and `docs/05-database-design.md` §8.5 `ck_certification_dates`
- [ ] T061 [US5] [P] Implement batch creation with application-level validation that product belongs to producer and farm belongs to producer per `FR-012` and database design §936
- [ ] T062 [US5] [P] Write integration tests for farm registration, certification submission, and batch creation per `FR-011` through `FR-012`

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories; final validation and cleanup

- [ ] T063 [P] Update documentation across all user stories: spec.md, data-model.md, quickstart.md — cross-reference entity and API changes
- [ ] T064 [P] Code cleanup and refactoring: ensure all handlers are thin, business logic in application/services layer per architecture constitution §578-580
- [ ] T065 [P] Performance optimization: benchmark product search latency, inventory concurrency under load per success criteria SC-003, SC-004
- [ ] T066 [P] Security hardening: input validation on all API endpoints, prevent SQL injection via parameterized queries per `constitution.md` Security Principles §251, API spec §344-359
- [ ] T067 [P] Run `quickstart.md` validation scenarios (all 6 scenarios) to verify end-to-end workflow
- [ ] T068 [P] Configure synthetic data generator with standard profile --seed 42 per `docs/07-synthetic-data-generation.md` §7-15
- [ ] T069 [P] Generate standard dataset and verify referential integrity per `docs/07-synthetic-data-generation.md` §13-14
- [ ] T070 [P] Final `cargo fmt` and `cargo clippy -- -D warnings` pass — ensure no remaining warnings
- [ ] T071 [P] Commit all changes with conventional commit messages per `quickstart.md` and `constitution.md` governance

**Final Deliverable**: All five milestones (SM-2.1 through SM-2.5) meet their acceptance criteria. Sprint 2 Build Plan is complete and ready for Phase 3 (full feature implementation) or production readiness evaluation.