# ADR 0001: Role Vocabulary Reconciliation

- **Status**: Accepted
- **Date**: 2026-11-06
- **Deciders**: Platform Team
- **Related**: `specs/003-identity-access/research.md` R11, `.specify/memory/constitution.md` §Authorization

## Context

Two authoritative documents define role names differently:

- The project constitution lists the canonical RBAC roles as **Admin, Producer, Customer, Support**.
- `docs/06-api-specification.md` §4.2 uses uppercase codes **ADMIN, CUSTOMER, FARMER, SELLER**.

Milestone 3 seeds roles into `platform.roles` and issues role codes inside JWT access tokens. A single vocabulary is required so that seeded data, token claims, and authorization checks agree.

## Decision

The constitution is authoritative. The canonical, case-sensitive role codes are:

`admin`, `producer`, `customer`, `support`

The API specification vocabulary is reconciled as follows and `docs/06` is updated to reference this ADR:

| API spec code | Canonical role | Notes |
| --- | --- | --- |
| `ADMIN` | `admin` | Administrative access, including ownership override. |
| `CUSTOMER` | `customer` | Buyer capabilities. |
| `FARMER` | `producer` | Producer/seller capabilities are owned by the producer role for this milestone. |
| `SELLER` | `producer` | Seller is not a separate role in Milestone 3. |

Implementation rules:

1. Role codes are stored and compared as opaque lowercase strings.
2. RBAC middleware does not hard-code a closed enum; additional role codes can be seeded by migration without code changes.
3. Tokens carry the current canonical codes in the `roles` claim. Role changes require re-authentication to appear in the token.

## Consequences

- Seeded roles and all authorization checks use the four canonical codes.
- Consumers written against the old uppercase/FARMER/SELLER vocabulary must map to the canonical codes.
- If seller-specific behavior diverges from producer behavior in a later milestone, a new role code can be introduced with a migration and an ADR rather than overloading `producer`.
- No database migration is needed to change role names because the canonical codes are seeded directly in `migrations/004_platform_auth.sql`.
