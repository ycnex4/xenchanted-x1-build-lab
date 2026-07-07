# Upgrade Execution Package Closure.1 — Existing program

Status:

UPGRADE_EXECUTION_PACKAGE_CLOSURE_1_EXISTING_PROGRAM_EXECUTION_BLOCKED_NO_RPC_NO_MUTATION

Execution:

BLOCKED

Path:

UPGRADE_EXISTING_PROGRAM_ONLY

## Summary

Theo approved Upgrade Execution Package closure only.

Execution remains blocked until a separate exact GO phrase.

This closure defines signer/keypair boundary, buffer boundary, future exact command sequence reference, stop conditions, evidence paths, rollback/recovery boundary, and future exact GO phrase binding.

## Target

source_commit: de6c96a90b5dc54d4b88cbda00d7d7175874e1c2

program_id: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my

programdata_account: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T

upgrade_authority: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc

expected_hash: e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1

expected_size: 20840

stale_live_hash_before_upgrade: fd4d3b659ccaea4f5e24eca4d9e80ff808c43de1bf1ecef7315961751a085a7e

stale_live_size_before_upgrade: 38584

## Signer / keypair boundary

~~~text
signer_boundary_status=CONFIRMED_BY_USER

expected_upgrade_authority=DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc

keypair_source:
local file keypair

repo_storage:
forbidden

chat_disclosure:
forbidden

env_var_required:
X1_UPGRADE_AUTHORITY_KEYPAIR

future_execution_required_check:
solana-keygen pubkey "$X1_UPGRADE_AUTHORITY_KEYPAIR"

required_result:
DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc

stop_if:
- X1_UPGRADE_AUTHORITY_KEYPAIR is empty
- file does not exist
- solana-keygen pubkey fails
- derived pubkey differs from DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc
~~~

## Buffer boundary

~~~text
buffer_pattern=WRITE_BUFFER_THEN_UPGRADE

buffer_keypair_env_var:
X1_UPGRADE_BUFFER_KEYPAIR

buffer_keypair_source:
local temporary file keypair

buffer_keypair_repo_storage:
forbidden

buffer_keypair_chat_disclosure:
forbidden

buffer_creation:
future execution package creates or verifies local buffer keypair only after exact GO

buffer_funding:
upgrade authority / fee payer funds buffer creation and upload transaction fees

minimum_authority_balance_policy:
authority balance must be >= 0.35 SOL before write-buffer

rent_policy:
future execution must record rent estimate for 20840 bytes before write-buffer.
If 2x rent-exempt amount plus fee cushion exceeds 0.35 SOL, STOP.

buffer_cleanup_plan:
If write-buffer succeeds but upgrade is aborted before final upgrade, close or recover buffer only in a separate cleanup package with separate exact GO.
No cleanup mutation is authorized by this closure package.
~~~

## Exact command sequence reference for future execution

~~~bash
# Exact command sequence reference for future execution only

# This file defines the intended future command sequence.
# It is NOT authorized by this closure checkpoint.
# It may only be executed after the exact GO phrase is provided separately.

set -euo pipefail

PHASE="upgrade-execution-1-existing-program-after-exact-go"
RPC_ENDPOINT="https://rpc.testnet.x1.xyz"
PROGRAM_ID="D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my"
PROGRAMDATA_ACCOUNT="9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T"
EXPECTED_UPGRADE_AUTHORITY="DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc"
EXPECTED_HASH="e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1"
EXPECTED_SIZE="20840"
MIN_AUTHORITY_BALANCE_SOL="0.35"
LOCAL_ARTIFACT="programs/xxxl-svm/target/deploy/xxxl_svm.so"
EVIDENCE_DIR="docs/gateway/evidence/${PHASE}"

test -n "${X1_UPGRADE_AUTHORITY_KEYPAIR:-}"
test -f "$X1_UPGRADE_AUTHORITY_KEYPAIR"
test -n "${X1_UPGRADE_BUFFER_KEYPAIR:-}"

AUTHORITY_PUBKEY="$(solana-keygen pubkey "$X1_UPGRADE_AUTHORITY_KEYPAIR")"
test "$AUTHORITY_PUBKEY" = "$EXPECTED_UPGRADE_AUTHORITY"

cargo build-sbf --manifest-path programs/xxxl-svm/Cargo.toml --no-default-features

FRESH_HASH="$(sha256sum "$LOCAL_ARTIFACT" | cut -d' ' -f1)"
FRESH_SIZE="$(stat -c%s "$LOCAL_ARTIFACT")"
test "$FRESH_HASH" = "$EXPECTED_HASH"
test "$FRESH_SIZE" = "$EXPECTED_SIZE"

solana account "$PROGRAM_ID" --url "$RPC_ENDPOINT" --output json > "$EVIDENCE_DIR/pre-upgrade-program-account.json"
solana account "$PROGRAMDATA_ACCOUNT" --url "$RPC_ENDPOINT" --output json > "$EVIDENCE_DIR/pre-upgrade-programdata-account.json"
solana program dump "$PROGRAM_ID" "$EVIDENCE_DIR/pre-upgrade-observed.so" --url "$RPC_ENDPOINT"
sha256sum "$EVIDENCE_DIR/pre-upgrade-observed.so" > "$EVIDENCE_DIR/pre-upgrade-observed.sha256"

solana balance "$AUTHORITY_PUBKEY" --url "$RPC_ENDPOINT" > "$EVIDENCE_DIR/authority-balance-before.txt"
solana rent "$EXPECTED_SIZE" --url "$RPC_ENDPOINT" > "$EVIDENCE_DIR/rent-estimate-${EXPECTED_SIZE}.txt"

solana-keygen new --no-bip39-passphrase --force --outfile "$X1_UPGRADE_BUFFER_KEYPAIR"
BUFFER_PUBKEY="$(solana-keygen pubkey "$X1_UPGRADE_BUFFER_KEYPAIR")"
echo "$BUFFER_PUBKEY" > "$EVIDENCE_DIR/buffer-pubkey.txt"

solana program write-buffer "$LOCAL_ARTIFACT" \
  --buffer "$X1_UPGRADE_BUFFER_KEYPAIR" \
  --upgrade-authority "$X1_UPGRADE_AUTHORITY_KEYPAIR" \
  --keypair "$X1_UPGRADE_AUTHORITY_KEYPAIR" \
  --url "$RPC_ENDPOINT" \
  > "$EVIDENCE_DIR/write-buffer.stdout" \
  2> "$EVIDENCE_DIR/write-buffer.stderr"

solana program dump "$BUFFER_PUBKEY" "$EVIDENCE_DIR/buffer-observed.so" --url "$RPC_ENDPOINT"
sha256sum "$EVIDENCE_DIR/buffer-observed.so" > "$EVIDENCE_DIR/buffer-observed.sha256"

BUFFER_HASH="$(cut -d' ' -f1 "$EVIDENCE_DIR/buffer-observed.sha256")"
BUFFER_SIZE="$(stat -c%s "$EVIDENCE_DIR/buffer-observed.so")"
test "$BUFFER_HASH" = "$EXPECTED_HASH"
test "$BUFFER_SIZE" = "$EXPECTED_SIZE"

solana program upgrade "$BUFFER_PUBKEY" "$PROGRAM_ID" \
  --upgrade-authority "$X1_UPGRADE_AUTHORITY_KEYPAIR" \
  --keypair "$X1_UPGRADE_AUTHORITY_KEYPAIR" \
  --url "$RPC_ENDPOINT" \
  > "$EVIDENCE_DIR/upgrade.stdout" \
  2> "$EVIDENCE_DIR/upgrade.stderr"

solana account "$PROGRAM_ID" --url "$RPC_ENDPOINT" --output json > "$EVIDENCE_DIR/post-upgrade-program-account.json"
solana account "$PROGRAMDATA_ACCOUNT" --url "$RPC_ENDPOINT" --output json > "$EVIDENCE_DIR/post-upgrade-programdata-account.json"
solana program dump "$PROGRAM_ID" "$EVIDENCE_DIR/post-upgrade-observed.so" --url "$RPC_ENDPOINT"
sha256sum "$EVIDENCE_DIR/post-upgrade-observed.so" > "$EVIDENCE_DIR/post-upgrade-observed.sha256"

POST_HASH="$(cut -d' ' -f1 "$EVIDENCE_DIR/post-upgrade-observed.sha256")"
POST_SIZE="$(stat -c%s "$EVIDENCE_DIR/post-upgrade-observed.so")"
test "$POST_HASH" = "$EXPECTED_HASH"
test "$POST_SIZE" = "$EXPECTED_SIZE"
~~~

## Stop conditions

~~~text
Stop immediately during future execution if any occurs:

- X1_UPGRADE_AUTHORITY_KEYPAIR missing
- authority keypair path does not exist
- authority pubkey != DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc
- X1_UPGRADE_BUFFER_KEYPAIR missing
- source commit != de6c96a90b5dc54d4b88cbda00d7d7175874e1c2
- fresh rebuild hash != e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1
- fresh rebuild size != 20840
- Program ID mismatch
- ProgramData account mismatch
- upgrade authority mismatch
- authority balance < 0.35 SOL
- rent/fee requirement exceeds 0.35 SOL policy
- write-buffer fails
- buffer hash != e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1
- buffer size != 20840
- any RPC error
- timeout on confirmation
- any command asks for unexpected authority
- any target differs from D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my / 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
- post-upgrade hash != e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1
- post-upgrade size != 20840
~~~

## Evidence paths

~~~text
future_execution_evidence_dir:
docs/gateway/evidence/upgrade-execution-1-existing-program-after-exact-go

required_future_evidence:
- exact-go-record.txt
- signer-keypair-verification.txt
- buffer-keypair-record.txt
- build-output.txt
- fresh-local-artifact.sha256
- fresh-local-artifact.size
- pre-upgrade-program-account.json
- pre-upgrade-programdata-account.json
- pre-upgrade-observed.so
- pre-upgrade-observed.sha256
- authority-balance-before.txt
- rent-estimate-20840.txt
- write-buffer.stdout
- write-buffer.stderr
- buffer-pubkey.txt
- buffer-observed.so
- buffer-observed.sha256
- upgrade.stdout
- upgrade.stderr
- upgrade-signature.txt
- post-upgrade-program-account.json
- post-upgrade-programdata-account.json
- post-upgrade-observed.so
- post-upgrade-observed.sha256
- final-verification-summary.txt
~~~

## Rollback / recovery boundary

~~~text
rollback_status:
not_automatic

stale_binary_recovery:
The stale live binary was dumped earlier as:
docs/gateway/evidence/read-only-network-precheck-execution-1-after-exact-go/observed-xxxl-svm.so

rollback_policy:
No rollback is authorized by this closure package.
If upgrade fails before final upgrade transaction, stop and preserve buffer evidence.
If upgrade succeeds but post-upgrade verification fails, stop and create separate rollback/recovery package.

authority_recovery:
Upgrade authority must remain DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc unless separately changed by an approved package.

buffer_cleanup:
If buffer cleanup is needed, it requires separate cleanup package and separate exact GO.
~~~

## Future exact GO phrase

~~~text
future_exact_go_phrase=GO_TESTNET_UPGRADE_EXISTING_PROGRAM_ONLY_TDEX1_SOURCE_DE6C96A_EXPECTED_E68ADA36_PROGRAM_D7AQMZNT

status:
DEFINED_BUT_NOT_GRANTED

binding:
source_commit=de6c96a90b5dc54d4b88cbda00d7d7175874e1c2
program_id=D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
programdata_account=9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
expected_hash=e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1
expected_size=20840
path=UPGRADE_EXISTING_PROGRAM_ONLY
scope=upgrade_existing_program_only

not_authorized:
This phrase is not granted by this closure checkpoint.
~~~

## Non-GO boundary

~~~text
This is closure only.

execution_authorized=false
rpc_used=false
testnet_used=false
programdata_read_executed=false
executable_bytes_dumped=false
live_hash_comparison_executed=false
write_buffer_executed=false
upgrade_executed=false
signing_executed=false
submit_executed=false
mutation_executed=false
~~~

## Progress

~~~text
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
✅ 9: Precheck Result Decision
✅ 10: Local Rebuild Investigation.1
✅ 11: Investigation Result Decision
✅ 12: Testnet Upgrade Package Planning.1
✅ 13: Upgrade Execution Package Closure.1

👉 14: Upgrade execution only after separate exact GO
⏭ 15: Post-upgrade verification
⏭ 16: Separate activation path

blocked:
RPC/write-buffer/upgrade/sign/submit/mutation
~~~

## Result

closure_only: true

execution_authorized: false

future_go_phrase_defined_but_not_granted: GO_TESTNET_UPGRADE_EXISTING_PROGRAM_ONLY_TDEX1_SOURCE_DE6C96A_EXPECTED_E68ADA36_PROGRAM_D7AQMZNT

rpc_used: false

testnet_used: false

write_buffer_executed: false

upgrade_executed: false

signing_executed: false

submit_executed: false

mutation_executed: false
