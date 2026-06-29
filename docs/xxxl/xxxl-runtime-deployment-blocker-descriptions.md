# XXXL Runtime Deployment Blocker Descriptions

Status: COMPLETED.

This stage adds stable machine-readable codes and human-readable descriptions for the XXXL runtime deployment status and deployment blockers.

## Goal

The previous stage made the runtime explicitly nondeployable.

This stage makes that status easier to surface in future tooling, docs, scripts, UI, or predeploy checks.

## What changed

The deployment status module now exposes:

- stable status code
- human-readable status description
- stable blocker codes
- human-readable blocker descriptions

Added methods:

- `XxxlRuntimeDeploymentStatus::code`
- `XxxlRuntimeDeploymentStatus::description`
- `XxxlRuntimeDeploymentBlocker::code`
- `XxxlRuntimeDeploymentBlocker::description`

Added helpers:

- `xxxl_runtime_deployment_status_code`
- `xxxl_runtime_deployment_status_description`

## Stable status code

Current status code:

    SCAFFOLD_ONLY_NOT_DEPLOYABLE

Current description:

    The XXXL SVM runtime is a scaffold-only build and is not deployable.

## Stable blocker codes

Current blocker codes:

- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

Current non-blocker X1 testnet status:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`

Retired X1 testnet blanket blocker:

- `PLACEHOLDER_PROGRAM_ID`

## Human-readable blocker descriptions

Each blocker now has a stable human-readable explanation.

This is useful for:

- future README rendering
- future CLI status output
- future predeploy scripts
- future UI panels
- future deployment checklists

## Safety boundary

No live route was activated.

No SPL CPI behavior was enabled.

No `invoke_signed` path was enabled.

No minting was enabled.

No deployability predicate was changed.

The runtime remains scaffold-only and not deployable.

## Verification

Hard checks passed:

- `cargo build-sbf`
- `cargo fmt --check`
- `cargo test`
- `cargo test --test mollusk_consume_gateway_mint -- --ignored --nocapture`
- `cargo clippy --all-targets -- -D warnings`
- `cargo audit`
- `cargo deny check licenses`
- `cargo deny check bans`
- `cargo deny check sources`

## Decision

The deployment blocker description boundary is accepted.

Deployment blockers are now both machine-readable and human-readable.
