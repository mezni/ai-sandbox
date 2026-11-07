# Implementation Plan: Identity and Access Management

**Branch**: `003-identity-access` | **Date**: 2026-11-06 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/003-identity-access/spec.md`

**Source**: Milestone 3 (Identity and Access Management) from `docs/08-implementation-plan.md`.

## Summary

Deliver the Platform Domain's authentication and authorization capability: user registration, login, JWT access tokens, refresh-token rotation and revocation, logout, current-user/profile retrieval, and backend-enforced role-based access control (RBAC). The feature also reconciles the existing partial scaffolding (`src/security`, `src/models`, `src/db`) into a contiguous, compiling, tested `platform` module that follows Clean Architecture, adds the missing `platform.refresh_tokens` and `platform.audit_logs` tables plus role/permission seeds, and exposes the `/api/v1/auth/*` and `/api/v1/users/*` endpoints defined in `docs/06-api-specification.md` §4–§5.1.

Technical approach: Rust + Axum 0.7 + SQLx 0.7 + PostgreSQL. Passwords are hashed with Argon2id; access tokens are short-lived HS256 JWTs; refresh tokens are opaque random values whose SHA-256 hashes are persisted and rotated on every use. RBAC is data-driven (roles/permissions tables seeded idempotently) and enforced in a backend authentication/authorization layer, never in the frontend.

## Technical Context

**Language/Version**: Rust edition 2021 (CI toolchain 1.96 per `.github/workflows/ci.yml`)

**Primary Dependencies**:
- Existing: `axum` 0.7, `sqlx` 0.7 (`postgres`, `default-features = false`), `tokio` 1.37 (`full`), `serde` 1.0 (`derive`), `serde_json` 1.0
- To add for this feature: `uuid` (v4, serde), `time` (serde, macros), `jsonwebtoken` (HS256), `argon2` (Argon2id), `rand`/`getrandom`, `sha2` + `base64` (refresh-token hashing/encoding), `thiserror` + `anyhow`, `tracing` + `tracing-subscriber` (structured logs), `dotenvy` (env loading), `tower-http` (CORS, request-id, tracing). Replace the unused `cors = "0.1"` stub with `tower-http`.

**Storage**: PostgreSQL single database. Platform schema tables: `platform.users`, `platform.roles`, `platform.permissions`, `platform.user_roles`, `platform.role_permissions`, plus new `platform.refresh_tokens` and `platform.audit_logs`. Access via SQLx pool.

**Testing**: `cargo test` (unit + integration), `cargo clippy -- -D warnings`, `cargo fmt --check`. Integration tests run against a real PostgreSQL instance using `DATABASE_URL` (per-test schema/transaction isolation); `tests/migration_test.rs` validates up/down behaviour.

**Target Platform**: Linux server (CI `ubuntu-latest`); local development via Docker Compose / local PostgreSQL. HTTP surface served under `/api/v1` (dev base `http://localhost:8080/api/v1` per API spec §3.1; the current `.env` uses `127.0.0.1:3000`, which this plan reconciles to a configurable `PORT`/`API_BASE`).

**Project Type**: Modular monolith. Single Rust crate (`biosouk`) with a new `platform` module organized in Clean Architecture layers (`domain`, `application`, `infrastructure`, `presentation`). Frontend (React + Vite + Tailwind) is out of scope for this milestone except as future consumer of the contracts.

**Performance Goals**:
- Authentication (register/login) response p95 < 200 ms against a local PostgreSQL instance.
- Support ≥100 concurrent authentication requests without degradation (aligns with 002 SC-002 and spec SC-002).
- Access-token validation must be stateless and O(1) (signature + claims check, no database round-trip).

**Constraints**:
- Passwords MUST be stored only as Argon2id PHC hashes; never plaintext (constitution §226–269).
- Refresh tokens MUST be stored only as hashes; rotation invalidates the previous token; logout revokes the session (constitution §226–231; DB design §4.7).
- Authorization MUST be enforced on the backend through RBAC plus resource-ownership checks; frontend permissions are not a security boundary (constitution §233–244).
- API responses MUST use the standard success envelope `{data, meta.request_id}` and error envelope `{error:{code,message,details,request_id}}` (API spec §6.4, §8).
- No secrets, tokens, or password material in source control or logs (constitution §262–269; API spec §495, §505).
- Domain/application layers MUST NOT depend on Axum/SQLx/PostgreSQL directly; those belong to infrastructure/presentation (constitution §63–73).

**Scale/Scope**:
- One bounded context: **Platform Domain** (User, Role, Permission, Authentication, Authorization, Audit).
- 7 HTTP endpoints (5 auth + 2 user), 7 tables, 4 canonical roles seeded, ~12 functional requirements from the spec (FR-001–FR-012), 10 success criteria.
- Out of scope: social/OAuth identity providers, email verification delivery, MFA, password-reset email flow, frontend screens, rate-limiting tuning (design noted, deferred).

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Gate 1 — Business First (domain model is source of truth)**: PASS. The feature models `User`, `Role`, `Permission`, `RefreshToken`, and `AuditLog`, all taken from the approved Platform Domain (constitution §98–116) and database design §4. No DTO/controller-first modelling.

**Gate 2 — Bounded Contexts**: PASS. All work is assigned to the **Platform Domain**, one of the eight approved domains (constitution §39–49). No other domain's aggregates are modified.

**Gate 3 — Explicit Ownership**: PASS. The Platform Domain owns the business rules, aggregates, entities, persistence schema (`platform.*`), and application services for identity. The `audit_logs` record is owned by Platform (constitution §106–116). Other domains consume identity only through public application interfaces / token claims.

**Gate 4 — Clean Architecture**: PASS (by design). Dependencies point inward: `domain` (entities/value objects/rules) → `application` (use cases/ports) ← `infrastructure` (SQLx repositories, JWT/Argon2 adapters) ← `presentation` (Axum handlers). The domain layer has no Axum/SQLx/PostgreSQL imports.

**Gate 5 — Modular Monolith**: PASS. Single repository, single backend application, single PostgreSQL database, separate `platform` schema. No cross-domain direct repository access. Access tokens carry only identity claims; downstream domains validate tokens via a public interface.

**Security principle compliance**: PASS — JWT access tokens, Argon2id hashing, refresh-token rotation and revocation, backend RBAC, input validation, audit logging, and secret externalization are all explicitly planned (constitution §224–269).

**Constitution Check Result**: All gates pass. Proceed to Phase 0 research.

**Recorded deviation (requires ADR, tracked in Complexity Tracking)**: Role codes differ between the constitution (`Admin`, `Producer`, `Customer`, `Support` — §239–244) and API spec §4.2 (`CUSTOMER`, `FARMER`, `SELLER`, `ADMIN`). The constitution supersedes; role codes are seeded data, and the reconciliation is documented in `research.md` (Decision R11) with an ADR action item.

### Post-Design Re-evaluation (after Phase 1)

Re-checked against the generated `research.md`, `data-model.md`, `contracts/openapi.yaml`, and `quickstart.md`:

- **Gate 1 — Business First**: PASS. Data model is entity-first (User/Role/Permission/RefreshToken/AuditLog); DTOs are explicitly separated.
- **Gate 2 — Bounded Contexts**: PASS. Only the Platform Domain is touched.
- **Gate 3 — Explicit Ownership**: PASS. Platform owns `platform.*` schema, entities, use cases, and audit records.
- **Gate 4 — Clean Architecture**: PASS. Structure separates `domain` from `application`/`infrastructure`/`presentation`; no Axum/SQLx imports in `domain`.
- **Gate 5 — Modular Monolith**: PASS. Single crate, single app, single DB, isolated schema; tokens are the integration contract to other domains.
- **Security**: PASS. Argon2id, hashed refresh tokens with rotation/revocation, backend RBAC + ownership checks, audit logging, secret externalization, no secrets in DTOs/logs.
- **No unresolved clarifications**: PASS. `research.md` resolves all Technical Context items; no `[NEEDS CLARIFICATION]` remain.

**Result**: All gates PASS after design. The two Complexity Tracking items remain justified and require an ADR (role reconciliation) and a follow-up (workspace extraction) respectively.

## Project Structure

### Documentation (this feature)

```text
specs/003-identity-access/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
│   └── openapi.yaml
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created here)
```

### Source Code (repository root)

The existing single-crate layout is retained; the identity capability is added as a cohesive `platform` module. Existing files that already hint at this feature (`src/security/*`, `src/models/{user,role,permission}.rs`, `src/db/*`) are consolidated/moved under `src/platform` and made to compile.

```text
src/
├── lib.rs                      # App composition; registers platform routes + middleware
├── main.rs                     # (add) binary entrypoint: env load, pool, migrate, serve
├── config.rs                   # (add) Config from env (DATABASE_URL, JWT_*, ARGON2_*, HOST, PORT)
├── hello.rs                    # existing health/status handlers
├── db/
│   ├── mod.rs                  # pool creation (fix LogStatement/query! usage)
│   └── transaction.rs
├── platform/                   # NEW: Platform Domain module (Clean Architecture)
│   ├── mod.rs
│   ├── domain/
│   │   ├── mod.rs
│   │   ├── user.rs             # User entity, UserStatus state machine, invariants
│   │   ├── role.rs             # Role / Permission entities
│   │   ├── refresh_token.rs    # RefreshToken entity + rotation/revocation rules
│   │   └── errors.rs           # PlatformDomainError
│   ├── application/
│   │   ├── mod.rs
│   │   ├── ports.rs            # UserRepository, RoleRepository, TokenRepository, Hasher, TokenIssuer
│   │   └── use_cases/
│   │       ├── register.rs
│   │       ├── login.rs
│   │       ├── refresh.rs
│   │       ├── logout.rs
│   │       └── get_current_user.rs
│   ├── infrastructure/
│   │   ├── mod.rs
│   │   ├── persistence/
│   │   │   ├── user_repository.rs
│   │   │   ├── role_repository.rs
│   │   │   └── refresh_token_repository.rs
│   │   ├── argon2_hasher.rs
│   │   └── jwt_issuer.rs
│   └── presentation/
│       ├── mod.rs
│       ├── routes.rs           # /auth/*, /users/:id
│       ├── handlers.rs
│       ├── dto.rs              # request/response DTOs (separate from DB models)
│       └── middleware/
│           ├── auth.rs         # Bearer extraction + claims -> AuthUser extension
│           └── rbac.rs         # require_role(...) guard + ownership checks
├── models/                     # legacy location (migrate into platform/, then remove)
└── security/                   # legacy location (migrate into platform/, then remove)

migrations/
├── 001_create_platform_schema.sql      # existing
├── 002_create_commerce_schema.sql      # existing
├── 003_create_traceability_schema.sql  # existing
└── 004_platform_auth.sql               # NEW: refresh_tokens, audit_logs, role/permission seeds

tests/
├── migration_test.rs           # existing (extend/repair)
├── auth_integration.rs         # NEW: register/login/refresh/logout/me + RBAC
└── common/mod.rs               # NEW: test pool/harness helpers
```

**Structure Decision**: Single-crate modular monolith with a `src/platform` module split into `domain` / `application` / `infrastructure` / `presentation`. This preserves the Sprint 1–2 foundation (single `biosouk` crate, existing `src/` and `migrations/`) while honoring Clean Architecture boundaries. Extracting `biosouk-platform` into a Cargo workspace crate is deferred until the module stabilizes (constitution: "avoid creating empty abstractions without a concrete use case"; architecture doc §392).

## Complexity Tracking

> Justified deviations from the default/architectural-doc structure.

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| Identity implemented as a module in the single `biosouk` crate rather than a separate `biosouk-platform` workspace crate (architecture doc §6) | Sprint 1–2 already established a single-crate layout; a workspace migration now would churn unrelated code and block the milestone. | Introducing a full Cargo workspace and crate split adds build/config overhead and cross-crate plumbing before the module boundaries are proven; the constitution warns against premature empty abstractions. |
| Role codes seeded as data instead of hardcoded enum matching API spec §4.2 | Constitution §239–244 (Admin/Producer/Customer/Support) supersedes API spec; data-driven roles let the code support both vocabularies and future roles without code change. | Hardcoding CUSTOMER/FARMER/SELLER/ADMIN would contradict the constitution; hardcoding the constitution set alone would hide the API-spec divergence rather than reconciling it. Requires an ADR to update docs. |
