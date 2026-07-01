# XXXL X1 Testnet Local Runtime Skeleton Phase 41A SVM Instructions Sysvar Runtime Integration Plan

Status: Docs-only reviewed runtime integration plan.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41a-instructions-sysvar-runtime-integration-plan`

## Purpose

Phase 41A opens Phase 41 after the Phase 40A-40G control point.

It defines a docs-only safe plan for future SVM Instructions sysvar runtime
integration.

It also assigns owning requirements to the four orphan rejection cases identified
by the Phase 40 reviews.

Phase 41A adds no Rust code.

It modifies no Rust source file.

It does not parse raw Instructions sysvar data.

It does not call `load_instruction`.

It does not parse `AccountInfo`.

It does not verify Ed25519 signatures.

It does not accept cryptographic proof.

It does not count quorum.

It does not authorize minting.

It does not enable execution.

## Files Added Or Changed

Added:

- `docs/xxxl/xxxl-phase-41a-svm-instructions-sysvar-runtime-integration-plan.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41a-svm-instructions-sysvar-runtime-integration-plan.md`
- `docs/reviews/xxxl-phase-41a-svm-instructions-sysvar-runtime-integration-plan-review-request.md`

Changed:

- `docs/checkpoints/current-design-checkpoint.md`

No Rust source file is changed.

No TypeScript source file is changed.

No TypeScript test file is changed.

No Cargo manifest is changed.

No package manifest or lockfile is changed.

No dependency is added.

No `programs/xxxl-svm/src/lib.rs` change is required.

No `programs/xxxl-svm/src/verifier/mod.rs` change is required.

## Review Inputs

Phase 40 review consensus:

- verdict: `ACCEPT WITH NOTES`
- required fixes: none
- blocking risks: none
- Phase 41A must be docs-only
- four orphan rejection cases must receive owning requirements before any
  implementation

## Orphan Rejection Ownership

| Owning requirement | Rejection case |
| --- | --- |
| `InstructionsSysvarReadable` | `UnreadableInstructionsSysvar` |
| `PriorEd25519InstructionOrdering` | `Ed25519InstructionAfterCurrentInstruction` |
| `GuardianEvidenceUniqueness` | `DuplicateGuardianEvidence` |
| `SingleCandidateResolution` | `AmbiguousCandidateEvidence` |

## Boundary Preserved

~~~text
located candidate evidence
  != parsed evidence
  != prior-instruction ordering
  != requirement coverage
  != runtime sysvar read
  != verification evidence
  != quorum
  != authorization
  != execution
~~~

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

Suggested validation:

- `git diff --check`
- `npm run typecheck`
- `npm run build`

Cargo validation is not required because no Rust source file is changed.

No SBF build should be run.

## Recommended Next Stage

After review, Phase 41B should be model-only.

Phase 41B should not parse real raw Instructions sysvar account data.
