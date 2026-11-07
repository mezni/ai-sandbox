# Quickstart: Identity and Access Management (Milestone 3)

**Feature**: `specs/003-identity-access`
**Purpose**: Runnable validation scenarios proving the identity feature works end-to-end.
**Contract**: [`contracts/openapi.yaml`](./contracts/openapi.yaml)
**Data model**: [`data-model.md`](./data-model.md)

> This is a validation/run guide, not an implementation spec. Implementation tasks live in `tasks.md` (generated later).

---

## 1. Prerequisites

- Rust toolchain 1.96 (matches CI) and Cargo.
- PostgreSQL 13+ running locally (or via Docker Compose).
- `curl` and `jq` (for the scenarios below).
- Environment variables (see `.env`; extend with the TTL keys added by this feature):

```bash
DATABASE_URL=postgres://postgres:postgres@localhost:5432/biosouk
HOST=127.0.0.1
PORT=8080
JWT_SECRET=<at-least-32-random-bytes>
JWT_ACCESS_TTL=900          # seconds (15 min)
JWT_REFRESH_TTL=2592000     # seconds (30 days)
ARGON2ID_MEMORY=65536
ARGON2ID_THREADS=4
```

> Do not commit real secrets. `JWT_SECRET` has no insecure default outside development.

---

## 2. Setup

```bash
# 1. Create the database if missing
createdb biosouk || true

# 2. Apply migrations (001 platform, 002 traceability, 003 commerce, 004 platform auth)
sqlx migrate run --database-url "$DATABASE_URL"
# or, if the binary entrypoint runs migrations on startup, skip this step

# 3. Build
cargo build

# 4. Run quality gates
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

Expected: build succeeds, fmt/clippy clean, tests pass.

---

## 3. Run the service

```bash
cargo run
# Server listens on http://$HOST:$PORT, routes under /api/v1
```

Health check (pre-existing):

```bash
curl -s http://localhost:8080/health        # existing endpoints
```

---

## 4. End-to-end scenarios

Set a base URL and a unique email:

```bash
BASE=http://localhost:8080/api/v1
EMAIL="test+$(date +%s)@example.com"
PASS='S3cure-passphrase!'
```

### 4.1 Register (201, tokens issued)

```bash
curl -s -X POST "$BASE/auth/register" \
  -H 'Content-Type: application/json' \
  -d "{\"email\":\"$EMAIL\",\"password\":\"$PASS\"}" | tee /tmp/reg.json | jq .
```

Expected: HTTP 201; `data.access_token` and `data.refresh_token` present; `meta.request_id` present.

```bash
ACCESS=$(jq -r '.data.access_token' /tmp/reg.json)
REFRESH=$(jq -r '.data.refresh_token' /tmp/reg.json)
```

### 4.2 Duplicate registration (409)

```bash
curl -s -o /dev/null -w '%{http_code}\n' -X POST "$BASE/auth/register" \
  -H 'Content-Type: application/json' \
  -d "{\"email\":\"$EMAIL\",\"password\":\"$PASS\"}"
```

Expected: `409` with `error.code = EMAIL_ALREADY_EXISTS`.

### 4.3 Validation failure (400)

```bash
curl -s -X POST "$BASE/auth/register" \
  -H 'Content-Type: application/json' \
  -d '{"email":"not-an-email","password":"short"}' | jq .
```

Expected: HTTP 400; `error.code = VALIDATION_ERROR`.

### 4.4 Login (200)

```bash
curl -s -X POST "$BASE/auth/login" \
  -H 'Content-Type: application/json' \
  -d "{\"email\":\"$EMAIL\",\"password\":\"$PASS\"}" | tee /tmp/login.json | jq .
```

Expected: HTTP 200; fresh token pair.

### 4.5 Invalid credentials (401, non-enumerating)

```bash
curl -s -o /dev/null -w '%{http_code}\n' -X POST "$BASE/auth/login" \
  -H 'Content-Type: application/json' \
  -d "{\"email\":\"$EMAIL\",\"password\":\"wrong-password\"}"
```

Expected: `401` with `error.code = INVALID_CREDENTIALS` (same for unknown user).

### 4.6 Current user (200)

```bash
curl -s "$BASE/auth/me" -H "Authorization: Bearer $ACCESS" | jq .
```

Expected: HTTP 200; `data.email == $EMAIL`; `data.roles` array; no `password_hash`.

### 4.7 Protected endpoint without token (401)

```bash
curl -s -o /dev/null -w '%{http_code}\n' "$BASE/auth/me"
```

Expected: `401`.

### 4.8 Refresh + rotation (200, old refresh invalid)

```bash
ACCESS=$(jq -r '.data.access_token' /tmp/login.json)
REFRESH=$(jq -r '.data.refresh_token' /tmp/login.json)

curl -s -X POST "$BASE/auth/refresh" \
  -H 'Content-Type: application/json' \
  -d "{\"refresh_token\":\"$REFRESH\"}" | tee /tmp/refresh.json | jq .

NEW_REFRESH=$(jq -r '.data.refresh_token' /tmp/refresh.json)

# Reusing the OLD refresh token must now fail (401)
curl -s -o /dev/null -w '%{http_code}\n' -X POST "$BASE/auth/refresh" \
  -H 'Content-Type: application/json' \
  -d "{\"refresh_token\":\"$REFRESH\"}"
```

Expected: first refresh 200 with a new pair; reuse of the old token `401`.

### 4.9 Logout revokes session (204, then refresh fails)

```bash
curl -s -o /dev/null -w '%{http_code}\n' -X POST "$BASE/auth/logout" \
  -H "Authorization: Bearer $ACCESS" -H 'Content-Type: application/json' \
  -d "{\"refresh_token\":\"$NEW_REFRESH\"}"

curl -s -o /dev/null -w '%{http_code}\n' -X POST "$BASE/auth/refresh" \
  -H 'Content-Type: application/json' \
  -d "{\"refresh_token\":\"$NEW_REFRESH\"}"
```

Expected: logout `204`; subsequent refresh `401`.

### 4.10 Authorization / ownership (403)

Authenticate as user A, then attempt to `GET /users/{id_of_B}`. Expected `403` unless the caller is `admin`. An admin token may access any user.

```bash
curl -s -o /dev/null -w '%{http_code}\n' \
  "$BASE/users/00000000-0000-0000-0000-000000000000" \
  -H "Authorization: Bearer $ACCESS"
# Expected: 403 (not owner) or 404 (not found) — never 200 for another user's id
```

---

## 5. Automated test validation

Map scenarios to test suites (see `tasks.md` for the exact task breakdown):

| Concern | Command / suite |
|---|---|
| Unit: hashing, JWT, domain invariants, RBAC | `cargo test --lib` |
| Integration: registration, login, sessions, RBAC | `cargo test --test auth_register --test auth_login --test auth_session --test auth_rbac` |
| Migrations apply + seeds | `cargo test --test migration_test` |
| Lint/format | `cargo clippy -- -D warnings && cargo fmt --check` |

Concurrency check (rotation safety): run two `/auth/refresh` calls with the same token concurrently; exactly one must return 200 and the other 401.

---

## 6. Definition of Done (feature-level)

- [ ] `cargo build`, `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` all pass.
- [ ] Migration 004 applies cleanly and rolls back safely.
- [ ] All scenarios in §4 produce the expected outcomes.
- [ ] No `password_hash`, token, or secret appears in any response body or log line.
- [ ] OpenAPI contract (`contracts/openapi.yaml`) matches implemented handlers.
- [ ] Role/permission reconciliation ADR filed (research R11).
