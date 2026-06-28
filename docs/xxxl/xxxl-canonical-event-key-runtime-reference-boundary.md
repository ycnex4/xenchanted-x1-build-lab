# XXXL Canonical Event Key Runtime Reference Boundary

Status: COMPLETED.

This document records the XXXL SVM runtime reference for the existing Stage 1 `canonicalEventKey` policy.

It is a documentation-only boundary.

No runtime code is changed by this stage.

## Purpose

The goal is to make the existing Stage 1 canonical event key policy directly visible to future XXXL SVM runtime reviewers.

This document does not invent a new key derivation rule.

It extracts the already-established Stage 1 policy into a short runtime-facing reference before any future implementation work introduces account writes, processed-burn registry mutation, SPL CPI execution, `invoke_signed`, or minting.

## Current runtime state

The current runtime remains:

- scaffold-only
- locked
- unreleasable
- not deployable

Current release decision:

- release allowed: `false`
- release blocked: `true`
- primary blocker code: `RUNTIME_SAFETY_LOCK_ACTIVE`

## Canonical event key definition

The canonical event key is the replay anchor for an Ethereum XNTD burn event consumed by the XXXL gateway.

The established Stage 1 direction is:

- `canonicalEventKey = keccak256(canonicalEventKeyPreimage)`

The canonical event key preimage is the ordered concatenation of:

1. `sourceChainId`
2. `sourceToken`
3. `sourceBurnTxHash`
4. `sourceBurnEventIndex`

The compact notation is:

- `canonicalEventKey = keccak256(sourceChainId || sourceToken || sourceBurnTxHash || sourceBurnEventIndex)`

The current Stage 1 exact cryptographic vectors document the canonical event key preimage length as 128 bytes.

Future runtime implementation must use the locked Stage 1 canonical encoding and test vectors for exact byte layout.

## Field roles

`sourceChainId` identifies the source chain domain.

`sourceToken` identifies the source token contract within that chain domain.

`sourceBurnTxHash` identifies the source transaction.

`sourceBurnEventIndex` distinguishes the specific burn event inside the source transaction.

`sourceBurnTxHash` alone is not sufficient because one transaction may emit multiple relevant events.

`sourceBurnEventIndex` must not be omitted, ignored, normalized differently, or replaced with a relayer-local index.

The exact byte-level encoding, width, and normalization of `sourceBurnEventIndex` must be taken from the Stage 1 exact cryptographic vectors and canonical encoding documents.

Future implementation must not infer a new local type, byte order, or width for this field.

## Replay-protection rule

The processed-burn registry key must be exactly `canonicalEventKey`.

One `canonicalEventKey` can produce at most one successful XXXL mint.

A repeated submission with the same `canonicalEventKey` must not mint again.

The replay rule must hold even if:

- a different relayer submits the same source event
- guardian approvals are reordered
- duplicate guardian approvals are included
- non-key message fields are re-presented in a different transport context
- the same source event is routed through a later runtime path

## Runtime verification requirement

A future runtime implementation must recompute `canonicalEventKey` from the source event identity fields and compare it to the message field before any state mutation.

The runtime must reject the message without state changes if:

- `sourceChainId` does not match the expected source domain
- `sourceToken` does not match the expected source token
- `sourceBurnTxHash` is malformed or does not match the signed message
- `sourceBurnEventIndex` is malformed or does not match the signed message
- computed `canonicalEventKey` does not match the provided `canonicalEventKey`
- the computed `canonicalEventKey` is already processed

The runtime must not accept caller-provided `canonicalEventKey` as trusted without recomputation and comparison.

The recompute-and-compare step must happen before the processed registry replay check.

A replay check must only be performed against a canonical event key that has already been recomputed from the source event identity fields and matched against the message field.

## Relationship to recipient and amount binding

`canonicalEventKey` identifies the source burn event.

It is not, by itself, sufficient authorization to mint.

Recipient binding, amount binding, route validation, finality checks, guardian quorum verification, mint account validation, mint authority validation, token program validation, and replay protection must all pass before mint execution.

The message hash and guardian approvals must bind the canonical event key together with the recipient, amount, route, source, mint token, and finality fields.

## Atomic mutation requirement

The future runtime must preserve the Stage 1 rule:

- verify the canonical event key
- check that it is unprocessed
- execute the corresponding mint
- mark the same canonical event key as processed
- avoid partial success paths

The runtime must not allow a successful mint without marking the corresponding `canonicalEventKey` processed.

The runtime must not mark a `canonicalEventKey` processed without the corresponding successful mint.

## Immutability requirement

The meaning of `canonicalEventKey` must not silently change after processed-burn registry entries exist.

Any future change to canonical event key derivation, field order, byte encoding, source domain interpretation, or fork-disambiguation policy requires an explicit reviewed migration or invalidation rule.

A runtime upgrade, route version, guardian set version, or relayer version must not bypass existing processed-burn registry entries for the same source event.

## Review requirements before implementation

Before implementation work begins, reviewers must confirm:

- exact Stage 1 canonical encoding reference
- exact byte order
- exact field order
- exact preimage length expectation
- exact hash function
- source chain id normalization
- source token normalization
- source burn tx hash normalization
- source burn event index encoding from the Stage 1 exact cryptographic vectors and canonical encoding documents
- recompute-and-compare behavior before replay check
- processed registry key derivation
- replay rejection behavior
- no-state-change-on-failure tests
- test vector coverage for wrong source burn tx hash
- test vector coverage for wrong source burn event index
- test vector coverage for wrong canonical event key
- test vector coverage for duplicate canonical event key

## Non-goals

This document does not implement runtime mutation.

This document does not enable runtime account writes.

This document does not enable live route execution.

This document does not enable SPL CPI execution.

This document does not enable `invoke_signed`.

This document does not enable SPL Token `mint_to`.

This document does not enable XXXL minting.

This document does not select a real Program ID.

This document does not regenerate production PDA fixtures.

This document does not remove deployment blockers.

This document does not change deployability predicates.

## Decision

The XXXL canonical event key runtime reference boundary is accepted.

The current runtime remains scaffold-only, locked, unreleasable, and not deployable.
