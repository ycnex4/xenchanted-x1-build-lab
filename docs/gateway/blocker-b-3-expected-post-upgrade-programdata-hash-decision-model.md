# Blocker B.3 — Expected post-upgrade ProgramData hash decision model

Status:

BLOCKER_B_OPEN_EXPECTED_POST_UPGRADE_PROGRAMDATA_HASH_DECISION_MODEL_RECORDED_NO_BUILD_NO_HASH_NO_RPC_NO_EXECUTION

Current decision:

FULL_HASH_BUNDLE_WITH_PROGRAMDATA_EXECUTABLE_BYTES_SHA256_AS_CANONICAL_RUNTIME_HASH

Canonical hash domain:

PROGRAMDATA_EXECUTABLE_BYTES_EXCLUDING_LOADER_METADATA

Canonical hash algorithm:

SHA256

Local artifact hash policy:

LOCAL_SBF_ARTIFACT_SHA256_REQUIRED_AS_FUTURE_PRE_UPGRADE_EVIDENCE

Build binding policy:

SOURCE_COMMIT_BUILD_COMMAND_TOOLCHAIN_LOCKFILES_AND_FEATURE_FLAGS_REQUIRED

Baseline binding:

BASELINE_PROGRAM_ID_AND_PROGRAMDATA_ACCOUNT_REQUIRED

Pre-upgrade policy:

EXPECTED_HASH_PACKAGE_REQUIRED_BEFORE_ANY_UPGRADE_GO

Post-upgrade policy:

READ_ONLY_PROGRAMDATA_EXECUTABLE_BYTES_SHA256_VERIFICATION_REQUIRED_AFTER_UPGRADE

Mismatch policy:

HASH_MISMATCH_IS_STOP_CONDITION_NO_AUTOMATIC_RETRY

User GO policy:

EXPLICIT_SCOPED_USER_GO_REQUIRED_BEFORE_ANY_BUILD_HASH_UPGRADE_OR_RECOVERY_ACTION

Execution boundary:

FUTURE_FINAL_SCOPED_GO_REQUIRED_BEFORE_ANY_BUILD_HASH_DEPLOY_UPGRADE_WRITE_BUFFER_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGE_SIGNING_SUBMIT_OR_MUTATION

NO-GO REMAINS_FOR_BUILD_HASH_DEPLOY_UPGRADE_WRITE_BUFFER_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Purpose

Blocker B.3 records the expected post-upgrade ProgramData hash decision model.

B.3 is decision-model only.

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

## Background

B.1 opened expected post-upgrade ProgramData hash planning.

B.2 completed repo-grounded ProgramData hash inventory.

B.3 selects the hash model and hash-bundle requirements without computing hashes.

## Selected model

FULL_HASH_BUNDLE_WITH_PROGRAMDATA_EXECUTABLE_BYTES_SHA256_AS_CANONICAL_RUNTIME_HASH

Meaning:

- canonical runtime hash is sha256 over ProgramData executable bytes
- loader metadata is excluded from the canonical runtime hash domain
- local SBF artifact sha256 is still required as future pre-upgrade evidence
- source commit, build command, toolchain, lockfiles, and feature flags must be bound
- baseline program id and ProgramData account must be bound
- expected hash package is required before any upgrade GO
- post-upgrade read-only verification is required
- hash mismatch is a stop condition
- automatic retry remains rejected

## Required future hash bundle fields

- source commit
- repo clean status
- build command
- toolchain versions
- lockfiles
- feature flags
- dangerous feature gate status
- local SBF artifact path
- local SBF artifact sha256
- canonical ProgramData executable-bytes sha256
- baseline program id
- baseline ProgramData account
- baseline upgrade authority observation
- pre-upgrade expected hash package
- post-upgrade read-only verification procedure
- mismatch stop condition
- explicit scoped user GO reference

## Rejected shortcuts

- source commit only
- local artifact hash only
- raw ProgramData account hash as canonical runtime hash
- upgrade without expected post-upgrade hash
- automatic retry after hash mismatch
- continuing after missing post-upgrade read-only verification
- build/hash/upgrade/submit inside B.3

## Decision matrix

```text
# Expected post-upgrade ProgramData hash decision matrix

B3_MODEL_0_NO_EXPECTED_HASH
status: rejected
meaning: Proceed toward upgrade without a recorded expected post-upgrade hash.
reason_rejected: unsafe and incompatible with final scoped GO.

B3_MODEL_1_SOURCE_COMMIT_ONLY
status: rejected
meaning: Use only git commit hash as expected upgrade identity.
reason_rejected: source commit does not prove build artifact bytes or ProgramData executable bytes.

B3_MODEL_2_LOCAL_ARTIFACT_SHA_ONLY
status: rejected_as_insufficient
meaning: Record sha256 of local SBF artifact only.
reason_insufficient: local artifact hash alone does not prove post-upgrade ProgramData executable bytes.

B3_MODEL_3_RAW_PROGRAMDATA_ACCOUNT_HASH
status: rejected_as_canonical_runtime_hash
meaning: Hash full raw ProgramData account data including loader metadata.
reason_rejected: loader metadata may include deployment-specific fields and is not the canonical runtime executable-byte identity.

B3_MODEL_4_PROGRAMDATA_EXECUTABLE_BYTES_SHA256
status: selected_canonical_runtime_hash
meaning: Canonical runtime hash is sha256 over ProgramData executable bytes, excluding loader metadata.
reason_selected: closest to what the program actually runs while avoiding loader metadata instability.

B3_MODEL_5_FULL_HASH_BUNDLE
status: selected_required_package
meaning: Future expected-hash package must record source commit, build command, toolchain, lockfiles, feature flags, artifact sha256, expected ProgramData executable-bytes sha256, baseline ProgramData, and post-upgrade read-only verification plan.
reason_selected: provides deterministic pre-upgrade and post-upgrade evidence.

SELECTED_B3_DECISION
FULL_HASH_BUNDLE_WITH_PROGRAMDATA_EXECUTABLE_BYTES_SHA256_AS_CANONICAL_RUNTIME_HASH

SELECTED_B3_CANONICAL_HASH_DOMAIN
PROGRAMDATA_EXECUTABLE_BYTES_EXCLUDING_LOADER_METADATA

SELECTED_B3_CANONICAL_HASH_ALGORITHM
SHA256

SELECTED_B3_LOCAL_ARTIFACT_HASH_POLICY
LOCAL_SBF_ARTIFACT_SHA256_REQUIRED_AS_FUTURE_PRE_UPGRADE_EVIDENCE

SELECTED_B3_BUILD_BINDING_POLICY
SOURCE_COMMIT_BUILD_COMMAND_TOOLCHAIN_LOCKFILES_AND_FEATURE_FLAGS_REQUIRED

SELECTED_B3_BASELINE_BINDING
BASELINE_PROGRAM_ID_AND_PROGRAMDATA_ACCOUNT_REQUIRED

SELECTED_B3_PRE_UPGRADE_POLICY
EXPECTED_HASH_PACKAGE_REQUIRED_BEFORE_ANY_UPGRADE_GO

SELECTED_B3_POST_UPGRADE_POLICY
READ_ONLY_PROGRAMDATA_EXECUTABLE_BYTES_SHA256_VERIFICATION_REQUIRED_AFTER_UPGRADE

SELECTED_B3_MISMATCH_POLICY
HASH_MISMATCH_IS_STOP_CONDITION_NO_AUTOMATIC_RETRY

SELECTED_B3_USER_GO_POLICY
EXPLICIT_SCOPED_USER_GO_REQUIRED_BEFORE_ANY_BUILD_HASH_UPGRADE_OR_RECOVERY_ACTION

SELECTED_B3_EXECUTION_BOUNDARY
FUTURE_FINAL_SCOPED_GO_REQUIRED_BEFORE_ANY_BUILD_HASH_DEPLOY_UPGRADE_WRITE_BUFFER_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGE_SIGNING_SUBMIT_OR_MUTATION
```

## Remaining open items before B closure

- ProgramData hash invariant review package
- Blocker B closure decision record
- future actual expected-hash package
- future actual build/hash execution with explicit scoped GO
- future final scoped GO package before any network mutation

## Non-closure statement

B.3 does not close Blocker B.

B.3 does not approve:

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

BLOCKER_B_OPEN_EXPECTED_POST_UPGRADE_PROGRAMDATA_HASH_DECISION_MODEL_RECORDED_NO_BUILD_NO_HASH_NO_RPC_NO_EXECUTION

Current decision:

FULL_HASH_BUNDLE_WITH_PROGRAMDATA_EXECUTABLE_BYTES_SHA256_AS_CANONICAL_RUNTIME_HASH

Execution boundary:

FUTURE_FINAL_SCOPED_GO_REQUIRED_BEFORE_ANY_BUILD_HASH_DEPLOY_UPGRADE_WRITE_BUFFER_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGE_SIGNING_SUBMIT_OR_MUTATION

NO-GO REMAINS_FOR_BUILD_HASH_DEPLOY_UPGRADE_WRITE_BUFFER_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Next safe step

Blocker B.4 — ProgramData hash invariant review package.

B.4 should review canonical hash domain, sha256 algorithm, full hash bundle, baseline binding, mismatch stop condition, no automatic retry, explicit scoped user GO, and no-execution boundary.

B.4 must not run build, compute hashes, deploy, upgrade, initialize state, configure SPL, construct packages, sign, call RPC, use testnet, submit, or mutate.

## Evidence preview

metadata:

```text
phase=blocker-b-3-expected-post-upgrade-programdata-hash-decision-model
timestamp_utc=2026-07-06T20:24:04Z
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
