# Read-only Network Baseline.1 — Precheck package planning

Status:

READ_ONLY_NETWORK_BASELINE_1_OPEN_PRECHECK_PACKAGE_PLANNING_ONLY_NO_RPC_NO_TESTNET_NO_MUTATION

Current decision:

READ_ONLY_NETWORK_PRECHECK_PACKAGE_SHAPE_SELECTED_RPC_NOT_APPROVED

Current GO state:

READ_ONLY_NETWORK_GO_NOT_GRANTED

## Purpose

Read-only Network Baseline.1 plans the future read-only network precheck package.

This step is planning-only.

It does not call RPC.

It does not use testnet.

It does not read ProgramData from network.

It does not deploy, upgrade, write buffer, sign, submit, or mutate.

## Evidence basis

- BuildHash Execution.2 local build/hash evidence
- BuildHash Execution.1.3 GO package closure
- Upgrade authority custody map
- Blocker B.5 expected ProgramData hash model

## Precheck package shape

~~~text
# Read-only Network Baseline.1 precheck package shape

package_status: PLANNING_ONLY_NOT_EXECUTABLE
operation_class: READ_ONLY_NETWORK_BASELINE_PRECHECK_PLANNING_ONLY
current_rpc_approval: false
current_testnet_approval: false
current_mutation_approval: false

future_precheck_goal:
- read X1 testnet ProgramData account
- confirm ProgramData account identity
- confirm observed upgrade authority
- extract executable bytes or equivalent local-comparable representation
- compute observed executable-bytes SHA256
- compare observed hash to expected local canonical ProgramData executable-bytes SHA256
- record result as evidence

future_precheck_must_not_do:
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

future_precheck_requires_later_exact_go: true
~~~

## Baseline bindings

~~~text
# Read-only Network Baseline.1 baseline bindings

network_candidate: X1_TESTNET
rpc_status_now: NOT_APPROVED_NOT_USED
program_id: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
programdata_account: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
observed_upgrade_authority_public_key: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc

expected_local_artifact_sha256: e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1
expected_canonical_programdata_executable_bytes_sha256: e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1
canonical_runtime_hash_domain: PROGRAMDATA_EXECUTABLE_BYTES_EXCLUDING_LOADER_METADATA

hash_match_required_for_future_mutation_package: true
hash_mismatch_stop_condition: true
automatic_retry_rejected: true
~~~

## Future read-only precheck stop conditions

~~~text
# Future read-only network precheck stop conditions

stop_if:
- exact scoped GO phrase is missing or differs
- RPC endpoint is missing, ambiguous, or not explicitly selected
- network is not X1 testnet
- program id differs
- ProgramData account differs
- upgrade authority differs
- ProgramData cannot be read
- executable bytes cannot be extracted or canonicalized
- observed executable-bytes SHA256 cannot be computed
- observed executable-bytes SHA256 differs from expected hash
- any signing is requested
- any transaction submit is requested
- any deploy/upgrade/write-buffer/mutation action is attempted
- any secret material is requested, printed, or required

automatic_retry: rejected
~~~

## Non-GO boundary

~~~text
# Read-only Network Baseline.1 non-GO boundary

Read-only Network Baseline.1 does not grant GO.

Read-only Network Baseline.1 does not approve:
- RPC
- testnet call
- ProgramData read
- hash comparison against live network
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

Read-only Network Baseline.1 is planning-only.
A later exact scoped GO package is required before any read-only network precheck.
~~~

## Result

planning_only: true

rpc_used: false

testnet_used: false

programdata_read_executed: false

deploy_executed: false

upgrade_executed: false

write_buffer_executed: false

signing_executed: false

submit_executed: false

mutation_executed: false

## Next safe step

Read-only Network Baseline.2 — exact read-only precheck requirements inventory.

Read-only Network Baseline.2 must still not call RPC, use testnet, read ProgramData, sign, submit, or mutate.

## Evidence preview

metadata:

~~~text
phase=read-only-network-baseline-1-precheck-package-planning
timestamp_utc=2026-07-07T02:49:30Z
repo_only=true
planning_only=true
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
~~~
