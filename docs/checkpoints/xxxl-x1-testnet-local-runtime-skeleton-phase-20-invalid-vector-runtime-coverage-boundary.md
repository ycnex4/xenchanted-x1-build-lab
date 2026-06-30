# XXXL X1 Testnet Local Runtime Skeleton Phase 20 Invalid-vector Runtime Coverage Boundary

Status: Rust test evidence checkpoint - runtime source remains unchanged.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-20-invalid-vector-runtime-coverage-boundary`

## Purpose

Phase 20 adds executable Rust test evidence for the current invalid-vector
runtime coverage boundary.

The purpose is to distinguish:

- invalid-vector classes that are currently observable at the SVM runtime
  boundary
- invalid-vector classes that belong to earlier watcher/model/canonical-encoding
  layers
- invalid-vector classes that are not yet represented by the current runtime
  instruction/account model

Phase 20 does not claim full Stage 1 invalid-vector runtime completion.

Phase 20 does not enable live route execution.

Phase 20 does not enable SPL CPI execution.

Phase 20 does not change production runtime behavior.

## Scope

Phase 20 changes Rust tests only.

Changed test files:

- `programs/xxxl-svm/tests/mollusk_consume_gateway_mint.rs`
- `programs/xxxl-svm/tests/instruction_reserved_bytes.rs`

The `instruction_reserved_bytes.rs` change is rustfmt-only import formatting.

Preserved test:

- `consume_gateway_mint_accepts_nonzero_reserved_bytes_194_208_as_raw_only`

Not modified:

- `programs/xxxl-svm/src/**`
- `programs/xxxl-svm/Cargo.toml`
- workspace-level Cargo files
- deployment scripts
- upgrade scripts
- CI/CD workflows that deploy, upgrade, submit transactions, or spend SOL
- `.local-keys/**`
- keypair JSON files
- `.env`
- `target/deploy/**`
- `.so` artifacts

## Test Evidence Added

New test:

- `phase20_current_runtime_boundary_matrix_separates_unrepresented_stage1_classes`

New runtime-observable rejection tests:

- `mollusk_amount_above_u64_rejection_leaves_mutable_accounts_unchanged`
- `mollusk_wrong_mint_state_mint_id_rejection_leaves_mutable_accounts_unchanged`
- `mollusk_wrong_gateway_config_route_id_rejection_leaves_mutable_accounts_unchanged`
- `mollusk_wrong_gateway_config_guardian_set_id_rejection_leaves_mutable_accounts_unchanged`
- `mollusk_wrong_gateway_config_target_mint_rejection_leaves_mutable_accounts_unchanged`
- `mollusk_wrong_gateway_config_source_chain_weight_rejection_leaves_mutable_accounts_unchanged`
- `mollusk_wrong_guardian_set_id_rejection_leaves_mutable_accounts_unchanged`

Strengthened helper usage:

- `process_rejection_and_assert_mutable_accounts_unchanged`

This helper is now used across applicable Mollusk rejection tests to confirm
that rejection paths leave these mutable accounts unchanged:

- ProcessedEvent
- RecipientBalance
- SPL mint account
- recipient SPL token account

## Runtime-observable Classes Covered

Phase 20 records current runtime-observable coverage for:

- instruction discriminator
- instruction version
- instruction length
- account meta count
- account index / order
- account flags / signer expectations
- route id
- guardian set id
- mint id / target mint
- canonical event key
- recipient / recipient token mapping
- source-chain weight mismatch against gateway config
- zero amount
- amount greater than `u64::MAX` for the current SPL route boundary
- consumed ProcessedEvent / replay-like rejection
- ProcessedEvent field mismatches
- RecipientBalance owner mismatch
- RecipientBalance mint mismatch
- SPL mint owner mismatch
- SPL mint authority mismatch
- SPL mint initialized-state mismatch
- recipient token owner mismatch
- recipient token mint mismatch
- recipient token initialized-state mismatch
- mint authority PDA mismatch
- mint authority bump mismatch
- represented low-rent checks

## Explicitly Not Claimed As Covered

Phase 20 does not claim runtime coverage for:

- sourceChainId final runtime binding
- source block / finality fields
- messageNonce runtime replay semantics
- guardian signature / quorum validation, where not represented in the current
  runtime instruction/account path
- watcher/model canonical encoding field-order vectors
- decimal string encoding vectors from the Stage 1 model
- reserved bytes `194..208` as semantic sourceChainId
- live SPL mint success path
- rollback after live SPL CPI failure
- full Stage 1 invalid-vector runtime completion
- production readiness
- final live atomicity

## Phase 17 / 18 / 19 Evidence Preserved

Phase 20 preserves:

- `instruction_reserved_bytes.rs`
- `consume_gateway_mint_accepts_nonzero_reserved_bytes_194_208_as_raw_only`
- `mollusk_valid_scaffold_entrypoint_leaves_mutable_accounts_unchanged`
- `direct_local_mutation_boundary_is_separate_from_enabled_entrypoint_noop`
- `disabled_spl_cpi_gate_rejects_before_live_atomicity_mutations`

Phase 20 does not reinterpret reserved bytes `194..208`.

Phase 20 does not treat reserved bytes `194..208` as sourceChainId.

Phase 20 preserves the Phase 19 live atomicity no-mutation boundary.

## Validation

Commands run:

    cd /mnt/c/Users/user/xenchanted-x1-build-lab/programs/xxxl-svm
    cargo fmt --check
    cargo test --test mollusk_consume_gateway_mint
    cargo test --test disabled_cpi_reachability
    cargo test --test instruction_reserved_bytes
    cargo test --lib

Results:

- `cargo fmt --check`: passed
- `cargo test --test mollusk_consume_gateway_mint`: 51 passed, 0 failed, 10 ignored
- `cargo test --test disabled_cpi_reachability`: 7 passed, 0 failed
- `cargo test --test instruction_reserved_bytes`: 1 passed, 0 failed
- `cargo test --lib`: 201 passed, 0 failed, 1 ignored
- `git diff --check`: passed

## Correct Statement

Correct:

- Phase 20 adds runtime-observable invalid-vector test evidence.
- Phase 20 separates runtime-observable classes from unrepresented Stage 1
  classes.
- Rejection paths covered by the new helper leave mutable accounts unchanged.
- Runtime source remains unchanged.
- Live route remains disabled.
- SPL CPI remains disabled.

Incorrect:

- Phase 20 completes all Stage 1 invalid-vector runtime coverage.
- Phase 20 validates watcher/model canonical encoding vectors.
- Phase 20 gives reserved bytes `194..208` semantic sourceChainId meaning.
- Phase 20 enables live route.
- Phase 20 enables SPL CPI.
- Phase 20 proves production readiness.
- Phase 20 proves final immutability.

## Safety Blocker Preservation

Current X1 status remains:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`

Active blockers remain:

- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

Phase 20 made no production runtime source changes.

Phase 20 did not deploy.

Phase 20 did not upgrade.

Phase 20 did not submit transactions.

Phase 20 did not spend SOL.

No blocker was removed.

No production readiness is claimed.

No final immutability is claimed while upgrade authority exists.

## Next Recommended Stage

Recommended next stage:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-21-runtime-coverage-audit-followup-boundary`

This future stage should be chosen only after external audit feedback on Phase 20.
