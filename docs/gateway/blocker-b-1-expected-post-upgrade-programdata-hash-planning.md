# Blocker B.1 — Expected post-upgrade ProgramData hash planning

Status:

BLOCKER_B_OPEN_EXPECTED_POST_UPGRADE_PROGRAMDATA_HASH_PLANNING_ONLY_NO_BUILD_NO_RPC_NO_EXECUTION

Current decision:

BLOCKER_B_NOT_CLOSED

NO-GO REMAINS_FOR_BUILD_HASH_DEPLOY_UPGRADE_WRITE_BUFFER_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Purpose

Blocker B.1 opens the expected post-upgrade ProgramData hash track.

B.1 is planning-only.

It does not run build.

It does not compute artifact hash.

It does not compute ProgramData hash.

It does not write a buffer.

It does not deploy.

It does not upgrade.

It does not change authority.

It does not initialize state.

It does not configure SPL.

It does not construct guardian packages.

It does not sign.

It does not call RPC.

It does not use testnet.

It does not submit or mutate any network.

## Why B is last

Blockers A, C, D, E, F, G, and H are closed narrowly.

Those closures still do not approve execution.

Blocker B remains open because the expected post-upgrade ProgramData hash has not yet been defined and recorded.

Before any final scoped GO can be considered, the project needs a clear expected-hash model and evidence package.

## Planning questions

```text
# Expected post-upgrade ProgramData hash planning questions

B1_QUESTION_01_HASH_DOMAIN
What exactly is the expected hash domain: local SBF artifact bytes, buffer account data, ProgramData executable bytes, raw ProgramData account data, or a bundle of several hashes?

B1_QUESTION_02_HASH_ALGORITHM
Which hash algorithm is canonical for this blocker: sha256, keccak256, or another repo-standard hash?

B1_QUESTION_03_PROGRAMDATA_METADATA
Should ProgramData loader metadata be included in the expected hash or excluded?

B1_QUESTION_04_ARTIFACT_SOURCE
Which artifact path is authoritative for the future expected post-upgrade hash?

B1_QUESTION_05_BUILD_REPRODUCIBILITY
Which build command, toolchain, features, environment, lockfiles, and source commit must be bound to the expected hash?

B1_QUESTION_06_FEATURE_FLAGS
Which feature flags are allowed or forbidden for the expected post-upgrade artifact?

B1_QUESTION_07_DANGEROUS_GATES
How are dangerous feature gates represented in the hash evidence bundle?

B1_QUESTION_08_CURRENT_PROGRAMDATA_BASELINE
Which observed ProgramData account and current deployed program id are used as baseline evidence?

B1_QUESTION_09_PRE_UPGRADE_EXPECTED_HASH
What evidence must exist before any upgrade is allowed?

B1_QUESTION_10_POST_UPGRADE_READ_ONLY_VERIFICATION
What read-only observation must verify that the post-upgrade ProgramData matches the expected hash?

B1_QUESTION_11_MISMATCH_POLICY
What happens if local artifact hash, buffer hash, expected ProgramData hash, or observed post-upgrade hash mismatch?

B1_QUESTION_12_NO_AUTOMATIC_RETRY
How does the no-automatic-retry policy from Blocker G apply to hash mismatch?

B1_QUESTION_13_FINAL_GO_BOUNDARY
How does the expected hash package compose with the final scoped GO package?

B1_QUESTION_14_NON_REVERSIBLE_PATH
If an upgrade has occurred and the post-upgrade hash is wrong, when is the path rollback, abandon, or redeploy?

B1_QUESTION_15_SCOPE_LIMIT
Which B steps may run build/hash locally, and which steps remain strictly repo-only planning?
```

## Candidate models

```text
# Expected post-upgrade ProgramData hash candidate models

B1_MODEL_0_NO_EXPECTED_HASH
status: rejected_candidate
meaning: Proceed toward upgrade without a recorded expected post-upgrade hash.
reason_rejected: unsafe and incompatible with final scoped GO.

B1_MODEL_1_SOURCE_COMMIT_ONLY
status: rejected_candidate
meaning: Use only git commit hash as expected upgrade identity.
reason_rejected: source commit does not prove build artifact bytes or ProgramData bytes.

B1_MODEL_2_LOCAL_ARTIFACT_SHA_ONLY
status: partial_candidate
meaning: Record sha256 of local SBF artifact only.
limitation: Does not prove buffer bytes or post-upgrade ProgramData bytes.

B1_MODEL_3_PROGRAMDATA_EXECUTABLE_BYTES_HASH
status: preferred_candidate
meaning: Expected post-upgrade hash is computed over the executable program bytes stored in ProgramData, excluding loader metadata.
reason_preferred: closer to what the program actually runs while avoiding mutable loader metadata.

B1_MODEL_4_FULL_HASH_BUNDLE
status: preferred_required_property
meaning: Record source commit, build command, toolchain, feature flags, artifact hash, expected ProgramData executable bytes hash, baseline ProgramData, and post-upgrade read-only verification plan.
reason_preferred: gives deterministic evidence before and after upgrade.

B1_MODEL_5_RAW_PROGRAMDATA_ACCOUNT_HASH
status: open_question
meaning: Hash the full raw ProgramData account data including loader metadata.
open_issue: metadata may include deployment-specific fields and may not be stable before upgrade.

B1_MODEL_6_BUFFER_HASH_PLUS_POST_UPGRADE_PROGRAMDATA_HASH
status: open_question
meaning: Use buffer data hash before upgrade and ProgramData executable bytes hash after upgrade.
open_issue: requires future scoped procedure for buffer observation without unsafe mutation.

PREFERRED_DIRECTION_FOR_LATER_B_STEPS
FULL_HASH_BUNDLE_WITH_PROGRAMDATA_EXECUTABLE_BYTES_HASH_AS_CANONICAL_RUNTIME_HASH

B1_EXECUTION_BOUNDARY
B1_DOES_NOT_BUILD_HASH_DEPLOY_UPGRADE_CALL_RPC_USE_TESTNET_SUBMIT_OR_MUTATE
```

## Scope

```text
# Blocker B.1 scope

IN_SCOPE_FOR_B1_PLANNING:
- define hash-domain questions
- define candidate hash models
- define artifact/source/toolchain binding questions
- define future pre-upgrade expected-hash evidence requirement
- define future post-upgrade read-only verification requirement
- define mismatch/no-automatic-retry questions
- define relationship to final scoped GO package

OUT_OF_SCOPE_FOR_B1_EXECUTION:
- running build
- computing artifact hash
- computing ProgramData hash
- writing buffer
- deploying
- upgrading
- changing authority
- initializing state
- configuring SPL
- constructing guardian packages
- signing
- calling RPC
- using testnet
- submitting transactions
- mutating any network
```

## Initial direction

B.1 does not select the final hash model.

However, the preferred direction for later B steps is:

- full hash bundle
- canonical runtime hash over ProgramData executable bytes
- source commit binding
- build command binding
- toolchain/version binding
- feature flag binding
- artifact hash binding
- baseline ProgramData evidence binding
- post-upgrade read-only verification plan
- mismatch stop condition
- no automatic retry
- explicit scoped user GO before any build/hash/upgrade step that is outside pure planning

## Non-closure statement

B.1 does not close Blocker B.

B.1 does not approve:

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

BLOCKER_B_OPEN_EXPECTED_POST_UPGRADE_PROGRAMDATA_HASH_PLANNING_ONLY_NO_BUILD_NO_RPC_NO_EXECUTION

Current decision:

BLOCKER_B_NOT_CLOSED

NO-GO REMAINS_FOR_BUILD_HASH_DEPLOY_UPGRADE_WRITE_BUFFER_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Next safe step

Blocker B.2 — repo-grounded ProgramData hash inventory.

B.2 should inspect tracked repository code and docs only.

B.2 must not run build, compute hashes, deploy, upgrade, initialize state, configure SPL, construct packages, sign, call RPC, use testnet, submit, or mutate.

## Evidence preview

metadata:

```text
phase=blocker-b-1-expected-post-upgrade-programdata-hash-planning
timestamp_utc=2026-07-06T20:11:39Z
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
