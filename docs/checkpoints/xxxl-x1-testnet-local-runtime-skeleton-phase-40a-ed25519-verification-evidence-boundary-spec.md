# XXXL X1 Testnet Local Runtime Skeleton Phase 40A Ed25519 Verification Evidence Boundary Spec

Status: Docs-only boundary specification.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-40a-ed25519-verification-evidence-boundary-spec`

Base context:

- Phase 32 added the read-only Rust/SVM verifier scaffold.
- Phase 33 added the raw payload decoder.
- Phase 34 added canonical payload hash/domain validation.
- Phase 35 added guardian membership and quorum structural verification.
- Phase 36 documented the Ed25519 signature evidence boundary.
- Phase 37 added the Ed25519 instruction evidence layout model.
- Phase 38 added the Ed25519 instruction data parser.
- Phase 39 added the prepared-entry Instructions sysvar evidence scanner.

## Purpose

Phase 40A defines the boundary between located/parsed Ed25519 candidate evidence
and future Ed25519 verification evidence.

Phase 39 can locate and parse a candidate.

Phase 40A documents what a future verifier phase must prove before a candidate
can become verification evidence.

Phase 40A is docs-only.

It adds no Rust code.

It adds no runtime behavior.

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

- `docs/xxxl/xxxl-phase-40a-ed25519-verification-evidence-boundary-spec.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-40a-ed25519-verification-evidence-boundary-spec.md`

Changed:

- `docs/checkpoints/current-design-checkpoint.md`

No Rust source file is changed.

No TypeScript source file is changed.

No TypeScript test file is changed.

No Cargo file is changed.

No package manifest or lockfile is changed.

No dependency is added.

No `programs/xxxl-svm/src/lib.rs` change is required.

No `programs/xxxl-svm/src/verifier/mod.rs` change is required.

## Boundary Defined

Phase 40A defines these distinctions:

- located candidate evidence is not parsed evidence
- parsed evidence is not cryptographic proof
- cryptographic verification evidence is not quorum
- quorum is not full mint authorization
- authorization is not execution
- execution remains separately gated

The core rule is:

~~~text
located candidate evidence
  != parsed evidence
  != verification evidence
  != quorum
  != authorization
  != execution
~~~

## Future Verification Evidence Requirements

A future Rust/SVM phase must bind verification evidence to:

- expected guardian public key
- expected Phase 34 canonical payload hash
- exact Ed25519 signature bytes
- Ed25519 program id
- instruction identity
- Phase 38 parsed instruction data
- Phase 39 located candidate
- deterministic failure reasons

Future code must not trust caller-provided booleans such as:

- `signature_verified`
- `guardian_verified`
- `authorized`
- `quorum_reached`

## Explicit Non-Goals

Phase 40A does not add Rust code.

Phase 40A does not add a new verifier module.

Phase 40A does not modify `programs/xxxl-svm/src/lib.rs`.

Phase 40A does not modify `programs/xxxl-svm/src/verifier/mod.rs`.

Phase 40A does not parse raw Instructions sysvar account data.

Phase 40A does not parse `AccountInfo`.

Phase 40A does not call `load_instruction`.

Phase 40A does not verify Ed25519 signatures.

Phase 40A does not accept cryptographic signature proof.

Phase 40A does not count quorum.

Phase 40A does not authorize minting.

Phase 40A does not add `process_instruction`.

Phase 40A does not add a runtime instruction handler.

Phase 40A does not add account parsing.

Phase 40A does not add CPI.

Phase 40A does not enable `invoke_signed`.

Phase 40A does not enable SPL Token `mint_to`.

Phase 40A does not add replay writes.

Phase 40A does not mark processed events.

Phase 40A does not mutate runtime/account state.

Phase 40A does not unlock live route execution.

Phase 40A does not remove deployment blockers.

Phase 40A does not select a production Program ID.

Phase 40A does not claim production readiness.

Phase 40A does not claim final immutability while upgrade authority exists.

Phase 40A does not build SBF artifacts.

Phase 40A does not touch `target/deploy`.

Phase 40A does not read or modify keypair files.

Phase 40A does not read or modify `.env`.

Phase 40A does not inspect `.local-keys`.

Phase 40A does not run deploy commands.

Phase 40A does not run network commands.

Phase 40A does not spend SOL.

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

Suggested validation for this docs-only phase:

- `git diff --check`
- `npm run typecheck`
- `npm run build`
- `git status --short --untracked-files=all`

Cargo validation is not required because no Rust source file is changed.

No SBF build should be run.

## Recommended Next Stage

Phase 40B should add a read-only Rust/SVM verification evidence model, still
without quorum authorization, handler or account parsing, CPI, mint execution,
replay writes, or runtime unlock.
