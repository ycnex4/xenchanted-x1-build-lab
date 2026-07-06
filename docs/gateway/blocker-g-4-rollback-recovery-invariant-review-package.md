# Blocker G.4 — Rollback / recovery invariant review package

Status:

BLOCKER_G_REVIEW_READY_ROLLBACK_RECOVERY_INVARIANTS_RECORDED_NO_RPC_NO_EXECUTION

Current decision:

BLOCKER_G_NOT_CLOSED_PENDING_CLOSURE_DECISION

Closure candidate prepared:

ROLLBACK_RECOVERY_PLAN_REVIEWED_EXECUTION_NOT_APPROVED

NO-GO REMAINS_FOR_BUILD_DEPLOY_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Purpose

Blocker G.4 records the rollback / recovery invariant review package.

G.4 is review-only.

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

## Evidence basis

G.4 is based on:

- G.1 rollback / recovery planning
- G.2 repo-grounded rollback / recovery inventory
- G.3 rollback / recovery decision model

## Reviewed invariants

- full stage-gated recovery model
- explicit pre-mutation abort points
- post-submit observation evidence required
- automatic retry rejected
- explicit scoped user GO required before any mutation or recovery action
- abandon/redeploy path required when safe rollback is not possible
- evidence required before next stage or recovery branch
- no build/deploy/upgrade/state-init/SPL/package/signing/RPC/testnet/submit/mutation approved

## Invariant review matrix

```text
# Rollback / recovery invariant review matrix

G4_INVARIANT_01_FULL_STAGE_GATED_RECOVERY
status: reviewed
result: true
meaning: Recovery must be stage-gated across build, deploy/upgrade, state init, SPL setup, guardian package, submit, and observation boundaries.

G4_INVARIANT_02_PRE_MUTATION_ABORT_POINTS
status: reviewed
result: true
meaning: Every mutation stage must have an explicit pre-mutation abort point.

G4_INVARIANT_03_POST_SUBMIT_OBSERVATION
status: reviewed
result: true
meaning: Post-submit observation evidence is required before declaring success/failure or continuing.

G4_INVARIANT_04_NO_AUTOMATIC_RETRY
status: reviewed
result: true
meaning: Automatic retry after failure is rejected.

G4_INVARIANT_05_EXPLICIT_SCOPED_USER_GO
status: reviewed
result: true
meaning: Any mutation or recovery action requires explicit scoped user GO.

G4_INVARIANT_06_NON_REVERSIBLE_ABANDON_OR_REDEPLOY
status: reviewed
result: true
meaning: If safe rollback is not possible, the required path is abandon/redeploy decision, not fake rollback.

G4_INVARIANT_07_EVIDENCE_BEFORE_NEXT_STAGE
status: reviewed
result: true
meaning: Evidence is required before moving to the next stage or recovery branch.

G4_INVARIANT_08_NO_EXECUTION_APPROVED
status: reviewed
result: true
meaning: G.4 does not approve build, deploy, upgrade, state init, SPL setup, guardian package construction, signing, RPC, testnet, submit, or mutation.

G4_AGGREGATE
all_invariants_reviewed: true
blocker_g_closure_ready: true
closure_type: narrow_recovery_boundary_only
```

## Review result

all_invariants_reviewed: true

blocker_g_closure_ready: true

closure_type: narrow_recovery_boundary_only

## Closure candidate prepared

G.4 prepares, but does not itself record, a narrow closure candidate for Blocker G:

ROLLBACK_RECOVERY_PLAN_REVIEWED_EXECUTION_NOT_APPROVED

Meaning:

- rollback / recovery model has been reviewed
- recovery must be stage-gated
- automatic retry is rejected
- explicit scoped user GO is required
- evidence is required before the next stage
- non-reversible actions require abandon/redeploy decision path
- no execution is approved

## Remaining open items outside G closure

- Blocker G closure decision record
- Blocker B expected post-upgrade ProgramData hash
- future final scoped GO package
- future pre-mutation evidence bundle
- future post-submit observation bundle

## Non-closure statement

G.4 does not close Blocker G.

G.4 does not approve:

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

BLOCKER_G_REVIEW_READY_ROLLBACK_RECOVERY_INVARIANTS_RECORDED_NO_RPC_NO_EXECUTION

Current decision:

BLOCKER_G_NOT_CLOSED_PENDING_CLOSURE_DECISION

Closure candidate prepared:

ROLLBACK_RECOVERY_PLAN_REVIEWED_EXECUTION_NOT_APPROVED

NO-GO REMAINS_FOR_BUILD_DEPLOY_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Next safe step

Blocker G.5 — rollback / recovery closure decision record.

G.5 may close Blocker G narrowly as rollback/recovery model closure only.

G.5 must not run build, deploy, upgrade, initialize state, configure SPL, construct packages, sign, call RPC, use testnet, submit, or mutate.

## Evidence preview

metadata:

```text
phase=blocker-g-4-rollback-recovery-invariant-review-package
timestamp_utc=2026-07-06T20:05:16Z
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
