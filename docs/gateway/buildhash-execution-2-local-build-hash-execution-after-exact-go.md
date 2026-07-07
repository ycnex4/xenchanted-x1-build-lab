# BuildHash Execution.2 — Local build/hash execution after exact GO

Status:

BUILDHASH_EXECUTION_2_COMPLETED_LOCAL_BUILD_HASH_AFTER_EXACT_GO_NO_RPC_NO_TESTNET_NO_MUTATION

Current decision:

LOCAL_BUILD_HASH_EXECUTED_AFTER_EXACT_GO_EVIDENCE_RECORDED_NO_NETWORK_ACTION

Final package id:

BHX2_LOCAL_BUILD_HASH_ee0cb44f7d49

Bound program source commit:

ee0cb44f7d496e010e784608d0c7ccc8e84e7fb6

Current repo commit at execution:

0863ab1b14aaea1f2ca6b8803cadb7665abecb73

Build command:

cargo build-sbf --manifest-path programs/xxxl-svm/Cargo.toml --no-default-features

Artifact path:

programs/xxxl-svm/target/deploy/xxxl_svm.so

Artifact size bytes:

20840

Local artifact SHA256:

e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1

Canonical runtime hash domain:

PROGRAMDATA_EXECUTABLE_BYTES_EXCLUDING_LOADER_METADATA

Canonical ProgramData executable-bytes SHA256:

e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1

## Result

build_executed: true

artifact_hash_computed: true

programdata_hash_computed: true

rpc_used: false

testnet_used: false

deploy_executed: false

upgrade_executed: false

write_buffer_executed: false

signing_executed: false

submit_executed: false

mutation_executed: false

## Evidence files

- docs/gateway/evidence/buildhash-execution-2-local-build-hash-execution-after-exact-go/exact-go-verification.txt
- docs/gateway/evidence/buildhash-execution-2-local-build-hash-execution-after-exact-go/source-binding-check.txt
- docs/gateway/evidence/buildhash-execution-2-local-build-hash-execution-after-exact-go/toolchain-versions.txt
- docs/gateway/evidence/buildhash-execution-2-local-build-hash-execution-after-exact-go/build-command.txt
- docs/gateway/evidence/buildhash-execution-2-local-build-hash-execution-after-exact-go/build-output.txt
- docs/gateway/evidence/buildhash-execution-2-local-build-hash-execution-after-exact-go/local-artifact-sha256.txt
- docs/gateway/evidence/buildhash-execution-2-local-build-hash-execution-after-exact-go/canonical-programdata-executable-bytes-sha256.txt
- docs/gateway/evidence/buildhash-execution-2-local-build-hash-execution-after-exact-go/metadata-final.txt
- docs/gateway/evidence/buildhash-execution-2-local-build-hash-execution-after-exact-go/non-go-boundary.txt

## Next safe step

Read-only Network Baseline.1 — precheck package planning only.

BuildHash Execution.2 does not authorize RPC/testnet or any network mutation.
