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
- [XXXL runtime instruction serialization vectors](docs/xxxl/xxxl-runtime-instruction-serialization-vectors.md)
- [XXXL multichain low-weight route policy](docs/xxxl/xxxl-multichain-low-weight-route-policy.md)
- [XXXL runtime program skeleton](docs/xxxl/xxxl-runtime-program-skeleton.md)
- [XXXL runtime execution vectors](docs/xxxl/xxxl-runtime-execution-vectors.md)
- [XXXL runtime dry-run fixtures](docs/xxxl/xxxl-runtime-dry-run-fixtures.md)
- [XXXL runtime fixture report export](docs/xxxl/xxxl-runtime-fixture-report-export.md)
- [XXXL runtime predeploy review package](docs/xxxl/xxxl-runtime-predeploy-review-package.md)
- [XXXL production runtime byte layout](docs/xxxl/xxxl-production-runtime-byte-layout.md)
- [XXXL X1/SVM program skeleton](docs/xxxl/xxxl-x1-svm-program-skeleton.md)
- [XXXL SVM serialized runtime vectors](docs/xxxl/xxxl-svm-serialized-runtime-vectors.md)
- [XXXL SVM runtime decoder handler model](docs/xxxl/xxxl-svm-runtime-decoder-handler-model.md)
- [XXXL SVM runtime port readiness package](docs/xxxl/xxxl-svm-runtime-port-readiness-package.md)
- [XXXL X1/SVM port scaffold](docs/xxxl/xxxl-x1-svm-port-scaffold.md)
- [XXXL real PDA derivation fixture](docs/xxxl/xxxl-real-pda-derivation-fixture.md)
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
- Tests: 90 files / 761 tests passing
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
- XXXL runtime instruction serialization vectors
- XXXL multichain low-weight route policy
- XXXL runtime program skeleton
- XXXL runtime execution vectors
- XXXL runtime dry-run fixtures
- XXXL runtime fixture report export
- XXXL runtime predeploy review package
- XXXL production runtime byte layout
- XXXL X1/SVM program skeleton
- XXXL SVM serialized runtime vectors
- XXXL SVM runtime decoder handler model
- XXXL SVM runtime port readiness package
- XXXL X1/SVM port scaffold
- XXXL real PDA derivation fixture
- XXXL Program v1 deployment readiness

## Validation commands

Run:

    npm run typecheck
    npm test
    npm run build
    npm audit --audit-level=moderate

Expected current result:

    90 test files passed
    761 tests passed
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

## XXXL runtime account/instruction decode fixture

The XXXL X1/SVM port now has a Rust decode fixture stage for the runtime account and instruction byte boundary.

This stage fixes:

- real `consume_gateway_mint` instruction discriminator, version, length, account meta count, account indexes, and parsed fields
- real account view checks for length, discriminator, and version
- negative Rust tests for wrong instruction length, wrong instruction discriminator, wrong instruction version, wrong account discriminator, wrong account version, and truncated account data
- TypeScript checkpoint metadata for the decode fixture

This stage does not add SPL Token CPI, deployment, route activation, or authority freeze execution.

## XXXL SPL Token mint_to CPI fixture

The XXXL X1/SVM port now has a native SPL Token `mint_to` CPI fixture boundary.

This stage fixes:

- real `spl_token::instruction::mint_to` instruction construction
- PDA signer seeds for `invoke_signed`
- gateway mint authority PDA/bump verification
- initialized SPL Mint validation
- recipient token account validation against expected owner and mint
- owner and rent helper coverage

This stage does not activate live routes, deployment, authority freeze execution, processed-event mutation, or recipient-balance mutation.

## XXXL handler integration fixture

The XXXL X1/SVM port now has a handler integration fixture that connects decoded `consume_gateway_mint` instruction data, canonical account indexes, runtime account views, owner/rent checks, SPL Mint validation, recipient token account validation, gateway PDA/bump verification, and CPI boundary preparation.

This stage is still not live route execution:

- `process_instruction` does not call `mint_to_cpi_boundary`
- no route activation
- no processed-event mutation
- no recipient-balance mutation
- no deployment
- no authority freeze execution

## XXXL runtime state mutation fixture

The XXXL X1/SVM port now has deterministic state mutation helpers for runtime account data:

- `mark_processed_event_consumed` marks a processed event as consumed and writes consumed amount / consumed slot
- `credit_recipient_balance` credits recipient balance with overflow protection and writes last canonical event key

This stage remains fixture-only:

- no live route activation
- no mint_to invocation from handler
- no process_instruction state mutation
- no deployment
- no authority freeze execution

## XXXL atomic execution plan fixture

The XXXL X1/SVM port now has an atomic execution-plan fixture.

The fixed order is:

1. validate and prepare CPI boundary
2. mark processed event consumed
3. credit recipient balance
4. keep live route disabled

The fixture prechecks replay, recipient balance owner/mint, prepared CPI amount, and recipient balance overflow before applying state mutations.

This stage remains plan-only:

- no live route activation
- no mint_to invocation from process_instruction
- no process_instruction processed-event mutation
- no process_instruction recipient-balance mutation
- no deployment
- no authority freeze execution

## XXXL runtime tooling roadmap

The XXXL runtime tooling roadmap is now explicit.

Planned order:

1. current runtime layer checks: TypeScript typecheck/tests/build plus targeted Rust tests
2. Rust quality/security baseline: `cargo fmt --check`, `cargo test`, `cargo audit`, `cargo deny check`, `cargo geiger` report-only, and manual account checklist draft
3. Rust clippy warning cleanup: `cargo clippy --all-targets --all-features -- -D warnings`
4. manual account-constraint audit before guarded live-handler wiring
5. Mollusk instruction/state-transition tests after guarded handler wiring
6. Trident fuzzing after Mollusk and invariant catalog
7. full predeploy security readiness gate before any deploy/freeze action

Important decisions:

- clippy `-D warnings` is not a hard gate until known scaffold warnings are cleaned up
- `cargo geiger` is report-only until manual unsafe review
- Mollusk and Trident are not introduced before handler/invariant structure is mature

## XXXL Rust quality/security baseline

The XXXL SVM program now has a Rust quality/security baseline stage.

Current baseline:

- `cargo fmt --check` is enforced after applying rustfmt
- `cargo test` passes for the Rust SVM package
- `cargo audit` identifies an unresolved Solana dependency-chain blocker:
  - `RUSTSEC-2024-0344`
  - `curve25519-dalek v3.2.1`
  - required fix: `>=4.1.3`
  - current blocker path: `solana-program v1.18.26`
- `cargo deny` is configured with `programs/xxxl-svm/deny.toml`
- `cargo deny` licenses/bans/sources are green
- `cargo deny` advisories intentionally surface the same unresolved Solana dependency-chain blocker
- `cargo geiger` remains report-only and currently has a reporting/tooling limitation in this environment

Policy:

- no fake audit ignore
- no Solana/SPL runtime dependency upgrade inside the baseline stage
- no `clippy -D warnings` hard gate until known entrypoint cfg warnings are handled

## XXXL Solana/SPL dependency upgrade audit

The XXXL SVM dependency audit stage selected the minimal upgrade path that closes the RustSec hard vulnerability in the Solana dependency chain.

Selected versions:

- `solana-program = 2.3.0`
- `spl-token = 5.0.2`
- resolved `curve25519-dalek = 4.1.3`

Matrix result:

- Solana 1.18 candidates kept the audit blocker
- Solana 2 + SPL Token 5 was the first candidate with tests passing and `cargo audit` passing
- Solana 2 + SPL Token 6 also passed but was not minimal
- Solana 3 candidates passed audit but failed current tests

Verification:

- Rust tests pass: 63 passed, 0 failed
- `cargo audit` exits 0
- `cargo deny` licenses/bans/sources exit 0

## XXXL Rust clippy warning cleanup

The XXXL SVM scaffold now has a strict Rust warning baseline.

Completed:

- fixed the local `clippy::needless_lifetimes` warning
- documented and scoped the Solana `entrypoint!` macro `unexpected_cfgs` exception
- confirmed `cargo clippy --all-targets -- -D warnings` passes
- confirmed Rust tests, audit, and deny checks still pass

The exception is limited to `unexpected_cfgs` emitted by Solana entrypoint macro expansion under host clippy/check-cfg.

No runtime behavior was changed in this stage.

## XXXL manual account constraint audit checklist

The XXXL SVM runtime now has a documented manual account-constraint checklist before live handler wiring.

Recorded boundaries:

- canonical 9-account consume_gateway_mint order
- program-owned owner and rent checks
- SPL Token program check
- initialized SPL mint and recipient token account checks
- gateway mint authority PDA and invoke_signed seed policy
- processed-event replay boundary
- recipient-balance boundary
- amount constraints
- atomicity and no-state-change-on-failure policy
- writable/executable constraints required before live activation

This stage does not activate the live gateway route and does not change runtime behavior.

## XXXL guarded live-handler wiring fixture

The XXXL SVM runtime now has a guarded live-handler wiring fixture.

Added:

- `LIVE_ROUTE_ACTIVATION_FROM_PROCESS_INSTRUCTION_ENABLED = false`
- `build_guarded_consume_gateway_mint_live_handler_fixture`
- positive fixture test for building a disabled execution plan after validation
- negative fixture test rejecting invalid processed-event boundary before plan creation

The fixture wires account validation into execution-plan construction, but keeps live route activation disabled.

Verification:

- Rust tests pass: 65 passed, 0 failed
- `cargo clippy --all-targets -- -D warnings` passes
- `cargo audit` exits 0
- `cargo deny` licenses/bans/sources exit 0

No live gateway route was activated in this stage.

## XXXL Mollusk readiness harness plan

A doc-only readiness plan has been added for a future Mollusk runtime harness.

The plan records:

- why Mollusk should come before live route mutation
- the canonical 9-account consume_gateway_mint harness shape
- required positive scaffold case
- invalid instruction cases
- account boundary failure cases
- PDA failure cases
- route and guardian boundary cases
- replay boundary cases
- recipient token account cases
- SPL mint cases
- future atomicity invariants
- current scaffold invariants
- suggested harness structure
- dependency policy for a future Mollusk stage

No runtime code was changed.

No live gateway route was activated.

## XXXL Mollusk dependency compatibility probe

A doc-only compatibility probe was completed for adding `mollusk-svm` as a future dev-dependency.

Result:

- `mollusk-svm = 0.13.4` can be added in a temporary repo copy
- temporary `cargo test` passes with 65 tests
- temporary `cargo clippy --all-targets -- -D warnings` passes
- temporary `cargo audit` exits 0
- temporary `cargo deny` licenses/bans/sources exits 0

Important dependency result:

- baseline cargo audit scan: 196 crates
- temporary Mollusk cargo audit scan: 404 crates
- allowed audit warnings increase from 3 to 6

No repository dependency was changed in this stage.

No runtime code was changed.

No live gateway route was activated.

## XXXL Mollusk dev-dependency

`mollusk-svm = "0.13.4"` has been added as a dev-dependency for future SVM runtime harness testing.

This stage intentionally adds only the dependency and lockfile changes.

Verification:

- `cargo fmt --check` passes
- `cargo test` passes with 65 tests
- `cargo clippy --all-targets -- -D warnings` passes
- `cargo audit` exits 0
- `cargo deny` licenses/bans/sources exits 0

Dependency footprint:

- cargo audit scans 404 crate dependencies after adding Mollusk
- allowed audit warnings increase from 3 to 6

No runtime code was changed.

No Mollusk harness was added yet.

No live gateway route was activated.

## XXXL first scaffold-only Mollusk harness

The first real Mollusk execution harness has been added for the XXXL SVM runtime.

The harness:

- builds the SBF program with `cargo build-sbf`
- loads `target/deploy/xxxl_svm.so` through Mollusk
- executes a valid `consume_gateway_mint` instruction
- verifies scaffold-only success
- verifies target state accounts remain unchanged

The integration test is ignored by default because it requires a local SBF artifact:

    cargo build-sbf
    cargo test --test mollusk_consume_gateway_mint -- --ignored --nocapture

Verified behavior:

- runtime log confirms `live route execution is not activated`
- processed event remains unchanged
- recipient balance remains unchanged
- SPL mint supply remains unchanged
- recipient token balance remains unchanged

No live gateway route was activated.
No SPL Token `mint_to` is invoked.
No XXXL minting is enabled.

## XXXL Mollusk instruction decode negative harness

The Mollusk SBF harness now includes negative instruction decode tests.

The ignored integration test file verifies four SBF-level cases:

- valid `consume_gateway_mint` scaffold success
- invalid instruction length
- invalid discriminator
- invalid layout version

The negative cases assert the expected custom errors:

- `InvalidInstruction` -> `0x1`
- `InvalidDiscriminator` -> `0x6`
- `InvalidVersion` -> `0x7`

The tests remain ignored by default because they require a local SBF artifact:

    cargo build-sbf
    cargo test --test mollusk_consume_gateway_mint -- --ignored --nocapture

No live gateway route was activated.
No SPL Token `mint_to` is invoked.
No XXXL minting is enabled.

## XXXL Mollusk guarded account validation preflight

The XXXL SVM program now connects guarded account validation to the real `process_instruction` path.

The runtime path now:

- decodes `consume_gateway_mint`
- reads Rent from the runtime sysvar
- prepares the guarded CPI boundary
- validates account count, owners, rent exemption, PDA, processed event, recipient balance, SPL mint, recipient token account, and amount bounds
- returns success only after preflight validation
- keeps live route execution disabled

The ignored Mollusk SBF harness now covers 9 cases:

- valid preflight success without state mutation
- invalid instruction length
- invalid discriminator
- invalid layout version
- wrong account count
- wrong program-owned account owner
- consumed processed event
- wrong recipient token owner
- zero amount

The valid SBF path emits:

    XXXL consume_gateway_mint preflight validated; live route execution is not activated

No live gateway route was activated.
No SPL Token `mint_to` is invoked.
No XXXL minting is enabled.
No runtime state mutation is enabled.

## XXXL runtime execution plan boundary

The XXXL SVM runtime now builds an execution plan after guarded account validation.

The real SBF path now reaches:

    decode -> guarded account validation -> execution plan -> stop

The execution plan records canonical event key, route id, recipient, mint, amount, consumed slot, source chain weight, fixed atomic step order, and disabled safety flags.

The valid Mollusk path emits:

    XXXL consume_gateway_mint execution plan built; live route execution is not activated

No live route was activated.
No SPL Token `mint_to` is invoked.
No XXXL minting is enabled.
No runtime state mutation is enabled.

## XXXL processed event mutation boundary

The XXXL runtime model now has a separately tested processed-event mutation boundary.

The new boundary accepts an execution plan and processed-event account data, validates the event identity and safety flags, then marks the event as consumed by writing consumed flag, amount, and slot.

This stage does not connect mutation to `process_instruction`.

No live route was activated.
No SPL Token `mint_to` is invoked.
No XXXL minting is enabled.
No recipient balance mutation is enabled.

## XXXL recipient balance mutation boundary

The XXXL runtime model now has a separately tested recipient-balance mutation boundary.

The new boundary accepts an execution plan and recipient-balance account data, validates the account identity and safety flags, then credits recipient balance and records the canonical event key.

This stage does not connect mutation to `process_instruction`.

No live route was activated.
No SPL Token `mint_to` is invoked.
No XXXL minting is enabled.
No processed event mutation is enabled by this boundary.

## XXXL atomic state mutation composition boundary

The XXXL runtime model now has a composed atomic state-mutation boundary.

The new boundary accepts an execution plan, processed-event account data, and recipient-balance account data. It prechecks both accounts before writing either mutation.

The key property is:

    if recipient balance validation fails, processed_event remains unchanged

This prevents partial local state such as an event being marked consumed without the recipient balance being credited.

This stage does not connect mutation to `process_instruction`.

No live route was activated.
No SPL Token `mint_to` is invoked.
No XXXL minting is enabled.
No SPL mint supply mutation is enabled.

## XXXL SPL mint_to CPI planning boundary

The XXXL runtime model now has a planning-only SPL Token `mint_to` CPI boundary.

The new boundary accepts an execution plan and prepared CPI boundary, validates token program, mint mapping, PDA, bump, signer seed layout, and amount, then returns a planning result.

This stage does not call `invoke_signed`.

No live route was activated.
No SPL Token `mint_to` is invoked.
No XXXL minting is enabled.
No state mutation is connected to `process_instruction`.

## XXXL runtime planning composition boundary

The XXXL runtime model now composes execution planning with SPL Token `mint_to` CPI planning.

The new boundary accepts accounts and decoded consume-gateway-mint args, performs guarded validation, builds an execution plan, then builds a planning-only `mint_to` CPI plan.

This stage does not call `invoke_signed`.

No live route was activated.
No SPL Token `mint_to` is invoked.
No XXXL minting is enabled.
No runtime state mutation is performed.
No live execution is connected to `process_instruction`.

## XXXL runtime local state mutation composition boundary

The XXXL runtime model now composes planning with local program-owned state mutation.

The new boundary accepts accounts and decoded consume-gateway-mint args, performs guarded validation, builds the execution plan, builds the planning-only SPL `mint_to` CPI plan, and then mutates only local program-owned state:

- processed event account
- recipient balance account

The mutation uses the atomic state mutation composition boundary, which prechecks both local state accounts before writing.

This stage does not call `invoke_signed`.

No live route was activated.
No SPL Token `mint_to` is invoked.
No XXXL minting is enabled.
No SPL mint supply mutation is enabled.
No recipient SPL token account mutation is enabled.
No live execution is connected to `process_instruction`.

## XXXL guarded SPL CPI execution gate boundary

The XXXL runtime model now has an explicit disabled gate before real SPL Token `mint_to` CPI execution.

The existing real CPI function remains present behind the gate, but the new guarded boundary revalidates the execution plan and CPI planning boundary, then returns `CpiBoundaryNotReady` while `spl_mint_to_cpi_execution_enabled()` is false.

This stage does not call `invoke_signed`.

No live route was activated.
No SPL Token `mint_to` is invoked.
No XXXL minting is enabled.
No SPL mint supply mutation is enabled.
No recipient SPL token account mutation is enabled.
No CPI execution is connected to `process_instruction`.

## XXXL runtime disabled SPL CPI gate integration boundary

The XXXL runtime model now has a boundary that composes runtime validation, execution planning, CPI planning, and the guarded SPL CPI execution gate.

The gate remains disabled. The boundary reaches the gate and returns `CpiBoundaryNotReady` before real SPL CPI.

This stage proves that the runtime can fail closed at the disabled CPI gate without mutating local processed-event state, recipient-balance state, SPL mint supply, or recipient token account state.

No live route was activated.
No `invoke_signed` is called.
No SPL Token `mint_to` is invoked.
No XXXL minting is enabled.
No CPI execution is connected to `process_instruction`.

## XXXL live route activation and bootstrap guardian policy

The project now has explicit policy documents for live XXXL route activation and bootstrap guardians.

Live route activation is blocked until the activation checklist is satisfied.

The bootstrap guardian model is allowed only as a temporary, disclosed, capped, auditable launch mode.

The correct description is `operator-controlled bootstrap guardian set`.

The project must not describe bootstrap guardians as a decentralized guardian network unless guardians are actually independently operated.

Every live XXXL mint must be burn-backed and publicly explainable by a proof bundle.

## XXXL runtime account contract manifest boundary

The XXXL runtime now has an explicit account contract manifest for `consume_gateway_mint`.

The manifest fixes the 9-account runtime shape:

0. mint_state
1. gateway_config
2. guardian_set
3. processed_event
4. recipient_balance
5. spl_token_mint
6. recipient_token_account
7. mint_authority_pda
8. token_program

It documents index, name, writable/readonly requirement, signer requirement, and owner model.

No live route was activated.
No SPL CPI behavior was changed.
No `process_instruction` behavior was changed.

## XXXL runtime account contract enforcement boundary

The XXXL runtime now enforces the `consume_gateway_mint` account contract manifest.

The runtime rejects:

- readonly accounts passed as writable
- writable accounts passed as readonly
- unexpected external signer accounts

The enforcement is integrated into `prepare_consume_gateway_mint_cpi_boundary`.

The Mollusk fixture was updated to match the manifest:

- `mint_state` readonly
- `gateway_config` readonly
- mutable state/SPL accounts remain writable

No live route was activated.
No SPL CPI behavior was enabled.
No `invoke_signed` path was enabled.
No minting was enabled.

## XXXL Mollusk account contract negative coverage

The XXXL runtime account contract enforcement now has SBF/Mollusk negative coverage.

Added ignored Mollusk tests for:

- readonly account passed as writable
- required writable account passed as readonly
- unexpected external signer

Each invalid account meta case is rejected with `InvalidInstruction`.

No live route was activated.
No SPL CPI behavior was enabled.
No `invoke_signed` path was enabled.
No minting was enabled.

## XXXL runtime nondeployable status boundary

The XXXL SVM runtime now has an explicit deployment status module.

Current status:

- `ScaffoldOnlyNotDeployable`

Current deployability:

- `false`

Explicit blockers:

- placeholder Program ID
- live route disabled
- SPL CPI execution disabled
- production guardian set unset
- production proof log unset
- external review incomplete

No live route was activated.
No SPL CPI behavior was enabled.
No `invoke_signed` path was enabled from `process_instruction`.
No minting was enabled.
No deployment behavior was enabled.

## XXXL runtime deployment blocker descriptions

The XXXL runtime deployment status and blockers now expose stable machine-readable codes and human-readable descriptions.

Current status code:

- `SCAFFOLD_ONLY_NOT_DEPLOYABLE`

Current blocker codes:

- `PLACEHOLDER_PROGRAM_ID`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

This is intended for future README rendering, CLI status output, predeploy scripts, UI panels, and deployment checklists.

No live route was activated.
No SPL CPI behavior was enabled.
No `invoke_signed` path was enabled.
No minting was enabled.
No deployability predicate was changed.

## XXXL runtime deployment status report boundary

The XXXL runtime now exposes a stable deployment status report object.

The report includes:

- deployment status
- status code
- status description
- deployable flag
- blocker reports

Current status code:

- `SCAFFOLD_ONLY_NOT_DEPLOYABLE`

Current deployability:

- `false`

Current blocker codes:

- `PLACEHOLDER_PROGRAM_ID`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

This report can later be consumed by predeploy checks, CLI status output, README/status rendering, deployment checklists, and UI status panels.

No live route was activated.
No SPL CPI behavior was enabled.
No `invoke_signed` path was enabled.
No minting was enabled.
No deployability predicate was changed.

## XXXL runtime deployment blocker resolution guidance

The XXXL runtime deployment blocker reports now include resolution guidance.

Each blocker now explains:

- what is blocking deployment
- why it matters
- what must happen before the blocker can be removed

Current blocker resolution themes:

- set/review the real Program ID and regenerate Program-ID-dependent PDA fixtures
- activate the live route only in a reviewed stage after blockers are resolved
- enable SPL Token mint_to CPI execution only after live route, PDA authority, account contract, and Mollusk coverage are complete
- define, publish, and review the production guardian set, threshold, rotation policy, and key custody model
- define the production proof-log format, retention policy, public audit trail, and operator publication flow
- complete external review of live route, guardian policy, CPI path, account contract, replay protection, and deployment checklist

No live route was activated.
No SPL CPI behavior was enabled.
No `invoke_signed` path was enabled.
No minting was enabled.
No deployability predicate was changed.

## XXXL runtime predeploy gate result boundary

The XXXL runtime now exposes a stable predeploy gate result.

Added:

- `XxxlRuntimeDeploymentGateResult`
- `xxxl_runtime_deployment_gate_result`
- `xxxl_runtime_predeploy_gate_allows_deploy`

Current gate result:

- `Blocked(report)`

Current deploy allow value:

- `false`

The gate allows deployment only when:

- `report.deployable == true`
- `report.blockers.is_empty() == true`

The current report remains blocked because deployability is false and blockers are still present.

No live route was activated.
No SPL CPI behavior was enabled.
No `invoke_signed` path was enabled.
No minting was enabled.
No deployment behavior was enabled.
No deployability predicate was changed.

## XXXL runtime predeploy readiness checklist

A predeploy readiness checklist has been added for the XXXL SVM runtime.

The checklist maps each current blocker to:

- meaning
- required resolution
- required evidence before removal

Covered blockers:

- `PLACEHOLDER_PROGRAM_ID`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

Current runtime status:

- `SCAFFOLD_ONLY_NOT_DEPLOYABLE`

Current gate result:

- `Blocked(report)`

Current deploy allow value:

- `false`

No live route was activated.
No SPL CPI behavior was enabled.
No `invoke_signed` path was enabled.
No minting was enabled.
No deployment behavior was enabled.
No deployability predicate was changed.

## XXXL runtime predeploy evidence matrix

A predeploy evidence matrix has been added for future XXXL runtime blocker-removal work.

The matrix maps each current blocker to:

- required evidence
- expected evidence artifact
- current status

Covered blockers:

- `PLACEHOLDER_PROGRAM_ID`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

All blockers remain:

- `BLOCKED`

Current gate result remains:

- `Blocked(report)`

Current deploy allow value remains:

- `false`

No live route was activated.
No SPL CPI behavior was enabled.
No `invoke_signed` path was enabled.
No minting was enabled.
No deployment behavior was enabled.
No deployability predicate was changed.

## XXXL Program ID and PDA fixture readiness plan

A readiness plan has been added for the `PLACEHOLDER_PROGRAM_ID` deployment blocker.

The plan defines:

- required future inputs
- PDA derivation inventory
- required evidence before blocker removal
- required tests for future blocker removal
- suggested future stage order
- interaction with other blockers

No real Program ID was selected.
No PDA fixtures were regenerated.
No deployment blocker was removed.

The `PLACEHOLDER_PROGRAM_ID` blocker remains:

- `BLOCKED`

No live route was activated.
No SPL CPI behavior was enabled.
No `invoke_signed` path was enabled.
No minting was enabled.
No deployment behavior was enabled.
No deployability predicate was changed.

## XXXL PDA derivation inventory boundary

An explicit PDA derivation inventory boundary has been added.

Added:

- `XxxlPdaDerivationKind`
- `XxxlPdaDerivationInventoryEntry`
- `GATEWAY_MINT_AUTHORITY_SEEDS`
- `XXXL_PDA_DERIVATION_INVENTORY`
- `xxxl_pda_derivation_inventory`
- `xxxl_pda_derivation_inventory_entry`

Current PDA entry:

- `gateway_mint_authority`

Current seeds:

- `xxxl`
- `gateway-mint-authority`
- `v1`

Current Program ID dependency:

- `true`

No real Program ID was selected.
No PDA fixtures were regenerated.
No deployment blocker was removed.
No live route was activated.
No SPL CPI behavior was enabled.
No `invoke_signed` path was enabled.
No minting was enabled.
No deployment behavior was enabled.
No deployability predicate was changed.
