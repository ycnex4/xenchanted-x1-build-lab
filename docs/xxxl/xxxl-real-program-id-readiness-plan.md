# XXXL Real Program ID Readiness Plan

Status: Completed
Branch: `stage-xxxl-real-program-id-readiness-plan`
Base: `a48548a Add XXXL Program ID transition assessment`

## Purpose

This document defines the readiness plan for a future real Program ID transition.

This is a planning stage only.

This stage does not select a real Program ID.

This stage does not replace the placeholder Program ID.

This stage does not regenerate production PDA fixtures.

This stage does not verify production PDA fixtures.

This stage does not remove `PLACEHOLDER_PROGRAM_ID`.

This stage does not remove any deployment blocker.

This stage does not make the runtime deployable.

## Current State

The current runtime still exposes a placeholder Program ID boundary.

The current placeholder Program ID constant is:

- `XXXL_PROGRAM_ID_PLACEHOLDER`
- `XXXLProgram111111111111111111111111111111111`

The current Program ID readiness status remains:

- `Placeholder`

The current Program ID readiness status code remains:

- `PLACEHOLDER_PROGRAM_ID_BOUNDARY`

The current Program ID deployable path status remains:

- `false`

The linked deployment blocker remains:

- `PLACEHOLDER_PROGRAM_ID`

## PDA Dependency State

The current PDA inventory contains one Program-ID-dependent PDA:

- `gateway_mint_authority`

Its seeds are:

- `xxxl`
- `gateway-mint-authority`
- `v1`

Its purpose is to act as the SPL Token mint authority for gateway-backed XXXL minting.

The PDA derivation depends on Program ID.

Therefore, production PDA fixtures must be regenerated after the final real Program ID is selected.

No production PDA fixture should be considered final before final Program ID selection.

## Readiness Goal

The goal of the future real Program ID transition is to make the Program ID boundary ready while preserving all other deployment safety boundaries.

A future transition may remove `PLACEHOLDER_PROGRAM_ID` only after there is evidence that:

- final real Program ID is selected
- Program ID readiness status no longer reports `Placeholder`
- Program ID deployable path is ready for this specific boundary
- production PDA fixtures are regenerated from the final Program ID
- production PDA fixtures are verified
- safety invariants reflect real Program ID selected
- safety invariants reflect production PDA fixtures verified
- active deployment report no longer includes `PLACEHOLDER_PROGRAM_ID`
- all non-Program-ID blockers remain active
- runtime remains not deployable

## Required Future Work

A future implementation stage must define or update the Program ID readiness model.

The readiness model must distinguish at least:

1. placeholder Program ID state
2. real Program ID selected state
3. production PDA fixtures regenerated state
4. production PDA fixtures verified state

A future transition stage must not collapse these steps unless the evidence is explicit and reviewed.

## Required Future Evidence

A future `PLACEHOLDER_PROGRAM_ID` transition requires all of the following evidence.

### 1. Real Program ID Selection Evidence

The repository must record the final real Program ID.

The evidence must include:

- exact Program ID string
- source of the Program ID decision
- branch and commit where it was introduced
- confirmation that it is not the placeholder value
- confirmation that it is not a local-only fixture value
- confirmation that all Program-ID-dependent derivations use this value

### 2. Program ID Readiness Evidence

The Program ID readiness report must be updated so that:

- status is no longer `Placeholder`
- status code is no longer `PLACEHOLDER_PROGRAM_ID_BOUNDARY`
- configured Program ID is the final real Program ID
- deployable path ready for this boundary is true
- linked blocker no longer requires `PLACEHOLDER_PROGRAM_ID`

This must be proven by focused tests.

### 3. PDA Fixture Regeneration Evidence

Production PDA fixture reports must be regenerated from the final real Program ID.

The generated fixture report must include:

- PDA kind
- PDA name
- input Program ID
- derived PDA
- bump
- seed list or reference to inventory
- derivation method

The current known PDA inventory entry is:

- `gateway_mint_authority`

A future stage must regenerate this fixture from the final real Program ID.

### 4. PDA Fixture Verification Evidence

The verification path must accept the regenerated production PDA fixture report.

It must reject:

- wrong report count
- wrong PDA kind
- wrong PDA name
- wrong Program ID
- wrong PDA
- wrong bump

The verification evidence must be committed and reviewed before removing `PLACEHOLDER_PROGRAM_ID`.

### 5. Safety Invariant Evidence

Safety invariants must be updated to reflect:

- real Program ID selected: true
- production PDA fixtures verified: true
- Program ID placeholder boundary active: false
- Program ID placeholder blocker active in deployment report: false

The same stage must prove:

- live route remains disabled unless separately transitioned
- SPL CPI execution remains disabled unless separately transitioned
- production guardian set remains unset unless separately transitioned
- production proof log remains unset unless separately transitioned
- external review remains incomplete unless separately transitioned
- runtime remains not deployable

### 6. Deployment Blocker Evidence

The deployment blocker report may remove `PLACEHOLDER_PROGRAM_ID` only after the above evidence exists.

After removal, the active blocker list should still include:

- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

The active blocker count would become 5 only if no other blocker is added.

This future count must be proven by tests.

## Required Future Tests

A future transition stage should include focused tests proving:

- placeholder Program ID constant is not used as active configured Program ID
- real Program ID is recorded
- Program ID readiness status is not `Placeholder`
- Program ID readiness report uses the final real Program ID
- Program ID deployable path is ready for this boundary
- `PLACEHOLDER_PROGRAM_ID` is absent from active deployment blocker reports
- all remaining blockers are still active
- PDA fixture report derives from the final real Program ID
- PDA fixture verification accepts the production fixture
- PDA fixture verification rejects wrong Program ID
- PDA fixture verification rejects wrong PDA
- PDA fixture verification rejects wrong bump
- safety invariants reflect real Program ID selected
- safety invariants reflect production PDA fixtures verified
- runtime remains not deployable
- predeploy gate remains blocked

## Required Future Documentation

A future transition stage should add:

- real Program ID selection record
- production PDA fixture derivation report
- production PDA fixture verification report
- Program ID blocker transition document
- checkpoint for the transition
- current design checkpoint update

## Non-Goals

This plan does not authorize:

- deployment
- live route activation
- SPL CPI execution
- `invoke_signed`
- SPL Token `mint_to`
- guardian production configuration
- proof-log production configuration
- external review closure
- runtime release

## Current Blocker Status

`PLACEHOLDER_PROGRAM_ID` remains active.

The current active deployment blockers remain:

- `PLACEHOLDER_PROGRAM_ID`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker is removed by this plan.

No blocker is transitioned by this plan.

## Recommended Future Stage Sequence

Recommended sequence:

1. `stage-xxxl-real-program-id-selection-record`
2. `stage-xxxl-production-pda-fixture-regeneration`
3. `stage-xxxl-production-pda-fixture-verification`
4. `stage-xxxl-program-id-readiness-model-update`
5. `stage-xxxl-placeholder-program-id-blocker-transition`

The transition stage should happen only after the earlier evidence stages are complete.

## Result

This stage defines the plan required before `PLACEHOLDER_PROGRAM_ID` can be transitioned.

It does not perform the transition.

The runtime remains scaffold-only, locked, unreleasable, and not deployable.
