# Phase 26 — Authorization-Runtime Handoff Spec Reviewed

Status: Reviewed
Type: Documentation-only / review-closure-only
Base spec: `docs/gateway/phase-26-authorization-runtime-handoff-spec.md`
Review refinements: `docs/gateway/phase-26-authorization-runtime-handoff-spec-review-refinements.md`
Target checkpoint: `Authorization-Runtime Handoff Spec Reviewed`

## 0. Purpose

This document closes the Phase 26 review gate for the authorization-runtime handoff specification.

This is not a Phase 27 implementation document.

This document records that the Phase 26 handoff model has been reviewed after the spec-review refinement pass and is acceptable as the boundary for choosing a future Phase 27 candidate.

## 1. Reviewed Inputs

The review closure is based on:

- Phase 26 authorization-runtime handoff specification
- Phase 26 spec review refinements
- read-only audit result for the spec review refinements
- preserved deployment/runtime blockers
- existing XXXL runtime scaffold safety boundaries

## 2. Preserved Core Security Decision

The reviewed security decision remains:

~~~text
TS layer = preflight / model / watcher-side decision
Runtime = independent verifier
No authorized=true -> execute
~~~

The future runtime must not trust an off-chain boolean.

The runtime must independently verify all authority-bearing data before any future execution path can mutate state or mint tokens.

## 3. Review Closure Decision

The Phase 26 handoff model is accepted for review-closure purposes.

This means:

- the off-chain TypeScript authorization stack may be used as preflight/model/watcher-side logic
- the future runtime must independently verify the handoff proof bundle
- the handoff proof bundle must not be treated as an authorization boolean
- caller-provided `payload_hash` must not be trusted
- caller-provided encoded payload must be recomputed/validated
- guardian signatures must be verified by runtime rules
- guardian quorum must be verified by runtime rules
- replay protection must be enforced in runtime state before execution can be live
- route bindings must be checked against runtime config
- target mint legitimacy must be checked against runtime config
- failure before execution must not mutate runtime state
- no processed-event marking may occur unless paired atomically with the corresponding successful future execution

## 4. Accepted Review Defaults

The following defaults are accepted as Phase 26 review defaults.

### 4.1 Message Nonce

Accepted default:

~~~text
message_nonce = uniqueness-only signed message identifier
canonical_event_key = primary replay identity
~~~

The nonce must not replace `canonical_event_key`.

Nonce-only replay protection is rejected.

### 4.2 Expiration

Accepted default:

~~~text
runtime expiration is slot-based
current_slot == expiration_slot => valid
current_slot > expiration_slot => expired
~~~

A maximum validity window must be defined before any runtime verifier can accept live messages.

### 4.3 Replay Storage

Accepted default:

~~~text
runtime execution requires on-chain ProcessedEvent replay storage
off-chain replay state is index/preflight support only
~~~

Required future atomicity invariant:

~~~text
verify canonical event key
-> check unprocessed
-> execute corresponding mint
-> mark the same canonical event key processed
~~~

No mint without mark.

No mark without mint.

### 4.4 Amount Cap Minimum

Accepted default:

~~~text
per-route max amount per message is the minimum required amount-control surface
~~~

This does not reject later rolling, global, or emergency-lowered caps.

Rolling/global/emergency caps remain future reviewed extensions.

### 4.5 Target Mint Legitimacy

Accepted default:

Runtime must check target mint legitimacy against runtime config.

Caller-provided target mint fields are not authority.

### 4.6 TS/SVM Parity

Accepted default:

Before future runtime implementation, create a TS/SVM parity vector suite.

The TypeScript implementation must not become the informal runtime spec.

The parity suite must be based on canonical specs and fixed vectors.

## 5. Open Items Reclassified As Future Gated Work

The following items are not blockers to closing Phase 26 review.

They are blockers to future runtime implementation or live execution unless separately specified and reviewed.

### 5.1 Guardian Set Lifecycle

Still required before runtime guardian-set execution:

- exact `active_from` semantics
- exact `expires_at` semantics
- slot/timestamp basis
- revoked vs expired priority
- rotated set handling
- payload submission after guardian-set expiration
- whether any grace window exists

Reviewed default until changed:

~~~text
No grace window.
Submission after guardian-set expiration is rejected.
~~~

### 5.2 Emergency Halt Authority

Emergency halt remains accepted as an explicit runtime-config concept.

The authority model is not yet closed.

Required before live execution:

- who can halt
- who can unhalt
- whether halt is guardian-controlled, program-controlled, multisig-controlled, or permanently unavailable
- audit/proof log behavior
- interaction with pending but unprocessed payloads
- public status/reporting requirement

This review does not claim that emergency halt authority is solved.

### 5.3 Maximum Expiration Validity Window

A max validity rule is required before runtime acceptance of live messages.

Draft shape:

~~~text
expiration_slot <= observed_finality_slot + max_validity_slots
~~~

The exact `max_validity_slots` value is not selected in Phase 26.

### 5.4 Rolling / Global / Emergency Amount Caps

Per-message cap is accepted as minimum.

Rolling/global/emergency caps are not required for the first parity-vector phase.

They may be required before live route activation depending on future review.

## 6. Phase 27 Candidate Gate

Phase 27 is now allowed only as a bounded follow-up phase selected from reviewed candidates.

Allowed safe Phase 27 candidates:

- TS/SVM parity vector suite
- guardian set lifecycle model
- replay storage design model
- runtime handoff account mapping

Preferred first Phase 27 candidate:

~~~text
TS/SVM parity vector suite
~~~

Reason:

The parity vector suite reduces the risk that a future Rust/SVM verifier interprets the Phase 22/23 payload differently from the reviewed canonical model.

This candidate does not enable live execution.

## 7. Still Not Ready For

This review closure does not make the project ready for:

- live route activation
- SPL CPI execution
- `invoke_signed`
- SPL Token `mint_to`
- runtime/account mutation in a live execution path
- processed-event marking in a live execution path
- production Program ID selection
- production PDA fixture regeneration
- deployment blocker removal
- deployability predicate changes
- production readiness claim
- final immutability claim while upgrade authority exists

## 8. Explicitly Preserved Blockers

The following blockers remain active:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

This document does not remove, weaken, rename, or satisfy any deployment blocker.

## 9. Checkpoint Statement

The Phase 26 authorization-runtime handoff spec is reviewed for boundary purposes.

The project may proceed to a bounded Phase 27 candidate.

The recommended first Phase 27 candidate is TS/SVM parity vectors.

Phase 27 must still preserve all live-execution blockers unless a separate reviewed boundary explicitly changes them.

No live execution is enabled by this checkpoint.
