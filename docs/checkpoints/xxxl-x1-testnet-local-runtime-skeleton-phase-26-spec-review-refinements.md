# XXXL X1 Testnet Local Runtime Skeleton — Phase 26 Spec Review Refinements

Status: Review refinements
Type: documentation-only / spec-review-only

## Summary

This checkpoint records a documentation-only review refinement pass over the Phase 26 authorization-runtime handoff specification.

Primary document:

~~~text
docs/gateway/phase-26-authorization-runtime-handoff-spec-review-refinements.md
~~~

Base spec:

~~~text
docs/gateway/phase-26-authorization-runtime-handoff-spec.md
~~~

## Purpose

The purpose is to review the draft Phase 26 handoff decisions before any Phase 27 implementation work.

This checkpoint does not start Phase 27.

## Preserved Security Decision

The Phase 26 security decision remains:

~~~text
TS layer = preflight / model / watcher-side decision
Runtime = independent verifier
No authorized=true -> execute
~~~

Runtime must not trust an off-chain boolean.

## Review Refinement Outcomes

The review refinement accepts as default direction:

- `message_nonce` as uniqueness-only signed message identifier
- `canonical_event_key` as primary replay identity
- slot-based expiration
- inclusive expiration boundary
- on-chain `ProcessedEvent` storage for runtime replay protection
- per-route max amount per message as minimum amount cap
- explicit emergency halt runtime config state
- runtime-config-based target mint legitimacy
- TS/SVM parity vectors before runtime implementation

The review leaves open before future live execution:

- exact guardian set lifecycle semantics
- emergency halt authority model
- max expiration validity window
- optional rolling/global amount cap design
- complete TS/SVM parity vector scope and fixture format

## Phase Boundary

This checkpoint does not:

- implement runtime code
- modify `programs/xxxl-svm`
- modify Cargo files
- modify package files
- build SBF artifacts
- touch `target/deploy`
- touch keypair files
- inspect `.local-keys`
- read `.env`
- enable live route
- enable SPL CPI
- enable `invoke_signed`
- enable SPL Token `mint_to`
- mutate runtime/account state
- mark processed events
- claim production readiness
- claim final immutability while upgrade authority exists

## Active Blockers

All relevant blockers remain active.

Explicitly preserved blockers:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

This checkpoint does not remove or weaken any deployment blocker.

## Recommended Next Step

Run review on the refinement document.

If accepted, create the next checkpoint:

~~~text
Authorization-Runtime Handoff Spec Reviewed
~~~

Preferred future Phase 27 candidate after review:

~~~text
TS/SVM parity vector suite
~~~

No live execution should be enabled by Phase 27 without a separate reviewed boundary.
