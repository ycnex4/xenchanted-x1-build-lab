# XXXL X1 Testnet Local Runtime Skeleton Phase 21 SBF Artifact Mollusk Revalidation Boundary

Status: Local ignored SBF artifact refresh and Mollusk revalidation boundary.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-21-sbf-artifact-mollusk-revalidation-boundary`

Base:

- `417eea9 Merge XXXL phase 21 source chain id binding`

## Purpose

This boundary resolves the Phase 21 Mollusk validation blocker caused by a stale
local SBF artifact.

After Phase 21, Rust source and native tests used instruction layout version `2`,
but the local ignored Mollusk artifact at:

- `programs/xxxl-svm/target/deploy/xxxl_svm.so`

was still built from the previous layout version and rejected v2 instructions with
`InvalidVersion`.

This boundary refreshes only the local ignored SBF artifact used by Mollusk and
reruns Mollusk validation against the Phase 21 v2 runtime.

## Scope

In scope:

- build a fresh local SBF artifact for test validation
- place only the refreshed `xxxl_svm.so` into the existing ignored
  `programs/xxxl-svm/target/deploy/` path
- run `cargo test --test mollusk_consume_gateway_mint`
- rerun the Phase 21 core green suite
- document the boundary

Out of scope:

- deploy
- upgrade
- transaction submission
- SOL spend
- network actions
- live route activation
- SPL CPI activation
- `invoke_signed` activation
- SPL Token `mint_to` execution
- tracked artifact commits
- keypair read/copy/modify
- production readiness
- final immutability claim

## Artifact Handling

The target deploy directory remains ignored by git through:

- `programs/xxxl-svm/.gitignore`
- `/target/`

The following files are not tracked by git:

- `programs/xxxl-svm/target/deploy/xxxl_svm.so`
- `programs/xxxl-svm/target/deploy/xxxl_svm-keypair.json`

The local SBF build was emitted to a temporary directory:

- `/tmp/xxxl-svm-phase21-sbf-out`

Only this file was copied into the ignored Mollusk artifact path:

- `/tmp/xxxl-svm-phase21-sbf-out/xxxl_svm.so`
- to `programs/xxxl-svm/target/deploy/xxxl_svm.so`

The temporary build directory was removed after validation.

The temporary SBF build also produced a temporary keypair file, but it was not
read, copied, committed, or used.

The existing local target deploy keypair was not read, copied, modified, or
committed.

Only metadata checks were performed for the local keypair path.

## Artifact Evidence

Before refresh:

- local ignored `xxxl_svm.so`: `38584` bytes

After refresh:

- local ignored `xxxl_svm.so`: `39072` bytes

Existing local target deploy keypair metadata remained:

- `xxxl_svm-keypair.json`: `225` bytes

The keypair file content was not read.

## Validation

The local SBF refresh command completed:

- `cargo build-sbf --sbf-out-dir /tmp/xxxl-svm-phase21-sbf-out`: passed

Mollusk v2 revalidation:

- `cargo test --test mollusk_consume_gateway_mint`: 55 passed, 0 failed, 10 ignored

Core green suite:

- `cargo fmt --check`: passed
- `cargo test --test disabled_cpi_reachability`: 7 passed, 0 failed
- `cargo test --test instruction_reserved_bytes`: 3 passed, 0 failed
- `cargo test --lib`: 211 passed, 0 failed, 1 ignored

Hygiene:

- `target/deploy` remains ignored
- no tracked `.so` artifact was added
- no tracked keypair file was added
- no deploy script was changed
- no upgrade script was changed
- no Cargo file was changed

## Safety Invariants

Live route remains disabled.

SPL CPI remains disabled.

The enabled `process_instruction` path remains a disabled-plan no-op for live
atomicity.

This boundary does not enable `invoke_signed`.

This boundary does not enable SPL Token `mint_to`.

This boundary does not mutate ProcessedEvent from the enabled entrypoint path.

This boundary does not credit RecipientBalance from the enabled entrypoint path.

This boundary does not mutate SPL mint supply from the enabled entrypoint path.

This boundary does not mutate recipient SPL token account amount from the enabled
entrypoint path.

GatewayConfig layout remains unchanged.

No production readiness is claimed.

No final immutability is claimed while upgrade authority exists.

## Current X1 Status

Current X1 status remains:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`

Active blockers remain:

- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker was removed.

No production readiness is claimed.

No final immutability is claimed while upgrade authority exists.

## Result

The stale Mollusk SBF artifact blocker from Phase 21 is resolved locally for the
current ignored test artifact.

Phase 21 v2 Mollusk validation is now green.

No deployable artifact is committed.

No deployment action was performed.

No production status changed.

Recommended next stage:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-22-guardian-payload-structure-boundary`
