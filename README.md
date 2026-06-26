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
- [X1 Build v1 runtime boundary](docs/build/build-v1-x1-runtime-boundary.md)
- [XXXL Program v1 design boundary](docs/xxxl/xxxl-program-v1-design-boundary.md)
- [XXXL Stage 1 gateway authorization consumer](docs/xxxl/xxxl-stage-1-gateway-authorization-consumer.md)
- [XXXL Genesis supply invariant](docs/xxxl/xxxl-genesis-supply-invariant.md)
- [XXXL Program v1 X1 runtime mapping](docs/xxxl/xxxl-program-v1-x1-runtime-mapping.md)
- [XXXL xDex listing plan](docs/xxxl/xxxl-xdex-listing-plan.md)
- [XXXL Program v1 review summary](docs/xxxl/xxxl-program-v1-review-summary.md)
- [XXXL Genesis Phase public explanation](docs/xxxl/xxxl-genesis-phase-public-explanation.md)
- [XXXL Program v1 review request](docs/xxxl/xxxl-program-v1-review-request.md)
- [XXXL Program v1 Theo review refinements](docs/xxxl/xxxl-program-v1-theo-review-refinements.md)
- [XXXL runtime candidate account and instruction schema](docs/xxxl/xxxl-runtime-candidate-account-instruction-schema.md)
- [XXXL runtime candidate transition semantics](docs/xxxl/xxxl-runtime-candidate-transition-semantics.md)
- [XXXL runtime route, guardian, and finality policy](docs/xxxl/xxxl-runtime-route-guardian-finality-policy.md)
- [XXXL incident response and emergency freeze policy](docs/xxxl/xxxl-incident-response-emergency-freeze-policy.md)
- [XXXL deployment dry-run model](docs/xxxl/xxxl-deployment-dry-run-model.md)
- [XXXL authority freeze procedure model](docs/xxxl/xxxl-authority-freeze-procedure-model.md)
- [XXXL Program v1 production readiness review summary v2](docs/xxxl/xxxl-program-v1-production-readiness-review-v2.md)
- [XXXL Program v1 Theo approval and runtime gap notes](docs/xxxl/xxxl-program-v1-theo-approval-runtime-gap-notes.md)
- [XXXL runtime serialization boundary](docs/xxxl/xxxl-runtime-serialization-boundary.md)
- [XXXL runtime account serialization vectors](docs/xxxl/xxxl-runtime-account-serialization-vectors.md)
- [XXXL Program v1 deployment readiness](docs/xxxl/xxxl-program-v1-deployment-readiness.md)

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
- Tests: 76 files / 538 tests passing
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
- Build v1 X1 runtime boundary design
- XXXL Program v1 design boundary
- XXXL Stage 1 gateway authorization consumer
- XXXL Genesis supply invariant hardening
- XXXL Program v1 X1 runtime mapping
- XXXL xDex listing plan
- XXXL Program v1 review summary
- XXXL Genesis Phase public explanation
- XXXL Program v1 review request
- XXXL Program v1 Theo review refinements
- XXXL runtime candidate account and instruction schema
- XXXL runtime candidate transition semantics
- XXXL runtime route, guardian, and finality policy
- XXXL incident response and emergency freeze policy
- XXXL deployment dry-run model
- XXXL authority freeze procedure model
- XXXL Program v1 production readiness review summary v2
- XXXL Program v1 Theo approval and runtime gap notes
- XXXL runtime serialization boundary
- XXXL runtime account serialization vectors
- XXXL Program v1 deployment readiness

## Validation commands

Run:

    npm run typecheck
    npm test
    npm run build
    npm audit --audit-level=moderate

Expected current result:

    76 test files passed
    538 tests passed
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

The current Build model stores durable public contribution history, not live spendable balances.

Build State distinguishes:

- `history_bld` — historical BLD from verified redeemed Core history; non-decreasing
- `origin_bld` — Genesis Origin BLD tier cap reached by history; not earned history
- `history_xbp` — historical XEN Burn Power from verified global XEN burns; non-decreasing
- stable XNTD commitment facts: `locked_xntd`, `required_xntd_lock`, `lock_epoch`, `xntd_commitment_accepted`
- X1 fee contribution checkpoint facts
- replay protection state

Build State does not store public spendable balances such as `available_bld` or `available_xbp`.

Spendable / transferable BLD is a separate token. Build State and Build view do not display, mirror, or cache BLD token balance. Relock must not depend on a public `Build.available_bld` field because that field does not exist.

Build Identity is separate from protocol accounting:

- `buildName`
- `logoUri`
- `metadataUpdatedAt`

Build Identity is owner-controlled display metadata and has no effect on BLD, XBP, XNTD lock, Genesis Origin, fee contribution, or replay protection.

The current app/service view helper is:

    appGetBuildView()

It returns:

    AppBuildView {
      build
      commitmentStatus
    }

`commitmentStatus` is derived from accepted XNTD commitment facts only. It does not mean Build validity and does not erase historical contribution.

Public Build commitment status does not expose `UNKNOWN`. Missing live external context should be handled by operation-level validation or infrastructure errors, not as public Build state.

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
- [Stage 2.21 Watcher event ambiguous recovery evidence](docs/gateway/evidence/stage-2-21-watcher-event-ambiguous-recovery-evidence.md)
- [Stage 2.22 Watcher event operational submit wrapper evidence](docs/gateway/evidence/stage-2-22-watcher-event-operational-submit-wrapper-evidence.md)
- [Stage 2.23 Watcher event batch queue processing evidence](docs/gateway/evidence/stage-2-23-watcher-event-batch-queue-processing-evidence.md)
- [Stage 2.24 Durable relayer journal model evidence](docs/gateway/evidence/stage-2-24-durable-relayer-journal-model-evidence.md)
- [Stage 2.25 Watcher-to-relayer contract boundary evidence](docs/gateway/evidence/stage-2-25-watcher-to-relayer-contract-boundary-evidence.md)
- [Stage 2.26 Relayer dedupe journal replay guard evidence](docs/gateway/evidence/stage-2-26-relayer-dedupe-journal-replay-guard-evidence.md)
- [Stage 2.27 Relayer import pipeline evidence](docs/gateway/evidence/stage-2-27-relayer-import-pipeline-evidence.md)
- [Stage 2.28 Import pipeline durable resume plan evidence](docs/gateway/evidence/stage-2-28-import-pipeline-durable-resume-plan-evidence.md)
- [Stage 2.29 Resume plan execution model evidence](docs/gateway/evidence/stage-2-29-resume-plan-execution-model-evidence.md)
- [Stage 2.30 Relayer operator report run summary evidence](docs/gateway/evidence/stage-2-30-relayer-operator-report-run-summary-evidence.md)
- [Stage 2.31 Operator report serialization log artifact evidence](docs/gateway/evidence/stage-2-31-operator-report-serialization-log-artifact-evidence.md)
- [Stage 2.32 Operator report audit log append model evidence](docs/gateway/evidence/stage-2-32-operator-report-audit-log-append-model-evidence.md)
- [Stage 2.33 Audit log integrity digest model evidence](docs/gateway/evidence/stage-2-33-audit-log-integrity-digest-model-evidence.md)
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

## Gateway Build activation boundary

- [Gateway full-profile Build activation boundary](docs/gateway/build-full-profile-activation-boundary.md)
