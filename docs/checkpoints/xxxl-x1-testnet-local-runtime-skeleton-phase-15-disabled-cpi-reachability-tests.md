# XXXL X1 Testnet Local Runtime Skeleton Phase 15 Disabled CPI Reachability Tests

Status: Test evidence checkpoint complete - all runtime blockers remain active.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-15-disabled-cpi-reachability-tests`

## Purpose

Phase 15 adds direct test evidence around the disabled CPI boundary.

The goal is to test that the current runtime skeleton does not accidentally
enable:

- SPL CPI execution
- `invoke_signed` execution from the process-instruction path
- SPL Token `mint_to` execution

This is not live-route readiness.

This is not production readiness.

This is not final immutability.

## Scope

Phase 15 changes only tests and checkpoint documentation.

Allowed changes:

- `programs/xxxl-svm/tests/disabled_cpi_reachability.rs`
- checkpoint documentation

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

## Test File Added

New test file:

- `programs/xxxl-svm/tests/disabled_cpi_reachability.rs`

The file adds 4 tests:

1. `spl_mint_to_cpi_execution_gate_is_disabled`
2. `guarded_mint_to_cpi_boundary_rejects_before_invoke_signed_when_global_gate_disabled`
3. `guarded_mint_to_cpi_boundary_rejects_live_execution_plan_flag`
4. `guarded_mint_to_cpi_boundary_rejects_invoke_signed_planning_flag`

## Test Evidence

The tests verify:

- `spl_mint_to_cpi_execution_enabled()` is false
- the guarded CPI boundary rejects with `CpiBoundaryNotReady`
- a live execution-plan flag is rejected
- an invoke-signed planning flag is rejected

The tests use the public guarded CPI boundary.

They do not enable live route execution.

They do not call `mint_to_cpi_boundary` directly.

They do not submit transactions.

They do not build or use an SBF deploy artifact.

## Validation

Command:

    cd /mnt/c/Users/user/xenchanted-x1-build-lab/programs/xxxl-svm
    cargo test --test disabled_cpi_reachability

Result:

- 4 tests total
- 4 passed
- 0 failed
- 0 ignored
- 0 measured
- 0 filtered out

Regression command:

    cd /mnt/c/Users/user/xenchanted-x1-build-lab/programs/xxxl-svm
    cargo test --test mollusk_consume_gateway_mint

Regression result:

- 53 tests total
- 43 passed
- 0 failed
- 10 ignored
- 0 measured
- 0 filtered out

The ignored Mollusk tests remain ignored with the existing reason:

- requires `cargo build-sbf` and `target/deploy/xxxl_svm.so`

## Current Disabled Semantics Preserved

Current `Ok(())` still means:

- validation succeeded
- disabled execution plan was built
- no live gateway success
- no XXXL mint success
- no Processed Event consumption
- no Recipient Balance credit
- no Mint State / supply accounting mutation
- no SPL CPI
- no `invoke_signed`
- no SPL Token `mint_to`

## Stage 1 / Runtime Boundary Preserved

Phase 15 preserves these boundaries:

- Stage 1 remains the deterministic authorization model
- runtime remains a consumer / mapping layer
- relayer-submitted SVM instruction data alone is not Stage 1 authorization
- Processed Event state is not Stage 1 authorization
- runtime replay identity remains `canonicalEventKey`
- `messageNonce` has no current runtime replay semantics
- persistent Stage 1 processed-burn tracking belongs to off-chain watcher /
  orchestrator / authorization-service boundary
- `burnedAmount == xxxlMintAmount` remains a Stage 1 responsibility because the
  current SVM instruction does not carry `burnedAmount` separately

## Gate Preservation

Phase 15 preserves these gates:

- bytes `194..208` remain reserved, unparsed, and not zero-validated
- the `u128` amount layout with `u64` SPL range remains a design gap
- dormant CPI helpers remain gated
- 10 ignored Mollusk tests remain an evidence gap
- current `Ok(())` remains disabled-plan no-op return
- live atomicity remains unimplemented
- `sourceChainId` runtime handling remains unresolved before live-route or
  SPL-CPI enablement
- source-chain ID must not be resolved silently through bytes `194..208`

## What Phase 15 Does Not Prove

Phase 15 does not prove:

- live-route readiness
- SPL CPI readiness
- production readiness
- deploy readiness
- external review completion
- live atomicity
- rollback behavior after SPL CPI failure
- rollback behavior after local state mutation failure
- persistent Stage 1 processed-burn storage
- source-chain ID binding
- `messageNonce` runtime replay semantics
- invalid-vector e2e coverage
- removal of the 10 ignored Mollusk tests

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

Phase 15 made no runtime code changes.

Phase 15 did not deploy.

Phase 15 did not upgrade.

Phase 15 did not submit transactions.

Phase 15 did not spend SOL.

No blocker was removed.

No production readiness is claimed.

No final immutability is claimed while upgrade authority exists.

## Next Recommended Stage

Recommended next stage:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-16-local-mutation-reachability-boundary`

That future stage should clarify that the direct-call local mutation boundary
can mutate local state, while the currently enabled `process_instruction` path
does not reach it.
