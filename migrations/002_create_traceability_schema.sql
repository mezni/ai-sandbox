-- Migration: 002_create_traceability_schema.sql
-- Per docs/05-database-design.md §8.2-8.5
-- Creates traceability schema tables: producers, farms, certifications

CREATE SCHEMA IF NOT EXISTS traceability;

-- Create TABLE traceability.producers
CREATE TABLE IF NOT EXISTS traceability.producers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL UNIQUE REFERENCES platform.users(id),
    business_name VARCHAR(200) NOT NULL,
    registration_number VARCHAR(150),
    verification_status VARCHAR(40) NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Comment: user_id references the Platform identity through an application-level contract.
-- It is not a cross-context write permission.

-- Create TABLE traceability.farms
CREATE TABLE IF NOT EXISTS traceability.farms (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    producer_id UUID NOT NULL REFERENCES traceability.producers(id),
    name VARCHAR(200) NOT NULL,
    location_description TEXT NOT NULL,
    farm_type VARCHAR(80),
    status VARCHAR(40) NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Comment: Farm belongs to exactly one producer; producer_id NOT NULL.

-- Create TABLE traceability.certifications
CREATE TABLE IF NOT EXISTS traceability.certifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    producer_id UUID REFERENCES traceability.producers(id),
    farm_id UUID REFERENCES traceability.farms(id),
    certification_type VARCHAR(100) NOT NULL,
    certifying_body VARCHAR(200) NOT NULL,
    certificate_number VARCHAR(150) NOT NULL,
    valid_from DATE NOT NULL,
    valid_to DATE NOT NULL,
    verification_status VARCHAR(40) NOT NULL DEFAULT 'pending',
    verified_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Comment: Certification scope must be explicit. If product-level certification is required,
-- add a controlled association rather than assuming every producer certificate applies to every product.

-- Constraints
ALTER TABLE traceability.certifications
    ADD CONSTRAINT ck_certification_dates CHECK (valid_to >= valid_from),
    ADD CONSTRAINT ck_certification_subject CHECK (producer_id IS NOT NULL OR farm_id IS NOT NULL);
