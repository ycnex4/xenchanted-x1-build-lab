# Phase 41K.6 B1 Closure — Hostile Review Package

Status: review package
Base merge: fce6ddf Merge phase 41K.6 B1C handler integration

## Purpose

This document closes the B1 blocker review.

B1 blocker statement:

The live ConsumeGatewayMint mark + mint path must be unreachable unless guardian quorum authorization is proven from real prior Ed25519 precompile evidence.

Phase 41K.6 B1C now implements that authorization pipeline end-to-end, feature-gated, with default builds closed.

## Current conclusion

B1 is resolved at implementation level.

The gated handler path now requires:

1. authoritative guardian set loading
2. real instructions sysvar evidence
3. prior Ed25519 precompile parsing
4. payload hash binding to the current mint operation
5. guardian membership validation
6. unique guardian quorum counting
7. authorization before mark + mint

Default builds remain closed.

## Architecture trace

### B1A — V3 account contract

Adds the V3 ConsumeGatewayMint account contract with instructions_sysvar.

Key invariants:

- the handler path that uses guardian authorization has access to the real instructions sysvar account
- account order and mutability requirements are explicit
- default legacy account contract remains separate

### B1B — Guardian set loading bridge

Loads the authoritative guardian set from the program-controlled PDA.

Key invariants:

- no caller-supplied guardian set
- account key checked
- account owner checked
- PDA checked
- active status checked
- threshold checked
- guardian count checked
- duplicate guardian keys rejected
- loading remains read-only

Current guardian set layout has active status but no expiry slot. If a future layout adds active_until_slot, B1C.7 must enforce current_slot <= active_until_slot.

### B1C.1 — Authorization result types

Defines isolated authorization result types.

Key invariant:

- authorization result modeling does not mutate state

### B1C.2 — Instructions sysvar evidence bridge

Loads prior Ed25519 evidence from the real instructions sysvar boundary.

Key invariants:

- uses real instructions sysvar account info
- checks instructions sysvar account id
- loads current instruction index through checked runtime API
- loads only prior instructions
- filters Ed25519 precompile instructions
- accepts no frontend/watcher proof as authorization

### B1C.3 — Ed25519 evidence parser

Parses Ed25519 precompile instruction data.

Key invariants:

- parser checks header/offset layout
- extracts signer public key
- extracts signed message
- does not perform authorization alone
- does not mutate state

### B1C.3-connect — Adapter

Connects checked prior instruction loading to B1C.3 parser.

Key invariants:

- consumes runtime-loaded prior instructions
- rejects malformed evidence
- preserves source instruction index
- remains non-authorizing alone

### B1C.4 — Payload hash binding

Computes expected payload hash locally and binds parsed Ed25519 signed messages to it.

Payload context includes:

- processed event account
- mint
- recipient token account
- amount
- guardian set numeric id

Key invariants:

- local hash computation
- no caller-provided hash accepted
- current slot intentionally excluded
- mismatched signed message rejects before authorization
- no mutation

### B1C.5 — Guardian membership validation

Validates that payload-bound signers are members of the authoritative B1B guardian set.

Key invariants:

- consumes B1B-loaded guardian set
- rejects unauthorized signer
- preserves duplicate signers for later quorum deduplication
- no mutation

### B1C.6 — Quorum counting

Counts unique validated guardians and compares them to the B1B threshold.

Key invariants:

- duplicate signer counts once
- threshold comes from authoritative guardian set
- quorum met means sufficient unique guardians signed
- authorization remains disabled in B1C.6 alone

### B1C.7 — Handler authorization boundary

Composes the full B1C pipeline.

Pipeline:

    B1B guardian set
    -> B1C.2 instructions_sysvar evidence
    -> B1C.3/connect parser
    -> B1C.4 payload hash binding
    -> B1C.5 guardian membership
    -> B1C.6 quorum counting
    -> authorization_enabled = true

Key invariants:

- fail-fast before mutation
- no mark
- no mint
- no live route by boundary alone

### B1C.7 final — Mark + mint wiring

Wires B1C.7 authorization before the actual mark + mint boundary.

Key invariants:

- B1C.7 authorization is called before mark + mint
- rejected authorization exits before mutation
- mark + mint only runs after authorization_enabled = true
- CPI gate is checked before mutation
- default/non-B1C7 path remains unchanged
- B1C7-gated handler path uses V3 account contract with instructions_sysvar

## Safety invariants to review

### Authorization-before-mutation

There must be no path where processed_event is marked before full B1C authorization succeeds.

### Mint-before-authorization

There must be no path where SPL mint CPI is attempted before full B1C authorization succeeds.

### Caller-provided evidence

There must be no path where frontend, watcher, or caller-supplied proof bytes are accepted as authorization evidence.

Only real prior Ed25519 precompile instructions loaded from instructions sysvar may authorize.

### Payload binding

A valid guardian signature for another operation must not authorize this operation.

Binding must include operation-specific values and reject payload mismatch.

### Guardian authority

Guardian membership and threshold must come from the authoritative B1B guardian set.

### Duplicate signer

The same guardian signing multiple Ed25519 precompile instructions must count once.

### Replay protection

Replay protection remains check-before-mark.

Mark + mint remains one execution path after authorization.

### Feature gate

Default builds must remain closed.

B1C7 integration must require explicit dangerous test allow feature when feature-enabled.

Earlier compile_error guards must not be removed or weakened.

### CPI gate

Even after authorization, CPI gate must be checked before mutation.

If CPI gate is closed, there must be no processed_event mark.

## Validation already run

Latest accepted validation:

    default lib passed
    B1C7 gated lib passed
    closed-gate Mollusk passed
    zero regression

## Hostile review questions

1. Can any mutable account change happen before B1C.7 authorization succeeds?
2. Can processed_event be marked if authorization fails?
3. Can SPL mint CPI run if authorization fails?
4. Can a caller provide fake evidence bytes instead of real prior Ed25519 precompile instructions?
5. Can a valid signature for another operation authorize this operation?
6. Can an unauthorized signer pass membership validation?
7. Can one guardian sign multiple times and satisfy a threshold greater than 1?
8. Can a non-authoritative guardian set define threshold/signers?
9. Can default builds accidentally open the B1C7 path?
10. Can CPI gate failure happen after processed_event mark?
11. Does replay protection still reject already-consumed events?
12. Are B1C slices still isolated enough to audit individually?

## Known non-goals

B1 closure does not include:

- production guardian keys
- off-chain signer infrastructure
- guardian operations runbook
- production deployment
- production feature enablement
- guardian rotation governance
- future guardian expiry field

These are later operational/deployment topics.

## Review request

Theo / Claude:

Please hostile-review the B1 closure state.

Review target:

    main @ fce6ddf
    Phase 41K.6 B1C complete
    B1C.7 mark+mint wiring merged

Question:

Does this fully resolve the B1 blocker, or is there any remaining authorization bypass / mutation-before-authorization path?
