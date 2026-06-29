# XXXL Mollusk Entrypoint No-Mutation Coverage

## 1. Purpose

This stage adds focused Mollusk/SVM entrypoint coverage for the current locked
`consume_gateway_mint` scaffold.

The goal is to prove through the real SBF entrypoint that valid scaffold
execution builds the current execution plan without mutating mutable
state/token accounts, and that selected rejected paths also leave those mutable
accounts unchanged.

This is not a blocker-transition stage.

This is not deployment readiness.

## 2. Exact Coverage Scope

This stage covers the current `process_instruction` entrypoint behavior for:

- valid scaffold execution
- already-consumed processed event rejection
- zero amount rejection
- wrong recipient token account rejection
- wrong processed event recipient rejection

Each test asserts the expected entrypoint result and unchanged data for:

- `processed_event`
- `recipient_balance`
- `spl_mint`
- `recipient_token_account`

The tests run through `Mollusk::new` and execute the XXXL SBF program through
Mollusk/SVM.

## 3. Changed Files

- `programs/xxxl-svm/tests/mollusk_consume_gateway_mint.rs`
- `docs/xxxl/xxxl-mollusk-entrypoint-no-mutation-coverage.md`
- `docs/checkpoints/xxxl-mollusk-entrypoint-no-mutation-coverage.md`
- `docs/checkpoints/current-design-checkpoint.md`

## 4. Cargo / Dependency Statement

No Cargo files changed.

No dependencies were added.

The existing Mollusk dev-dependency and existing SBF test harness are reused.

## 5. Tests Added

Added non-ignored Mollusk tests:

- `mollusk_valid_scaffold_entrypoint_leaves_mutable_accounts_unchanged`
- `mollusk_consumed_processed_event_rejection_leaves_mutable_accounts_unchanged`
- `mollusk_zero_amount_rejection_leaves_mutable_accounts_unchanged`
- `mollusk_wrong_recipient_token_account_rejection_leaves_mutable_accounts_unchanged`
- `mollusk_wrong_processed_event_recipient_rejection_leaves_mutable_accounts_unchanged`

## 6. What Each Test Proves

`mollusk_valid_scaffold_entrypoint_leaves_mutable_accounts_unchanged` proves
that the current locked scaffold entrypoint returns success for a valid
`consume_gateway_mint` instruction without changing `processed_event`,
`recipient_balance`, `spl_mint`, or `recipient_token_account` data.

`mollusk_consumed_processed_event_rejection_leaves_mutable_accounts_unchanged`
proves that an already-consumed processed event is rejected with
`XxxlError::InvalidInstruction` and leaves the mutable accounts unchanged.

`mollusk_zero_amount_rejection_leaves_mutable_accounts_unchanged` proves that a
zero mint amount is rejected with `XxxlError::InvalidInstruction` and leaves the
mutable accounts unchanged.

`mollusk_wrong_recipient_token_account_rejection_leaves_mutable_accounts_unchanged`
proves that a recipient token account ownership mismatch is rejected with
`XxxlError::InvalidRecipientAta` and leaves the mutable accounts unchanged.

`mollusk_wrong_processed_event_recipient_rejection_leaves_mutable_accounts_unchanged`
proves that a processed event recipient mismatch is rejected with
`XxxlError::InvalidInstruction` and leaves the mutable accounts unchanged.

## 7. What This Stage Does Not Prove

This stage does not prove:

- direct Mollusk entrypoint coverage for the disabled SPL CPI gate
- SPL Token `mint_to` CPI execution
- future SPL CPI success or failure atomicity
- future live route execution atomicity
- `invoke_signed` execution
- production PDA fixture regeneration
- production guardian configuration
- production proof-log configuration
- instruction reserved-bytes policy
- rent or account lifecycle coverage
- Mollusk coverage completeness
- runtime deployability
- release readiness

The disabled SPL CPI gate remains Rust-boundary evidence until a future reviewed
runtime-composition stage makes that boundary reachable through the SBF
entrypoint.

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

The next Mollusk stage should be reachable replay/validation rejection coverage
or instruction strictness coverage, depending on the remaining evidence gap
selected for closure.

Expected next coverage may include:

- wrong canonical event key rejection
- wrong route id rejection
- wrong recipient balance owner rejection
- wrong recipient balance mint rejection
- amount larger than the SPL Token `u64` range
- instruction byte strictness and reserved-byte policy

`MOLLUSK_COVERAGE_INCOMPLETE` must remain active until the full required
Mollusk/SVM coverage set is implemented, reviewed, and separately accepted.

## 10. Final Statement

The XXXL Mollusk entrypoint no-mutation coverage stage adds real SBF/Mollusk
coverage for the current locked scaffold entrypoint.

The runtime remains locked, unreleasable, and not deployable.

This stage does not unlock live route execution, SPL CPI execution,
`invoke_signed`, or SPL Token `mint_to`.
