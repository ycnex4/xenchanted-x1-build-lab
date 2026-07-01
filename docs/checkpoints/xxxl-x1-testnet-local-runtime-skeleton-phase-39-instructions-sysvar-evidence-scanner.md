# XXXL X1 Testnet Local Runtime Skeleton Phase 39 Instructions Sysvar Evidence Scanner

Status: Narrow read-only scanner over prepared Instructions sysvar evidence
entries.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-39-instructions-sysvar-evidence-scanner`

Base context:

- Phase 32 added the read-only Rust/SVM verifier scaffold.
- Phase 33 added the raw payload decoder.
- Phase 34 added canonical payload hash/domain validation.
- Phase 35 added guardian membership and quorum structural verification.
- Phase 37 added the Ed25519 instruction evidence layout model.
- Phase 38 added the Ed25519 instruction data parser.

## Purpose

Phase 39 scans read-only Instructions sysvar evidence entries.

It identifies candidate Ed25519 program instruction entries.

It feeds candidate instruction data bytes into the Phase 38 parser.

It returns matched parsed evidence for a specific expected guardian public key
and expected Phase 34 payload hash.

It does not verify Ed25519 signatures.

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

- `programs/xxxl-svm/src/verifier/instructions_sysvar_evidence_scanner.rs`
- `docs/xxxl/xxxl-phase-39-instructions-sysvar-evidence-scanner.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-39-instructions-sysvar-evidence-scanner.md`

Changed:

- `programs/xxxl-svm/src/verifier/mod.rs`
- `docs/checkpoints/current-design-checkpoint.md`

No TypeScript source file is changed.

No TypeScript test file is changed.

No Cargo file is changed.

No package manifest or lockfile is changed.

No dependency is added.

No `programs/xxxl-svm/src/lib.rs` change is required.

## Representation Choice

Phase 39 uses prepared read-only instruction entries:

- program id reference
- instruction data bytes

Phase 39 does not parse raw Instructions sysvar account data.

Phase 39 does not parse `AccountInfo`.

Phase 39 does not own full sysvar account integration.

This keeps the phase focused on scanner logic while preserving the future
account/sysvar integration boundary.

## Scanner Boundary

Scanner marker:

- `INSTRUCTIONS_SYSVAR_EVIDENCE_SCANNER_PHASE_39`

Scanner version:

- `1`

The scanner requires:

- Phase 37 layout model
- Phase 38 instruction data parser
- Phase 34 hash validator remains available and recomputing
- Phase 35 quorum remains separate and not counted

## Behavior Implemented

The scanner:

- scans zero or more prepared instruction entries
- skips non-Ed25519 program instructions
- identifies Ed25519 program instruction candidates
- calls `parse_ed25519_instruction_data_evidence` for candidate data bytes
- returns success only when exactly one candidate matches expected guardian and
  expected Phase 34 hash
- rejects empty instruction sets
- rejects no matching Ed25519 evidence
- rejects duplicate matching Ed25519 evidence
- reports malformed Ed25519 candidate data through deterministic Phase 38 parser
  errors
- reports guardian public key mismatch through deterministic Phase 38 parser
  errors
- reports message hash mismatch through deterministic Phase 38 parser errors

The success result includes:

- matched instruction index
- parsed Phase 38 evidence
- scanned instruction count
- Ed25519 candidate count
- non-Ed25519 instruction count
- public key match flag set to true
- message hash match flag set to true
- Ed25519 signature verification flag set to false
- cryptographic proof accepted flag set to false
- quorum counted flag set to false
- authorization granted flag set to false

## Explicit Honesty

Phase 39 scans Instructions sysvar evidence in read-only form.

Phase 39 may locate Ed25519 program instruction candidates.

Phase 39 may feed candidate Ed25519 instruction data into the Phase 38 parser.

Phase 39 does not verify Ed25519 signatures.

Phase 39 does not prove that a signature is valid.

Phase 39 does not count quorum.

Phase 39 does not authorize minting.

Phase 39 does not add `process_instruction`.

Phase 39 does not add account parsing.

Phase 39 does not add CPI, `invoke_signed`, `mint_to`, replay writes,
processed-event marking, runtime mutation, live route unlock, or deployment
blocker removal.

A successful scan is only located and parsed candidate evidence.

It is not cryptographic proof and not authorization.

Signature bytes remain future verification evidence only.

## Remaining Obligations

Remaining obligations include:

- raw Instructions sysvar account integration, if selected in a future phase
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

Phase 39 does not perform Ed25519 cryptographic verification.

Phase 39 does not accept cryptographic signature proof.

Phase 39 does not count quorum.

Phase 39 does not authorize minting.

Phase 39 does not add `process_instruction`.

Phase 39 does not add a runtime instruction handler.

Phase 39 does not add account parsing.

Phase 39 does not add CPI.

Phase 39 does not enable `invoke_signed`.

Phase 39 does not enable SPL Token `mint_to`.

Phase 39 does not add replay writes.

Phase 39 does not mark processed events.

Phase 39 does not mutate runtime/account state.

Phase 39 does not unlock live route execution.

Phase 39 does not remove deployment blockers.

Phase 39 does not select a production Program ID.

Phase 39 does not claim production readiness.

Phase 39 does not claim final immutability while upgrade authority exists.

Phase 39 does not build SBF artifacts.

Phase 39 does not touch `target/deploy`.

Phase 39 does not read or modify keypair files.

Phase 39 does not read or modify `.env`.

Phase 39 does not inspect `.local-keys`.

Phase 39 does not run deploy commands.

Phase 39 does not run network commands.

Phase 39 does not spend SOL.

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
- `wsl bash -lc "cd /mnt/c/Users/user/xenchanted-x1-build-lab/programs/xxxl-svm && cargo test instructions_sysvar_evidence_scanner --lib"`: passed, 15 tests passed
- `wsl bash -lc "cd /mnt/c/Users/user/xenchanted-x1-build-lab/programs/xxxl-svm && cargo test verifier --lib"`: passed, 99 tests passed
- `wsl bash -lc "cd /mnt/c/Users/user/xenchanted-x1-build-lab/programs/xxxl-svm && cargo test --lib --locked"`: passed, 310 tests passed, 1 ignored
- `npm run typecheck`: passed
- `npm run build`: passed
- `git status --short --untracked-files=all`: run for final workspace state

No Cargo manifest was changed.

No Cargo lockfile was changed.

No SBF build was run.

## Recommended Next Stage

Phase 40 should add a read-only Ed25519 verification evidence integration
boundary, still without quorum authorization, handler or account parsing, CPI,
mint execution, replay writes, or runtime unlock.
