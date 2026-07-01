# XXXL Phase 41D2.0 Current Identity Runtime Mechanism Clarification Review Request

Status: External review request.

Audience:

- Audit demon
- Theo

## Context

Phase 41D1 was accepted.

Phase 41D2.0 is a docs-only clarification before future Phase 41D2 code.

It addresses the forward-looking review note that Phase 41D2 must derive current-instruction identity without `load_instruction_at`, because the Phase 41D0 per-flag plan keeps `load_instruction_called: false` until 41D3.

## Review Scope

Please review Phase 41D2.0 only.

Primary files:

- `docs/xxxl/xxxl-phase-41d2-0-current-identity-runtime-mechanism-clarification.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41d2-0-current-identity-runtime-mechanism-clarification.md`
- `docs/reviews/xxxl-phase-41d2-0-current-identity-runtime-mechanism-clarification-review-request.md`
- `docs/checkpoints/current-design-checkpoint.md`

## Questions For Reviewers

1. Did Phase 41D2.0 remain docs-only?

2. Does it correctly address the Phase 41D1 forward-looking note?

3. Is future 41D2 correctly limited to current-instruction identity population?

4. Is future 41D2 correctly based on direct entrypoint `program_id` and `instruction_data`?

5. Does future 41D2 avoid `load_instruction`, `load_instruction_at`, and `load_instruction_at_checked`?

6. If current-index access is used, is it correctly limited to checked current-index access and not full instruction loading?

7. Does the plan keep `load_instruction_called: false` through 41D2?

8. Does the plan allow only `current_instruction_identity_derived_from_runtime: true` to flip in 41D2?

9. Does the plan keep prior enumeration and Phase 41C3 descriptors deferred to 41D3?

10. Does the plan keep proof, evidence, quorum, authorization, replay, CPI, mint, handler, and live route forbidden?

11. May Phase 41D2 code start after this clarification is accepted?

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
- whether Phase 41D2 code may start
- minimum safe Phase 41D2 boundary
