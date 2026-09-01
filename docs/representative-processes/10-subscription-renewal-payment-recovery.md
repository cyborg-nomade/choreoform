<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# RP-10: Subscription renewal and payment recovery

| Attribute | Value |
| --- | --- |
| Domain | Billing operations |
| Complexity | Level 2 — Coordinated |
| Primary participants | Customer, subscription service, billing system, payment provider, support agent |
| Typical duration | Seconds to several weeks |
| Automation mix | Timer- and event-driven automation with customer and support intervention |

## Purpose and corpus role

Renew a subscription or reach a clear non-renewal outcome when payment does not
immediately succeed. The case stresses scheduled work, asynchronous provider
events, stable monetary-effect identity, configurable retry policy, customer
action, entitlement changes, and late success after cancellation or expiry.

## Scope and assumptions

The case begins before a subscription period ends and covers invoice creation,
payment attempts, a bounded recovery sequence, and renewal or non-renewal.
Pricing, taxation, accounting, consumer rights, card-network rules, and the
payment provider's exact state machine are external. This synthetic case is not
financial, legal, tax, or payment-compliance advice.

## Participants

- **Customer:** receives permitted notices, may update payment information,
  authenticate, cancel, or contact support.
- **Subscription service:** owns the commercial subscription and entitlement
  decision.
- **Billing system:** creates invoice identity and tracks the amount due.
- **Payment provider:** attempts payment and emits asynchronous outcomes.
- **Support agent:** resolves approved exceptions without editing provider facts.

## Trigger and preconditions

A renewal-due schedule identifies the subscription, customer, product and price
revision, period boundary, billing configuration, payment reference, notice
requirements, and recovery policy version. The subscription must still be
eligible to renew when the effect is attempted.

## Information and state

- Subscription, renewal cycle, invoice, payment intent, attempt, provider event,
  and entitlement-effect identities.
- Amount and currency, policy revision, next attempt or expiry time, customer
  action required, notices, cancellation effective time, and provider outcome.
- Commercial status, payment status, communication status, and entitlement
  state kept distinct.

## Main success path

1. Required pre-renewal communication is scheduled and recorded separately from
   the renewal decision.
2. One invoice and stable payment intent are created for the renewal cycle.
3. Payment succeeds immediately or after required customer authentication.
4. The subscription advances exactly one period and entitlement is confirmed.
5. The customer receives a receipt; notification delivery does not alter the
   financial outcome.

## Alternatives and failures

- A transient or decline outcome starts a policy-bounded sequence of retries
  and notices; each attempt has its own identity but targets one payment intent.
- A provider requests customer action; the process waits until action, expiry,
  cancellation, or a provider result wins under defined ordering.
- Customer cancellation prevents attempts not yet effective and applies the
  configured end-of-service rule; it does not erase an already-settled invoice.
- Exhausted recovery ends renewal and changes entitlement according to explicit
  policy, with support exceptions recorded separately.
- Duplicate or out-of-order webhooks are retained or deduplicated without
  regressing later provider state.
- A late success after apparent expiry triggers reconciliation before access is
  extended, refunded, or escalated; it is never ignored or blindly applied.
- A notification failure retries independently of payment.

## Time, concurrency, and scale

Period boundaries, notice windows, retry schedules, authentication expiry, and
grace periods coexist. Payment events, customer cancellation, payment-method
updates, and timers may race. Large services run many cycles concurrently and
must apply rate limits and provider back-pressure without creating duplicate
attempts or drifting contractual times.

## Capabilities and effects

- **Create invoice or payment intent:** establish stable provider-correlated
  monetary identity for one renewal cycle.
- **Attempt or reconcile payment:** distinguish requested, processing,
  succeeded, declined, action-required, and unknown outcomes.
- **Schedule policy timer:** retain the policy and time basis that selected it.
- **Change entitlement:** perform a separately observable, reversible where
  allowed, service-access effect.
- **Notify customer:** send revisioned content for the appropriate state and
  record delivery independently.

## Invariants and protections

- One renewal cycle cannot create multiple settled charges through retry or
  duplicate event delivery.
- Subscription and entitlement advance only from an authorized, reconciled
  outcome and at most once per cycle.
- A later provider fact cannot be discarded solely because the local process
  previously timed out or changed state.
- Retry timing and content use the identified policy revision and respect
  cancellation and required customer action.
- Payment credentials remain behind provider references and restricted
  capability boundaries.

## Observable outcomes

- Customer and support can distinguish payment retry, customer action required,
  grace period, cancelled, expired, paid, and entitlement reconciliation.
- Operators can correlate provider events and attempts and explain why each
  timer or notice occurred.
- Financial and access effects can be reconciled independently after failure.

## Acceptance scenarios

### RP-10-A — Immediate renewal

- **Given** an eligible subscription and one renewal invoice,
- **when** its stable payment intent succeeds,
- **then** the subscription and entitlement advance once and a receipt is
  requested independently.

### RP-10-B — Recovery succeeds after update

- **Given** an initial decline under a policy allowing retry,
- **when** the customer updates the payment method and the correlated retry
  succeeds,
- **then** the same renewal cycle is paid once and remaining retry timers are
  cancelled.

### RP-10-C — Cancellation races with scheduled retry

- **Given** a failed renewal with a future retry,
- **when** cancellation and the retry timer become effective near the same time,
- **then** defined ordering and the cancellation policy determine whether the
  attempt is allowed, with no duplicate charge.

### RP-10-D — Late success after local expiry

- **Given** the local instance ended recovery while a provider attempt was
  unresolved,
- **when** a verified late success arrives,
- **then** reconciliation creates an explicit reinstate, credit, refund, or
  human-review outcome rather than silently ignoring or applying it.

## Semantic pressures exposed

- How are schedules and asynchronous facts combined without timer races?
- How are one business effect and its multiple technical attempts related?
- How does policy versioning affect already-scheduled retries?
- How can a visual representation show independent financial, subscription,
  communication, and entitlement state without becoming four divergent models?

## Out of scope

- Price calculation, taxation, revenue recognition, payment credential storage,
  chargebacks, collections, provider-specific retry algorithms, and statutory
  notice content.

## Sources and inspiration

- [Stripe, Using webhooks with subscriptions](https://docs.stripe.com/billing/subscriptions/webhooks)
- [Stripe, Smart Retries](https://docs.stripe.com/billing/revenue-recovery/smart-retries)
- These sources illustrate asynchronous billing and retry pressures; the case
  is provider-neutral and does not reproduce Stripe behavior or policy.
