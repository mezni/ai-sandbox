# PRD: BioSouk Organic Marketplace

**Created:** 2026-01-02
**Updated:** 2026-11-06
**Status:** draft

---
title: BioSouk Organic Marketplace
---

# 0. Document Purpose

This PRD defines the requirements for BioSouk, a trusted, transparent marketplace for organic and natural products backed by end-to-end traceability. This document is intended for the Engineering, Product, and Compliance teams. It is structured with features grouped under globally numbered FRs (Functional Requirements), user journeys referenced by ID, and assumptions tagged inline. The document builds on the vision and domain model defined in `01-VISION.md`, `02-DOMAIN-MODEL.md`, `03-SCHEMA-DDL.sql`, and `04-API-SPEC.yaml`.

# 1. Vision

To build an AI-ready modular platform that bridges traditional organic agriculture with modern digital commerce, specializing in high-trust certified goods such as honey, dates, olive oil, herbal products, and natural cosmetics. The platform must provide verified origin, harvest dates, expiry, and certification tracking for every batch, while enabling semantic search and recommendation capabilities natively via PostgreSQL vector extensions. The system should empower conscious consumers to make informed decisions and support producers through a dedicated portal to manage products, batches, and compliance documentation.

# 2. Target User

## 2.1 Jobs To Be Done

- **As a conscious consumer**, I want to find and purchase certified organic products with full visibility into origin and certification, so I can trust the authenticity of what I buy.
- **As a producer**, I want to list my products with detailed traceability information, manage batches and certifications, and reach customers who value organic certification, so I can grow my business transparently.
- **As an admin**, I want to enforce certification standards, manage recalls, and ensure compliance with organic regulations, so the marketplace maintains its trust reputation.

## 2.2 Non-Users (v1)

- Third-party production payment gateways (Stripe, PayPal) — deferred to Phase 2
- Distributed message brokers (Kafka, RabbitMQ) — deferred to Phase 2
- Full microservice extraction — system remains a highly decoupled modular monolith in v1

## 2.3 Key User Journeys

- **UJ-1. Producer Onboards Product:** Producer logs in, creates a product with pricing model (FIXED/WEIGHT_BASED/VOLUME_BASED), adds batch with origin and harvest date, uploads certification. Realizes FR-1, FR-5, FR-12.
- **UJ-2. Customer Browses & Searches:** Customer visits the marketplace, filters by category (Honey, Dates, Olive Oil, Herbal Products, Natural Cosmetics), performs semantic search using natural language (e.g., "products for digestion"), and views product details with traceability info. Realizes FR-22, FR-23.
- **UJ-3. Customer Places Order:** Customer adds a product (with batch selection) to cart, checks out, completes mock payment, and receives order confirmation with shipment tracking. Realizes FR-27, FR-28, FR-30.
- **UJ-4. Admin Initiates Recall:** Admin identifies a batch with quality issue, initiates recall with severity classification, notifies affected customers, and tracks resolution. Realizes BG-005, FR-35.

# 3. Glossary

| Term | Definition | Relationships | Cardinality |
|------|-----------|---------------|-----------|
| **Product** | An organic product listed on the marketplace, identified by a UUID, linked to a producer and category. | belongsTo Producer, belongsTo Category, has many Batches | 1 ↔ many |
| **Batch** | A production batch of a Product, identified by a UUID, with harvest/expiry dates, origin, and certifications. | belongsTo Product, has one Origin, has many Certifications, triggers many Recalls | 1 ↔ many |
| **Producer** | An entity that produces and supplies organic products, identified by a UUID, linked to a User account. | has many Products, verified via verification_status | 1 ↔ many |
| **Customer** | A consumer account on the marketplace, identified by a UUID, linked to a User account. | has many Orders, one Loyalty tier | 1 ↔ many |
| **Order** | A customer purchase, identified by a UUID, containing OrderItems with specific Batches. | belongsTo Customer, has many OrderItems, one Payment, one Shipment | 1 ↔ many |
| **Certification** | Organic certificate attached to a Batch, identified by a UUID, with certificate number, type, validity dates, and verification URL. | belongsTo Batch | 1 ↔ 1 |
| **Semantic Search** | Natural language search powered by pgvector embeddings, returning product similarity matches. | stored in Embededdings table, queried via /search/semantic endpoint | n/a |
| **Pricing Model** | The way a product is priced: FIXED (unit price), WEIGHT_BASED (per kg/liter), or VOLUME_BASED (per volume). | applied to Product, used in OrderItem price calculation | 1 of 3 |

# 4. Features

## 4.1 Product Catalog Management

**Description:** Producers can create, publish, and manage organic products in the marketplace catalog. Each product must have a name, category, producer, pricing model, and status. Products go through DRAFT → PUBLISHED → ARCHIVED lifecycle.

**Functional Requirements:**

- **FR-1: Create Product** — [Producer] can create a new product with required fields (product_name, category_id, pricing_model) and optional fields (shelf_life). The system validates that product_name, category, and producer are strictly required. Realizes UJ-1.

  **Consequences (testable):**
  - System rejects product creation if any required field is missing
  - Pricing model must be one of: FIXED, WEIGHT_BASED, VOLUME_BASED
  - Product status defaults to DRAFT upon creation

- **FR-2: Update Product** — [Producer] can update product fields (shelf_life, status) for a published product. Status can be changed to ARCHIVED to retire the product from the catalog.

  **Consequences (testable):**
  - Only PUBLISHED products can be archived; DRAFT products can be deleted
  - Price and inventory changes require product re-publication

- **FR-3: Category Management** — [Admin] can create and manage product categories (Honey, Dates, Olive Oil, Herbal Products, Natural Cosmetics). Categories are pre-seeded and read-only for producers.

  **Out of Scope:** Creating new categories beyond the 5 MVP categories — deferred to Phase 2.

- **FR-4: Product Search** — [Customer] can browse products by category and search by product name.

  **Consequences (testable):**
  - API endpoint GET /products with optional category_id query parameter
  - Pagination supported (page, limit parameters)

## 4.2 Batch & Traceability Management

**Description:** Producers and admins can create production batches linked to products and farm origins, upload certifications, and track quality inspections. Full traceability from farm → origin → batch → certification must be visible to customers.

**Functional Requirements:**

- **FR-5: Create Batch** — [Producer] can create a new production batch linked to a product and optional origin. Required fields: product_id, harvest_date, expiry_date. Invariant: expiry_date must be strictly later than harvest_date. Optional: production_date, origin_id. Realizes UJ-1.

  **Consequences (testable):**
  - Database check constraint: `chk_batch_expiry` ensures `expiry_date > harvest_date`
  - System rejects batch creation if expiry_date ≤ harvest_date
  - Origin linking is optional; if omitted, batch traces to product only

- **FR-6: Upload Certification** — [Producer] can attach an organic certificate to a batch. Required fields: batch_id, certificate_number, certification_type (USDA Organic, EU Organic, Ecocert, ISO 22000), valid_from, valid_until, verification_url. Realizes BG-002, BG-005.

  **Consequences (testable):**
  - Certificate number must be unique across all batches
  - System validates certification_type against allowed list
  - Certification validity: valid_from ≤ current_date ≤ valid_until

- **FR-7: Quality Inspection** — [Admin] can record quality inspection results for a batch. Required fields: batch_id, inspector_id, result (PASS/FAIL), notes. Realizes BG-005.

  **Consequences (testable):**
  - Inspection result links to batch; failed inspections trigger recall workflow

- **FR-8: Recall Management** — [Admin] can initiate a recall for a batch with severity classification (Low, Medium, High, Critical). Required fields: batch_id, severity, reason. Status flows: Initiated → Active → Resolved. Realizes BG-005.

  **Consequences (testable):**
  - Recall status defaults to INITIATED
  - System tracks all recall events with batch_id, severity, reason, timestamp
  - Affected orders/customers are notified based on shipment history

## 4.3 Order & Commerce Workflow

**Description:** Customers can browse products, add to cart, checkout, and complete orders. The system supports mock payment flow for MVP and tracks order fulfillment through shipping and cold chain compliance.

**Functional Requirements:**

- **FR-9: Product Listing** — [Customer] can view product catalog with category filtering, pagination, and product details including price, shelf life, and traceability summary (origin, certification status).

  **Consequences (testable):**
  - API GET /products returns paginated results
  - Product detail page shows full traceability chain (origin → farm → batch → certification)

- **FR-10: Shopping Cart** — [Customer] can add products (with specific batch selection) to cart, adjust quantities, and remove items. Cart is tied to customer_id and persists across sessions via JWT.

  **Consequences (testable):**
  - Cart can contain multiple products with different batches
  - System validates batch availability against inventory before checkout

- **FR-11: Create Order** — [Customer] can submit an order from cart. Required fields: customer_id, items (each with product_id, batch_id, quantity). System calculates total_amount from product prices. Payment status defaults to PENDING with gateway='MOCK'. Realizes UJ-3.

  **Consequences (testable):**
  - Order status defaults to PENDING
  - Total amount calculated as sum of (quantity × unit price) for all items
  - Mock payment gateway processes transaction; order moves to CONFIRMED on approval

- **FR-12: Order Fulfillment** — [Operations] can assign order to warehouse, track shipment, and log cold chain temperature data. Required fields: order_id, warehouse_id. Cold chain logs: temperature_class (Ambient/Refrigerated/Frozen), recorded_temp, timestamp.

  **Consequences (testable):**
  - Fulfillment status flows: PACKING → SHIPPED → DELIVERED
  - Cold chain logs are immutable once created
  - System alerts if temperature exceeds class-specific thresholds

## 4.4 AI Semantic Search

**Description:** Customers and staff can perform natural language semantic search against product embeddings stored in PostgreSQL pgvector. The system supports searching for products tailored to specific needs (e.g., "products for digestion," "immunity-boosting honey").

**Functional Requirements:**

- **FR-13: Semantic Search Endpoint** — [Customer/Staff] can POST to /search/semantic with a natural language query and optional limit. The system queries the pgvector embeddings table and returns matching products with similarity scores.

  **Consequences (testable):**
  - Request body: {query: string, limit: integer}
  - Response: array of ProductResponse objects ranked by vector similarity
  - Embedding dimension: 1536 (compatible with OpenAI/text-embedding models)

- **FR-14: Product Embedding Generation** — [System] automatically generates and stores vector embeddings for all PUBLISHED products at product creation/update. Embedding model: sentence-transformers or OpenAI text-embedding-ada-002.

  **Consequences (testable):**
  - Each product has one embedding record in engagement.embeddings
  - Embedding is generated when product status changes to PUBLISHED
  - Metadata JSONB stores: category, pricing_model, key_tags

## 4.5 Engagement & Analytics

**Description:** Customers can leave reviews, the system tracks loyalty points, and analytics read models provide business insights for customers, producers, and admins.

**Functional Requirements:**

- **FR-15: Product Reviews** — [Customer] can submit a rating (1-5) and comment for a purchased product. Reviews are linked to customer_id and product_id. Domain event: ReviewCreated.

  **Consequences (testable):**
  - Rating must be integer between 1 and 5
  - Each review has created_at timestamp
  - Product average rating displayed on product detail page

- **FR-16: Loyalty Program** — [System] awards points to customers per order (1 point per $1 spent). Tier progression: BRONZE (0-1000 points), SILVER (1001-5000 points), GOLD (5001-15000 points), PLATINUM (15001+ points). Realizes BG-003.

  **Consequences (testable):**
  - Loyalty tier auto-updates when points_balance crosses thresholds
  - Points earned at order completion, not at cart creation

- **FR-17: Customer 360 Read Model** — [Admin/Staff] can query analytics customer_360 view with lifetime_value, total_orders, last_active_date per customer.

  **Consequences (testable):**
  - View is updated via background worker after each order completion
  - last_active_date updates on any customer activity (login, order, search)

- **FR-18: Producer Analytics** — [Admin/Staff] can query analytics producer_analytics with active_products, total_sales, compliance_score per producer. Compliance score starts at 100.00 and decreases for expired certifications or failed inspections.

  **Consequences (testable):**
  - Compliance score recalculated daily based on certification validity
  - Score floor: 0.00, ceiling: 100.00

- **FR-19: Revenue Analytics** — [Admin/Staff] can query analytics revenue_analytics with gross_revenue, net_revenue, order_count per period_date.

  **Consequences (testable):**
  - Period is calendar day (UTC)
  - Gross revenue = sum of all order total_amounts
  - Net revenue = gross revenue minus mock payment processing fees (5%)

# 5. Non-Goals (Explicit)

- ❌ Third-party payment gateway integration (Stripe, PayPal) — deferred to Phase 2
- ❌ Distributed message brokers (Kafka, RabbitMQ) — deferred to Phase 2
- ❌ Full microservice extraction — system remains modular monolith in v1
- ❌ Container orchestration (Kubernetes) — deferred to Phase 2
- ❌ Creating new product categories beyond the 5 MVP categories — deferred to Phase 2
- ❌ Full UX design with custom UI/visual branding — minimal UI for MVP
- ❌ GDPR/CCPA compliance framework beyond basic consent flags — basic opt-in/opt-out only
- ❌ Multi-tenant isolation at database level beyond schema-per-domain — single PostgreSQL instance

# 6. MVP Scope

## 6.1 In Scope

- **Applications:** Customer App, Producer Portal, Admin Portal
- **Core Domains:** Platform, Customer, Commerce, Order, Traceability, Operations, Engagement, Analytics
- **Technical Stack:** PostgreSQL single-instance with schema-per-domain isolation, pgvector extension, Rust-based modular monolith, Mock payment flow, Transactional outbox pattern, Background workers for notifications and expiry alerts
- **Pricing Models:** FIXED, WEIGHT_BASED, VOLUME_BASED
- **Certification Types:** USDA Organic, EU Organic, Ecocert, ISO 22000 (first-year focus)
- **Core Features:** Product catalog with 3 pricing models, batch traceability with certifications, order workflow with mock payments, semantic search via pgvector, customer reviews, loyalty program (4 tiers)

## 6.2 Out of Scope for MVP

- Third-party payment gateways (Stripe, PayPal)
- Kafka/RabbitMQ distributed messaging
- Kubernetes container orchestration
- Full microservice extraction
- Additional product categories (beyond 5 MVP)
- Advanced compliance frameworks (beyond basic certification tracking)
- Custom UI/visual branding beyond functional interfaces
- Multi-tenant isolation at database level beyond schemas

# 7. Success Metrics

## Primary

- **SM-1: Product Discovery Conversion** — Percentage of search queries (including semantic) that result in a product view or order. Target: ≥ 15% by MVP launch. Validates FR-13, FR-2, FR-9, FR-10.
- **SM-2: Traceability Visibility** — Percentage of products with complete traceability chain (origin → farm → batch → certification visible). Target: 100% of listed products. Validates FR-5, FR-6, BG-002.
- **SM-3: Order Completion Rate** — Percentage of carts that result in completed orders with mock payment. Target: ≥ 60% by MVP launch. Validates FR-11, FR-12.
- **SM-4: Producer Onboarding Time** — Average time for a producer to create product, add batch, and upload certification from signup. Target: ≤ 15 minutes. Validates FR-1, FR-5, FR-6.

## Secondary

- **SM-5: Semantic Search Relevance** — Precision @ 5 for sample queries ("digestion," "immunity," "organic honey"). Target: ≥ 70% by MVP launch. Validates FR-13, domain model embedding quality.
- **SM-6: Customer Retention (30-day)** — Percentage of first-time customers who return to place a second order within 30 days. Target: ≥ 25%. Validates UX, loyalty program (FR-16).
- **SM-7: Certification Compliance Rate** — Percentage of active batches with valid, non-expired certifications. Target: ≥ 95%. Validates FR-6, BG-005.

## Counter-metrics (do not optimize)

- **SM-C1: System Latency Under Load** — p99 API response time under concurrent load. Monitor but do not optimize at expense of functionality. Validates all FRs.
- **SM-C2: Mock Payment Deference Count** — Number of times customers attempt checkout with intention to integrate real payment gateway. Track but do not optimize; this is expected in MVP.

# 8. Open Questions

1. What embedding model should be used for pgvector (OpenAI vs. sentence-transformers) and what are the cost implications?
2. Should the loyalty points earning rate be 1 point per $1 or a different ratio (e.g., 2 points per $1)?
3. What is the acceptable temperature threshold deviation for cold chain alerts (e.g., ±2°C for refrigerated, ±5°C for frozen)?
4. How should the system handle batch expiry alerts — proactive notification to producer 30 days before expiry, or on-demand only?
5. What is the minimum viable set of certification types for MVP vs. what can be added in Phase 2?

# 9. Assumptions Index

- [ASSUMPTION 1] pgvector extension version 0.5.0+ is available and compatible with PostgreSQL 15+.
- [ASSUMPTION 2] Mock payment gateway is sufficient for MVP; real gateway integration requires Phase 2.
- [ASSUMPTION 3] Producers will voluntarily provide complete origin and certification information; verification is producer-initiated.
- [ASSUMPTION 4] Customer semantic search queries will follow predictable patterns initially (digestion, immunity, organic, by category).
- [ASSUMPTION 5] Single PostgreSQL instance can handle MVP load; scaling to multi-instance considered in Phase 2.
- [ASSUMPTION 6] Rust compilation and deployment pipeline is established before Sprint 1.
- [ASSUMPTION 7] Admin team will manage recall workflows; no automated recommendation engine for recalls in v1.
- [ASSUMPTION 8] Loyalty tier thresholds (BRONZE/SILVER/GOLD/PLATINUM) are fixed for MVP and can be adjusted in v2.
- [ASSUMPTION 9] The 5 core categories (Honey, Dates, Olive Oil, Herbal Products, Natural Cosmetics) are sufficient for MVP; additional categories follow market demand.
- [ASSUMPTION 10] Transactional outbox pattern will prevent event loss during database commits; Rust-based dispatcher is production-ready.

---
*Note: This PRD builds on the Vision (01-VISION.md), Domain Model (02-DOMAIN-MODEL.md), Schema DDL (03-SCHEMA-DDL.sql), and API Spec (04-API-SPEC.yaml) documents in the ./docs/ directory. All FRs, UJs, and SMs reference the domain invariants and data models defined in those documents.*