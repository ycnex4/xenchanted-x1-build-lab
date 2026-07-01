# XXXL X1 Testnet Local Runtime Skeleton Phase 38 Ed25519 Instruction Data Parser

Status: Narrow read-only parser for supplied Solana Ed25519 program instruction
data bytes.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-38-ed25519-instruction-data-parser`

Base context:

- Phase 32 added the read-only Rust/SVM verifier scaffold.
- Phase 33 added the raw payload decoder.
- Phase 34 added canonical payload hash/domain validation.
- Phase 35 added guardian membership and quorum structural verification.
- Phase 37 added the Ed25519 instruction evidence layout model.

## Purpose

Phase 38 parses actual Ed25519 program instruction data bytes supplied to the
parser.

It extracts signature, guardian public key, and message bytes.

It compares the extracted guardian public key with the expected guardian public
key.

It compares the extracted message bytes with the expected Phase 34 canonical
payload hash.

It does not verify Ed25519 signatures.

It does not read the Instructions sysvar.

It does not count quorum.

It does not authorize minting.

The preserved security decision remains:

~~~text
TS layer = preflight / model / watcher-side decision
Runtime = independent verifier
No authorized=true -> execute
~~~

## Files Added Or Changed

Added:

- `programs/xxxl-svm/src/verifier/ed25519_instruction_data_parser.rs`
- `docs/xxxl/xxxl-phase-38-ed25519-instruction-data-parser.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-38-ed25519-instruction-data-parser.md`

Changed:

- `programs/xxxl-svm/src/verifier/mod.rs`
- `docs/checkpoints/current-design-checkpoint.md`

No TypeScript source file is changed.

No TypeScript test file is changed.

No Cargo file is changed.

No package manifest or lockfile is changed.

No dependency is added.

No `programs/xxxl-svm/src/lib.rs` change is required.

## Parser Boundary

Parser marker:

- `ED25519_INSTRUCTION_DATA_PARSER_PHASE_38`

Parser version:

- `1`

The parser reuses Phase 37 constants:

- `ED25519_INSTRUCTION_HEADER_LEN`
- `ED25519_SIGNATURE_OFFSETS_RECORD_LEN`
- `ED25519_SIGNATURE_LEN`
- `ED25519_PUBLIC_KEY_LEN`
- `EXPECTED_MESSAGE_LEN`
- `EXPECTED_SIGNATURE_COUNT`
- `CURRENT_INSTRUCTION_INDEX_SENTINEL`

It also reuses `validate_ed25519_evidence_layout` to enforce the Phase 37
layout shape after reading the real instruction data bytes.

## Behavior Implemented

The parser rejects:

- instruction data shorter than header plus one offsets record
- signature count other than one
- nonzero padding
- signature instruction index not equal to `u16::MAX`
- public key instruction index not equal to `u16::MAX`
- message instruction index not equal to `u16::MAX`
- message size other than 32 bytes
- signature region overlapping the header or out of bounds
- public key region overlapping the header or out of bounds
- message region overlapping the header or out of bounds
- guardian public key mismatch
- message hash mismatch

The parser accepts well-shaped data only when:

- extracted public key bytes equal the expected guardian public key
- extracted message bytes equal the expected Phase 34 canonical payload hash

The success result includes:

- signature bytes
- guardian public key
- message bytes
- public key match flag set to true
- message hash match flag set to true
- Ed25519 signature verification flag set to false
- cryptographic proof accepted flag set to false
- Instructions sysvar read flag set to false
- quorum counted flag set to false
- authorization granted flag set to false

## Explicit Honesty

Phase 38 reads only actual instruction data bytes supplied to the parser.

Phase 38 does not read the Instructions sysvar.

Phase 38 does not scan transaction instructions.

Phase 38 does not call `load_instruction`.

Phase 38 does not verify the Ed25519 signature.

Phase 38 does not prove that the signature is valid.

Phase 38 does not count quorum.

Phase 38 does not authorize minting.

Signature bytes are extracted as future verification evidence only.

The only new capability is byte parsing plus expected guardian public key value
comparison plus expected Phase 34 message hash value comparison.

## Remaining Obligations

Remaining obligations include:

- read-only Instructions sysvar evidence scanner
- Ed25519 cryptographic verification
- proof that guardian signatures cover the Phase 34 canonical payload hash
- quorum counting over verified signature evidence
- source proof verification
- route config verification
- target mint account legitimacy verification
- amount cap enforcement
- replay storage
- replay checks
- replay writes
- account parsing
- instruction processing
- mint execution
- runtime unlock

## Explicit Non-Goals

Phase 38 does not implement Ed25519 cryptographic verification.

Phase 38 does not accept cryptographic signature proof.

Phase 38 does not read the Instructions sysvar.

Phase 38 does not scan transaction instructions.

Phase 38 does not use `load_instruction`.

Phase 38 does not count quorum.

Phase 38 does not authorize minting.

Phase 38 does not add `process_instruction`.

Phase 38 does not add account parsing.

Phase 38 does not add CPI.

Phase 38 does not enable `invoke_signed`.

Phase 38 does not enable SPL Token `mint_to`.

Phase 38 does not add replay writes.

Phase 38 does not mark processed events.

Phase 38 does not mutate runtime/account state.

Phase 38 does not unlock live route execution.

Phase 38 does not remove deployment blockers.

Phase 38 does not select a production Program ID.

Phase 38 does not claim production readiness.

Phase 38 does not claim final immutability while upgrade authority exists.

Phase 38 does not build SBF artifacts.

Phase 38 does not touch `target/deploy`.

Phase 38 does not read or modify keypair files.

Phase 38 does not read or modify `.env`.

Phase 38 does not inspect `.local-keys`.

Phase 38 does not run deploy commands.

Phase 38 does not run network commands.

Phase 38 does not spend SOL.

## Active Blockers Preserved

Current X1 status remains:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`

Active blockers remain:

- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker was removed.

No production readiness is claimed.

No final immutability is claimed while upgrade authority exists.

## Validation

Commands run:

- `git diff --check`: passed
- `wsl bash -lc "cd /mnt/c/Users/user/xenchanted-x1-build-lab/programs/xxxl-svm && cargo fmt --check"`: passed
- `wsl bash -lc "cd /mnt/c/Users/user/xenchanted-x1-build-lab/programs/xxxl-svm && cargo test ed25519_instruction_data_parser --lib"`: passed, 24 tests passed
- `wsl bash -lc "cd /mnt/c/Users/user/xenchanted-x1-build-lab/programs/xxxl-svm && cargo test verifier --lib"`: passed, 84 tests passed
- `wsl bash -lc "cd /mnt/c/Users/user/xenchanted-x1-build-lab/programs/xxxl-svm && cargo test --lib --locked"`: passed, 295 tests passed, 1 ignored
- `npm run typecheck`: passed
- `npm run build`: passed
- `git status --short --untracked-files=all`: run for final workspace state

No Cargo manifest was changed.

No Cargo lockfile was changed.

No SBF build was run.

## Recommended Next Stage

Phase 39 should add a read-only Instructions sysvar evidence scanner, still
without Ed25519 cryptographic verification, quorum authorization, handler or
account parsing, CPI, mint execution, replay writes, or runtime unlock.
