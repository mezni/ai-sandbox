# BioSouk Vision & System Context Document (01-VISION.md)

## 1. Executive Summary & Mission

### Product Name
BioSouk

### Mission
To provide a trusted, transparent marketplace for organic and natural products backed by end-to-end traceability, connecting conscious consumers directly with certified producers.

### Vision
To build an AI-ready modular platform that bridges traditional organic agriculture with modern digital commerce, specializing in high-trust certified goods such as:
- Honey
- Dates
- Olive Oil
- Herbal Products
- Natural Cosmetics

## 2. Core Business Goals

| Goal ID | Goal Description |
|---------|------------------|
| BG-001 | Organic Marketplace - Enable seamless buying and selling of certified organic and natural products. |
| BG-002 | End-to-End Traceability - Provide verified origin, harvest dates, expiry, and certification tracking for every batch. |
| BG-003 | Producer Empowerment - Support producers through a dedicated portal to manage products, batches, and compliance documentation. |
| BG-004 | AI-Ready Intelligence - Incorporate semantic search and recommendation capabilities natively via PostgreSQL vector extensions. |
| BG-005 | Compliance & Safety - Enforce strict certification standards and rapid recall management workflows. |

## 3. Scope Boundaries (MVP vs. Phase 2)

### ✅ Included in MVP Scope
- **Applications**: Customer App, Producer Portal, Admin Portal.
- **Core Domains**: Platform, Customer, Commerce, Order, Traceability, Operations, Engagement, Analytics.
- **Technical Stack**: 
  - PostgreSQL single-instance database with strict schema-per-domain isolation.
  - Native pgvector extension for semantic search and product similarity.
  - Transactional Outbox pattern and internal event dispatcher (Rust-based).
  - Background worker for notifications, batch/certificate expiry alerts, and report generation.
  - Pricing Models: Fixed unit, weight-based, and volume-based pricing.
  - Payments: Mock payment flow for MVP.

### ❌ Deferred to Phase 2 (Post-MVP)
- Distributed message brokers (e.g., Apache Kafka, RabbitMQ).
- Container orchestration infrastructure (e.g., Kubernetes).
- Third-party production payment gateways (Stripe, PayPal integration).
- Full microservice extraction (system remains a highly decoupled modular monolith initially).

## 4. Technical Architecture & Principles

### Modular Monolith Pattern
Built using a clean domain-driven design (DDD) structure in Rust, ensuring that business logic is strictly encapsulated within domain boundaries.

### Schema-per-Domain Database Strategy
All data resides in a single PostgreSQL database partitioned into isolated schemas (platform, customer, commerce, order, operations, engagement, traceability, analytics) to simplify transactions while preserving future microservice extraction readiness.

### Event-Driven Internal Communications
Domain events (OrderPlaced, InventoryReserved, PaymentApproved, etc.) are published and processed via an internal event bus and outbox table.

### AI-First Design
Leveraging PostgreSQL's built-in vector capabilities (`CREATE EXTENSION vector;`) to support natural language semantic search (e.g., searching for products tailored to digestion or immunity) without the operational overhead of external vector databases.