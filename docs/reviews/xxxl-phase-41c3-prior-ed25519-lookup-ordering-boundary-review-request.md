# XXXL Phase 41C3 Prior Ed25519 Lookup Ordering Boundary Review Request

Status: External review request.

Audience:

- Audit demon
- Theo

## Context

Phase 41C2 was accepted.

Phase 41C3 introduces prior Ed25519 lookup and strict ordering over explicit
descriptors.

It does not read real Solana `AccountInfo`.

It does not populate descriptors from real Instructions sysvar data.

It does not call `load_instruction`, `load_instruction_at`, or
`load_instruction_at_checked`.

## Review Scope

Please review Phase 41C3 only.

Primary files:

- `programs/xxxl-svm/src/verifier/prior_ed25519_lookup_ordering_boundary.rs`
- `programs/xxxl-svm/src/verifier/mod.rs`
- `docs/xxxl/xxxl-phase-41c3-prior-ed25519-lookup-ordering-boundary.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41c3-prior-ed25519-lookup-ordering-boundary.md`
- `docs/reviews/xxxl-phase-41c3-prior-ed25519-lookup-ordering-boundary-review-request.md`
- `docs/checkpoints/current-design-checkpoint.md`

## Questions For Reviewers

1. Did Phase 41C3 remain limited to descriptor-based prior Ed25519 lookup and
   strict ordering?

2. Did Phase 41C3 avoid real `AccountInfo` parsing?

3. Did Phase 41C3 avoid real Instructions sysvar parsing?

4. Did Phase 41C3 avoid `load_instruction`, `load_instruction_at`, and
   `load_instruction_at_checked`?

5. Did Phase 41C3 avoid concrete instruction content reading?

6. Did Phase 41C3 avoid Ed25519 signature verification?

7. Did Phase 41C3 avoid proof, quorum, authorization, replay, CPI, and mint
   execution?

8. Are rejection cases mapped to the Phase 41B taxonomy correctly?

9. Is strict ordering enforced as `candidate_index < current_instruction_index`?

10. Is the located-and-ordered success state correctly non-authorizing?

11. Is it acceptable that real runtime population remains deferred to a separate
    future reviewed phase?

12. What should the next safe phase be?

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
- whether a next phase may start
- minimum safe next phase boundary
