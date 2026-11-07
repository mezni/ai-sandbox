-- Migration: 004_platform_auth.sql
-- Identity and Access Management (Milestone 3)
-- Per docs/05-database-design.md §4.7-4.8 and specs/003-identity-access/data-model.md

CREATE SCHEMA IF NOT EXISTS platform;

-- Refresh sessions: only the SHA-256 hash of the opaque token is stored.
CREATE TABLE IF NOT EXISTS platform.refresh_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES platform.users(id) ON DELETE CASCADE,
    token_hash TEXT NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    revoked_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT uq_refresh_tokens_token_hash UNIQUE (token_hash)
);

CREATE INDEX IF NOT EXISTS idx_platform_refresh_tokens_user_id
    ON platform.refresh_tokens (user_id);
CREATE INDEX IF NOT EXISTS idx_platform_refresh_tokens_expires_at
    ON platform.refresh_tokens (expires_at);

-- Append-only audit trail. Never stores credentials or token material.
CREATE TABLE IF NOT EXISTS platform.audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    actor_id UUID,
    action VARCHAR(100) NOT NULL,
    resource_type VARCHAR(100) NOT NULL,
    resource_id UUID,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    correlation_id UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_platform_audit_logs_actor_id
    ON platform.audit_logs (actor_id);
CREATE INDEX IF NOT EXISTS idx_platform_audit_logs_action
    ON platform.audit_logs (action);
CREATE INDEX IF NOT EXISTS idx_platform_audit_logs_created_at
    ON platform.audit_logs (created_at);

-- Canonical roles (constitution §239-244).
INSERT INTO platform.roles (code, name, description) VALUES
    ('admin', 'Administrator', 'Platform administration and full access'),
    ('producer', 'Producer', 'Manages farms, batches, and certifications'),
    ('customer', 'Customer', 'Purchases products and manages own orders'),
    ('support', 'Support', 'Customer support and read-only audit access')
ON CONFLICT (code) DO NOTHING;

-- Baseline permissions.
INSERT INTO platform.permissions (code, description) VALUES
    ('users.read', 'Read user profiles'),
    ('users.update', 'Update user profiles'),
    ('roles.assign', 'Assign roles to users'),
    ('audit.read', 'Read audit records')
ON CONFLICT (code) DO NOTHING;

-- Role -> permission mapping (idempotent).
INSERT INTO platform.role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM platform.roles r
JOIN platform.permissions p ON
    (r.code = 'admin')
    OR (r.code = 'producer' AND p.code IN ('users.read', 'users.update'))
    OR (r.code = 'customer' AND p.code IN ('users.read', 'users.update'))
    OR (r.code = 'support' AND p.code IN ('users.read', 'audit.read'))
ON CONFLICT (role_id, permission_id) DO NOTHING;
