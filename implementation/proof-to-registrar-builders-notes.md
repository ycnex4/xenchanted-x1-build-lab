# Proof to Registrar Payload Builders Notes

## Branch

proof-to-registrar-builders

## Purpose

This branch adds proof-to-registrar payload builders for the xEnchanted X1 Build Lab.

The builders connect the proof object layer to the registrar instruction layer.

They do not apply state transitions directly.

## Implemented files

- src/proofs/registrar-builders.ts
- tests/proof-registrar-builders.test.ts

Updated:

- src/index.ts

## Implemented builder input

The branch adds CreateRegistrarPayloadInput.

It includes:

- submittedBy
- createdAt
- optional messageId

If messageId is not provided, the builder derives a deterministic message id from:

- proof kind
- canonical event key

## Implemented payload types

The branch adds registrar payload types for:

- CoreRedeemRegistrarPayload
- XenBurnRegistrarPayload
- XntdLockRegistrarPayload
- XntdRelockRegistrarPayload
- X1FeeCheckpointRegistrarPayload

These are grouped under:

- RegistrarPayloadFromProof

## Implemented builder helpers

The branch adds:

- buildCoreRedeemRegistrarPayload
- buildXenBurnRegistrarPayload
- buildXntdLockRegistrarPayload
- buildXntdRelockRegistrarPayload
- buildX1FeeCheckpointRegistrarPayload
- buildRegistrarPayloadFromProof

## Validation policy

Builders require validated proofs.

Non-validated proofs are rejected through assertValidatedProof before payload construction.

## Genesis Origin policy

Genesis Origin eligibility proof does not map to a registrar payload.

The generic builder rejects Genesis Origin proof with an explicit error.

This preserves the current model where Genesis Origin claim is not a registrar message flow.

## State mutation policy

The builders do not mutate BuildState.

They only convert validated proof objects into registrar payload data that can later be submitted through application service helpers.

## Test coverage

Added test file:

- tests/proof-registrar-builders.test.ts

Covered cases:

- Core redeem proof to registrar payload
- XEN burn proof to registrar payload
- XNTD lock proof to registrar payload
- XNTD relock proof to registrar payload
- X1 fee checkpoint proof to registrar payload
- custom messageId support
- default deterministic messageId support
- non-validated proof rejection
- Genesis Origin proof rejection
- generic proof payload builder routing

## Validation

Before commit:

- npm run typecheck: passed
- npm test: passed
- 24 test files passed
- 134 tests passed

## Current known exclusions

This milestone does not implement:

- proof validation
- watcher-to-proof conversion
- automatic registrar submission
- Build lookup by buildId
- application service proof submission
- CLI proof submission
- persisted proof store

## Main invariant

Proof-to-registrar builders translate validated facts into registrar payloads.

They must not apply protocol accounting directly.
