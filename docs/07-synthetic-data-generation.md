# Synthetic Data Generation Specification

**Project:** BioSouk  
**Document:** `docs/07-synthetic-data-generation.md`  
**Version:** 1.0  
**Status:** Active

---

## 1. Purpose

BioSouk requires realistic data to support:

- Application development.
- Integration testing.
- Frontend demonstrations.
- Database validation.
- Marketplace simulations.
- Analytics and reporting.
- AI/ML experimentation.

The synthetic data generator MUST produce realistic, internally consistent data without depending on real customer or farmer information.

The generator MUST be reusable, configurable, deterministic, and independent of production data.

## 2. Design Principles

1. All generated data MUST be synthetic.
2. Generated data MUST respect domain constraints.
3. Generation MUST be reproducible using a random seed.
4. Related entities MUST maintain referential integrity.
5. Business rules MUST be respected.
6. Generated datasets MUST support different testing scenarios.
7. Generation MUST be configurable.
8. Sensitive production data MUST never be copied.
9. Generated data MUST be distinguishable from production data.
10. The generator MUST support incremental development.

## 3. Data Generation Scope

The generator covers the following domain entities.

| Domain | Synthetic Data |
|---|---|
| Identity | Users, roles, accounts |
| Agriculture | Farmers, farms, farm locations |
| Certification | Organic certifications, verification records |
| Catalog | Categories, products, product attributes |
| Inventory | Stock levels, batches, movements |
| Marketplace | Product listings, prices, availability |
| Orders | Orders, order items, statuses |
| Payments | Payment transactions, refunds |
| Traceability | Production, harvesting, processing, distribution events |

Additional entities may be introduced when the domain model evolves.

## 4. Dataset Profiles

The generator MUST support predefined dataset profiles.

| Profile | Purpose | Characteristics |
|---|---|---|
| Minimal | Local development | Small dataset |
| Standard | Integration testing | Realistic marketplace |
| Large | Performance testing | High-volume records |
| Demo | Product demonstrations | Curated realistic scenarios |
| Edge Cases | Business-rule testing | Invalid and exceptional scenarios |

Profiles MUST be configurable rather than hardcoded into generation logic.

## 5. Generation Dependencies

Data generation MUST follow dependency order.

```text
Users
  |
  v
Farmers
  |
  v
Farms
  |
  v
Certifications
  |
  v
Product Categories
  |
  v
Products
  |
  v
Inventory & Batches
  |
  v
Product Listings
  |
  v
Orders
  |
  v
Payments
  |
  v
Traceability Events
```

The generator MUST create parent records before dependent records.

Generation MUST respect the actual relationships defined in the domain model.

## 6. Realism Requirements

Generated data SHOULD represent realistic agricultural marketplace scenarios.

### 6.1 Farmers and Farms

Generate:

- Farmer profiles.
- Farm names.
- Farm locations.
- Farm sizes.
- Agricultural activities.
- Production capabilities.

Geographic data SHOULD reflect the intended BioSouk operating region.

### 6.2 Certifications

Generate:

- Certification type.
- Issuing organization.
- Issue date.
- Expiration date.
- Verification status.
- Associated farm.

Certification states MUST include realistic valid, expired, pending, and rejected scenarios.

Synthetic certification records MUST NOT be presented as genuine third-party certifications.

### 6.3 Products

Generate products such as:

- Tomatoes.
- Potatoes.
- Carrots.
- Apples.
- Strawberries.
- Lettuce.
- Cucumbers.
- Onions.

Product attributes include:

- Name.
- Category.
- Description.
- Unit.
- Price.
- Organic status.
- Farm ownership.
- Availability.

Product generation MUST respect category and farm relationships.

### 6.4 Inventory

Generate:

- Product batches.
- Available quantities.
- Reserved quantities.
- Inventory movements.
- Expiration dates where applicable.
- Stock adjustments.

Inventory quantities MUST remain consistent with the inventory business rules.

### 6.5 Orders and Payments

Generate realistic customer transactions.

Order scenarios MUST include:

- Pending orders.
- Confirmed orders.
- Processing orders.
- Shipped orders.
- Delivered orders.
- Cancelled orders.

Payment scenarios MUST include:

- Pending payments.
- Successful payments.
- Failed payments.
- Refunds.

Order totals MUST be calculated from order items and applicable pricing rules.

### 6.6 Traceability

Generate traceability events such as:

- Production started.
- Harvest completed.
- Quality inspection.
- Packaging completed.
- Distribution started.
- Delivery completed.

Events MUST maintain chronological consistency and reference valid products or batches.

## 7. Generator Architecture

The generator MUST be implemented as a reusable application component.

Suggested structure:

```text
synthetic-data/
├── config/
│   ├── profiles.yaml
│   └── generator.yaml
│
├── src/
│   ├── generators/
│   │   ├── users.py
│   │   ├── farms.py
│   │   ├── certifications.py
│   │   ├── products.py
│   │   ├── inventory.py
│   │   ├── orders.py
│   │   ├── payments.py
│   │   └── traceability.py
│   │
│   ├── orchestration/
│   │   ├── pipeline.py
│   │   └── dependencies.py
│   │
│   ├── validation/
│   │   ├── integrity.py
│   │   └── business_rules.py
│   │
│   ├── exporters/
│   │   ├── json_exporter.py
│   │   └── database_loader.py
│   │
│   └── cli.py
│
├── tests/
│   ├── test_generators.py
│   ├── test_integrity.py
│   └── test_reproducibility.py
│
└── README.md
```

The implementation language and runtime MUST follow the project's established technology decisions.

The generator MUST remain logically separate from BioSouk's production application.

## 8. Configuration

Generation MUST be configurable through YAML configuration files and CLI arguments.

Example:

```yaml
profile: standard

seed: 42

region:
  country: Canada
  currency: CAD

volumes:
  users: 100
  farmers: 20
  farms: 25
  certifications: 30
  products: 200
  batches: 300
  orders: 500

options:
  include_edge_cases: false
  include_expired_certifications: true
  include_traceability: true
```

The generator MUST validate configuration before execution.

## 9. Reproducibility

The generator MUST support deterministic generation.

Given:

- Identical configuration.
- Identical random seed.
- Identical generator version.

The generator SHOULD produce equivalent datasets.

The random seed MUST be configurable.

Example:

```bash
python -m synthetic_data generate --profile standard --seed 42
```

Reproducibility MUST be covered by automated tests.

## 10. Output Formats

The generator MUST support:

### JSON

Used for:

- API integration testing.
- Frontend development.
- Data inspection.
- Test fixtures.

### PostgreSQL

Used for:

- Local development.
- Integration testing.
- End-to-end testing.

Additional formats such as CSV MAY be supported for analytics and data science workflows.

Database loading MUST use the application's database schema and respect constraints.

## 11. Data Validation

Every generated dataset MUST pass validation before being considered ready.

Validation MUST cover:

### Structural Integrity

- Required fields.
- Valid data types.
- Unique identifiers.
- Referential integrity.
- Valid enum values.

### Business Integrity

- Orders reference existing customers.
- Order items reference existing products.
- Farms reference valid owners.
- Certifications reference valid farms.
- Inventory quantities respect domain constraints.
- Payments reference valid orders.
- Traceability events reference valid batches.
- Event timestamps follow chronological rules.

### Data Quality

- No unintended duplicate records.
- No impossible dates.
- No invalid monetary values.
- No negative quantities unless explicitly permitted by a movement type.
- No contradictory business statuses.

## 12. Edge Case Scenarios

The generator MUST support dedicated scenarios for testing exceptional conditions.

Examples:

| Scenario | Expected Purpose |
|---|---|
| Expired certification | Prevent invalid organic claims |
| Pending certification | Test verification workflow |
| Zero inventory | Test product availability |
| Insufficient stock | Test order rejection |
| Cancelled order | Test cancellation rules |
| Failed payment | Test payment recovery |
| Duplicate payment request | Test idempotency |
| Expired product batch | Test inventory eligibility |
| Missing traceability event | Test traceability completeness |

Edge cases MUST be explicitly enabled rather than appearing randomly in normal datasets.

## 13. Safety and Isolation

The generator MUST:

- Never connect to production databases.
- Never import production customer information.
- Never use real personal information.
- Clearly identify synthetic records.
- Require explicit configuration for database writes.
- Prevent accidental destructive operations.
- Support isolated development and test environments.

Database reset operations MUST require explicit confirmation or a dedicated test-environment safeguard.

## 14. Execution Interface

The generator MUST provide a CLI.

Example commands:

```bash
# Generate minimal dataset
python -m synthetic_data generate --profile minimal

# Generate standard dataset
python -m synthetic_data generate --profile standard --seed 42

# Generate edge cases
python -m synthetic_data generate --profile edge-cases

# Export JSON
python -m synthetic_data generate --profile demo --format json

# Load into development database
python -m synthetic_data generate --profile standard --target database

# Validate generated dataset
python -m synthetic_data validate
```

The CLI MUST report:

- Selected profile.
- Random seed.
- Number of generated records.
- Generation duration.
- Validation results.
- Export or loading status.

## 15. Testing Requirements

Automated tests MUST verify:

- Generator configuration.
- Entity generation.
- Referential integrity.
- Business-rule consistency.
- Seed reproducibility.
- JSON export.
- Database loading.
- Edge case generation.
- Production environment protection.

The generator MUST NOT be considered complete until generated datasets pass all validation tests.

## 16. Definition of Done

- [ ] Generator architecture implemented.
- [ ] Dataset profiles implemented.
- [ ] Entity generators implemented.
- [ ] Dependency orchestration implemented.
- [ ] Seed-based reproducibility supported.
- [ ] JSON export supported.
- [ ] PostgreSQL loading supported.
- [ ] Referential integrity validation implemented.
- [ ] Business-rule validation implemented.
- [ ] Edge case scenarios supported.
- [ ] Automated tests implemented.
- [ ] Production safety protections implemented.
- [ ] CLI documented.

---

**End of Document**
