# BioSouk — Build Plan: Sprint 2

**Sprint:** 2  
**Period:** Following Sprint 1 (Project Foundation)  
**Version:** 1.0  
**Status:** Active

---

## 1. Sprint Goal

Deliver the core domain infrastructure and enable the first bounded-context API surfaces. This sprint transitions from project foundation into database, domain, and API implementation for the Identity and Access, and Commerce bounded contexts.

---

## 2. Sprint Focus

| Focus Area | Key Activities |
|---|---|
| **Database & Schema** | Implement PostgreSQL schemas for platform, commerce, and traceability; establish SQLx migration foundation |
| **Domain Model** | Implement core domain entities: users, roles, permissions, farms, certifications, products, categories, inventory |
| **API Surfaces** | Implement REST endpoints for authentication, farm management, product catalog, and inventory queries |
| **Testing Foundations** | Write unit and integration tests for domain logic, repository behavior, and API contracts |
| **Synthetic Data** | Configure generator with standard profile for development and testing datasets |

---

## 3. Milestones and Deliverables

### Milestone SM-2.1 — Database and Schema Foundation

**Objective:** Establish PostgreSQL schemas and migration infrastructure for the first bounded contexts.

**Deliverables:**

- SQLx migration files for `platform`, `commerce`, and `traceability` schemas
- Schema creation and versioning strategy
- Index definitions for core lookup tables
- Migration test suite validating up/down behavior

**Acceptance criteria:**

- `cargo sqlx migrate` executes migrations against a local PostgreSQL instance
- Migrations can be rolled back safely
- Core schema tables are created: `platform.users`, `platform.roles`, `commerce.categories`, `commerce.products`, `traceability.producers`, `traceability.farms`
- Migration test suite passes 100%

---

### Milestone SM-2.2 — Identity Domain and Authentication

**Objective:** Implement the identity and access domain with full CRUD and authentication support.

**Deliverables:**

- Domain entities: `User`, `Role`, `Permission`
- Repository interfaces and SQLx implementations
- JWT access token and refresh token generation
- Registration and login API endpoints
- Password hashing with Argon2 or bcrypt
- RBAC middleware for authorization

**Acceptance criteria:**

- Users can register with email/password; invalid registration is rejected
- Users can login and receive valid JWT access token
- JWT access tokens have configured expiration
- Refresh token rotation works correctly
- RBAC middleware enforces role-based access on protected endpoints
- Domain invariants (e.g., unique email) are tested

---

### Milestone SM-2.3 — Commerce Domain: Products and Inventory

**Objective:** Implement product catalog and inventory management for the marketplace.

**Deliverables:**

- Domain entities: `Product`, `Category`, `InventoryItem`, `InventoryReservation`
- Repository interfaces and SQLx implementations
- Product creation with category and farm association
- Product search and filtering API endpoints
- Inventory tracking: quantity on hand, reserved quantity, availability calculation
- Inventory adjustment API endpoint with concurrency safety

**Acceptance criteria:**

- Products can be created and associated with valid categories and farms
- Product search supports category, farm, organic status, price range filters
- Inventory quantities are consistent (reserved ≤ on-hand)
- Inventory adjustments respect concurrency safety (no overselling)
- Products become unavailable when inventory is depleted
- Search and listing endpoints return correct pagination

---

### Milestone SM-2.4 — Traceability Domain: Farms and Certifications

**Objective:** Implement farm and certification management for organic produce traceability.

**Deliverables:**

- Domain entities: `Producer`, `Farm`, `Certification`, `Batch`
- Repository interfaces and SQLx implementations
- Farm registration associated with a producer
- Certification submission and review workflow
- Batch creation linked to products, farms, and origins
- Certification validity checking (expired, pending, valid states)

**Acceptance criteria:**

- Farmers can register farms under their producer account
- Certifications can be submitted with valid date ranges
- Expired certifications are correctly identified as invalid
- Batches reference valid products, farms, and origins
- Certification status transitions follow domain rules

---

### Milestone SM-2.5 — Synthetic Data and Testing Foundation

**Objective:** Establish synthetic data generation capability and comprehensive test coverage for the milestones completed in this sprint.

**Deliverables:**

- Synthetic data generator configuration with `standard` profile
- Dataset profiles: minimal, standard, large, demo, edge-cases
- Seed-based reproducibility configuration
- Referential integrity validation for generated data
- Integration test fixtures for database and API endpoints

**Acceptance criteria:**

- Generator produces a `standard` dataset with realistic records across all covered domains
- Dataset is reproducible with the same seed
- Generated records respect domain constraints and referential integrity
- Edge case scenarios (expired certifications, zero inventory, failed payments) are generatable
- Integration test fixtures cover authentication, product creation, and inventory queries

---

## 4. Sprint Dependencies

```
SM-2.1 (Database/Schema)       →  SM-2.2 (Identity)       →  SM-2.3 (Commerce)
       ↑                                    |
       └────────────────── SM-2.4 ──────────┘
                                                 ↓
                                          SM-2.5 (Synthetic Data / Testing)
```

**Parallel workstreams (may begin once interfaces are stable):**

- Frontend wireframes and UI components for authentication and product catalog can begin during backend foundation work
- Synthetic data generator design can begin during database implementation
- Traceability can progress independently of payment integration after the relevant domain interfaces exist

---

## 5. Definition of Done (Sprint 2)

A milestone is complete only when:

- [ ] Required functionality is implemented per the deliverables
- [ ] Domain rules are enforced and tested
- [ ] Automated tests pass (unit, integration, API)
- [ ] Security requirements are satisfied (auth, RBAC, password hashing)
- [ ] API documentation is updated (OpenAPI)
- [ ] Relevant architectural decisions are documented
- [ ] Code passes formatting (`cargo fmt`) and linting (`cargo clippy`)
- [ ] No unresolved critical defects remain
- [ ] Changes are committed to Git with conventional commit messages
- [ ] Synthetic data generator produces a valid standard dataset

**Sprint 2 is complete when all five milestones (SM-2.1 through SM-2.5) meet their acceptance criteria.**

---

## 6. Acceptance Criteria Summary

| Milestone | Key Deliverable |
|---|---|
| SM-2.1 | SQLx migrations executed; core schemas created |
| SM-2.2 | Auth registration/login; JWT tokens; RBAC middleware |
| SM-2.3 | Product catalog; inventory with concurrency safety |
| SM-2.4 | Farm/certification management; batch tracking |
| SM-2.5 | Synthetic data generator; test fixtures ready |

---

## 7. Git Workflow

- Branch from `develop` for sprint work
- Feature branches: `feature/sm-2-x-description`
- Merge via pull request into `develop`
- Conventional commit messages required

**Example commit messages:**

```bash
git commit -m "feat(biosouk): add SQLx migration for platform schema (SM-2.1)"
git commit -m "feat(biosouk): implement user registration and JWT auth (SM-2.2)"
git commit -m "feat(biosouk): add product catalog with search and filtering (SM-2.3)"
git commit -m "feat(biosouk): implement farm and certification management (SM-2.4)"
git commit -m "feat(biosouk): configure synthetic data generator with standard profile (SM-2.5)"
```

---

## 8. References

- `docs/01-product-specification.md` — Product requirements
- `docs/02-domain-model.md` — Domain model entities and relationships
- `docs/03-system-architecture.md` — System architecture and bounded contexts
- `docs/04-domain-workflows.md` — Domain workflows and use cases
- `docs/05-database-design.md` — Database design and schema details
- `docs/06-api-specification.md` — API contracts and endpoints
- `docs/07-synthetic-data-generation.md` — Synthetic data generator specification
- `docs/08-implementation-plan.md` — Full implementation plan and milestones M1–M11
- `docs/constitution.md` — Project constitution and engineering standards
- `quickstart.md` — Project foundation validation scenarios