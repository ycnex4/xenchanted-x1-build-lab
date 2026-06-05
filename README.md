# xEnchanted X1 Build Lab

Implementation and design lab for the xEnchanted X1 Build model.

This repository models how verified xEnchanted Crypto / X1 activity can be converted into deterministic Build state through watcher candidates, proof objects, registrar handlers, replay protection, snapshots, and read-only CLI inspection.

This is not a production chain deployment yet.

It is a tested MVP implementation lab intended for architecture review, accounting review, and incremental hardening.

## Review entrypoint

Start here:

- `docs/review-readiness-summary.md`

Important supporting documents:

- `docs/assumptions.md`
- `docs/checkpoints/current-design-checkpoint.md`

The review summary explains:

- project purpose
- current review target
- watcher -> proof -> registrar -> BuildState architecture
- Build state model
- BLD terminology
- Genesis Origin model
- XNTD lock / relock model
- source event protection
- implemented layers
- snapshot safety model
- CLI safety model
- validation status
- non-goals
- review questions

The assumptions document explicitly records MVP trust boundaries and known limitations.

## Current status

Current main baseline:

- TypeScript typecheck: passing
- Tests: 28 files / 171 tests passing
- Build: passing
- npm audit: 0 vulnerabilities

Latest reviewed main includes:

- registrar mutation order hardening
- explicit MVP assumptions / known limitations
- review readiness summary
- CLI snapshot verification and recovery commands
- snapshot verification / backup / recovery helpers
- Vitest 4 upgrade with `dist/**` excluded from test discovery

## Validation commands

Run:

    npm run typecheck
    npm test
    npm run build
    npm audit --audit-level=moderate

Expected current result:

    28 test files passed
    171 tests passed
    found 0 vulnerabilities

## CLI

The CLI is intentionally minimal and read-only.

Current commands:

    npm run cli -- help
    npm run cli -- version
    npm run cli -- snapshot:show --file <path>
    npm run cli -- snapshot:verify --file <path>
    npm run cli -- snapshot:recover --file <path> [--backup <path>]

CLI boundaries:

- no protocol state mutation
- no snapshot mutation
- no automatic restore
- no migration
- no deletion of corrupted files

The CLI only reads, validates, summarizes, and reports recovery source.

## Implemented MVP layers

The current repository includes tested layers for:

- Build state
- Build registry
- create build flow
- Core redeem BLD accounting
- XEN Burn Power accounting
- XNTD lock and relock
- X1 fee contribution checkpoints
- registrar replay protection
- registrar handlers
- proof object types
- proof-to-registrar builders
- watcher candidate types
- watcher-to-proof conversion
- application proof submission
- end-to-end watcher-proof-registrar scenario
- storage serialization
- snapshot verification
- snapshot backup-enabled save
- snapshot recovery load
- CLI command layer
- CLI binary entry point
- CLI snapshot show
- CLI snapshot verify
- CLI snapshot recover

## Architecture overview

High-level flow:

    Watcher candidate
      -> Proof object
      -> Application proof submission
      -> Registrar handler
      -> BuildState update
      -> Replay protection update
      -> Snapshot persistence / CLI inspection

The project separates:

- domain state types
- application service layer
- registrar handlers
- proof object builders
- watcher candidate models
- watcher-to-proof conversion
- snapshot serialization / verification / recovery
- CLI read-only commands

## Core accounting concepts

The current Build model distinguishes:

- `history_bld` — historical BLD, non-decreasing
- `available_bld` — usable / spendable / transferable BLD
- `origin_bld` — Genesis allocation, not history

The model also tracks:

- XEN Burn Power
- XNTD lock / relock state
- X1 fee contribution checkpoints
- replay protection state

## Snapshot safety

Snapshot helpers are intentionally conservative.

The implemented model supports:

- deterministic snapshot serialization
- snapshot verification through the same decode / deserialize path
- backup-enabled save with verification
- recovery load from canonical or backup snapshot
- read-only recovery reporting

Snapshot recovery does not:

- repair canonical snapshots
- copy backup into canonical
- delete corrupted files
- migrate snapshot files
- create new backup files

## MVP assumptions and limitations

See:

- `docs/assumptions.md`

Important current assumptions include:

- trusted indexer / registrar model
- Build ownership mapping assumption
- XNTD lock / relock source-event replay protection
- XNTD lock / relock monotonic lockEpoch ordering guard
- lock / relock are overwrite operations
- `requiredXntdLock` is accepted from registrar in the MVP
- no unlock flow in the MVP
- canonicalEventKey convention
- fee checkpoint finality assumption
- snapshot recovery is read-only
- no production integration guarantees yet

## Non-goals in the current MVP

The current MVP intentionally does not include:

- production chain deployment
- live RPC integration
- real watcher service runtime
- bridge execution
- token issuance logic
- UI
- operator restore tooling
- trustless proof verification

These are future layers and should not be assumed to exist in the current MVP.

## Document map

Review / checkpoint:

- `docs/review-readiness-summary.md`
- `docs/assumptions.md`
- `docs/checkpoints/current-design-checkpoint.md`

Build:

- `docs/build/terminology.md`
- `docs/build/build-v1-spec.md`
- `docs/build/buildstate-fields.md`
- `docs/build/state-transitions.md`
- `docs/build/program-instruction-layout.md`
- `docs/build/pda-account-layout.md`
- `docs/build/program-authority-model.md`
- `docs/build/build-reader-interface.md`

Registrar:

- `docs/registrar/ethereum-registrar-concept.md`
- `docs/registrar/message-format.md`
- `docs/registrar/trust-model-evolution.md`

Indexers:

- `docs/indexers/x1-fee-contribution.md`
- `docs/indexers/xen-burn-power-indexing.md`
- `docs/indexers/mvp-trusted-indexer-schemas.md`

Economics:

- `docs/economics/bld-origin-and-native-entry.md`
- `docs/economics/bld-transfer-and-burn-mechanics.md`
- `docs/economics/xntd-lock-and-relock.md`
- `docs/economics/bld-tokenization-decision.md`
- `docs/economics/xntd-lock-proof-model.md`
- `docs/economics/post-mvp-bld-composability.md`

Implementation notes:

- `implementation/`

Tests:

- `tests/`

Source:

- `src/`

## Current review posture

The repository is ready for design and implementation review.

The next recommended step is review-driven planning before adding new integration complexity.
