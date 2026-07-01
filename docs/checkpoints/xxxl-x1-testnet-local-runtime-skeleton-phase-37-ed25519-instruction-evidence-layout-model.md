# XXXL X1 Testnet Local Runtime Skeleton Phase 37 Ed25519 Instruction Evidence Layout Model

Status: Read-only structural Ed25519 instruction evidence layout model.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-37-ed25519-instruction-evidence-layout-model`

Base context:

- Phase 32 added the read-only Rust/SVM verifier scaffold.
- Phase 33 added the Rust/SVM raw payload decoder.
- Phase 34 added Rust/SVM canonical payload hash/domain validation.
- Phase 35 added guardian membership and quorum structural verification.
- Phase 36 defined the Ed25519 signature evidence boundary (docs-only).

## Purpose

Phase 37 turns the Ed25519 evidence model selected in Phase 36 into a typed and
tested Rust layout model.

It models the shape of the Solana Ed25519 program instruction data for the
single-guardian-per-instruction evidence model.

It adds a Rust source module and Rust unit tests.

It does not implement Ed25519 verification.

It does not parse the Instructions sysvar.

It does not inspect real ed25519 program instructions.

It does not read accounts.

It does not unlock runtime execution.

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

## Modeled Layout

The modeled Solana Ed25519 program instruction data layout is:

- header: `num_signatures: u8` at offset 0, `padding: u8` at offset 1 (2 bytes);
- one `Ed25519SignatureOffsets` record of 7 x `u16` little-endian fields
  (14 bytes): `signature_offset`, `signature_instruction_index`,
  `public_key_offset`, `public_key_instruction_index`, `message_data_offset`,
  `message_data_size`, `message_instruction_index`;
- signature length = 64 bytes;
- public key length = 32 bytes;
- expected message length = 32 bytes = the Phase 34 canonical payload hash;
- `EXPECTED_SIGNATURE_COUNT = 1` (single guardian per instruction);
- `u16::MAX` instruction-index sentinel meaning "this same ed25519 instruction's
  data".

`validate_ed25519_evidence_layout` performs only structural bounds and size
checks: single signature, 32-byte message bound to the Phase 34 hash length, and
all declared regions within the declared instruction data length and not
overlapping the header.

On success the result exposes:

- `ed25519_signature_verification_performed: false`
- `instructions_sysvar_read: false`
- `cryptographic_signature_proof_accepted: false`

## Files Added Or Changed

Added:

- `programs/xxxl-svm/src/verifier/ed25519_evidence_layout.rs`
- `docs/xxxl/xxxl-phase-37-ed25519-instruction-evidence-layout-model.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-37-ed25519-instruction-evidence-layout-model.md`

Changed:

- `programs/xxxl-svm/src/verifier/mod.rs`
- `docs/checkpoints/current-design-checkpoint.md`

This phase does add a Rust source file and Rust tests, unlike Phase 36.

No Cargo manifest is changed.

No Cargo lockfile is changed.

No `deny.toml` is changed.

No package manifest or lockfile is changed.

No dependency is added.

No TypeScript source file is changed.

No TypeScript test file is changed.

No `programs/xxxl-svm/src/lib.rs` change is required.

No instruction handler is added.

No account parser is added.

## Explicit Non-Goals

Phase 37 does not implement Ed25519 verification.

Phase 37 does not parse real ed25519 instruction data.

Phase 37 does not parse the Instructions sysvar.

Phase 37 does not add an instruction handler.

Phase 37 does not parse runtime accounts.

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

Phase 37 does not enable processed-event marking.

Phase 37 does not select a production Program ID.

Phase 37 does not regenerate production PDA fixtures.

Phase 37 does not remove deployment blockers.

Phase 37 does not claim production readiness.

Phase 37 does not claim final immutability while upgrade authority exists.

Phase 37 does not change Cargo manifests.

Phase 37 does not add dependencies.

Phase 37 does not build SBF artifacts.

Phase 37 does not touch `target/deploy`.

Phase 37 does not read or modify keypair files.

Phase 37 does not read or modify `.env`.

Phase 37 does not inspect `.local-keys`.

Phase 37 does not run deploy commands.

Phase 37 does not run network commands.

Phase 37 does not spend SOL.

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

Commands to run:

- `cargo test --lib` from `programs/xxxl-svm`
- `git diff --check`
- `npm run typecheck`
- `npm run build`
- `git status --short --untracked-files=all`

A Rust source file was added and Rust unit tests were added.

No Cargo manifest was changed.

No Cargo lockfile was changed.

No SBF build was run.

No deploy or network command was run.

Recommended next stage:

- Phase 38 Rust/SVM Ed25519 instruction evidence parser over the real
  Instructions sysvar, still read-only, still without runtime unlock.
