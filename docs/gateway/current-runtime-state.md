# Canonical Runtime State

This file is the single source of truth for the current gateway/runtime state.

Historical package documents, evidence folders, review packets, and chat context are not canonical if they conflict with this file.

Every package that changes or reclassifies project state must update this file in the same closure commit, or explicitly state that no runtime-state update is required.

If source/docs/context disagree, source is the runtime ground truth, and this ledger is the coordination ground truth after reconciliation.

## Ledger Metadata

- reconciled_source_base_commit: 65a8e83
- last_updated_package: external-review-blocker-removal-source-change
- ledger_author: Sergey Stepanenko / ChatGPT assisted
- package_scope: DOCUMENTATION_ONLY
- next_required_action: continue with program-id-binding-source-change approval request

## Authorization Boundary

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

## State Model Rules

1. This file is canonical for coordination.
2. Source code remains the ground truth for runtime behavior.
3. Review approval is not the same as source blocker removal.
4. Exactly one work lane should be marked ACTIVE at a time.
5. Future packages must update this file when they change or reclassify project state.
6. If docs/context/source diverge, feature work stops until reconciliation is documented.
7. Package history may have numbered folders, but canonical state must remain in this single file.

## Active Lanes

| Lane | Status | Blocking Condition | Next Step |
|:---|:---|:---|:---|
| gateway-state-reconciliation | CLOSED | reconciliation package complete | no further action in this lane |
| program-id-binding | ACTIVE | PlaceholderProgramId | continue with program-id-binding-source-change approval request |
| guardian-proof-log | PAUSED | ProductionGuardianSetUnset, ProductionProofLogUnset | future instantiation package |
| local-only-fixtures | NO-GO | local-only fixture emission not approved | separate future GO required |

## Source Blockers

Ground truth from current main/source state.

| Blocker | Source Status | Review Status | Effective Status |
|:---|:---|:---|:---|
| PlaceholderProgramId | ACTIVE | N/A | ACTIVE |
| LiveRouteDisabled | ACTIVE | N/A | ACTIVE |
| SplCpiExecutionDisabled | ACTIVE | N/A | ACTIVE |
| ProductionGuardianSetUnset | ACTIVE | policy_defined_not_instantiated | ACTIVE |
| ProductionProofLogUnset | ACTIVE | policy_defined_not_instantiated | ACTIVE |
| ExternalReviewIncomplete | REMOVED_FROM_ACTIVE_SOURCE_BLOCKERS | review_approved_source_changed | REMOVED |

## Review Approval vs Source Removal

| Item | Review Approved | Source Blocker Removed | Effective Runtime Status |
|:---|:---:|:---:|:---|
| ExternalReviewIncomplete | true | true | REMOVED |
| ProductionGuardianSetUnset | policy_defined | false | ACTIVE |
| ProductionProofLogUnset | policy_defined | false | ACTIVE |

## Safety Invariants

| Invariant | Value | Source Location |
|:---|:---|:---|
| program_id_placeholder_binding | active | programs/xxxl-svm/src/lib.rs::XXXL_PROGRAM_ID_PLACEHOLDER |
| program_id_placeholder_blocker | active | programs/xxxl-svm/src/program_id_status.rs |
| live_route_activation | disabled | programs/xxxl-svm/src/processor.rs::LIVE_ROUTE_ACTIVATION_FROM_PROCESS_INSTRUCTION_ENABLED |
| spl_cpi_execution | disabled | programs/xxxl-svm/src/cpi.rs |
| runtime_deployable | false | programs/xxxl-svm/src/deployment_status.rs |
| predeploy_gate | blocked | programs/xxxl-svm/src/deployment_status.rs |
| safety_lock | active | programs/xxxl-svm/src/safety_invariants.rs |

## Doc/Source Sync Status

| Document | Status | Required Action |
|:---|:---|:---|
| docs/gateway/current-runtime-state.md | CANONICAL | keep updated in future state-changing package closures |
| docs/checkpoints/current-design-checkpoint.md | DIVERGENT | update during this reconciliation package |
| docs/gateway/guardian-set-public-policy-1.md | CONSISTENT_AS_POLICY | no source instantiation yet |
| docs/gateway/proof-log-public-policy-1.md | CONSISTENT_AS_POLICY | no source instantiation yet |
| docs/gateway/external-review-packet-1.md | REVIEW_APPROVED | blocker still active in source |
| docs/gateway/phase-41k6-b6-57-local-only-fixture-emission-go-form.md | NO_GO_LANE | do not mix with Program ID binding |

## State Drift Discovered Before Program ID Binding Plan

- ExternalReviewIncomplete drift resolved by external-review-blocker-removal-source-change: source active blocker removed and tests now expect it inactive.
- current-design-checkpoint.md is stale relative to later B6.55/B6.56/B6.57 documents and guardian/proof-log policy packages.
- Program ID binding planning must inventory actual main/source state, not assumed context state.
- Program ID source-change must wait until this reconciliation package is closed.

## Pending Reconciliations

1. ExternalReviewIncomplete:
   - review_approved=true
   - source_blocker_removed=true
   - effective_blocker_status=REMOVED
   - review_integrity_condition=removal_does_not_preapprove_subsequent_program_id_binding

2. current-design-checkpoint.md:
   - status=DIVERGENT
   - future action inside this package: update checkpoint to reference current-runtime-state.md as canonical

3. Program ID binding:
   - status=PAUSED
   - current action: review program-id-binding-resolution-plan and choose next approved package

4. Local-only fixture lane:
   - status=NO-GO
   - future action: do not emit fixtures unless separate explicit GO is received

## Required Before Next Source Change

- [x] current-runtime-state.md exists and matches main/source state
- [x] current-design-checkpoint.md updated to stop acting as stale current truth
- [x] all known divergences documented with reconciliation plans
- [x] exactly one lane marked ACTIVE
- [x] no source blocker removed by this package
- [x] no activation, deploy, route enablement, RPC mutation, or upgrade performed

## Next Required Decision

Review docs/gateway/program-id-binding-resolution-plan.md and choose the next approved package.

Selected next package for approval request:

- external-review-blocker-removal-source-change

program-id-binding-source-change is the next approval-request candidate.

Neither is authorized by this ledger.

## Forbidden Next Actions Without Separate Explicit Approval

- source changes
- blocker removal
- Program ID source binding
- additional blocker removal beyond ExternalReviewIncomplete
- live route enablement
- SPL CPI execution enablement
- deployment
- upgrade
- RPC mutation
- local-only fixture emission
