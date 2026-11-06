# BioSouk Agile Build Plan

## Mission & Objectives
To deliver a trusted, transparent marketplace for organic and natural products with end-to-end traceability, AI-ready semantic search, and modular monolith architecture—starting with MVP in 4 sprints and extending to Phase 2.

---

## Release Strategy

### **Release 1.0 (MVP) — 4 Sprints**
**Goal:** Deliver core marketplace functionality enabling producers to list products, customers to purchase, and full traceability from farm to consumer.

| Sprint | Duration | Focus | Key Deliverables |
|--------|----------|-------|------------------|
| **Sprint 1** | 2 weeks | Foundations & Auth | - Project bootstrap (Rust project structure)<br>- Database schema initialization (platform, customer, commerce schemas)<br>- Authentication & JWT issuance (`/auth/login`, `/auth/register`)<br>- RBAC: Role assignment (Customer, Producer, Admin)<br>- Basic audit logging framework|
| **Sprint 2** | 2 weeks | Product Catalog & Pricing | - Product CRUD with 3 pricing models (FIXED, WEIGHT_BASED, VOLUME_BASED)<br>- Category management (Honey, Dates, Olive Oil, Herbal Products, Natural Cosmetics)<br>- Product validation invariants (name, category, producer required)<br>- Domain events: `ProductCreated`, `ProductPublished`|
| **Sprint 3** | 2 weeks | Traceability & Orders | - Batch creation with origin linking<br>- Certification upload & verification URL<br>- Full order workflow: Cart → Order → Payment (mock) → Shipment<br>- Order domain events: `OrderPlaced`, `OrderConfirmed`, `OrderCancelled`<br>- Cold chain logging framework|
| **Sprint 4** | 2 weeks | Engagement & AI Search | - Product reviews (rating 1-5)<br>- Semantic search endpoint (`/search/semantic`) with pgvector<br>- Vector embeddings storage and similarity search<br>- Customer insights read model<br>- Basic analytics read models|

**MVP Acceptance Criteria:**
- ✅ Producer can create product with any of 3 pricing models
- ✅ Customer can register, login, browse products, and place orders
- ✅ Batch traceability: origin → farm → batch → certification visible
- ✅ Mock payment flow works end-to-end
- ✅ Semantic search returns relevant products via pgvector
- ✅ All 8 domain schemas created and migrated

---

### **Release 1.1 (Post-MVP) — 3 Sprints**
**Goal:** Enhance usability, add compliance features, and prepare for Phase 2 extraction.

| Sprint | Duration | Focus | Key Deliverables |
|--------|----------|-------|------------------|
| **Sprint 5** | 2 weeks | Producer Portal & Compliance | - Producer dashboard to manage products, batches, certifications<br>- Verification status flow (Pending → Verified → Suspended)<br>- Compliance score calculation<br>- Recall initiation workflow|
| **Sprint 6** | 2 weeks | Loyalty, Consents & Preferences | - Loyalty program (points, Bronze/Silver/Gold/Platinum tiers)<br>- GDPR consent management<br>- Customer preference storage (fav categories, dietary tags)<br>- Address management with default flag|
| **Sprint 7** | 2 weeks | Analytics & Reporting | - Customer 360 read model (lifetime value, total orders, last active)<br>- Producer analytics (active products, total sales, compliance score)<br>- Revenue analytics by period<br>- Product affinity recommendations|

**Post-MVP Acceptance Criteria:**
- ✅ Producer can manage full product lifecycle via portal
- ✅ Loyalty points earned per order, tiers upgrade automatically
- ✅ All analytics read models populated and queryable
- ✅ Compliance scoring reflects certification validity

---

### **Release 2.0 (Phase 2) — 4 Sprints**
**Goal:** Scale the system with distributed infrastructure and third-party integrations.

| Sprint | Duration | Focus | Key Deliverables |
|--------|----------|-------|------------------|
| **Sprint 8** | 2 weeks | Distributed Messaging | - Migrate to Kafka/RabbitMQ for domain events<br>- Implement transactional outbox pattern<br>- Decouple order processing from payment gateways|
| **Sprint 9** | 2 weeks | Container & Orchestration | - Kubernetes deployment manifests<br>- Helm charts for all schemas<br>- Resource requests/limits per domain|
| **Sprint 10** | 2 weeks | Payment Gateway Integration | - Stripe & PayPal production integration<br>- Payment success/failure webhooks<br>- Refund workflow via returns domain|
| **Sprint 11** | 2 weeks | Full Microservice Extraction | - Extract commerce domain to independent service<br>- Extract traceability domain to independent service<br>- API gateway routing & service mesh|

**Phase 2 Success Metrics:**
- ✅ 99.9% uptime with Kubernetes HA
- ✅ <200ms p99 latency for product searches
- ✅ Zero data loss during schema migration
- ✅ Full PCI-DSS compliance for payment processing

---

## Cross-Cutting Concerns (All Sprints)

### **Technical Debt & Architecture**
- **Modular Monolith**: Keep business logic encapsulated within domain boundaries; avoid premature microservice extraction
- **Schema-per-Domain**: All data in single PostgreSQL instance with isolated schemas (platform, customer, commerce, order, traceability, operations, engagement, analytics)
- **Event-Driven**: Domain events published via internal event bus and outbox table; processed by Rust-based dispatcher
- **Transactional Outbox**: Ensure no event loss on database commit failure
- **pgvector AI**: Native vector search for semantic product discovery; no external vector database required initially

### **Quality Gates**
- All new endpoints must have OpenAPI 3.0 spec (`04-API-SPEC.yaml`)
- SQL migrations must pass `03-SCHEMA-DDL.sql` without errors
- Domain models must match `02-DOMAIN-MODEL.md` invariants
- Feature branches require passing: unit tests → integration tests → schema validation → API contract test

### **Definition of Done**
1. Code implemented and reviewed
2. SQL migration applied to dev database
3. API spec updated with new/changed endpoints
4. Domain model invariants validated
5. Unit test coverage ≥ 80%
6. Integration test suite passing
7. Documentation updated (all `./docs/` files)

---

## Risk Mitigation

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Schema migration breaks invariants | Medium | High | Run migrations in transaction; validate check constraints (`chk_batch_expiry`) after each sprint |
| pgvector version compatibility | Low | Medium | Pin pgvector extension version; test embedding dimension (1536) against OpenAI/sentence-transformers |
| Producer onboarding friction | Medium | Medium | Provide sandbox/test producer accounts; simplify verification flow for MVP |
| Mock payment → real gateway gap | Medium | High | Design payment abstraction layer early; mock gateway interface mirrors Stripe/PayPal API |
| Semantic search relevance | Low | Medium | Train embeddings with domain-specific data; iterate on query examples from actual user queries |

---

## Sprint Cadence & Rituals

- **Sprint Planning**: Monday 10:00 UTC — agree on sprint goal and backlog
- **Daily Standup**: 15 min async — progress, blockers, focus for today
- **Sprint Review**: Thursday 16:00 UTC — demo working software, collect feedback
- **Sprint Retrospective**: Friday 14:00 UTC — what went well, what to improve, action items
- **Release Cut**: End of final sprint day — tag version, generate changelog

---

## Success Metrics by Release

| Metric | Release 1.0 (MVP) | Release 1.1 | Release 2.0 |
|--------|-------------------|-------------|-------------|
| Products listed | ≥ 50 certified products | ≥ 200 products | ≥ 500 products |
| Active producers | ≥ 5 | ≥ 20 | ≥ 50 |
| Orders processed | ≥ 100 | ≥ 500 | ≥ 2,000 |
| Search queries/month | ≥ 500 | ≥ 5,000 | ≥ 20,000 |
| Average semantic search relevance | N/A (baseline) | ≥ 70% precision @ 5 | ≥ 85% precision @ 5 |
| System uptime | 99.5% | 99.8% | 99.9% |