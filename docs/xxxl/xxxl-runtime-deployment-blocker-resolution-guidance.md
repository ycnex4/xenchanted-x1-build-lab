# XXXL Runtime Deployment Blocker Resolution Guidance

Status: COMPLETED.

This stage adds resolution guidance to each XXXL runtime deployment blocker.

## Goal

Previous stages made deployment status and deployment blockers explicit and machine-readable.

This stage adds the missing operational question:

What must happen before each blocker can be removed?

## What changed

Extended:

- `XxxlRuntimeDeploymentBlockerReport`

Added field:

- `resolution`

Added method:

- `XxxlRuntimeDeploymentBlocker::resolution`

Each blocker report now includes:

- blocker enum value
- stable code
- human-readable description
- human-readable resolution guidance

## Current blocker resolution guidance

### PLACEHOLDER_PROGRAM_ID

Set and review the real Program ID and regenerate all Program-ID-dependent PDA fixtures.

### LIVE_ROUTE_DISABLED

Activate the live route only in a reviewed stage after all deployment blockers are resolved.

### SPL_CPI_EXECUTION_DISABLED

Enable SPL Token mint_to CPI execution only after live route activation, PDA authority, account contract, and Mollusk coverage are complete.

### PRODUCTION_GUARDIAN_SET_UNSET

Define, publish, and review the production guardian set, threshold, rotation policy, and key custody model.

### PRODUCTION_PROOF_LOG_UNSET

Define the production proof-log format, retention policy, public audit trail, and operator publication flow.

### EXTERNAL_REVIEW_INCOMPLETE

Complete external review of the live route, guardian policy, CPI path, account contract, replay protection, and deployment checklist.

## Intended future use

This resolution guidance can be used later by:

- deployment readiness checklists
- predeploy status scripts
- CLI status output
- README/status rendering
- UI deployment panels
- external review handoff notes

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

The deployment blocker resolution guidance boundary is accepted.

Each deployment blocker now explains both why it blocks deployment and what must happen before it can be removed.
