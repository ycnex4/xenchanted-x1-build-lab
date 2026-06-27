# Checkpoint: XXXL Runtime Tooling Roadmap

Stage: `stage-xxxl-runtime-tooling-roadmap`

Status: `RUNTIME_TOOLING_ROADMAP_PLANNED`

## Completed

- Planned current runtime layer checks.
- Planned Rust quality/security baseline.
- Planned clippy warning cleanup as a later hard gate.
- Planned manual account-constraint audit before guarded live-handler wiring.
- Planned Mollusk instruction/state-transition tests after guarded live-handler wiring.
- Planned Trident fuzzing after Mollusk and invariant catalog.
- Planned full predeploy security readiness gate.
- Added TypeScript roadmap fixture and validation tests.
- Updated README and current design checkpoint.

## Immediate sequence

1. Finish and merge this roadmap stage.
2. Add `stage-xxxl-rust-quality-security-baseline`.
3. Add `stage-xxxl-rust-clippy-warning-cleanup`.
4. Add `stage-xxxl-manual-account-constraint-audit-checklist`.
5. Continue toward guarded live-handler wiring.
6. Add Mollusk after guarded handler model.
7. Add Trident after Mollusk and invariant catalog.

## Explicit decisions

- no runtime logic change in this stage
- no tool installation in this stage
- no deployment
- no route activation
- no heavy fuzzing before handler model
- no clippy `-D warnings` hard gate before warning cleanup
