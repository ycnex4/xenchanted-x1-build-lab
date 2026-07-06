# Scoped Package.4 — Expected-hash/build-hash evidence package invariant review

Status:

SCOPED_PACKAGE_4_REVIEW_READY_EXPECTED_HASH_BUILD_HASH_EVIDENCE_INVARIANTS_RECORDED_NO_BUILD_NO_HASH_NO_RPC_NO_EXECUTION

Current decision:

EXPECTED_HASH_BUILD_HASH_EVIDENCE_MODEL_REVIEWED_FINAL_GO_NOT_GRANTED_PENDING_CLOSURE_DECISION

Closure candidate prepared:

STRICT_EXPECTED_HASH_BUILD_HASH_EVIDENCE_MODEL_REVIEWED_EXECUTION_NOT_APPROVED

Selected model:

STRICT_EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE_MODEL_REQUIRED

Current GO state:

FINAL_GO_NOT_GRANTED

NO-GO REMAINS_FOR_BUILD_HASH_DEPLOY_UPGRADE_WRITE_BUFFER_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Purpose

Scoped Package.4 records the invariant review for the expected-hash/build-hash evidence package model.

This is invariant review only.

It does not draft a runnable package.

It does not grant GO.

It does not run build.

It does not compute artifact hash.

It does not compute ProgramData executable-bytes hash.

It does not call RPC.

It does not use testnet.

It does not deploy.

It does not upgrade.

It does not write a buffer.

It does not change authority.

It does not initialize state.

It does not configure SPL.

It does not construct guardian packages.

It does not sign.

It does not submit or mutate any network.

## Evidence basis

- Scoped Package.1 — first operation class selection planning
- Scoped Package.2 — expected-hash/build-hash evidence package requirements inventory
- Scoped Package.3 — expected-hash/build-hash evidence package decision model
- Final GO.5 — final scoped GO model closure decision record
- Blocker B.5 — expected post-upgrade ProgramData hash closure decision record

## Reviewed invariants

- current GO is not granted
- strict expected-hash/build-hash evidence model required
- Scoped Package.4 is invariant review only
- full hash bundle required
- canonical runtime hash domain is ProgramData executable bytes excluding loader metadata
- SHA256 required
- repo/source/build/toolchain/lockfile/feature flag bindings required
- baseline program id, ProgramData, and authority bindings required
- local package must not include RPC/testnet
- local package must not authorize upgrade/write-buffer or mutation
- exact scoped user GO required before future build/hash execution
- no secret material in evidence
- any mismatch requires stop
- automatic retry rejected
- no execution approved

## Invariant review matrix

```text
# Expected-hash/build-hash evidence model invariant review matrix

SP4_INVARIANT_01_CURRENT_GO_NOT_GRANTED
status: reviewed
result: true
meaning: Scoped Package.3 records a decision model only and current GO remains not granted.

SP4_INVARIANT_02_STRICT_MODEL_REQUIRED
status: reviewed
result: true
meaning: Future expected-hash/build-hash evidence package must use the strict model.

SP4_INVARIANT_03_DECISION_MODEL_ONLY
status: reviewed
result: true
meaning: Scoped Package.4 is review-only and does not execute build/hash.

SP4_INVARIANT_04_FULL_HASH_BUNDLE_REQUIRED
status: reviewed
result: true
meaning: Future package must bind both local SBF artifact SHA256 and canonical ProgramData executable-bytes SHA256.

SP4_INVARIANT_05_CANONICAL_PROGRAMDATA_DOMAIN
status: reviewed
result: true
meaning: Canonical runtime hash domain is ProgramData executable bytes excluding loader metadata.

SP4_INVARIANT_06_SHA256_REQUIRED
status: reviewed
result: true
meaning: SHA256 is the selected hash algorithm.

SP4_INVARIANT_07_SOURCE_BUILD_TOOLCHAIN_BINDINGS
status: reviewed
result: true
meaning: Repo, branch, source commit, clean status, build command, toolchain, lockfiles, and feature flags must be bound.

SP4_INVARIANT_08_BASELINE_BINDINGS
status: reviewed
result: true
meaning: Program id, ProgramData account, and upgrade authority baseline must be bound.

SP4_INVARIANT_09_NO_RPC_TESTNET_LOCAL_PACKAGE
status: reviewed
result: true
meaning: Local expected-hash/build-hash evidence package must not include RPC/testnet.

SP4_INVARIANT_10_NO_UPGRADE_WRITE_BUFFER_MUTATION
status: reviewed
result: true
meaning: Expected-hash/build-hash evidence package does not authorize upgrade/write-buffer or mutation.

SP4_INVARIANT_11_EXACT_USER_GO_BEFORE_BUILD_HASH
status: reviewed
result: true
meaning: Future build/hash execution requires later exact scoped user GO phrase.

SP4_INVARIANT_12_NO_SECRETS
status: reviewed
result: true
meaning: Evidence must not include private keys, seed phrases, or secret material.

SP4_INVARIANT_13_STOP_NO_RETRY
status: reviewed
result: true
meaning: Any mismatch requires stop. Automatic retry is forbidden.

SP4_INVARIANT_14_NO_EXECUTION_APPROVED
status: reviewed
result: true
meaning: Scoped Package.4 does not approve build, hash computation, RPC, testnet, deploy, upgrade, write-buffer, state init, SPL setup, guardian package construction, signing, submit, or mutation.

SP4_AGGREGATE
all_invariants_reviewed: true
evidence_model_closure_ready: true
closure_type: narrow_expected_hash_build_hash_evidence_model_boundary_only
current_go_state: FINAL_GO_NOT_GRANTED
execution_approved: false
```

## Review result

all_invariants_reviewed: true

evidence_model_closure_ready: true

closure_type: narrow_expected_hash_build_hash_evidence_model_boundary_only

current_go_state: FINAL_GO_NOT_GRANTED

execution_approved: false

## Closure candidate prepared

STRICT_EXPECTED_HASH_BUILD_HASH_EVIDENCE_MODEL_REVIEWED_EXECUTION_NOT_APPROVED

Meaning:

- expected-hash/build-hash evidence model has been reviewed
- strict local evidence package model remains required
- full hash bundle remains required
- canonical ProgramData executable-bytes SHA256 domain remains required
- exact bindings remain required
- no-RPC/testnet boundary remains required
- no-upgrade/write-buffer/mutation boundary remains required
- exact scoped user GO remains required before build/hash execution
- stop-on-mismatch remains required
- automatic retry remains rejected
- no execution is approved
- current GO state remains FINAL_GO_NOT_GRANTED

## Remaining gaps

- Scoped Package.5 closure decision not recorded
- actual future execution package not drafted
- exact source commit not selected
- exact build command not selected
- exact toolchain versions not selected
- exact feature flags not selected
- exact artifact path not selected
- local SBF artifact SHA256 not computed
- canonical ProgramData executable-bytes SHA256 not computed
- expected-hash package id not generated
- exact scoped user GO phrase not selected
- build/hash execution remains not approved
- RPC/testnet remains not approved
- deploy/upgrade/write-buffer remains not approved
- mutation remains not approved

## Non-GO statement

Scoped Package.4 does not grant GO.

Scoped Package.4 does not approve:

- build
- local artifact hash computation
- ProgramData executable-bytes hash computation
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

SCOPED_PACKAGE_4_REVIEW_READY_EXPECTED_HASH_BUILD_HASH_EVIDENCE_INVARIANTS_RECORDED_NO_BUILD_NO_HASH_NO_RPC_NO_EXECUTION

Current decision:

EXPECTED_HASH_BUILD_HASH_EVIDENCE_MODEL_REVIEWED_FINAL_GO_NOT_GRANTED_PENDING_CLOSURE_DECISION

Closure candidate prepared:

STRICT_EXPECTED_HASH_BUILD_HASH_EVIDENCE_MODEL_REVIEWED_EXECUTION_NOT_APPROVED

Selected model:

STRICT_EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE_MODEL_REQUIRED

Current GO state:

FINAL_GO_NOT_GRANTED

execution_approved: false

NO-GO REMAINS_FOR_BUILD_HASH_DEPLOY_UPGRADE_WRITE_BUFFER_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Next safe step

Scoped Package.5 — expected-hash/build-hash evidence model closure decision record.

Scoped Package.5 may close the expected-hash/build-hash evidence model narrowly while keeping GO not granted.

Scoped Package.5 must not run build, compute hashes, call RPC, use testnet, deploy, upgrade, initialize state, configure SPL, construct packages, sign, submit, or mutate.

## Evidence preview

metadata:

```text
phase=scoped-package-4-expected-hash-build-hash-evidence-invariant-review
timestamp_utc=2026-07-06T22:12:27Z
repo_only=true
invariant_review_only=true
future_go_granted=false
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
