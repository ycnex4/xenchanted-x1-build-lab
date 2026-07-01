# XXXL Phase 39 Instructions Sysvar Evidence Scanner

Status: Narrow read-only scanner over prepared Instructions sysvar evidence
entries.

## Purpose

Phase 39 scans a read-only representation derived from Instructions sysvar data.

The scanner receives prepared instruction entries:

- program id reference
- instruction data bytes

It identifies candidate Ed25519 program instruction entries.

It feeds candidate instruction data bytes into the Phase 38 parser.

It returns success only when exactly one candidate matches the expected guardian
public key and expected Phase 34 canonical payload hash.

This is located and parsed candidate evidence only.

It is not cryptographic proof.

It is not authorization.

## Base Context

Previous Rust/SVM verifier phases:

- Phase 32: read-only runtime verifier scaffold.
- Phase 33: raw payload decoder.
- Phase 34: canonical payload hash/domain validation.
- Phase 35: guardian membership and quorum structural verifier.
- Phase 37: Ed25519 instruction evidence layout model.
- Phase 38: Ed25519 instruction data parser.

Phase 39 requires:

- Phase 37 layout constants and layout model.
- Phase 38 instruction data parser.
- Phase 34 hash validator remains available and recomputing.
- Phase 35 quorum remains separate and not counted.

## Preserved Security Decision

The preserved security decision remains:

~~~text
TS layer = preflight / model / watcher-side decision
Runtime = independent verifier
No authorized=true -> execute
~~~

TypeScript authorization output is not runtime authority.

TypeScript parity results are not runtime authority.

Located Ed25519 instruction candidates are not runtime authority.

Parsed signature bytes are not runtime authority.

## Source Boundary

New Rust module:

- `programs/xxxl-svm/src/verifier/instructions_sysvar_evidence_scanner.rs`

Updated Rust verifier export:

- `programs/xxxl-svm/src/verifier/mod.rs`

No `programs/xxxl-svm/src/lib.rs` change is required.

No Cargo manifest change is required.

No package manifest or lockfile change is required.

No dependency is added.

## Scanner API

The Phase 39 marker is:

~~~text
INSTRUCTIONS_SYSVAR_EVIDENCE_SCANNER_PHASE_39
~~~

The scanner version is:

~~~text
1
~~~

The exposed Rust API includes:

- `InstructionsSysvarInstructionView`
- `ScannedEd25519InstructionEvidence`
- `InstructionsSysvarEvidenceScannerReport`
- `InstructionsSysvarEvidenceScannerError`
- `InstructionsSysvarEvidenceScannerErrorKind`
- `scan_instructions_sysvar_for_ed25519_evidence`
- `instructions_sysvar_evidence_scanner_report`

## Representation Choice

Phase 39 uses prepared read-only instruction entries.

It does not parse raw Instructions sysvar account data.

It does not parse `AccountInfo`.

It does not own full sysvar account integration.

This keeps the phase focused on scanner logic:

- scan zero or more prepared entries
- skip non-Ed25519 program entries
- parse Ed25519 candidate instruction data through Phase 38
- require exactly one matching parsed candidate

## Behavior

The scanner rejects:

- empty instruction set
- no matching Ed25519 evidence
- duplicate matching Ed25519 evidence
- malformed Ed25519 candidate instruction data through Phase 38 parser errors
- guardian public key mismatch through Phase 38 parser errors
- message hash mismatch through Phase 38 parser errors

The scanner returns success when:

- exactly one candidate Ed25519 program instruction parses through Phase 38
- the parsed public key matches the expected guardian public key
- the parsed message matches the expected Phase 34 payload hash

The success result includes:

- matched instruction index
- parsed Phase 38 Ed25519 instruction evidence
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

Signature bytes remain future verification evidence only.

A successful scan is only located and parsed candidate evidence.

It is not cryptographic proof and not authorization.

## Explicit Non-Goals

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

## Preserved Blockers

The following blockers remain active:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker is removed, weakened, renamed, or satisfied by Phase 39.

## Recommended Next Stage

Phase 40 should add a read-only Ed25519 verification evidence integration
boundary.

That future phase should still avoid quorum authorization, handler or account
parsing, CPI, mint execution, replay writes, and runtime unlock.
