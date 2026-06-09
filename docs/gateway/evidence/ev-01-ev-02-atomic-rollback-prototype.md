# EV-01 / EV-02 Atomic Rollback Prototype Evidence Plan

## Purpose

This document defines the first prototype-only evidence step for the future X1-side gateway.

Covered evidence IDs:

- EV-01 transaction-level atomicity
- EV-02 account write rollback

This is not gateway implementation.

This is not bridge implementation.

This is not production runtime code.

This is not a minting path.

This is only an evidence plan for testing whether runtime state writes roll back when an instruction or transaction fails.

## Branch

- prototype-x1-evidence-atomic-rollback

## Source planning checkpoints

This evidence plan follows:

- Stage 2.6 X1 runtime evidence collection plan
- Stage 2.7 prototype-only experiment boundaries
- Stage 2.8 gateway planning baseline checkpoint

## What is being tested

The intended experiment should test whether a runtime instruction or local model can:

1. write to one or more state accounts
2. intentionally fail after the write
3. prove that the write does not persist after failure

If multiple writes are possible, the experiment should also test whether all writes roll back together.

## What is not being tested

This experiment does not test:

- gateway message verification
- guardian signatures
- canonicalEventKey derivation
- processed burn registry design
- token minting
- CPI/token mint rollback
- claim creation
- claim redemption
- real cross-chain events
- live X1 deployment
- frontend bridge UX
- production mint authority

Those belong to later EV-* evidence items.

## Required result

The minimum useful result is:

- failed transaction leaves no partial account writes
- failed instruction leaves no partial account writes
- repeated failure remains deterministic
- post-failure inspection proves state was not mutated

If rollback is not guaranteed, both direct mint and claim-based runtime implementation remain blocked.

## Pass condition

The evidence passes only if a repeatable prototype shows:

- initial state is known
- state is modified during the attempted operation
- the operation intentionally fails
- final state equals the initial state
- no partial state persists

## Fail condition

The evidence fails if:

- any partial write persists after failure
- different accounts roll back inconsistently
- rollback behavior depends on unclear runtime behavior
- the result cannot be repeated
- the test requires manual/admin repair

## Safety boundaries

This prototype must not:

- deploy production code
- use production mint authority
- mint live tokens
- process real source events
- expose a frontend user flow
- print secrets
- print private keys
- print RPC API keys
- rely on admin recovery

## Expected documentation after experiment

After the experiment is performed, this document should be updated with:

- environment
- commands used
- observed result
- pass/fail status
- limitations
- affected risks
- affected architecture candidates
- remaining uncertainty
- conclusion


## Local reference model

A local TypeScript reference model was added in a later branch:

- src/prototypes/atomic-rollback-reference.ts
- tests/prototypes/atomic-rollback-reference.test.ts

Purpose:

- define expected atomic rollback semantics
- prove that the local reference model rolls back all writes on failure
- prove that successful operations persist all writes
- explicitly keep EV-01 and EV-02 open

This is not X1 runtime evidence.

This does not prove X1 transaction atomicity.

This does not prove X1 account write rollback.

The next required step is to repeat the same rollback scenario in the X1 testnet/runtime environment.

## Current status

Status: planned.

No runtime evidence has been collected yet.

Direct mint and claim-based runtime implementation remain blocked until EV-01 and EV-02 have evidence-backed results.
