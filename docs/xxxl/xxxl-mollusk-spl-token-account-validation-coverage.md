# XXXL Mollusk SPL Token Account Validation Coverage

## 1. Purpose

This stage adds focused Mollusk/SVM negative coverage for SPL Token mint and
recipient token account validation in `consume_gateway_mint`.

The goal is to prove through the real Mollusk harness that selected SPL Token
account owner, mint authority, initialization, recipient mint, and recipient
owner failures are rejected before any live route execution, SPL CPI execution,
`invoke_signed`, or SPL Token `mint_to` path becomes reachable.

This is not a blocker-transition stage.

This is not deployment readiness.

## 2. Exact Coverage Scope

This stage covers:

- wrong owner for the `spl_token_mint` account
- wrong SPL Token mint authority
- uninitialized SPL Token mint account
- wrong recipient token account mint
- wrong recipient token account owner
- uninitialized recipient token account

The tests run through `Mollusk::new` and execute the XXXL SBF program through
Mollusk/SVM.

The tests intentionally assert rejected paths only.

## 3. Changed Files

- `programs/xxxl-svm/tests/mollusk_consume_gateway_mint.rs`
- `docs/xxxl/xxxl-mollusk-spl-token-account-validation-coverage.md`
- `docs/checkpoints/xxxl-mollusk-spl-token-account-validation-coverage.md`
- `docs/checkpoints/current-design-checkpoint.md`

## 4. Cargo / Dependency Statement

No Cargo files changed.

No dependencies were added.

The existing Mollusk dev-dependency and existing SBF test harness are reused.

## 5. Tests Added

Added non-ignored Mollusk tests:

- `mollusk_rejects_wrong_spl_mint_owner_without_live_route`
- `mollusk_rejects_wrong_spl_mint_authority_without_live_route`
- `mollusk_rejects_uninitialized_spl_mint_without_live_route`
- `mollusk_rejects_wrong_recipient_token_mint_without_live_route`
- `mollusk_rejects_wrong_recipient_token_owner_without_live_route`
- `mollusk_rejects_uninitialized_recipient_token_account_without_live_route`

## 6. What Each Test Proves

`mollusk_rejects_wrong_spl_mint_owner_without_live_route` proves that a
`spl_token_mint` account not owned by the SPL Token program is rejected by the
SBF program through Mollusk with `XxxlError::InvalidAccountOwner` before live
route execution.

`mollusk_rejects_wrong_spl_mint_authority_without_live_route` proves that an
initialized SPL Token mint whose authority does not match the expected mint
authority PDA is rejected by the SBF program through Mollusk with
`XxxlError::InvalidPda` before live route execution.

`mollusk_rejects_uninitialized_spl_mint_without_live_route` proves that an
uninitialized SPL Token mint account is rejected by the SBF program through
Mollusk with `XxxlError::InvalidInstruction` before live route execution.

`mollusk_rejects_wrong_recipient_token_mint_without_live_route` proves that a
recipient token account pointing at the wrong mint is rejected by the SBF
program through Mollusk with `XxxlError::InvalidRecipientAta` before live route
execution.

`mollusk_rejects_wrong_recipient_token_owner_without_live_route` proves that a
recipient token account owned by the wrong recipient is rejected by the SBF
program through Mollusk with `XxxlError::InvalidRecipientAta` before live route
execution.

`mollusk_rejects_uninitialized_recipient_token_account_without_live_route`
proves that an uninitialized recipient token account is rejected by the SBF
program through Mollusk with `XxxlError::InvalidRecipientAta` before live route
execution.

## 7. What This Stage Does Not Prove

This stage does not prove:

- all SPL Token account data corruption permutations
- SPL Token `mint_to` CPI execution
- PDA production transition
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

The next Mollusk stage should be PDA Coverage, not blocker transition.

Expected next coverage:

- wrong mint authority PDA account key
- wrong mint authority bump
- PDA mismatch before any `invoke_signed` path
- PDA validation through the SBF/Mollusk boundary

`MOLLUSK_COVERAGE_INCOMPLETE` must remain active until the full required
Mollusk/SVM coverage set is implemented, reviewed, and separately accepted.

## 10. Final Statement

The XXXL Mollusk SPL Token account validation coverage stage adds real
SBF/Mollusk negative coverage for selected SPL Token mint and recipient token
account rejection boundaries.

The runtime remains locked, unreleasable, and not deployable.

This stage does not unlock live route execution, SPL CPI execution,
`invoke_signed`, or SPL Token `mint_to`.
