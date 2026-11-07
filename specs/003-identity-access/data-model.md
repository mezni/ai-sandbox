# Data Model: Identity and Access Management

**Feature**: `specs/003-identity-access`
**Domain**: Platform Domain (constitution §98–116)
**Sources**: `docs/05-database-design.md` §4.2–4.8, `docs/06-api-specification.md` §4–§8, `docs/04-domain-workflows.md` §16, feature `spec.md`.
**Generated**: 2026-11-06 (Phase 1 of `/speckit.plan`)

This document defines persistence entities, their constraints and state transitions, and the API-facing DTOs. DTOs are intentionally separate from persistence/domain models (API spec §258).

---

## 1. Persistence Entities

### 1.1 User (`platform.users`)

| Field | Type | Constraints |
|---|---|---|
| `id` | UUID | PK, default `gen_random_uuid()` |
| `email` | VARCHAR(320) | NOT NULL; normalized (lowercased, trimmed); unique via `uq_users_email` and `idx_platform_users_email_lower` |
| `password_hash` | TEXT | NOT NULL; Argon2id PHC string only |
| `status` | VARCHAR(30) | NOT NULL; CHECK in (`pending`,`active`,`suspended`,`closed`) |
| `email_verified_at` | TIMESTAMPTZ | NULL |
| `created_at` | TIMESTAMPTZ | NOT NULL, default `NOW()` |
| `updated_at` | TIMESTAMPTZ | NOT NULL, default `NOW()`; bumped on mutation |

**Invariants**
- Email is normalized and case-insensitively unique.
- `password_hash` is never returned by any API or written to logs.
- Only `pending` or `active` users may authenticate (`can_authenticate`).
- Registration creates `pending` users; `email_verified_at` is set only when verification is implemented (out of scope).

**State transitions (`UserStatus`)**
```
pending --activate--> active
active  --suspend-->  suspended
suspended --activate--> active
pending/active --close--> closed   (terminal)
suspended --close--> closed
```
Rules: `closed` is terminal; only admins may `suspend`/`close`; a user may not authenticate while `suspended`/`closed`.

**Relationships**
- M:N with Role via `platform.user_roles`.
- 1:N with RefreshToken (`refresh_tokens.user_id`).

### 1.2 Role (`platform.roles`)

| Field | Type | Constraints |
|---|---|---|
| `id` | UUID | PK, default `gen_random_uuid()` |
| `code` | VARCHAR(50) | NOT NULL, UNIQUE |
| `name` | VARCHAR(100) | NOT NULL |
| `description` | TEXT | NULL |
| `created_at` | TIMESTAMPTZ | NOT NULL, default `NOW()` |

**Canonical seeded codes** (constitution §239–244): `admin`, `producer`, `customer`, `support`.
**Note**: Do **not** add `status`/`updated_at` columns — they are not in the design; the existing `Role` model's assumptions must be corrected (see research R9).

**Relationships**
- M:N with Permission via `role_permissions`.
- M:N with User via `user_roles`.

### 1.3 Permission (`platform.permissions`)

| Field | Type | Constraints |
|---|---|---|
| `id` | UUID | PK, default `gen_random_uuid()` |
| `code` | VARCHAR(100) | NOT NULL, UNIQUE (e.g., `users.read`, `users.update`, `roles.assign`) |
| `description` | TEXT | NULL |

**Relationships**: M:N with Role via `role_permissions`. Permissions are not assigned directly to users.

### 1.4 UserRole (`platform.user_roles`)

| Field | Type | Constraints |
|---|---|---|
| `user_id` | UUID | NOT NULL, FK → `platform.users(id)` ON DELETE CASCADE |
| `role_id` | UUID | NOT NULL, FK → `platform.roles(id)` ON DELETE CASCADE |
| `assigned_at` | TIMESTAMPTZ | NOT NULL, default `NOW()` |
| `assigned_by` | UUID | NULL |

**PK**: (`user_id`, `role_id`).

### 1.5 RolePermission (`platform.role_permissions`)

| Field | Type | Constraints |
|---|---|---|
| `role_id` | UUID | NOT NULL, FK → `platform.roles(id)` ON DELETE CASCADE |
| `permission_id` | UUID | NOT NULL, FK → `platform.permissions(id)` ON DELETE CASCADE |

**PK**: (`role_id`, `permission_id`).

### 1.6 RefreshToken (`platform.refresh_tokens`) — NEW (migration 004)

| Field | Type | Constraints |
|---|---|---|
| `id` | UUID | PK, default `gen_random_uuid()` |
| `user_id` | UUID | NOT NULL, FK → `platform.users(id)` ON DELETE CASCADE |
| `token_hash` | TEXT | NOT NULL, UNIQUE (SHA-256 of the opaque token) |
| `expires_at` | TIMESTAMPTZ | NOT NULL |
| `revoked_at` | TIMESTAMPTZ | NULL |
| `created_at` | TIMESTAMPTZ | NOT NULL, default `NOW()` |

**Indexes**: `idx_platform_refresh_tokens_user_id (user_id)`, `idx_platform_refresh_tokens_expires_at (expires_at)`. Unique constraint on `token_hash`.

**Invariants**
- Only the hash is stored; the raw token exists only in the response to the client.
- A token is *active* iff `revoked_at IS NULL AND expires_at > now()`.

**Lifecycle**
```
issued (active) --refresh--> rotated: old row set revoked_at=now(); new active row created (same tx)
issued (active) --logout-->  revoked: revoked_at=now()
issued (active) --expiry-->  expired (expires_at <= now()) → inactive
```
Rotation is atomic: revoking the old token and inserting the new one occur in one transaction.

### 1.7 AuditLog (`platform.audit_logs`) — NEW (migration 004)

| Field | Type | Constraints |
|---|---|---|
| `id` | UUID | PK, default `gen_random_uuid()` |
| `actor_id` | UUID | NULL (system actions have no actor) |
| `action` | VARCHAR(100) | NOT NULL (e.g., `user.registered`, `user.login`, `user.login_failed`, `token.refreshed`, `user.logged_out`, `role.assigned`) |
| `resource_type` | VARCHAR(100) | NOT NULL (e.g., `user`, `refresh_token`) |
| `resource_id` | UUID | NULL |
| `metadata` | JSONB | NOT NULL, default `'{}'` |
| `correlation_id` | UUID | NULL (maps to request id) |
| `created_at` | TIMESTAMPTZ | NOT NULL, default `NOW()` |

**Invariants**
- Append-only.
- MUST NOT contain passwords, password hashes, access tokens, refresh tokens, or other secrets (DB design §296).
- Security-relevant actions (register, login success/failure, refresh, logout, status change, role assignment) MUST produce a record (constitution §255).

---

## 2. Entity Relationship Diagram

```text
platform.users 1───M platform.user_roles M───1 platform.roles
                                                   │ M
                                                   │
                                                   M
                                          platform.role_permissions
                                                   │ M
                                                   │
                                                   1
                                          platform.permissions

platform.users 1───M platform.refresh_tokens
platform.users 1───M platform.audit_logs (actor_id, nullable)
```

---

## 3. API DTOs (presentation layer; separate from persistence)

> All request bodies validated before use-case execution (API spec §344–359). Timestamps ISO 8601 UTC. IDs are UUID strings.

### 3.1 RegisterRequest (`POST /api/v1/auth/register`)
```json
{ "email": "user@example.com", "password": "S3cure-passphrase!" }
```
Rules: valid email; password length ≥ 12 (configurable) and ≤ 128; email normalized before uniqueness check.

### 3.2 LoginRequest (`POST /api/v1/auth/login`)
```json
{ "email": "user@example.com", "password": "S3cure-passphrase!", "remember_me": false }
```
`remember_me` (optional, default false) extends refresh lifetime when supported.

### 3.3 RefreshRequest (`POST /api/v1/auth/refresh`)
```json
{ "refresh_token": "<opaque>" }
```

### 3.4 LogoutRequest (`POST /api/v1/auth/logout`)
```json
{ "refresh_token": "<opaque>" }
```
Requires valid access token; revokes the given refresh token.

### 3.5 TokenResponse (register/login/refresh `data`)
```json
{
  "access_token": "<jwt>",
  "refresh_token": "<opaque>",
  "token_type": "Bearer",
  "expires_in": 900,
  "refresh_expires_in": 2592000
}
```

### 3.6 UserResponse (`GET /auth/me`, `GET /users/{id}`, register/login metadata)
```json
{
  "id": "uuid",
  "email": "user@example.com",
  "status": "pending",
  "roles": ["customer"],
  "email_verified_at": null,
  "created_at": "2026-11-06T12:00:00Z",
  "updated_at": "2026-11-06T12:00:00Z"
}
```
MUST NOT include `password_hash`, token material, or internal fields.

### 3.7 UpdateUserRequest (`PATCH /api/v1/users/{user_id}`)
```json
{ "email": "new@example.com" }
```
Updatable fields: `email` only in this milestone (profile extension later). Requires ownership or `admin`; email re-normalized and re-validated for uniqueness.

### 3.8 Envelopes
- Success: `{ "data": <payload>, "meta": { "request_id": "uuid" } }`
- Error: `{ "error": { "code": "UNAUTHORIZED", "message": "...", "details": [], "request_id": "uuid" } }`

Relevant error codes: `VALIDATION_ERROR`, `UNAUTHORIZED`, `FORBIDDEN`, `RESOURCE_NOT_FOUND`, `EMAIL_ALREADY_EXISTS` (maps to HTTP 409), `INVALID_CREDENTIALS` (401), `TOKEN_EXPIRED`, `INTERNAL_ERROR`.

---

## 4. Validation Rules

| Rule | Applies to | Enforcement layer |
|---|---|---|
| Email format + normalization | register, update | DTO validation + domain invariant |
| Email uniqueness (case-insensitive) | register, update | DB unique index + repo check → `EMAIL_ALREADY_EXISTS` |
| Password length/complexity | register | DTO validation |
| Password strength/hash | register | domain → Argon2id adapter |
| Credentials validity | login | application use case (verify hash); identical error for unknown user vs wrong password |
| Role code exists | assignment | application/domain |
| Refresh token active (not revoked/expired) | refresh, logout | repository query within tx |
| Ownership or admin | get/update user | authorization guard |
| No secrets in output/logs | all | serializers + logging policy |

---

## 5. Assumptions

- Email verification, password reset, and MFA are out of scope for this milestone; `email_verified_at` remains nullable and not enforced for login.
- Additional profile columns (name, phone) will be added in a later milestone; `PATCH /users/{id}` limited to `email` now.
- Audit retention/purge policy is deferred; append-only for now.
- Role/permission seeds are exactly the constitution's four roles plus a minimal permission set (`users.read`, `users.update`, `roles.assign`, `audit.read`).
