# XXXL Phase 41D3.1 Current Index Runtime Boundary Review Request

Status: External review request.

Audience:

- Audit Demon
- Theo

## Context

Phase 41D3.0 was externally accepted.

Reviewers noted that Phase 41D3 is a dense runtime step.

Phase 41D3.1 therefore splits out only checked current-instruction index acquisition.

## Review Scope

Review Phase 41D3.1 only.

Primary files:

- `programs/xxxl-svm/src/verifier/current_instruction_index_runtime_boundary.rs`
- `programs/xxxl-svm/src/verifier/mod.rs`
- `docs/xxxl/xxxl-phase-41d3-1-current-index-runtime-boundary.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41d3-1-current-index-runtime-boundary.md`
- `docs/reviews/xxxl-phase-41d3-1-current-index-runtime-boundary-review-request.md`
- `docs/checkpoints/current-design-checkpoint.md`

## Questions For Reviewers

1. Is it acceptable to split current-index acquisition out as Phase 41D3.1?

2. Is Phase 41D3.1 limited to checked current-index acquisition?

3. Does it avoid `load_instruction`, `load_instruction_at`, and `load_instruction_at_checked`?

4. Does it avoid prior-instruction enumeration?

5. Does it avoid raw Instructions sysvar data parsing?

6. Does it avoid Phase 41C3 candidate descriptor construction?

7. Is `load_current_index_checked` the correct checked runtime boundary for this sub-step?

8. Does missing account or wrong account key fail closed?

9. Does checked current-index read failure fail closed?

10. Is the current index used only for ordering?

11. Is the implementation panic-safe?

12. Are all proof, evidence, quorum, authorization, replay, CPI, mint, handler, and live-route flags still false?

13. Are any blockers weakened, renamed, removed, or bypassed?

14. May the next prior-enumeration/loading sub-step start after this is accepted?

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
- whether the next Phase 41D3 sub-step may start
- minimum safe boundary for the next sub-step
