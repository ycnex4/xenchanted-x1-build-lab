# Phase 26 — Authorization-Runtime Handoff Specification

Status: Draft v0.1
Scope: Specification only
Base checkpoint: Off-Chain Gateway Authorization Stack Complete
Base commit: `0f2d712`
Latest completed phase: Phase 25 — gateway authorization decision boundary

## 0. Security Decision

This specification starts from one security decision:

**The runtime must not trust an off-chain boolean.**

The selected model is:

~~~text
TS layer = preflight / model / watcher-side decision
Runtime = independent verifier
No authorized=true -> execute
~~~

The handoff from the off-chain layer to the future runtime must be a structured proof bundle, not a trusted boolean.

The future runtime must independently verify the critical authorization conditions before any execution, state mutation, processed-event marking, or mint operation.

## 1. Purpose

Phase 20 through Phase 25 completed the off-chain TypeScript authorization stack:

~~~text
guardian payload semantics
-> canonical byte encoding
-> domain-separated payload hash
-> Ed25519 guardian signatures
-> guardian quorum verification
-> replay check
-> expiration check
-> route/runtime binding check
-> authorization decision
~~~

Phase 26 defines how this off-chain decision layer should hand off data to a future runtime verification layer.

This document does not implement runtime logic.

This document does not enable:

- live route execution
- SPL CPI
- `invoke_signed`
- SPL Token `mint_to`
- runtime/account mutation
- processed-event marking
- production readiness
- final immutability while upgrade authority exists

## 2. Canonical AuthProofBundle

The handoff object must be a structured bundle.

It must contain enough data for the future runtime to independently verify the authorization, not merely trust a precomputed decision.

Proposed canonical bundle:

~~~text
AuthProofBundle {
  payload_fields
  encoded_payload
  payload_hash
  guardian_set_id
  guardian_public_keys
  guardian_signatures
  guardian_threshold
  runtime_binding_expectations
  replay_identity
  expiration_context
  amount_limit_context
  target_mint_context
}
~~~

### 2.1 Payload fields

The bundle must include the canonical Phase 22/23 payload fields:

~~~text
message_type
schema_version
instruction_layout_version
route_id
source_chain_id
source_token
source_sender
source_burn_tx_hash
source_burn_event_index
source_block_number
source_block_hash
source_finality_block
canonical_event_key
x1_recipient
burned_amount
source_chain_weight_bps
xxxl_mint_amount
target_mint
guardian_set_id
message_nonce
expiration_slot_or_unix_ts
~~~

### 2.2 Encoded payload and payload hash

The bundle may include `encoded_payload` and `payload_hash` for observability and debugging.

However, the runtime must not trust those values blindly.

The runtime must be able to recompute:

~~~text
encoded_payload = canonical_binary_encode(payload_fields)
payload_hash = keccak256(domain_separator || encoded_payload)
~~~

### 2.3 Guardian approvals

The bundle must include guardian approvals as data:

~~~text
GuardianApproval {
  guardian_public_key
  guardian_signature
}
~~~

Guardians sign the 32-byte Phase 23 payload hash.

They do not sign:

- a caller-supplied hash
- a JSON object
- the full preimage
- a runtime instruction blob
- a mutable off-chain decision object

### 2.4 Runtime binding expectations

The bundle must include or imply the expected runtime bindings:

~~~text
route_id
source_chain_id
target_mint
guardian_set_id
source_chain_weight_bps
~~~

The future runtime must compare the signed payload fields against its own route/config state.

The off-chain bundle cannot be the source of truth for runtime bindings.

## 3. Runtime MUST Verify

The future runtime must verify the following checks independently.

### 3.1 Canonical payload encoding

Runtime must verify that the payload fields serialize exactly according to the Phase 23 canonical byte encoding.

Reason: if TS and SVM encode differently, guardians may sign one message while runtime verifies another.

### 3.2 Payload hash

Runtime must recompute the domain-separated payload hash.

Reason: a caller-supplied `payload_hash` must not be authoritative.

### 3.3 Guardian signatures

Runtime must verify Ed25519 signatures over the 32-byte payload hash.

Reason: signature verification is the cryptographic authorization root.

### 3.4 Guardian quorum

Runtime must verify quorum using distinct, known, active guardian public keys.

Reason: replaying one valid signature multiple times must not satisfy quorum.

### 3.5 Guardian set lifecycle

Runtime must verify that `guardian_set_id` refers to a valid guardian set for the payload context.

The guardian set must be:

- known
- active at the relevant verification time
- not revoked
- not expired for the payload being executed

### 3.6 Replay identity

Runtime must verify that `canonical_event_key` has not already been processed.

Reason: the same source event must not mint twice.

### 3.7 Expiration

Runtime must verify payload expiration using the canonical expiration semantics selected in this spec.

Reason: stale signed payloads must not remain valid indefinitely.

### 3.8 Runtime route bindings

Runtime must verify that signed payload fields match runtime route/config state:

- `route_id`
- `source_chain_id`
- `target_mint`
- `guardian_set_id`
- `source_chain_weight_bps`

Reason: guardians must not authorize one route while runtime executes another.

### 3.9 Amount limits

Runtime must verify route-level and/or global amount limits.

Reason: a valid signature should not automatically authorize unlimited mint size.

### 3.10 Target mint legitimacy

Runtime must verify that `target_mint` is the legitimate mint for the route/wrapped asset.

Reason: matching a user-provided mint is not enough; the runtime source of truth must define legitimate mints.

## 4. Guardian Set Lifecycle

Guardian set lifecycle must be explicit.

Proposed states:

~~~text
Created -> Active -> Expired
                 -> Revoked
                 -> Rotated
~~~

### 4.1 Created

A guardian set exists but is not yet valid for signing.

### 4.2 Active

A guardian set can authorize payloads.

Required data:

~~~text
guardian_set_id
guardian_public_keys
threshold
active_from
expires_at
revoked
~~~

### 4.3 Expired

An expired set cannot authorize new payloads.

Open design point:

A payload signed before expiration but submitted after expiration should be rejected unless the final spec explicitly allows a grace window.

Default recommendation: reject after expiration.

### 4.4 Revoked

A revoked guardian set must not authorize payloads.

Revocation should be stronger than expiration.

### 4.5 Rotated

Rotation creates a new active set.

The previous set may become expired or revoked depending on governance/security policy.

Open design point:

Historical set validity must be defined. A payload signed by an old set before rotation must not remain ambiguous.

## 5. Replay Storage Architecture

Replay protection must be atomic relative to future execution.

Phase 25 checks a processed registry snapshot, but this is only an off-chain model.

Future runtime must define actual replay storage.

### 5.1 Recommended model

Use on-chain `ProcessedEvent` accounts keyed by `canonical_event_key`.

Reason:

~~~text
check not processed
-> execute mint
-> mark processed
~~~

must be atomic in one transaction boundary.

### 5.2 Forbidden model

Do not rely on off-chain-only replay storage for runtime execution.

Reason: if replay check and mint execution live in different trust domains, race conditions and double-execution risks appear.

### 5.3 Hybrid model

A hybrid model may exist for watcher indexing and UI.

However, the runtime execution path must still have an on-chain replay commitment.

### 5.4 Atomicity requirement

The runtime must guarantee:

~~~text
If mint execution succeeds, processed marking succeeds atomically.
If processed marking cannot be guaranteed, mint execution must not occur.
If verification fails, processed marking must not occur.
~~~

## 6. Message Nonce Semantics

`message_nonce` is currently part of the signed payload.

Its semantics must be explicit before runtime integration.

Possible semantics:

### 6.1 Uniqueness only

The nonce provides unique message identity but does not impose ordering.

This is simpler and aligns with event-key replay protection.

### 6.2 Strict ordering

The nonce enforces per-route or per-guardian-set ordering.

This is more complex and can create liveness issues if messages arrive out of order.

### 6.3 Signed entropy only

The nonce only prevents accidental hash collision / duplicate signed messages.

This is weaker and should not be relied on for replay protection.

### 6.4 Current recommendation

For Phase 26 draft:

~~~text
message_nonce = uniqueness-only signed message identifier
canonical_event_key = primary replay identity
~~~

Strict ordering is not selected in this draft.

Strict ordering may be reconsidered only if a clear runtime need appears.

## 7. Expiration Semantics

Phase 25 model accepted exact expiration boundary:

~~~text
currentTimeOrSlot == expiration_slot_or_unix_ts => valid
currentTimeOrSlot > expiration_slot_or_unix_ts => expired
~~~

Runtime must either adopt this inclusive boundary or explicitly revise the spec before implementation.

### 7.1 Slot-based vs timestamp-based

Current recommendation:

~~~text
Use slot-based expiration for runtime execution.
~~~

Reason:

- runtime has direct access to slot
- avoids wall-clock ambiguity
- better for deterministic on-chain checks

### 7.2 Max validity window

Runtime should define a maximum validity window.

Example:

~~~text
expiration_slot <= observed_finality_slot + max_validity_slots
~~~

This prevents guardians from signing extremely long-lived payloads.

## 8. Amount Limits / Rate Limits

Phase 25 verifies `burned_amount > 0` and `xxxl_mint_amount > 0` through Phase 23 validation and checks signed route weight binding.

It does not yet define amount caps.

Runtime handoff spec must define:

~~~text
per-route max amount
per-time-window max amount
optional global max amount
optional emergency lowered cap
~~~

### 8.1 Route-level caps

Each route should have a maximum authorized mint amount per message.

### 8.2 Window caps

Each route may have a rolling or epoch-based cap.

Open design point:

Window accounting requires additional state and must be designed before implementation.

### 8.3 Guardian match

Runtime must verify that the signed amount matches the amount being executed.

The execution amount must not be independently supplied by the caller.

## 9. Target Mint Legitimacy

Phase 25 compares `target_mint` against expected binding.

Future runtime must also verify mint legitimacy.

Required checks:

~~~text
target_mint matches route config
target_mint is the expected XXXL/wrapped asset mint
mint authority model is valid
mint decimals/supply policy match route assumptions
~~~

This is not the same as merely checking that payload and instruction agree.

Runtime config must be the source of truth.

## 10. Parity Matrix: TS Check -> Future SVM Check

| Check | TS layer status | Future runtime requirement | Source of truth |
| --- | --- | --- | --- |
| Payload field order | Phase 22/23 fixed | Must match exactly | Canonical spec |
| Byte encoding | Phase 23 implemented | Must match exactly | Canonical spec |
| Hash domain | Phase 23 implemented | Must match exactly | Canonical spec |
| Ed25519 signature | Phase 24 implemented | Must verify independently | Canonical spec |
| Guardian quorum | Phase 24 implemented | Must verify independently | Runtime guardian set state |
| Guardian set active | Not implemented | Must implement | Guardian set lifecycle spec |
| Replay check | Phase 25 model | Must implement atomically | Runtime replay storage |
| Expiration | Phase 25 model | Must implement with selected semantics | Expiration spec |
| Route binding | Phase 25 model | Must implement against runtime config | Runtime route config |
| Source chain weight | Phase 25 model | Must implement against runtime config | Runtime route config |
| Amount limits | Not implemented | Must implement before production | Runtime route/global cap config |
| Target mint legitimacy | Partial model | Must implement against runtime mint config | Runtime route config |
| Processed marking | Not implemented | Must be atomic with execution | Runtime replay storage |
| SPL CPI mint | Not implemented | Future gated phase only | Runtime execution spec |

## 11. Failure Behavior Taxonomy

Failure behavior must be specified before implementation.

### 11.1 Failure before execution

Examples:

- invalid payload
- invalid signatures
- quorum not reached
- already processed
- expired
- binding mismatch
- amount cap exceeded
- invalid target mint

Expected behavior:

~~~text
reject
do not mark processed
do not mint
do not mutate runtime state
~~~

### 11.2 Failure during execution planning

Expected behavior:

~~~text
reject
do not mark processed
do not mint
~~~

### 11.3 Failure after future mint attempt

This must be avoided by atomic transaction design.

If SPL CPI mint can fail after replay marking, the design is unsafe.

Required invariant:

~~~text
processed marking and mint execution must be atomic
~~~

### 11.4 Retry policy

A failed verification should be retryable only after the cause is corrected.

A successfully processed event must never be retry-minted.

## 12. Off-Chain Only Data

The runtime must not consume the off-chain authorization result as authority.

Off-chain-only fields may exist for:

- UI diagnostics
- watcher logs
- audit reports
- preflight summaries
- human-readable errors

But runtime authority must come from independently verified payload/proof data.

## 13. Phase Boundaries

### Phase 25

Completed.

Decision-only TypeScript authorization boundary.

### Phase 26

Current phase.

Spec-only authorization-runtime handoff.

No code implementation.

### Phase 27+

Allowed only after Phase 26 spec review.

Potential Phase 27 candidates:

~~~text
runtime handoff account mapping
guardian set lifecycle model
replay storage design model
TS/SVM parity vector suite
~~~

No live execution until a separate reviewed boundary.

## 14. Review Checklist Before Phase 27

Before Phase 27, answer:

1. Is the handoff object canonical?
2. Can runtime independently verify all critical fields?
3. Is guardian set lifecycle fully specified?
4. Is replay storage on-chain/hybrid/off-chain decision finalized?
5. Is replay check + processed marking atomic relative to execution?
6. Are nonce semantics finalized?
7. Are expiration semantics finalized?
8. Are amount/rate limits defined?
9. Is target mint legitimacy defined against runtime config?
10. Is the TS/SVM parity matrix complete?
11. Are failure modes specified?
12. Is the spec reviewed after a cooling-off period?
13. Is there a clear list of what Phase 27 may and may not implement?

## Appendix A — Draft Decisions To Review

These are not production decisions yet. They are Phase 26 draft defaults that require review.

### A.1 Nonce

Draft decision:

~~~text
message_nonce = uniqueness-only signed message identifier
canonical_event_key = primary replay identity
~~~

Strict ordering is not selected.

### A.2 Expiration

Draft decision:

~~~text
runtime expiration should be slot-based
boundary is inclusive:
current_slot == expiration_slot => valid
current_slot > expiration_slot => expired
~~~

### A.3 Replay storage

Draft decision:

~~~text
runtime execution requires on-chain ProcessedEvent storage
off-chain DB may exist only as index/preflight support
~~~

### A.4 Amount caps

Draft decision:

~~~text
minimum required before runtime execution:
per-route max amount per message
~~~

Open later:

~~~text
rolling window cap
global cap
emergency lowered cap
~~~

### A.5 Emergency halt

Draft decision:

~~~text
emergency halt must be explicit runtime config state
halted route cannot execute new mints
halt does not retroactively invalidate already processed events
~~~

Open design question:

~~~text
Who can trigger halt while preserving no-admin / limited-authority principles?
~~~

## Appendix B — Non-Goals

Phase 26 does not:

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

## Appendix C — Checkpoint Statement

Checkpoint name:

~~~text
Off-Chain Gateway Authorization Stack Complete
~~~

Next checkpoint target:

~~~text
Authorization-Runtime Handoff Spec Reviewed
~~~

The current safe state is:

~~~text
off-chain authorization stack complete
runtime execution disabled
handoff spec draft in review
~~~
