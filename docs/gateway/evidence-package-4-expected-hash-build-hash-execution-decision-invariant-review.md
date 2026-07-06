# Evidence Package.4 — Expected-hash/build-hash execution decision invariant review

Status:

EVIDENCE_PACKAGE_4_REVIEW_READY_EXPECTED_HASH_BUILD_HASH_EXECUTION_INVARIANTS_RECORDED_NO_BUILD_NO_HASH_NO_RPC_NO_EXECUTION

Current decision:

STRICT_LOCAL_BUILD_HASH_EXECUTION_MODEL_REVIEWED_FINAL_GO_NOT_GRANTED_PENDING_CLOSURE_DECISION

Closure candidate prepared:

STRICT_LOCAL_BUILD_HASH_EXECUTION_MODEL_REVIEWED_EXECUTION_NOT_APPROVED

Selected execution model:

STRICT_LOCAL_BUILD_HASH_EXECUTION_WITH_EXACT_GO_ONLY

Current GO state:

FINAL_GO_NOT_GRANTED

NO-GO REMAINS_FOR_BUILD_HASH_DEPLOY_UPGRADE_WRITE_BUFFER_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Purpose

Evidence Package.4 records the invariant review for the expected-hash/build-hash execution decision model.

This is invariant review only.

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

- Evidence Package.1 — exact expected-hash/build-hash execution package planning
- Evidence Package.2 — exact expected-hash/build-hash package values inventory
- Evidence Package.3 — expected-hash/build-hash execution decision model
- Scoped Package.5 — expected-hash/build-hash evidence model closure decision record
- Blocker B.5 — expected post-upgrade ProgramData hash closure decision record

## Reviewed invariants

- current GO is not granted
- exact scoped GO gate required
- clean repo before execution required
- exact source commit binding required
- toolchain version capture required
- exact build command binding required
- dangerous features rejected
- artifact existence and local SHA256 success criteria required
- canonical ProgramData executable-bytes SHA256 required
- canonical hash method missing or ambiguous is stop condition
- no RPC/testnet boundary required
- no deploy/upgrade/write-buffer/signing/submit/mutation boundary required
- no secret material boundary required
- stop on mismatch required
- automatic retry rejected
- future execution evidence file list required
- no execution approved now

## Invariant review matrix

```text
# Evidence Package.4 execution invariant review matrix

EP4_INVARIANT_01_CURRENT_GO_NOT_GRANTED
status: reviewed
result: true
meaning: Current GO state remains FINAL_GO_NOT_GRANTED.

EP4_INVARIANT_02_EXACT_GO_GATE_REQUIRED
status: reviewed
result: true
meaning: Future build/hash execution requires an exact scoped user GO phrase.

EP4_INVARIANT_03_CLEAN_REPO_REQUIRED
status: reviewed
result: true
meaning: Future build/hash execution requires clean working tree before execution.

EP4_INVARIANT_04_SOURCE_COMMIT_BOUND
status: reviewed
result: true
meaning: Future build/hash execution requires exact source commit binding.

EP4_INVARIANT_05_TOOLCHAIN_CAPTURE_REQUIRED
status: reviewed
result: true
meaning: Rust, Cargo, Solana CLI, and cargo-build-sbf versions must be captured.

EP4_INVARIANT_06_EXACT_BUILD_COMMAND_REQUIRED
status: reviewed
result: true
meaning: Only the package-bound local build command may run.

EP4_INVARIANT_07_NO_DANGEROUS_FEATURES
status: reviewed
result: true
meaning: Dangerous feature flags must not be selected.

EP4_INVARIANT_08_ARTIFACT_AND_HASH_SUCCESS_CRITERIA_REQUIRED
status: reviewed
result: true
meaning: Expected artifact must exist, be non-empty, and local SBF artifact SHA256 must be recorded.

EP4_INVARIANT_09_CANONICAL_PROGRAMDATA_HASH_REQUIRED
status: reviewed
result: true
meaning: Canonical ProgramData executable-bytes SHA256 must be recorded.

EP4_INVARIANT_10_CANONICAL_HASH_METHOD_REQUIRED
status: reviewed
result: true
meaning: Missing or ambiguous canonicalization method is a stop condition.

EP4_INVARIANT_11_NO_RPC_TESTNET
status: reviewed
result: true
meaning: Future local build/hash execution package must not call RPC or testnet.

EP4_INVARIANT_12_NO_MUTATION
status: reviewed
result: true
meaning: Future local build/hash execution package must not deploy, upgrade, write buffer, change authority, initialize state, configure SPL, construct guardian packages, sign, submit, or mutate.

EP4_INVARIANT_13_NO_SECRETS
status: reviewed
result: true
meaning: Future package must not request, print, or require private keys, seed phrases, or secret material.

EP4_INVARIANT_14_STOP_ON_MISMATCH
status: reviewed
result: true
meaning: Any mismatch requires stop.

EP4_INVARIANT_15_NO_AUTOMATIC_RETRY
status: reviewed
result: true
meaning: Automatic retry remains rejected.

EP4_INVARIANT_16_EVIDENCE_FILES_REQUIRED
status: reviewed
result: true
meaning: Future execution evidence file list is defined.

EP4_INVARIANT_17_NO_EXECUTION_APPROVED_NOW
status: reviewed
result: true
meaning: Evidence Package.4 does not approve build, hash computation, RPC, testnet, deploy, upgrade, write-buffer, signing, submit, or mutation.

EP4_AGGREGATE
all_invariants_reviewed: true
execution_model_closure_ready: true
closure_type: narrow_expected_hash_build_hash_execution_decision_model_boundary_only
current_go_state: FINAL_GO_NOT_GRANTED
execution_approved: false
```

## Success/stop boundary review

```text
# Evidence Package.4 success/stop boundary review

success_criteria_reviewed: true
stop_conditions_reviewed: true
no_rpc_testnet_boundary_reviewed: true
no_mutation_boundary_reviewed: true
no_secrets_boundary_reviewed: true
exact_go_gate_reviewed: true

success_requires:
- exact scoped user GO phrase
- clean repo before execution
- source commit binding
- toolchain capture
- exact build command
- no dangerous features
- artifact exists and is non-empty
- local artifact SHA256 recorded
- canonical ProgramData executable-bytes SHA256 recorded
- no RPC/testnet
- no mutation
- no secrets
- evidence complete
- final status captured

stop_required_on:
- missing or wrong GO phrase
- dirty repo before execution
- source commit mismatch
- missing toolchain
- build command mismatch
- dangerous feature selected
- build failure
- artifact missing or empty
- local hash failure
- canonical hash method missing or ambiguous
- canonical hash failure
- RPC/testnet attempt
- mutation attempt
- secret material
- unexpected output
- unexplained final git status

automatic_retry: rejected
```

## Review result

all_invariants_reviewed: true

execution_model_closure_ready: true

closure_type: narrow_expected_hash_build_hash_execution_decision_model_boundary_only

current_go_state: FINAL_GO_NOT_GRANTED

execution_approved: false

## Closure candidate prepared

STRICT_LOCAL_BUILD_HASH_EXECUTION_MODEL_REVIEWED_EXECUTION_NOT_APPROVED

Meaning:

- strict local build/hash execution model has been reviewed
- exact scoped GO remains required
- clean repo/source commit/toolchain/build command bindings remain required
- no dangerous features boundary remains required
- artifact/hash success criteria remain required
- stop conditions remain required
- no RPC/testnet boundary remains required
- no mutation boundary remains required
- no secrets boundary remains required
- no execution is approved
- current GO state remains FINAL_GO_NOT_GRANTED

## Remaining gaps

- Evidence Package.5 closure decision not recorded
- exact scoped user GO phrase not selected
- future execution package not closed
- toolchain versions not captured
- build not executed
- local SBF artifact SHA256 not computed
- canonical ProgramData executable-bytes SHA256 not computed
- RPC/testnet not approved
- deploy/upgrade/write-buffer not approved
- mutation not approved

## Non-GO boundary

```text
# Evidence Package.4 non-GO boundary

Evidence Package.4 does not grant GO.

Evidence Package.4 does not approve:
- build
- local artifact hash computation
- ProgramData executable-bytes hash computation
- RPC
- testnet
- deploy
- upgrade
- write-buffer
- authority change
- state initialization
- SPL setup
- guardian package construction
- signing
- transaction submit
- mutation
- production activation

Evidence Package.4 is invariant review only.
Any actual build/hash execution still requires a later exact scoped user GO phrase.
```

## Result

Current status:

EVIDENCE_PACKAGE_4_REVIEW_READY_EXPECTED_HASH_BUILD_HASH_EXECUTION_INVARIANTS_RECORDED_NO_BUILD_NO_HASH_NO_RPC_NO_EXECUTION

Current decision:

STRICT_LOCAL_BUILD_HASH_EXECUTION_MODEL_REVIEWED_FINAL_GO_NOT_GRANTED_PENDING_CLOSURE_DECISION

Closure candidate prepared:

STRICT_LOCAL_BUILD_HASH_EXECUTION_MODEL_REVIEWED_EXECUTION_NOT_APPROVED

Selected execution model:

STRICT_LOCAL_BUILD_HASH_EXECUTION_WITH_EXACT_GO_ONLY

Current GO state:

FINAL_GO_NOT_GRANTED

execution_approved: false

NO-GO REMAINS_FOR_BUILD_HASH_DEPLOY_UPGRADE_WRITE_BUFFER_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Next safe step

Evidence Package.5 — expected-hash/build-hash execution decision closure record.

Evidence Package.5 may close the strict local build/hash execution decision model narrowly while keeping GO not granted.

Evidence Package.5 must not run build, compute hashes, call RPC, use testnet, deploy, upgrade, initialize state, configure SPL, construct packages, sign, submit, or mutate.

## Evidence preview

metadata:

```text
phase=evidence-package-4-expected-hash-build-hash-execution-decision-invariant-review
timestamp_utc=2026-07-06T22:43:36Z
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
