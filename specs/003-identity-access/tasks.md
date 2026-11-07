---

description: "Task list for Identity and Access Management (Milestone 3)"
---

# Tasks: Identity and Access Management

**Input**: Design documents from `/specs/003-identity-access/`
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/openapi.yaml`, `quickstart.md`

**Tests**: Included. The feature spec requires verification ("enforced and verified by automated tests"; SC-001–SC-010) and the project Definition of Done mandates `cargo test`, `cargo fmt`, and `cargo clippy`. Write each story's tests first and confirm they fail before implementing.

**Organization**: Tasks are grouped by user story so each story is independently implementable and testable.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no incomplete dependencies)
- **[Story]**: User story label (US1–US4); omitted for Setup, Foundational, and Polish phases
- Every task includes an exact file path

## Path Conventions

Single Rust crate at repository root: `src/`, `migrations/`, `tests/` (see `plan.md` Project Structure).

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Repair the non-compiling baseline and add the dependencies and bootstrap required by every story.

- [X] T001 Update `Cargo.toml` with required dependencies: `uuid` (v4, serde), `time` (serde, macros), `jsonwebtoken`, `argon2`, `sha2`, `base64`, `rand`, `thiserror`, `anyhow`, `tracing`, `tracing-subscriber`, `dotenvy`, `tower-http`; remove the unused `cors = "0.1"` stub
- [X] T002 Repair `.github/workflows/ci.yml` YAML so the `fmt`/`clippy`/`test` jobs run (fix `factors:` → `strategy.matrix`, `name.` → `name:`) 
- [X] T003 [P] Create the `src/platform/` module tree (`domain/`, `application/`, `infrastructure/`, `presentation/` with `mod.rs` files) and declare `pub mod platform;` in `src/lib.rs`
- [X] T004 [P] Implement typed config in `src/config.rs` (DATABASE_URL, JWT_SECRET, JWT_ACCESS_TTL, JWT_REFRESH_TTL, ARGON2_*, HOST, PORT) with fail-fast on missing `JWT_SECRET` outside dev
- [X] T005 Create `src/main.rs` binary entrypoint: load `.env`, build pool, run migrations, serve router on `HOST:PORT`
- [X] T006 Scaffold `AppState` and `/api/v1` route composition in `src/lib.rs` (health routes preserved)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Persistence, domain core, adapters, middleware, and envelopes shared by all stories.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

- [X] T007 Write `migrations/004_platform_auth.sql`: create `platform.refresh_tokens` and `platform.audit_logs` (per `data-model.md` §1.6–1.7), add indexes, and idempotently seed roles (`admin`,`producer`,`customer`,`support`) and baseline permissions (`users.read`,`users.update`,`roles.assign`,`audit.read`) with `ON CONFLICT DO NOTHING`
- [X] T008 [P] Fix `src/db/mod.rs` pool creation (remove compile-time `query!` dependency; use runtime `query`, remove invalid `log_statements` path) and keep `health_check`
- [X] T009 [P] Create `User` entity, `UserStatus` state machine, email normalization, and invariants in `src/platform/domain/user.rs` (correct the existing broken file; remove it from `src/models/`)
- [X] T010 [P] Create `Role` and `Permission` entities in `src/platform/domain/role.rs` (align to DB design §4.3–4.4; do not assume `status`/`updated_at` columns)
- [X] T011 [P] Create `RefreshToken` entity with active/revoked/expired rules in `src/platform/domain/refresh_token.rs`
- [X] T012 [P] Create `PlatformError` and error categories in `src/platform/domain/errors.rs`
- [X] T013 Define application ports (`UserRepository`, `RoleRepository`, `RefreshTokenRepository`, `AuditRepository`, `PasswordHasher`, `TokenIssuer`) in `src/platform/application/ports.rs`
- [X] T014 [P] Implement Argon2id hasher adapter in `src/platform/infrastructure/argon2_hasher.rs` (PHC strings, env-tuned params)
- [X] T015 [P] Implement JWT access-token issuer/decoder in `src/platform/infrastructure/jwt_issuer.rs` (HS256, `sub`/`iss`/`aud`/`iat`/`exp`/roles, configurable TTL)
- [X] T016 [P] Implement `UserRepository` over SQLx in `src/platform/infrastructure/persistence/user_repository.rs`
- [X] T017 [P] Implement `RoleRepository` over SQLx in `src/platform/infrastructure/persistence/role_repository.rs`
- [X] T018 [P] Implement `RefreshTokenRepository` over SQLx in `src/platform/infrastructure/persistence/refresh_token_repository.rs` (hash lookup, revoke, rotate within a transaction)
- [X] T019 [P] Implement `AuditRepository` over SQLx in `src/platform/infrastructure/persistence/audit_repository.rs` (append-only; never store secrets)
- [X] T020 Implement standard response/error envelopes and request-id middleware in `src/platform/presentation/dto.rs` and `src/platform/presentation/middleware/request_id.rs`
- [X] T021 Implement Bearer authentication extractor (`AuthUser` with roles) in `src/platform/presentation/middleware/auth.rs`
- [X] T022 Wire the platform router and `AppState` (pool + config + adapters) into `src/lib.rs` and `src/platform/presentation/routes.rs`

**Checkpoint**: Foundation ready — user stories can begin.

---

## Phase 3: User Story 1 — User Registration (Priority: P1) 🎯 MVP

**Goal**: A new user can register with email and password and immediately receive a token pair.

**Independent Test**: `POST /api/v1/auth/register` with a fresh email returns 201 + tokens; duplicate email returns 409; invalid email/weak password returns 400. Covered by `tests/auth_register.rs`.

### Tests for User Story 1

- [X] T023 [P] [US1] Write API/integration tests for registration (201 success, 409 duplicate, 400 validation) in `tests/auth_register.rs`

### Implementation for User Story 1

- [X] T024 [US1] Add `RegisterRequest`, `TokenResponse`, `UserResponse` DTOs to `src/platform/presentation/dto.rs`
- [X] T025 [US1] Implement register use case in `src/platform/application/use_cases/register.rs` (normalize/lowercase email, enforce unique email, hash password, create `pending` user, assign `customer` role, issue token pair, write audit `user.registered`)
- [X] T026 [US1] Implement `POST /api/v1/auth/register` handler and route in `src/platform/presentation/handlers.rs` and `src/platform/presentation/routes.rs`
- [X] T027 [US1] Map registration outcomes to the standard error envelope (`EMAIL_ALREADY_EXISTS`→409, `VALIDATION_ERROR`→400, business rule→422) in `src/platform/presentation/errors.rs`

**Checkpoint**: Registration works and is independently testable (MVP).

---

## Phase 4: User Story 2 — User Login (Priority: P1)

**Goal**: A registered user authenticates with email/password and receives a token pair; invalid attempts are rejected without revealing whether the account exists.

**Independent Test**: `POST /api/v1/auth/login` with valid credentials returns 200 + tokens; wrong password or unknown email both return identical 401 `INVALID_CREDENTIALS`; suspended/closed users are rejected. Covered by `tests/auth_login.rs`.

### Tests for User Story 2

- [X] T028 [P] [US2] Write API/integration tests for login (200 success, 401 invalid non-enumerating, 401 suspended/closed) in `tests/auth_login.rs`

### Implementation for User Story 2

- [X] T029 [US2] Add `LoginRequest` (with `remember_me`) to `src/platform/presentation/dto.rs`
- [X] T030 [US2] Implement login use case in `src/platform/application/use_cases/login.rs` (lookup by normalized email, verify Argon2id hash, enforce `can_authenticate`, identical failure response, issue tokens)
- [X] T031 [US2] Implement `POST /api/v1/auth/login` handler and route in `src/platform/presentation/handlers.rs` and `src/platform/presentation/routes.rs`
- [X] T032 [US2] Emit audit records `user.login` and `user.login_failed` (no credential material) via `src/platform/infrastructure/persistence/audit_repository.rs`

**Checkpoint**: US1 and US2 both work; registration + login complete the core MVP.

---

## Phase 5: User Story 3 — Session and Token Management (Priority: P2)

**Goal**: Users refresh tokens with rotation, log out to revoke sessions, and retrieve their own profile via a valid access token.

**Independent Test**: `POST /auth/refresh` returns a new pair and invalidates the old refresh token (reuse → 401); `POST /auth/logout` returns 204 and makes the refresh token unusable; `GET /auth/me` returns the caller. Covered by `tests/auth_session.rs`.

### Tests for User Story 3

- [X] T033 [P] [US3] Write API/integration tests for refresh rotation, old-token reuse → 401, logout revocation, expired access token → 401, and `/auth/me` in `tests/auth_session.rs`

### Implementation for User Story 3

- [X] T034 [US3] Add `RefreshRequest` and `LogoutRequest` DTOs to `src/platform/presentation/dto.rs`
- [X] T035 [US3] Implement refresh use case in `src/platform/application/use_cases/refresh.rs` (hash presented token, require active row, rotate old→new atomically, issue new pair, audit `token.refreshed`)
- [X] T036 [US3] Implement logout use case in `src/platform/application/use_cases/logout.rs` (revoke presented refresh token, audit `user.logged_out`)
- [X] T037 [US3] Implement current-user use case in `src/platform/application/use_cases/get_current_user.rs`
- [X] T038 [US3] Implement `POST /auth/refresh`, `POST /auth/logout`, `GET /auth/me` handlers and routes in `src/platform/presentation/handlers.rs` and `src/platform/presentation/routes.rs`

**Checkpoint**: Session lifecycle is complete and independently testable.

---

## Phase 6: User Story 4 — Role-Based Access Control (Priority: P2)

**Goal**: Backend enforces role-based access and resource ownership; users can retrieve/update only their own profile unless they are admins.

**Independent Test**: A role guard allows an `admin` and denies a `customer` on a role-protected operation; `GET/PATCH /users/{id}` succeeds for the owner, returns 403 for another non-admin user, and succeeds for an admin; unauthenticated requests return 401. Covered by `tests/auth_rbac.rs`.

### Tests for User Story 4

- [X] T039 [P] [US4] Write API/integration tests for RBAC guard (allow/deny), ownership 403, admin override, and 401 unauthenticated in `tests/auth_rbac.rs`

### Implementation for User Story 4

- [X] T040 [US4] Implement `require_role(...)` guard / role-check extractor in `src/platform/presentation/middleware/rbac.rs`
- [X] T041 [US4] Extend `RoleRepository` with role assignment and "list roles for user" methods in `src/platform/infrastructure/persistence/role_repository.rs` and `src/platform/application/ports.rs`
- [X] T042 [US4] Implement get-user use case with ownership/admin checks in `src/platform/application/use_cases/get_user.rs`
- [X] T043 [US4] Implement update-user use case (email update, ownership/admin, re-normalize + uniqueness) in `src/platform/application/use_cases/update_user.rs`
- [X] T044 [US4] Implement `GET /api/v1/users/{user_id}` and `PATCH /api/v1/users/{user_id}` handlers and routes in `src/platform/presentation/handlers.rs` and `src/platform/presentation/routes.rs`
- [X] T045 [US4] Enforce ownership/admin authorization on user routes and emit audit `user.updated` / `role.assigned` in `src/platform/presentation/middleware/rbac.rs`

**Checkpoint**: All four user stories are independently functional.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Governance, contract sync, cleanup, and end-to-end validation.

- [X] T046 [P] Write ADR reconciling role vocabulary (constitution `admin/producer/customer/support` vs API spec `CUSTOMER/FARMER/SELLER/ADMIN`) in `docs/adr/0001-role-vocabulary-reconciliation.md`
- [X] T047 [P] Sync `specs/003-identity-access/contracts/openapi.yaml` with implemented handlers and validate the schema
- [X] T048 [P] Add a concurrent-refresh test (two `/auth/refresh` calls with the same token → exactly one 200) in `tests/auth_session.rs`
- [X] T049 [P] Remove legacy `src/security/` and `src/models/` (after consolidation) and eliminate dead code; confirm no orphan modules
- [X] T050 [P] Update `.env`/`.env.example` with `JWT_ACCESS_TTL`/`JWT_REFRESH_TTL`, add a note in `docs/06-api-specification.md` pointing to the role ADR, and refresh `specs/003-identity-access/quickstart.md` if commands changed
- [X] T051 Security hardening review of `src/platform/presentation/` (middleware, handlers, dto) and `src/platform/infrastructure/`: confirm no secrets/tokens/hashes in logs or responses, secure failure responses, and validate role checks on every protected route
- [X] T052 Run all `quickstart.md` §4 scenarios end-to-end and record results; fix discrepancies
- [X] T053 Final quality gates against the crate rooted at `Cargo.toml`: `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`; resolve all failures

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Starts immediately.
- **Foundational (Phase 2)**: Depends on Setup; **blocks all user stories**.
- **User Stories (Phase 3–6)**: Depend on Foundational completion.
- **Polish (Phase 7)**: Depends on the desired user stories being complete.

### User Story Dependencies

- **US1 Registration (P1)**: After Foundational. No dependency on other stories. Issues tokens itself, so it is a valid MVP.
- **US2 Login (P1)**: After Foundational. Independent; complements US1.
- **US3 Session/Token (P2)**: After Foundational. Uses token issuance/rotation from Foundational; for realistic testing it registers/logs in (US1/US2) but can be built in parallel.
- **US4 RBAC (P2)**: After Foundational (auth extractor T021). Independent test via seeded roles; integrates with US1–US3 tokens.

### Within Each User Story

- Tests first and must fail before implementation.
- DTOs → use cases → handlers/routes → error mapping/audit.
- Complete and validate a story before moving to the next priority (unless staffing allows parallelism).

### Parallel Opportunities

- Setup: T003 and T004 in parallel.
- Foundational: T008–T012 (domain/pool) in parallel; T014–T019 (adapters/repos) in parallel.
- Tests: T023, T028, T033, T039 are in separate files and can run in parallel.
- US1/US2 can be developed concurrently by different developers after Foundational.
- Polish: T046–T050 are independent files.

---

## Parallel Example: Foundational Adapters

```bash
# After T013 (ports) is defined, these can proceed together:
Task: "Implement Argon2id hasher in src/platform/infrastructure/argon2_hasher.rs"   # T014
Task: "Implement JWT issuer in src/platform/infrastructure/jwt_issuer.rs"            # T015
Task: "Implement UserRepository in src/platform/infrastructure/persistence/user_repository.rs"  # T016
Task: "Implement RoleRepository in src/platform/infrastructure/persistence/role_repository.rs"  # T017
Task: "Implement RefreshTokenRepository in src/platform/infrastructure/persistence/refresh_token_repository.rs"  # T018
```

## Parallel Example: User Story Tests

```bash
Task: "Registration tests in tests/auth_register.rs"  # T023 (US1)
Task: "Login tests in tests/auth_login.rs"            # T028 (US2)
Task: "Session tests in tests/auth_session.rs"        # T033 (US3)
Task: "RBAC tests in tests/auth_rbac.rs"              # T039 (US4)
```

---

## Implementation Strategy

### MVP First (User Story 1)

1. Complete Phase 1: Setup (repair baseline, deps, bootstrap).
2. Complete Phase 2: Foundational (migration 004, domain, adapters, middleware) — **blocks everything**.
3. Complete Phase 3: US1 Registration (issuing tokens makes it a usable MVP).
4. **STOP and VALIDATE** via `tests/auth_register.rs` and `quickstart.md` §4.1–4.3.

### Incremental Delivery

1. Setup + Foundational → foundation ready.
2. US1 Registration → validate → demo (MVP).
3. US2 Login → validate → demo.
4. US3 Session/Token → validate → demo.
5. US4 RBAC → validate → demo.
6. Polish → full DoD, ADR, contract sync, quickstart validation.

### Parallel Team Strategy

1. Team completes Setup + Foundational together.
2. After Foundational: Dev A → US1, Dev B → US2, then converge on US3/US4.
3. Each story is validated independently before integration.

---

## Notes

- `[P]` = different files, no incomplete dependencies.
- Tests are required by the feature spec/DoD; verify they fail before implementing.
- Migrations are immutable: all new DDL lives in `migrations/004_platform_auth.sql`.
- Never log or return `password_hash`, access tokens, or refresh tokens.
- Commit after each task or logical group; use conventional commit messages (`feat(platform): ...`).


---

## Implementation Notes

- Migration ordering was repaired: `002_create_traceability_schema.sql` now runs before
  `003_create_commerce_schema.sql` because commerce products reference
  `traceability.producers`. The certification constraints in migration 002 are applied via
  `ALTER TABLE`, and the `commerce.product_prices` index is deferred until that table is
  introduced in a later milestone.
- T002 was resolved by removing the invalid `factors:` blocks entirely; the CI workflow now
  uses a single Rust 1.96 toolchain and a PostgreSQL 17 service for tests.
- `role.assigned` is audited when registration assigns the default `customer` role; there is
  no public role-assignment endpoint in this milestone's contract.
- Quickstart §4 scenarios were executed against a running server; all expected status codes
  and payload fields matched (see completion report).
