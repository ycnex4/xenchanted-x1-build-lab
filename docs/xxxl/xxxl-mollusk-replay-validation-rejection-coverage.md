# XXXL Mollusk Replay and Validation Rejection Coverage

## 1. Purpose

This stage adds focused Mollusk/SVM entrypoint coverage for reachable replay
and validation rejection paths in the current locked `consume_gateway_mint`
scaffold.

The goal is to prove through the real SBF entrypoint that selected
processed-event replay, processed-event binding, and recipient-balance binding
mismatches are rejected before any live route execution, SPL CPI execution,
`invoke_signed`, or SPL Token `mint_to` path becomes reachable.

This is not a blocker-transition stage.

This is not deployment readiness.

## 2. Exact Coverage Scope

This stage covers the current `process_instruction` entrypoint behavior for:

- consumed processed event replay
- wrong processed event canonical event key
- wrong processed event route id
- wrong processed event recipient
- wrong recipient balance owner
- wrong recipient balance mint

The tests run through `Mollusk::new` and execute the XXXL SBF program through
Mollusk/SVM.

The tests intentionally assert rejected paths only.

## 3. Changed Files

- `programs/xxxl-svm/tests/mollusk_consume_gateway_mint.rs`
- `docs/xxxl/xxxl-mollusk-replay-validation-rejection-coverage.md`
- `docs/checkpoints/xxxl-mollusk-replay-validation-rejection-coverage.md`
- `docs/checkpoints/current-design-checkpoint.md`

## 4. Cargo / Dependency Statement

No Cargo files changed.

No dependencies were added.

The existing Mollusk dev-dependency and existing SBF test harness are reused.

## 5. Tests Added

Added non-ignored Mollusk tests:

- `mollusk_rejects_consumed_processed_event_replay_without_live_route`
- `mollusk_rejects_wrong_processed_event_canonical_event_key_without_live_route`
- `mollusk_rejects_wrong_processed_event_route_id_without_live_route`
- `mollusk_rejects_wrong_processed_event_recipient_without_live_route`
- `mollusk_rejects_wrong_recipient_balance_owner_without_live_route`
- `mollusk_rejects_wrong_recipient_balance_mint_without_live_route`

## 6. What Each Test Proves

`mollusk_rejects_consumed_processed_event_replay_without_live_route` proves that
an already-consumed processed event is rejected by the SBF entrypoint with
`XxxlError::InvalidInstruction` before live route execution.

`mollusk_rejects_wrong_processed_event_canonical_event_key_without_live_route`
proves that a processed event canonical event key mismatch is rejected by the
SBF entrypoint with `XxxlError::InvalidInstruction` before live route execution.

`mollusk_rejects_wrong_processed_event_route_id_without_live_route` proves that
a processed event route id mismatch is rejected by the SBF entrypoint with
`XxxlError::InvalidInstruction` before live route execution.

`mollusk_rejects_wrong_processed_event_recipient_without_live_route` proves that
a processed event recipient mismatch is rejected by the SBF entrypoint with
`XxxlError::InvalidInstruction` before live route execution.

`mollusk_rejects_wrong_recipient_balance_owner_without_live_route` proves that a
recipient balance owner mismatch is rejected by the SBF entrypoint with
`XxxlError::InvalidRecipientAta` before live route execution.

`mollusk_rejects_wrong_recipient_balance_mint_without_live_route` proves that a
recipient balance mint mismatch is rejected by the SBF entrypoint with
`XxxlError::InvalidRecipientAta` before live route execution.

## 7. What This Stage Does Not Prove

This stage does not prove:

- future live route atomicity
- future SPL CPI success or failure behavior
- direct Mollusk entrypoint coverage for the disabled SPL CPI gate
- SPL Token `mint_to` CPI execution
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

The next Mollusk stage should be instruction strictness coverage or rent and
lifecycle coverage, depending on the remaining evidence gap selected for
closure.

Expected next coverage may include:

- wrong discriminator rejection through Mollusk/SVM
- wrong instruction version rejection through Mollusk/SVM
- wrong instruction length rejection through Mollusk/SVM
- extra bytes rejection or documented reserved-byte policy
- low-rent rejection
- account lifecycle assumptions

`MOLLUSK_COVERAGE_INCOMPLETE` must remain active until the full required
Mollusk/SVM coverage set is implemented, reviewed, and separately accepted.

## 10. Final Statement

The XXXL Mollusk replay and validation rejection coverage stage adds real
SBF/Mollusk coverage for selected reachable replay and validation rejection
paths in the current locked scaffold entrypoint.

The runtime remains locked, unreleasable, and not deployable.

This stage does not unlock live route execution, SPL CPI execution,
`invoke_signed`, or SPL Token `mint_to`.
