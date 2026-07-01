# XXXL Phase 41C2 Current Instruction Identity Boundary Review Request

Status: External review request.

Audience:

- Audit demon
- Theo

## Context

Phase 41C1 was accepted.

Phase 41C2 introduces a current-instruction identity boundary over an explicit
descriptor.

It does not read real Solana `AccountInfo`.

It does not populate the descriptor from real Instructions sysvar data.

It does not call `load_instruction`, `load_instruction_at`, or
`load_instruction_at_checked`.

## Review Scope

Please review Phase 41C2 only.

Primary files:

- `programs/xxxl-svm/src/verifier/current_instruction_identity_boundary.rs`
- `programs/xxxl-svm/src/verifier/mod.rs`
- `docs/xxxl/xxxl-phase-41c2-current-instruction-identity-boundary.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41c2-current-instruction-identity-boundary.md`
- `docs/reviews/xxxl-phase-41c2-current-instruction-identity-boundary-review-request.md`
- `docs/checkpoints/current-design-checkpoint.md`

## Questions For Reviewers

1. Did Phase 41C2 remain limited to current-instruction identity descriptor
   binding?

2. Did Phase 41C2 avoid real `AccountInfo` parsing?

3. Did Phase 41C2 avoid real Instructions sysvar parsing?

4. Did Phase 41C2 avoid `load_instruction`, `load_instruction_at`, and
   `load_instruction_at_checked`?

5. Did Phase 41C2 avoid concrete instruction content reading?

6. Did Phase 41C2 avoid prior Ed25519 lookup?

7. Did Phase 41C2 avoid proof, quorum, authorization, replay, CPI, and mint
   execution?

8. Are missing and inconsistent current identity mapped to the Phase 41B
   `MissingCurrentInstructionIdentity` rejection case?

9. Is `CurrentInstructionIdentityBound` correctly non-authorizing?

10. Is it acceptable that real runtime population remains deferred to a separate
    future reviewed phase?

11. May Phase 41C3 start, and if so, what is its minimum safe boundary?

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
- whether Phase 41C3 may start
- minimum safe Phase 41C3 boundary
