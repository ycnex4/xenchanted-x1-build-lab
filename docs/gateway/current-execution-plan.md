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

- active runtime lane: pre-go-operational-readiness
- next required decision: request/review production-proof-log-instantiation-plan; exact activation GO remains premature and blocked
- next runtime package scope: PRE_GO_OPERATIONAL_READINESS_REQUIRES_SEPARATE_APPROVAL
- ExternalReviewIncomplete effective status: REMOVED
- source blockers removed to date: ExternalReviewIncomplete, PlaceholderProgramId
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
| 5 | program-id-dependent-pda-evidence | PURE_EVIDENCE_DOCUMENTATION_NO_CODE_MUTATION_NO_ACTIVATION | Record/review Program-ID-dependent PDA evidence | COMPLETE; evidence reviewed, no live action |
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

Do not request exact activation GO yet. Complete PRE-GO operational-readiness packages first.

Next required decision:

- request/review production-proof-log-instantiation-plan
- exact activation GO remains blocked until PRE-GO operational-readiness packages complete

Execution remains blocked.

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

## Completed: program-id-dependent-pda-evidence

Status: COMPLETE.

This package recorded Program-ID-dependent PDA evidence for the already-bound Program ID:

- Program ID: `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`
- gateway_mint_authority PDA: `BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG`
- gateway_mint_authority bump: `252`

No source-code mutation, activation, deploy, upgrade, RPC mutation, route enablement, SPL CPI enablement, guardian set instantiation, proof log instantiation, local-only fixture emission, or cleanup was authorized or performed.

Next required decision:

- separate exact activation GO

Execution remains blocked until that separate GO.

## Completed: live-route-spl-cpi-activation-source-plan

Status: COMPLETE.

This package recorded the non-executing source-readiness and activation-plan boundaries for future live route and SPL CPI activation work.

It defined:

- future boundary for `LiveRouteDisabled` removal;
- future boundary for `SplCpiExecutionDisabled` removal;
- guardian set preconditions;
- proof log preconditions;
- final deployment-readiness review gates.

No source-code mutation, activation, deploy, upgrade, RPC mutation, route enablement, SPL CPI enablement, guardian set instantiation, proof log instantiation, local-only fixture emission, cleanup, or blocker removal was authorized or performed.

Next required decision:

- separate exact activation GO

Execution remains blocked until that separate GO.

## Completed: activation-package-closure

Status: COMPLETE.

This package recorded the final non-executing activation package closure.

It summarized completed packages:

- external-review-blocker-removal-source-change;
- program-id-binding-source-change;
- program-id-dependent-pda-evidence;
- live-route-spl-cpi-activation-source-plan.

It recorded the current blocked runtime state and defined the remaining requirements before any exact activation GO.

No source-code mutation, activation, deploy, upgrade, RPC mutation, route enablement, SPL CPI enablement, guardian set instantiation, proof log instantiation, blocker removal, local-only fixture emission, cleanup, or private-key material commit/share was authorized or performed.

Next required decision:

- separate exact activation GO

Execution remains blocked until a separate exact activation GO.

## PRE-GO Decision Review Result

Status: COMPLETE.

Reviewer consensus:

- Claude: NOT_READY_REQUIRES_PRE_GO_PACKAGE
- Theo: NOT_READY_REQUIRES_PRE_GO_PACKAGE
- Codex: NOT_READY_REQUIRES_PRE_GO_PACKAGE

The project is safely closed, non-executing, and blocked, but it is not ready to request exact activation GO.

Required PRE-GO sequence before any exact activation GO request:

1. production-guardian-set-instantiation-source-change;
2. production-proof-log-instantiation-source-change;
3. programdata-upgrade-authority-evidence;
4. pre-go-deployment-readiness-review.

Codex also recommends staged production-safe SPL CPI gate/source-change and live-route source-change, with live route retained as the final exposure gate.

No source-code mutation, formatting changes, activation, deploy, upgrade, RPC mutation, route enablement, SPL CPI enablement, guardian set instantiation, proof log instantiation, blocker removal, cleanup, or private-key/keypair material handling is authorized by this review.

## Production Guardian Set Instantiation Plan Result

Status: COMPLETE.

Package:

- production-guardian-set-instantiation-plan

Result:

- recommended production guardian set model: program-owned PDA/config account;
- only public guardian keys may be recorded;
- exact guardian public key list and threshold must be supplied by the next source/config package;
- guardian set versioning is required;
- domain separation is required;
- duplicate signers must not inflate quorum;
- unknown signers must not count toward quorum;
- rotation is out of scope and requires separate approval.

Preserved blockers:

- LiveRouteDisabled = ACTIVE
- SplCpiExecutionDisabled = ACTIVE
- ProductionGuardianSetUnset = ACTIVE
- ProductionProofLogUnset = ACTIVE

Forbidden actions preserved:

- no activation;
- no deploy;
- no upgrade;
- no RPC mutation;
- no route enablement;
- no SPL CPI enablement;
- no guardian set instantiation;
- no proof log instantiation;
- no blocker removal;
- no source code mutation.

Next required package:

- production-guardian-set-instantiation-source-change

The next package must be separately approved and must either keep `ProductionGuardianSetUnset` active with explanation or resolve it only if explicit blocker-removal/source-change approval is granted and tests/evidence prove the production guardian set.

## Production Guardian Set V1 Source Binding Result

Status: COMPLETE.

Package:

- production-guardian-set-v1-public-record-and-source-change

Result:

- guardian_set_version = 1
- guardian_count = 5
- threshold = 3
- quorum_model = 3-of-5
- key_type = Ed25519 / Solana public keys
- descriptor_hash_sha256 = 4088a1f71870e617f3635d1c29aedd9fc53a0c136c6f69e0cb343d217ab1cd83
- public record = docs/gateway/production-guardian-set-v1-public-record.md
- source binding = programs/xxxl-svm/src/production_guardian_set_v1.rs
- ProductionGuardianSetUnset = REMOVED

Preserved blockers:

- LiveRouteDisabled = ACTIVE
- SplCpiExecutionDisabled = ACTIVE
- ProductionProofLogUnset = ACTIVE

Preserved forbidden actions:

- no activation;
- no deploy;
- no upgrade;
- no RPC mutation;
- no route enablement;
- no SPL CPI enablement;
- no proof log instantiation;
- no unrelated blocker removal;
- no private key or keypair material;
- no signing package construction;
- no exact activation GO.

Next required package:

- production-proof-log-instantiation-plan

Execution remains blocked.

<!-- BEGIN production-proof-log-instantiation-plan -->
## Production Proof Log Instantiation Plan

Updated at UTC: 2026-07-11T05:04:53Z
Package: production-proof-log-instantiation-plan
Approval: APPROVE_PRODUCTION_PROOF_LOG_INSTANTIATION_PLAN_NO_ACTIVATION

### Completed in this package

This package defines the production proof-log model and the criteria required for a future separately approved proof-log source/config package.

No proof-log instantiation occurs in this package.

### Preserved restrictions

| Action | State |
| --- | --- |
| Activation | BLOCKED |
| Deploy | BLOCKED |
| Upgrade | BLOCKED |
| RPC mutation | BLOCKED |
| Route enablement | BLOCKED |
| SPL CPI enablement | BLOCKED |
| Proof-log instantiation | BLOCKED |
| Guardian set mutation | BLOCKED |
| Source code mutation | BLOCKED |
| Blocker removal | BLOCKED |
| Signing package construction | BLOCKED |
| Exact activation GO | BLOCKED |

### Next required action

Request separate approval for:

production-proof-log-source-config-package

or equivalent separately approved package.

The next package must not imply activation. ProductionProofLogUnset may only be removed after separate approval and evidence that the proof-log publication model has been instantiated.

<!-- END production-proof-log-instantiation-plan -->
