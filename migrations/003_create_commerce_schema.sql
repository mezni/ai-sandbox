-- Migration: 003_create_commerce_schema.sql
-- Per docs/05-database-design.md §6.2-6.4
-- Creates commerce schema tables: categories, brands, products

CREATE SCHEMA IF NOT EXISTS commerce;

-- Create TABLE commerce.categories
CREATE TABLE IF NOT EXISTS commerce.categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    parent_category_id UUID REFERENCES commerce.categories(id),
    name VARCHAR(150) NOT NULL,
    slug VARCHAR(180) NOT NULL UNIQUE,
    description TEXT,
    status VARCHAR(30) NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Comment: Self-referencing foreign key for hierarchical categories.
-- parent_category_id references commerce.categories(id) enabling product category hierarchies.

-- Create UNIQUE index on slug with case-insensitive consideration
CREATE UNIQUE INDEX IF NOT EXISTS uq_categories_slug ON commerce.categories (lower(slug));

-- Create TABLE commerce.brands
CREATE TABLE IF NOT EXISTS commerce.brands (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(150) NOT NULL,
    description TEXT,
    status VARCHAR(30) NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Comment: Unique name per business context; status enum: 'active'

-- Create TABLE commerce.products
CREATE TABLE IF NOT EXISTS commerce.products (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    producer_id UUID NOT NULL REFERENCES traceability.producers(id),
    category_id UUID NOT NULL REFERENCES commerce.categories(id),
    brand_id UUID REFERENCES commerce.brands(id),
    name VARCHAR(200) NOT NULL,
    slug VARCHAR(220) NOT NULL UNIQUE,
    description TEXT NOT NULL,
    product_type VARCHAR(80) NOT NULL,
    pricing_model VARCHAR(40) NOT NULL,
    unit_of_measure VARCHAR(30) NOT NULL,
    traceability_required BOOLEAN NOT NULL DEFAULT TRUE,
    status VARCHAR(30) NOT NULL DEFAULT 'draft',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Comment: producer_id is a reference to the Traceability context.
-- Commerce must not own or update the producer record.
-- Product publication requires application-level validation of producer status,
-- required certifications, and traceability conditions.

-- Create UNIQUE index on slug with case-insensitive consideration
CREATE UNIQUE INDEX IF NOT EXISTS uq_products_slug ON commerce.products (lower(slug));

-- Create indexes for catalog filtering per database design §1366-1368
CREATE INDEX IF NOT EXISTS idx_products_status_category_id ON commerce.products (status, category_id);
CREATE INDEX IF NOT EXISTS idx_products_producer_id_status ON commerce.products (producer_id, status);
-- NOTE: commerce.product_prices is introduced in a later milestone; its index
-- is intentionally deferred until the table exists.
