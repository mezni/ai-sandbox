# Research: Sprint 2 Build Plan — BioSouk

**Created**: 2026-11-06  
**Feature**: Sprint 2 Build Plan (specs/002-sprint-2-build-plan)  
**Status**: Complete — all technical context items resolved, no NEEDS CLARIFICATION markers remain

## Resolution Summary

All Technical Context items in `plan.md` are fully resolved with explicit values. Zero `[NEEDS CLARIFICATION]` markers remain in the feature specification or plan (validated against quality checklist: 0/18 items incomplete). No research agents were required to resolve ambiguous requirements.

## Dependencies — Best Practices

### sqlx (PostgreSQL query builder)
- **Version**: 0.7 (per Cargo.toml)
- **Features**: `postgres` feature enabled; `default-features = false` for minimal surface area
- **Best practices**:
  - Use `sqlx::query!` and `sqlx::query_as!` macros for compile-checked queries
  - Leverage `FromRow` trait for row mapping instead of manual index access
  - Use `sqlx::postgres::PgPool` for connection pooling with `max_connections` configured
  - Employ `sqlx::transaction` for atomic operations (migrations, inventory reservation, order creation)
  - Prefer `sqlx::query` with `fetch_one`/`fetch_all` over raw `SELECT` for predictable results
  - Handle `sqlx::Error` variants explicitly (`RowNotFound`, `DatabaseError`) for graceful degradation
  - Use `Acquire` pattern for manual connection management in tests

### tokio (asynchronous runtime)
- **Version**: 1.37 (per Cargo.toml)
- **Features**: `full` feature enabled (includes macros, time, net, fs)
- **Best practices**:
  - Structure application with `tokio::spawn` for long-running tasks
  - Use `tokio::select!` for race condition-free concurrent operations
  - Leverage `tokio::task::spawn_blocking` for CPU-bound work without blocking the runtime
  - Configure `RuntimeBuilder` with `max_blocking_threads` for I/O-bound workloads
  - Employ `tokio::time::timeout` for external operation latency guarantees
  - Use `tokio::sync::Mutex`, `RwLock`, `Semaphore` for async synchronization

### axum (HTTP framework)
- **Version**: 0.7 (per Cargo.toml)
- **Best practices**:
  - Keep handlers thin — delegate business logic to application/services layer (per architecture constitution §578-580)
  - Use `axum::extract::State` for shared application state (database pool, config)
  - Leverage `axum::Json` extractor for request/response body deserialization/seialization
  - Use `axum::routing:: Router::new` with method routing (`get`, `post`, `patch`, `delete`)
  - Implement middleware for request ID, timing, and error handling at router level
  - Use `axum::middleware::from_fn` for cross-cutting concerns (auth, rate limiting, logging)
  - Return `impl IntoResponse` for flexible response types (JSON, error objects, 204 No Content)

### serde (serialization/deserialization)
- **Version**: 1.0 (per Cargo.toml)
- **Features**: `derive` enabled
- **Best practices**:
  - Use `#[serde(try_from = "...")]` for converting between domain models and DTOs
  - Employ `#[serde(with = "...")]` for custom serialization of numeric types (monetary values)
  - Use `#[serde(rename = "...")]` for JSON key naming (kebab-case for API, snake_case for DB)
  - Leverage `#[serde(skip)]` / `#[serde(skip_serializing)]` / `#[serde(deserialize_if)]` for optional fields
  - Combine `#[serde(rename_all = "kebab-case")]` with external crates for consistent API naming

### cors (CORS middleware)
- **Version**: 0.1 (per Cargo.toml)
- **Best practices**:
  - Restrict `allow_origins` to specific domains in production (not `*`.) 
  - Configure `allow_methods` to only required HTTP verbs per endpoint
  - Set `allow_headers` to explicit list including `Authorization`, `Content-Type`, `Idempotency-Key`
  - Set `expose_headers` for custom headers like `X-Request-ID`
  - Implement `max_age` for browser preflight caching
  - Differentiate development (`localhost` origins) vs production (domain-specific) configurations

## Integrations — Patterns

### PostgreSQL connection (SQLx)
- **Database**: `biosouk_db` with schemas `platform`, `commerce`, `traceability` (per database design §2-3)
- **Connection pool**: `sqlx::postgres::PgPool` with `_acquire` pattern
- **URL format**: `postgres://user:password@localhost:5432/biosouk_db`
- **Migration strategy**: SQLx migrations version-controlled under `backend/migrations/` (per database design §1446-1462)
- **Schema management**: Separate schemas per bounded context; cross-context references handled at application level (database design §1256-1278)
- **Connection management**: `connect` with `options(log_statements = "warn")` for production; `log_statements = "all"` for development debugging

### JWT authentication
- **Provider**: JWT access tokens with configurable expiration (per API specification §4.1-4.3)
- **Token lifecycle**: Access token (short-lived, e.g., 15 min) + refresh token (long-lived, e.g., 30 days) per constitution §111-112
- **Password hashing**: Argon2id or bcrypt (per constitution §229, Security Principles §226-230)
- **Refresh token rotation**: On refresh, previous token invalidated per best practices
- **RBAC enforcement**: Middleware checks user role against endpoint permissions per API specification §4.3

### Inventory concurrency safety
- **Pattern**: Conditional UPDATE with availability check (per database design §1303-1312)
- **SQL pattern**: `UPDATE commerce.inventory_items SET quantity_reserved = quantity_reserved + $1, updated_at = NOW() WHERE id = $2 AND quantity_on_hand - quantity_reserved >= $1 RETURNING id;`
- **Overselling prevention**: If no row returned, reservation failed — caller must handle gracefully
- **Idempotency**: Each reservation has an idempotency key; duplicate requests with same key return original result

## NEEDS CLARIFICATION Status

**Zero `[NEEDS CLARIFICATION]` markers remain** in the feature specification (validated against quality checklist). All technical context values were resolved from:
- Feature specification (`specs/002-sprint-2-build-plan/spec.md`)
- Database design (`docs/05-database-design.md`)
- API specification (`docs/06-api-specification.md`)
- Implementation plan (`docs/08-implementation-plan.md`)
- Constitution (`docs/constitution.md`, `.specify/memory/constitution.md`)
- Project structure (Cargo.toml, quickstart.md, .github/workflows/ci.yml)

No research agents dispatched. All requirements are testable and unambiguous per quality validation.