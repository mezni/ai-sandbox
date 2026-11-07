# API Specification

**Project:** BioSouk  
**Document:** `docs/06-api-specification.md`  
**Version:** 1.0  
**Status:** Active

---

## 1. Purpose

This document defines the public and internal REST API contracts for BioSouk.

It establishes:

- API conventions and versioning.
- Authentication and authorization.
- Resource endpoints.
- Request and response DTOs.
- Validation rules.
- Error handling.
- Pagination and filtering.
- Idempotency requirements.
- API security requirements.
- OpenAPI documentation requirements.

The API acts as the interface between the React frontend, external integrations, and BioSouk's backend application.

---

## 2. API Design Principles

The API MUST follow these principles:

1. RESTful resource-oriented design.
2. Consistent naming and response structures.
3. Explicit request and response DTOs.
4. Strict input validation.
5. Authentication for protected operations.
6. Role-based authorization.
7. Domain ownership of business rules.
8. No direct database access from API handlers.
9. Consistent error responses.
10. Versioned API contracts.
11. Idempotency for sensitive operations.
12. Traceability of business-critical operations.

API handlers MUST delegate business operations to their respective application services.

---

## 3. API Conventions

### 3.1 Base URL

Development:

`http://localhost:8080/api/v1`

Production:

`https://api.biosouk.com/api/v1`

### 3.2 HTTP Methods

| Method | Purpose |
|---|---|
| GET | Retrieve resources |
| POST | Create resources or execute commands |
| PUT | Replace resources |
| PATCH | Partially update resources |
| DELETE | Remove resources where permitted |

### 3.3 Naming Conventions

- Use lowercase resource names.
- Use plural nouns for collections.
- Use kebab-case for multiword URL segments.
- Use UUIDs for resource identifiers.
- Use ISO 8601 timestamps in UTC.
- Use JSON for request and response bodies.

Example:

`GET /api/v1/products/{product_id}`

### 3.4 Standard Headers

| Header | Description |
|---|---|
| Authorization | Bearer JWT token |
| Content-Type | application/json |
| Accept | application/json |
| X-Request-ID | Request correlation identifier |
| Idempotency-Key | Unique identifier for retry-safe operations |

---

## 4. Authentication and Authorization

### 4.1 Authentication

BioSouk uses JWT-based authentication.

Endpoints:

| Method | Endpoint | Description |
|---|---|---|
| POST | `/auth/register` | Register an account |
| POST | `/auth/login` | Authenticate user |
| POST | `/auth/refresh` | Refresh access token |
| POST | `/auth/logout` | Revoke refresh session |
| GET | `/auth/me` | Retrieve current user |

### 4.2 Authorization

Protected endpoints require a valid access token.

Authorization is enforced through RBAC and domain-level ownership checks.

Roles include:

- `admin`
- `producer`
- `customer`
- `support`

Role codes are canonical per the project constitution. The API's former
`CUSTOMER`, `FARMER`, `SELLER`, and `ADMIN` vocabulary is reconciled by
[ADR 0001](adr/0001-role-vocabulary-reconciliation.md): `FARMER` and `SELLER`
map to `producer` for this milestone.

A user may have multiple roles where permitted by the domain model.

### 4.3 Authorization Rules

- Customers manage their own accounts and orders.
- Farmers manage their own farm information and production records.
- Sellers manage their own marketplace listings.
- Administrators manage platform-level operations.
- Sensitive administrative operations require explicit authorization.

Possessing a role does not automatically grant access to another user's resources.

---

## 5. API Modules

The API is organized around BioSouk's bounded contexts.

### 5.1 Identity and Access

| Method | Endpoint | Description |
|---|---|---|
| POST | `/auth/register` | Register |
| POST | `/auth/login` | Login |
| POST | `/auth/refresh` | Refresh token |
| POST | `/auth/logout` | Logout |
| GET | `/auth/me` | Current user |
| GET | `/users/{user_id}` | Retrieve user |
| PATCH | `/users/{user_id}` | Update user |

### 5.2 Farm Management

| Method | Endpoint | Description |
|---|---|---|
| POST | `/farms` | Create farm |
| GET | `/farms` | List farms |
| GET | `/farms/{farm_id}` | Retrieve farm |
| PATCH | `/farms/{farm_id}` | Update farm |
| GET | `/farms/{farm_id}/certifications` | List certifications |
| POST | `/farms/{farm_id}/certifications` | Submit certification |

### 5.3 Product Catalog

| Method | Endpoint | Description |
|---|---|---|
| POST | `/products` | Create product |
| GET | `/products` | Search products |
| GET | `/products/{product_id}` | Retrieve product |
| PATCH | `/products/{product_id}` | Update product |
| DELETE | `/products/{product_id}` | Archive product |
| GET | `/products/{product_id}/traceability` | Retrieve traceability information |

Product search supports:

- Category.
- Farm.
- Organic status.
- Availability.
- Price range.
- Geographic location.
- Certification status.

### 5.4 Inventory

| Method | Endpoint | Description |
|---|---|---|
| GET | `/inventory` | List inventory |
| GET | `/inventory/{product_id}` | Retrieve inventory |
| POST | `/inventory/adjustments` | Record adjustment |
| GET | `/inventory/{product_id}/movements` | Retrieve movements |

Inventory changes MUST be validated by the inventory domain.

### 5.5 Orders

| Method | Endpoint | Description |
|---|---|---|
| POST | `/orders` | Create order |
| GET | `/orders` | List customer orders |
| GET | `/orders/{order_id}` | Retrieve order |
| POST | `/orders/{order_id}/cancel` | Request cancellation |
| GET | `/orders/{order_id}/items` | Retrieve order items |

Order creation MUST validate product availability and applicable pricing rules.

### 5.6 Payments

| Method | Endpoint | Description |
|---|---|---|
| POST | `/payments` | Initiate payment |
| GET | `/payments/{payment_id}` | Retrieve payment |
| POST | `/payments/{payment_id}/refund` | Request refund |
| POST | `/payments/webhooks/{provider}` | Receive payment provider events |

Payment processing MUST use a trusted payment provider.

Payment webhooks MUST be authenticated and idempotently processed.

### 5.7 Traceability

Traceability is a core BioSouk capability.

| Method | Endpoint | Description |
|---|---|---|
| GET | `/traceability/products/{product_id}` | Product traceability |
| GET | `/traceability/batches/{batch_id}` | Batch history |
| GET | `/traceability/batches/{batch_id}/events` | Batch events |
| GET | `/traceability/products/{product_id}/journey` | Product journey |

Traceability responses MUST expose only verified and authorized information.

The API MUST NOT represent unverified claims as certified organic status.

### 5.8 Administration and Audit

| Method | Endpoint | Description |
|---|---|---|
| GET | `/admin/users` | List users |
| PATCH | `/admin/users/{user_id}/status` | Change account status |
| GET | `/admin/certifications` | Review certifications |
| POST | `/admin/certifications/{id}/approve` | Approve certification |
| POST | `/admin/certifications/{id}/reject` | Reject certification |
| GET | `/admin/audit-logs` | Retrieve audit records |

Administrative operations MUST generate audit records.

---

## 6. Request and Response DTOs

API DTOs MUST be separate from database models and internal domain entities.

### 6.1 Product Creation Request

`POST /api/v1/products`

Example:

```json
{
  "name": "Organic Tomatoes",
  "description": "Fresh organic tomatoes",
  "category_id": "uuid",
  "farm_id": "uuid",
  "price": {
    "amount": "4.50",
    "currency": "CAD"
  },
  "unit": "kg",
  "organic_status": "certified"
}
```

### 6.2 Product Response

```json
{
  "id": "uuid",
  "name": "Organic Tomatoes",
  "description": "Fresh organic tomatoes",
  "farm_id": "uuid",
  "price": {
    "amount": "4.50",
    "currency": "CAD"
  },
  "unit": "kg",
  "organic_status": "certified",
  "availability": "available",
  "created_at": "2026-11-06T12:00:00Z",
  "updated_at": "2026-11-06T12:00:00Z"
}
```

The API MUST use decimal-safe monetary representations rather than floating-point arithmetic.

Organic certification status MUST be derived from validated domain records, not trusted directly from client input.

### 6.3 Order Creation Request

`POST /api/v1/orders`

```json
{
  "items": [
    {
      "product_id": "uuid",
      "quantity": "2.00"
    }
  ],
  "shipping_address_id": "uuid"
}
```

The server MUST determine prices, calculate totals, validate availability, and apply business rules.

Clients MUST NOT supply authoritative order totals.

### 6.4 Standard Success Response

```json
{
  "data": {
    "id": "uuid"
  },
  "meta": {
    "request_id": "uuid"
  }
}
```

Collection responses may additionally include pagination metadata.

---

## 7. Validation Rules

All incoming requests MUST be validated before executing application use cases.

Validation includes:

- Required fields.
- Field length restrictions.
- Valid UUIDs.
- Valid email addresses.
- Positive quantities.
- Valid currency codes.
- Valid enum values.
- Valid timestamps.
- Resource ownership.
- Business-rule constraints.

Domain invariants MUST be enforced by the domain/application layers, not exclusively by HTTP validation.

Invalid requests MUST NOT produce partial business operations.

---

## 8. Error Handling

### 8.1 Standard Error Format

All API errors MUST follow a consistent structure.

```json
{
  "error": {
    "code": "PRODUCT_NOT_AVAILABLE",
    "message": "The requested product is unavailable.",
    "details": [],
    "request_id": "uuid"
  }
}
```

### 8.2 HTTP Status Codes

| Status | Meaning |
|---|---|
| 200 | Successful request |
| 201 | Resource created |
| 202 | Request accepted for asynchronous processing |
| 204 | Successful operation without response body |
| 400 | Invalid request |
| 401 | Authentication required |
| 403 | Permission denied |
| 404 | Resource not found |
| 409 | Resource conflict |
| 422 | Business validation failure |
| 429 | Rate limit exceeded |
| 500 | Internal server error |
| 503 | Service temporarily unavailable |

### 8.3 Error Codes

Examples:

- `VALIDATION_ERROR`
- `UNAUTHORIZED`
- `FORBIDDEN`
- `RESOURCE_NOT_FOUND`
- `PRODUCT_NOT_AVAILABLE`
- `INSUFFICIENT_INVENTORY`
- `ORDER_ALREADY_CANCELLED`
- `PAYMENT_FAILED`
- `CERTIFICATION_NOT_VALID`
- `IDEMPOTENCY_CONFLICT`
- `INTERNAL_ERROR`

Internal implementation details, database errors, credentials, and stack traces MUST never be exposed to API clients.

---

## 9. Pagination, Filtering and Sorting

Collection endpoints MUST support pagination.

Example:

`GET /api/v1/products?page=1&page_size=20`

Supported parameters:

| Parameter | Description |
|---|---|
| page | Page number |
| page_size | Number of records |
| sort_by | Sorting field |
| sort_order | asc or desc |
| search | Text search |
| category_id | Filter by category |
| farm_id | Filter by farm |
| min_price | Minimum price |
| max_price | Maximum price |

Default page size: 20.

Maximum page size: 100.

The backend MUST validate pagination and sorting parameters.

### Pagination Response

```json
{
  "data": [],
  "meta": {
    "page": 1,
    "page_size": 20,
    "total_items": 120,
    "total_pages": 6
  }
}
```

---

## 10. Idempotency

Idempotency MUST be supported for operations where duplicate execution could cause financial or business inconsistencies.

Examples:

- Order creation.
- Payment initiation.
- Refund requests.
- Payment webhook processing.
- Inventory adjustments.

Clients MUST provide an `Idempotency-Key` header for supported operations.

The backend MUST:

1. Associate the key with the authenticated client and operation.
2. Store the original operation result.
3. Return the original result for a valid retry.
4. Reject reuse with a different request payload.
5. Apply an explicit retention policy.

---

## 11. Security Requirements

The API MUST enforce:

- HTTPS in production.
- JWT signature and expiration validation.
- Role-based access control.
- Resource ownership validation.
- Request payload validation.
- Rate limiting.
- CORS restrictions.
- Secure secret management.
- SQL injection protection through parameterized queries.
- Protection against mass assignment.
- Audit logging for sensitive operations.
- Secure payment webhook verification.

Sensitive information MUST NOT appear in application logs.

---

## 12. Asynchronous Processing

Long-running or externally dependent operations MUST NOT block HTTP request handling unnecessarily.

Examples:

- Payment provider interactions.
- External certification verification.
- Notification delivery.
- Large traceability exports.

The API may return HTTP 202 when processing continues asynchronously.

Example:

```json
{
  "data": {
    "operation_id": "uuid",
    "status": "pending"
  }
}
```

The API MUST expose an appropriate status endpoint for asynchronous operations where clients need to track completion.

---

## 13. API Architecture

The request lifecycle follows:

```text
React / External Client
        |
        v
   Axum Router
        |
        v
 Middleware
 - Authentication
 - Authorization
 - Request ID
 - Rate Limiting
        |
        v
   API Handler
        |
        v
   Request DTO
        |
        v
 Application Service
        |
        v
   Domain Model
        |
        v
 Repository Interface
        |
        v
 SQLx Repository
        |
        v
    PostgreSQL
```

Rules:

- Handlers MUST remain thin.
- Business logic MUST NOT live in handlers.
- Repositories MUST be accessed through application/domain-defined interfaces.
- Each bounded context owns its business operations.
- Cross-context communication MUST use defined application interfaces or domain events.
- API DTOs MUST NOT expose persistence models.

---

## 14. OpenAPI Documentation

BioSouk MUST maintain an OpenAPI 3.x specification.

The specification MUST document:

- Endpoints and HTTP methods.
- Request schemas.
- Response schemas.
- Authentication requirements.
- Authorization requirements.
- Query parameters.
- Error responses.
- Idempotency requirements.
- Examples.

The OpenAPI specification MUST remain synchronized with the implemented API.

Recommended endpoints:

- `/docs` — Interactive API documentation.
- `/openapi.json` — OpenAPI specification.

These endpoints may be restricted in production.

---

## 15. API Testing Requirements

API tests MUST cover:

### Functional Testing

- Successful resource creation.
- Resource retrieval.
- Resource updates.
- Resource deletion or archival.
- Search and filtering.
- Pagination.

### Security Testing

- Missing JWT.
- Invalid JWT.
- Expired JWT.
- Insufficient permissions.
- Unauthorized resource access.
- Invalid ownership.

### Business Rule Testing

- Invalid organic certification.
- Unavailable products.
- Insufficient inventory.
- Invalid order cancellation.
- Duplicate payment requests.

### Reliability Testing

- Idempotent retries.
- Concurrent requests.
- Payment webhook retries.
- Database failures.
- External provider failures.

Tests MUST include both successful and failure scenarios.

---

## 16. Non-Negotiable Rules

1. All public APIs MUST be versioned.
2. All protected endpoints MUST enforce authentication.
3. All sensitive operations MUST enforce authorization.
4. Business rules MUST remain inside their owning domains.
5. API DTOs MUST remain independent of database entities.
6. Monetary values MUST use decimal-safe representations.
7. Organic certification claims MUST be backed by verified records.
8. Payment operations MUST support idempotency.
9. Errors MUST follow the standard error format.
10. Sensitive operations MUST be auditable.
11. API changes MUST preserve backward compatibility within a version or explicitly introduce a new version.
12. OpenAPI documentation MUST reflect the actual implementation.

---

## 17. Definition of Done

The API specification is considered complete when:

- [ ] All bounded-context API responsibilities are documented.
- [ ] Resource endpoints are defined.
- [ ] Request and response DTO conventions are established.
- [ ] Authentication and authorization rules are defined.
- [ ] Validation rules are documented.
- [ ] Error response conventions are defined.
- [ ] Pagination and filtering are specified.
- [ ] Idempotency requirements are established.
- [ ] Security requirements are documented.
- [ ] OpenAPI requirements are established.
- [ ] API testing requirements are documented.

---

**End of Document**
