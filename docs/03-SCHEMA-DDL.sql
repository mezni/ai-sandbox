-- ============================================================================
-- BioSouk Enterprise Schema-per-Domain DDL Migration Script
-- Database: PostgreSQL
-- Features: Schema isolation, pgvector integration, full relational mapping
-- ============================================================================

-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "vector";

-- ============================================================================
-- 1. SCHEMA DEFINITIONS
-- ============================================================================
CREATE SCHEMA IF NOT EXISTS platform;
CREATE SCHEMA IF NOT EXISTS customer;
CREATE SCHEMA IF NOT EXISTS commerce;
CREATE SCHEMA IF NOT EXISTS "order";
CREATE SCHEMA IF NOT EXISTS traceability;
CREATE SCHEMA IF NOT EXISTS operations;
CREATE SCHEMA IF NOT EXISTS engagement;
CREATE SCHEMA IF NOT EXISTS analytics;

-- ============================================================================
-- 2. PLATFORM DOMAIN
-- ============================================================================
CREATE TABLE platform.users (
    user_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'ACTIVE',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE platform.roles (
    role_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    role_name VARCHAR(100) UNIQUE NOT NULL
);

CREATE TABLE platform.permissions (
    permission_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    permission_code VARCHAR(100) UNIQUE NOT NULL
);

CREATE TABLE platform.user_roles (
    user_id UUID REFERENCES platform.users(user_id) ON DELETE CASCADE,
    role_id UUID REFERENCES platform.roles(role_id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, role_id)
);

CREATE TABLE platform.audit_logs (
    audit_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    actor_id UUID REFERENCES platform.users(user_id),
    action VARCHAR(100) NOT NULL,
    entity_type VARCHAR(100) NOT NULL,
    entity_id UUID NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- ============================================================================
-- 3. TRACEABILITY DOMAIN (Defined early for producer dependencies)
-- ============================================================================
CREATE TABLE traceability.producers (
    producer_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES platform.users(user_id) ON DELETE CASCADE,
    company_name VARCHAR(255) NOT NULL,
    verification_status VARCHAR(50) NOT NULL DEFAULT 'PENDING'
);

CREATE TABLE traceability.farms (
    farm_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    producer_id UUID REFERENCES traceability.producers(producer_id) ON DELETE CASCADE,
    farm_name VARCHAR(255) NOT NULL,
    location VARCHAR(255) NOT NULL
);

CREATE TABLE traceability.origins (
    origin_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    farm_id UUID REFERENCES traceability.farms(farm_id) ON DELETE CASCADE,
    region VARCHAR(100) NOT NULL,
    country VARCHAR(100) NOT NULL,
    latitude DECIMAL(9, 6),
    longitude DECIMAL(9, 6),
    production_method VARCHAR(100) NOT NULL
);

-- ============================================================================
-- 4. COMMERCE DOMAIN
-- ============================================================================
CREATE TABLE commerce.categories (
    category_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    category_name VARCHAR(100) UNIQUE NOT NULL
);

CREATE TABLE commerce.products (
    product_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    producer_id UUID REFERENCES traceability.producers(producer_id) ON DELETE CASCADE,
    category_id UUID REFERENCES commerce.categories(category_id),
    product_name VARCHAR(255) NOT NULL,
    pricing_model VARCHAR(50) NOT NULL,
    shelf_life VARCHAR(100),
    status VARCHAR(50) NOT NULL DEFAULT 'DRAFT'
);

CREATE TABLE commerce.prices (
    price_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id UUID REFERENCES commerce.products(product_id) ON DELETE CASCADE,
    amount DECIMAL(10, 2) NOT NULL,
    currency VARCHAR(3) DEFAULT 'USD',
    unit_type VARCHAR(50) NOT NULL
);

CREATE TABLE commerce.inventory (
    inventory_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id UUID REFERENCES commerce.products(product_id) ON DELETE CASCADE,
    stock_quantity DECIMAL(12, 3) NOT NULL DEFAULT 0.000,
    reserved_quantity DECIMAL(12, 3) NOT NULL DEFAULT 0.000
);

CREATE TABLE commerce.promotions (
    promotion_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    code VARCHAR(50) UNIQUE NOT NULL,
    discount_percentage DECIMAL(5, 2) NOT NULL,
    valid_from TIMESTAMP WITH TIME ZONE,
    valid_to TIMESTAMP WITH TIME ZONE
);

-- ============================================================================
-- 5. TRACEABILITY BATCHES & CERTIFICATIONS
-- ============================================================================
CREATE TABLE traceability.batches (
    batch_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id UUID REFERENCES commerce.products(product_id) ON DELETE CASCADE,
    origin_id UUID REFERENCES traceability.origins(origin_id),
    harvest_date DATE NOT NULL,
    production_date DATE,
    expiry_date DATE NOT NULL,
    CONSTRAINT chk_batch_expiry CHECK (expiry_date > harvest_date)
);

CREATE TABLE traceability.certifications (
    certification_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    batch_id UUID REFERENCES traceability.batches(batch_id) ON DELETE CASCADE,
    certificate_number VARCHAR(100) UNIQUE NOT NULL,
    certification_type VARCHAR(100) NOT NULL,
    valid_from DATE NOT NULL,
    valid_until DATE NOT NULL,
    verification_url VARCHAR(500)
);

CREATE TABLE traceability.quality_inspections (
    inspection_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    batch_id UUID REFERENCES traceability.batches(batch_id) ON DELETE CASCADE,
    inspector_id UUID REFERENCES platform.users(user_id),
    result VARCHAR(50) NOT NULL,
    notes TEXT
);

CREATE TABLE traceability.recalls (
    recall_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    batch_id UUID REFERENCES traceability.batches(batch_id) ON DELETE CASCADE,
    severity VARCHAR(50) NOT NULL,
    reason TEXT NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'INITIATED'
);

-- ============================================================================
-- 6. CUSTOMER DOMAIN
-- ============================================================================
CREATE TABLE customer.customers (
    customer_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES platform.users(user_id) ON DELETE CASCADE,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    phone VARCHAR(50)
);

CREATE TABLE customer.addresses (
    address_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID REFERENCES customer.customers(customer_id) ON DELETE CASCADE,
    street VARCHAR(255) NOT NULL,
    city VARCHAR(100) NOT NULL,
    state VARCHAR(100),
    country VARCHAR(100) NOT NULL,
    postal_code VARCHAR(20) NOT NULL,
    is_default BOOLEAN DEFAULT FALSE
);

CREATE TABLE customer.loyalty (
    loyalty_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID REFERENCES customer.customers(customer_id) ON DELETE CASCADE,
    points_balance INT DEFAULT 0,
    tier VARCHAR(50) DEFAULT 'BRONZE'
);

CREATE TABLE customer.consents (
    consent_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID REFERENCES customer.customers(customer_id) ON DELETE CASCADE,
    gdpr_accepted BOOLEAN NOT NULL DEFAULT FALSE,
    marketing_opt_in BOOLEAN NOT NULL DEFAULT FALSE,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE customer.preferences (
    preference_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID REFERENCES customer.customers(customer_id) ON DELETE CASCADE,
    favorite_categories UUID[],
    dietary_tags VARCHAR[]
);

-- ============================================================================
-- 7. ORDER DOMAIN
-- ============================================================================
CREATE TABLE "order".carts (
    cart_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID REFERENCES customer.customers(customer_id) ON DELETE CASCADE
);

CREATE TABLE "order".cart_items (
    cart_item_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    cart_id UUID REFERENCES "order".carts(cart_id) ON DELETE CASCADE,
    product_id UUID REFERENCES commerce.products(product_id),
    quantity DECIMAL(12, 3) NOT NULL DEFAULT 1.000
);

CREATE TABLE "order".orders (
    order_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID REFERENCES customer.customers(customer_id),
    status VARCHAR(50) NOT NULL DEFAULT 'PENDING',
    total_amount DECIMAL(10, 2) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE "order".order_items (
    order_item_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID REFERENCES "order".orders(order_id) ON DELETE CASCADE,
    product_id UUID REFERENCES commerce.products(product_id),
    batch_id UUID REFERENCES traceability.batches(batch_id),
    quantity DECIMAL(12, 3) NOT NULL,
    price DECIMAL(10, 2) NOT NULL
);

CREATE TABLE "order".payments (
    payment_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID REFERENCES "order".orders(order_id) ON DELETE CASCADE,
    amount DECIMAL(10, 2) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'PENDING',
    gateway VARCHAR(50) DEFAULT 'MOCK'
);

CREATE TABLE "order".shipments (
    shipment_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID REFERENCES "order".orders(order_id) ON DELETE CASCADE,
    tracking_number VARCHAR(100) UNIQUE,
    carrier VARCHAR(100),
    status VARCHAR(50) NOT NULL DEFAULT 'PROCESSING'
);

CREATE TABLE "order".returns (
    return_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID REFERENCES "order".orders(order_id) ON DELETE CASCADE,
    reason TEXT NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'REQUESTED'
);

-- ============================================================================
-- 8. OPERATIONS DOMAIN
-- ============================================================================
CREATE TABLE operations.warehouses (
    warehouse_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    warehouse_name VARCHAR(255) NOT NULL,
    location VARCHAR(255) NOT NULL
);

CREATE TABLE operations.procurement (
    procurement_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    producer_id UUID REFERENCES traceability.producers(producer_id),
    status VARCHAR(50) NOT NULL DEFAULT 'ORDERED'
);

CREATE TABLE operations.fulfillments (
    fulfillment_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID REFERENCES "order".orders(order_id),
    warehouse_id UUID REFERENCES operations.warehouses(warehouse_id),
    status VARCHAR(50) NOT NULL DEFAULT 'PACKING'
);

CREATE TABLE operations.cold_chains (
    log_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    shipment_id UUID REFERENCES "order".shipments(shipment_id) ON DELETE CASCADE,
    temperature_class VARCHAR(50) NOT NULL,
    recorded_temp DECIMAL(5, 2) NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- ============================================================================
-- 9. ENGAGEMENT DOMAIN (AI-Ready with pgvector)
-- ============================================================================
CREATE TABLE engagement.reviews (
    review_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID REFERENCES customer.customers(customer_id),
    product_id UUID REFERENCES commerce.products(product_id) ON DELETE CASCADE,
    rating INT CHECK (rating BETWEEN 1 AND 5),
    comment TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE engagement.campaigns (
    campaign_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(255) NOT NULL,
    discount_code VARCHAR(50) UNIQUE,
    active BOOLEAN DEFAULT TRUE
);

-- Native vector embeddings for Semantic Product Search via pgvector
CREATE TABLE engagement.embeddings (
    embedding_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id UUID REFERENCES commerce.products(product_id) ON DELETE CASCADE,
    vector_embedding VECTOR(1536),
    metadata JSONB
);

CREATE TABLE engagement.customer_insights (
    insight_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID REFERENCES customer.customers(customer_id) ON DELETE CASCADE,
    metrics JSONB
);

-- ============================================================================
-- 10. ANALYTICS DOMAIN (Read Models)
-- ============================================================================
CREATE TABLE analytics.customer_360 (
    customer_id UUID PRIMARY KEY,
    lifetime_value DECIMAL(12, 2) DEFAULT 0.00,
    total_orders INT DEFAULT 0,
    last_active_date TIMESTAMP WITH TIME ZONE
);

CREATE TABLE analytics.product_affinity (
    product_id UUID PRIMARY KEY,
    frequently_bought_together UUID[]
);

CREATE TABLE analytics.revenue_analytics (
    period_date DATE PRIMARY KEY,
    gross_revenue DECIMAL(12, 2) DEFAULT 0.00,
    net_revenue DECIMAL(12, 2) DEFAULT 0.00,
    order_count INT DEFAULT 0
);

CREATE TABLE analytics.producer_analytics (
    producer_id UUID PRIMARY KEY,
    active_products INT DEFAULT 0,
    total_sales DECIMAL(12, 2) DEFAULT 0.00,
    compliance_score DECIMAL(5, 2) DEFAULT 100.00
);