# Read-only Network Precheck Package.2 — Exact scoped read-only precheck package requirements/invariant review

Status:

READ_ONLY_NETWORK_PRECHECK_PACKAGE_2_REVIEW_READY_EXACT_SCOPED_READ_ONLY_PRECHECK_PACKAGE_REQUIREMENTS_INVARIANTS_RECORDED_NO_RPC_NO_TESTNET_NO_PROGRAMDATA_READ_NO_MUTATION

Current decision:

EXACT_SCOPED_READ_ONLY_PRECHECK_PACKAGE_REVIEWED_GO_NOT_GRANTED_PENDING_CLOSURE

Closure candidate:

EXACT_SCOPED_READ_ONLY_PRECHECK_PACKAGE_REVIEWED_GO_NOT_GRANTED_EXECUTION_NOT_APPROVED

Current GO state:

READ_ONLY_PRECHECK_GO_NOT_GRANTED

Reviewed draft package id:

RONPP1_DRAFT_f3bff313af20

Reviewed RONPP1 source commit:

f3bff313af205285d4635fa709fc04f162edbb05

Current repo commit at review:

eed37774c40c1ce1f028435d9d3e08015108d4b1

## Purpose

Read-only Network Precheck Package.2 reviews the requirements and invariants of the RONPP1 draft package.

This step is requirements/invariant review only.

It does not call RPC.

It does not use testnet.

It does not read ProgramData from network.

It does not perform live hash comparison.

It does not deploy, upgrade, write buffer, sign, submit, or mutate.

## Requirements/invariant review matrix

~~~text
# Read-only Network Precheck Package.2 requirements/invariant review matrix

RONPP2_INVARIANT_01_DRAFT_EXISTS:
status: reviewed
result: true
meaning: RONPP1 draft exists and is marked draft-only/not executable.

RONPP2_INVARIANT_02_OPERATION_CLASS_BOUND:
status: reviewed
result: true
meaning: Operation class is READ_ONLY_NETWORK_PRECHECK_ONLY.

RONPP2_INVARIANT_03_DRAFT_PACKAGE_ID_BOUND:
status: reviewed
result: true
meaning: Draft package id is RONPP1_DRAFT_f3bff313af20.

RONPP2_INVARIANT_04_SOURCE_COMMIT_BOUND:
status: reviewed
result: true
meaning: RONPP1 source commit is f3bff313af205285d4635fa709fc04f162edbb05.

RONPP2_INVARIANT_05_NETWORK_BOUND:
status: reviewed
result: true
meaning: Network is X1_TESTNET.

RONPP2_INVARIANT_06_RPC_ENDPOINT_PROPOSED_NOT_USED:
status: reviewed
result: true
meaning: Proposed RPC endpoint is https://rpc.testnet.x1.xyz but remains not used/not verified.

RONPP2_INVARIANT_07_PROGRAM_ID_BOUND:
status: reviewed
result: true
meaning: Program id is D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my.

RONPP2_INVARIANT_08_PROGRAMDATA_BOUND:
status: reviewed
result: true
meaning: ProgramData account is 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T.

RONPP2_INVARIANT_09_EXPECTED_AUTHORITY_BOUND:
status: reviewed
result: true
meaning: Expected upgrade authority is DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc.

RONPP2_INVARIANT_10_EXPECTED_HASH_BOUND:
status: reviewed
result: true
meaning: Expected canonical executable bytes SHA256 is e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1.

RONPP2_INVARIANT_11_HASH_DOMAIN_AND_ALGORITHM_BOUND:
status: reviewed
result: true
meaning: Canonical domain is PROGRAMDATA_EXECUTABLE_BYTES_EXCLUDING_LOADER_METADATA and algorithm is SHA256.

RONPP2_INVARIANT_12_COMMAND_SET_PROPOSED_NOT_EXECUTED:
status: reviewed
result: true
meaning: Proposed command set is recorded but not executable in RONPP2.

RONPP2_INVARIANT_13_COMMAND_SET_READ_ONLY_INTENT:
status: reviewed
result: true
meaning: Proposed command set is intended to read account metadata, dump executable bytes, and compute SHA256 locally.

RONPP2_INVARIANT_14_COMMAND_SET_REQUIRES_FUTURE_REVIEW:
status: reviewed
result: true
meaning: Exact future command set still requires closure review before any RPC use.

RONPP2_INVARIANT_15_EXACT_GO_DRAFT_NOT_FINAL:
status: reviewed
result: true
meaning: Proposed GO phrase is draft-only and not executable.

RONPP2_INVARIANT_16_READ_ONLY_GO_NOT_GRANTED:
status: reviewed
result: true
meaning: Read-only precheck GO remains not granted.

RONPP2_INVARIANT_17_NO_SIGNER_KEYPAIR_SECRET:
status: reviewed
result: true
meaning: Signer flags, keypair paths, and secret material remain forbidden.

RONPP2_INVARIANT_18_NO_TRANSACTION_SUBMIT:
status: reviewed
result: true
meaning: Transaction submit remains forbidden.

RONPP2_INVARIANT_19_NO_MUTATION:
status: reviewed
result: true
meaning: Deploy, upgrade, write-buffer, authority change, state initialization, SPL setup, guardian package construction, and mutation remain forbidden.

RONPP2_INVARIANT_20_STOP_ON_MISMATCH:
status: reviewed
result: true
meaning: Missing/different GO, dirty tree, endpoint mismatch, network mismatch, identity mismatch, authority mismatch, hash mismatch, and command-boundary mismatch remain stop conditions.

RONPP2_INVARIANT_21_NO_AUTOMATIC_RETRY:
status: reviewed
result: true
meaning: Automatic retry remains rejected.

RONPP2_AGGREGATE:
all_requirements_reviewed: true
all_invariants_reviewed: true
draft_package_reviewed: true
closure_ready: true
closure_candidate: EXACT_SCOPED_READ_ONLY_PRECHECK_PACKAGE_REVIEWED_GO_NOT_GRANTED_EXECUTION_NOT_APPROVED
rpc_approved_now: false
testnet_approved_now: false
programdata_read_approved_now: false
live_hash_comparison_approved_now: false
execution_approved_now: false
mutation_approved_now: false
~~~

## Command boundary review

~~~text
# Read-only Network Precheck Package.2 command boundary review

reviewed_proposed_commands:
- solana account 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T --url https://rpc.testnet.x1.xyz --output json
- solana account D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my --url https://rpc.testnet.x1.xyz --output json
- solana program dump D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my docs/gateway/evidence/FUTURE_EXACT_PRECHECK_EXECUTION/observed-xxxl-svm.so --url https://rpc.testnet.x1.xyz
- sha256sum docs/gateway/evidence/FUTURE_EXACT_PRECHECK_EXECUTION/observed-xxxl-svm.so

review_result:
- commands are proposed only
- commands were not executed
- RPC was not called
- testnet was not used
- ProgramData was not read
- executable bytes were not dumped
- live hash comparison was not performed

future_command_review_requirements:
- exact evidence path must be finalized
- exact command set must be reviewed again before closure
- command set must contain no signer flags
- command set must contain no keypair paths
- command set must contain no transaction submit commands
- command set must contain no deploy/upgrade/write-buffer commands
- command set must contain no mutation commands
- output paths must be inside future evidence directory
- no secrets may be requested or printed
~~~

## GO phrase review

~~~text
# Read-only Network Precheck Package.2 GO phrase review

reviewed_draft_phrase:
GO_RONPP_READ_ONLY_PRECHECK_ONLY_RONPP1_DRAFT_f3bff313af20_SOURCE_f3bff313af20

phrase_status:
DRAFT_NOT_FINAL_NOT_EXECUTABLE

review_result:
- phrase is recorded as draft only
- phrase has not been used by user
- phrase does not grant GO now
- phrase must be re-selected or finalized in a later closure record
- future exact user GO must be verbatim
- future exact user GO must authorize only READ_ONLY_NETWORK_PRECHECK_ONLY
- future exact user GO must not authorize deploy/upgrade/write-buffer/sign/submit/mutation

read_only_precheck_go_granted_now: false
~~~

## Closure candidate

~~~text
# Read-only Network Precheck Package.2 closure candidate

closure_candidate:
EXACT_SCOPED_READ_ONLY_PRECHECK_PACKAGE_REVIEWED_GO_NOT_GRANTED_EXECUTION_NOT_APPROVED

closure_candidate_meaning:
- RONPP1 draft package has been reviewed
- package remains not executable
- proposed RPC endpoint remains not used/not verified
- proposed command set remains not executed
- proposed GO phrase remains draft/not final
- read-only precheck GO remains not granted
- RPC/testnet/ProgramData read/live hash comparison remain not approved
- signing/submit/mutation remain not approved

next_safe_step:
Read-only Network Precheck Package.3 — exact scoped read-only precheck package closure record

next_step_must_not:
- call RPC
- use testnet
- read ProgramData
- dump executable bytes
- compute live observed hash
- perform live hash comparison
- sign
- submit
- mutate
~~~

## Remaining gaps

~~~text
# Read-only Network Precheck Package.2 remaining gaps

- RONPP3 closure record not recorded
- final package id not selected
- final exact GO phrase not selected
- proposed RPC endpoint not verified
- exact command set not finalized
- exact evidence path not finalized
- execution package not closed
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
# Read-only Network Precheck Package.2 non-GO boundary

Read-only Network Precheck Package.2 does not grant GO.

Read-only Network Precheck Package.2 does not approve:
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

Read-only Network Precheck Package.2 is requirements/invariant review only.
~~~

## Result

requirements_invariant_review_only: true

all_requirements_reviewed: true

all_invariants_reviewed: true

draft_package_reviewed: true

closure_ready: true

closure_candidate: EXACT_SCOPED_READ_ONLY_PRECHECK_PACKAGE_REVIEWED_GO_NOT_GRANTED_EXECUTION_NOT_APPROVED

proposed_rpc_endpoint: https://rpc.testnet.x1.xyz

program_id: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my

programdata_account: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T

expected_canonical_programdata_executable_bytes_sha256: e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1

read_only_precheck_go_granted: false

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

Read-only Network Precheck Package.3 — exact scoped read-only precheck package closure record.

Read-only Network Precheck Package.3 must still not call RPC, use testnet, read ProgramData, perform live hash comparison, sign, submit, or mutate.

## Evidence preview

metadata:

~~~text
phase=read-only-network-precheck-package-2-exact-scoped-read-only-precheck-package-requirements-invariant-review
timestamp_utc=2026-07-07T05:27:18Z
repo_only=true
requirements_invariant_review_only=true
current_commit=eed37774c40c1ce1f028435d9d3e08015108d4b1
current_short=eed37774c40c
reviewed_draft_package_id=RONPP1_DRAFT_f3bff313af20
reviewed_draft_source_commit=f3bff313af205285d4635fa709fc04f162edbb05
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
