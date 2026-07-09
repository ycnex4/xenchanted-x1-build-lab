# Current Execution Plan

This document defines the current execution roadmap for the gateway/runtime work.

It is not the canonical runtime-state ledger.

Canonical runtime state is maintained in:

docs/gateway/current-runtime-state.md

If this document conflicts with docs/gateway/current-runtime-state.md, the canonical runtime-state ledger wins for coordination.

If source code conflicts with documentation, source code remains the runtime ground truth until reconciliation is documented.

## Roadmap Metadata

- roadmap_package: gateway-execution-roadmap-1
- roadmap_base_commit: e05de82
- package_scope: DOCUMENTATION_ONLY
- runtime_state_update_required: false
- reason: this roadmap defines order, goals, gates, and no-go boundaries; it does not reclassify runtime state

## Current Position

The canonical runtime-state ledger currently identifies:

- active runtime lane: program-id-binding
- next runtime package: program-id-binding-resolution-plan
- next runtime package scope: PLANNING_ONLY
- ExternalReviewIncomplete effective status: REMOVED
- source_blocker_removed=false
- activation_authorized=false
- deploy_authorized=false
- route_enablement_authorized=false
- rpc_mutation_authorized=false
- source_change_authorized=false

This roadmap package does not change those values.

## Final Goal

Reach an activation-ready gateway/runtime state where:

- real Program ID binding has been selected, reviewed, and reflected in all Program-ID-dependent surfaces;
- Program-ID-dependent PDA evidence has been regenerated and reviewed;
- source blocker state is explicit, current, and consistent with tests;
- ExternalReviewIncomplete was removed only by the approved dedicated source-change package;
- guardian set policy is not only defined but instantiated through an approved package;
- proof log policy is not only defined but instantiated through an approved package;
- live route gate has been reviewed and only changes through a dedicated package;
- SPL CPI execution gate has been reviewed and only changes through a dedicated package;
- local-only and local-validator evidence exists before any live route/deploy/activation step;
- every state-changing package updates docs/gateway/current-runtime-state.md in the same closure commit;
- final activation/deploy/route/RPC mutation remains impossible without separate exact GO.

Activation-ready does not mean activated.

## Non-Goals

This roadmap does not authorize:

- source changes;
- blocker removal;
- Program ID source binding;
- additional blocker removal beyond ExternalReviewIncomplete;
- live route enablement;
- SPL CPI execution enablement;
- deployment;
- upgrade;
- RPC mutation;
- local-only fixture emission;
- testnet transaction submission;
- activation.

## Operating Rules

1. One active runtime lane at a time.
2. One package changes only one axis of project state.
3. Review approval is not blocker removal.
4. Documentation approval is not source authorization.
5. Source blocker removal requires a dedicated source-change package.
6. Program ID binding source changes require a dedicated source-change package.
7. Activation, deployment, route enablement, SPL CPI enablement, and RPC mutation each require separate explicit approval.
8. Every future state-changing package must update docs/gateway/current-runtime-state.md in the same closure commit.
9. If docs/context/source diverge, feature work stops and reconciliation comes first.
10. Historical package docs and evidence are not canonical if they conflict with docs/gateway/current-runtime-state.md.

## Default Execution Sequence

This is the intended order unless explicitly changed by a later approved roadmap update.

| Step | Package / Lane | Scope | Purpose | Exit Gate |
|:---:|:---|:---|:---|:---|
| 0 | gateway-state-reconciliation-1 | DOCUMENTATION_ONLY | Create canonical runtime-state ledger and repair stale checkpoint drift | complete on main |
| 1 | gateway-execution-roadmap-1 | DOCUMENTATION_ONLY | Define final goal, package order, gates, and no-go boundaries | this roadmap merged |
| 2 | program-id-binding-resolution-plan | PLANNING_ONLY | Inventory Program ID binding surfaces, PDA dependencies, source/doc drift, and future source-change targets/non-targets | Theo/reviewer approval for next source-change path |
| 3 | external-review-blocker-removal-source-change | SOURCE_CHANGE_NO_ACTIVATION | Remove ExternalReviewIncomplete only if explicitly approved; update tests and safety expectations | source blocker removed, no activation |
| 4 | program-id-binding-source-change | SOURCE_CHANGE_NO_ACTIVATION | Replace placeholder Program ID model with reviewed real Program ID binding path | PlaceholderProgramId state updated only as approved |
| 5 | program-id-dependent-pda-evidence | DOCUMENTATION_ONLY or LOCAL_ONLY_GENERATION | Regenerate/review Program-ID-dependent PDA evidence | evidence reviewed, no live action |
| 6 | guardian-set-instantiation-plan | PLANNING_ONLY | Define production guardian set instantiation requirements | approval for source/config package if needed |
| 7 | proof-log-instantiation-plan | PLANNING_ONLY | Define production proof log instantiation requirements | approval for source/config package if needed |
| 8 | live-route-readiness-plan | PLANNING_ONLY | Review live route activation boundary without enabling route | approval for future route package only |
| 9 | spl-cpi-readiness-plan | PLANNING_ONLY | Review SPL CPI execution boundary without enabling CPI | approval for future CPI package only |
| 10 | local-validator-evidence | LOCAL_VALIDATOR_ONLY | Produce local-validator evidence before live action | evidence reviewed |
| 11 | deployment-readiness-review | REVIEW_ONLY | Confirm all blockers, evidence, tests, and docs are aligned | activation/deploy still blocked |
| 12 | activation-package | ACTIVATION_PACKAGE | Define exact activation/deploy/route/RPC actions | exact GO required |
| 13 | activation-execution | ACTIVATION_EXECUTION | Execute only the approved live actions | post-action evidence and ledger update |

## Required Package Template Going Forward

Every package should declare:

- package_name
- package_scope
- active_lane_before
- active_lane_after
- canonical_runtime_state_file
- source_change_authorized
- blocker_removal_authorized
- activation_authorized
- deploy_authorized
- upgrade_authorized
- rpc_mutation_authorized
- route_enablement_authorized
- spl_cpi_enablement_authorized
- Program ID source binding authorized
- state_ledger_update_required
- source blockers before package
- source blockers after package
- review approvals before package
- review approvals after package
- explicit non-goals
- next allowed package
- forbidden next actions

## Definition of Done for Any Package

A package is not closed unless it states:

- what changed;
- what did not change;
- whether source changed;
- whether blockers changed;
- whether docs/gateway/current-runtime-state.md was updated;
- whether tests were run or why tests were not required;
- which lane is active next;
- what action is forbidden next without separate approval.

## Next Immediate Step

Review docs/gateway/program-id-binding-resolution-plan.md and choose the next approved package.

Next package for approval request:

- program-id-dependent-pda-evidence

Required behavior until a separate explicit approval is received:

- use docs/gateway/current-runtime-state.md as canonical coordination state;
- keep ExternalReviewIncomplete removed from active source blockers;
- keep PlaceholderProgramId removed from active source blockers;
- do not change source;
- do not remove blockers;
- do not activate route;
- do not enable SPL CPI;
- do not deploy;
- do not mutate RPC state.

## Roadmap Update Rule

This roadmap may be updated by future documentation-only packages.

Roadmap updates do not change runtime state unless docs/gateway/current-runtime-state.md is also updated in the same closure commit.
