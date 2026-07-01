# Phase 26 — Authorization-Runtime Handoff Spec Review Refinements

Status: Review refinements
Type: Documentation-only / spec-review-only
Base spec: `docs/gateway/phase-26-authorization-runtime-handoff-spec.md`
Base checkpoint: `Off-Chain Gateway Authorization Stack Complete`
Target checkpoint after review: `Authorization-Runtime Handoff Spec Reviewed`

## 0. Purpose

This document records the first review refinement pass for the Phase 26 authorization-runtime handoff specification.

The purpose is not to start Phase 27 implementation.

The purpose is to review and tighten the Phase 26 draft decisions before any future runtime handoff account mapping, guardian lifecycle model, replay storage model, or TS/SVM parity vector suite begins.

## 1. Preserved Security Decision

The core Phase 26 security decision remains unchanged:

~~~text
TS layer = preflight / model / watcher-side decision
Runtime = independent verifier
No authorized=true -> execute
~~~

The future runtime must not trust an off-chain boolean.

The handoff object must remain a structured proof bundle whose critical fields can be independently verified by runtime.

## 2. Scope Boundary

This review refinement is documentation-only.

It does not:

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

## 3. Review Result Summary

Phase 26 is directionally correct.

The review confirms that the selected handoff model avoids the unsafe pattern:

~~~text
authorized=true -> runtime executes
~~~

The review also confirms that the runtime must independently verify:

- canonical payload encoding
- payload hash
- guardian signatures
- guardian quorum
- guardian set lifecycle
- replay identity
- expiration
- route/runtime bindings
- amount limits
- target mint legitimacy

However, Phase 26 should not be considered fully reviewed until the draft decisions below are explicitly accepted, revised, or left as blockers for Phase 27.

## 4. Draft Decision Review

### 4.1 Message Nonce

Current Phase 26 draft:

~~~text
message_nonce = uniqueness-only signed message identifier
canonical_event_key = primary replay identity
~~~

Review refinement:

Accept this as the default direction for now.

Reason:

- `canonical_event_key` is already the replay anchor for the source burn event.
- Strict nonce ordering can introduce liveness problems if otherwise valid messages arrive out of order.
- Nonce ordering is not necessary to prevent duplicate minting if runtime replay storage is keyed by `canonical_event_key`.

Constraint:

`message_nonce` must not replace `canonical_event_key` as the replay key.

Future implementation must not use nonce-only replay protection.

### 4.2 Expiration

Current Phase 26 draft:

~~~text
runtime expiration should be slot-based
current_slot == expiration_slot => valid
current_slot > expiration_slot => expired
~~~

Review refinement:

Accept slot-based expiration and inclusive boundary as the default direction.

Reason:

- Slot is directly available to runtime.
- Slot avoids wall-clock ambiguity.
- The inclusive boundary matches Phase 25 behavior.

Required future addition:

Before runtime implementation, define a maximum validity window:

~~~text
expiration_slot <= observed_finality_slot + max_validity_slots
~~~

This prevents extremely long-lived signed payloads.

### 4.3 Replay Storage

Current Phase 26 draft:

~~~text
runtime execution requires on-chain ProcessedEvent storage
off-chain DB may exist only as index/preflight support
~~~

Review refinement:

Accept this direction.

Reason:

Runtime replay protection must be in the same trust domain as execution.

Off-chain replay state is useful for watcher/indexer/preflight, but it must not be the runtime source of truth.

Required invariant:

~~~text
verify canonical event key
-> check unprocessed
-> execute corresponding mint
-> mark the same canonical event key processed
~~~

This must be atomic.

### 4.4 Guardian Set Lifecycle

Current Phase 26 draft defines the lifecycle direction:

~~~text
Created -> Active -> Expired
                 -> Revoked
                 -> Rotated
~~~

Review refinement:

Direction accepted, but not final enough for runtime implementation.

Required before Phase 27 implementation:

- exact `active_from` semantics
- exact `expires_at` semantics
- whether expiration is slot-based
- whether activation is slot-based
- whether revoked always overrides active/expired
- whether rotated sets are treated as expired or separately tracked
- whether payloads signed before expiration but submitted after expiration are rejected
- whether any grace window exists

Recommended default:

No grace window.

A payload submitted after guardian set expiration should be rejected unless a later reviewed spec explicitly introduces a grace policy.

### 4.5 Amount Caps

Current Phase 26 draft:

~~~text
minimum required before runtime execution:
per-route max amount per message
~~~

Review refinement:

Accept per-route max amount per message as minimum required before any live execution.

Do not require rolling-window accounting in the first runtime model unless there is a clear implementation reason.

Reason:

- per-message caps are simpler and easier to verify
- rolling caps require additional runtime state
- rolling cap state introduces more account mutation and atomicity surface

Open for later:

- rolling window cap
- global cap
- emergency lowered cap

This is not a rejection of rolling, global, or emergency-lowered caps.

It only means the first runtime model should not require additional rolling-window accounting unless that extra state surface is separately reviewed.

### 4.6 Emergency Halt

Current Phase 26 draft:

~~~text
emergency halt must be explicit runtime config state
halted route cannot execute new mints
halt does not retroactively invalidate already processed events
~~~

Review refinement:

Direction accepted, authority model still open.

The spec must not hide the governance/admin tradeoff.

Open design question remains:

~~~text
Who can trigger halt while preserving no-admin / limited-authority principles?
~~~

Required before live execution:

- halt authority model
- halt activation rule
- halt audit/proof log behavior
- halt interaction with guardian set
- halt interaction with pending but unprocessed signed payloads
- public status/reporting requirement

### 4.7 Target Mint Legitimacy

Review refinement:

Phase 26 is correct that target mint legitimacy must be checked against runtime config, not caller-provided fields.

Required future checks:

- `target_mint` matches route config
- target mint is expected XXXL/wrapped asset mint
- mint authority model is valid
- mint decimals and supply policy match route assumptions
- SPL Token mint account validation is consistent with the account contract review boundary

### 4.8 TS/SVM Parity

Review refinement:

The TS implementation must not become the informal runtime spec.

Before runtime implementation, create a parity vector suite based on canonical specs, not by copying TS behavior blindly.

Required vector coverage:

- valid canonical payload
- wrong field order
- wrong byte encoding
- wrong hash domain
- malformed bytes32 fields
- malformed var bytes
- invalid source chain id
- wrong target mint
- wrong guardian set id
- wrong source chain weight
- invalid signature
- duplicate guardian approval
- insufficient quorum
- expired payload
- duplicate canonical event key
- wrong canonical event key preimage
- wrong source burn tx hash
- wrong source burn event index
- amount over route cap
- invalid target mint

## 5. Phase 27 Candidate Readiness

After this review refinement, the safest Phase 27 candidates remain:

- runtime handoff account mapping
- guardian set lifecycle model
- replay storage design model
- TS/SVM parity vector suite

The preferred next candidate is:

~~~text
TS/SVM parity vector suite
~~~

Reason:

Parity vectors reduce the risk that the Rust/SVM runtime later implements a subtly different interpretation of the Phase 22/23 payload.

## 6. Not Yet Ready For

This review refinement does not make the project ready for:

- live route activation
- SPL CPI execution
- `invoke_signed`
- SPL Token `mint_to`
- processed-event mutation in the live runtime path
- production Program ID selection
- production PDA fixture regeneration
- deployment blocker removal
- deployability predicate changes

## 7. Recommended Follow-up

After this document is reviewed, the next checkpoint can be:

~~~text
Authorization-Runtime Handoff Spec Reviewed
~~~

But only if reviewers accept or resolve the remaining open decisions:

- guardian lifecycle exact semantics
- emergency halt authority model
- max expiration validity window
- amount cap minimum policy
- TS/SVM parity vector scope
