# XXXL X1 Testnet Local Runtime Skeleton Phase 41C0A Clarify 41C1 Sysvar Access Boundary

Status: Docs-only clarification.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41c0a-clarify-41c1-sysvar-access-boundary`

## Purpose

Phase 41C0A records the review decision that Phase 41C1 must not call
`load_instruction`, `load_instruction_at`, or equivalent helpers that read a
specific transaction instruction.

Phase 41C1 is limited to concrete API selection and Instructions sysvar
presence/readability.

## Files Added Or Changed

Added:

- `docs/xxxl/xxxl-phase-41c0a-clarify-41c1-sysvar-access-boundary.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41c0a-clarify-41c1-sysvar-access-boundary.md`
- `docs/reviews/xxxl-phase-41c0a-clarify-41c1-sysvar-access-boundary-review-request.md`

Changed:

- `docs/checkpoints/current-design-checkpoint.md`

No Rust source file is changed.

No TypeScript source file is changed.

No Cargo manifest is changed.

No package manifest or lockfile is changed.

No dependency is added.

No runtime handler is added.

No deploy artifact is touched.

## Clarified 41C1 Boundary

Allowed in Phase 41C1:

- concrete runtime API/helper selection
- Instructions sysvar presence check
- Instructions sysvar readability check
- deterministic result:

~~~text
MissingInstructionsSysvar
UnreadableInstructionsSysvar
PresentAndReadable
~~~

Forbidden in Phase 41C1:

- `load_instruction`
- `load_instruction_at`
- `load_instruction_at_checked`
- reading concrete transaction instruction contents
- current instruction identity derivation
- prior Ed25519 lookup
- proof
- quorum
- authorization
- replay
- CPI
- mint execution

## Active Blockers Preserved

Current X1 status remains:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`

Active blockers remain:

- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker was removed.

No production readiness is claimed.

No final immutability is claimed while upgrade authority exists.

## Validation

Suggested validation:

- `git diff --check`
- `npm run typecheck`
- `npm run build`

Cargo validation is not required because no Rust source file is changed.

No SBF build should be run.

## Review Gate

Phase 41C0A must be reviewed before Phase 41C1.

Phase 41C1 must not include `load_instruction` or concrete instruction content
reading.
