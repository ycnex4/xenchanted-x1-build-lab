# xEnchanted X1 Build Lab final MVP readiness checkpoint

This document records the final MVP readiness checkpoint for the xEnchanted X1 Build Lab.

This checkpoint is documentation-only.

No runtime code is changed in this checkpoint.

No dependencies are changed in this checkpoint.

No real RPC is executed in this checkpoint.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Status

The current repository is a tested MVP implementation lab.

It is ready for:

- architecture review
- accounting review
- implementation review
- incremental hardening planning
- future integration planning

It is not yet:

- a production chain deployment
- a trustless proof verification system
- a live watcher service runtime
- a bridge execution system
- a token issuance deployment
- a UI product
- an operator production stack

## Current main baseline

Latest completed main milestone:

    main -> 433e041 Merge branch 'xc-build-commitment-status-app-view-completion-checkpoint'

Final validation baseline:

- npm run typecheck passed
- npm test passed: 42 test files, 328 tests
- npm run build passed
- npm audit --audit-level=moderate found 0 vulnerabilities

## Implemented MVP architecture

The implemented MVP lab covers the core flow:

    watcher candidate
    -> proof object
    -> appSubmitProof
    -> registrar handler
    -> BuildState update
    -> replay protection
    -> snapshot persistence / CLI inspection

The implemented layers include:

- Build state
- Build registry
- create build flow
- Core redeem BLD accounting
- XEN Burn Power accounting
- Genesis Origin BLD accounting
- XNTD lock and relock state
- X1 fee contribution checkpoints
- registrar replay protection
- source event replay protection
- XNTD commitment event replay protection
- registrar handlers
- proof object types
- proof-to-registrar payload builders
- watcher candidate types
- watcher-to-proof conversion
- app proof submission
- end-to-end watcher-proof-registrar scenario
- storage serialization
- snapshot verification
- backup-enabled snapshot save
- snapshot recovery load
- read-only CLI command layer
- executable CLI entry point
- CLI snapshot show
- CLI snapshot verify
- CLI snapshot recover
- Ethereum read provider wrappers
- Ethereum script config checks
- Ethereum script runner boundaries
- authoritative XC epoch minimum source
- authoritative XC epoch minimum provider source
- XC protocol params source
- XC protocol params build validation
- XC Build validation context
- XC Build commitment status model
- app Build view exposing commitmentStatus

## Build accounting model

The Build model tracks:

- historyBld
- availableBld
- originBld
- xenBurnPower
- lockedXntd
- requiredXntdLock
- lockEpoch
- X1 fee contribution checkpoints
- replay protection state

Important meaning:

    historyBld is historical contribution.
    availableBld is usable / spendable / transferable BLD.
    originBld is Genesis allocation, not history.
    commitmentStatus is current XNTD commitment signal.

## Commitment status model

The project now uses commitment status terminology.

Retired terminology:

    active status
    inactive status

Accepted terminology:

    commitmentStatus

Implemented helper:

    getBuildCommitmentStatus()

Implemented app view helper:

    appGetBuildView()

Implemented status values:

    COMMITTED
    UNCOMMITTED
    UNKNOWN

Implemented reason values:

    COMMITMENT_CURRENT
    NO_HISTORY
    NO_COMMITMENT
    COMMITMENT_BELOW_REQUIRED
    RECOMMITMENT_REQUIRED
    UNKNOWN_NO_CURRENT_CONTEXT

Meaning:

    commitmentStatus = current XNTD commitment signal

It does not mean:

- Build validity
- historical contribution validity
- global eligibility
- user punishment

UNCOMMITTED does not erase or invalidate:

- historyBld
- availableBld
- originBld
- Core redeem history
- Build history

`appGetBuildView()` may return `UNKNOWN` when context-dependent epoch information is required but unavailable.

`UNKNOWN` is a status signal, not an error.

## appGetBuildView()

The app view helper returns:

    AppBuildView {
      build
      commitmentStatus
    }

The helper is read-only and non-mutating.

It does not change:

- appSubmitProof behavior
- watcher behavior
- registrar behavior
- proof payload behavior
- ethereum/RPC code
- scripts
- dependencies
- CLI commands
- BLD transfer/sale rules
- Forge requirements
- unlock mechanics
- Build actor scope

## Validation and replay protection

The MVP includes replay protection for:

- registrar messages
- source Core redeem events
- source XEN burn events
- XNTD commitment events

The XNTD lock / relock path includes:

- observedRequiredXntdLock propagation
- lock epoch ordering guard
- authoritative XC epoch minimum validation path
- appSubmitProof support for optional XC epoch minimum source
- XC Build validation context support

## Snapshot and CLI safety

Snapshot helpers are conservative.

The implemented snapshot model supports:

- deterministic snapshot serialization
- snapshot verification through decode / deserialize path
- backup-enabled save with verification
- recovery load from canonical or backup snapshot
- read-only recovery reporting

Snapshot recovery does not:

- repair canonical snapshots
- copy backup into canonical
- delete corrupted files
- migrate snapshot files
- create new backup files

The CLI remains read-only.

CLI commands include:

- help
- version
- snapshot:show
- snapshot:verify
- snapshot:recover

The CLI does not:

- mutate protocol state
- mutate snapshot files
- restore automatically
- migrate data
- delete corrupted files

## Trust assumptions

The MVP remains a trusted-indexer / trusted-registrar implementation lab.

The MVP assumes:

- trusted watcher / indexer input
- registrar authority for applying verified messages
- canonical event key convention
- finalized source events as provided by watcher candidates
- authoritative XC state source for epoch minimum validation when supplied
- explicit replay protection through tracked message and source event keys
- read-only CLI recovery behavior

The MVP does not claim trustless proof verification.

## Non-goals

The current MVP intentionally does not include:

- production chain deployment
- live watcher service runtime
- bridge execution
- token issuance logic
- UI
- operator restore tooling
- trustless proof verification
- Merkle proof verification
- registrar signature scheme
- external project policy enforcement
- Build actor profile
- Forge participation requirement
- unlock flow
- BLD marketplace
- production monitoring
- production incident response automation

## Build actor future layer

Build actor is a future layer.

Current MVP does not implement Build actor.

Future Build actor direction may describe the semantic actor behind a Build owner address, such as:

- user
- contract
- project
- DAO
- unknown

This future layer should not be mixed into the current MVP completion.

## Review readiness

The current repository is ready for review of:

- domain model boundaries
- Build accounting semantics
- proof-to-registrar flow
- replay protection model
- XNTD lock / relock requirement validation path
- snapshot safety model
- read-only CLI behavior
- commitmentStatus terminology and app view
- MVP assumptions and non-goals

## Recommended post-MVP work

Post-MVP work may include:

- production watcher runtime design
- production registrar authority / signature model
- trustless proof verification design
- live RPC integration strategy
- operator restore tooling
- production monitoring and alerting
- Build actor profile design
- UI / API integration
- external project integration policy
- security review
- README and review summary synchronization
- public documentation polishing

## Decision

The xEnchanted X1 Build Lab MVP implementation lab is complete at the current scope.

The completed scope is a tested implementation lab, not a production deployment.

The next work should be post-MVP readiness, review, and hardening, not further MVP scope expansion.
