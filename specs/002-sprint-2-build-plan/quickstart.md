# Quickstart: Sprint 2 Build Plan Validation

**Feature**: `002-sprint-2-build-plan` — BioSouk Sprint 2 Build Plan  
**Generated**: 2026-11-06  
**Purpose**: Runnable validation scenarios to prove the Sprint 2 infrastructure works end-to-end, covering database/schema foundation, identity/auth, commerce products+inventory, and traceability/farms+certifications

---

## Prerequisites

- Git installed
- Rust 1.75+ installed (`rustup toolchain install 1.75 && rustup default 1.75`)
- Node.js + npm installed
- Docker + Docker Compose installed (optional — local PostgreSQL also supported)
- PostgreSQL installed and running (local or Docker); default: `postgres://postgres:postgres@127.0.0.1:5432/biosouk_db`
- `cargo sqlx` available (requires `cargo install sqlx-cli`)
- Project already bootstrapped from Sprint 1 (Project Foundation)

---

## Setup Validation Scenarios

### Scenario 1: Sprint 2 Foundation Validation

**Validate** the Sprint 2 build plan artifacts are properly structured and the feature specification is valid.

**Commands**:

```bash
# Verify feature specification exists and is valid JSON/YAML
ls specs/002-sprint-2-build-plan/spec.md

# Validate the spec.md has all mandatory sections
grep -c "## User Scenarios" specs/002-sprint-2-build-plan/spec.md
grep -c "## Requirements" specs/002-sprint-2-build-plan/spec.md
grep -c "## Success Criteria" specs/002-sprint-2-build-plan/spec.md
grep -c "## Assumptions" specs/002-sprint-2-build-plan/spec.md

# Expected outcome: All four mandatory section headings present

# Verify quality checklist exists and passes
ls specs/002-sprint-2-build-plan/checklists/requirements.md
```

**Expected outcome**: Feature specification is well-formed with all mandatory sections present; quality checklist validates successfully.

---

### Scenario 2: Database and Schema Foundation

**Validate** SQLx migrations execute against a local PostgreSQL instance and core schema tables are created.

**Commands**:

```bash
# Ensure PostgreSQL is running
pg_isready -h 127.0.0.1 -U postgres

# Apply SQLx migrations for platform, commerce, and traceability schemas
cd biosouk
cargo sqlx migrate --all --database-url postgres://postgres:postgres@127.0.0.1:5432/biosouk_db

# Expected outcome: Migrations execute successfully; core tables created:
#   platform.users, platform.roles, commerce.categories, commerce.products,
#   traceability.producers, traceability.farms

# Verify tables exist
psql -h 127.0.0.1 -U postgres -d biosouk_db -c "\dt platform.*"
psql -h 127.0.0.1 -U postgres -d biosouk_db -c "\dt commerce.*"
psql -h 127.0.0.1 -U postgres -d biosouk_db -c "\dt traceability.*"

# Rollback migrations to verify safe reversal
cargo sqlx migrate --all --force --database-url postgres://postgres:postgres@127.0.0.1:5432/biosouk_db

# Expected outcome: Database state reverts safely; no data corruption
```

**Expected outcome**: Migrations execute and roll back safely; core schema tables created across all three bounded contexts.

---

### Scenario 3: Identity and Access Domain

**Validate** user registration, authentication, and JWT token generation work correctly.

**Commands**:

```bash
# Start the backend server
cargo run

# In a separate terminal, register a new user
curl -X POST http://127.0.0.1:8080/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"test@biosouk.test","password":"SecurePass123!"}'

# Expected outcome: HTTP 201 Created; response includes user ID and request ID

# Login with the registered credentials
curl -X POST http://127.0.0.1:8080/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"test@biosouk.test","password":"SecurePass123!"}'

# Expected outcome: HTTP 200 OK; JSON response with JWT access token, token type, expires_in

# Verify the JWT token is valid by accessing a protected endpoint
# (the plan.md Technical Context specifies RBAC middleware protects protected endpoints)
curl -H "Authorization: Bearer <access_token>" http://127.0.0.1:8080/auth/me

# Expected outcome: HTTP 200 OK; JSON with current user details (email, status, roles)

# Test invalid registration (duplicate email)
curl -X POST http://127.0.0.1:8080/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"test@biosouk.test","password":"AnotherPass123!"}'

# Expected outcome: HTTP 409 Resource conflict or HTTP 422 Business validation failure

# Test invalid credentials
curl -X POST http://127.0.0.1:8080/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"test@biosouk.test","password":"WrongPassword!"}'

# Expected outcome: HTTP 401 Unauthorized; invalid credentials rejected
```

**Expected outcome**: Users can register and authenticate; JWT access tokens have configured expiration; refresh token rotation works; invalid credentials and duplicate email are rejected; RBAC middleware enforces role-based access on protected endpoints.

---

### Scenario 4: Commerce Domain — Products and Inventory

**Validate** product creation, search, and inventory concurrency safety.

**Commands** (assuming backend is running from Scenario 3):

```bash
# Create a product category first (or use existing)
# Create a product associated with a valid category and farm
curl -X POST http://127.0.0.1:8080/products \
  -H "Authorization: Bearer <access_token>" \
  -H "Content-Type: application/json" \
  -d '{"name":"Organic Tomatoes","description":"Fresh organic tomatoes","category_id":"<category-uuid>","farm_id":"<farm-uuid>","price":{"amount":"4.50","currency":"CAD"},"unit":"kg","organic_status":"certified"}'

# Expected outcome: HTTP 201 Created; product persisted with valid category and farm association

# Search products by category
curl "http://127.0.0.1:8080/products?category_id=<category-uuid>&page=1&page_size=20" \
  -H "Authorization: Bearer <access_token>"

# Expected outcome: HTTP 200 OK; paginated results filtered to specified category

# Search products by organic status
curl "http://127.0.0.1:8080/products?organic_status=certified&page=1&page_size=20" \
  -H "Authorization: Bearer <access_token>"

# Expected outcome: HTTP 200 OK; only certified organic products returned

# Search with price range filter
curl "http://127.0.0.1:8080/products?min_price=1.00&max_price=10.00&page=1&page_size=20" \
  -H "Authorization: Bearer <access_token>"

# Expected outcome: HTTP 200 OK; products within price range returned

# Test inventory concurrency safety — attempt concurrent purchases
# (this validates the overselling prevention from plan.md Constraints section)
curl -X POST http://127.0.0.1:8080/orders \
  -H "Authorization: Bearer <access_token>" \
  -H "Content-Type: application/json" \
  -d '{"items":[{"product_id":"<product-uuid>","quantity":"1.00"}],"shipping_address_id":"<uuid>"}'

# Expected outcome: HTTP 201 Created (if inventory available) or HTTP 409/422 (if inventory insufficient)

# Verify product becomes unavailable when inventory depleted
# (repeat the purchase attempt until inventory exhausted)
```

**Expected outcome**: Products can be created and associated with valid categories and farms; product search supports category, farm, organic status, and price range filters with pagination; inventory quantities consistent (reserved ≤ on-hand); inventory adjustments respect concurrency safety (no overselling); products become unavailable when inventory depleted.

---

### Scenario 5: Traceability Domain — Farms and Certifications

**Validate** farm registration, certification submission, and batch tracking.

**Commands** (assuming backend is running):

```bash
# Register a farm under a producer account
curl -X POST http://127.0.0.1:8080/farms \
  -H "Authorization: Bearer <access_token>" \
  -H "Content-Type: application/json" \
  -d '{"producer_id":"<producer-uuid>","name":"Sunrise Farms","location_description":"Greenhouse facility, Zone 3","farm_type":"greenhouse","status":"active"}'

# Expected outcome: HTTP 201 Created; farm associated with the producer

# Submit a certification
curl -X POST http://127.0.0.1:8080/certifications \
  -H "Authorization: Bearer <access_token>" \
  -H "Content-Type: application/json" \
  -d '{"producer_id":"<producer-uuid>","farm_id":"<farm-uuid>","certification_type":"Organic","certifying_body":"Third-Party Certifier","certificate_number":"ORG-12345","valid_from":"2024-01-01","valid_to":"2025-12-31","verification_status":"pending"}'

# Expected outcome: HTTP 201 Created; certification persisted with valid date range

# Check certification validity (expired)
curl -X POST http://127.0.0.1:8080/certifications/validate \
  -H "Authorization: Bearer <access_token>" \
  -H "Content-Type: application/json" \
  -d '{"certification_id":"<certification-uuid>"}'

# Expected outcome: HTTP 200 OK; status indicates expired if valid_to is in the past

# Create a batch linked to a product, farm, and origin
curl -X POST http://127.0.0.1:8080/batches \
  -H "Authorization: Bearer <access_token>" \
  -H "Content-Type: application/json" \
  -d '{"batch_number":"BATCH-001","product_id":"<product-uuid>","producer_id":"<producer-uuid>","farm_id":"<farm-uuid>","origin_id":"<origin-uuid>","harvest_date":"2024-06-01","production_date":"2024-05-15","expiry_date":"2025-06-01","quantity_produced":"100.00","unit_of_measure":"kg","status":"produced"}'

# Expected outcome: HTTP 201 Created; batch referencing valid product, farm, and origin

# Verify expired certifications are correctly identified
# (create a certification with past valid_to and check validity)
```

**Expected outcome**: Farmers can register farms under their producer account; certifications can be submitted with valid date ranges; expired certifications correctly identified as invalid; batches reference valid products, farms, and origins; certification status transitions follow domain rules.

---

### Scenario 6: Synthetic Data Generation

**Validate** the synthetic data generator produces a standard dataset with reproducible results.

**Commands**:

```bash
# Generate a standard dataset with seed 42
python -m synthetic_data generate --profile standard --seed 42

# Expected outcome: Dataset generated with realistic records across all covered domains:
#   users, farmers, farms, certifications, categories, products,
#   inventory items, inventory reservations, orders, payments,
#   traceability events

# Verify reproducibility — run again with same seed, compare record counts
python -m synthetic_data generate --profile standard --seed 42

# Expected outcome: Identical record counts and referential integrity on second run

# Validate generated dataset respecting domain constraints
python -m synthetic_data validate

# Expected outcome: All generated records pass referential integrity and business rule validation

# Generate edge case scenarios
python -m synthetic_data generate --profile edge-cases --include-expired-certifications --include-zero-inventory

# Expected outcome: Edge case scenarios generated (expired certifications, zero inventory, etc.)
```

**Expected outcome**: Generator produces a standard dataset with reproducible results; generated records respect domain constraints and referential integrity; edge case scenarios generatable on demand.

---

## Validation Summary

All 6 scenarios must pass for Sprint 2 Build Plan to be considered successfully validated. Each scenario maps to a functional requirement from the specification:

| Scenario | Mapped FR |
|----------|-----------|
| 1 — Foundation Validation | FR-005, FR-006 (spec structure) |
| 2 — Database/Schema | FR-005, FR-006 (migrations, core tables) |
| 3 — Identity/Auth | FR-001, FR-002, FR-003 (registration, JWT, refresh) |
| 4 — Commerce/Products+Inventory | FR-007, FR-008, FR-009, FR-010 (product, search, inventory) |
| 5 — Traceability/Farms+Certifications | FR-011, FR-012 (certifications, batches) |
| 6 — Synthetic Data | FR-013, FR-014 (generator, reproducibility) |

---

## Notes

- These validation scenarios are the **minimum** to verify Sprint 2 infrastructure is properly set up
- Additional scenario-specific prerequisites may apply based on individual environment
- Full feature implementation validation will be covered in subsequent guides (Sprint 3+)
- References to contracts and data model details are intentionally summarized (see `data-model.md` for entity details; `contracts/` for interface contracts)
- The synthetic data generator is configured but may not be fully implemented — generation capability is a deliverable, not a runtime dependency at this stage