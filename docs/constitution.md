# BioSouk Constitution

## Architecture & Engineering Principles

**Document:** `constitution.md`  
**Project:** BioSouk  
**Version:** 1.1  
**Status:** Active  
**Supersedes:** v1.0

---

# Purpose

This constitution defines the non-negotiable rules governing the design, implementation, testing, security, and evolution of BioSouk.

Every architecture decision, code change, database schema, API, frontend component, integration, and AI-generated artifact must comply with this constitution.

When specifications, designs, or code conflict with this document, the Constitution takes precedence.

All engineering documentation must reference and respect this Constitution.

---

# Article I: Product Mission

BioSouk is a specialized marketplace for:

- Organic food products.
- Honey.
- Dates.
- Olive oil.
- Herbal products.
- Natural cosmetics.
- Sustainable agriculture products.

BioSouk's competitive advantage is built on:

- Customer trust.
- Product traceability.
- Producer transparency.
- Organic certification compliance.
- Sustainable commerce.
- Future AI-enabled customer experience.

BioSouk is not a generic eCommerce platform.

Every product and architectural decision must reinforce these differentiators.

---

# Article II: Architecture Principles

## Principle 1: Business First

Business capabilities drive architecture.

Correct:

- Product
- Producer
- Farm
- Batch
- Certification
- Inventory
- Order
- Shipment

Incorrect:

- Controller
- DTO
- Database Table

The domain model is the source of truth.

## Principle 2: Bounded Contexts

All functionality must belong to an explicitly defined bounded context.

Approved domains:

1. Platform Domain
2. Customer Domain
3. Commerce Domain
4. Order Domain
5. Traceability Domain
6. Operations Domain
7. Engagement Domain
8. Analytics Domain

No feature may be implemented without assigning domain ownership.

## Principle 3: Explicit Ownership

Each domain owns:

- Business rules.
- Aggregates.
- Domain entities and value objects.
- Persistence schema.
- Domain events.
- Application services.

A domain must not modify another domain's aggregates directly.

## Principle 4: Clean Architecture

BioSouk follows Clean Architecture.

Dependencies must point inward:

- Domain layer contains business rules.
- Application layer orchestrates use cases.
- Infrastructure layer implements technical adapters.
- Presentation layer exposes APIs and user interfaces.

The domain layer must not depend on Axum, SQLx, PostgreSQL, React, or external service implementations.

## Principle 5: Modular Monolith

BioSouk is implemented as a Modular Monolith.

Current architecture:

- Single repository.
- Single backend application.
- Single PostgreSQL database.
- Separate domain schemas.
- Independently structured frontend applications.

Modules communicate through:

- Public application interfaces.
- Domain events.
- Explicit integration contracts.

Direct access to another domain's private implementation or database repositories is forbidden.

Future microservices must remain possible without requiring a fundamental domain redesign.

---

# Article III: Domain Ownership

## Platform Domain

Owns:

- User.
- Role.
- Permission.
- Authentication.
- Authorization.
- Audit.

Responsible for:

- JWT.
- RBAC.
- Password security.
- Access control.
- Session management.

## Customer Domain

Owns:

- Customer.
- Address.
- Preferences.
- Consent.
- Loyalty.

## Commerce Domain

Owns:

- Product.
- Category.
- Brand.
- Price.
- Promotion.
- Inventory.

Products support:

- Unit pricing.
- Weight pricing.
- Volume pricing.

## Order Domain

BioSouk's Core Domain.

Owns:

- Cart.
- Order.
- OrderItem.
- Payment.
- Shipment.
- Return.

Responsible for:

- Checkout.
- Purchasing commitment.
- Order lifecycle.
- Payment orchestration.
- Refund orchestration.

## Traceability Domain

Mandatory strategic domain.

Owns:

- Producer.
- Farm.
- Origin.
- Batch.
- Lot.
- Certification.
- Recall.
- QualityInspection.

Responsible for product provenance and traceability integrity.

## Operations Domain

Owns:

- Warehouse.
- Supplier.
- Procurement.
- Fulfillment.
- ColdChain.

Supports:

- Perishable products.
- Temperature constraints.
- Batch tracking.
- Inventory allocation.

## Engagement Domain

Owns:

- Review.
- Campaign.
- Recommendation.
- Interaction.
- SemanticSearch.

## Analytics Domain

Owns:

- Customer360.
- RevenueAnalytics.
- ProductAffinity.
- ProducerAnalytics.

Analytics is read-only with respect to transactional domains.

It must not modify business aggregates.

---

# Article IV: Traceability Requirements

Traceability is mandatory.

Every eligible product must be traceable from purchase to origin.

## Required Traceability Chain

Product → Batch → Origin → Producer → Certification

## Product Requirements

Every traceable product must contain:

- ProducerId.
- Traceability classification.
- Applicable batch associations.

## Batch Requirements

Every batch must contain:

- HarvestDate, where applicable.
- ExpiryDate, where applicable.
- OriginId.
- ProducerId or a verifiable producer relationship.
- Batch identifier.
- Current status.

## Certification Requirements

Every certification must contain:

- CertificationType.
- CertifyingBody.
- ValidFrom.
- ValidTo.
- Certificate identifier.
- Verification status.

## Recall Requirements

Recall must support:

- Batch recall.
- Producer recall.
- Product recall.

## Traceability Integrity

- Provenance history must be auditable.
- Historical traceability records must not be silently overwritten.
- Certification validity must be checked before claiming a product is currently certified.
- Expired, suspended, and revoked certifications must be distinguishable.
- Recalls must identify affected batches and associated orders.
- Traceability information presented to customers must be verifiable.

---

# Article V: Security Principles

Security is mandatory across all applications and domains.

## Authentication

Approved technologies:

- JWT.
- Argon2id password hashing.
- Refresh tokens.
- Secure token rotation and revocation.

## Authorization

Authorization model:

RBAC.

Roles:

- Admin.
- Producer.
- Customer.
- Support.

Authorization must be enforced on the backend.

Frontend permissions are not security boundaries.

## Security Requirements

- Input validation.
- Secure password storage.
- Secrets management.
- API rate limiting.
- Audit logging.
- Secure file uploads.
- Dependency vulnerability scanning.
- Protection against common web application vulnerabilities.
- Secure refresh-token handling.
- Appropriate data access restrictions.

## Forbidden

- Plaintext passwords.
- Hardcoded credentials.
- Hardcoded tokens.
- Production secrets in source control.
- Trusting frontend authorization.
- Unauthorized cross-domain data access.

---

# Article VI: Data Ownership

BioSouk uses a single PostgreSQL database with separate domain schemas.

Approved schemas:

- platform
- customer
- commerce
- order
- traceability
- operations
- engagement
- analytics

## Rules

1. Every domain owns its schema.
2. Direct writes across domain schemas are forbidden.
3. Cross-domain interaction occurs through application services or events.
4. Database migrations must preserve domain ownership.
5. Referential integrity must be enforced where appropriate.
6. Database transactions must respect aggregate boundaries.
7. Critical business invariants must not depend exclusively on frontend validation.
8. Database constraints must reinforce domain rules.

Cross-domain foreign keys require explicit justification and must not create unauthorized write dependencies.

---

# Article VII: Event-Driven Design

BioSouk uses domain events for important business activities.

An internal event bus is required.

Required components:

- Domain Event.
- Event Dispatcher.
- Event Handlers.
- Outbox Pattern.

## Example Events

- CustomerCreated.
- ProductCreated.
- BatchCreated.
- CertificationUploaded.
- CertificationRevoked.
- OrderCreated.
- OrderPaid.
- ShipmentCreated.
- RecallInitiated.
- InventoryReserved.

## Event Rules

- Events represent completed business facts.
- Events are published after successful business transactions.
- Events must have stable identifiers.
- Events must support schema versioning.
- Events must include correlation identifiers.
- Consumers must be idempotent.
- Event handlers must not assume exactly-once delivery.

---

# Article VIII: Outbox Pattern

All integration events must use the Outbox Pattern.

Business data and corresponding outbox records must be persisted inside the same database transaction.

## Minimum Outbox Fields

- event_id.
- event_type.
- event_version.
- aggregate_id.
- payload.
- created_at.
- processed_at.
- processing_status.
- retry_count.

## Processing Requirements

- At-least-once delivery.
- Idempotent processing.
- Retry with backoff.
- Failed-event monitoring.
- Error recording.
- Correlation tracking.

The system must prevent duplicate business effects when an event is delivered more than once.

---

# Article IX: Frontend Architecture

BioSouk contains three frontend applications.

## Customer Application

Purpose: Shopping experience.

Features:

- Catalog.
- Product details.
- Traceability information.
- Cart.
- Checkout.
- Orders.
- Reviews.
- Customer profile.

## Producer Portal

Purpose: Producer self-service.

Features:

- Producer profile.
- Farm management.
- Product management.
- Batch management.
- Certifications.
- Inventory.
- Orders.
- Payout visibility.

## Admin Portal

Purpose: Business operations.

Features:

- Orders.
- Customers.
- Producers.
- Product approvals.
- Certifications.
- Inventory.
- Recalls.
- Reports.
- Platform administration.

Frontend applications must consume backend contracts rather than access domain persistence directly.

---

# Article X: AI Readiness

BioSouk is AI-Ready, not AI-First.

## Current MVP

Must support:

- Domain events.
- Structured business data.
- Read models.
- Analytics views.
- Traceability data.
- Auditable business operations.

## Future AI Capabilities

- Recommendation Engine.
- Semantic Search.
- Product Advisor.
- Demand Forecasting.
- Customer Insights.
- Producer Insights.

## AI Safety

AI must never directly modify domain aggregates.

Required interaction:

AI Suggestion → Validation → Human Approval where required → Authorized Command → Domain Execution.

AI-generated recommendations must not bypass:

- Authorization.
- Business rules.
- Certification checks.
- Inventory constraints.
- Payment controls.
- Audit requirements.

---

# Article XI: Testing Requirements

Every feature requires automated tests.

## Domain Tests

Validate:

- Business rules.
- Aggregate invariants.
- State transitions.
- Domain events.
- Invalid operations.

## Integration Tests

Validate:

- API behavior.
- Database interactions.
- Authentication.
- Authorization.
- External integrations.
- Transaction boundaries.

## Additional Testing

- Application service tests.
- Architecture boundary tests.
- Contract tests.
- Frontend tests.
- End-to-end tests.
- Performance tests.
- Security tests.

Code without appropriate automated tests is incomplete.

---

# Article XII: Documentation Requirements

The official BioSouk documentation is maintained under `docs/`.

Every new feature must update relevant specifications.

Required documentation coverage:

- Product Specification.
- Domain Model.
- System Architecture.
- Domain Workflows.
- Database Design.
- API Specification.
- Security Architecture.
- Frontend Architecture.
- Event-Driven Architecture.
- Integration Architecture.
- Synthetic Data Generation.
- Testing Strategy.
- Observability and Operations.
- Deployment and Infrastructure.
- Implementation Roadmap.

Important architectural decisions require an Architecture Decision Record.

Documentation must remain consistent with the implementation.

---

# Article XIII: Architecture Decision Records

Important architecture decisions must be documented using ADRs.

Examples:

- Database technology.
- Authentication strategy.
- Messaging strategy.
- New bounded context.
- External integration.
- Architecture pattern.
- Infrastructure technology.
- Synthetic data generation technology.

Every ADR must document:

- Context.
- Decision.
- Alternatives.
- Consequences.
- Trade-offs.
- Status.

No major architectural change may occur without an ADR.

---

# Article XIV: Synthetic Data Generation

Synthetic data generation is a mandatory engineering capability of BioSouk.

It supports:

- Development.
- Automated testing.
- Demonstrations.
- Performance testing.
- Analytics validation.
- Future AI/ML experimentation.

## Principle 1: Domain-Aware Generation

Generated data must respect BioSouk's domain model, relationships, business rules, and aggregate invariants.

## Principle 2: Data Integrity

Generated records must maintain:

- Referential integrity.
- Valid domain relationships.
- Valid business state transitions.
- Valid dates and monetary values.
- Product, batch, producer, and certification consistency.

## Principle 3: Deterministic Generation

The generator must support configurable random seeds.

The same generator version, configuration, and seed must produce reproducible datasets, subject to documented versioning.

## Principle 4: Privacy

- Never use real customer personal information.
- Never copy production customer records.
- Generated identities and contact details must be fictional.
- Synthetic datasets must be clearly identifiable.

## Principle 5: Scenario-Based Generation

The generator must support:

- Organic product catalogs.
- Producer and farm registration.
- Product batches and harvests.
- Certification lifecycles.
- Inventory and procurement.
- Customer registration.
- Shopping carts and orders.
- Payments and shipments.
- Returns and refunds.
- Product recalls.
- Reviews and interactions.

## Principle 6: Environment Isolation

Synthetic data generation is permitted in:

- Local development.
- Automated testing.
- Staging.
- Demonstration environments.
- Performance testing environments.

Production execution is forbidden by default and requires explicit authorization and safeguards.

## Principle 7: Domain Integrity

The generator must use domain application interfaces or approved seeding interfaces.

It must not bypass domain invariants through arbitrary cross-schema database writes.

## Principle 8: Validation

Every generated dataset must be validated for:

- Referential integrity.
- Business invariants.
- Domain consistency.
- Duplicate constraints.
- Traceability completeness.
- Certification validity.
- Inventory consistency.

## Principle 9: Generation Profiles

The generator must support:

- Minimal: smoke testing.
- Standard: development.
- Complete: demonstrations.
- Large: performance testing.
- Scenario: targeted business workflows.

## Principle 10: Auditability

Every generation execution must record:

- Dataset identifier.
- Generator version.
- Configuration.
- Random seed.
- Execution timestamp.
- Generated record counts.
- Validation results.

Synthetic data is a testing and engineering asset, not a source of production business truth.

---

# Article XV: Simplicity and Technology Principles

BioSouk prefers the simplest architecture that satisfies current requirements.

## Approved Current Stack

### Backend

- Rust.
- Axum.
- Tokio.
- SQLx.
- PostgreSQL.
- Serde.

### Frontend

- React.
- TypeScript.

### Infrastructure

- Docker.
- Docker Compose.

### Security

- JWT.
- Argon2id.
- RBAC.

### Testing

- Rust unit and integration tests.
- Frontend automated tests.
- API and end-to-end tests.

### Synthetic Data

- Rust CLI.
- Clap.
- Rand.
- Fake.
- Serde.
- SQLx.

## Avoid Premature Complexity

Do not introduce the following without a justified ADR:

- Kafka.
- Kubernetes.
- Service Mesh.
- CQRS.
- Event Sourcing.
- Distributed microservices.
- Unnecessary infrastructure dependencies.

Technology choices must serve business requirements rather than dictate them.

---

# Article XVI: Success Criteria

BioSouk succeeds when:

- Customers can trust product origins.
- Producers can manage products, batches, and certifications.
- Product traceability is verifiable.
- Orders can be fulfilled reliably.
- Payments and refunds are auditable.
- Inventory remains consistent.
- Architecture boundaries remain clean.
- Synthetic datasets support realistic development and testing.
- Features can evolve without major redesign.
- AI capabilities can be introduced incrementally and safely.

---

# Final Principle

> Protect the domain.
>
> Protect traceability.
>
> Protect customer trust.
>
> Technology changes.
>
> Frameworks change.
>
> Infrastructure changes.
>
> The domain model, business language, traceability guarantees, data integrity, and customer trust are BioSouk's most valuable assets.

Every engineering decision must protect these principles.
