# xEnchanted X1 Build Lab

Implementation and design lab for the xEnchanted X1 Build model.

This repository models how verified xEnchanted Crypto / X1 activity can be converted into deterministic Build state through watcher candidates, proof objects, registrar handlers, replay protection, snapshots, and read-only CLI inspection.

This is not a production chain deployment yet.

It is a tested MVP implementation lab intended for architecture review, accounting review, and incremental hardening.

## Review entrypoint

Start here:

- [Review readiness summary](docs/review-readiness-summary.md)

Important supporting documents:

- [Final MVP readiness checkpoint](docs/final-mvp-readiness-checkpoint.md)
- [Assumptions](docs/assumptions.md)
- [Current design checkpoint](docs/checkpoints/current-design-checkpoint.md)

The review summary explains:

- project purpose
- current review target
- watcher -> proof -> registrar -> BuildState architecture
- Build state model
- BLD terminology
- Genesis Origin model
- XNTD lock / relock model
- commitmentStatus / appGetBuildView model
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
- Tests: 42 files / 328 tests passing
- Build: passing
- npm audit: 0 vulnerabilities

Latest reviewed main includes:

- final MVP readiness checkpoint
- commitmentStatus model
- appGetBuildView app/service view
- authoritative XC epoch minimum validation path
- XC Build validation context
- XNTD observed required lock propagation
- XNTD commitment event replay protection
- registrar mutation order hardening
- explicit MVP assumptions / known limitations
- CLI snapshot verification and recovery commands
- snapshot verification / backup / recovery helpers
- post-MVP deployment readiness notes
- read-only RPC smoke path review
- successful mainnet XC protocol params smoke via getProtocolParams()
- XNTD-to-XXXL burn-to-mint gateway design
- Stage 1 XXXL gateway architecture boundary
- Stage 1 XXXL gateway implementation plan
- Stage 1 Ethereum burn event schema
- Stage 1 gateway message schema
- Stage 1 gateway canonical encoding
- Stage 1 gateway test vectors
- Stage 1 gateway Theo review notes
- Stage 1 gateway pre-implementation blockers
- Stage 1 gateway hash, signature, and recipient decisions
- Stage 1 gateway mandatory source block fields
- Stage 1 X1 mint core immutability
- Stage 1 processed burn atomicity
- Stage 1 Ethereum finality rule
- Stage 1 recipient safety policy
- Stage 1 burn amount policy
- Stage 1 X1 deployment authority model
- Stage 1 exact cryptographic test vectors
- Stage 1 exact vectors Theo review notes
- Stage 1 generated gateway vectors
- Stage 1 gateway vector fixture tests
- Stage 1 gateway encoding helpers
- Stage 1 generator shared encoding helpers
- Stage 1 gateway verifier helpers
- X1-native Forge / Stake dual nominal model

## Validation commands

Run:

    npm run typecheck
    npm test
    npm run build
    npm audit --audit-level=moderate

Expected current result:

    42 test files passed
    328 tests passed
    found 0 vulnerabilities

## CLI

The CLI is intentionally minimal and read-only.

Current commands:

    npm run cli -- help
    npm run cli -- version
    npm run cli -- snapshot:show --file <path>
    npm run cli -- snapshot:verify --file <path>
    npm run cli -- snapshot:recover --file <path> [--backup <path>]

Read-only RPC smoke commands:

    npm run smoke:xc-epoch-minimum:rpc
    npm run smoke:xc-protocol-params:rpc

The protocol params smoke command has been confirmed against mainnet xEnchantedNFTLens through getProtocolParams().

The RPC smoke commands require local environment variables and must not print RPC URLs, API keys, private keys, mnemonics, seed phrases, or `.env` contents.

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
- source event replay protection
- XNTD commitment event replay protection
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
- authoritative XC epoch minimum source
- authoritative XC epoch minimum provider source
- XC protocol params source
- XC protocol params build validation
- XC Build validation context
- XC Build commitment status model
- app Build view exposing commitmentStatus

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
- commitmentStatus as current XNTD commitment signal

The current app/service view helper is:

    appGetBuildView()

It returns:

    AppBuildView {
      build
      commitmentStatus
    }

`commitmentStatus` does not mean Build validity and does not erase historical contribution.

`appGetBuildView()` may return `UNKNOWN` when context-dependent epoch information is required but unavailable. `UNKNOWN` is a status signal, not an error.

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

- [Assumptions](docs/assumptions.md)

Important current assumptions include:

- trusted indexer / registrar model
- Build ownership mapping assumption
- XNTD lock / relock source-event replay protection
- XNTD lock / relock monotonic lockEpoch ordering guard
- lock / relock are overwrite operations
- `requiredXntdLock` is accepted from registrar in the MVP
- XNTD epoch minimum validation design is documented separately
- authoritative XC state source design is documented separately
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

- [Review readiness summary](docs/review-readiness-summary.md)
- [Final MVP readiness checkpoint](docs/final-mvp-readiness-checkpoint.md)
- [Deployment readiness](docs/deployment-readiness.md)
- [Read-only RPC smoke review](docs/read-only-rpc-smoke-review.md)
- [XC protocol params RPC smoke result](docs/xc-protocol-params-rpc-smoke-result.md)
- [Assumptions](docs/assumptions.md)
- [Current design checkpoint](docs/checkpoints/current-design-checkpoint.md)

Gateway:

- [XNTD-to-XXXL burn-to-mint gateway design](docs/gateway/xntd-to-xxxl-burn-to-mint-gateway-design.md)
- [Stage 1 XXXL Gateway architecture](docs/gateway/stage-1-xxxl-gateway-architecture.md)
- [Stage 1 XXXL Gateway implementation plan](docs/gateway/stage-1-xxxl-gateway-implementation-plan.md)
- [Stage 1 Ethereum burn event schema](docs/gateway/stage-1-ethereum-burn-event-schema.md)

X1-native:

- [X1 Forge / Stake dual nominal model](docs/x1-native/x1-forge-stake-dual-nominal-model.md)

Build:

- [Terminology](docs/build/terminology.md)
- [Build v1 spec](docs/build/build-v1-spec.md)
- [Build state fields](docs/build/buildstate-fields.md)
- [State transitions](docs/build/state-transitions.md)
- [Program instruction layout](docs/build/program-instruction-layout.md)
- [PDA account layout](docs/build/pda-account-layout.md)
- [Program authority model](docs/build/program-authority-model.md)
- [Build reader interface](docs/build/build-reader-interface.md)

Registrar:

- [Ethereum registrar concept](docs/registrar/ethereum-registrar-concept.md)
- [Message format](docs/registrar/message-format.md)
- [Trust model evolution](docs/registrar/trust-model-evolution.md)

Indexers:

- [X1 fee contribution](docs/indexers/x1-fee-contribution.md)
- [XEN burn power indexing](docs/indexers/xen-burn-power-indexing.md)
- [MVP trusted indexer schemas](docs/indexers/mvp-trusted-indexer-schemas.md)

Economics:

- [BLD origin and native entry](docs/economics/bld-origin-and-native-entry.md)
- [BLD transfer and burn mechanics](docs/economics/bld-transfer-and-burn-mechanics.md)
- [XNTD lock and relock](docs/economics/xntd-lock-and-relock.md)
- [BLD tokenization decision](docs/economics/bld-tokenization-decision.md)
- [XNTD lock proof model](docs/economics/xntd-lock-proof-model.md)
- [Post-MVP BLD composability](docs/economics/post-mvp-bld-composability.md)

Implementation notes:

- `implementation/`

Tests:

- `tests/`

Source:

- `src/`

## Current review posture

The repository is ready for design, implementation, and post-MVP deployment-readiness review.

MVP implementation lab scope is complete.

A controlled read-only mainnet RPC smoke for XC protocol params has completed successfully.

The XNTD-to-XXXL burn-to-mint gateway is documented as a design-only future layer. Stage 1 gateway architecture, implementation planning, and Ethereum burn event schema are also documented, but the gateway is not implemented, deployed, or approved for production by the current repository.

The X1-native Forge / Stake dual nominal model is also documented as a design-only future layer. It is not implemented, deployed, or approved for production by the current repository.

The next recommended step is staging/runtime, gateway-risk review, and X1-native model review before adding new protocol features or Build actor scope.
