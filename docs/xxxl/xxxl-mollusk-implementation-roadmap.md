# XXXL Mollusk Implementation Roadmap

## 1. Purpose

This document defines a conservative implementation roadmap for future
Mollusk/SVM runtime coverage.

This is a planning boundary only.

It does not implement Mollusk tests.

It does not add dependencies.

It does not change Rust runtime code.

It does not remove `MOLLUSK_COVERAGE_INCOMPLETE`.

It does not remove any deployment blocker.

It does not enable live route execution, SPL CPI execution, `invoke_signed`, or
SPL Token `mint_to`.

## 2. Current Baseline

Current baseline:

- `ACCOUNT_CONTRACT_UNREVIEWED` has already been transitioned after review closure.
- `MOLLUSK_COVERAGE_INCOMPLETE` remains active.
- Remaining active blockers are inventoried.
- Rust unit tests currently pass for the locked scaffold.
- Existing Rust unit tests are useful but are not equivalent to Mollusk/SVM runtime coverage.
- Runtime remains scaffold-only.
- Runtime remains locked.
- Runtime remains unreleasable.
- Runtime remains not deployable.
- Live route execution remains disabled.
- SPL CPI execution remains disabled.
- `invoke_signed` remains unreachable from live route execution.
- SPL Token `mint_to` remains disabled from live route execution.

## 3. Design Principle

Future Mollusk coverage must be implemented in small, independently reviewable
stages.

No stage should combine test harness setup, runtime behavior changes, blocker
transition, live route activation, SPL CPI activation, guardian policy, Program
ID transition, and external review.

Each stage should answer one narrow question and preserve the locked scaffold
unless the stage is explicitly reviewed as a transition boundary.

## 4. Non-Negotiable Safety Rules

The following rules apply to every future Mollusk-related implementation stage:

- do not remove deployment blockers as a side effect
- do not enable live route execution as a side effect
- do not enable SPL CPI execution as a side effect
- do not enable `invoke_signed` as a side effect
- do not enable SPL Token `mint_to` as a side effect
- do not change Program ID as a side effect
- do not regenerate production PDA fixtures as a side effect
- do not introduce production guardian configuration as a side effect
- do not introduce production proof-log configuration as a side effect
- do not claim deployment readiness from test coverage alone

## 5. Stage 1: Mollusk Harness Boundary

### Goal

Introduce the minimal Mollusk/SVM test harness required to run runtime-like tests
against the locked scaffold.

### Allowed Scope

- add Mollusk test dependency/config if required
- add test harness files
- add minimal fixture construction helpers
- document how the harness maps to the existing locked scaffold

### Required Evidence

- harness compiles
- existing Rust tests still pass
- at least one harmless sanity test proves the harness can execute a rejected or
  no-op locked-scaffold path
- no runtime source file is changed unless strictly required and separately
  explained
- no blocker is removed

### Must Not Do

- no live route activation
- no SPL CPI activation
- no `invoke_signed`
- no SPL Token `mint_to`
- no blocker transition

### Exit Criteria

The project has a working Mollusk harness, but `MOLLUSK_COVERAGE_INCOMPLETE`
remains active.

## 6. Stage 2: Account Meta and Ordering Coverage

### Goal

Add Mollusk/SVM tests for account count, account ordering, writability, signer
expectations, and account aliasing risks.

### Required Coverage

- correct account count accepted up to the current locked boundary
- wrong account count rejected
- wrong account order rejected
- missing writable account rejected
- unexpected writable readonly account rejected
- unexpected signer rejected
- duplicate account keys / aliasing attempts rejected or explicitly documented
  as a future required fix if current runtime does not yet enforce them

### Must Not Do

- no SPL CPI execution
- no `invoke_signed`
- no SPL Token `mint_to`
- no blocker transition

### Exit Criteria

Account meta/order coverage exists, but `MOLLUSK_COVERAGE_INCOMPLETE` remains
active.

## 7. Stage 3: Program-Owned Account Validation Coverage

### Goal

Add Mollusk/SVM tests for program-owned account layout and binding validation.

### Required Coverage

- wrong owner rejected for program-owned accounts
- wrong discriminator rejected
- wrong version rejected
- truncated data rejected
- route id mismatch rejected
- mint id mismatch rejected
- recipient owner mismatch rejected
- canonical event key mismatch rejected
- guardian set id mismatch rejected

### Production Requirement Reminder

Production PDA derivation for program-owned accounts remains a future
requirement and is not completed merely by this coverage.

### Must Not Do

- no Program ID transition
- no production PDA fixture regeneration
- no blocker transition

### Exit Criteria

Program-owned validation coverage exists, but `MOLLUSK_COVERAGE_INCOMPLETE`
remains active.

## 8. Stage 4: SPL Token Mint and Recipient Account Coverage

### Goal

Add Mollusk/SVM tests for SPL Token mint and recipient token account validation
while SPL CPI remains disabled.

### Required Coverage

- wrong SPL Token program rejected
- wrong SPL mint owner rejected
- wrong mint authority rejected
- wrong recipient token mint rejected
- wrong recipient token owner rejected
- uninitialized recipient token account rejected
- uninitialized mint rejected
- close/reinitialize race risk documented or covered if applicable

### Must Not Do

- no SPL CPI execution
- no `invoke_signed`
- no SPL Token `mint_to`
- no blocker transition

### Exit Criteria

SPL Token validation coverage exists, but SPL CPI execution remains disabled and
`MOLLUSK_COVERAGE_INCOMPLETE` remains active.

## 9. Stage 5: PDA Coverage

### Goal

Add Mollusk/SVM tests for PDA validation and semantic separation.

### Required Coverage

- wrong mint authority PDA rejected
- wrong bump rejected
- PDA changes with Program ID
- PDA semantic separation documented and tested where applicable
- production PDA derivation requirements remain explicit for future transition

### Must Not Do

- no Program ID transition
- no production PDA fixture transition
- no `invoke_signed`
- no SPL Token `mint_to`
- no blocker transition

### Exit Criteria

PDA coverage exists, but runtime remains locked and not deployable.

## 10. Stage 6: Disabled Execution Gate and No-Mutation Coverage

### Goal

Add Mollusk/SVM tests proving disabled gates reject before mutation.

### Required Coverage

- live route disabled rejects before mutation
- SPL CPI disabled rejects before mutation
- no `invoke_signed`
- no SPL Token `mint_to`
- rejected path does not mark processed event
- rejected path does not credit recipient balance
- rejected path does not create partial state
- wrong recipient token account rejected before mutation
- zero amount rejected before mutation

### Must Not Do

- no live route activation
- no SPL CPI activation
- no blocker transition

### Exit Criteria

Disabled gate/no-mutation coverage exists, but locked scaffold state remains
unchanged.

## 11. Stage 7: Replay and Atomicity Coverage

### Goal

Add Mollusk/SVM tests for replay rejection and atomicity guarantees at the
runtime-like boundary.

### Required Coverage

- consumed processed event rejected
- replay rejected before credit
- wrong event key rejected
- wrong route rejected
- wrong recipient rejected
- wrong mint rejected
- overflow rejected before mutation
- failed or disabled CPI path does not leave ProcessedBurnEntry
- failed or disabled CPI path does not change recipient balance

### Must Not Do

- no SPL CPI activation
- no live route activation
- no blocker transition

### Exit Criteria

Replay and atomicity coverage exists, but `MOLLUSK_COVERAGE_INCOMPLETE` remains
active until full coverage review.

## 12. Stage 8: Instruction Bytes and Reserved-Bytes Coverage

### Goal

Add Mollusk/SVM tests for instruction byte parsing and strictness.

### Required Coverage

- wrong discriminator rejected
- wrong version rejected
- wrong length rejected
- extra bytes rejected or reserved-bytes policy documented and enforced
- padding policy documented
- malformed instruction data rejected before mutation

### Must Not Do

- no instruction format expansion unless separately reviewed
- no live route activation
- no blocker transition

### Exit Criteria

Instruction byte coverage exists and reserved-byte policy is explicit.

## 13. Stage 9: Rent and Lifecycle Coverage

### Goal

Add Mollusk/SVM tests for rent and account lifecycle concerns.

### Required Coverage

- rent exemption check accepted at minimum balance
- low lamports rejected
- account close/reinitialize risk documented
- lifecycle assumptions documented for all mutable accounts
- no partial mutation on lifecycle rejection

### Must Not Do

- no SPL CPI activation
- no live route activation
- no blocker transition

### Exit Criteria

Rent/lifecycle coverage exists, but deployment remains blocked.

## 14. Stage 10: Mollusk Coverage Review Package

### Goal

Collect all Mollusk/SVM coverage evidence into a review package.

### Required Evidence

- list of test files
- list of test categories
- mapping to gap-analysis requirements
- list of still-missing coverage
- explicit statement whether coverage is sufficient for blocker transition
- explicit statement that coverage does not itself authorize deployment
- explicit statement that live route / SPL CPI / `invoke_signed` / `mint_to`
  remain disabled unless separately transitioned

### Must Not Do

- no blocker transition in the same stage
- no runtime unlock
- no deployment approval

### Exit Criteria

A review package exists and is ready for independent assessment.

## 15. Stage 11: Mollusk Coverage Assessment

### Goal

Have the coverage review package assessed before any blocker transition.

### Required Evidence

- reviewer notes
- accepted findings
- rejected findings
- unresolved findings
- recommended remediation if needed
- explicit yes/no on whether a blocker-transition stage is justified

### Must Not Do

- no blocker transition in the same stage
- no runtime unlock
- no deployment approval

### Exit Criteria

The project has review evidence for whether `MOLLUSK_COVERAGE_INCOMPLETE` can be
transitioned later.

## 16. Stage 12: Mollusk Blocker Transition

### Goal

Transition `MOLLUSK_COVERAGE_INCOMPLETE` only if coverage and review evidence
justify it.

### Required Preconditions

- Mollusk harness exists
- all required coverage areas are implemented or explicitly waived with review
- review package exists
- assessment accepts transition
- remaining blockers still block deployment
- runtime remains not deployable unless all other blockers are independently
  transitioned

### Must Not Do

- no live route activation
- no SPL CPI activation
- no `invoke_signed`
- no SPL Token `mint_to`
- no Program ID transition
- no guardian/proof-log production transition
- no external review transition unless separately completed

### Exit Criteria

Only `MOLLUSK_COVERAGE_INCOMPLETE` is transitioned, and all other blockers remain
active.

## 17. Recommended Execution Order

Recommended future order:

1. Mollusk Harness Boundary
2. Account Meta and Ordering Coverage
3. Program-Owned Account Validation Coverage
4. SPL Token Mint and Recipient Account Coverage
5. PDA Coverage
6. Disabled Execution Gate and No-Mutation Coverage
7. Replay and Atomicity Coverage
8. Instruction Bytes and Reserved-Bytes Coverage
9. Rent and Lifecycle Coverage
10. Mollusk Coverage Review Package
11. Mollusk Coverage Assessment
12. Mollusk Blocker Transition

This order is intentionally conservative.

It separates implementation, evidence, assessment, and blocker transition.

## 18. Current Final Statement

This roadmap does not implement Mollusk tests.

This roadmap does not change Rust runtime code.

This roadmap does not add dependencies.

This roadmap does not remove `MOLLUSK_COVERAGE_INCOMPLETE`.

This roadmap does not remove any blocker.

The runtime remains scaffold-only, locked, unreleasable, and not deployable.

Live route execution, SPL CPI execution, `invoke_signed`, and SPL Token
`mint_to` remain disabled.
