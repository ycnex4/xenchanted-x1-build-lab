# XXXL Phase 41C1 Instructions Sysvar Presence Readability Boundary Review Request

Status: External review request.

Audience:

- Audit demon
- Theo

## Context

Phase 41C0A was accepted.

Phase 41C1 is the first narrow Rust boundary after the Phase 41C split.

It must remain limited to:

- concrete runtime API/helper selection
- Instructions sysvar presence
- Instructions sysvar readability
- deterministic results:
  - `MissingInstructionsSysvar`
  - `UnreadableInstructionsSysvar`
  - `PresentAndReadable`

## Review Scope

Please review Phase 41C1 only.

Primary files:

- `programs/xxxl-svm/src/verifier/instructions_sysvar_presence_readability_boundary.rs`
- `programs/xxxl-svm/src/verifier/mod.rs`
- `docs/xxxl/xxxl-phase-41c1-instructions-sysvar-presence-readability-boundary.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41c1-instructions-sysvar-presence-readability-boundary.md`
- `docs/reviews/xxxl-phase-41c1-instructions-sysvar-presence-readability-boundary-review-request.md`
- `docs/checkpoints/current-design-checkpoint.md`

## Questions For Reviewers

1. Did Phase 41C1 remain limited to presence/readability?

2. Did Phase 41C1 avoid `load_instruction`, `load_instruction_at`, and
   `load_instruction_at_checked`?

3. Did Phase 41C1 avoid concrete instruction content reading?

4. Did Phase 41C1 avoid current instruction identity derivation?

5. Did Phase 41C1 avoid prior Ed25519 lookup?

6. Did Phase 41C1 avoid proof, quorum, authorization, replay, CPI, and mint
   execution?

7. Is `concrete_runtime_api_selected` the only true safety flag?

8. Are `MissingInstructionsSysvar` and `UnreadableInstructionsSysvar` correctly
   mapped to the Phase 41B rejection taxonomy?

9. Is `PresentAndReadable` correctly treated as non-authorizing?

10. May Phase 41C2 start, and if so, what is its minimum safe boundary?

## Requested Verdict Format

Please answer with one of:

- ACCEPT
- ACCEPT WITH NOTES
- REQUEST CHANGES
- BLOCK

Please include:

- required fixes, if any
- blocking risks, if any
- optional notes, if any
- whether Phase 41C2 may start
- minimum safe Phase 41C2 boundary
