# Blocker G.3 — Rollback / recovery decision model

Status:

BLOCKER_G_OPEN_ROLLBACK_RECOVERY_DECISION_MODEL_RECORDED_NO_RPC_NO_EXECUTION

Current decision:

FULL_STAGE_GATED_RECOVERY_NO_AUTOMATIC_RETRY_USER_GO_REQUIRED

Rollback model:

STAGE_GATED_ABORT_OBSERVE_RECOVER_OR_ABANDON_REDEPLOY

Automatic retry policy:

AUTOMATIC_RETRY_REJECTED

Non-reversible action policy:

ABANDON_OR_REDEPLOY_IF_SAFE_ROLLBACK_NOT_POSSIBLE

User GO policy:

EXPLICIT_SCOPED_USER_GO_REQUIRED_BEFORE_ANY_MUTATION_OR_RECOVERY_ACTION

Evidence policy:

EVIDENCE_REQUIRED_BEFORE_NEXT_STAGE_OR_RECOVERY_BRANCH

Execution boundary:

FUTURE_FINAL_SCOPED_GO_REQUIRED_BEFORE_ANY_BUILD_DEPLOY_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGE_SIGNING_SUBMIT_OR_RECOVERY_MUTATION

NO-GO REMAINS_FOR_BUILD_DEPLOY_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Purpose

Blocker G.3 records the rollback / recovery decision model.

G.3 is decision-model only.

It does not run a build.

It does not deploy.

It does not upgrade.

It does not write a buffer.

It does not change authority.

It does not initialize state.

It does not configure SPL.

It does not construct guardian packages.

It does not sign.

It does not call RPC.

It does not use testnet.

It does not submit or mutate any network.

## Background

G.1 opened rollback / recovery planning.

G.2 completed repo-grounded rollback / recovery inventory.

G.3 selects the recovery model and safety policies without executing anything.

## Selected model

FULL_STAGE_GATED_RECOVERY_NO_AUTOMATIC_RETRY_USER_GO_REQUIRED

Meaning:

- recovery must be stage-gated
- every stage must have explicit abort point
- every stage must have evidence before moving forward
- automatic retry is rejected
- non-reversible actions require abandon/redeploy decision path
- every mutation or recovery action requires explicit scoped user GO

## Required stage gates

- pre-build / pre-artifact gate
- post-build / pre-deploy gate
- post-deploy-or-upgrade / pre-state-init gate
- post-state-init / pre-SPL-setup gate
- post-SPL-setup / pre-guardian-package gate
- post-guardian-package / pre-submit gate
- post-submit observation gate
- non-reversible action abandon/redeploy gate

## Rejected actions

- automatic retry after failed mutation
- continuing after missing evidence
- continuing after ambiguous post-submit state
- pretending non-reversible mutation can always be rolled back
- recovery action without explicit scoped user GO
- build/deploy/upgrade/state-init/SPL/package/signing/submit inside G.3

## Decision matrix

```text
# Rollback / recovery decision matrix

G3_MODEL_0_NO_RECOVERY_PLAN
status: rejected
meaning: Proceed toward mutation without documented rollback/recovery.
reason_rejected: unsafe and incompatible with final scoped GO.

G3_MODEL_1_AUTOMATIC_RETRY_ON_FAILURE
status: rejected
meaning: Scripts retry failed mutation steps automatically.
reason_rejected: unsafe; can cause duplicate or unintended mutation.

G3_MODEL_2_PRE_MUTATION_ABORT_POINTS_ONLY
status: insufficient
meaning: Abort points exist before mutation, but post-mutation observation/recovery is incomplete.
reason_insufficient: does not cover partial/final submit evidence and non-reversible branches.

G3_MODEL_3_FULL_STAGE_GATED_RECOVERY_PLAN
status: selected
meaning: Each stage has explicit precondition, abort point, evidence requirement, success condition, failure condition, and next allowed action.
reason_selected: matches blocker-by-blocker safety model.

G3_MODEL_4_REDEPLOY_OR_ABANDON_IF_NON_REVERSIBLE
status: selected_required_property
meaning: If an action cannot be safely rolled back, recovery must explicitly switch to abandon/redeploy decision path.
reason_selected: avoids fake rollback assumptions.

G3_MODEL_5_USER_FINAL_GO_AND_MANUAL_CONFIRMATION
status: selected_required_property
meaning: Any mutation or recovery action requires explicit scoped user GO.
reason_selected: preserves human authorization boundary.

SELECTED_G3_DECISION
FULL_STAGE_GATED_RECOVERY_NO_AUTOMATIC_RETRY_USER_GO_REQUIRED

SELECTED_G3_ROLLBACK_MODEL
STAGE_GATED_ABORT_OBSERVE_RECOVER_OR_ABANDON_REDEPLOY

SELECTED_G3_AUTOMATIC_RETRY_POLICY
AUTOMATIC_RETRY_REJECTED

SELECTED_G3_NON_REVERSIBLE_POLICY
ABANDON_OR_REDEPLOY_IF_SAFE_ROLLBACK_NOT_POSSIBLE

SELECTED_G3_USER_GO_POLICY
EXPLICIT_SCOPED_USER_GO_REQUIRED_BEFORE_ANY_MUTATION_OR_RECOVERY_ACTION

SELECTED_G3_EVIDENCE_POLICY
EVIDENCE_REQUIRED_BEFORE_NEXT_STAGE_OR_RECOVERY_BRANCH

SELECTED_G3_EXECUTION_BOUNDARY
FUTURE_FINAL_SCOPED_GO_REQUIRED_BEFORE_ANY_BUILD_DEPLOY_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGE_SIGNING_SUBMIT_OR_RECOVERY_MUTATION
```

## Remaining open items before G closure

- rollback / recovery invariant review package
- closure decision record
- final scoped GO package after B and G are closed
- actual future pre-mutation evidence bundle
- actual future post-submit observation bundle

## Non-closure statement

G.3 does not close Blocker G.

G.3 does not approve:

- build
- deploy
- upgrade
- write-buffer
- authority change
- state initialization
- SPL setup
- guardian package construction
- signing
- RPC
- testnet
- transaction submit
- mutation
- production activation

## Result

Current status:

BLOCKER_G_OPEN_ROLLBACK_RECOVERY_DECISION_MODEL_RECORDED_NO_RPC_NO_EXECUTION

Current decision:

FULL_STAGE_GATED_RECOVERY_NO_AUTOMATIC_RETRY_USER_GO_REQUIRED

Execution boundary:

FUTURE_FINAL_SCOPED_GO_REQUIRED_BEFORE_ANY_BUILD_DEPLOY_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGE_SIGNING_SUBMIT_OR_RECOVERY_MUTATION

NO-GO REMAINS_FOR_BUILD_DEPLOY_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Next safe step

Blocker G.4 — rollback / recovery invariant review package.

G.4 should review stage gates, no automatic retry, abandon/redeploy path, evidence requirements, explicit user GO, and no-execution boundary.

G.4 must not run build, deploy, upgrade, initialize state, configure SPL, construct packages, sign, call RPC, use testnet, submit, or mutate.

## Evidence preview

metadata:

```text
phase=blocker-g-3-rollback-recovery-decision-model
timestamp_utc=2026-07-06T20:02:59Z
repo_only=true
rpc_used=false
testnet_used=false
rollback_executed=false
recovery_executed=false
deploy_executed=false
upgrade_executed=false
write_buffer_executed=false
authority_change_executed=false
state_initialized=false
spl_setup_executed=false
guardian_package_constructed=false
signing_executed=false
mutation_executed=false
```
