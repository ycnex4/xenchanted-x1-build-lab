# XXXL Phase 37 Ed25519 Instruction Evidence Layout Model

Status: Read-only structural Rust/SVM layout model.

## Purpose

Phase 37 turns the Ed25519 signature evidence model selected in Phase 36 into a
typed, tested Rust layout model.

It models the shape of the Solana Ed25519 program instruction data for the
single-guardian-per-instruction evidence model.

This phase adds a Rust source module and Rust unit tests.

This phase does not implement Ed25519 verification.

This phase does not parse the Instructions sysvar.

This phase does not inspect real ed25519 program instructions.

This phase does not read accounts.

This phase does not unlock runtime execution.

## Base Context

Previous Rust/SVM verifier phases:

- Phase 32: read-only runtime verifier scaffold.
- Phase 33: raw payload decoder.
- Phase 34: canonical payload hash/domain validation.
- Phase 35: guardian membership and quorum structural verifier.
- Phase 36: Ed25519 signature evidence boundary specification (docs-only).

Phase 36 selected the SVM ed25519 instruction evidence model and required a
future signature to bind the expected guardian public key to the exact Phase 34
canonical payload hash.

Phase 37 models the layout shape of that evidence as typed constants, descriptor
structs, and a pure shape-validation function.

## Preserved Security Decision

The preserved security decision remains:

~~~text
TS layer = preflight / model / watcher-side decision
Runtime = independent verifier
No authorized=true -> execute
~~~

TypeScript authorization output is not runtime authority.

TypeScript parity results are not runtime authority.

Guardian structural quorum is not runtime authority.

Caller-provided signature claims are not runtime authority.

A caller-provided layout descriptor is not runtime authority.

## What The Layout Model Is

The layout model is a typed and tested description of the Solana Ed25519 program
instruction data shape.

It provides:

- named layout constants (header length, offsets record length, signature
  length, public key length, expected message length, single-signature count,
  current-instruction-index sentinel);
- an `Ed25519SignatureOffsetsModel` struct holding the seven `u16` little-endian
  offset fields;
- an `Ed25519EvidenceLayoutDescriptor` struct that a caller supplies to be
  shape-checked (declared instruction data length, declared number of
  signatures, and the offsets record);
- a `validate_ed25519_evidence_layout` function that performs only structural
  bounds and size checks against the expected constants;
- an `Ed25519EvidenceLayoutModelResult` whose fields make explicit that no
  cryptographic verification, sysvar read, or signature-proof acceptance
  happened;
- an `Ed25519EvidenceLayoutModelReport` describing which layout-shape checks are
  implemented and which verification and execution surfaces remain disabled.

## What The Layout Model Is Not

The layout model does not verify any Ed25519 signature.

The layout model does not parse the Instructions sysvar.

The layout model does not read any real instruction.

The layout model does not read any account.

The layout model does not recompute or accept a signature proof.

The layout model does not count guardian approvals.

The layout model does not unlock runtime execution.

A descriptor that passes shape validation is not authorized evidence. It is only
a well-shaped description of where signature, public key, and message bytes would
be found in an ed25519 program instruction.

## Modeled Layout

The modeled Solana Ed25519 program instruction data layout is:

~~~text
offset 0: num_signatures: u8
offset 1: padding: u8
offset 2: Ed25519SignatureOffsets record (7 x u16 little-endian, 14 bytes)
~~~

The `Ed25519SignatureOffsets` record fields are:

- `signature_offset: u16`
- `signature_instruction_index: u16`
- `public_key_offset: u16`
- `public_key_instruction_index: u16`
- `message_data_offset: u16`
- `message_data_size: u16`
- `message_instruction_index: u16`

Fixed sizes:

- header length = 2 bytes
- offsets record length = 14 bytes
- signature length = 64 bytes
- public key length = 32 bytes
- expected message length = 32 bytes

The expected message is the 32-byte Phase 34 canonical payload hash, so
`EXPECTED_MESSAGE_LEN = 32`.

The `u16::MAX` instruction-index sentinel means "this same ed25519 instruction's
data".

The single-guardian-per-instruction model expects exactly one signature record
per ed25519 program instruction (`EXPECTED_SIGNATURE_COUNT = 1`).

## Shape Validation

`validate_ed25519_evidence_layout` rejects a descriptor when:

- the declared signature count is not exactly one;
- the declared message size is not exactly 32 bytes;
- the declared instruction data length is too small to hold the header plus one
  offsets record (truncated instruction data);
- the signature region overlaps the header or runs out of bounds;
- the public key region overlaps the header or runs out of bounds;
- the message region overlaps the header or runs out of bounds.

On success it returns a result whose safety fields are explicitly false:

- `ed25519_signature_verification_performed: false`
- `instructions_sysvar_read: false`
- `cryptographic_signature_proof_accepted: false`

## Relationship To Phase 35 And Phase 36

Phase 35 structural quorum answers only:

~~~text
Are these guardian public keys known, unique, and enough to meet threshold?
~~~

Phase 36 defined the future evidence boundary for answering:

~~~text
Did those guardians cryptographically sign the exact Phase 34 canonical payload hash?
~~~

Phase 37 models the layout shape that a future evidence parser would read to
answer that Phase 36 question.

Structural quorum (Phase 35) remains a separate, still-required check. A
well-shaped ed25519 layout without structural guardian membership and quorum is
not enough. Structural guardian membership and quorum without cryptographically
verified ed25519 evidence is also not enough.

Phase 37 alone cannot make `authorized=true`.

## Future Runtime Acceptance Rule

The future runtime acceptance rule is unchanged and still not implemented.

A future runtime verifier may count a guardian approval only if all of the
following are true:

- raw payload decoding succeeds;
- canonical payload hash validation succeeds;
- guardian public key is known in the selected guardian set;
- guardian set id matches;
- approval is not duplicated;
- Ed25519 evidence proves the guardian signed the expected Phase 34 payload hash;
- the guardian evidence is not reused in a way that violates replay rules;
- quorum threshold is reached using unique cryptographically verified guardians.

Phase 37 does not implement this rule.

Phase 37 only models and tests the instruction evidence layout shape.

## Explicit Non-Goals

Phase 37 does not implement Ed25519 verification.

Phase 37 does not parse real ed25519 instruction data.

Phase 37 does not parse the Instructions sysvar.

Phase 37 does not add an instruction handler.

Phase 37 does not parse runtime accounts.

Phase 37 does not read route accounts.

Phase 37 does not read target mint accounts.

Phase 37 does not implement source proof verification.

Phase 37 does not implement route config verification.

Phase 37 does not implement target mint account legitimacy verification.

Phase 37 does not implement amount cap enforcement.

Phase 37 does not implement replay storage.

Phase 37 does not implement replay checks.

Phase 37 does not implement replay writes.

Phase 37 does not enable live route execution.

Phase 37 does not enable SPL CPI.

Phase 37 does not enable `invoke_signed`.

Phase 37 does not enable SPL Token `mint_to`.

Phase 37 does not add mint execution.

Phase 37 does not mutate runtime/account state.

Phase 37 does not mark processed events.

Phase 37 does not select a production Program ID.

Phase 37 does not regenerate production PDA fixtures.

Phase 37 does not remove deployment blockers.

Phase 37 does not claim production readiness.

Phase 37 does not claim final immutability while upgrade authority exists.

Phase 37 does not change Cargo manifests.

Phase 37 does not change package manifests.

Phase 37 does not add dependencies.

Phase 37 does not build SBF artifacts.

Phase 37 does not touch `target/deploy`.

Phase 37 does not read or modify keypair files.

Phase 37 does not read or modify `.env`.

Phase 37 does not inspect `.local-keys`.

Phase 37 does not run deploy commands.

Phase 37 does not run network commands.

Phase 37 does not spend SOL.

## Preserved Blockers

The following blockers remain active:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker is removed, weakened, renamed, or satisfied by Phase 37.

## Recommended Next Stage

Recommended next stage:

- Phase 38: Rust/SVM Ed25519 instruction evidence parser over the real
  Instructions sysvar.

That next stage should still be read-only and should not unlock runtime
execution.

It should read real ed25519 program instruction evidence against the Phase 37
layout model, still without an instruction handler, account parsing, CPI, mint
execution, replay checks or writes, or runtime unlock.
