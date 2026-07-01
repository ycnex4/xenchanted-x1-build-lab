# XXXL Phase 41C3A Edge Case Semantics Clarification Review Request

Status: External review request.

Audience:

- Audit demon
- Theo

## Context

Phase 41C3 was accepted.

The audit demon raised two non-blocking edge-case notes:

1. valid prior match plus same-index/later matching candidate
2. unrelated non-Ed25519 instructions should not be confused with evidence
   candidates in future real runtime-wiring

Phase 41C3A pins the descriptor-layer semantics and clarifies the future
runtime-wiring input contract.

## Review Scope

Please review Phase 41C3A only.

Primary files:

- `programs/xxxl-svm/src/verifier/prior_ed25519_lookup_ordering_boundary.rs`
- `docs/xxxl/xxxl-phase-41c3a-edge-case-semantics-clarification.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41c3a-edge-case-semantics-clarification.md`
- `docs/reviews/xxxl-phase-41c3a-edge-case-semantics-clarification-review-request.md`
- `docs/checkpoints/current-design-checkpoint.md`

## Questions For Reviewers

1. Did Phase 41C3A avoid changing runtime logic?

2. Does the added mixed-case test correctly pin current semantics?

3. Is it acceptable that one valid strictly-prior match plus one same-index or
   later matching descriptor returns `PriorEd25519InstructionLocatedAndOrdered`?

4. Is the descriptor input contract clear enough?

5. Is it clear that unrelated real non-Ed25519 transaction instructions must not
   be forwarded into Phase 41C3 as candidate descriptors?

6. Is `WrongEd25519ProgramId` correctly clarified as an evidence-candidate
   descriptor classification, not an arbitrary unrelated-instruction condition?

7. Does real runtime wiring remain deferred?

8. May the 41C descriptor series be closed after this clarification?

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
- whether the 41C descriptor series may be closed
- minimum safe next phase boundary
