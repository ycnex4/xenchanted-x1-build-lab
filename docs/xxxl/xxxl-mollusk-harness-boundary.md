# XXXL Mollusk Harness Boundary

## 1. Purpose

This stage records the first narrow Mollusk/SVM harness boundary for the
current locked XXXL SVM scaffold.

The goal is to prove that a real Mollusk harness can execute the compiled XXXL
SBF program on a harmless rejected path without enabling live route execution,
SPL CPI execution, `invoke_signed`, or SPL Token `mint_to`.

This is not a coverage-complete stage.

This is not a blocker-transition stage.

This is not deployment readiness.

## 2. Harness Scope

The harness scope is intentionally minimal.

Added test:

- `mollusk_harness_rejects_malformed_instruction_without_live_route`

Test file:

- `programs/xxxl-svm/tests/mollusk_consume_gateway_mint.rs`

The sanity test:

- constructs a fresh program id
- loads the XXXL SBF artifact through `Mollusk::new`
- submits a malformed `consume_gateway_mint` instruction with invalid length
- supplies no accounts
- expects `XxxlError::InvalidInstruction`

This proves the harness is real because the program is executed through
Mollusk/SVM and returns the expected rejected result from the locked scaffold
instruction path.

## 3. Changed Files

- `programs/xxxl-svm/tests/mollusk_consume_gateway_mint.rs`
- `docs/xxxl/xxxl-mollusk-harness-boundary.md`
- `docs/checkpoints/xxxl-mollusk-harness-boundary.md`
- `docs/checkpoints/current-design-checkpoint.md`

## 4. Dependency Changes

No Cargo dependency changes were needed in this stage.

The Mollusk test dependency already exists:

- `mollusk-svm = "0.13.4"`

The split-crate test dependencies required by the existing harness already
exist in `programs/xxxl-svm/Cargo.toml`.

`Cargo.toml` and `Cargo.lock` are unchanged.

## 5. What The Sanity Test Proves

The sanity test proves:

- the Mollusk harness can load the XXXL SBF artifact
- a test whose name contains `mollusk` can be run by filter
- the harness executes a rejected instruction path through Mollusk/SVM
- malformed instruction length is rejected before account validation
- the rejected path does not require live route execution
- the rejected path does not require SPL CPI execution
- the rejected path does not require `invoke_signed`
- the rejected path does not require SPL Token `mint_to`

## 6. What The Sanity Test Does Not Prove

The sanity test does not prove:

- account meta/order coverage is complete
- program-owned account validation is complete under Mollusk
- SPL Token mint or recipient account validation is complete under Mollusk
- PDA coverage is complete under Mollusk
- disabled execution gate no-mutation coverage is complete
- replay or atomicity coverage is complete under Mollusk
- instruction bytes coverage is complete beyond the malformed length sanity path
- rent or lifecycle coverage is complete under Mollusk
- the runtime is deployable
- the runtime is release-ready
- `MOLLUSK_COVERAGE_INCOMPLETE` can be removed

## 7. Safety State

`MOLLUSK_COVERAGE_INCOMPLETE` remains active.

No blocker is removed.

No blocker is transitioned.

The runtime remains:

- scaffold-only
- locked
- unreleasable
- not deployable

Live route execution remains disabled.

SPL CPI execution remains disabled.

`invoke_signed` is not enabled.

SPL Token `mint_to` is not enabled.

## 8. Next Stage

The next Mollusk stage should be account meta/order coverage, not blocker
transition.

Expected next coverage:

- correct account count
- wrong account count
- wrong account order
- unexpected writable account
- missing writable account
- unexpected signer
- duplicate account keys / aliasing attempts

`MOLLUSK_COVERAGE_INCOMPLETE` must remain active until the full required
Mollusk/SVM coverage set is implemented, reviewed, and separately accepted.

## 9. Final Statement

The XXXL Mollusk harness boundary is established with one safe rejected-path
sanity test.

The runtime remains locked, unreleasable, and not deployable.

This stage does not unlock live route execution, SPL CPI execution,
`invoke_signed`, or SPL Token `mint_to`.
