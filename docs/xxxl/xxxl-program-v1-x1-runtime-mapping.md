# XXXL Program v1 X1 Runtime Mapping

## Purpose

This document maps the XXXL Program v1 deterministic model to a future X1 runtime design.

This is not production runtime code.

The goal is to preserve the already-tested model invariants at the account / instruction / state-transition level.

## Current model layers

The current TypeScript model has three relevant layers:

1. XXXL Program v1 boundary
2. Stage 1 gateway authorization consumer
3. Genesis supply invariant hardening

Together they define:

- XXXL starts gateway-only
- no manual mint
- no premine
- no founder allocation
- no hidden emission path
- successful Stage 1 authorization is required before XXXL mint
- each canonical event key is consumed exactly once
- supply increases only by the authorized gateway mint amount
- rejected transitions must not mutate supply or replay state
- Build is not required for XNTD transfer to X1
- Build is not a supply source

## Runtime state objects

A future X1 runtime should separate these state objects.

### XXXL mint state

Stores canonical token-level state.

Required fields:

- mint id / token id
- decimals
- total supply
- mint authority mode
- upgrade authority status
- genesis phase status

Genesis Phase mint authority mode:

    gateway-only

Not allowed:

- arbitrary admin mint
- founder allocation mint
- premine mint
- hidden emission mint
- Build-derived mint
- current-balance-derived mint

### Gateway configuration state

Stores route-level authorization configuration.

Required fields:

- route id
- source chain id
- source token
- target mint token
- target X1 network id
- target mint core id
- guardian set id
- quorum threshold
- finality rule reference
- active / frozen status

The route configuration must bind the gateway to the canonical Ethereum XNTD to X1 XXXL path.

### Guardian set state

Stores the active signer set used by Stage 1 authorization.

Required fields:

- guardian set id
- guardian public keys
- quorum threshold
- activation timestamp / slot
- optional retirement timestamp / slot
- rotation status

Guardian set rotation is separate from arbitrary mint authority.

Rotation must not create supply rights.

### Processed event state

Stores consumed canonical event keys.

Required fields:

- canonical event key
- route id
- source chain id
- source transaction hash
- source event index
- consumed by instruction id / slot
- consumed amount
- recipient

This state is the runtime version of replay protection.

### Recipient balance state

Stores holder balances for XXXL.

Required fields:

- owner / recipient
- balance
- token mint id

Balance state must be updated only by authorized runtime instructions.

## Runtime instruction: consume gateway mint

Canonical instruction:

    consume_gateway_mint

Inputs:

- gateway mint message fields
- X1 recipient bytes
- domain separator
- message hash
- guardian approvals
- route configuration account
- guardian set account
- processed event account
- XXXL mint state account
- recipient balance account

Required checks:

1. message fields are structurally valid
2. route id matches canonical XXXL gateway route
3. source chain id matches Ethereum mainnet
4. source token matches canonical XNTD source token
5. mint token matches XXXL
6. burned amount is greater than zero
7. XXXL mint amount equals burned amount during Genesis Phase
8. canonical event key matches source chain / source token / tx hash / event index
9. X1 recipient hash matches recipient bytes
10. domain separator matches route configuration
11. message hash matches canonical encoded message
12. guardian quorum is valid
13. canonical event key has not already been consumed
14. mint authority mode is gateway-only
15. upgrade status does not bypass gateway-only rules

Required state transition:

- increase recipient balance by authorized amount
- increase total supply by authorized amount
- mark canonical event key consumed

These effects must happen atomically.

## Atomicity requirement

The runtime must preserve this all-or-nothing rule:

    if gateway mint succeeds:
        balance increases
        total supply increases
        event key is marked consumed

    if gateway mint fails:
        balance is unchanged
        total supply is unchanged
        event key is not marked consumed

No partial success is allowed.

## Account write order refinement

The runtime correctness requirement is transaction-level atomicity, not a specific low-level account write order.

On SVM-style execution, account writes are committed atomically at transaction success and rolled back on transaction failure.

Therefore, the correctness invariant is:

    success = balance update + supply update + consumed event mark
    failure = no balance update + no supply update + no consumed event mark

The implementation should still document write order for engineering review because account ordering may affect:

- compute cost
- account contention
- instruction layout
- audit readability

But write order must not be used as the core safety mechanism. Atomicity is the safety mechanism.

## Check-before-mark rule

The runtime must check replay before marking the event consumed.

Required order:

1. verify message
2. verify quorum
3. check event not consumed
4. compute authorized amount
5. apply mint state transition
6. mark event consumed in the same atomic transaction

The implementation may order low-level account writes differently only if the final transaction semantics are atomic and equivalent.

## Failure cases

The consume gateway mint instruction must fail without mutation on:

- wrong route
- wrong source chain
- wrong source token
- wrong mint token
- zero burned amount
- mint amount mismatch
- wrong canonical event key
- wrong recipient hash
- wrong domain separator
- wrong message hash
- invalid guardian signature
- quorum not reached
- consumed event replay
- inactive route
- frozen route
- non-gateway mint authority mode
- malformed recipient
- malformed message fields

## Build separation

The consume gateway mint instruction must not require:

- Build account
- Build activation
- Build identity
- Build history
- Build commitment status
- BLD state
- XBP state
- X1 fee contribution state

A future combined UI may guide a user through transfer and Build activation, but runtime responsibilities must remain separate.

## Upgrade authority boundary

Temporary upgradeability may exist only for staged protocol finalization.

Runtime rules:

- upgrade authority must not be able to mint XXXL directly
- upgrade authority must not bypass gateway authorization
- upgrade authority must not rewrite user balances
- upgrade authority must not clear processed event history
- upgrade authority must not create founder allocation
- upgrade authority must not create hidden emission

Future upgrades may only add deterministic user-action protocol mechanics.

After planned X1-native emission mechanics are complete, upgrade authority must be removed / frozen.

## Freeze readiness requirements

Before final freeze, the project must have:

- documented final X1-native emission mechanics
- deterministic model tests
- runtime mapping
- replay protection tests
- supply invariant tests
- community explanation
- deployment readiness checklist
- authority removal / freeze procedure

## Out of scope

This mapping does not implement:

- production X1 program code
- deployment scripts
- live accounts
- live guardian keys
- xDex listing
- frontend integration
- RPC scripts
- secret management
