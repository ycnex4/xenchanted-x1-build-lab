# XXXL Mollusk Program-Owned Account Validation Coverage

## 1. Purpose

This stage adds focused Mollusk/SVM negative coverage for program-owned account
validation in `consume_gateway_mint`.

The goal is to prove through the real Mollusk harness that selected
program-owned account owner and account-data layout failures are rejected before
any live route execution, SPL CPI execution, `invoke_signed`, or SPL Token
`mint_to` path becomes reachable.

This is not a blocker-transition stage.

This is not deployment readiness.

## 2. Exact Coverage Scope

This stage covers:

- wrong owner for the `mint_state` program-owned account
- wrong owner for the `gateway_config` program-owned account
- wrong owner for the `guardian_set` program-owned account
- wrong discriminator for the `mint_state` program-owned account
- truncated account data for the `gateway_config` program-owned account

The tests run through `Mollusk::new` and execute the XXXL SBF program through
Mollusk/SVM.

The tests intentionally assert rejected paths only.

## 3. Changed Files

- `programs/xxxl-svm/tests/mollusk_consume_gateway_mint.rs`
- `docs/xxxl/xxxl-mollusk-program-owned-account-validation-coverage.md`
- `docs/checkpoints/xxxl-mollusk-program-owned-account-validation-coverage.md`
- `docs/checkpoints/current-design-checkpoint.md`

## 4. Cargo / Dependency Statement

No Cargo files changed.

No dependencies were added.

The existing Mollusk dev-dependency and existing SBF test harness are reused.

## 5. Tests Added

Added non-ignored Mollusk tests:

- `mollusk_rejects_wrong_mint_state_owner_without_live_route`
- `mollusk_rejects_wrong_gateway_config_owner_without_live_route`
- `mollusk_rejects_wrong_guardian_set_owner_without_live_route`
- `mollusk_rejects_wrong_mint_state_discriminator_without_live_route`
- `mollusk_rejects_truncated_gateway_config_without_live_route`

## 6. What Each Test Proves

`mollusk_rejects_wrong_mint_state_owner_without_live_route` proves that a
`mint_state` account not owned by the XXXL program is rejected by the SBF
program through Mollusk with `XxxlError::InvalidAccountOwner` before live route
execution.

`mollusk_rejects_wrong_gateway_config_owner_without_live_route` proves that a
`gateway_config` account not owned by the XXXL program is rejected by the SBF
program through Mollusk with `XxxlError::InvalidAccountOwner` before live route
execution.

`mollusk_rejects_wrong_guardian_set_owner_without_live_route` proves that a
`guardian_set` account not owned by the XXXL program is rejected by the SBF
program through Mollusk with `XxxlError::InvalidAccountOwner` before live route
execution.

`mollusk_rejects_wrong_mint_state_discriminator_without_live_route` proves that
a `mint_state` account with the wrong discriminator is rejected by the SBF
program through Mollusk with `XxxlError::InvalidDiscriminator` before live route
execution.

`mollusk_rejects_truncated_gateway_config_without_live_route` proves that a
`gateway_config` account with truncated data is rejected by the SBF program
through Mollusk with `XxxlError::InvalidInstruction` before live route
execution.

## 7. What This Stage Does Not Prove

This stage does not prove:

- all program-owned account owner permutations
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

The next Mollusk stage should be SPL Token Mint and Recipient Account Coverage,
not blocker transition.

Expected next coverage:

- wrong SPL Token program id
- wrong SPL Token mint owner or data
- wrong SPL Token mint authority
- wrong recipient token account owner
- wrong recipient token account mint
- recipient token account initialization state

`MOLLUSK_COVERAGE_INCOMPLETE` must remain active until the full required
Mollusk/SVM coverage set is implemented, reviewed, and separately accepted.

## 10. Final Statement

The XXXL Mollusk program-owned account validation coverage stage adds real
SBF/Mollusk negative coverage for selected program-owned account owner and
layout rejection boundaries.

The runtime remains locked, unreleasable, and not deployable.

This stage does not unlock live route execution, SPL CPI execution,
`invoke_signed`, or SPL Token `mint_to`.
