# XXXL Mollusk Account Meta / Order Coverage

## 1. Purpose

This stage adds the first focused Mollusk/SVM account meta and account ordering
coverage for the XXXL SVM locked scaffold.

The goal is to prove through the real Mollusk harness that malformed account
metadata and account order errors are rejected before any live route execution,
SPL CPI execution, `invoke_signed`, or SPL Token `mint_to` path becomes
reachable.

This is not a blocker-transition stage.

This is not deployment readiness.

## 2. Exact Coverage Scope

This stage covers:

- wrong account count
- wrong account order
- unexpected signer
- readonly account passed writable

The tests run through `Mollusk::new` and execute the XXXL SBF program through
Mollusk/SVM.

The tests intentionally assert rejected paths only.

## 3. Changed Files

- `programs/xxxl-svm/tests/mollusk_consume_gateway_mint.rs`
- `docs/xxxl/xxxl-mollusk-account-meta-order-coverage.md`
- `docs/checkpoints/xxxl-mollusk-account-meta-order-coverage.md`
- `docs/checkpoints/current-design-checkpoint.md`

## 4. Cargo / Dependency Statement

No Cargo files changed.

No dependencies were added.

The existing Mollusk dev-dependency and existing SBF test harness are reused.

## 5. Tests Added

Added non-ignored Mollusk tests:

- `mollusk_rejects_wrong_account_count_without_live_route`
- `mollusk_rejects_wrong_account_order_without_live_route`
- `mollusk_rejects_unexpected_signer_without_live_route`
- `mollusk_rejects_writable_readonly_mismatch_without_live_route`

## 6. What Each Test Proves

`mollusk_rejects_wrong_account_count_without_live_route` proves that removing
one account meta and one runtime account is rejected by the SBF program through
Mollusk before live route execution.

`mollusk_rejects_wrong_account_order_without_live_route` proves that swapping
the gateway config and guardian set account metas/accounts is rejected by the
SBF program through Mollusk before live route execution.

`mollusk_rejects_unexpected_signer_without_live_route` proves that adding an
unexpected signer to an account that the account contract marks as not signer is
rejected by the SBF program through Mollusk before live route execution.

`mollusk_rejects_writable_readonly_mismatch_without_live_route` proves that
marking a readonly account writable is rejected by the SBF program through
Mollusk before live route execution.

## 7. What This Stage Does Not Prove

This stage does not prove:

- duplicate account key / aliasing coverage
- full program-owned account validation coverage
- SPL Token mint or recipient account validation coverage
- PDA validation coverage
- disabled SPL CPI gate no-mutation coverage
- replay or atomicity coverage
- instruction reserved-bytes policy
- rent or account lifecycle coverage
- Mollusk coverage completeness
- runtime deployability
- release readiness

## 8. Safety State

`MOLLUSK_COVERAGE_INCOMPLETE` remains active.

No blocker was removed.

No blocker was transitioned.

The runtime remains:

- scaffold-only
- locked
- unreleasable
- not deployable

Live route execution remains disabled.

SPL CPI execution remains disabled.

`invoke_signed` is not enabled.

SPL Token `mint_to` is not enabled.

## 9. Next Stage

The next Mollusk stage should be Program-Owned Account Validation Coverage, not
blocker transition.

Expected next coverage:

- wrong owner for program-owned accounts
- wrong discriminator
- wrong version
- truncated data
- route id mismatch
- mint id mismatch
- recipient owner mismatch
- canonical event key mismatch
- guardian set id mismatch

`MOLLUSK_COVERAGE_INCOMPLETE` must remain active until the full required
Mollusk/SVM coverage set is implemented, reviewed, and separately accepted.

## 10. Final Statement

The XXXL Mollusk account meta/order coverage stage adds real SBF/Mollusk
negative coverage for the locked scaffold.

The runtime remains locked, unreleasable, and not deployable.

This stage does not unlock live route execution, SPL CPI execution,
`invoke_signed`, or SPL Token `mint_to`.
