# Program ID Binding Resolution Plan

Status: PLANNING_ONLY
Package: program-id-binding-resolution-plan
Active lane: program-id-binding
Canonical runtime state: docs/gateway/current-runtime-state.md
Execution roadmap: docs/gateway/current-execution-plan.md

This package is a planning package only.

It inventories Program ID binding surfaces, Program-ID-dependent PDA surfaces, blocker state, and future source-change boundaries.

It does not change source code, remove blockers, bind a real Program ID, enable route execution, enable SPL CPI execution, deploy, upgrade, mutate RPC state, emit fixtures, or activate anything.

## Approval Basis

Theo verdict:

APPROVE_PROGRAM_ID_BINDING_RESOLUTION_PLAN_PLANNING_ONLY

This approval authorizes planning only.

It does not authorize:

- source changes;
- blocker removal;
- Program ID source binding;
- ExternalReviewIncomplete blocker removal;
- PlaceholderProgramId blocker removal;
- live route enablement;
- SPL CPI execution enablement;
- deployment;
- upgrade;
- RPC mutation;
- local-only fixture emission;
- activation.

## Base State

This plan starts from the post-reconciliation and post-roadmap main state.

Required coordination sources:

- docs/gateway/current-runtime-state.md
- docs/gateway/current-execution-plan.md

The canonical runtime state currently records:

- active lane: program-id-binding
- next package: program-id-binding-resolution-plan
- ExternalReviewIncomplete effective status: ACTIVE
- PlaceholderProgramId effective status: ACTIVE
- source_blocker_removed=false
- source_change_authorized=false
- activation_authorized=false
- route_enablement_authorized=false
- spl_cpi_enablement_authorized=false
- deploy_authorized=false
- rpc_mutation_authorized=false

## State Drift Included in This Plan

This plan must use actual source/main state, not older chat context.

The discovered drift before this package was:

- working context treated ExternalReviewIncomplete as closed;
- source still treats ExternalReviewIncomplete as an active blocker;
- tests still expect EXTERNAL_REVIEW_INCOMPLETE as active;
- current-design-checkpoint.md had become stale before gateway-state-reconciliation-1;
- gateway-state-reconciliation-1 created the canonical runtime-state ledger;
- gateway-execution-roadmap-1 created the roadmap and final goal.

Therefore this plan treats:

- ExternalReviewIncomplete_review_approved=true
- ExternalReviewIncomplete_source_blocker_removed=false
- ExternalReviewIncomplete_effective_status=ACTIVE

## Evidence Files

Evidence for this planning package is stored under:

docs/gateway/evidence/program-id-binding-resolution-plan/

Required evidence files:

- program-id-binding-plan-scope.txt
- program-id-binding-state-snapshot.txt
- program-id-binding-source-surfaces.txt
- program-id-binding-doc-surfaces.txt

The raw broad inventory was intentionally replaced with focused source/doc inventories to avoid noisy stale checkpoint evidence.

## Current Source Blockers

The current effective source blockers remain:

| Blocker | Effective Status | This Package Changes It |
|:---|:---:|:---:|
| PlaceholderProgramId | ACTIVE | no |
| LiveRouteDisabled | ACTIVE | no |
| SplCpiExecutionDisabled | ACTIVE | no |
| ProductionGuardianSetUnset | ACTIVE | no |
| ProductionProofLogUnset | ACTIVE | no |
| ExternalReviewIncomplete | ACTIVE | no |

This package removes none of them.

## Program ID Binding Surfaces

Future source-change planning must account for at least these source surfaces:

| Surface | Role | Future Concern |
|:---|:---|:---|
| programs/xxxl-svm/src/lib.rs | Defines XXXL_PROGRAM_ID_PLACEHOLDER and runtime status constants | future Program ID binding must not silently imply activation |
| programs/xxxl-svm/src/program_id_status.rs | Reports configured Program ID, placeholder blocker, readiness status | future source-change must update report semantics and tests |
| programs/xxxl-svm/src/deployment_status.rs | Lists effective deployment blockers | blocker changes require separate approval |
| programs/xxxl-svm/src/safety_invariants.rs | Encodes safety lock and unlock criteria | future source-change must preserve safety lock unless explicitly approved |
| programs/xxxl-svm/src/pda.rs | Derives PDA from Program ID using find_program_address | real Program ID requires regenerated PDA evidence |
| programs/xxxl-svm/src/processor.rs | Contains live route activation gate | must remain disabled in Program ID source-change unless separately approved |
| programs/xxxl-svm/src/cpi.rs | Contains SPL CPI execution gate | must remain disabled in Program ID source-change unless separately approved |

## PDA Dependency Surfaces

The Program ID is not only a label.

It affects PDA derivation.

Therefore future Program ID source-change must include a PDA evidence step.

Known PDA dependency concerns:

- gateway mint authority PDA depends on Program ID;
- PDA derivation uses fixed seeds;
- changing Program ID changes derived PDA;
- any real Program ID binding must regenerate and review Program-ID-dependent PDA evidence;
- PDA evidence must not imply live route enablement or SPL CPI execution;
- PDA evidence must not submit transactions or mutate RPC state.

## Future Source-Change Target Classes

A future Program ID source-change package may be allowed to touch only explicitly approved target classes.

Potential target classes:

1. Program ID placeholder/reporting surface
   - lib.rs
   - program_id_status.rs

2. Program ID blocker/reporting surface
   - deployment_status.rs
   - safety_invariants.rs
   - related tests

3. Program-ID-dependent PDA evidence surface
   - pda.rs
   - docs/xxxl PDA docs
   - evidence files generated by an approved package

4. Documentation alignment surface
   - current-runtime-state.md if runtime-state classification changes
   - current-execution-plan.md if roadmap ordering changes
   - package-specific evidence

These are target classes only.

They are not source-change authorization.

## Future Source-Change Non-Targets

A Program ID source-change package must not change these unless a separate explicit approval says so:

- live route activation gate;
- SPL CPI execution gate;
- guardian set instantiation;
- proof log instantiation;
- deployment execution;
- upgrade execution;
- RPC mutation;
- local-only fixture emission;
- testnet transaction submission;
- activation package;
- activation execution;
- private keys or secrets;
- cleanup of unrelated evidence;
- removal of ExternalReviewIncomplete unless the package is specifically external-review-blocker-removal-source-change.

## Recommended Next Decision

After this planning package, there are two possible future paths.

### Path A — External Review Blocker Removal First

Package:

external-review-blocker-removal-source-change

Purpose:

- remove ExternalReviewIncomplete from active source blockers;
- update tests and safety expectations;
- keep all other blockers active;
- no activation;
- no deploy;
- no route enablement;
- no SPL CPI enablement.

This path cleans the blocker model before Program ID source binding.

### Path B — Program ID Source Binding First

Package:

program-id-binding-source-change

Purpose:

- replace the placeholder Program ID model with a reviewed real Program ID binding path;
- update Program ID status/reporting;
- regenerate or prepare Program-ID-dependent PDA evidence;
- keep non-Program-ID blockers active;
- no activation;
- no deploy;
- no route enablement;
- no SPL CPI enablement.

This path should proceed only if reviewers agree that ExternalReviewIncomplete may remain active during Program ID source binding.

## Recommended Order

Preferred order:

1. external-review-blocker-removal-source-change
2. program-id-binding-source-change
3. program-id-dependent-pda-evidence
4. guardian/proof-log instantiation planning
5. route/CPI readiness planning
6. local-validator evidence
7. deployment-readiness review
8. activation package only after exact GO

Reason:

Cleaning ExternalReviewIncomplete first reduces ambiguity before Program ID source changes.

Review-integrity condition:

Removing ExternalReviewIncomplete reflects only the already-completed external review of the reviewed source state.

It does not pre-approve the subsequent Program ID binding.

The final deployment-readiness review must re-gate the complete post-Program-ID-binding state as an external-review equivalent, so external review complete is never decoupled from the actually deployed source.

## Required Reviewer Questions

Before moving to any source-change package, reviewers should answer:

1. Should ExternalReviewIncomplete be removed before Program ID source binding?
2. If not, is it acceptable to bind real Program ID while ExternalReviewIncomplete remains active?
3. Which exact source files may the Program ID source-change package touch?
4. Which exact source files are forbidden?
5. Should Program ID blocker removal and real Program ID binding happen in one package or separate packages?
6. What PDA evidence is required before any Program ID blocker status changes?
7. Must current-runtime-state.md be updated in the source-change closure commit?
8. Which tests must change, and which tests must remain unchanged?
9. What wording should appear in the future exact GO phrase?
10. What must remain explicitly NO-GO after source-change?

## Planning Package Closure Criteria

This package is complete when:

- focused source surfaces are inventoried;
- focused doc surfaces are inventoried;
- current blocker state is recorded;
- Program ID and PDA dependencies are documented;
- future source-change target classes are listed;
- future source-change non-targets are listed;
- no source files are changed;
- no blockers are removed;
- no activation/deploy/route/CPI/RPC action is authorized;
- next reviewer decision is clear.

## Expected State After This Package

After this package:

- program-id-binding lane remains active;
- PlaceholderProgramId remains ACTIVE;
- ExternalReviewIncomplete remains ACTIVE;
- LiveRouteDisabled remains ACTIVE;
- SplCpiExecutionDisabled remains ACTIVE;
- ProductionGuardianSetUnset remains ACTIVE;
- ProductionProofLogUnset remains ACTIVE;
- source_change_authorized=false;
- blocker_removal_authorized=false;
- activation_authorized=false;
- deploy_authorized=false;
- route_enablement_authorized=false;
- spl_cpi_enablement_authorized=false;
- rpc_mutation_authorized=false.

## Next Allowed Action

Submit this planning package for review.

Recommended reviewer verdict options:

- APPROVE_PROGRAM_ID_BINDING_RESOLUTION_PLAN
- APPROVE_WITH_DOC_FIXES
- BLOCKED_REQUIRES_PLAN_FIXES

Approval of this plan must not be interpreted as approval for source changes.
