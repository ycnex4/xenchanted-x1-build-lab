# Blocker G.5 — Rollback / recovery closure decision record

Status:

BLOCKER_G_CLOSED_NARROW_ROLLBACK_RECOVERY_PLAN_REVIEWED_EXECUTION_NOT_APPROVED

Current decision:

BLOCKER_G_CLOSED_NARROW_RECOVERY_INVARIANTS_ONLY

NO-GO REMAINS_FOR_BUILD_DEPLOY_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Purpose

Blocker G.5 records the closure decision for Blocker G.

The closure is narrow.

It closes only the rollback / recovery model and invariant review blocker.

It does not approve execution.

It does not approve build.

It does not approve deploy.

It does not approve upgrade.

It does not approve write-buffer.

It does not approve authority change.

It does not approve state initialization.

It does not approve SPL setup.

It does not approve guardian package construction.

It does not approve signing.

It does not approve RPC.

It does not approve testnet.

It does not approve transaction submit.

It does not approve mutation.

## Closure state

Blocker G is closed as:

ROLLBACK_RECOVERY_PLAN_REVIEWED_EXECUTION_NOT_APPROVED

Meaning:

- rollback / recovery model has been reviewed
- recovery must be full stage-gated
- every mutation stage must have explicit pre-mutation abort point
- post-submit observation evidence is required
- automatic retry is rejected
- explicit scoped user GO is required before any mutation or recovery action
- evidence is required before moving to the next stage or recovery branch
- if safe rollback is not possible, abandon/redeploy decision path is required
- no execution is approved by this closure

## Evidence chain

G.5 is based on:

1. G.1 — rollback / recovery planning
2. G.2 — repo-grounded rollback / recovery inventory
3. G.3 — rollback / recovery decision model
4. G.4 — rollback / recovery invariant review package

## Accepted G.2 inventory result

Accepted G.2 inventory categories:

- rollback / recovery references found
- deploy / upgrade references found
- state / SPL / guardian references found
- final GO / NO-GO references found
- evidence / observation references found
- no execution performed by G.2

Accepted stage-gated recovery inventory:

- pre-build abort branch required
- post-build / pre-deploy abort branch required
- post-deploy / pre-state-init observation branch required
- post-state-init stop condition required
- post-SPL-setup stop condition required
- post-guardian-descriptor / pre-package abort branch required
- post-package / pre-submit abort branch required
- post-submit evidence branch required
- abandon/redeploy path required for non-reversible actions
- automatic retry rejected
- explicit scoped user GO required before mutation/recovery action

## Accepted G.3 decision

G.3 decision accepted:

FULL_STAGE_GATED_RECOVERY_NO_AUTOMATIC_RETRY_USER_GO_REQUIRED

Accepted rollback model:

STAGE_GATED_ABORT_OBSERVE_RECOVER_OR_ABANDON_REDEPLOY

Accepted automatic retry policy:

AUTOMATIC_RETRY_REJECTED

Accepted non-reversible action policy:

ABANDON_OR_REDEPLOY_IF_SAFE_ROLLBACK_NOT_POSSIBLE

Accepted user GO policy:

EXPLICIT_SCOPED_USER_GO_REQUIRED_BEFORE_ANY_MUTATION_OR_RECOVERY_ACTION

Accepted evidence policy:

EVIDENCE_REQUIRED_BEFORE_NEXT_STAGE_OR_RECOVERY_BRANCH

## Accepted G.4 invariant result

G.4 invariant result accepted:

all_invariants_reviewed: true

blocker_g_closure_ready: true

closure_type: narrow_recovery_boundary_only

Accepted invariant categories:

- full stage-gated recovery model
- explicit pre-mutation abort points
- post-submit observation evidence required
- automatic retry rejected
- explicit scoped user GO required before any mutation or recovery action
- abandon/redeploy path required when safe rollback is not possible
- evidence required before next stage or recovery branch
- no execution approved

## What this closure allows

This closure allows future planning to treat Blocker G as closed for the narrow rollback / recovery model and invariant question.

It allows the project to proceed to the next separately scoped blocker.

Recommended next blocker:

- B — expected post-upgrade ProgramData hash

## What this closure does not allow

This closure does not approve:

- build
- deploy
- upgrade
- write-buffer
- authority change
- state initialization execution
- SPL setup
- guardian package construction
- signing
- RPC
- testnet
- transaction submit
- mutation
- production activation

## Remaining blockers

After G.5:

- Blocker A — CLOSED narrowly: upgrade authority present but accepted for test phase
- Blocker B — OPEN: expected post-upgrade ProgramData hash
- Blocker C — CLOSED narrowly: B1C7 handler boundary / invariants only
- Blocker D — CLOSED narrowly: state initialization design / invariants only
- Blocker E — CLOSED narrowly: SPL mint authority architecture / invariants only
- Blocker F — CLOSED narrowly: guardian descriptor model / invariants only
- Blocker G — CLOSED narrowly: rollback / recovery model / invariants only
- Blocker H — CLOSED narrowly: local-validator health dry-run only

## Safety invariant

Closing Blocker G must not weaken the overall NO-GO boundary.

Overall testnet mutation remains NO-GO until Blocker B is closed and a final scoped GO package is recorded.

A future final scoped GO remains required before any build, deploy, upgrade, state initialization, SPL setup, guardian package construction, signing, submit, or recovery mutation.

Automatic retry remains rejected.

If safe rollback is not possible, the required path is abandon/redeploy decision, not fake rollback.

## Result

Current status:

BLOCKER_G_CLOSED_NARROW_ROLLBACK_RECOVERY_PLAN_REVIEWED_EXECUTION_NOT_APPROVED

Current decision:

BLOCKER_G_CLOSED_NARROW_RECOVERY_INVARIANTS_ONLY

NO-GO REMAINS_FOR_BUILD_DEPLOY_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Next safe step

Proceed to Blocker B — expected post-upgrade ProgramData hash.

Recommended next step:

Blocker B.1 — expected post-upgrade ProgramData hash planning.

Do not proceed to build, deploy, upgrade, state init execution, SPL setup, guardian package construction, signing, RPC, testnet, submit, or mutation.
