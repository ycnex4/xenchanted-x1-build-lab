# Blocker B.4 — ProgramData hash invariant review package

Status:

BLOCKER_B_REVIEW_READY_PROGRAMDATA_HASH_INVARIANTS_RECORDED_NO_BUILD_NO_HASH_NO_RPC_NO_EXECUTION

Current decision:

BLOCKER_B_NOT_CLOSED_PENDING_CLOSURE_DECISION

Closure candidate prepared:

EXPECTED_PROGRAMDATA_HASH_MODEL_REVIEWED_EXECUTION_NOT_APPROVED

NO-GO REMAINS_FOR_BUILD_HASH_DEPLOY_UPGRADE_WRITE_BUFFER_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Purpose

Blocker B.4 records the ProgramData hash invariant review package.

B.4 is review-only.

It does not run build.

It does not compute artifact hash.

It does not compute ProgramData hash.

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

B.4 is based on:

- B.1 expected post-upgrade ProgramData hash planning
- B.2 repo-grounded ProgramData hash inventory
- B.3 expected post-upgrade ProgramData hash decision model

## Reviewed invariants

- full hash bundle required
- canonical runtime hash domain is ProgramData executable bytes excluding loader metadata
- canonical hash algorithm is SHA256
- local SBF artifact sha256 required but insufficient alone
- source commit / repo clean status / build command / toolchain / lockfiles / feature flags binding required
- baseline program id / ProgramData account / upgrade authority observation binding required
- expected hash package required before any upgrade GO
- post-upgrade read-only ProgramData executable-bytes sha256 verification required
- hash mismatch is a stop condition
- automatic retry after hash mismatch rejected
- explicit scoped user GO required before build/hash/upgrade/recovery action
- no execution approved

## Invariant review matrix

```text
# ProgramData hash invariant review matrix

B4_INVARIANT_01_FULL_HASH_BUNDLE
status: reviewed
result: true
meaning: Future expected-hash package must be a full hash bundle, not a single weak identifier.

B4_INVARIANT_02_CANONICAL_RUNTIME_HASH_DOMAIN
status: reviewed
result: true
meaning: Canonical runtime hash domain is ProgramData executable bytes excluding loader metadata.

B4_INVARIANT_03_CANONICAL_HASH_ALGORITHM
status: reviewed
result: true
meaning: Canonical hash algorithm is SHA256.

B4_INVARIANT_04_LOCAL_ARTIFACT_HASH_REQUIRED_BUT_INSUFFICIENT_ALONE
status: reviewed
result: true
meaning: Local SBF artifact sha256 is required as future pre-upgrade evidence but is not sufficient alone.

B4_INVARIANT_05_BUILD_BINDING_REQUIRED
status: reviewed
result: true
meaning: Source commit, repo clean status, build command, toolchain, lockfiles, and feature flags must be bound.

B4_INVARIANT_06_BASELINE_BINDING_REQUIRED
status: reviewed
result: true
meaning: Baseline program id, ProgramData account, and upgrade authority observation must be bound.

B4_INVARIANT_07_EXPECTED_HASH_PACKAGE_BEFORE_UPGRADE_GO
status: reviewed
result: true
meaning: Expected hash package is required before any upgrade GO.

B4_INVARIANT_08_POST_UPGRADE_READ_ONLY_VERIFICATION
status: reviewed
result: true
meaning: Post-upgrade read-only ProgramData executable-bytes sha256 verification is required.

B4_INVARIANT_09_HASH_MISMATCH_STOP_CONDITION
status: reviewed
result: true
meaning: Hash mismatch is a stop condition.

B4_INVARIANT_10_NO_AUTOMATIC_RETRY
status: reviewed
result: true
meaning: Automatic retry after hash mismatch remains rejected.

B4_INVARIANT_11_EXPLICIT_SCOPED_USER_GO
status: reviewed
result: true
meaning: Explicit scoped user GO is required before any build, hash, upgrade, or recovery action.

B4_INVARIANT_12_NO_EXECUTION_APPROVED
status: reviewed
result: true
meaning: B.4 does not approve build, hash computation, deploy, upgrade, write-buffer, state init, SPL setup, guardian package construction, signing, RPC, testnet, submit, or mutation.

B4_AGGREGATE
all_invariants_reviewed: true
blocker_b_closure_ready: true
closure_type: narrow_programdata_hash_model_boundary_only
```

## Review result

all_invariants_reviewed: true

blocker_b_closure_ready: true

closure_type: narrow_programdata_hash_model_boundary_only

## Closure candidate prepared

B.4 prepares, but does not itself record, a narrow closure candidate for Blocker B:

EXPECTED_PROGRAMDATA_HASH_MODEL_REVIEWED_EXECUTION_NOT_APPROVED

Meaning:

- expected ProgramData hash model has been reviewed
- full hash bundle is required
- canonical runtime hash is ProgramData executable bytes sha256 excluding loader metadata
- pre-upgrade expected hash package is required
- post-upgrade read-only verification is required
- mismatch is a stop condition
- automatic retry is rejected
- explicit scoped user GO is required
- no execution is approved

## Remaining open items outside B closure

- Blocker B closure decision record
- future actual expected-hash package
- future actual build/hash execution with explicit scoped GO
- future post-upgrade read-only verification bundle
- future final scoped GO package before any network mutation

## Non-closure statement

B.4 does not close Blocker B.

B.4 does not approve:

- build
- local hash computation
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

BLOCKER_B_REVIEW_READY_PROGRAMDATA_HASH_INVARIANTS_RECORDED_NO_BUILD_NO_HASH_NO_RPC_NO_EXECUTION

Current decision:

BLOCKER_B_NOT_CLOSED_PENDING_CLOSURE_DECISION

Closure candidate prepared:

EXPECTED_PROGRAMDATA_HASH_MODEL_REVIEWED_EXECUTION_NOT_APPROVED

NO-GO REMAINS_FOR_BUILD_HASH_DEPLOY_UPGRADE_WRITE_BUFFER_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Next safe step

Blocker B.5 — expected post-upgrade ProgramData hash closure decision record.

B.5 may close Blocker B narrowly as expected-hash model closure only.

B.5 must not run build, compute hashes, deploy, upgrade, initialize state, configure SPL, construct packages, sign, call RPC, use testnet, submit, or mutate.

## Evidence preview

metadata:

```text
phase=blocker-b-4-programdata-hash-invariant-review-package
timestamp_utc=2026-07-06T20:27:01Z
repo_only=true
build_executed=false
artifact_hash_computed=false
programdata_hash_computed=false
rpc_used=false
testnet_used=false
deploy_executed=false
upgrade_executed=false
write_buffer_executed=false
authority_change_executed=false
state_initialized=false
spl_setup_executed=false
guardian_package_constructed=false
signing_executed=false
submit_executed=false
mutation_executed=false
```
