# Checkpoint: XXXL Runtime Deployment Blocker Descriptions

Stage: stage-xxxl-runtime-deployment-blocker-descriptions

Status: COMPLETED

## Goal

Add stable codes and human-readable descriptions for the XXXL runtime deployment status and deployment blockers.

## Completed

Added status methods:

- `XxxlRuntimeDeploymentStatus::code`
- `XxxlRuntimeDeploymentStatus::description`

Added blocker methods:

- `XxxlRuntimeDeploymentBlocker::code`
- `XxxlRuntimeDeploymentBlocker::description`

Added helper functions:

- `xxxl_runtime_deployment_status_code`
- `xxxl_runtime_deployment_status_description`

## Current status

Code:

- `SCAFFOLD_ONLY_NOT_DEPLOYABLE`

Description:

- `The XXXL SVM runtime is a scaffold-only build and is not deployable.`

## Current blocker codes

- `PLACEHOLDER_PROGRAM_ID`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Safety boundary

No live route was activated.

No SPL CPI behavior was enabled.

No `invoke_signed` path was enabled.

No minting was enabled.

No deployability predicate was changed.

The runtime remains scaffold-only and not deployable.

## Verification

Hard checks passed:

- cargo build-sbf
- cargo fmt --check
- cargo test
- cargo test --test mollusk_consume_gateway_mint -- --ignored --nocapture
- cargo clippy --all-targets -- -D warnings
- cargo audit
- cargo deny check licenses
- cargo deny check bans
- cargo deny check sources

## Decision

The runtime deployment blockers are now stable for both machine and human consumers.
