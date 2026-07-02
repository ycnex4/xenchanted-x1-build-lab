# XXXL Phase 41D2 Current Identity Runtime Boundary Review Request

Status: External review request.

Audience:

- Audit demon
- Theo

## Context

Phase 41D2.0 was accepted.

Phase 41D2 introduces real current-instruction identity population from direct entrypoint context.

It must not use `load_instruction_at`.

## Review Scope

Please review Phase 41D2 only.

Primary files:

- `programs/xxxl-svm/src/verifier/current_instruction_identity_runtime_boundary.rs`
- `programs/xxxl-svm/src/verifier/mod.rs`
- `docs/xxxl/xxxl-phase-41d2-current-identity-runtime-boundary.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41d2-current-identity-runtime-boundary.md`
- `docs/reviews/xxxl-phase-41d2-current-identity-runtime-boundary-review-request.md`
- `docs/checkpoints/current-design-checkpoint.md`

## Questions For Reviewers

1. Is Phase 41D2 limited to current-instruction identity population?

2. Is identity derived from direct entrypoint `program_id` and `instruction_data`?

3. Does Phase 41D2 avoid `load_instruction`, `load_instruction_at`, and `load_instruction_at_checked`?

4. Does Phase 41D2 avoid raw Instructions sysvar parsing?

5. Does Phase 41D2 avoid prior-instruction enumeration?

6. Does Phase 41D2 avoid Phase 41C3 candidate descriptor construction?

7. Is discriminator checking length-safe?

8. Does short instruction data deterministically map to inconsistency, not panic?

9. Does missing program id, missing instruction data, or empty expected discriminator deterministically map to missing current identity?

10. Does valid current identity map only to `CurrentInstructionIdentityBound`, without proof/evidence/auth?

11. Is the implementation panic-safe?

12. Does Phase 41D2 flip only the intended current-identity runtime flag?

13. Are all proof, evidence, quorum, authorization, replay, CPI, mint, handler, and live-route flags still false?

14. May Phase 41D3 start after this phase is accepted?

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
- whether Phase 41D3 may start
- minimum safe Phase 41D3 boundary
