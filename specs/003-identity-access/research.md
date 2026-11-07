# Research: Identity and Access Management (Milestone 3)

**Created**: 2026-11-06
**Feature**: `specs/003-identity-access`
**Status**: Complete — all Technical Context unknowns resolved; no `[NEEDS CLARIFICATION]` markers remain.

This document records the decisions made during Phase 0 for the Platform Domain identity feature. Each entry states the decision, rationale, and alternatives considered.

---

## R1 — Runtime and language

- **Decision**: Rust edition 2021, built/tested on toolchain 1.96 (as pinned in `.github/workflows/ci.yml`).
- **Rationale**: Matches the existing `Cargo.toml` and CI configuration; no reason to change.
- **Alternatives considered**: Pinning a local `rust-toolchain.toml` (none exists) — rejected for this milestone to avoid CI drift; can be added later.

## R2 — Web framework and async runtime

- **Decision**: Axum 0.7 on Tokio 1.37 (`full`).
- **Rationale**: Already the project's chosen stack (`docs/08`, `quickstart.md`, `Cargo.toml`); Axum extractors and middleware compose cleanly with the planned auth/RBAC layers.
- **Alternatives considered**: Actix-web — rejected to avoid a stack rewrite.

## R3 — Persistence

- **Decision**: SQLx 0.7 with PostgreSQL, runtime-checked queries (`query_as`, `FromRow`) during this milestone; migrate to compile-checked `query!` macros once a live `DATABASE_URL` is guaranteed in CI.
- **Rationale**: The repo currently uses `sqlx::query!` in places without a configured DB at build time, which breaks offline builds. Runtime-checked queries keep the milestone unblocked; a follow-up can enable `SQLX_OFFLINE` with a checked-in `.sqlx` cache.
- **Alternatives considered**: Diesel/SeaORM — rejected (new dependency, diverges from established stack). Full `query!` now — rejected (requires CI DB provisioning first).

## R4 — Password hashing

- **Decision**: Argon2id via the `argon2` crate, producing PHC-format strings (`$argon2id$...`). Parameters sourced from env (`ARGON2ID_MEMORY`, `ARGON2ID_THREADS`, plus time cost), with secure defaults.
- **Rationale**: Directly mandated by constitution §229 and already anticipated in `.env` (`ARGON2ID_*`). PHC strings embed salt+params, simplifying verification and future re-hashing.
- **Alternatives considered**: bcrypt (allowed as fallback by constitution) — rejected because Argon2id is explicitly preferred and env vars are already defined; storing raw salt/hash columns — rejected (PHC string is self-describing).

## R5 — Access tokens

- **Decision**: HS256 JWTs via `jsonwebtoken`, signed with `JWT_SECRET`. Claims: `sub` (user UUID), `iss`, `aud`, `iat`, `exp`, and a roles claim. Default access lifetime 15 minutes (configurable via `JWT_ACCESS_TTL`), based on `.env`/002 assumptions.
- **Rationale**: Stateless validation (no DB round-trip) meets the O(1) constraint; HS256 is sufficient for a single backend; secret is externally configured. Embedding roles in the token avoids a DB lookup on every request, while ownership checks remain data-access-bound.
- **Alternatives considered**: RS256/asymmetric — deferred until multiple services verify tokens; opaque DB-backed access tokens — rejected (adds per-request DB load).

## R6 — Refresh tokens and rotation

- **Decision**: Refresh tokens are opaque 256-bit random values (base64url). Only their SHA-256 hash is stored in `platform.refresh_tokens.token_hash` (unique). On `/auth/refresh`, the presented token is hashed, matched to a non-revoked, non-expired row, then the old row is revoked and a new token pair is issued in a single transaction (rotation). `/auth/logout` revokes the presented refresh token. Default refresh lifetime 30 days.
- **Rationale**: Satisfies constitution §226–231 and DB design §4.7 ("store only token hashes"). Rotation + revocation provides replay detection and session invalidation. Hashing at rest means a DB leak does not expose usable tokens.
- **Alternatives considered**: Storing raw refresh tokens — rejected (violates DB design). JWT refresh tokens without storage — rejected (cannot revoke). One-time-use with reuse-detection family tracking — considered but deferred as a hardening step (documented as follow-up, not required by spec).

## R7 — RBAC model

- **Decision**: Data-driven RBAC using `platform.roles`, `platform.permissions`, `platform.user_roles`, `platform.role_permissions`. Roles are seeded idempotently by migration 004. Authorization is enforced by presentation-layer middleware/extractors that require a role (e.g., `require_role("admin")`) and by application-layer resource-ownership checks (e.g., a user may read/update only their own profile unless admin).
- **Rationale**: Matches constitution §51–59 (explicit ownership) and §233–244; keeps authorization on the backend. Data-driven roles avoid code changes for new roles and support the reconciliation in R11.
- **Alternatives considered**: Hardcoded role enum — rejected (inflexible, conflicts with reconciliation). Frontend gating — explicitly forbidden by constitution.

## R8 — Modular structure and layering

- **Decision**: Add `src/platform/{domain,application,infrastructure,presentation}` inside the existing single crate; consolidate the existing `src/security/*` and `src/models/{user,role,permission}.rs` into it. Keep migrations at repo root.
- **Rationale**: Honors Clean Architecture inward dependencies while avoiding a disruptive workspace migration mid-sprint (see plan Complexity Tracking). Existing partial code already implies this seam.
- **Alternatives considered**: Immediate Cargo-workspace `biosouk-platform` crate — deferred (churn, premature). Leaving code in `src/security`/`src/models` — rejected (no clear domain boundary, duplicates).

## R9 — Database migration for auth

- **Decision**: Add `migrations/004_platform_auth.sql` that (a) creates `platform.refresh_tokens` and `platform.audit_logs` per DB design §4.7–4.8, (b) seeds the canonical roles and baseline permissions idempotently (`ON CONFLICT DO NOTHING`), and (c) aligns the `roles` table usage to the documented columns. Also correct pre-existing mismatches: the `Role` model must not assume `status`/`updated_at` columns that do not exist in `platform.roles` (DB design §4.3), and `user.rs` syntax errors must be fixed.
- **Rationale**: Migrations 001–003 create the base platform tables but omit `refresh_tokens` and `audit_logs`. Seeds make first-run testing deterministic. Keeping the documented schema authoritative avoids drift.
- **Alternatives considered**: Editing migration 001 in place — rejected (migrations are immutable once applied; a new migration is the safe path). Adding `status` to roles — rejected (not in the design; use permissions instead).

## R10 — API conventions, DTOs, and errors

- **Decision**: Base path `/api/v1`. Success responses: `{ "data": ..., "meta": { "request_id": ... } }`. Errors: `{ "error": { "code", "message", "details", "request_id" } }` with the status codes/error codes from API spec §8. DTOs live in `presentation/dto.rs` and are distinct from DB/domain models. A request-id middleware populates `X-Request-ID`/`meta.request_id`.
- **Rationale**: Required by API spec §6.4, §8, §3.4. Separating DTOs prevents leaking domain/DB internals.
- **Alternatives considered**: Bare JSON bodies — rejected (violates standard envelope). Sharing DB models as DTOs — rejected (leaks `password_hash`, coupling).

## R11 — Role vocabulary reconciliation (ADR required)

- **Decision**: Treat the **constitution as authoritative**: canonical role codes are `admin`, `producer`, `customer`, `support` (§239–244). The API spec's `CUSTOMER`, `FARMER`, `SELLER`, `ADMIN` (§4.2) are reconciled by seeding canonical roles and documenting the mapping (`farmer`/`seller` responsibilities fall under `producer`/`customer` capabilities for this milestone; additional role codes can be seeded without code change). RBAC code treats role codes as opaque strings. An ADR will be filed to update `docs/06` to match the constitution.
- **Rationale**: The constitution "supersedes all other practices" (constitution Governance). Data-driven codes make the reconciliation non-breaking. Prevents shipping a security model that contradicts governance.
- **Alternatives considered**: Follow API spec codes — rejected (violates supremacy of constitution). Support both sets as distinct roles — rejected (ambiguous authorization, role sprawl).

## R12 — Configuration and secrets

- **Decision**: Load env via `dotenvy` (dev) into a typed `Config` struct. Required: `DATABASE_URL`, `JWT_SECRET`, `JWT_ACCESS_TTL`, `JWT_REFRESH_TTL`, `ARGON2_*`, `HOST`, `PORT`. No defaults for `JWT_SECRET` in non-dev; fail fast if missing in production. Never log secrets/tokens/password hashes.
- **Rationale**: Constitution §249–269 (secrets management, forbidden hardcoded secrets); `.env` already declares these keys. Fail-fast prevents insecure defaults.
- **Alternatives considered**: Hardcoded defaults — rejected (forbidden). Keeping the current `"secret"` default — must be removed.

## R13 — Testing strategy

- **Decision**:
  - **Unit**: domain invariants (email normalization/uniqueness, status transitions), password hash/verify, JWT encode/decode/expiry, RBAC decision logic.
  - **Integration**: repositories against real PostgreSQL (per-test transaction rollback or schema), refresh rotation/revocation.
  - **API**: Axum handler tests through the router using a test pool — register → login → me → refresh → logout; RBAC 401/403; duplicate email 409; validation 400/422.
  - **Concurrency**: parallel `/auth/refresh` with the same token → exactly one succeeds (rotation).
- **Rationale**: Covers spec SC-001–SC-010 and constitution §56 (domain rules tested). Level split matches docs/08 testing strategy.
- **Alternatives considered**: Only unit tests — rejected (would miss SQL/transaction and middleware bugs). Testcontainers — optional; prefer `DATABASE_URL` + `sqlx::test` to keep CI simple, with `tests/migration_test.rs` (currently mis-using testcontainers) repaired or gated.

## R14 — Rate limiting (scoped)

- **Decision**: Design for rate limiting on auth endpoints (`tower-http`/`tower-governor` or a small in-process limiter) but treat implementation as **optional/deferred** for this milestone; document the extension point. Input validation, lockout-by-audit, and constant-time-ish credential failure responses are in scope.
- **Rationale**: Constitution §253 lists rate limiting as a security requirement, but it is not in the feature spec's functional requirements and needs operational tuning. Recording it prevents it being forgotten while keeping MVP scope tight.
- **Alternatives considered**: Implement full rate limiting now — deferred to avoid scope creep; omit entirely — rejected (constitution lists it).

## R15 — Known prerequisites and risks (must be addressed during implementation)

- **R15.1**: `Cargo.toml` is missing many crates referenced by existing code (`uuid`, `time`, `anyhow`, `jsonwebtoken`, `argon2`, etc.) — the crate currently cannot compile. Adding dependencies is part of this feature.
- **R15.2**: `src/lib.rs` only declares `mod hello;`; `db`, `security`, `models` are not wired in. They will be consolidated under `platform`.
- **R15.3**: `src/models/user.rs` has structurally broken braces and free functions outside `impl`; `src/security/rbac.rs` imports a non-existent `traceability_domain` crate and contains an invalid `assert_eq?`; `src/db/mod.rs` uses `sqlx::query!` without `anyhow`/`From` context and a non-existent `log_statements` path. These must be rewritten, not patched.
- **R15.4**: `.github/workflows/ci.yml` contains YAML errors (`factors:` instead of `strategy.matrix`, `name.` instead of `name:`). CI must be repaired for the Definition of Done to be verifiable.
- **R15.5**: `.env` lacks `JWT_ACCESS_TTL`/`JWT_REFRESH_TTL` and uses port 3000 while docs use 8080; `Config` should provide documented defaults and the discrepancy should be resolved in docs.

- **Rationale for listing**: These are real blockers discovered during context loading; capturing them prevents "hidden" work and makes the tasks phase accurate.
- **Alternatives considered**: Silently fixing during coding — rejected (hides scope; violates traceability).

---

## Resolution Summary

| Topic | Decision |
|---|---|
| Stack | Rust 2021 / Axum 0.7 / Tokio / SQLx 0.7 / PostgreSQL |
| Password hashing | Argon2id (PHC string) |
| Access token | HS256 JWT, 15 min default, roles claim |
| Refresh token | Opaque random, SHA-256 hash stored, rotate on use, revoke on logout, 30 day default |
| RBAC | Data-driven roles/permissions; backend enforcement; ownership checks |
| Roles | Constitution canonical (`admin`,`producer`,`customer`,`support`); ADR to reconcile API spec |
| Structure | `src/platform/{domain,application,infrastructure,presentation}` in single crate |
| Migration | New `004_platform_auth.sql` (refresh_tokens, audit_logs, seeds) |
| API | `/api/v1`, standard success/error envelopes, DTOs separate from models |
| Config | Env via dotenvy; no insecure defaults; secrets never logged |
| Testing | Unit + integration + API + refresh concurrency |
| Rate limiting | Designed, deferred implementation |

No `[NEEDS CLARIFICATION]` markers remain. Proceed to Phase 1.
