# xEnchanted X1 Build Lab

Private design repository for X1 Build architecture, Ethereum registrar logic, X1 fee contribution tracking, and related documentation.

This repository is currently documentation-only.

## Current focus

- X1 Build v1 specification
- BuildState fields
- State transitions
- Ethereum registrar model
- Global XEN Burn Power indexing
- BLD accounting
- Genesis Origin BLD allocation
- XNTD lock / relock logic
- X1 Fee Contribution tracking
- Source event replay protection

## Documents

### Build

- docs/build/terminology.md
- docs/build/build-v1-spec.md
- docs/build/buildstate-fields.md
- docs/build/state-transitions.md
- docs/build/program-instruction-layout.md
- docs/build/pda-account-layout.md
- docs/build/program-authority-model.md
- docs/build/build-reader-interface.md

### Registrar

- docs/registrar/ethereum-registrar-concept.md
- docs/registrar/message-format.md
- docs/registrar/trust-model-evolution.md

### Indexers

- docs/indexers/x1-fee-contribution.md
- docs/indexers/xen-burn-power-indexing.md
- docs/indexers/mvp-trusted-indexer-schemas.md

### Economics

- docs/economics/bld-origin-and-native-entry.md
- docs/economics/bld-transfer-and-burn-mechanics.md
- docs/economics/xntd-lock-and-relock.md
- docs/economics/bld-tokenization-decision.md
- docs/economics/xntd-lock-proof-model.md

### Checkpoints

- docs/checkpoints/current-design-checkpoint.md
- docs/checkpoints/mvp-implementation-sequence.md
- docs/checkpoints/documentation-consistency-review.md

## Current status

The current stage is design and documentation.

No implementation code should be started until the core Build spec, state fields, state transitions, registrar model, and indexer models are reviewed.














