# XXXL Phase 34 Rust/SVM Canonical Payload Hash Validation

Status: Narrow Rust/SVM canonical payload hash/domain validation.

## Purpose

Phase 34 adds a bounded Rust/SVM verifier helper for the Phase 23 canonical
guardian payload hash.

The helper exists so the future runtime verifier cannot drift from the
reviewed TypeScript payload hash model.

Phase 34 does not implement a full runtime verifier.

Phase 34 does not unlock runtime execution.

## Inputs

Phase 34 depends on these prior boundaries:

- Phase 23 TypeScript canonical guardian payload encoding and hash model.
- Phase 31 docs-only runtime verifier boundary specification.
- Phase 32 read-only Rust/SVM verifier scaffold.
- Phase 33 Rust/SVM raw payload decoder.

The TypeScript hash model is:

- domain label: `XXXL_GUARDIAN_PAYLOAD_HASH_V1`
- domain separator: `keccak256(utf8(domain label))`
- hash preimage: `domain_separator || encodeXxxlGuardianPayload(fields)`
- payload hash: `keccak256(hash preimage)`

The Phase 23 valid vector remains:

- domain separator:
  `0xf1958bbf04d45ddbc5a9f93f200f5005ee47b05cf61a90faf4d93cd6e3eccd66`
- valid payload hash:
  `0xab0ee59a1268f3eebf4a9d42725640ce68226e642a61dabd5f904e7680f08015`

## Source Boundary

New Rust module:

- `programs/xxxl-svm/src/verifier/canonical_payload.rs`

Updated Rust verifier export:

- `programs/xxxl-svm/src/verifier/mod.rs`

No `programs/xxxl-svm/src/lib.rs` change is required.

No Cargo manifest change is required.

The implementation uses `solana_program::keccak`, already available through
the existing `solana-program` dependency.

## Validator API

The Phase 34 marker is:

~~~text
CANONICAL_PAYLOAD_HASH_VALIDATOR_PHASE_34
~~~

The validator version is:

~~~text
1
~~~

The exposed helper functions are:

- `compute_guardian_payload_hash_domain_separator`
- `compute_guardian_payload_hash`
- `validate_guardian_payload_hash`
- `canonical_payload_hash_validation_report`

`compute_guardian_payload_hash` calls the Phase 33 raw payload decoder first.

Only after the raw payload decodes does it compute:

~~~text
keccak256(keccak256("XXXL_GUARDIAN_PAYLOAD_HASH_V1") || payload_bytes)
~~~

`validate_guardian_payload_hash` recomputes the payload hash from the supplied
payload bytes before comparing it with the expected hash.

The validator does not trust a caller-provided payload hash without
recomputation.

## Preserved Security Decision

The preserved security decision remains:

~~~text
TS layer = preflight / model / watcher-side decision
Runtime = independent verifier
No authorized=true -> execute
~~~

TypeScript authorization output is not runtime authority.

TypeScript parity results are not runtime authority.

Caller-provided payload hashes are not accepted as proof unless the runtime
recomputes the same hash from the decoded payload bytes.

## Rejections Implemented

Phase 34 distinguishes:

- Phase 33 raw payload decode failure.
- canonical payload hash mismatch.

Malformed raw payload bytes are rejected before hash acceptance.

An expected hash mismatch is rejected after recomputation.

## Honest Remaining Obligations

Phase 34 is hash/domain validation only.

The following future obligations remain:

- `wrong-canonical-event-key-preimage`
- `wrong-source-burn-tx-hash`
- `wrong-source-burn-event-index`
- `amount-over-route-cap`
- `invalid-target-mint`
- same-shape `wrong-field-order` variants not structurally rejected by Phase 33

A payload can hash correctly while still lying about source proof fields.

Canonical payload hash validation is not source proof verification.

Canonical payload hash validation is not guardian signature verification.

Canonical payload hash validation is not guardian quorum verification.

Canonical payload hash validation is not route config verification.

Canonical payload hash validation is not target mint account legitimacy
verification.

Canonical payload hash validation is not amount cap enforcement.

Canonical payload hash validation is not replay protection.

## Explicit Non-Goals

Phase 34 does not implement Ed25519 verification.

Phase 34 does not implement guardian quorum.

Phase 34 does not implement source proof verification.

Phase 34 does not implement route config verification.

Phase 34 does not implement target mint account legitimacy verification.

Phase 34 does not implement amount cap enforcement.

Phase 34 does not implement replay storage.

Phase 34 does not implement replay checks.

Phase 34 does not implement replay writes.

Phase 34 does not parse runtime accounts.

Phase 34 does not add an instruction handler.

Phase 34 does not enable live route execution.

Phase 34 does not enable SPL CPI.

Phase 34 does not enable `invoke_signed`.

Phase 34 does not enable SPL Token `mint_to`.

Phase 34 does not add mint execution.

Phase 34 does not mutate runtime/account state.

Phase 34 does not mark processed events.

Phase 34 does not select a production Program ID.

Phase 34 does not regenerate production PDA fixtures.

Phase 34 does not remove deployment blockers.

Phase 34 does not claim production readiness.

Phase 34 does not claim final immutability while upgrade authority exists.

Phase 34 does not change Cargo manifests.

Phase 34 does not change package manifests.

Phase 34 does not build SBF artifacts.

Phase 34 does not touch `target/deploy`.

Phase 34 does not read or modify keypair files.

Phase 34 does not read or modify `.env`.

Phase 34 does not inspect `.local-keys`.

Phase 34 does not run deploy commands.

Phase 34 does not run network commands.

Phase 34 does not spend SOL.

## Preserved Blockers

The following blockers remain active:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker is removed, weakened, renamed, or satisfied by Phase 34.
