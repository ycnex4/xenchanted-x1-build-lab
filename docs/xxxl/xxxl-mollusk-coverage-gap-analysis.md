# XXXL Mollusk Coverage Gap Analysis

## 1. Purpose

This document records the docs-only gap analysis for the
`MOLLUSK_COVERAGE_INCOMPLETE` deployment blocker.

The purpose is to define the exact future Mollusk/SVM coverage requirements
needed before any later blocker-transition stage can consider removing
`MOLLUSK_COVERAGE_INCOMPLETE`.

This stage does not implement Mollusk tests.

This stage does not remove any blocker.

This stage does not change Rust runtime code.

## 2. Current Blocker State

`ACCOUNT_CONTRACT_UNREVIEWED` has already been transitioned after the account
contract review closure boundary.

The active deployment blockers remain:

- `PLACEHOLDER_PROGRAM_ID`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `MOLLUSK_COVERAGE_INCOMPLETE`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

The active blocker count remains 7.

`MOLLUSK_COVERAGE_INCOMPLETE` remains active.

The runtime remains scaffold-only, locked, unreleasable, and not deployable.

Live route execution remains disabled.

SPL CPI execution remains disabled.

`invoke_signed` remains unreachable from `process_instruction`.

SPL Token `mint_to` remains disabled.

## 3. Why `MOLLUSK_COVERAGE_INCOMPLETE` Remains Active

The current Rust unit tests provide useful boundary evidence, but they are not
equivalent to Mollusk/SVM runtime coverage.

Unit tests validate Rust functions, instruction parsing, account contract
metadata, semantic account binding, execution-plan composition, disabled CPI
gates, PDA derivation, and state mutation helpers.

They do not prove SVM account scheduling, account aliasing behavior, runtime
borrow behavior, CPI call behavior, lamport/rent lifecycle behavior, or
transaction-level failure atomicity under a Mollusk harness.

`MOLLUSK_COVERAGE_INCOMPLETE` must therefore remain active until a later stage
adds, reviews, and documents Mollusk/SVM tests for the required runtime
scenarios below.

## 4. Existing Rust Unit-Test Coverage That Is Not Equivalent to Mollusk

Current Rust unit tests already cover many important boundaries:

- deployment status reports keep the runtime not deployable
- safety invariant tests keep release blocked
- account contract tests cover 9-account shape, index mapping, writable flags,
  signer requirements, and owner model classification
- instruction tests cover wrong discriminator, wrong version, wrong length,
  wrong account meta count, and wrong account index bytes
- processor tests cover account count, account order, program owner, gateway
  config, guardian set, processed event, recipient balance, SPL Token program,
  SPL mint authority, mint authority bump, recipient token mint, zero amount,
  and amount overflow cases
- validation tests cover SPL Token mint ownership/initialization/authority,
  recipient token account owner/mint/initialization, and rent exemption
- state and execution-plan tests cover consumed event rejection, event marking,
  recipient balance credit, replay rejection, wrong event key, wrong recipient,
  wrong mint, wrong owner, zero amount, overflow, and atomic ordering
- CPI planning tests cover wrong token program, wrong mint, wrong PDA, wrong
  bump, disabled gate behavior, no `invoke_signed` in planning, and no enabled
  SPL Token `mint_to` path
- PDA tests cover seed inventory, gateway mint authority derivation, wrong
  bump, wrong PDA, wrong Program ID, and PDA changes with Program ID

This coverage is necessary evidence, but it is still lower-level Rust
coverage. It does not replace Mollusk/SVM execution coverage for future live
route or SPL CPI activation.

## 5. Required Future Mollusk Coverage Areas

The following coverage areas are required before a future transition can
consider removing `MOLLUSK_COVERAGE_INCOMPLETE`.

### A. Account ordering / account meta coverage

Future Mollusk coverage must include:

- correct account count
- wrong account count
- wrong account order
- unexpected writable account
- missing writable account
- unexpected signer
- duplicate account keys / aliasing attempts

The duplicate key cases must include attempts to alias mutable program-owned
accounts and writable SPL Token accounts in ways that could expose SVM borrow
or account scheduling behavior not visible in ordinary Rust unit tests.

### B. Program-owned account validation

Future Mollusk coverage must include:

- wrong owner for program-owned accounts
- wrong discriminator
- wrong version
- truncated data
- route id field binding mismatch
- mint id field binding mismatch
- recipient owner field binding mismatch
- canonical event key field binding mismatch
- guardian set id field binding mismatch

Coverage must exercise the program-owned accounts used by
`consume_gateway_mint`, including `mint_state`, `gateway_config`,
`guardian_set`, `processed_event`, and `recipient_balance`.

### C. SPL Token account / mint coverage

Future Mollusk coverage must include:

- wrong SPL Token program
- wrong SPL mint owner
- wrong mint authority
- wrong recipient token mint
- wrong recipient token owner
- uninitialized recipient token account
- uninitialized mint
- SPL Token close/reinit race risk scenario if applicable

The close/reinit race case should be treated as a future live-CPI concern. If
the future runtime design proves it is not applicable, the blocker-transition
stage must explicitly document why.

### D. PDA coverage

Future Mollusk coverage must include:

- wrong mint authority PDA
- wrong bump
- PDA changes with Program ID
- PDA semantic separation
- production PDA derivation requirements for program-owned accounts

The current gateway mint authority PDA derivation evidence is not enough for
production account identity. Future coverage must also address the production
PDA derivation requirements recorded by account-contract closure for
program-owned accounts.

### E. Execution gate coverage

Future Mollusk coverage must include:

- live route disabled rejects before mutation
- SPL CPI disabled rejects before mutation
- no `invoke_signed`
- no SPL Token `mint_to`
- no state mutation on disabled CPI gate
- no processed event mark on rejected path
- no recipient balance credit on rejected path

This coverage must show that disabled execution gates fail before any durable
state mutation or CPI side effect.

### F. Replay / atomicity coverage

Future Mollusk coverage must include:

- consumed processed event rejected
- replay rejected before credit
- zero amount rejected
- overflow rejected
- wrong event key rejected
- failed CPI must not leave `ProcessedBurnEntry` if future live CPI is
  introduced

The failed-CPI case is a future live-route requirement. It must be tested before
any path can mark processed events or credit balances around a real SPL Token
CPI.

### G. Instruction bytes coverage

Future Mollusk coverage must include:

- wrong discriminator
- wrong version
- wrong length
- padding / reserved bytes policy

The padding/reserved bytes policy must be decided before blocker transition:
reserved bytes must either be rejected, required to be zero, explicitly ignored,
or otherwise documented as part of the canonical instruction policy.

### H. Rent / lifecycle coverage

Future Mollusk coverage must include:

- rent exemption check
- low lamports rejected
- account close/reinit timing concerns

This coverage should reflect SVM account lifecycle behavior rather than only
standalone helper-function behavior.

## 6. What Must Not Change In This Stage

This stage must not change:

- Rust source files
- `Cargo.toml`
- `Cargo.lock`
- Mollusk dependency or configuration
- any test implementation
- live route execution flags
- SPL CPI execution flags
- `invoke_signed` reachability
- SPL Token `mint_to` reachability
- deployment blocker list
- Program ID
- PDA fixtures
- production guardian configuration
- production proof-log configuration
- runtime deployment status
- deployability predicates
- release lock semantics

## 7. Future Blocker-Transition Criteria

A future transition of `MOLLUSK_COVERAGE_INCOMPLETE` must be a separate
reviewed stage.

At minimum, that stage must:

- implement Mollusk/SVM tests for the required coverage areas in this document
- document the exact test names and outcomes
- prove the active blocker count changes only by the intended blocker
- keep runtime not deployable unless every other deployment blocker has also
  been separately resolved
- keep live route execution disabled unless a separate reviewed boundary allows
  activation
- keep SPL CPI execution disabled unless a separate reviewed boundary allows
  activation
- never enable `invoke_signed` unless a separate reviewed boundary explicitly
  allows it
- never enable SPL Token `mint_to` unless a separate reviewed boundary
  explicitly allows it
- preserve release blocking unless the full runtime safety lock has been
  separately cleared

## 8. Final Statement

This gap analysis defines the required future Mollusk/SVM coverage scope.

`MOLLUSK_COVERAGE_INCOMPLETE` remains active.

No Mollusk tests are implemented in this stage.

No blocker is removed in this stage.

The runtime remains scaffold-only, locked, unreleasable, and not deployable.
