# Current Design Checkpoint

Status: RECONCILED_BY_CANONICAL_RUNTIME_STATE_LEDGER
Last updated package: gateway-state-reconciliation-1
Last updated commit: 65a8e83
Scope: DOCUMENTATION_ONLY

## Canonical State Source

The canonical current gateway/runtime state is now maintained in:

docs/gateway/current-runtime-state.md

This checkpoint is no longer the primary runtime-state ledger. It is a human-readable checkpoint that points to the canonical state ledger.

If this checkpoint conflicts with docs/gateway/current-runtime-state.md, the canonical runtime state ledger wins for coordination.

If source code conflicts with documentation, source code remains the runtime ground truth until reconciliation is documented.

## Why This Checkpoint Was Updated

A project-state drift was discovered before continuing with program-id-binding-resolution-plan.

The drift was structural, not an execution failure:

- working context treated ExternalReviewIncomplete as closed;
- source still treated ExternalReviewIncomplete as an active blocker;
- tests still expected EXTERNAL_REVIEW_INCOMPLETE as active;
- this checkpoint had become stale relative to later B6.55/B6.56/B6.57 documents;
- multiple work lanes were visible at once without one canonical active-lane record.

Theo reviewed the issue and approved a documentation-only reconciliation package:

APPROVE_GATEWAY_STATE_RECONCILIATION_1_DOCUMENTATION_ONLY

## Latest State Reconciliation Package

gateway-state-reconciliation-1

Purpose:

- create docs/gateway/current-runtime-state.md;
- record the real source-level blocker state from main;
- distinguish review approval from source blocker removal;
- make the runtime-state ledger the single coordination truth;
- update this checkpoint so it no longer acts as stale current truth.

## Current Authorization Boundary

| Action | Authorized |
|:---|:---:|
| source_change | false |
| blocker_removal | false |
| activation | false |
| rpc_mutation | false |
| deploy | false |
| upgrade | false |
| route_enablement | false |
| program_id_binding_source_change | false |
| external_review_blocker_removal | false |
| local_only_fixture_emission | false |

## Current Effective Source Blockers

The following blockers remain effective according to current source/runtime state:

| Blocker | Effective Status |
|:---|:---|
| PlaceholderProgramId | ACTIVE |
| LiveRouteDisabled | ACTIVE |
| SplCpiExecutionDisabled | ACTIVE |
| ProductionGuardianSetUnset | ACTIVE |
| ProductionProofLogUnset | ACTIVE |
| ExternalReviewIncomplete | ACTIVE |

Important distinction:

ExternalReviewIncomplete has review approval, but source blocker removal has not happened.

Therefore:

review_approved=true
source_blocker_removed=false
effective_blocker_status=ACTIVE

## Current Work Lanes

| Lane | Status | Notes |
|:---|:---|:---|
| gateway-state-reconciliation | CLOSED | reconciliation complete |
| program-id-binding | ACTIVE | next allowed package: program-id-binding-resolution-plan |
| guardian-proof-log | PAUSED | policy defined, production instantiation not done |
| local-only-fixtures | NO-GO | separate explicit GO required |

Exactly one lane should be ACTIVE at a time.

## Current Runtime Safety Summary

| Invariant | Current Value |
|:---|:---|
| Program ID placeholder binding | active |
| live route activation | disabled |
| SPL CPI execution | disabled |
| runtime deployable | false |
| predeploy gate | blocked |
| activation | blocked |

## State Management Rule Going Forward

Every future package that changes or reclassifies project state must update:

docs/gateway/current-runtime-state.md

in the same closure commit, or explicitly state that no runtime-state update is required.

A package closure is incomplete if it changes project state but does not update the canonical runtime state ledger.

## Next Allowed Package After Reconciliation

program-id-binding-resolution-plan

Scope remains PLANNING_ONLY.

The Program ID binding plan must include the discovered state drift in its inventory and must use actual main/source state as its base.

## Forbidden Without Separate Explicit Approval

- source changes;
- blocker removal;
- Program ID source binding;
- ExternalReviewIncomplete blocker removal;
- live route enablement;
- SPL CPI execution enablement;
- deployment;
- upgrade;
- RPC mutation;
- local-only fixture emission.
