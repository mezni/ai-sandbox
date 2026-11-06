# Data Model: Project Foundation

**Generated**: 2026-11-06
**Feature**: Project Foundation Specs (specs/001-project-foundation-specs)
**Source**: Feature specification analysis

## Core Entities

### Repository
- **Purpose**: Project root repository containing all crates and configuration
- **Attributes**:
  - Root path (filesystem location)
  - Cargo workspace members
  - Configuration files (environment variables, defaults)
- **Relationships**: 
  - Contains Configuration entity
  - Contains Docker Environment entity

### Configuration
- **Purpose**: Environment variable and settings management for development
- **Attributes**:
  - Environment variables (RUST_LOG, DATABASE_URL, etc.)
  - Development vs production settings
  - Docker Compose service configurations
- **Relationships**:
  - Belongs to Repository
  - Supplies settings to Docker Environment

### Docker Environment
- **Purpose**: Development container and service definitions
- **Attributes**:
  - Docker Compose configuration
  - Service definitions (backend, frontend, database)
  - Volume mounts and network configurations
- **Relationships**:
  - Belongs to Repository
  - Depends on Configuration for environment variables

## Foundation-Level Notes

- This data model represents the **infrastructure/setup entities** for the Project Foundation milestone
- Domain entities (User, Product, Farm, Certification, Order, etc.) are defined in later milestones
- Relationships above capture the dependency structure established in milestone 1
- All entities are defined at the **project infrastructure level**, not the application domain level
- The full domain data model will be generated in subsequent milestones when actual feature domains are implemented

## Assumptions
- Repository structure follows Rust workspace conventions (from constitution: Modular Monolith)
- Configuration follows environment variable best practices (from constitution: no secrets in Git)
- Docker Compose development setup follows project conventions (from implementation plan)