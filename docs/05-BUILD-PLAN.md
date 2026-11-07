# BioSouk — Build Plan: Sprint 3

**Sprint:** 3  
**Period:** Following Sprint 2 (Core Domain Infrastructure)  
**Version:** 1.1  
**Status:** Active

---

## 1. Sprint Goal

Deliver the marketplace transaction infrastructure and prepare the foundation for frontend integration. This sprint builds on the core domain infrastructure from Sprint 2 to enable order placement, payment processing, and product traceability, while establishing synthetic data and testing capabilities for the complete workflow.

---

## 2. Sprint Focus

| Focus Area | Key Activities |
|---|---|
| **Order Management** | Implement shopping cart, order creation, and order status management |
| **Payment Integration** | Implement payment provider abstraction, payment initiation, and webhook handling |
| **Product Traceability** | Enhance farm/certification/batch management with full lifecycle tracking |
| **API Expansion** | Implement REST endpoints for orders, payments, and traceability queries |
| **Testing Expansion** | Write integration tests for end-to-end business workflows |
| **Synthetic Data** | Expand generator with dataset profiles for edge cases and full marketplace simulation |

---

## 3. Milestones and Deliverables

### Milestone SM-3.1 — Marketplace and Order Management

**Objective:** Implement the purchasing workflow for the BioSouk marketplace.

**Deliverables:**

- Domain entities: `ShoppingCart`, `Order`, `OrderItem`, `OrderStatus`
- Repository interfaces and SQLx implementations
- Cart management API endpoints (add, remove, update items)
- Order creation API with inventory validation
- Order status tracking and transitions
- Order cancellation API
- Customer order history endpoint

**Acceptance criteria:**

- Customers can add products to cart and place orders
- Order creation validates inventory availability (no overselling)
- Inventory is decremented atomically on order placement
- Orders have valid status transitions (pending → confirmed → processing → shipped → delivered/cancelled)
- Order cancellation restores inventory quantities
- Customer order history returns orders with correct pagination
- Search and listing endpoints support order filtering

---

### Milestone SM-3.2 — Payment Integration

**Objective:** Implement secure payment processing for marketplace orders.

**Deliverables:**

- Payment provider abstraction layer (plug-in architecture)
- Payment initiation API for order payment
- Payment confirmation and status tracking
- Webhook handler for payment provider callbacks
- Refund processing API
- Payment idempotency key support
- Payment audit records

**Acceptance criteria:**

- Orders can be marked as paid after successful payment initiation
- Webhooks are authenticated and duplicate events are handled safely
- Failed payments are handled without corrupting order state
- Refunds are traceable and reduce order payment status appropriately
- Idempotency keys prevent duplicate payment processing
- Payment audit records are created for all transactions
- Sensitive payment credentials are never stored in application logs

---

### Milestone SM-3.3 — Product Traceability Enhancement

**Objective:** Extend farm and certification management with full batch traceability lifecycle.

**Deliverables:**

- Domain events: `ProductionStarted`, `HarvestCompleted`, `QualityInspection`, `PackagingCompleted`, `DistributionStarted`, `DeliveryCompleted`
- Traceability event repository and SQLx implementation
- Batch lifecycle management from production to delivery
- Certification validity checking with state transitions
- Traceability history API endpoint
- Product journey representation

**Acceptance criteria:**

- Batches track complete journey from production to delivery
- Certification states (valid, expired, pending, rejected) are enforced
- Traceability events reference valid products, farms, and certifications
- History API returns chronological event sequence for a given product/batch
- Expired certifications prevent associated batches from being marked as valid
- Administrative review workflow for certification approval/rejection

---

### Milestone SM-3.4 — Synthetic Data and Testing Expansion

**Objective:** Establish comprehensive test coverage and synthetic data for end-to-end workflows.

**Deliverables:**

- Synthetic data generator profiles: `full` (all domains), `orders` (order-focused), `payments` (payment-focused), `traceability` (traceability-focused)
- Edge case scenarios: cancelled orders, failed payments, expired certifications, zero inventory
- Seed-based reproducibility for all new profiles
- Integration test fixtures for complete business workflows
- End-to-end test scenarios for order placement through payment and traceability

**Acceptance criteria:**

- Generator produces `full` dataset with realistic records across all domains including orders and payments
- Edge case scenarios are generatable with `--profile edge-cases`
- Dataset is reproducible with the same seed across all new profiles
- Integration test fixtures cover complete workflows: cart → order → payment → traceability
- End-to-end tests pass for core marketplace business flows

---

### Milestone SM-3.5 — Frontend Integration Preparation

**Objective:** Prepare API contracts and documentation for frontend development streams.

**Deliverables:**

- OpenAPI documentation updated for all new endpoints
- API contract specifications for order, payment, and traceability services
- Request/response schemas validated
- Authentication flow documented for protected routes

**Acceptance criteria:**

- OpenAPI spec includes all new order, payment, and traceability endpoints
- Request/response schemas validate against implemented handlers
- Authentication middleware documented and tested for new endpoints
- API contracts are consumer-friendly for frontend team

---

## 4. Sprint Dependencies

```
SM-3.1 (Marketplace/Orders)       →  SM-3.2 (Payments)       →  SM-3.3 (Traceability Enhancement)
             ↓                                        ↑                              |
             └──────────────────────────────────────┘                              ↓
                                                    SM-3.4 (Synthetic Data / Testing)
                                                   ↓
                                              SM-3.5 (Frontend Preparation)

**Parallel workstreams (may begin once interfaces are stable):**

- Frontend wireframes and UI components for order management can begin during backend foundation work
- Synthetic data generator design can begin during milestone implementation
- Payment provider integration can progress once payment abstraction interface exists
- Traceability can progress independently of payment integration after domain interfaces exist

---

## 5. Definition of Done (Sprint 3)

A milestone is complete only when:

- [ ] Required functionality is implemented per the deliverables
- [ ] Domain rules are enforced and tested
- [ ] Automated tests pass (unit, integration, API, end-to-end)
- [ ] Security requirements are satisfied (auth, RBAC, payment security, data validation)
- [ ] API documentation is updated (OpenAPI)
- [ ] Relevant architectural decisions are documented
- [ ] Code passes formatting (`cargo fmt`) and linting (`cargo clippy`)
- [ ] No unresolved critical defects remain
- [ ] Changes are committed to Git with conventional commit messages
- [ ] Synthetic data generator produces valid dataset for new profiles

**Sprint 3 is complete when all five milestones (SM-3.1 through SM-3.5) meet their acceptance criteria.**

---

## 6. Acceptance Criteria Summary

| Milestone | Key Deliverable |
|---|---|
| SM-3.1 | Order management: cart, creation, status, cancellation |
| SM-3.2 | Payment integration: provider abstraction, webhooks, refunds |
| SM-3.3 | Traceability enhancement: batch lifecycle, certification states |
| SM-3.4 | Synthetic data generator: full/edge-case profiles; test fixtures |
| SM-3.5 | Frontend preparation: OpenAPI docs; API contracts validated |

---

## 7. Git Workflow

- Branch from `develop` for sprint work
- Feature branches: `feature/sm-3-x-description`
- Merge via pull request into `develop`
- Conventional commit messages required

**Example commit messages:**

```bash
git commit -m "feat(biosouk): implement shopping cart and order creation (SM-3.1)"
git commit -m "feat(biosouk): add payment provider abstraction and webhook handler (SM-3.2)"
git commit -m "feat(biosouk): extend batch traceability with event lifecycle (SM-3.3)"
git commit -m "feat(biosouk): expand synthetic data generator with full and edge-case profiles (SM-3.4)"
git commit -m "feat(biosouk): update OpenAPI documentation for order/payment/traceability endpoints (SM-3.5)"
```

---

## 8. References

- `docs/01-product-specification.md` — Product requirements
- `docs/02-domain-model.md` — Domain model entities and relationships
- `docs/03-system-architecture.md` — System architecture and bounded contexts
- `docs/04-domain-workflows.md` — Domain workflows and use cases
- `docs/05-database-design.md` — Database design and schema details
- `docs/06-api-specification.md` — API contracts and endpoints
- `docs/07-synthetic-data-generation.md` — Synthetic data generator specification
- `docs/08-implementation-plan.md` — Full implementation plan and milestones M1–M11
- `docs/constitution.md` — Project constitution and engineering standards
- `quickstart.md` — Project foundation validation scenarios