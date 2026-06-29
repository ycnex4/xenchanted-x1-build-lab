# XXXL Program ID Transition Assessment

Status: Completed
Branch: `stage-xxxl-program-id-transition-assessment`
Base: `88fc20d Transition XXXL Mollusk coverage blocker`

## Purpose

This document assesses whether the `PLACEHOLDER_PROGRAM_ID` deployment blocker is ready for transition.

This is an assessment stage only.

This stage does not transition `PLACEHOLDER_PROGRAM_ID`.

This stage does not remove any deployment blocker.

This stage does not select a real Program ID.

This stage does not regenerate production PDA fixtures.

This stage does not make the runtime deployable.

## Assessment Decision

`PLACEHOLDER_PROGRAM_ID` is not ready to transition.

The blocker must remain active.

The runtime still exposes a placeholder Program ID boundary.

A real Program ID has not been selected.

Production PDA fixtures have not been regenerated.

Production PDA fixtures have not been verified against a final Program ID.

## Evidence Reviewed

The assessment reviewed the current Program ID and PDA boundary state in:

- `programs/xxxl-svm/src/lib.rs`
- `programs/xxxl-svm/src/program_id_status.rs`
- `programs/xxxl-svm/src/pda.rs`
- `programs/xxxl-svm/src/deployment_status.rs`
- `programs/xxxl-svm/src/safety_invariants.rs`

## Current Program ID State

The runtime still exposes:

- `XXXL_PROGRAM_ID_PLACEHOLDER`

The configured placeholder value is:

- `XXXLProgram111111111111111111111111111111111`

The runtime documentation in code still states that the Program ID is intentionally kept as a placeholder boundary.

The code also states that real Program ID, real PDA fixture, real decode fixtures, and real SPL Token CPI fixtures must be completed before deployment.

## Program ID Readiness State

`program_id_status.rs` currently has only one readiness status:

- `Placeholder`

The current Program ID readiness report states:

- status code: `PLACEHOLDER_PROGRAM_ID_BOUNDARY`
- configured Program ID: `XXXL_PROGRAM_ID_PLACEHOLDER`
- deployable path ready: `false`
- linked blocker: `PLACEHOLDER_PROGRAM_ID`

The helper `xxxl_program_id_placeholder_boundary_is_active()` remains true.

The helper `xxxl_program_id_deployable_path_ready()` remains false.

The helper `xxxl_program_id_placeholder_blocker_is_active_in_deployment_report()` remains true.

## PDA Dependency State

The PDA inventory currently contains:

- `gateway_mint_authority`

The PDA inventory explicitly records:

- seed count: 3
- depends on Program ID: true
- purpose: SPL Token mint authority for gateway-backed XXXL minting

The PDA derivation functions derive PDA fixtures from a supplied Program ID.

The current tests prove that PDA fixtures change when Program ID changes.

This means that a final Program ID decision must happen before production PDA fixture regeneration and verification can be considered complete.

## Safety Invariant State

The safety invariant layer still records:

- Program ID placeholder boundary active
- Program ID placeholder blocker active in deployment report
- real Program ID selected: false
- production PDA fixtures verified: false

The safety unlock criteria require both:

- real Program ID selected
- production PDA fixtures verified

Those criteria are currently false.

Therefore the safety layer correctly prevents Program ID transition and runtime unlock.

## Why Transition Is Not Allowed Yet

`PLACEHOLDER_PROGRAM_ID` cannot be transitioned in this stage because:

1. no real Program ID has been selected
2. the placeholder Program ID constant remains active
3. Program ID readiness status is still `Placeholder`
4. deployable path ready remains false
5. placeholder blocker remains active in deployment report
6. PDA fixtures depend on the final Program ID
7. production PDA fixtures have not been regenerated
8. production PDA fixtures have not been verified
9. safety invariants still require the placeholder blocker to remain active
10. runtime remains scaffold-only, locked, unreleasable, and not deployable

## Required Future Evidence Before Transition

A future transition of `PLACEHOLDER_PROGRAM_ID` requires a separate evidence package proving:

- final real Program ID selected
- placeholder Program ID constant replaced or moved out of active runtime readiness state
- Program ID readiness status updated from `Placeholder` to a reviewed real-program-id state
- deployable path remains blocked by other blockers, but no longer by Program ID placeholder
- production PDA fixtures regenerated from the final Program ID
- PDA fixture verification accepts the regenerated production fixtures
- PDA fixture verification rejects mismatched Program ID, PDA, bump, name, kind, and report count
- safety invariants updated to reflect real Program ID selected
- safety invariants updated to reflect production PDA fixtures verified
- `PLACEHOLDER_PROGRAM_ID` absent from active deployment blocker reports
- all other blockers remain active
- runtime remains not deployable

## Non-Changes

This stage does not change:

- Rust source
- Cargo files
- tests
- Program ID constants
- PDA derivation logic
- PDA fixtures
- deployment status logic
- safety invariant logic
- live route execution
- SPL CPI execution
- `invoke_signed`
- SPL Token `mint_to`

## Blocker Status

`PLACEHOLDER_PROGRAM_ID` remains active.

Remaining active blockers are:

- `PLACEHOLDER_PROGRAM_ID`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker is removed.

No blocker is transitioned.

## Recommended Next Stage

The next Program-ID-related stage should not be a blocker transition.

The recommended next stage is:

`stage-xxxl-real-program-id-readiness-plan`

That stage should define the exact plan for:

- selecting the final real Program ID
- updating Program ID readiness state
- regenerating production PDA fixtures
- verifying generated PDA fixtures
- preserving all non-Program-ID deployment blockers

Only after that plan and evidence are complete should a future `PLACEHOLDER_PROGRAM_ID` blocker transition be considered.
