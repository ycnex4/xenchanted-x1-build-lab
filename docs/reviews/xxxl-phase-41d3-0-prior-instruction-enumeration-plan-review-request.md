# XXXL Phase 41D3.0 Prior Instruction Enumeration Plan Review Request

Status: External review request.

Audience:

- Audit Demon
- Theo

## Context

Phase 41D2 has been externally accepted.

Phase 41D3 may start.

Before adding code, Phase 41D3.0 defines the safety boundary for real prior-instruction enumeration.

## Review Scope

Please review only the Phase 41D3.0 docs-only plan.

Primary files:

- `docs/xxxl/xxxl-phase-41d3-0-prior-instruction-enumeration-runtime-boundary-plan.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41d3-0-prior-instruction-enumeration-plan.md`
- `docs/reviews/xxxl-phase-41d3-0-prior-instruction-enumeration-plan-review-request.md`
- `docs/checkpoints/current-design-checkpoint.md`

## Questions For Reviewers

1. Is it correct to make Phase 41D3.0 docs-only before 41D3 code?

2. Is the allowed Phase 41D3 scope narrow enough?

3. Is real prior-instruction enumeration via Instructions sysvar acceptable at 41D3?

4. Is checked instruction loading through `load_instruction_at_checked` or reviewed equivalent the correct boundary?

5. Is raw Instructions sysvar byte parsing still correctly excluded?

6. Is prefiltering unrelated instructions correctly separated from evidence acceptance?

7. Are Phase 41C3 candidate descriptors correctly treated as not proof, not evidence, and not authorization?

8. Is the same/later fully-matching Ed25519 anomaly decision required at 41D3?

9. Is the proposed minimum safe decision acceptable:
   - same-index match: reject
   - later-index match: reject
   - prior-index match: candidate only, not proof

10. Are the allowed flag flips limited enough?

11. Are all trust-sensitive flags kept false?

12. Is the proposed test coverage sufficient for 41D3 code?

13. Are any blockers weakened, renamed, removed, or bypassed?

14. May Phase 41D3 code begin from this boundary?

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
- whether Phase 41D3 code may start
- minimum safe Phase 41D3 code boundary
