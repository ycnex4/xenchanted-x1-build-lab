# Proof Object Types Notes

## Branch

proof-object-types

## Purpose

This branch starts the proof object implementation layer for the xEnchanted X1 Build Lab.

The first implementation step adds shared proof object types and canonical event key helpers.

This milestone does not add real on-chain proof validation yet.

## Implemented files

- src/proofs/proof-types.ts
- tests/proof-types.test.ts

Updated:

- src/index.ts

## Implemented proof categories

The branch defines proof object types for:

- CoreRedeemProof
- XenBurnProof
- XntdLockProof
- XntdRelockProof
- X1FeeCheckpointProof
- GenesisOriginEligibilityProof

These are grouped under:

- BuildProof

## Implemented proof status model

The branch defines ProofValidationStatus:

- CANDIDATE
- VALIDATED
- REJECTED

This separates observed proof candidates from validated proof objects.

## Implemented source metadata

The branch defines ProofSourceMetadata with:

- sourceChainId
- sourceAddress
- transactionHash
- eventIndex
- blockNumber
- slot
- observedAt
- finalized

This prepares future source-specific proof validation.

## Implemented canonical event key helper

The branch adds createCanonicalEventKey.

The key is deterministic and uses:

- sourceChainId
- sourceAddress
- eventKind
- transactionHash
- eventIndex

The helper rejects:

- empty string parts
- string parts containing ":"
- negative event indexes
- non-integer event indexes

## Implemented helper functions

The branch adds:

- createProofSourceMetadata
- isValidatedProof
- assertValidatedProof

## Test coverage

Added test file:

- tests/proof-types.test.ts

Covered cases:

- deterministic canonical event key creation
- invalid canonical event key input rejection
- proof source metadata defaults
- validated Core redeem proof object
- non-validated proof assertion rejection

## Validation

Before commit:

- npm run typecheck: passed
- npm test: passed
- 20 test files passed
- 112 tests passed

## Current known exclusions

This milestone does not implement:

- on-chain proof validation
- ABI decoding
- log parsing
- Merkle proofs
- signature checks
- finality validation
- watcher candidate types
- registrar payload builders from proofs
- proof persistence
- API / CLI proof submission

## Main invariant

Proof objects describe validated or candidate external facts.

They must not mutate BuildState directly.
