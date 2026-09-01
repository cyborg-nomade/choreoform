<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# RP-03: E-commerce order fulfillment

| Attribute | Value |
| --- | --- |
| Domain | Commerce and logistics |
| Complexity | Level 3 — Long-running |
| Primary participants | Customer, commerce service, payment provider, warehouse, carrier, support agent |
| Typical duration | Seconds to several weeks |
| Automation mix | Software orchestration with physical fulfillment and human exceptions |

## Purpose and corpus role

Accept and fulfill a customer order without overselling, overcharging, or
hiding partial failure. The case couples parallel reservations, irreversible
and reversible effects, split shipments, asynchronous messages, and
compensation when the desired outcome can no longer be completed.

## Scope and assumptions

The case starts from a submitted order and ends when all lines are fulfilled,
cancelled, refunded, or placed into explicit exception handling. Pricing and
fraud decisions are external inputs. Returns after accepted delivery belong to
a separate process. This synthetic case is not accounting, consumer-law, tax,
or payment-compliance guidance.

## Participants

- **Customer:** submits, may cancel within policy, and receives status.
- **Commerce service:** owns order identity and coordinates outcomes.
- **Payment provider:** authorizes, captures, voids, or refunds money.
- **Warehouse:** reserves stock, picks, packs, and hands over shipments.
- **Carrier:** transports parcels and reports milestones.
- **Support agent:** resolves ambiguous or policy-sensitive exceptions.

## Trigger and preconditions

An order-submitted request contains a stable order identity, priced line
revisions, delivery choice, customer contact, payment reference, and the terms
accepted at submission. Products and fulfillment locations are resolvable.

## Information and state

- Order and line identities, quantities, money and currency, address revision,
  reservation and payment references, shipment groups, and tracking events.
- Per-line fulfillment rather than one misleading order-wide Boolean.
- Pending effects, compensation obligations, cancellation scope, and ambiguous
  provider outcomes.

## Main success path

1. Inventory reservation and payment authorization begin independently where
   policy permits.
2. Both succeed for all required lines, establishing an accepted order.
3. Lines are allocated into one or more warehouse shipments.
4. Each shipment is picked, packed, handed to a carrier, and tracked.
5. Payment is captured at the configured commitment point.
6. Delivery evidence for all non-cancelled lines completes the order.

## Alternatives and failures

- If inventory fails after payment authorization, the authorization is voided
  or allowed to expire; an ambiguous void is reconciled.
- Partial inventory may offer a split, backorder, substitution, or cancellation
  choice without silently changing the order.
- Customer cancellation stops work that can still be stopped and records
  compensation for reservations, captures, or labels already created.
- Warehouse rejection may reallocate a line; work already handed to a carrier
  follows an intercept or later return policy rather than fictional rollback.
- Duplicate payment or carrier events are recognized by stable provider event
  and effect identities.
- A permanent integration failure moves the affected obligation to visible
  human exception handling instead of declaring the entire order failed.

## Time, concurrency, and scale

Reservations expire, authorizations have validity windows, cancellation races
with packing, and delivery events may be delayed or reordered. Lines and
shipments can proceed in parallel subject to inventory and warehouse limits.
High-volume sales may start many instances at once; per-resource capacity and
back-pressure must not alter per-order invariants.

## Capabilities and effects

- **Reserve inventory:** acquire a time-bounded claim for identified lines.
- **Authorize, capture, void, or refund payment:** perform separately
  identifiable monetary effects with reconciliation support.
- **Request warehouse work:** transfer a versioned fulfillment instruction and
  receive attributable milestones.
- **Purchase or void shipment:** create a carrier commitment whose reversal may
  have different limits from its creation.
- **Notify customer:** communicate consolidated status without treating
  message delivery as proof of fulfillment.

## Invariants and protections

- Captured money never exceeds the amount justified by fulfilled or explicitly
  accepted commitments, net of recorded refunds.
- A unit cannot be promised to two orders through this process's reservation
  identity.
- A line's terminal outcome is exactly one of fulfilled, cancelled, or an
  explicit unresolved exception; an order summary is derived from its lines.
- Compensation is recorded as new work and evidence, never as erasure of the
  original effect.
- Sensitive payment data stays behind provider references and authorized
  capability boundaries.

## Observable outcomes

- Customer, operator, and each provider can correlate their identities without
  exposing unnecessary data.
- An operator can distinguish a requested effect, confirmed effect, confirmed
  rejection, and unknown outcome.
- History explains partial fulfillment and every remaining compensation
  obligation.

## Acceptance scenarios

### RP-03-A — Parallel acceptance and split delivery

- **Given** a two-line order with stock at different warehouses,
- **when** inventory and payment authorization succeed and both shipments are
  delivered,
- **then** each line is fulfilled, one allowed capture total is recorded, and
  the order completes despite separate delivery times.

### RP-03-B — Stock failure releases payment

- **Given** payment authorized while the last inventory unit is unavailable,
- **when** the reservation fails,
- **then** no shipment is requested and a void or expiry obligation remains
  visible until confirmed.

### RP-03-C — Cancellation races with packing

- **Given** an accepted order whose warehouse work is active,
- **when** cancellation and packed confirmation arrive concurrently,
- **then** defined ordering and provider facts determine whether shipment can
  stop, and any capture or refund obligations are explicit.

### RP-03-D — Duplicate capture confirmation

- **Given** one capture request already confirmed,
- **when** the same provider event is delivered again,
- **then** history may note duplicate delivery but money and order state change
  only once.

## Semantic pressures exposed

- How are parallel work, partial joins, and per-item state represented?
- How are uncertain external effects separated from business failure?
- How are cancellation and compensation scoped across concurrent branches?
- How does a compact visual view reveal, rather than conceal, outstanding
  obligations?

## Out of scope

- Catalog management, pricing, fraud models, taxation, customs, post-delivery
  returns, chargebacks, and warehouse route optimization.

## Sources and inspiration

- Synthetic composite informed by common commerce, payment, and logistics
  integration patterns; no provider contract is incorporated.
