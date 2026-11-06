-- Migration: 001_create_platform_schema.sql
-- Per docs/05-database-design.md §4.2-4.6
-- Creates platform schema tables: users, roles, permissions, user_roles, role_permissions

-- Create the platform schema (if not using schema-qualified tables, set search_path)
-- DO $$ BEGIN
--     CREATE SCHEMA IF NOT EXISTS platform;
-- EXCEPTION
--     WHEN duplicate_schema THEN null;
-- END $$;

-- Create TABLE platform.users
CREATE TABLE IF NOT EXISTS platform.users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(320) NOT NULL,
    password_hash TEXT NOT NULL,
    status VARCHAR(30) NOT NULL DEFAULT 'pending',
    email_verified_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT uq_users_email UNIQUE (email),
    CONSTRAINT ck_users_status CHECK (status IN ('pending', 'active', 'suspended', 'closed'))
);

-- Comment: Email normalization and case-insensitive uniqueness must be enforced consistently.
-- A suitable implementation is a normalized email column or a unique index on lower(email).

-- Create TABLE platform.roles
CREATE TABLE IF NOT EXISTS platform.roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(50) NOT NULL UNIQUE,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create TABLE platform.permissions
CREATE TABLE IF NOT EXISTS platform.permissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(100) NOT NULL UNIQUE,
    description TEXT
);

-- Create TABLE platform.user_roles (junction table M:N users-roles)
CREATE TABLE IF NOT EXISTS platform.user_roles (
    user_id UUID NOT NULL REFERENCES platform.users(id) ON DELETE CASCADE,
    role_id UUID NOT NULL REFERENCES platform.roles(id) ON DELETE CASCADE,
    assigned_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    assigned_by UUID,

    PRIMARY KEY (user_id, role_id)
);

-- Create TABLE platform.role_permissions (junction table M:N roles-permissions)
CREATE TABLE IF NOT EXISTS platform.role_permissions (
    role_id UUID NOT NULL REFERENCES platform.roles(id) ON DELETE CASCADE,
    permission_id UUID NOT NULL REFERENCES platform.permissions(id) ON DELETE CASCADE,

    PRIMARY KEY (role_id, permission_id)
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_platform_users_email_lower ON platform.users (lower(email));
CREATE INDEX IF NOT EXISTS idx_platform_users_status ON platform.users (status);
CREATE INDEX IF NOT EXISTS idx_platform_roles_code ON platform.roles (code);
CREATE INDEX IF NOT EXISTS idx_platform_permissions_code ON platform.permissions (code);
CREATE INDEX IF NOT EXISTS idx_platform_userroles_user_id ON platform.user_roles (user_id);
CREATE INDEX IF NOT EXISTS idx_platform_userroles_role_id ON platform.user_roles (role_id);
CREATE INDEX IF NOT EXISTS idx_platform_rolepermissions_role_id ON platform.role_permissions (role_id);
CREATE INDEX IF NOT EXISTS idx_platform_rolepermissions_permission_id ON platform.role_permissions (permission_id);