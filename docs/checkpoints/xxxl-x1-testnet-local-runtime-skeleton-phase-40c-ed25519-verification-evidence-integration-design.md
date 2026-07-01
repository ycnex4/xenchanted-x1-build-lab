# XXXL X1 Testnet Local Runtime Skeleton Phase 40C Ed25519 Verification Evidence Integration Design

Status: Docs-only integration design.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-40c-ed25519-verification-evidence-integration-design`

Base context:

- Phase 37 added the Ed25519 instruction evidence layout model.
- Phase 38 added the Ed25519 instruction data parser.
- Phase 39 added the prepared-entry Instructions sysvar evidence scanner.
- Phase 40A documented the verification evidence boundary.
- Phase 40B added a tiny Rust model proving located/parsed evidence is still
  not proof, quorum, authorization, or execution.

## Purpose

Phase 40C documents the future design for integrating SVM Ed25519 verification
evidence.

It defines what future code must prove before parsed candidate evidence can
become verification evidence.

Phase 40C is docs-only.

It adds no Rust code.

It does not parse the raw Instructions sysvar.

It does not call `load_instruction`.

It does not parse `AccountInfo`.

It does not verify Ed25519 signatures.

It does not count quorum.

It does not authorize minting.

It does not enable execution.

## Files Added Or Changed

Added:

- `docs/xxxl/xxxl-phase-40c-ed25519-verification-evidence-integration-design.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-40c-ed25519-verification-evidence-integration-design.md`

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

## Core Boundary

The preserved rule is:

~~~text
located candidate evidence
  != parsed evidence
  != verification evidence
  != quorum
  != authorization
  != execution
~~~

Phase 40C is only about the design boundary between parsed candidate evidence
and future verification evidence.

## Future Integration Requirements

Future verification evidence integration should require:

- prior SVM Ed25519 verification instruction
- actual Instructions sysvar read by runtime code
- exact Ed25519 program id
- supported Ed25519 instruction data layout
- Phase 37 layout constraints
- Phase 38 data parsing constraints
- Phase 34 runtime-recomputed payload hash match
- guardian public key match against the active guardian set
- deterministic failure reasons
- no caller-provided authorization booleans

## Future Rejection Cases

Future code should reject:

- missing Instructions sysvar
- unreadable Instructions sysvar
- missing current instruction identity
- Ed25519 instruction not found
- Ed25519 instruction after current XXXL instruction
- wrong Ed25519 program id
- malformed Ed25519 instruction data
- unsupported offset layout
- wrong guardian public key
- wrong message hash
- wrong guardian set id
- duplicated evidence where uniqueness is required
- ambiguous evidence
- expired evidence
- evidence for another route
- evidence for another target mint
- evidence for another recipient
- evidence for another amount

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

Phase 40D should be a tiny read-only Rust design surface for future Ed25519
verification evidence integration, still without quorum authorization, handler
or account parsing, CPI, mint execution, replay writes, or runtime unlock.
