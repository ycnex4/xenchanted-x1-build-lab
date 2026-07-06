# Blocker G.1 — Rollback / recovery plan planning

Status:

BLOCKER_G_OPEN_ROLLBACK_RECOVERY_PLAN_PLANNING_ONLY_NO_RPC_NO_EXECUTION

Current decision:

BLOCKER_G_NOT_CLOSED

NO-GO REMAINS_FOR_BUILD_DEPLOY_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Purpose

Blocker G.1 opens the rollback / recovery plan track.

G.1 is planning-only.

It does not run a build.

It does not deploy.

It does not upgrade.

It does not write a buffer.

It does not change authority.

It does not initialize state.

It does not create or configure SPL mint state.

It does not construct guardian packages.

It does not sign.

It does not call RPC.

It does not use testnet.

It does not submit or mutate any network.

## Why G follows F

Blockers A, C, D, E, and F are now closed narrowly.

Those closures still do not approve execution.

Before any final GO package can be considered, rollback / recovery expectations must be explicit.

## Planning questions

```text
# Rollback / recovery planning questions

G1_QUESTION_01_SCOPE
What exactly must rollback / recovery cover: bad build, bad deploy artifact, bad upgrade, bad state init, bad SPL setup, bad guardian descriptor, bad submit, or bad post-submit observation?

G1_QUESTION_02_PRE_MUTATION_ABORT
What are the abort points before any network mutation?

G1_QUESTION_03_POST_BUILD_RECOVERY
What happens if a build artifact is wrong before deploy?

G1_QUESTION_04_POST_DEPLOY_PRE_STATE_INIT_RECOVERY
What happens if a program upgrade/deploy is wrong before state initialization?

G1_QUESTION_05_POST_STATE_INIT_RECOVERY
What happens if state initialization is wrong?

G1_QUESTION_06_POST_SPL_SETUP_RECOVERY
What happens if SPL mint authority setup is wrong?

G1_QUESTION_07_POST_GUARDIAN_DESCRIPTOR_RECOVERY
What happens if guardian descriptor data is wrong before package construction?

G1_QUESTION_08_POST_PACKAGE_CONSTRUCTION_ABORT
What happens if guardian packages are constructed but not submitted?

G1_QUESTION_09_POST_SUBMIT_OBSERVATION
What evidence is required after a submit to decide success/failure?

G1_QUESTION_10_STOP_CONDITIONS
What conditions force stop / no further actions?

G1_QUESTION_11_OWNER_ACTIONS
Which actions require the user's explicit GO?

G1_QUESTION_12_NO_AUTOMATIC_RETRY
Which steps must never be retried automatically?

G1_QUESTION_13_EVIDENCE_REQUIREMENTS
What evidence must be saved for each recovery branch?

G1_QUESTION_14_COMMUNICATION
What status wording should be used when rollback is impossible and only abandonment/redeploy is possible?

G1_QUESTION_15_FINAL_GO_BOUNDARY
How does the recovery plan compose with the final scoped GO package?
```

## Candidate models

```text
# Rollback / recovery candidate models

G1_MODEL_0_NO_RECOVERY_PLAN
status: rejected_candidate
meaning: Proceed to mutation without documented rollback/recovery.
reason_rejected: incompatible with safe testnet activation.

G1_MODEL_1_AUTOMATIC_RETRY_ON_FAILURE
status: rejected_candidate
meaning: Scripts retry mutation steps automatically after failure.
reason_rejected: unsafe; risks duplicate or unintended mutation.

G1_MODEL_2_PRE_MUTATION_ABORT_POINTS_ONLY
status: partial_candidate
meaning: Define explicit abort points before deploy/state init/SPL setup/package submit.
limitation: Does not cover post-mutation observation and recovery.

G1_MODEL_3_FULL_STAGE_GATED_RECOVERY_PLAN
status: preferred_candidate
meaning: Define pre-mutation abort points, post-build recovery, post-upgrade observation, state-init stop conditions, SPL setup stop conditions, guardian package stop conditions, and post-submit evidence.
reason_preferred: matches current blocker-by-blocker safety model.

G1_MODEL_4_REDEPLOY_OR_ABANDON_IF_NON_REVERSIBLE
status: required_future_property
meaning: If an action is irreversible or not safely reversible, the recovery plan must explicitly say so and switch to abandon/redeploy decision path.
reason_required: avoids pretending that every mutation is reversible.

G1_MODEL_5_USER_FINAL_GO_AND_MANUAL_CONFIRMATION
status: required_future_property
meaning: Any mutation or recovery branch requires explicit user GO and no automatic retries.
reason_required: preserves scoped authorization boundary.
```

## Scope

```text
# Rollback / recovery planning scope

IN_SCOPE_FOR_PLANNING:
- bad local build artifact
- bad deploy artifact
- wrong expected ProgramData hash
- wrong upgrade authority observation
- wrong state initialization package
- wrong SPL mint authority setup package
- wrong guardian descriptor/package
- failed or partial submit observation
- post-submit evidence collection
- stop conditions
- no-automatic-retry policy
- explicit user GO boundary

OUT_OF_SCOPE_FOR_G1_EXECUTION:
- running build
- deploying
- upgrading
- writing buffers
- changing authority
- initializing state
- creating SPL mint
- setting SPL authority
- constructing guardian packages
- signing
- calling RPC
- using testnet
- submitting transactions
- mutating any network
```

## Initial direction

G.1 does not select a final recovery plan.

However, the preferred direction for later G steps is:

- full stage-gated recovery plan
- explicit pre-mutation abort points
- no automatic retry after failure
- explicit post-submit observation requirements
- explicit user GO before any mutation or recovery action
- explicit abandon/redeploy path when rollback is not possible
- evidence saved for every branch

## Non-closure statement

G.1 does not close Blocker G.

G.1 does not approve:

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

BLOCKER_G_OPEN_ROLLBACK_RECOVERY_PLAN_PLANNING_ONLY_NO_RPC_NO_EXECUTION

Current decision:

BLOCKER_G_NOT_CLOSED

NO-GO REMAINS_FOR_BUILD_DEPLOY_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Next safe step

Blocker G.2 — repo-grounded rollback / recovery inventory.

G.2 should inspect tracked repository code and docs only.

G.2 must not run build, deploy, upgrade, initialize state, configure SPL, construct packages, sign, call RPC, use testnet, submit, or mutate.

## Evidence preview

metadata:

```text
phase=blocker-g-1-rollback-recovery-plan-planning
timestamp_utc=2026-07-06T19:57:06Z
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
