# XXXL Phase 41A SVM Instructions Sysvar Runtime Integration Plan Review Request

Status: External review request.

Audience:

- Audit demon
- Theo

## Context

Phase 40A-40G closed with independent review verdicts:

- `ACCEPT WITH NOTES`
- required fixes: none
- blocking risks: none

Both reviews agreed that Phase 41A must be docs-only.

Phase 41A now defines the safe runtime integration plan before any real
Instructions sysvar implementation.

## Review Scope

Please review Phase 41A only.

Primary files:

- `docs/xxxl/xxxl-phase-41a-svm-instructions-sysvar-runtime-integration-plan.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41a-svm-instructions-sysvar-runtime-integration-plan.md`
- `docs/reviews/xxxl-phase-41a-svm-instructions-sysvar-runtime-integration-plan-review-request.md`
- `docs/checkpoints/current-design-checkpoint.md`

## Core Boundary To Review

Phase 41A must preserve:

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

## Questions For Reviewers

1. Did Phase 41A correctly remain docs-only?

2. Did Phase 41A avoid selecting or calling a concrete runtime API?

3. Did Phase 41A avoid implying real raw Instructions sysvar parsing?

4. Did Phase 41A avoid implying Ed25519 proof acceptance?

5. Did Phase 41A avoid implying quorum, authorization, replay consumption, or
   mint execution?

6. Did Phase 41A correctly assign owning requirements to the four orphan
   rejection cases?

7. Are these ownership assignments correct?

   - `InstructionsSysvarReadable` -> `UnreadableInstructionsSysvar`
   - `PriorEd25519InstructionOrdering` -> `Ed25519InstructionAfterCurrentInstruction`
   - `GuardianEvidenceUniqueness` -> `DuplicateGuardianEvidence`
   - `SingleCandidateResolution` -> `AmbiguousCandidateEvidence`

8. Is the proposed safe read-only runtime integration contract sufficient?

9. Is the proposed Phase 41B shape safe as model-only?

10. Is anything missing before a future implementation phase?

## Requested Verdict Format

Please answer with one of:

- ACCEPT
- ACCEPT WITH NOTES
- REQUEST CHANGES
- BLOCK

Please include:

- required fixes, if any
- optional notes, if any
- whether Phase 41B may start as model-only
- minimum safe boundary for Phase 41B
