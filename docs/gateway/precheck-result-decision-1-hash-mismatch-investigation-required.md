# Precheck Result Decision.1 — Hash mismatch investigation required

Status:

PRECHECK_RESULT_DECISION_1_HASH_MISMATCH_INVESTIGATION_REQUIRED_DEPLOY_UPGRADE_BLOCKED_NO_RPC_NO_MUTATION

Decision:

STOPPED_CORRECTLY_BY_READ_ONLY_PRECHECK

Classification:

HASH_MISMATCH_INVESTIGATION_REQUIRED_BEFORE_CATEGORIZATION

## Purpose

This checkpoint records the decision after Read-only Network Precheck Execution.1.

The read-only precheck stopped correctly after detecting a hash mismatch.

This is a repo-only decision checkpoint.

It does not call RPC.

It does not use testnet.

It does not read ProgramData.

It does not dump executable bytes.

It does not perform live hash comparison.

It does not deploy, upgrade, write buffer, sign, submit, or mutate.

## Theo verdict

~~~text
# Theo verdict record

verdict_timestamp_user_supplied: 2026-07-07

verdict:
APPROVE — Precheck Result Decision checkpoint. Deploy/upgrade remains blocked.

classification:
STOPPED_CORRECTLY_BY_READ_ONLY_PRECHECK

interpretation:
The read-only precheck functioned exactly as designed. Hash mismatch was detected, execution halted, zero mutation. This is a success of the safety system, not a failure.

recommended_classification:
investigation-required before categorization

not_allowed:
Do not proceed anyway.

recommended_next_steps:
1. Investigate observed binary.
2. Rebuild from current main.
3. Compare observed vs rebuilt.

critical_question:
Where did expected hash e68ada36... come from?
~~~

## Precheck mismatch summary

~~~text
# Precheck mismatch summary

precheck_status:
READ_ONLY_NETWORK_PRECHECK_EXECUTION_1_STOPPED_AFTER_EXACT_GO_MISMATCH_OR_READ_FAILURE_NO_MUTATION

program_id:
D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my

programdata_account_expected:
9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T

programdata_account_observed:
9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T

programdata_account_match:
true

upgrade_authority_expected:
DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc

upgrade_authority_observed:
DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc

upgrade_authority_match:
true

expected_executable_bytes_sha256:
e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1

observed_live_testnet_executable_bytes_sha256:
fd4d3b659ccaea4f5e24eca4d9e80ff808c43de1bf1ecef7315961751a085a7e

observed_live_testnet_executable_bytes_size:
38584

expected_buildhash_artifact_size:
20840

hash_match:
false

size_match_against_buildhash_artifact:
false

decision:
STOPPED_AFTER_HASH_MISMATCH_INVESTIGATION_REQUIRED

deploy_upgrade_status:
BLOCKED
~~~

## Expected hash origin

~~~text
# Expected hash origin

expected_hash:
e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1

source_document:
docs/gateway/buildhash-execution-2-local-build-hash-execution-after-exact-go.md

source_package_id:
BHX2_LOCAL_BUILD_HASH_ee0cb44f7d49

bound_program_source_commit:
ee0cb44f7d496e010e784608d0c7ccc8e84e7fb6

repo_commit_at_buildhash_execution:
0863ab1b14aaea1f2ca6b8803cadb7665abecb73

build_command:
cargo build-sbf --manifest-path programs/xxxl-svm/Cargo.toml --no-default-features

artifact_size_bytes_at_buildhash_execution:
20840

important_decision_note:
The expected hash was produced by BuildHash Execution.2, not by a fresh rebuild from current main 33874af99722cfe38912f1e9cee1b63c4bf1ef2f.

Therefore, Theo's stale-hash hypothesis remains plausible until a fresh local rebuild from current main is performed and compared.
~~~

## Decision summary

~~~text
# Precheck Result Decision.1

status:
PRECHECK_RESULT_DECISION_1_HASH_MISMATCH_INVESTIGATION_REQUIRED_DEPLOY_UPGRADE_BLOCKED_NO_RPC_NO_MUTATION

decision:
STOPPED_CORRECTLY_BY_READ_ONLY_PRECHECK

classification:
HASH_MISMATCH_INVESTIGATION_REQUIRED_BEFORE_CATEGORIZATION

not_classified_yet_as:
- expected stale testnet binary
- stale local expected hash
- wrong build artifact/hash domain
- wrong deployment target

current_likelihood_notes_from_theo:
- expected stale/different testnet binary: medium
- local expected hash stale: medium-high
- wrong build artifact/hash domain: low-medium
- wrong deployment target: low

reason:
ProgramData account and upgrade authority match expected values, but live testnet executable bytes hash differs from the expected local build hash.

critical_observation:
The expected hash originated from BuildHash Execution.2, not from a fresh rebuild from current main.

next_required_checkpoint:
local rebuild / observed-binary investigation package

deploy_upgrade_blocked:
true
~~~

## Investigation plan

~~~text
# Investigation plan

Next checkpoint should be local-only unless explicitly scoped otherwise.

Required investigation before deploy/upgrade:
1. Rebuild from current main using:
   cargo build-sbf --manifest-path programs/xxxl-svm/Cargo.toml --no-default-features

2. Compute local artifact SHA256.

3. Compare:
   - fresh local rebuild hash
   - old expected hash e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1
   - observed live testnet hash fd4d3b659ccaea4f5e24eca4d9e80ff808c43de1bf1ecef7315961751a085a7e

4. Interpret:
   - if fresh local rebuild == e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1, expected hash remains current and live testnet binary is different/stale/unknown.
   - if fresh local rebuild != e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1, expected hash is stale or build environment changed.
   - if fresh local rebuild == fd4d3b659ccaea4f5e24eca4d9e80ff808c43de1bf1ecef7315961751a085a7e, live testnet already matches current rebuild.
   - if fresh local rebuild differs from both, investigate build environment, features, SBF toolchain, or artifact/hash domain.

5. Optional later, after local rebuild:
   investigate observed binary origin without mutation.

Forbidden until classification:
- deploy
- upgrade
- write-buffer
- signing
- transaction submit
- mutation
- activation
~~~

## Non-GO boundary

~~~text
# Non-GO boundary

This decision checkpoint does not grant GO.

This checkpoint does not approve:
- new RPC calls
- testnet calls
- ProgramData reads
- executable bytes dumps
- live hash comparisons
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

This checkpoint only records a decision: the hash mismatch stopped correctly and investigation is required before categorization.
~~~

## Progress state

~~~text
# X1 Testnet Deploy Track progress state after Precheck Result Decision.1

✅ 0: repo sanity review before GO
✅ 1: local build/hash evidence
✅ 2: RONB — read-only network baseline model
✅ 3: RONPP1 — read-only precheck package draft
✅ 4: RONPP2 — requirements / invariant review
✅ 5: RONPP3 — exact read-only package closure
✅ 6: checkpoint + Theo review package
✅ 6R: Theo repo-grounded verdict
✅ 7: RONPP3 alignment to current main merge commit
✅ 8: Read-only Network Precheck Execution.1
✅ 9: Precheck Result Decision — hash mismatch stopped correctly

👉 10: local rebuild / observed-binary investigation package

⏭ 11: New build/hash after investigation
⏭ 12: Testnet deploy/upgrade package only after classification
⏭ 13: Testnet deploy/upgrade execution only after separate exact GO
⏭ 14: Post-deploy verification
⏭ 15: Separate activation path

blocked:
deploy/upgrade/write-buffer/sign/submit/mutation
~~~

## Result

decision_checkpoint_only: true

repo_only: true

decision: STOPPED_CORRECTLY_BY_READ_ONLY_PRECHECK

classification: HASH_MISMATCH_INVESTIGATION_REQUIRED_BEFORE_CATEGORIZATION

expected_hash: e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1

observed_hash: fd4d3b659ccaea4f5e24eca4d9e80ff808c43de1bf1ecef7315961751a085a7e

observed_size: 38584

expected_hash_source_package_id: BHX2_LOCAL_BUILD_HASH_ee0cb44f7d49

expected_hash_source_bound_program_source_commit: ee0cb44f7d496e010e784608d0c7ccc8e84e7fb6

expected_hash_source_repo_commit_at_execution: 0863ab1b14aaea1f2ca6b8803cadb7665abecb73

deploy_upgrade_blocked: true

rpc_used: false

testnet_used: false

programdata_read_executed: false

executable_bytes_dumped: false

live_hash_comparison_executed: false

deploy_executed: false

upgrade_executed: false

write_buffer_executed: false

signing_executed: false

submit_executed: false

mutation_executed: false

## Next safe step

Local rebuild / observed-binary investigation package.

No deploy, upgrade, write-buffer, signing, transaction submit, mutation, or activation is authorized.
