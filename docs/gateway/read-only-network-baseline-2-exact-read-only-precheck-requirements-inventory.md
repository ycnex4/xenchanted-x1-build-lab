# Read-only Network Baseline.2 — Exact read-only precheck requirements inventory

Status:

READ_ONLY_NETWORK_BASELINE_2_OPEN_EXACT_READ_ONLY_PRECHECK_REQUIREMENTS_INVENTORY_NO_RPC_NO_TESTNET_NO_MUTATION

Current decision:

READ_ONLY_PRECHECK_REQUIREMENTS_INVENTORIED_RPC_NOT_APPROVED

Current GO state:

READ_ONLY_NETWORK_GO_NOT_GRANTED

## Purpose

Read-only Network Baseline.2 inventories the exact requirements for a future read-only network precheck.

This step is requirements inventory only.

It does not call RPC.

It does not use testnet.

It does not read ProgramData from network.

It does not perform live hash comparison.

It does not deploy, upgrade, write buffer, sign, submit, or mutate.

## Evidence basis

- Read-only Network Baseline.1 precheck package planning
- BuildHash Execution.2 local build/hash evidence
- Upgrade authority custody map
- Blocker B.5 expected ProgramData hash model

## Requirements inventory

~~~text
# Read-only Network Baseline.2 requirements inventory

operation_class: READ_ONLY_NETWORK_BASELINE_PRECHECK_REQUIREMENTS_INVENTORY_ONLY
requirements_status: INVENTORIED_NOT_EXECUTABLE
current_go_state: READ_ONLY_NETWORK_GO_NOT_GRANTED

RONB2_REQ_01_EXACT_NETWORK_BINDING:
required: true
current_value: X1_TESTNET
status: inventoried

RONB2_REQ_02_RPC_ENDPOINT_SELECTION:
required: true
current_value: UNSET_NOT_SELECTED
status: deferred_to_future_exact_precheck_package

RONB2_REQ_03_PROGRAM_ID_BINDING:
required: true
current_value: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
status: inventoried

RONB2_REQ_04_PROGRAMDATA_ACCOUNT_BINDING:
required: true
current_value: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
status: inventoried

RONB2_REQ_05_EXPECTED_UPGRADE_AUTHORITY_BINDING:
required: true
current_value: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc
status: inventoried

RONB2_REQ_06_EXPECTED_HASH_BINDING:
required: true
current_value: e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1
status: inventoried

RONB2_REQ_07_CANONICAL_HASH_DOMAIN:
required: true
current_value: PROGRAMDATA_EXECUTABLE_BYTES_EXCLUDING_LOADER_METADATA
status: inventoried

RONB2_REQ_08_HASH_ALGORITHM:
required: true
current_value: SHA256
status: inventoried

RONB2_REQ_09_READ_ONLY_METHOD_SELECTION:
required: true
current_value: UNSET_NOT_SELECTED
status: deferred_to_future_exact_precheck_package

RONB2_REQ_10_EVIDENCE_PATH_SELECTION:
required: true
current_value: UNSET_NOT_SELECTED
status: deferred_to_future_exact_precheck_package

RONB2_REQ_11_EXACT_GO_PHRASE_SELECTION:
required: true
current_value: UNSET_NOT_SELECTED
status: deferred_to_future_exact_precheck_package

RONB2_REQ_12_NO_SIGNING:
required: true
current_value: signing_forbidden
status: inventoried

RONB2_REQ_13_NO_TRANSACTION_SUBMIT:
required: true
current_value: submit_forbidden
status: inventoried

RONB2_REQ_14_NO_MUTATION:
required: true
current_value: mutation_forbidden
status: inventoried

RONB2_REQ_15_STOP_ON_MISMATCH:
required: true
current_value: stop_on_any_identity_authority_hash_method_boundary_mismatch
status: inventoried

RONB2_REQ_16_NO_AUTOMATIC_RETRY:
required: true
current_value: automatic_retry_rejected
status: inventoried
~~~

## Read-only precheck data requirements

~~~text
# Read-only precheck data requirements

future_precheck_must_capture:
- exact user GO phrase
- selected RPC endpoint
- network identity
- current timestamp UTC
- program id queried
- ProgramData account queried
- ProgramData account owner / loader identity if available
- observed upgrade authority
- observed executable bytes or canonical equivalent
- observed executable bytes length
- observed executable bytes SHA256
- expected executable bytes SHA256
- hash comparison result
- no-signing evidence
- no-submit evidence
- no-mutation evidence
- final command status

future_precheck_success_requires:
- exact GO phrase matches
- RPC endpoint explicitly selected
- network is X1 testnet
- program id matches expected
- ProgramData account matches expected
- observed upgrade authority matches expected
- observed executable bytes extracted/canonicalized successfully
- observed executable bytes SHA256 computed successfully
- observed executable bytes SHA256 equals expected hash
- no signing
- no transaction submit
- no mutation
- evidence files written

future_precheck_result_if_hash_mismatch:
- stop
- record mismatch
- do not retry automatically
- do not proceed to mutation package
~~~

## Future command shape inventory

~~~text
# Future read-only command shape inventory

command_status: INVENTORY_ONLY_NOT_EXECUTABLE
rpc_status_now: NOT_APPROVED_NOT_USED

future_read_only_command_categories:
- read account data for ProgramData account
- inspect program account / ProgramData relationship if required
- decode upgradeable loader metadata if required
- extract executable bytes or equivalent canonical representation
- compute SHA256 locally
- compare against expected hash

future_command_must_not_include:
- solana program deploy
- solana program write-buffer
- solana program set-upgrade-authority
- solana program close
- solana transfer
- any transaction submit
- any signer flag
- any keypair path
- any secret material
- any state initialization
- any SPL setup
- any guardian package construction

future_exact_commands: UNSET_NOT_SELECTED
future_rpc_endpoint: UNSET_NOT_SELECTED
future_exact_go_phrase: UNSET_NOT_SELECTED
~~~

## Remaining gaps

~~~text
# Read-only Network Baseline.2 remaining gaps

- future exact read-only precheck package not drafted
- RPC endpoint not selected
- exact read-only command set not selected
- exact evidence path not selected
- exact GO phrase not selected
- ProgramData read not executed
- observed upgrade authority not re-read
- observed executable bytes not extracted
- observed executable bytes SHA256 not computed
- observed-vs-expected hash comparison not executed
- RPC/testnet not approved
- signing/submit/mutation not approved
~~~

## Non-GO boundary

~~~text
# Read-only Network Baseline.2 non-GO boundary

Read-only Network Baseline.2 does not grant GO.

Read-only Network Baseline.2 does not approve:
- RPC
- testnet call
- ProgramData read
- live account inspection
- live hash comparison
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

Read-only Network Baseline.2 is requirements inventory only.
A later exact scoped read-only precheck package is required before any RPC/testnet read.
~~~

## Result

requirements_inventory_only: true

rpc_used: false

testnet_used: false

programdata_read_executed: false

live_hash_comparison_executed: false

deploy_executed: false

upgrade_executed: false

write_buffer_executed: false

signing_executed: false

submit_executed: false

mutation_executed: false

## Next safe step

Read-only Network Baseline.3 — exact read-only precheck decision model.

Read-only Network Baseline.3 must still not call RPC, use testnet, read ProgramData, sign, submit, or mutate.

## Evidence preview

metadata:

~~~text
phase=read-only-network-baseline-2-exact-read-only-precheck-requirements-inventory
timestamp_utc=2026-07-07T02:54:25Z
repo_only=true
requirements_inventory_only=true
rpc_used=false
testnet_used=false
programdata_read_executed=false
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
