# XXXL Mollusk Instruction Strictness Coverage

## 1. Purpose

This stage adds focused Mollusk/SVM entrypoint coverage for strict instruction
parsing and encoded instruction field validation in the current locked
`consume_gateway_mint` scaffold.

The goal is to prove through the real SBF entrypoint that malformed instruction
bytes and wrong encoded account/index/count fields are rejected before any live
route execution, SPL CPI execution, `invoke_signed`, or SPL Token `mint_to` path
becomes reachable.

This is not a blocker-transition stage.

This is not deployment readiness.

## 2. Exact Coverage Scope

This stage covers the current `process_instruction` entrypoint behavior for:

- wrong instruction discriminator
- wrong instruction layout version
- extra instruction bytes
- wrong encoded account meta count
- wrong encoded processed event account index
- wrong encoded recipient balance account index

The tests run through `Mollusk::new` and execute the XXXL SBF program through
Mollusk/SVM.

The tests intentionally assert rejected paths only.

## 3. Changed Files

- `programs/xxxl-svm/tests/mollusk_consume_gateway_mint.rs`
- `docs/xxxl/xxxl-mollusk-instruction-strictness-coverage.md`
- `docs/checkpoints/xxxl-mollusk-instruction-strictness-coverage.md`
- `docs/checkpoints/current-design-checkpoint.md`

## 4. Cargo / Dependency Statement

No Cargo files changed.

No dependencies were added.

The existing Mollusk dev-dependency and existing SBF test harness are reused.

## 5. Tests Added

Added non-ignored Mollusk tests:

- `mollusk_rejects_wrong_instruction_discriminator_without_live_route`
- `mollusk_rejects_wrong_instruction_version_without_live_route`
- `mollusk_rejects_extra_instruction_bytes_without_live_route`
- `mollusk_rejects_wrong_encoded_account_meta_count_without_live_route`
- `mollusk_rejects_wrong_encoded_processed_event_account_index_without_live_route`
- `mollusk_rejects_wrong_encoded_recipient_balance_account_index_without_live_route`

## 6. What Each Test Proves

`mollusk_rejects_wrong_instruction_discriminator_without_live_route` proves that
a wrong instruction discriminator is rejected by the SBF entrypoint with
`XxxlError::InvalidDiscriminator` before live route execution.

`mollusk_rejects_wrong_instruction_version_without_live_route` proves that a
wrong instruction layout version is rejected by the SBF entrypoint with
`XxxlError::InvalidVersion` before live route execution.

`mollusk_rejects_extra_instruction_bytes_without_live_route` proves that extra
instruction bytes are rejected by the SBF entrypoint with
`XxxlError::InvalidInstruction` before live route execution.

`mollusk_rejects_wrong_encoded_account_meta_count_without_live_route` proves
that a wrong encoded account meta count is rejected by the SBF entrypoint with
`XxxlError::InvalidInstruction` before live route execution.

`mollusk_rejects_wrong_encoded_processed_event_account_index_without_live_route`
proves that a wrong encoded processed event account index is rejected by the SBF
entrypoint with `XxxlError::InvalidInstruction` before live route execution.

`mollusk_rejects_wrong_encoded_recipient_balance_account_index_without_live_route`
proves that a wrong encoded recipient balance account index is rejected by the
SBF entrypoint with `XxxlError::InvalidInstruction` before live route execution.

## 7. What This Stage Does Not Prove

This stage does not prove:

- account meta/order coverage beyond the already completed stage
- replay or validation rejection coverage beyond the already completed stage
- no-mutation coverage beyond the already completed stage
- direct Mollusk entrypoint coverage for the disabled SPL CPI gate
- future live route atomicity
- future SPL CPI success or failure behavior
- SPL Token `mint_to` CPI execution
- `invoke_signed` execution
- production PDA fixture regeneration
- production guardian configuration
- production proof-log configuration
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

The next Mollusk stage should be rent and lifecycle coverage or a coverage
review package, depending on the remaining evidence gap selected for closure.

Expected next coverage may include:

- low-rent rejection
- rent exemption acceptance at the current boundary
- account lifecycle assumptions
- explicit mapping between Mollusk/SBF evidence and Rust-boundary evidence

`MOLLUSK_COVERAGE_INCOMPLETE` must remain active until the full required
Mollusk/SVM coverage set is implemented, reviewed, and separately accepted.

## 10. Final Statement

The XXXL Mollusk instruction strictness coverage stage adds real SBF/Mollusk
coverage for selected malformed instruction bytes and encoded instruction field
strictness paths in the current locked scaffold entrypoint.

The runtime remains locked, unreleasable, and not deployable.

This stage does not unlock live route execution, SPL CPI execution,
`invoke_signed`, or SPL Token `mint_to`.
