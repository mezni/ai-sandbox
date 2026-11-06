# Data Model: Sprint 2 Build Plan — BioSouk

**Based on**: Feature specification `specs/002-sprint-2-build-plan/spec.md`  
**Constitution reference**: `.specify/memory/constitution.md` §52-59, §99-178, §1256-1278  
**Generated**: 2026-11-06 (Phase 1 of `/speckit.plan`)

---

## Core Entities

### Platform Domain

#### User
- **Fields**: `id` (UUID, PRIMARY KEY, DEFAULT gen_random_uuid()), `email` (VARCHAR(320), NOT NULL, UNIQUE — normalized/ case-insensitive), `password_hash` (TEXT, NOT NULL), `status` (VARCHAR(30), NOT NULL — enum: `pending`, `active`, `suspended`, `closed`), `email_verified_at` (TIMESTAMPTZ), `created_at` (TIMESTAMPTZ, NOT NULL, DEFAULT NOW()), `updated_at` (TIMESTAMPTZ, NOT NULL, DEFAULT NOW())
- **Constraints**: `uq_users_email` UNIQUE (email); `ck_users_status` CHECK (status IN ('pending', 'active', 'suspended', 'closed'))
- **Relationships**: 
  - `M:N` with Role through `user_roles` (junction: user_id, role_id, assigned_at, assigned_by)
  - One-to-one with Producer (traceability.producers.user_id is UNIQUE, references platform.users)

#### Role
- **Fields**: `id` (UUID, PRIMARY KEY, DEFAULT gen_random_uuid()), `code` (VARCHAR(50), NOT NULL UNIQUE), `name` (VARCHAR(100), NOT NULL), `description` (TEXT), `created_at` (TIMESTAMPTZ, NOT NULL, DEFAULT NOW())
- **Constraints**: Unique code; referenced by users via user_roles junction
- **Relationships**:
  - `M:N` with Permission through `role_permissions` (junction: role_id, permission_id, primary key: role_id, permission_id)
  - `1:N` with User through user_roles (role_id references platform.roles(id))

#### Permission
- **Fields**: `id` (UUID, PRIMARY KEY, DEFAULT gen_random_uuid()), `code` (VARCHAR(100), NOT NULL UNIQUE), `description` (TEXT)
- **Constraints**: Unique code; junction with Role is primary key
- **Relationships**:
  - `M:N` with Role through role_permissions (junction)
  - No direct user assignment; enforced through role hierarchy

#### UserRole (junction)
- **Fields**: `user_id` (UUID, NOT NULL, REFERENCES platform.users(id)), `role_id` (UUID, NOT NULL, REFERENCES platform.roles(id)), `assigned_at` (TIMESTAMPTZ, NOT NULL, DEFAULT NOW()), `assigned_by` (UUID, references platform.users(id))
- **Primary Key**: (user_id, role_id)
- **Constraints**: Composite primary key ensures one role assignment per user; foreign keys enforce referential integrity

#### RolePermission (junction)
- **Fields**: `role_id` (UUID, NOT NULL, REFERENCES platform.roles(id)), `permission_id` (UUID, NOT NULL, REFERENCES platform.permissions(id))
- **Primary Key**: (role_id, permission_id)
- **Constraints**: Composite primary key enforces role-permission assignment; foreign keys reference roles and permissions tables

### Commerce Domain

#### Product
- **Fields**: `id` (UUID, PRIMARY KEY, DEFAULT gen_random_uuid()), `producer_id` (UUID, NOT NULL, REFERENCES traceability.producers(id)), `category_id` (UUID, NOT NULL, REFERENCES commerce.categories(id)), `brand_id` (UUID, REFERENCES commerce.brands(id)), `name` (VARCHAR(200), NOT NULL), `slug` (VARCHAR(220), NOT NULL UNIQUE), `description` (TEXT, NOT NULL), `product_type` (VARCHAR(80), NOT NULL), `pricing_model` (VARCHAR(40), NOT NULL), `unit_of_measure` (VARCHAR(30), NOT NULL), `traceability_required` (BOOLEAN, NOT NULL, DEFAULT TRUE), `status` (VARCHAR(30), NOT NULL, DEFAULT 'draft'), `created_at` (TIMESTAMPTZ, NOT NULL, DEFAULT NOW()), `updated_at` (TIMESTAMPTZ, NOT NULL, DEFAULT NOW())
- **Constraints**: `uq_products_slug` UNIQUE (slug); status enum: 'draft', 'active', 'archived'
- **Relationships**:
  - `MANY-TO-ONE` to Category (category_id → commerce.categories.id); product belongs to category; catalog filtering by category
  - `MANY-TO-ONE` to Brand (brand_id → commerce.brands.id); optional brand association; product may have no brand
  - `MANY-TO-ONE` to Producer (producer_id → traceability.producers.id); producer reference from Traceability context; commerce must not own or update producer record

#### Category
- **Fields**: `id` (UUID, PRIMARY KEY, DEFAULT gen_random_uuid()), `parent_category_id` (UUID, REFERENCES commerce.categories(id)), `name` (VARCHAR(150), NOT NULL), `slug` (VARCHAR(180), NOT NULL UNIQUE), `description` (TEXT), `status` (VARCHAR(30), NOT NULL, DEFAULT 'active'), `created_at` (TIMESTAMPTZ, NOT NULL, DEFAULT NOW()), `updated_at` (TIMESTAMPTZ, NOT NULL, DEFAULT NOW())
- **Constraints**: `uq_categories_slug` UNIQUE (slug); self-referencing foreign key for hierarchical categories; status enum: 'active', 'inactive'
- **Relationships**:
  - `SELF-REFERENCING`: parent_category_id references commerce.categories(id)
  - `ONE-TO-MANY`: inverse of parent_category_id

#### Brand
- **Fields**: `id` (UUID, PRIMARY KEY, DEFAULT gen_random_uuid()), `name` (VARCHAR(150), NOT NULL), `description` (TEXT), `status` (VARCHAR(30), NOT NULL, DEFAULT 'active'), `created_at` (TIMESTAMPTZ, NOT NULL, DEFAULT NOW())
- **Constraints**: Unique name; status enum: 'active'
- **Relationships**:
  - `MANY-TO-ONE` inverse: `commerce.products.brand_id` → brands.id

#### InventoryItem
- **Fields**: `id` (UUID, PRIMARY KEY, DEFAULT gen_random_uuid()), `product_id` (UUID, NOT NULL, REFERENCES commerce.products(id)), `batch_id` (UUID, NOT NULL, REFERENCES traceability.batches(id)), `location_id` (UUID, NOT NULL, REFERENCES operations.warehouses(id)), `quantity_on_hand` (NUMERIC(19,4), NOT NULL, DEFAULT 0), `quantity_reserved` (NUMERIC(19,4), NOT NULL, DEFAULT 0), `unit_of_measure` (VARCHAR(30), NOT NULL), `updated_at` (TIMESTAMPTZ, NOT NULL, DEFAULT NOW())
- **Constraints**: `uq_inventory_product_batch_location` UNIQUE (product_id, batch_id, location_id); `ck_inventory_on_hand` CHECK (quantity_on_hand >= 0); `ck_inventory_reserved` CHECK (quantity_reserved >= 0 AND quantity_reserved <= quantity_on_hand)
- **Relationships**:
  - `MANY-TO-ONE` to Product (product_id → commerce.products.id)
  - `MANY-TO-ONE` to Batch (batch_id → traceability.batches.id); external domain reference, not permission to write to Traceability context
  - `MANY-TO-ONE` to Warehouse (location_id → operations.warehouses.id); external domain reference, not permission to write to Operations context

#### InventoryReservation
- **Fields**: `id` (UUID, PRIMARY KEY, DEFAULT gen_random_uuid()), `inventory_item_id` (UUID, NOT NULL, REFERENCES commerce.inventory_items(id)), `order_id` (UUID, NOT NULL, REFERENCES "order".orders(id)), `quantity` (NUMERIC(19,4), NOT NULL), `status` (VARCHAR(30), NOT NULL), `expires_at` (TIMESTAMPTZ), `idempotency_key` (VARCHAR(150), NOT NULL UNIQUE), `created_at` (TIMESTAMPTZ, NOT NULL, DEFAULT NOW()), `updated_at` (TIMESTAMPTZ, NOT NULL, DEFAULT NOW())
- **Constraints**: `ck_reservation_quantity` CHECK (quantity > 0); idempotency_key UNIQUE across all reservations
- **Relationships**:
  - `MANY-TO-ONE` to InventoryItem (inventory_item_id → commerce.inventory_items.id)
  - `MANY-TO-ONE` to Order (order_id → "order".orders.id)

### Traceability Domain

#### Producer
- **Fields**: `id` (UUID, PRIMARY KEY, DEFAULT gen_random_uuid()), `user_id` (UUID, NOT NULL UNIQUE, REFERENCES platform.users(id)), `business_name` (VARCHAR(200), NOT NULL), `registration_number` (VARCHAR(150)), `description` (TEXT), `verification_status` (VARCHAR(40), NOT NULL), `created_at` (TIMESTAMPTZ, NOT NULL, DEFAULT NOW()), `updated_at` (TIMESTAMPTZ, NOT NULL, DEFAULT NOW())
- **Constraints**: `uq_producers_user_id` UNIQUE (user_id); user_id references platform identity through application-level contract; verification_status enum per domain rules
- **Relationships**:
  - `ONE-TO-ONE` with User (user_id → platform.users.id); application-level contract
  - `ONE-TO-MANY` with Farm (producer_id → traceability.farms.producer_id)
  - `ONE-TO-MANY` with Certification (producer_id → traceability.certifications.producer_id); optional — either producer_id OR farm_id must be set per ck_certification_subject
  - `ONE-TO-MANY` with Batch (producer_id → traceability.batches.producer_id)

#### Farm
- **Fields**: `id` (UUID, PRIMARY KEY, DEFAULT gen_random_uuid()), `producer_id` (UUID, NOT NULL, REFERENCES traceability.producers(id)), `name` (VARCHAR(200), NOT NULL), `location_description` (TEXT, NOT NULL), `farm_type` (VARCHAR(80)), `status` (VARCHAR(40), NOT NULL), `created_at` (TIMESTAMPTZ, NOT NULL, DEFAULT NOW()), `updated_at` (TIMESTAMPTZ, NOT NULL, DEFAULT NOW())
- **Constraints**: Farm belongs to exactly one producer; status enum per domain rules
- **Relationships**:
  - `MANY-TO-ONE` to Producer (producer_id → traceability.producers.id)
  - `MANY-TO-ONE` with Certification (farm_id → traceability.certifications.farm_id); optional — same scope control as producer reference
  - `MANY-TO-ONE` with Batch (farm_id → traceability.batches.farm_id)

#### Certification
- **Fields**: `id` (UUID, PRIMARY KEY, DEFAULT gen_random_uuid()), `producer_id` (UUID, REFERENCES traceability.producers(id)), `farm_id` (UUID, REFERENCES traceability.farms(id)), `certification_type` (VARCHAR(100), NOT NULL), `certifying_body` (VARCHAR(200), NOT NULL), `certificate_number` (VARCHAR(150), NOT NULL), `valid_from` (DATE, NOT NULL), `valid_to` (DATE, NOT NULL), `verification_status` (VARCHAR(40), NOT NULL), `verified_at` (TIMESTAMPTZ), `created_at` (TIMESTAMPTZ, NOT NULL, DEFAULT NOW()), `updated_at` (TIMESTAMPTZ, NOT NULL, DEFAULT NOW())
- **Constraints**: `ck_certification_dates` CHECK (valid_to >= valid_from); `ck_certification_subject` CHECK (producer_id IS NOT NULL OR farm_id IS NOT NULL — at least one must be set); certification scope must be explicit
- **Relationships**:
  - `MANY-TO-ONE` to Producer (producer_id → traceability.producers.id); optional — controlled association rather than assuming every producer certificate applies to every product
  - `MANY-TO-ONE` to Farm (farm_id → traceability.farms.id); optional — same scope control

#### Batch
- **Fields**: `id` (UUID, PRIMARY KEY, DEFAULT gen_random_uuid()), `batch_number` (VARCHAR(150), NOT NULL), `product_id` (UUID, NOT NULL, REFERENCES commerce.products(id)), `producer_id` (UUID, NOT NULL, REFERENCES traceability.producers(id)), `farm_id` (UUID, NOT NULL, REFERENCES traceability.farms(id)), `origin_id` (UUID, NOT NULL, REFERENCES traceability.origins(id)), `harvest_date` (DATE), `production_date` (DATE), `expiry_date` (DATE), `quantity_produced` (NUMERIC(19,4), NOT NULL), `unit_of_measure` (VARCHAR(30), NOT NULL), `status` (VARCHAR(40), NOT NULL), `created_at` (TIMESTAMPTZ, NOT NULL, DEFAULT NOW()), `updated_at` (TIMESTAMPTZ, NOT NULL, DEFAULT NOW())
- **Constraints**: `uq_batch_producer_number` UNIQUE (producer_id, batch_number); `ck_batch_quantity` CHECK (quantity_produced > 0); `ck_batch_expiry` CHECK (expiry_date IS NULL OR production_date IS NULL OR expiry_date > production_date)
- **Relationships**:
  - `MANY-TO-ONE` to Product (product_id → commerce.products.id); validate that product belongs to producer and farm belongs to producer in application transaction
  - `MANY-TO-ONE` to Producer (producer_id → traceability.producers.id)
  - `MANY-TO-ONE` to Farm (farm_id → traceability.farms.id)
  - `MANY-TO-ONE` to Origin (origin_id → traceability.origins.id)

#### Origin
- **Fields**: `id` (UUID, PRIMARY KEY, DEFAULT gen_random_uuid()), `country_code` (CHAR(2), NOT NULL), `region` (VARCHAR(150)), `locality` (VARCHAR(150)), `geographic_description` (TEXT), `latitude` (NUMERIC(10,7)), `longitude` (NUMERIC(10,7)), `created_at` (TIMESTAMPTZ, NOT NULL, DEFAULT NOW())
- **Constraints**: `ck_origin_latitude` CHECK (latitude IS NULL OR latitude BETWEEN -90 AND 90); `ck_origin_longitude` CHECK (longitude IS NULL OR longitude BETWEEN -180 AND 180); coordinates optional; privacy controls before public exposure
- **Relationships**:
  - `MANY-TO-ONE` inverse: `traceability.batches.origin_id` → origins.id

---

## Entity Relationship Summary

| Entity | Platform | Commerce | Traceability |
|--------|----------|----------|--------------|
| User | ✅ (core) | — | — (via producer_user_id UNIQUE) |
| Role | ✅ (core) | — | — |
| Permission | ✅ (core) | — | — |
| Product | — | ✅ (core) | — (producer_id reference) |
| Category | — | ✅ (core, hierarchical) | — |
| Brand | — | ✅ (optional) | — |
| InventoryItem | — | ✅ (core) | batch_id, location_id (external refs) |
| InventoryReservation | — | ✅ (core) | order_id (order schema ref) |
| Producer | ✅ (via user_id UNIQUE) | — | ✅ (core) |
| Farm | — | — | ✅ (belongs to producer) |
| Certification | — | — | ✅ (producer_id OR farm_id, one required) |
| Batch | — (producer_id, product_id refs) | product_id (commerce ref) | ✅ (core) |
| Origin | — | — | ✅ (geo coordinates, optional) |

---

## Validation Rules (from Feature Requirements)

### Domain Invariants
- **FR-005**: System MUST create PostgreSQL schemas (platform, commerce, traceability) and SQLx migrations that execute successfully against a local database instance
- **FR-006**: System MUST create core schema tables: `platform.users`, `platform.roles`, `commerce.categories`, `commerce.products`, `traceability.producers`, `traceability.farms`
- **FR-009**: System MUST track inventory with quantity on hand and reserved quantity, ensuring reserved never exceeds on-hand (inventory_item ck_inventory_reserved constraint)
- **FR-011**: System MUST identify expired certifications as invalid and prevent their use in domain validations (certification valid_to <= today check)
- **FR-012**: System MUST create batches referencing valid products, farms, and origins through application-level validation (database design §936)

### Cross-Context Ownership (from constitution §52-59, §1256-1278)
- **customer.customers.user_id** → Platform (not cross-context write permission)
- **traceability.producers.user_id** → Platform (application-level contract)
- **commerce.products.producer_id** → Traceability (external domain reference)
- **traceability.batches.product_id** → Commerce (validate product belongs to producer in application transaction)
- **commerce.inventory_items.batch_id** → Traceability (external domain reference)
- **commerce.inventory_items.location_id** → Operations (external domain reference)
- **order.orders.customer_id** → Customer (not cross-context write permission)
- **order.order_items.product_id** → Commerce (historical reference, preserved at purchase time)
- **order.order_items.producer_id** → Traceability (historical reference)
- **engagement.reviews.product_id** → Commerce (cross-context identifier)
- **engagement.reviews.customer_id** → Customer (cross-context identifier)

### Key Business Rules Encoded in Schema
- **Unique email** across platform users (not case-sensitive normalization recommended)
- **Product slug** uniqueness within commerce context
- **Inventory uniqueness** per product+batch+location combination
- **Reservation quantity** never exceeds on-hand quantity (conditional UPDATE check)
- **Certification scope**: at least producer_id OR farm_id must be set (never both null)
- **Batch expiry logic**: expiry_date must be after production_date if both are set; otherwise null permitted
- **Provider batch number uniqueness**: (producer_id, batch_number) must be unique
- **Farm ownership**: each farm belongs to exactly one producer (producer_id NOT NULL on farms)
- **Product traceability**: traceability_required BOOLEAN defaults TRUE; product publication requires application-level validation of producer status, required certifications, and traceability conditions (database design §489-491)

---

## State Transitions

### User Status Lifecycle
`pending` → `active` (upon email verification) → `suspended` (admin action) → `closed` (account deletion)

### Product Status Lifecycle
`draft` → `active` (published to catalog) → `archived` (removed from sales)

### Certification Status Scenarios
- `valid`: valid_from <= today <= valid_to
- `expired`: today > valid_to
- `pending`: future-effective (valid_from > today)
- `rejected`: administratively rejected (application workflow)

### Batch Status Lifecycle
- `produced`: successfully created, available for inventory
- `reserved`: allocated to orders, quantity reserved
- `shipped`: delivered to customer
- `delivered`: customer receipt confirmed
- `expired`: past expiry_date, no longer sellable

### Order Status Lifecycle
- `pending`: order placed, not yet validated
- `confirmed`: inventory reserved, payment pending
- `processing`: payment completed, preparing shipment
- `shipped`: carrier has order
- `delivered`: customer received order
- `cancelled`: customer or system cancelled; inventory released
- `returned`: customer initiated return; refund may be processed

---

## Entity-Attribute Matrix

| Entity | id | created_at | updated_at | status/flags | Key foreign keys |
|--------|----|------------|------------|--------------|-----------------|
| User | ✅ | ✅ NOW() | ✅ NOW() | active/suspended/closed | — |
| Role | ✅ | ✅ NOW() | ✅ NOW() | active | — |
| Permission | ✅ | ✅ NOW() | ✅ NOW() | — | — |
| UserRole | ✅ | ✅ NOW() | ✅ NOW() | — | user_id, role_id |
| RolePermission | ✅ | ✅ NOW() | ✅ NOW() | — | role_id, permission_id |
| Product | ✅ | ✅ NOW() | ✅ NOW() | draft/active/archived | producer_id, category_id, brand_id |
| Category | ✅ | ✅ NOW() | ✅ NOW() | active/inactive | parent_category_id (self) |
| Brand | ✅ | ✅ NOW() | ✅ NOW() | active | — |
| InventoryItem | ✅ | ✅ NOW() | ✅ NOW() | — | product_id, batch_id, location_id |
| InventoryReservation | ✅ | ✅ NOW() | ✅ NOW() | — | inventory_item_id, order_id |
| Producer | ✅ | ✅ NOW() | ✅ NOW() | active/inactive/suspended | user_id (UNIQUE), verification_status |
| Farm | ✅ | ✅ NOW() | ✅ NOW() | active/inactive | producer_id (NOT NULL) |
| Certification | ✅ | ✅ NOW() | ✅ NOW() | valid/expired/pending/rejected | producer_id, farm_id (one required), valid_from/valid_to |
| Batch | ✅ | ✅ NOW() | ✅ NOW() | produced/reserved/shipped/delivered/expired | product_id, producer_id, farm_id, origin_id |
| Origin | ✅ | ✅ NOW() | ✅ NOW() | — | country_code, latitude/longitude (optional) |
| Order | ✅ | ✅ NOW() | ✅ NOW() | pending/confirmed/processing/shipped/delivered/cancelled/returned | customer_id |
| OrderItem | ✅ | ✅ NOW() | ✅ NOW() | — | order_id, product_id, producer_id |
| Payment | ✅ | ✅ NOW() | ✅ NOW() | pending/processing/paid/refunded | order_id, provider, provider_reference |
| PaymentEvent | ✅ | ✅ NOW() | ✅ NOW() | — | payment_id |
| Return | ✅ | ✅ NOW() | ✅ NOW() | — | order_id |
| Refund | ✅ | ✅ NOW() | ✅ NOW() | — | order_id, payment_id |
| Review | ✅ | ✅ NOW() | ✅ NOW() | — | customer_id, product_id, order_item_id |
| Campaign | ✅ | ✅ NOW() | ✅ NOW() | active/finished | starts_at, ends_at |
| Recommendation | ✅ | ✅ NOW() | ✅ NOW() | — | customer_id, product_id, score |
| Interaction | ✅ | ✅ NOW() | ✅ NOW() | — | customer_id, product_id (both optional) |
| SearchRequest | ✅ | ✅ NOW() | — | — | customer_id (optional), query, filters |