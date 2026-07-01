# XXXL Phase 38 Ed25519 Instruction Data Parser

Status: Narrow read-only parser for Solana Ed25519 program instruction data
bytes supplied directly to the parser.

## Purpose

Phase 38 parses actual Ed25519 program instruction data bytes.

It extracts:

- 64-byte signature bytes
- 32-byte guardian public key bytes
- 32-byte message bytes

It compares:

- extracted public key bytes to the expected guardian public key
- extracted message bytes to the expected Phase 34 canonical payload hash

This is only byte parsing plus value comparison.

Phase 38 does not verify the Ed25519 signature.

Phase 38 does not prove that the signature is valid.

Phase 38 does not count quorum.

Phase 38 does not authorize minting.

## Base Context

Previous Rust/SVM verifier phases:

- Phase 32: read-only runtime verifier scaffold.
- Phase 33: raw payload decoder.
- Phase 34: canonical payload hash/domain validation.
- Phase 35: guardian membership and quorum structural verifier.
- Phase 37: Ed25519 instruction evidence layout model.

Phase 38 reuses the Phase 37 layout constants and layout validator where clean:

- `ED25519_INSTRUCTION_HEADER_LEN`
- `ED25519_SIGNATURE_OFFSETS_RECORD_LEN`
- `ED25519_SIGNATURE_LEN`
- `ED25519_PUBLIC_KEY_LEN`
- `EXPECTED_MESSAGE_LEN`
- `EXPECTED_SIGNATURE_COUNT`
- `CURRENT_INSTRUCTION_INDEX_SENTINEL`
- `validate_ed25519_evidence_layout`

## Preserved Security Decision

The preserved security decision remains:

~~~text
TS layer = preflight / model / watcher-side decision
Runtime = independent verifier
No authorized=true -> execute
~~~

TypeScript authorization output is not runtime authority.

TypeScript parity results are not runtime authority.

Parsed signature bytes are not runtime authority.

Extracted guardian public key and message hash matches are not runtime
authority.

## Source Boundary

New Rust module:

- `programs/xxxl-svm/src/verifier/ed25519_instruction_data_parser.rs`

Updated Rust verifier export:

- `programs/xxxl-svm/src/verifier/mod.rs`

No `programs/xxxl-svm/src/lib.rs` change is required.

No Cargo manifest change is required.

No package manifest or lockfile change is required.

No dependency is added.

## Parser API

The Phase 38 marker is:

~~~text
ED25519_INSTRUCTION_DATA_PARSER_PHASE_38
~~~

The parser version is:

~~~text
1
~~~

The exposed Rust API includes:

- `ParsedEd25519InstructionEvidence`
- `Ed25519InstructionDataParserReport`
- `Ed25519InstructionDataParserError`
- `Ed25519InstructionDataParserErrorKind`
- `parse_ed25519_instruction_data_evidence`
- `ed25519_instruction_data_parser_report`

`parse_ed25519_instruction_data_evidence` receives:

- `instruction_data: &[u8]`
- expected `GuardianPublicKey`
- expected Phase 34 payload hash bytes

The parser reads only the supplied `instruction_data` slice.

It does not read the Instructions sysvar.

It does not scan transaction instructions.

It does not call `load_instruction`.

## Parsed Layout

The parser reads:

~~~text
offset 0: num_signatures: u8
offset 1: padding: u8
offset 2: Ed25519SignatureOffsets record (7 x u16 little-endian)
~~~

The offsets record fields are:

- `signature_offset`
- `signature_instruction_index`
- `public_key_offset`
- `public_key_instruction_index`
- `message_data_offset`
- `message_data_size`
- `message_instruction_index`

The parser requires:

- instruction data length at least header plus one offsets record
- `num_signatures == 1`
- `padding == 0`
- all three instruction-index fields equal `u16::MAX`
- `message_data_size == 32`
- signature region in bounds and not overlapping the header
- public key region in bounds and not overlapping the header
- message region in bounds and not overlapping the header
- public key bytes equal the expected guardian public key
- message bytes equal the expected Phase 34 canonical payload hash

## What The Parser Returns

On success the parser returns:

- `signature_bytes: [u8; 64]`
- `guardian_public_key: GuardianPublicKey`
- `message_bytes: [u8; 32]`
- `public_key_matches_expected_guardian: true`
- `message_matches_expected_phase_34_hash: true`
- `ed25519_signature_verification_performed: false`
- `cryptographic_signature_proof_accepted: false`
- `instructions_sysvar_read: false`
- `quorum_counted: false`
- `authorization_granted: false`

Signature bytes are extracted only as evidence bytes for a future verification
phase.

## What Phase 38 Is Not

Phase 38 does not implement Ed25519 cryptographic verification.

Phase 38 does not prove that a signature is valid.

Phase 38 does not parse the Instructions sysvar.

Phase 38 does not scan transaction instructions.

Phase 38 does not validate ed25519 program instructions from a transaction.

Phase 38 does not count guardian quorum.

Phase 38 does not authorize minting.

Phase 38 does not add an instruction handler.

Phase 38 does not add an account parser.

Phase 38 does not enable CPI.

Phase 38 does not enable `invoke_signed`.

Phase 38 does not enable SPL Token `mint_to`.

Phase 38 does not add replay writes.

Phase 38 does not mark processed events.

Phase 38 does not mutate runtime/account state.

Phase 38 does not unlock live route execution.

Phase 38 does not remove deployment blockers.

## Preserved Blockers

The following blockers remain active:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker is removed, weakened, renamed, or satisfied by Phase 38.

## Recommended Next Stage

Phase 39 should add a read-only Instructions sysvar evidence scanner.

That future phase should still avoid Ed25519 cryptographic verification, quorum
authorization, instruction handlers, account parsing, CPI, mint execution,
replay writes, and runtime unlock.
