# XXXL X1 Testnet Local Runtime Skeleton Phase 41D2.0 Current Identity Runtime Mechanism Clarification

Status: Docs-only clarification.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41d2-0-current-identity-runtime-mechanism-clarification`

## Purpose

Phase 41D2.0 clarifies how future Phase 41D2 must derive current-instruction identity without violating the Phase 41D0 per-flag plan.

## Files Added Or Changed

Added:

- `docs/xxxl/xxxl-phase-41d2-0-current-identity-runtime-mechanism-clarification.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41d2-0-current-identity-runtime-mechanism-clarification.md`
- `docs/reviews/xxxl-phase-41d2-0-current-identity-runtime-mechanism-clarification-review-request.md`

Changed:

- `docs/checkpoints/current-design-checkpoint.md`

No Rust source file is changed.

No TypeScript source file is changed.

No Cargo manifest is changed.

No package manifest or lockfile is changed.

No dependency is added.

No runtime handler is added.

No deploy artifact is touched.

## Clarified 41D2 Boundary

Future Phase 41D2 must derive current-instruction identity from reviewed runtime context:

- direct entrypoint `program_id`
- direct entrypoint `instruction_data`
- expected program id
- expected instruction discriminator
- expected payload/context binding checks

Future Phase 41D2 must not use:

- `load_instruction`
- `load_instruction_at`
- `load_instruction_at_checked`
- full current-instruction loading from Instructions sysvar
- prior-instruction enumeration
- Phase 41C3 descriptor construction

## Expected Validation

- `git diff --check`
- `npm run typecheck`
- `npm run build`

Cargo validation is not required because no Rust source file is changed.

No SBF build should be run.

## Review Gate

Phase 41D2.0 must be reviewed before Phase 41D2 code.

Phase 41D2 must not start before this clarification is accepted.
