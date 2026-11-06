# BioSouk Domain Model Specification (02-DOMAIN-MODEL.md)

## Overview
This document defines the domain-driven design (DDD) model for BioSouk, establishing the boundaries, aggregates, entities, value objects, business invariants, and domain events across all core domains.

## 1. Platform Domain
Manages fundamental multi-tenant system identity, role-based access control, and audit logs.

### Aggregates & Entities

**User**
- Fields: user_id (UUID), email (String), password_hash (String), status (Enum: Active, Suspended, Pending), created_at (Timestamp)

**Role**
- Fields: role_id (UUID), role_name (String: Customer, Producer, Admin, Support)

**Permission**
- Fields: permission_id (UUID), permission_code (String, e.g., PRODUCT_CREATE, BATCH_VERIFY)

**Audit**
- Fields: audit_id (UUID), actor_id (UUID), action (String), entity_type (String), entity_id (UUID), timestamp (Timestamp)

## 2. Customer Domain
Handles consumer accounts, delivery addresses, loyalty rewards, and regulatory compliance.

### Aggregates & Entities

**Customer**
- Fields: customer_id (UUID), user_id (UUID), first_name (String), last_name (String), phone (String)

**Address**
- Fields: address_id (UUID), customer_id (UUID), street, city, state, country, postal_code, is_default (Boolean)

**Loyalty**
- Fields: loyalty_id (UUID), customer_id (UUID), points_balance (Integer), tier (Enum: Bronze, Silver, Gold, Platinum)

**Consent**
- Fields: consent_id (UUID), customer_id (UUID), gdpr_accepted (Boolean), marketing_opt_in (Boolean), updated_at (Timestamp)

**Preferences**
- Fields: preference_id (UUID), customer_id (UUID), favorite_categories (Array of UUIDs), dietary_tags (Array of Strings)

## 3. Commerce Domain
Powers the organic product catalog, pricing models, inventory tracking, and promotions.

### Aggregates & Entities

**Product**
- Fields: product_id (UUID), producer_id (UUID), category_id (UUID), product_name (String), pricing_model (Enum: FIXED, WEIGHT_BASED, VOLUME_BASED), shelf_life (String), status (Enum: DRAFT, PUBLISHED, ARCHIVED)
- Invariants: Product name, category, and producer are strictly required.
- Domain Events: ProductCreated, ProductPublished, ProductArchived

**Category**
- Fields: category_id (UUID), category_name (String: Honey, Dates, Olive Oil, Herbal Products, Natural Cosmetics)

**Price**
- Fields: price_id (UUID), product_id (UUID), amount (Decimal), currency (String), unit_type (String)

**Inventory**
- Fields: inventory_id (UUID), product_id (UUID), stock_quantity (Decimal), reserved_quantity (Decimal)

**Promotion**
- Fields: promotion_id (UUID), code (String), discount_percentage (Decimal), valid_from, valid_to

## 4. Order Domain
Manages shopping carts, checkout workflows, payments, fulfillment tracking, and returns.

### Aggregates & Entities

**Cart**
- Fields: cart_id (UUID), customer_id (UUID), items (List of CartItems)

**Order**
- Fields: order_id (UUID), customer_id (UUID), status (Enum: Pending, Confirmed, Shipped, Cancelled), total_amount (Decimal)
- Domain Events: OrderPlaced, OrderConfirmed, OrderCancelled

**OrderItem**
- Fields: order_item_id (UUID), order_id (UUID), product_id (UUID), batch_id (UUID), quantity (Decimal), price (Decimal)

**Payment**
- Fields: payment_id (UUID), order_id (UUID), amount (Decimal), status (Enum: Pending, Approved, Failed), gateway (String)
- Domain Events: PaymentApproved, PaymentFailed

**Shipment**
- Fields: shipment_id (UUID), order_id (UUID), tracking_number (String), carrier (String), status (String)

**Return**
- Fields: return_id (UUID), order_id (UUID), reason (String), status (Enum: Requested, Approved, Refunded)

## 5. Traceability Domain
Provides strict authenticity, farm origins, production batches, quality inspection, and recall tracking.

### Aggregates & Entities

**Producer**
- Fields: producer_id (UUID), user_id (UUID), company_name (String), verification_status (Enum: Pending, Verified, Suspended)

**Farm**
- Fields: farm_id (UUID), producer_id (UUID), farm_name (String), location (String)

**Batch**
- Fields: batch_id (UUID), product_id (UUID), origin_id (UUID), harvest_date (Date), production_date (Date), expiry_date (Date)
- Invariants: Expiry date must be strictly later than the harvest/production date.
- Domain Events: BatchCreated, BatchExpired

**Certification**
- Fields: certification_id (UUID), batch_id (UUID), certificate_number (String), certification_type (String: USDA Organic, EU Organic, Ecocert, ISO 22000), valid_from (Date), valid_until (Date), verification_url (String)
- Domain Events: CertificationAdded, CertificationExpired

**Origin**
- Fields: origin_id (UUID), farm_id (UUID), region (String), country (String), latitude (Decimal), longitude (Decimal), production_method (String)

**QualityInspection**
- Fields: inspection_id (UUID), batch_id (UUID), inspector_id (UUID), result (Enum: Pass, Fail), notes (String)

**Recall**
- Fields: recall_id (UUID), batch_id (UUID), severity (Enum: Low, Medium, High, Critical), reason (String), status (Enum: Initiated, Active, Resolved)
- Domain Events: RecallInitiated

## 6. Operations Domain
Optimizes physical logistics, cold chain compliance, warehousing, and recall execution.

### Aggregates & Entities

**Warehouse**
- Fields: warehouse_id (UUID), warehouse_name (String), location (String)

**Procurement**
- Fields: procurement_id (UUID), producer_id (UUID), status (String)

**Fulfillment**
- Fields: fulfillment_id (UUID), order_id (UUID), warehouse_id (UUID), status (String)

**ColdChain**
- Fields: log_id (UUID), shipment_id (UUID), temperature_class (Enum: Ambient, Refrigerated, Frozen), recorded_temp (Decimal), timestamp (Timestamp)

## 7. Engagement Domain
Drives user interaction, ratings, semantic search embeddings, and personalized recommendations.

### Aggregates & Entities

**Review**
- Fields: review_id (UUID), customer_id (UUID), product_id (UUID), rating (Integer 1-5), comment (String)
- Domain Events: ReviewCreated

**Campaign**
- Fields: campaign_id (UUID), title (String), discount_code (String), active (Boolean)

**Recommendation / SemanticSearch**
- Fields: embedding_id (UUID), product_id (UUID), vector_embedding (Vector 1536 dimension via pgvector), metadata (JSONB)

**CustomerInsight**
- Fields: insight_id (UUID), customer_id (UUID), metrics (JSONB)

## 8. Analytics Domain
Provides business intelligence read models for performance and reporting.

### Aggregates & Entities

**Customer360 (Read Model)**
- Fields: customer_id, lifetime_value (Decimal), total_orders (Integer), last_active_date (Timestamp)

**ProductAffinity (Read Model)**
- Fields: product_id, frequently_bought_together (Array of UUIDs)

**RevenueAnalytics (Read Model)**
- Fields: period_date (Date), gross_revenue (Decimal), net_revenue (Decimal), order_count (Integer)

**ProducerAnalytics (Read Model)**
- Fields: producer_id, active_products (Integer), total_sales (Decimal), compliance_score (Decimal)