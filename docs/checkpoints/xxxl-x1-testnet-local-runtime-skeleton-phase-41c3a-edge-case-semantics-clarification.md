# XXXL X1 Testnet Local Runtime Skeleton Phase 41C3A Edge Case Semantics Clarification

Status: Narrow clarification.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41c3a-edge-case-semantics-clarification`

## Purpose

Phase 41C3A pins Phase 41C3 edge-case semantics after review.

It adds tests and documentation.

It does not change runtime logic.

It does not introduce real runtime wiring.

## Files Added Or Changed

Added:

- `docs/xxxl/xxxl-phase-41c3a-edge-case-semantics-clarification.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41c3a-edge-case-semantics-clarification.md`
- `docs/reviews/xxxl-phase-41c3a-edge-case-semantics-clarification-review-request.md`

Changed:

- `programs/xxxl-svm/src/verifier/prior_ed25519_lookup_ordering_boundary.rs`
- `docs/checkpoints/current-design-checkpoint.md`

No Cargo manifest is changed.

No package manifest or lockfile is changed.

No dependency is added.

No runtime handler is added.

No deploy artifact is touched.

## Clarified Semantics

Pinned by test:

- one valid strictly-prior match plus one same-index or later matching descriptor
  returns `PriorEd25519InstructionLocatedAndOrdered`

Clarified by docs:

- Phase 41C3 consumes candidate descriptors, not all real transaction
  instructions
- unrelated non-Ed25519 transaction instructions must not be forwarded into
  Phase 41C3 as candidate descriptors
- `WrongEd25519ProgramId` means an evidence-candidate descriptor has the wrong
  program id
- empty descriptor set means no evidence candidates and maps to
  `PriorEd25519InstructionNotFound`

## Expected Validation

- `git diff --check`
- `cargo fmt --check`
- `cargo test prior_ed25519_lookup_ordering_boundary --lib`
- `cargo test verifier --lib`
- `cargo test --lib --locked`
- `npm run typecheck`
- `npm run build`

No SBF build should be run.

## Review Gate

Phase 41C3A must be reviewed before any real runtime-wiring phase.

Real runtime wiring must remain separate.
