# XXXL Runtime Tooling Roadmap

Status: RUNTIME_TOOLING_ROADMAP_PLANNED.

This document fixes when each Rust/SVM security and quality tool enters the XXXL runtime workflow.

## 1. Current runtime layer checks

Timing: now and every runtime fixture stage.

Hard gates:

- `npm run typecheck`
- `npm test -- --reporter=dot`
- `npm run build`
- targeted Rust `cargo test` modules

Purpose: keep TypeScript fixtures, exported metadata, docs, and Rust runtime layers green while the runtime is still being assembled.

## 2. Rust quality/security baseline

Timing: after the atomic execution plan fixture.

Hard gates:

- `cargo fmt --check`
- `cargo test`
- `cargo audit`
- `cargo deny check`

Report-only:

- `cargo geiger`

Purpose: create the first Rust quality/security baseline without making unsafe statistics or known scaffold warnings block progress.

## 3. Rust clippy warning cleanup

Timing: after the Rust quality/security baseline.

Hard gate:

- `cargo clippy --all-targets --all-features -- -D warnings`

Purpose: resolve or explicitly isolate known `solana_program::entrypoint!` cfg warnings before making clippy a hard gate.

## 4. Manual account-constraint audit

Timing: before guarded live-handler wiring.

Checklist scope:

- account index
- owner
- signer flag
- writable flag
- rent exemption
- PDA seeds and bump
- SPL Mint authority
- recipient token account owner/mint/state
- processed-event replay status
- route config match
- guardian set match
- token program id
- mutation order
- CPI account order
- rollback assumptions

## 5. Mollusk instruction/state-transition tests

Timing: after guarded live-handler wiring model.

Target cases:

- valid consume_gateway_mint
- wrong owner
- wrong PDA
- wrong recipient token account
- replay
- overflow
- wrong route
- wrong guardian set
- wrong token program
- wrong account order

## 6. Trident fuzzing invariants

Timing: after Mollusk transition suite and invariant catalog.

Invariant examples:

- no double consume
- no balance overflow
- no credit without processed event
- no processed mark without valid event
- no mint amount mismatch
- no route mismatch acceptance
- no wrong recipient credit

## 7. Predeploy security readiness gate

Timing: before any real deploy or authority freeze.

Required gates:

- `npm run typecheck`
- `npm test -- --reporter=dot`
- `npm run build`
- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test`
- `cargo audit`
- `cargo deny check`
- `cargo geiger`
- Mollusk transition suite
- Trident fuzz suite
- manual account-constraint audit checklist
- manual authority/freeze checklist
- manual deployment config checklist

## Explicit decisions

- clippy `-D warnings` is not a hard gate until known scaffold warnings are cleaned up
- `cargo geiger` is report-only until manual unsafe review
- Mollusk starts after guarded handler wiring
- Trident starts after Mollusk and invariant catalog
- final predeploy gate combines all tools and manual checklists
