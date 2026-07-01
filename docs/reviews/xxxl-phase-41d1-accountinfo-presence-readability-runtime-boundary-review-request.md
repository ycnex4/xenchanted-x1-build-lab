# XXXL Phase 41D1 AccountInfo Presence Readability Runtime Boundary Review Request

Status: External review request.

Audience:

- Audit demon
- Theo

## Context

Phase 41D0 was accepted.

Phase 41D1 is the first real runtime-read boundary.

It is intentionally limited to Instructions sysvar `AccountInfo`
presence/readability and maps to existing Phase 41C1 descriptor states.

## Review Scope

Please review Phase 41D1 only.

Primary files:

- `programs/xxxl-svm/src/verifier/instructions_sysvar_accountinfo_presence_readability_runtime_boundary.rs`
- `programs/xxxl-svm/src/verifier/mod.rs`
- `docs/xxxl/xxxl-phase-41d1-accountinfo-presence-readability-runtime-boundary.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41d1-accountinfo-presence-readability-runtime-boundary.md`
- `docs/reviews/xxxl-phase-41d1-accountinfo-presence-readability-runtime-boundary-review-request.md`
- `docs/checkpoints/current-design-checkpoint.md`

## Questions For Reviewers

1. Is Phase 41D1 limited to real Instructions sysvar `AccountInfo`
   presence/readability?

2. Does Phase 41D1 avoid `load_instruction`, `load_instruction_at`, and
   `load_instruction_at_checked`?

3. Does Phase 41D1 avoid reading concrete instruction contents?

4. Does Phase 41D1 avoid current-instruction identity derivation?

5. Does Phase 41D1 avoid prior-instruction enumeration?

6. Does Phase 41D1 avoid Phase 41C3 candidate descriptor construction?

7. Does missing account map deterministically to `MissingInstructionsSysvar`?

8. Does wrong account key map deterministically to `MissingInstructionsSysvar`?

9. Does borrow failure map deterministically to `UnreadableInstructionsSysvar`?

10. Does readable account map only to `PresentAndReadable`, without proof or
    authorization?

11. Is the implementation panic-safe?

12. Does Phase 41D1 flip only the intended read-capability flag?

13. Are all proof, evidence, quorum, authorization, replay, CPI, mint, handler,
    and live-route flags still false?

14. May Phase 41D2 start after this phase is accepted?

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
- whether Phase 41D2 may start
- minimum safe Phase 41D2 boundary
