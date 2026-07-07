# Read-only Network Precheck Package.3 — Exact scoped read-only precheck package closure record

Status:

READ_ONLY_NETWORK_PRECHECK_PACKAGE_3_CLOSED_EXACT_SCOPED_READ_ONLY_PRECHECK_PACKAGE_READY_FOR_USER_EXACT_GO_NO_RPC_NO_TESTNET_NO_PROGRAMDATA_READ_NO_MUTATION

Current decision:

EXACT_SCOPED_READ_ONLY_PRECHECK_PACKAGE_CLOSED_FINAL_READ_ONLY_GO_NOT_GRANTED_UNTIL_USER_EXACT_PHRASE

Operation class:

READ_ONLY_NETWORK_PRECHECK_ONLY

Current GO state:

READ_ONLY_PRECHECK_GO_NOT_GRANTED

Final package id:

RONPP3_READ_ONLY_PRECHECK_fead873b9d8d

Source commit:

fead873b9d8d4e018106d1167e6b27494b03d89e

Final exact GO phrase:

GO_RONPP_READ_ONLY_PRECHECK_ONLY_RONPP3_READ_ONLY_PRECHECK_fead873b9d8d_SOURCE_fead873b9d8d

## Purpose

Read-only Network Precheck Package.3 closes the exact scoped read-only network precheck package.

This step is closure-record-only.

It does not call RPC.

It does not use testnet.

It does not read ProgramData from network.

It does not dump executable bytes from network.

It does not perform live hash comparison.

It does not deploy, upgrade, write buffer, sign, submit, or mutate.

## Closure summary

~~~text
# Read-only Network Precheck Package.3 closure summary

status: READ_ONLY_NETWORK_PRECHECK_PACKAGE_3_CLOSED_EXACT_SCOPED_READ_ONLY_PRECHECK_PACKAGE_READY_FOR_USER_EXACT_GO_NO_RPC_NO_TESTNET_NO_PROGRAMDATA_READ_NO_MUTATION
decision: EXACT_SCOPED_READ_ONLY_PRECHECK_PACKAGE_CLOSED_FINAL_READ_ONLY_GO_NOT_GRANTED_UNTIL_USER_EXACT_PHRASE
operation_class: READ_ONLY_NETWORK_PRECHECK_ONLY

final_package_id: RONPP3_READ_ONLY_PRECHECK_fead873b9d8d
source_commit: fead873b9d8d4e018106d1167e6b27494b03d89e
source_short: fead873b9d8d
network: X1_TESTNET
rpc_endpoint: https://rpc.testnet.x1.xyz
program_id: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
programdata_account: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
expected_upgrade_authority: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc
expected_canonical_programdata_executable_bytes_sha256: e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1
canonical_hash_domain: PROGRAMDATA_EXECUTABLE_BYTES_EXCLUDING_LOADER_METADATA
hash_algorithm: SHA256

final_exact_go_phrase: GO_RONPP_READ_ONLY_PRECHECK_ONLY_RONPP3_READ_ONLY_PRECHECK_fead873b9d8d_SOURCE_fead873b9d8d
read_only_precheck_go_granted_now: false

closure_meaning:
- exact scoped read-only precheck package is closed
- user exact GO phrase is required before any RPC/testnet read
- current closure does not execute RPC
- current closure does not read ProgramData
- current closure does not dump executable bytes
- current closure does not compute live observed hash
- current closure does not perform live hash comparison
- current closure does not authorize signing, submit, or mutation

allowed_after_exact_go_only:
- read-only ProgramData/account precheck on X1 testnet
- read-only executable bytes dump
- local SHA256 computation of observed dump
- observed-vs-expected hash comparison
- evidence recording

not_allowed_even_after_read_only_go:
- signer flag
- keypair path
- secret material
- transaction submit
- deploy
- upgrade
- write-buffer
- authority change
- state initialization
- SPL setup
- guardian package construction
- mutation
- production activation
~~~

## Final exact GO phrase

~~~text
# Final exact GO phrase

phrase_status: FINAL_SELECTED_NOT_GRANTED_UNTIL_USER_REPEATS_VERBATIM

final_exact_go_phrase:
GO_RONPP_READ_ONLY_PRECHECK_ONLY_RONPP3_READ_ONLY_PRECHECK_fead873b9d8d_SOURCE_fead873b9d8d

meaning:
- authorize only READ_ONLY_NETWORK_PRECHECK_ONLY
- bind final package id
- bind source commit short
- bind X1 testnet
- bind exact RPC endpoint
- bind program id
- bind ProgramData account
- bind expected hash
- forbid signer/keypair/submit/mutation

read_only_precheck_go_granted_now: false
~~~

## Final read-only command set

~~~text
# Final read-only command set selected for future execution after exact GO

command_set_status: FINAL_SELECTED_NOT_EXECUTED
execution_requires_user_exact_go: true

rpc_endpoint: https://rpc.testnet.x1.xyz
program_id: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
programdata_account: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
future_evidence_dir: docs/gateway/evidence/read-only-network-precheck-execution-1-after-exact-go

future_commands:
1. solana account 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T --url https://rpc.testnet.x1.xyz --output json
2. solana account D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my --url https://rpc.testnet.x1.xyz --output json
3. solana program dump D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my docs/gateway/evidence/read-only-network-precheck-execution-1-after-exact-go/observed-xxxl-svm.so --url https://rpc.testnet.x1.xyz
4. sha256sum docs/gateway/evidence/read-only-network-precheck-execution-1-after-exact-go/observed-xxxl-svm.so

future_command_must_not_include:
- signer flag
- keypair path
- secret material
- transaction submit
- deploy
- upgrade
- write-buffer
- set-upgrade-authority
- close
- transfer
- initialize
- SPL setup
- guardian package construction
- mutation

executed_now: false
rpc_used_now: false
testnet_used_now: false
programdata_read_executed_now: false
live_hash_comparison_executed_now: false
~~~

## Final success criteria

~~~text
# Final future execution success criteria

success_requires:
- user provides exact GO phrase verbatim
- exact GO phrase equals GO_RONPP_READ_ONLY_PRECHECK_ONLY_RONPP3_READ_ONLY_PRECHECK_fead873b9d8d_SOURCE_fead873b9d8d
- clean working tree before execution
- source commit binding verified
- operation class is READ_ONLY_NETWORK_PRECHECK_ONLY
- network is X1_TESTNET
- RPC endpoint is https://rpc.testnet.x1.xyz
- program id is D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
- ProgramData account is 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
- expected upgrade authority is DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc
- expected hash is e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1
- ProgramData/account metadata captured
- executable bytes dump captured
- observed executable bytes SHA256 computed locally
- observed executable bytes SHA256 equals expected hash
- evidence files written
- no signer flag
- no keypair path
- no secret material
- no transaction submit
- no deploy
- no upgrade
- no write-buffer
- no authority change
- no state initialization
- no SPL setup
- no guardian package construction
- no mutation
- final status captured

all_success_criteria_required: true
~~~

## Final stop conditions

~~~text
# Final future execution stop conditions

stop_if:
- exact GO phrase is missing or differs from GO_RONPP_READ_ONLY_PRECHECK_ONLY_RONPP3_READ_ONLY_PRECHECK_fead873b9d8d_SOURCE_fead873b9d8d
- working tree is dirty before execution
- source commit binding cannot be verified
- operation class differs
- RPC endpoint differs from https://rpc.testnet.x1.xyz
- network is not X1_TESTNET
- program id differs from D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
- ProgramData account differs from 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
- observed upgrade authority differs from DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc
- ProgramData/account metadata cannot be read
- executable bytes cannot be dumped
- observed executable bytes SHA256 cannot be computed
- observed executable bytes SHA256 differs from e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1
- command contains signer flag
- command contains keypair path
- command requests or prints secret material
- command can submit a transaction
- command can deploy, upgrade, write-buffer, close, transfer, initialize, configure SPL, construct guardian packages, or mutate
- final status is missing or unexplained

automatic_retry: rejected
mutation_after_precheck: not_authorized
~~~

## Allowed next step

~~~text
# Allowed next step after RONPP3

next_safe_step:
Read-only Network Precheck Execution.1 — after exact user GO phrase only.

exact_user_go_required:
GO_RONPP_READ_ONLY_PRECHECK_ONLY_RONPP3_READ_ONLY_PRECHECK_fead873b9d8d_SOURCE_fead873b9d8d

allowed_only_after_exact_go:
- verify exact GO phrase
- verify clean tree
- verify package bindings
- run final read-only command set
- write evidence
- compare observed SHA256 to expected SHA256

not_allowed_without_exact_go:
- RPC
- testnet call
- ProgramData read
- executable bytes dump
- live hash comparison

not_allowed_even_with_read_only_go:
- signer flag
- keypair path
- secret material
- transaction submit
- deploy
- upgrade
- write-buffer
- authority change
- state initialization
- SPL setup
- guardian package construction
- mutation
~~~

## Remaining gaps

~~~text
# Read-only Network Precheck Package.3 remaining gaps

- user exact GO phrase not provided
- Read-only Network Precheck Execution.1 not executed
- RPC/testnet read not executed
- ProgramData/account metadata not read
- observed upgrade authority not re-read
- observed executable bytes not dumped
- observed executable bytes SHA256 not computed
- observed-vs-expected hash comparison not executed
- signing/submit/mutation not approved
~~~

## Non-GO boundary

~~~text
# Read-only Network Precheck Package.3 non-GO boundary

Read-only Network Precheck Package.3 does not grant GO.

Read-only Network Precheck Package.3 does not execute:
- RPC
- testnet call
- ProgramData read
- live account inspection
- executable bytes dump
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

Read-only Network Precheck Package.3 only closes the exact scoped read-only package.
The user must provide the final exact GO phrase verbatim before any RPC/testnet read.
~~~

## Result

closure_record_only: true

exact_scoped_read_only_precheck_package_closed: true

operation_class: READ_ONLY_NETWORK_PRECHECK_ONLY

final_package_id: RONPP3_READ_ONLY_PRECHECK_fead873b9d8d

source_commit: fead873b9d8d4e018106d1167e6b27494b03d89e

source_short: fead873b9d8d

final_exact_go_phrase: GO_RONPP_READ_ONLY_PRECHECK_ONLY_RONPP3_READ_ONLY_PRECHECK_fead873b9d8d_SOURCE_fead873b9d8d

network: X1_TESTNET

rpc_endpoint: https://rpc.testnet.x1.xyz

program_id: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my

programdata_account: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T

expected_upgrade_authority: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc

expected_canonical_programdata_executable_bytes_sha256: e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1

canonical_hash_domain: PROGRAMDATA_EXECUTABLE_BYTES_EXCLUDING_LOADER_METADATA

hash_algorithm: SHA256

read_only_precheck_go_granted: false

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

Read-only Network Precheck Execution.1 — after exact user GO phrase only.

Required exact user GO phrase:

~~~text
GO_RONPP_READ_ONLY_PRECHECK_ONLY_RONPP3_READ_ONLY_PRECHECK_fead873b9d8d_SOURCE_fead873b9d8d
~~~

Until the exact phrase is provided, no RPC/testnet/ProgramData read/live hash comparison is authorized.

## Evidence preview

metadata:

~~~text
phase=read-only-network-precheck-package-3-exact-scoped-read-only-precheck-package-closure-record
timestamp_utc=2026-07-07T05:32:08Z
repo_only=true
closure_record_only=true
source_commit=fead873b9d8d4e018106d1167e6b27494b03d89e
source_short=fead873b9d8d
final_package_id=RONPP3_READ_ONLY_PRECHECK_fead873b9d8d
final_exact_go_phrase=GO_RONPP_READ_ONLY_PRECHECK_ONLY_RONPP3_READ_ONLY_PRECHECK_fead873b9d8d_SOURCE_fead873b9d8d
rpc_endpoint=https://rpc.testnet.x1.xyz
rpc_used=false
testnet_used=false
programdata_read_executed=false
live_hash_comparison_executed=false
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
