# Blocker B.5 — Expected post-upgrade ProgramData hash closure decision record

Status:

BLOCKER_B_CLOSED_NARROW_EXPECTED_PROGRAMDATA_HASH_MODEL_REVIEWED_EXECUTION_NOT_APPROVED

Current decision:

BLOCKER_B_CLOSED_NARROW_PROGRAMDATA_HASH_INVARIANTS_ONLY

NO-GO REMAINS_FOR_BUILD_HASH_DEPLOY_UPGRADE_WRITE_BUFFER_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Purpose

Blocker B.5 records the closure decision for Blocker B.

The closure is narrow.

It closes only the expected post-upgrade ProgramData hash model and invariant review blocker.

It does not approve execution.

It does not approve build.

It does not approve local hash computation.

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

Blocker B is closed as:

EXPECTED_PROGRAMDATA_HASH_MODEL_REVIEWED_EXECUTION_NOT_APPROVED

Meaning:

- expected ProgramData hash model has been reviewed
- full hash bundle is required
- canonical runtime hash domain is ProgramData executable bytes excluding loader metadata
- canonical hash algorithm is SHA256
- local SBF artifact sha256 is required as future pre-upgrade evidence but is insufficient alone
- source commit, repo clean status, build command, toolchain, lockfiles, and feature flags must be bound
- baseline program id, ProgramData account, and upgrade authority observation must be bound
- expected hash package is required before any upgrade GO
- post-upgrade read-only ProgramData executable-bytes SHA256 verification is required
- hash mismatch is a stop condition
- automatic retry after hash mismatch remains rejected
- explicit scoped user GO is required before any build, hash, upgrade, or recovery action
- no execution is approved by this closure

## Evidence chain

B.5 is based on:

1. B.1 — expected post-upgrade ProgramData hash planning
2. B.2 — repo-grounded ProgramData hash inventory
3. B.3 — expected post-upgrade ProgramData hash decision model
4. B.4 — ProgramData hash invariant review package

## Accepted B.2 inventory result

B.2 inventory accepted:

all_inventory_checks_passed: true

Accepted inventory categories:

- B.1 planning recorded
- preferred full hash bundle present
- baseline program id present
- baseline ProgramData account present
- expected ProgramData hash requirement present
- runtime scaffold/not-deployable state present
- dangerous feature gates present
- SPL CPI closed marker present
- live route disabled marker present
- no build/hash/RPC/execution performed by B.2

## Accepted B.3 decision

B.3 decision accepted:

FULL_HASH_BUNDLE_WITH_PROGRAMDATA_EXECUTABLE_BYTES_SHA256_AS_CANONICAL_RUNTIME_HASH

Accepted canonical hash domain:

PROGRAMDATA_EXECUTABLE_BYTES_EXCLUDING_LOADER_METADATA

Accepted canonical hash algorithm:

SHA256

Accepted local artifact hash policy:

LOCAL_SBF_ARTIFACT_SHA256_REQUIRED_AS_FUTURE_PRE_UPGRADE_EVIDENCE

Accepted build binding policy:

SOURCE_COMMIT_BUILD_COMMAND_TOOLCHAIN_LOCKFILES_AND_FEATURE_FLAGS_REQUIRED

Accepted baseline binding:

BASELINE_PROGRAM_ID_AND_PROGRAMDATA_ACCOUNT_REQUIRED

Accepted pre-upgrade policy:

EXPECTED_HASH_PACKAGE_REQUIRED_BEFORE_ANY_UPGRADE_GO

Accepted post-upgrade policy:

READ_ONLY_PROGRAMDATA_EXECUTABLE_BYTES_SHA256_VERIFICATION_REQUIRED_AFTER_UPGRADE

Accepted mismatch policy:

HASH_MISMATCH_IS_STOP_CONDITION_NO_AUTOMATIC_RETRY

Accepted user GO policy:

EXPLICIT_SCOPED_USER_GO_REQUIRED_BEFORE_ANY_BUILD_HASH_UPGRADE_OR_RECOVERY_ACTION

## Accepted B.4 invariant result

B.4 invariant result accepted:

all_invariants_reviewed: true

blocker_b_closure_ready: true

closure_type: narrow_programdata_hash_model_boundary_only

Accepted invariant categories:

- full hash bundle required
- canonical runtime hash domain is ProgramData executable bytes excluding loader metadata
- canonical hash algorithm is SHA256
- local SBF artifact sha256 required but insufficient alone
- build binding required
- baseline binding required
- expected hash package required before upgrade GO
- post-upgrade read-only verification required
- hash mismatch stop condition
- no automatic retry
- explicit scoped user GO
- no execution approved

## What this closure allows

This closure allows future planning to treat Blocker B as closed for the narrow expected ProgramData hash model and invariant question.

It allows the project to proceed to a separate final scoped GO planning package.

## What this closure does not allow

This closure does not approve:

- actual expected-hash package generation
- build
- local artifact hash computation
- ProgramData executable-bytes hash computation
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

After B.5:

- Blocker A — CLOSED narrowly: upgrade authority present but accepted for test phase
- Blocker B — CLOSED narrowly: expected ProgramData hash model / invariants only
- Blocker C — CLOSED narrowly: B1C7 handler boundary / invariants only
- Blocker D — CLOSED narrowly: state initialization design / invariants only
- Blocker E — CLOSED narrowly: SPL mint authority architecture / invariants only
- Blocker F — CLOSED narrowly: guardian descriptor model / invariants only
- Blocker G — CLOSED narrowly: rollback / recovery model / invariants only
- Blocker H — CLOSED narrowly: local-validator health dry-run only

## Safety invariant

Closing Blocker B must not weaken the overall NO-GO boundary.

Overall testnet mutation remains NO-GO until a future final scoped GO package is recorded.

A future actual expected-hash package remains required before any upgrade GO.

A future actual build/hash evidence bundle remains required before any upgrade GO.

A future post-upgrade read-only verification bundle remains required after any upgrade.

Automatic retry remains rejected.

If any expected/observed hash mismatches, the required path is stop, not retry.

## Result

Current status:

BLOCKER_B_CLOSED_NARROW_EXPECTED_PROGRAMDATA_HASH_MODEL_REVIEWED_EXECUTION_NOT_APPROVED

Current decision:

BLOCKER_B_CLOSED_NARROW_PROGRAMDATA_HASH_INVARIANTS_ONLY

NO-GO REMAINS_FOR_BUILD_HASH_DEPLOY_UPGRADE_WRITE_BUFFER_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Next safe step

Proceed to a separate final scoped GO planning package.

Recommended next step:

Final GO.1 — final scoped GO package planning.

Do not proceed to build, hash computation, deploy, upgrade, state init execution, SPL setup, guardian package construction, signing, RPC, testnet, submit, or mutation.
