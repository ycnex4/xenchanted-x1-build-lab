# XXXL Mollusk PDA Coverage

## 1. Purpose

This stage adds focused Mollusk/SVM negative coverage for the
`consume_gateway_mint` mint authority PDA boundary.

The goal is to prove through the real Mollusk harness that selected PDA key,
bump, wrong-program derivation, and semantic binding failures are rejected
before any live route execution, SPL CPI execution, `invoke_signed`, or SPL
Token `mint_to` path becomes reachable.

This is not a blocker-transition stage.

This is not deployment readiness.

## 2. Exact Coverage Scope

This stage covers:

- wrong `mint_authority_pda` account key
- wrong mint authority PDA bump in `mint_state`
- mint authority PDA derived for the wrong program id
- semantic mismatch between `mint_state` PDA binding and the passed PDA account

The tests run through `Mollusk::new` and execute the XXXL SBF program through
Mollusk/SVM.

The tests intentionally assert rejected paths only.

## 3. Changed Files

- `programs/xxxl-svm/tests/mollusk_consume_gateway_mint.rs`
- `docs/xxxl/xxxl-mollusk-pda-coverage.md`
- `docs/checkpoints/xxxl-mollusk-pda-coverage.md`
- `docs/checkpoints/current-design-checkpoint.md`

## 4. Cargo / Dependency Statement

No Cargo files changed.

No dependencies were added.

The existing Mollusk dev-dependency and existing SBF test harness are reused.

## 5. Tests Added

Added non-ignored Mollusk tests:

- `mollusk_rejects_wrong_mint_authority_pda_without_live_route`
- `mollusk_rejects_wrong_mint_authority_bump_without_live_route`
- `mollusk_rejects_mint_authority_pda_for_wrong_program_id_without_live_route`
- `mollusk_rejects_mint_authority_pda_semantic_mismatch_without_live_route`

## 6. What Each Test Proves

`mollusk_rejects_wrong_mint_authority_pda_without_live_route` proves that
substituting the `mint_authority_pda` account key is rejected by the SBF program
through Mollusk with `XxxlError::InvalidInstruction` before live route
execution.

`mollusk_rejects_wrong_mint_authority_bump_without_live_route` proves that a
`mint_state` bump mismatch for the otherwise correct PDA account is rejected by
the SBF program through Mollusk with `XxxlError::InvalidPda` before live route
execution.

`mollusk_rejects_mint_authority_pda_for_wrong_program_id_without_live_route`
proves that a PDA derived for another program id is rejected by the SBF program
through Mollusk with `XxxlError::InvalidPda` before live route execution.

`mollusk_rejects_mint_authority_pda_semantic_mismatch_without_live_route`
proves that a mismatch between the PDA recorded in `mint_state` and the passed
PDA account is rejected by the SBF program through Mollusk with
`XxxlError::InvalidInstruction` before live route execution.

## 7. What This Stage Does Not Prove

This stage does not prove:

- SPL Token `mint_to` CPI execution
- production PDA fixture regeneration
- production PDA readiness
- production guardian configuration
- production proof-log configuration
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

Production PDA fixtures were not regenerated.

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

The next Mollusk stage should be Disabled Execution Gate and No-Mutation
Coverage, not blocker transition.

Expected next coverage:

- disabled SPL CPI execution gate rejection
- no local state mutation on disabled execution gate failures
- no SPL Token balance or mint supply mutation
- rejection before any `invoke_signed` path

`MOLLUSK_COVERAGE_INCOMPLETE` must remain active until the full required
Mollusk/SVM coverage set is implemented, reviewed, and separately accepted.

## 10. Final Statement

The XXXL Mollusk PDA coverage stage adds real SBF/Mollusk negative coverage for
selected mint authority PDA rejection boundaries.

The runtime remains locked, unreleasable, and not deployable.

This stage does not unlock live route execution, SPL CPI execution,
`invoke_signed`, or SPL Token `mint_to`.
