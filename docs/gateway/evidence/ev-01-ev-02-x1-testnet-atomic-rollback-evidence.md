# EV-01 / EV-02 X1 Testnet Atomic Rollback Evidence

## Purpose

This document tracks the first real X1 testnet/runtime evidence experiment for the future X1-side gateway.

Covered evidence IDs:

- EV-01 transaction-level atomicity
- EV-02 account write rollback

This experiment follows the local reference model:

- src/prototypes/atomic-rollback-reference.ts
- tests/prototypes/atomic-rollback-reference.test.ts

The local reference model defines expected behavior.

This X1 testnet experiment must check whether the actual X1 runtime behaves the same way.

## Branch

- prototype-x1-testnet-atomic-rollback-evidence

## Scope

The testnet experiment should verify the minimal rollback scenario:

1. create or use a test-only state account
2. read and record initial state
3. perform one or more state writes
4. intentionally fail after the writes
5. read final state
6. prove final state equals initial state

If possible, the experiment should also verify multi-account rollback:

1. write account A
2. write account B
3. intentionally fail
4. prove both A and B remain unchanged

## Out of scope

This experiment must not include:

- production gateway implementation
- production bridge implementation
- XXXL mint authority
- live token minting
- guardian signature verification
- canonicalEventKey production handling
- processed burn registry production handling
- source chain event processing
- frontend bridge UX
- relayer production logic
- admin recovery assumptions

EV-03 token mint/CPI rollback is out of scope for this branch.

## Required safety boundaries

The experiment must be testnet-only.

The experiment must not print:

- private keys
- seed phrases
- RPC API keys
- wallet secrets
- access tokens
- raw environment values

Commands and logs should show only non-sensitive status, file names, public addresses, transaction signatures, and redacted configuration.

## Evidence fields to collect

When the experiment is performed, record:

- X1 network name
- toolchain used
- test account/program identifiers
- command used, with secrets redacted
- initial state
- attempted mutation
- intentional failure reason
- final state
- transaction signature or test output if available
- pass/fail result
- limitations
- remaining uncertainty

## Pass condition

EV-01 / EV-02 can be considered evidence-supported only if X1 testnet shows:

- failed transaction leaves no partial writes
- failed instruction leaves no partial writes
- multi-account write failure rolls back all tested accounts
- result is repeatable
- no manual/admin repair is required

## Fail condition

The experiment fails if:

- any partial state persists after failure
- one account rolls back while another does not
- rollback depends on manual cleanup
- behavior is inconsistent between runs
- the runtime behavior is unclear or unobservable

## Current status

Status: planned.

No X1 testnet runtime evidence has been collected in this branch yet.

EV-01 and EV-02 remain open.
