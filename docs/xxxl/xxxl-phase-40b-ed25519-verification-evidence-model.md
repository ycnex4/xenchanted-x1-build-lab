# XXXL Phase 40B Ed25519 Verification Evidence Model

Status: Tiny read-only Rust/SVM boundary model.

## Purpose

Phase 40B adds a Rust model for the boundary between Phase 39 located candidate
evidence and future Ed25519 verification evidence.

Phase 40B does not verify Ed25519 signatures.

Phase 40B does not accept cryptographic signature proof.

Phase 40B does not count quorum.

Phase 40B does not authorize minting.

The only new capability is a read-only model that checks that Phase 39 scanned
candidate evidence still does not claim verification, proof, quorum,
authorization, or execution.

## Base Context

Previous phases:

- Phase 37: Ed25519 instruction evidence layout model.
- Phase 38: Ed25519 instruction data parser.
- Phase 39: prepared-entry Instructions sysvar evidence scanner.
- Phase 40A: docs-only verification evidence boundary spec.

Phase 40B turns the Phase 40A boundary rule into a small Rust model.

## Boundary Rule

Future code must preserve this separation:

~~~text
located candidate evidence
  != parsed evidence
  != verification evidence
  != quorum
  != authorization
  != execution
~~~

Phase 40B accepts clean located/parsed candidate evidence only as input to a
non-authorizing boundary result.

That result still reports:

- Ed25519 signature verification performed: false
- cryptographic signature proof accepted: false
- verification evidence accepted: false
- quorum counted: false
- authorization granted: false
- execution surfaces enabled: false

## Source Boundary

New Rust module:

- `programs/xxxl-svm/src/verifier/ed25519_verification_evidence.rs`

Updated Rust verifier export:

- `programs/xxxl-svm/src/verifier/mod.rs`

No `programs/xxxl-svm/src/lib.rs` change is required.

No Cargo manifest change is required.

No package manifest or lockfile change is required.

No dependency is added.

## Model API

The Phase 40B marker is:

~~~text
ED25519_VERIFICATION_EVIDENCE_MODEL_PHASE_40B
~~~

The exposed Rust API includes:

- `Ed25519VerificationEvidenceBoundaryResult`
- `Ed25519VerificationEvidenceBoundaryError`
- `Ed25519VerificationEvidenceBoundaryErrorKind`
- `Ed25519VerificationEvidenceModelReport`
- `model_ed25519_verification_evidence_boundary`
- `ed25519_verification_evidence_model_report`

## Behavior

The model accepts a `ScannedEd25519InstructionEvidence` from Phase 39.

It rejects candidate evidence if it claims:

- public key mismatch
- message hash mismatch
- Instructions sysvar read by the Phase 38 parser
- Ed25519 signature verification
- cryptographic signature proof
- quorum counted
- authorization granted
- execution surface enabled

It returns a non-authorizing boundary result for clean candidate evidence.

## Explicit Non-Goals

Phase 40B does not implement Ed25519 cryptographic verification.

Phase 40B does not accept cryptographic signature proof.

Phase 40B does not produce verified guardian evidence.

Phase 40B does not count quorum.

Phase 40B does not authorize minting.

Phase 40B does not add `process_instruction`.

Phase 40B does not add a runtime instruction handler.

Phase 40B does not add account parsing.

Phase 40B does not parse raw Instructions sysvar account data.

Phase 40B does not parse `AccountInfo`.

Phase 40B does not call `load_instruction`.

Phase 40B does not add CPI.

Phase 40B does not enable `invoke_signed`.

Phase 40B does not enable SPL Token `mint_to`.

Phase 40B does not add replay writes.

Phase 40B does not mark processed events.

Phase 40B does not mutate runtime/account state.

Phase 40B does not unlock live route execution.

Phase 40B does not remove deployment blockers.

Phase 40B does not select a production Program ID.

Phase 40B does not claim production readiness.

Phase 40B does not claim final immutability while upgrade authority exists.

## Preserved Blockers

The following blockers remain active:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker is removed, weakened, renamed, or satisfied by Phase 40B.

## Recommended Next Stage

Phase 40C should define a read-only Ed25519 verification evidence integration
design for actual SVM Ed25519 verification evidence, still without quorum
authorization, handler or account parsing, CPI, mint execution, replay writes,
or runtime unlock.
