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
- Stage 1 Ed25519 guardian signature verifier helpers
- Stage 1 gateway approval verifier
- Stage 1 guardian quorum model
- Stage 1 processed burn registry model
- Stage 1 mint authorization model
- Stage 1 mint core model
- Stage 1 gateway state model
- Stage 1 gateway end-to-end scenario
- Stage 1 gateway negative end-to-end matrix
- Stage 1 gateway baseline checkpoint
- Stage 1.5 runtime mapping notes
- Stage 1.6 guardian set management design
- Stage 1.7 X1 account/storage layout design
- Stage 1.8 X1 runtime assumptions checkpoint
- Stage 1.9 Stage 2 planning readiness checkpoint
- Stage 2.0 gateway runtime planning outline
- Stage 2.1 runtime assumption dependency table
- Stage 2.2 direct mint candidate runtime design
- Stage 2.3 claim-based candidate runtime design
- Stage 2.4 direct mint vs claim-based architecture comparison
- Stage 2.5 gateway risk review
- Stage 2.6 X1 runtime evidence collection plan
- Stage 2.7 prototype-only experiment boundaries
- Stage 2.8 gateway planning baseline checkpoint
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

- [Stage 1.5 runtime mapping notes](docs/gateway/stage-1-5-runtime-mapping-notes.md)
- [Stage 1.6 guardian set management design](docs/gateway/stage-1-6-guardian-set-management-design.md)
- [Stage 1.7 X1 account/storage layout design](docs/gateway/stage-1-7-x1-account-storage-layout-design.md)
- [Stage 1.8 X1 runtime assumptions checkpoint](docs/gateway/stage-1-8-x1-runtime-assumptions-checkpoint.md)
- [Stage 1.9 Stage 2 planning readiness checkpoint](docs/gateway/stage-1-9-stage-2-planning-readiness.md)
- [Stage 1.10 X1 program instruction and PDA derivation design](docs/gateway/stage-1-10-x1-program-instruction-and-pda-derivation-design.md)
- [Stage 1.10 Theo review refinements](docs/gateway/stage-1-10-theo-review-refinements.md)
- [Stage 2 direct mint prototype start](docs/gateway/stage-2-direct-mint-prototype-start.md)
- [Stage 2 direct mint gateway skeleton testnet evidence](docs/gateway/evidence/stage-2-direct-mint-skeleton-testnet-evidence.md)
- [Stage 2 guardian signature verification compile evidence](docs/gateway/evidence/stage-2-guardian-signature-verification-compile-evidence.md)
- [Stage 2 guardian signature parser reference tests](docs/gateway/evidence/stage-2-guardian-signature-parser-reference-tests.md)
- [Stage 2 Theo guardian signature refinements](docs/gateway/evidence/stage-2-theo-guardian-signature-refinements.md)
- [Stage 2.4 Message hash binding design](docs/gateway/stage-2-4-message-hash-binding-design.md)
- [Stage 2.4 runtime message hash binding evidence](docs/gateway/evidence/stage-2-4-runtime-message-hash-binding-evidence.md)
- [Stage 2.4 runtime binding X1 testnet evidence](docs/gateway/evidence/stage-2-4-runtime-binding-testnet-evidence.md)
- [Stage 2.5 Token mint CPI planning](docs/gateway/stage-2-5-token-mint-cpi-planning.md)
- [Stage 2.5 SPL Token decision](docs/gateway/stage-2-5-spl-token-decision.md)
- [Stage 2.5 XXXL mint creation decision](docs/gateway/stage-2-5-xxxl-mint-creation-decision.md)
- [Stage 2.5 Mint authority PDA decision](docs/gateway/stage-2-5-mint-authority-pda-decision.md)
- [Stage 2.5 Recipient token account policy](docs/gateway/stage-2-5-recipient-token-account-policy.md)
- [Stage 2.5 Compute budget strategy](docs/gateway/stage-2-5-compute-budget-strategy.md)
- [Stage 2.5 Deployment prerequisites](docs/gateway/stage-2-5-deployment-prerequisites.md)
- [Stage 2.5 Token mint CPI testnet evidence](docs/gateway/evidence/stage-2-5-token-mint-cpi-testnet-evidence.md)
- [Stage 2.6 CPI failure rollback matrix evidence](docs/gateway/evidence/stage-2-6-cpi-failure-rollback-matrix-evidence.md)
- [Stage 2.7 Runtime account hygiene evidence](docs/gateway/evidence/stage-2-7-runtime-account-hygiene-evidence.md)
- [Stage 2.8 Relayer transaction shape](docs/gateway/stage-2-8-relayer-transaction-shape.md)
- [Stage 2.9 TypeScript relayer prototype evidence](docs/gateway/evidence/stage-2-9-typescript-relayer-prototype-evidence.md)
- [Stage 2.10 Relayer idempotency / retry evidence](docs/gateway/evidence/stage-2-10-relayer-idempotency-retry-evidence.md)
- [Stage 2.11 Ambiguous confirmation recovery evidence](docs/gateway/evidence/stage-2-11-ambiguous-confirmation-recovery-evidence.md)
- [Stage 2.12 Inconsistent recovery state evidence](docs/gateway/evidence/stage-2-12-inconsistent-recovery-state-evidence.md)
- [Stage 2.13 Relayer operational state machine evidence](docs/gateway/evidence/stage-2-13-relayer-operational-state-machine-evidence.md)
- [Stage 2.14 Relayer input preflight guard evidence](docs/gateway/evidence/stage-2-14-relayer-input-preflight-guard-evidence.md)
- [Stage 2.15 Preflight integrated submit path evidence](docs/gateway/evidence/stage-2-15-preflight-integrated-submit-path-evidence.md)
- [Stage 2.16 Relayer task normalization evidence](docs/gateway/evidence/stage-2-16-relayer-task-normalization-evidence.md)
- [Stage 2.17 Normalized task submit wrapper evidence](docs/gateway/evidence/stage-2-17-normalized-task-submit-wrapper-evidence.md)
- [Stage 2.18 Watcher event normalized task adapter evidence](docs/gateway/evidence/stage-2-18-watcher-event-normalized-task-adapter-evidence.md)
- [Stage 2.19 Watcher event full submit pipeline evidence](docs/gateway/evidence/stage-2-19-watcher-event-full-submit-pipeline-evidence.md)
- [Stage 2.20 Watcher event submit idempotency retry evidence](docs/gateway/evidence/stage-2-20-watcher-event-submit-idempotency-retry-evidence.md)
- [Stage 2.0 Gateway runtime planning outline](docs/gateway/stage-2-0-gateway-runtime-planning-outline.md)
- [Stage 2.1 Runtime assumption dependency table](docs/gateway/stage-2-1-runtime-assumption-dependency-table.md)
- [Stage 2.2 Direct mint candidate runtime design](docs/gateway/stage-2-2-direct-mint-candidate-runtime-design.md)
- [Stage 2.3 Claim-based candidate runtime design](docs/gateway/stage-2-3-claim-based-candidate-runtime-design.md)
- [Stage 2.4 Direct mint vs claim-based architecture comparison](docs/gateway/stage-2-4-direct-vs-claim-architecture-comparison.md)
- [Stage 2.5 Gateway risk review](docs/gateway/stage-2-5-gateway-risk-review.md)
- [Stage 2.6 X1 runtime evidence collection plan](docs/gateway/stage-2-6-x1-runtime-evidence-plan.md)
- [Stage 2.7 Prototype-only experiment boundaries](docs/gateway/stage-2-7-prototype-only-experiment-boundaries.md)
- [Stage 2.8 Gateway planning baseline checkpoint](docs/gateway/stage-2-8-gateway-planning-baseline-checkpoint.md)
- [EV-01 / EV-02 Atomic rollback prototype evidence plan](docs/gateway/evidence/ev-01-ev-02-atomic-rollback-prototype.md)
- [EV-01 / EV-02 X1 testnet atomic rollback evidence](docs/gateway/evidence/ev-01-ev-02-x1-testnet-atomic-rollback-evidence.md)
The XNTD-to-XXXL burn-to-mint gateway is documented as a design-only future layer. Stage 1 gateway architecture, implementation planning, and Ethereum burn event schema are also documented, but the gateway is not implemented, deployed, or approved for production by the current repository.

The X1-native Forge / Stake dual nominal model is also documented as a design-only future layer. It is not implemented, deployed, or approved for production by the current repository.

The next recommended step is the first prototype-only evidence branch for EV-01 transaction-level atomicity and EV-02 account write rollback, or collecting official X1 runtime documentation if available.
