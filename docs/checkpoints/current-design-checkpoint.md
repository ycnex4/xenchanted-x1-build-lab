# Current Design Checkpoint

Status: draft / design phase.

This repository tracks the current X1 Build design before implementation.

## Latest implementation checkpoint

The first implementation scaffold branch was completed and merged into main.

Merge commit:

- 1041972 Merge branch 'build-mvp-scaffold'

The scaffold adds:

- TypeScript model-first setup
- Vitest test setup
- placeholder BuildState model
- placeholder create_build instruction module
- basic error module
- scaffold test
- implementation/build-mvp-scaffold-notes.md

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- 1 test file passed
- 1 test passed

No real accounting logic is implemented yet.


## Latest BuildState checkpoint

The BuildState account / object milestone was completed and merged into main.

Merge commit:

- ae7ad29 Merge branch 'buildstate-account'

The milestone adds:

- full TypeScript BuildState shape
- BUILD_STATE_VERSION
- createEmptyBuildState factory
- initial default value tests
- implementation/buildstate-account-notes.md

Validation before merge:

- npm run typecheck: passed
- npm test: passed
- 2 test files passed
- 6 tests passed

No real accounting transitions are implemented yet.

## Latest create_build checkpoint

The create_build instruction milestone was completed and merged into main.

Merge commit:

- 3f12ba9 Merge branch 'create-build'

The milestone adds:

- createBuild input type
- createBuild function
- empty BuildState creation through createEmptyBuildState
- tests proving createBuild creates no contribution or commitment value
- implementation/create-build-notes.md

Validation before merge:

- npm run typecheck: passed
- npm test: passed
- 3 test files passed
- 12 tests passed

No duplicate Build prevention, registry logic, registrar processing, or accounting transitions are implemented yet.

## Latest Build registry checkpoint

The canonical Build registry / duplicate prevention milestone was completed and merged into main.

Merge commit:

- 4e8c8de Merge branch 'build-registry'

The milestone adds:

- BuildRegistry structure
- createEmptyBuildRegistry factory
- createRegisteredBuild helper
- duplicate buildId protection
- duplicate owner protection
- duplicate Ethereum identity protection
- BuildErrorCode values for duplicate registry errors
- tests proving registry creation does not create accounting value
- implementation/build-registry-notes.md

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- 4 test files passed
- 18 tests passed

No registrar message processing, source-event replay protection, or accounting transitions are implemented yet.

## Latest Registrar replay checkpoint

The Registrar message replay protection milestone was completed and merged into main.

Merge commit:

- 3b2f5ea Merge branch 'registrar-replay-protection'

The milestone adds:

- RegistrarState structure
- RegistrarMessage type
- RegistrarMessageKind type
- createRegistrarState factory
- acceptRegistrarMessage helper
- registrar authority check
- processedMessages replay protection
- UnauthorizedRegistrar error
- DuplicateRegistrarMessage error
- tests proving rejected messages do not mutate state
- implementation/registrar-replay-protection-notes.md

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- 5 test files passed
- 22 tests passed

No Core redeem accounting, XEN burn accounting, Genesis Origin, XNTD lock, fee checkpoint, signature validation, Merkle proof, or bridge proof logic is implemented yet.

## Latest Core redeem BLD checkpoint

The Core redeem -> history_bld milestone was completed and merged into main.

Merge commit:

- d5e864e Merge branch 'core-redeem-bld'

The milestone adds:

- applyCoreRedeemBld input type
- applyCoreRedeemBld transition
- InvalidBldAmount error
- positive BLD amount validation
- historyBld accumulation
- availableBld accumulation
- updatedAt update from redeemedAt
- tests proving unrelated layers do not change
- implementation/core-redeem-bld-notes.md

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- 6 test files passed
- 30 tests passed

No used_redeem_events replay protection, source redeem key validation, registrar integration, Core NFT proof validation, Genesis Origin BLD, XEN Burn Power, XNTD lock, or fee checkpoint logic is implemented yet.

## Latest used_redeem_events checkpoint

The used_redeem_events replay protection milestone was completed and merged into main.

Merge commit:

- 7cff54c Merge branch 'redeem-event-replay'

The milestone adds:

- RedeemEventKey type
- CoreRedeemEvent type
- RedeemEventState structure
- createRedeemEventState factory
- acceptCoreRedeemEvent helper
- usedRedeemEvents replay protection
- DuplicateRedeemEvent error
- tests proving duplicate redeemKey cannot apply BLD twice
- tests proving invalid BLD amount does not mark redeemKey as used
- implementation/redeem-event-replay-notes.md

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- 7 test files passed
- 35 tests passed

No registrar CORE_REDEEM integration, source redeem key derivation, Ethereum log proof validation, Core NFT proof validation, XEN burn accounting, Genesis Origin BLD, XNTD lock, or fee checkpoint logic is implemented yet.

## Latest Registrar CORE_REDEEM checkpoint

The Registrar CORE_REDEEM message integration milestone was completed and merged into main.

Merge commit:

- 0dd4a22 Merge branch 'registrar-core-redeem'

The milestone adds:

- ApplyRegistrarCoreRedeemInput type
- applyRegistrarCoreRedeem helper
- CORE_REDEEM message kind validation
- registrar authority validation
- processedMessages duplicate check
- usedRedeemEvents duplicate check
- Core redeem BLD application through acceptCoreRedeemEvent
- registrar message recording after successful redeem application
- InvalidRegistrarMessageKind error
- integration tests for non-mutating failure paths
- implementation/registrar-core-redeem-notes.md

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- 8 test files passed
- 40 tests passed

No source redeem key derivation, Ethereum log proof validation, Core NFT proof validation, registrar signature validation, Merkle proof, bridge proof, XEN burn accounting, Genesis Origin BLD, XNTD lock, or fee checkpoint logic is implemented yet.

## Latest XEN Burn Power checkpoint

The XEN Burn Power milestone was completed and merged into main.

Merge commit:

- 3852653 Merge branch 'xen-burn-power'

The milestone adds:

- ApplyXenBurnPowerInput type
- applyXenBurnPower transition
- InvalidXbpAmount error
- positive XBP amount validation
- earnedXbp accumulation
- availableXbp accumulation
- updatedAt update from burnedAt
- tests proving unrelated layers do not change
- implementation/xen-burn-power-notes.md

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- 9 test files passed
- 47 tests passed

No used_xen_burn_events replay protection, XEN burn event key validation, registrar XEN_BURN integration, Ethereum XEN.burn log proof validation, BLD accounting, Genesis Origin BLD, XNTD lock, or fee checkpoint logic is implemented yet.

## Latest used_xen_burn_events checkpoint

The used_xen_burn_events replay protection milestone was completed and merged into main.

Merge commit:

- f29bbf0 Merge branch 'xen-burn-event-replay'

The milestone adds:

- XenBurnEventKey type
- XenBurnEvent type
- XenBurnEventState structure
- createXenBurnEventState factory
- acceptXenBurnEvent helper
- usedXenBurnEvents replay protection
- DuplicateXenBurnEvent error
- tests proving duplicate xenBurnKey cannot apply XBP twice
- tests proving invalid XBP amount does not mark xenBurnKey as used
- implementation/xen-burn-event-replay-notes.md

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- 10 test files passed
- 52 tests passed

No registrar XEN_BURN integration, source XEN burn key derivation, Ethereum XEN.burn log proof validation, XEN burn amount normalization policy, Genesis Origin BLD, XNTD lock, or fee checkpoint logic is implemented yet.

## Latest Registrar XEN_BURN checkpoint

The Registrar XEN_BURN message integration milestone was completed and merged into main.

Merge commit:

- 1c97ee7 Merge branch 'registrar-xen-burn'

The milestone adds:

- ApplyRegistrarXenBurnInput type
- applyRegistrarXenBurn helper
- XEN_BURN message kind validation
- registrar authority validation
- processedMessages duplicate check
- usedXenBurnEvents duplicate check
- XEN Burn Power application through acceptXenBurnEvent
- registrar message recording after successful XEN burn application
- tests for non-mutating failure paths
- tests proving BLD and unrelated accounting values are not created
- implementation/registrar-xen-burn-notes.md

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- 11 test files passed
- 58 tests passed

No source XEN burn key derivation, Ethereum XEN.burn log proof validation, XEN burn amount normalization policy, registrar signature validation, Merkle proof, bridge proof, Genesis Origin BLD, XNTD lock, or fee checkpoint logic is implemented yet.

## Latest Genesis Origin BLD checkpoint

The Genesis Origin BLD milestone was completed and merged into main.

Merge commit:

- c622459 Merge branch 'genesis-origin-bld'

The milestone adds:

- ClaimGenesisOriginBldInput type
- calculateGenesisOriginBld helper
- claimGenesisOriginBld transition
- tiered Genesis Origin allocation
- duplicate claim protection
- not-eligible protection
- GenesisOriginAlreadyClaimed error
- GenesisOriginNotEligible error
- tests for all tier thresholds
- tests proving unrelated layers do not change
- implementation/genesis-origin-bld-notes.md

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- 12 test files passed
- 66 tests passed

No registrar GENESIS_ORIGIN integration, genesis_origin_claimed external registry, signature validation, Merkle proof, bridge proof, XNTD lock, or fee checkpoint logic is implemented yet.

## Latest XNTD lock / relock checkpoint

The XNTD lock / relock milestone was completed and merged into main.

Merge commit:

- ff27649 Merge branch 'xntd-lock-relock'

The milestone adds:

- LockXntdInput type
- RelockXntdInput type
- lockXntd transition
- relockXntd transition
- positive XNTD lock amount validation
- active commitment requirement for relock
- relock BLD integrity rule
- InvalidXntdLockAmount error
- XntdCommitmentNotActive error
- InsufficientAvailableBldForRelock error
- tests for lock and relock behavior
- tests proving Genesis Origin BLD does not block relock when availableBld >= historyBld
- tests proving unrelated accounting values are not created
- implementation/xntd-lock-relock-notes.md

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- 13 test files passed
- 73 tests passed

No registrar LOCK_XNTD / RELOCK_XNTD integration, unlock flow, lock proof validation, external XNTD escrow mechanics, epoch parameter source, XNTD amount calculation policy, BLD transfer / burn mechanics, or fee checkpoint logic is implemented yet.

## Latest X1 Fee Contribution checkpoint

The X1 Fee Contribution checkpoint milestone was completed and merged into main.

Merge commit:

- a2ca587 Merge branch 'x1-fee-contribution'

The milestone adds:

- ApplyX1FeeContributionCheckpointInput type
- applyX1FeeContributionCheckpoint transition
- positive fee amount validation
- positive tx count validation
- increasing countedUntilSlot validation
- x1FeeContribution accumulation
- x1TxCount accumulation
- x1FeeCountedUntilSlot update
- lastFeeUpdateAt update
- updatedAt update
- InvalidFeeContributionAmount error
- InvalidFeeContributionTxCount error
- NonIncreasingFeeCheckpointSlot error
- tests for valid and rejected checkpoints
- tests proving BLD, XBP, and XNTD commitment are not created or changed
- implementation/x1-fee-contribution-notes.md

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- 14 test files passed
- 80 tests passed

No registrar X1_FEE_CHECKPOINT integration, source transaction proof validation, external X1 fee indexing, slot finality policy, fee normalization policy, bridge proof logic, or BLD minting from fees is implemented yet.

## Latest Registrar LOCK_XNTD / RELOCK_XNTD checkpoint

The Registrar LOCK_XNTD / RELOCK_XNTD integration milestone was completed and merged into main.

Merge commit:

- 37ee89a Merge branch 'registrar-xntd-lock'

The milestone adds:

- ApplyRegistrarXntdLockInput type
- ApplyRegistrarXntdRelockInput type
- applyRegistrarXntdLock helper
- applyRegistrarXntdRelock helper
- LOCK_XNTD message kind support
- RELOCK_XNTD message kind support
- registrar authority validation
- processedMessages duplicate check
- lock / relock application
- registrar message recording after successful lock / relock
- tests for non-mutating failure paths
- tests proving unrelated accounting values are not created
- implementation/registrar-xntd-lock-notes.md

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- 15 test files passed
- 88 tests passed

No external XNTD escrow proof validation, registrar signature validation, Merkle proof, bridge proof, unlock flow, epoch parameter source, XNTD amount calculation policy, BLD transfer / burn mechanics, or Registrar X1_FEE_CHECKPOINT integration is implemented yet.

## Latest Registrar X1_FEE_CHECKPOINT checkpoint

The Registrar X1_FEE_CHECKPOINT integration milestone was completed and merged into main.

Merge commit:

- d09a07a Merge branch 'registrar-x1-fee-checkpoint'

The milestone adds:

- ApplyRegistrarX1FeeCheckpointInput type
- applyRegistrarX1FeeCheckpoint helper
- X1_FEE_CHECKPOINT message kind support
- X1_FEE_CHECKPOINT message kind validation
- registrar authority validation
- processedMessages duplicate check
- fee checkpoint application
- registrar message recording after successful checkpoint application
- tests for non-mutating failure paths
- tests proving BLD, XBP, and XNTD commitment are not created or changed
- implementation/registrar-x1-fee-checkpoint-notes.md

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- 16 test files passed
- 96 tests passed

No source transaction proof validation, external X1 fee indexing, slot finality policy, fee normalization policy, registrar signature validation, Merkle proof, bridge proof, or BLD minting from fees is implemented yet.

## Latest BuildState MVP review checkpoint

The BuildState MVP model review / consolidation milestone was completed and merged into main.

Merge commit:

- 2834ea3 Merge branch 'buildstate-mvp-review'

The milestone adds:

- implementation/buildstate-mvp-review.md
- consolidated BuildState MVP model overview
- accounting layer review
- replay protection review
- registrar integration review
- atomicity model review
- current test coverage summary
- known exclusions list
- next recommended milestones

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- 16 test files passed
- 96 tests passed

No TypeScript model logic was changed in this milestone.

## Latest Implementation test matrix checkpoint

The Implementation test matrix milestone was completed and merged into main.

Merge commit:

- 0ec81cc Merge branch 'implementation-test-matrix'

The milestone adds:

- implementation/implementation-test-matrix.md
- mapping of model layers to implementation files
- mapping of model layers to test files
- covered property summary for each implemented layer
- cross-cutting invariant review
- replay protection coverage review
- atomicity coverage review
- known MVP gaps
- recommended future test extensions

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- 16 test files passed
- 96 tests passed

No TypeScript model logic was changed in this milestone.

## Latest Registrar / instruction surface review checkpoint

The Registrar / instruction surface review milestone was completed and merged into main.

Merge commit:

- 0d5421f Merge branch 'registrar-instruction-surface-review'

The milestone adds:

- implementation/registrar-instruction-surface-review.md
- registrar message kind review
- registrar state review
- registrar integration surface table
- non-registrar instruction surface table
- model / state helper surface table
- atomicity pattern review
- failure behavior review
- accounting separation review
- surface consistency observations
- known exclusions
- recommended next milestones

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- 16 test files passed
- 96 tests passed

No TypeScript model logic was changed in this milestone.

## Latest Post-MVP integration policy checkpoint

The Post-MVP integration policy milestone was completed and merged into main.

Merge commit:

- dc7fbe3 Merge branch 'post-mvp-integration-policy'

The milestone adds:

- implementation/post-mvp-integration-policy.md
- MVP boundary definition
- post-MVP integration order
- proof layer policy
- storage layer policy
- serialization policy
- API / CLI policy
- registrar integration policy
- event key policy
- watcher / indexer policy
- migration policy
- security policy
- testing policy
- explicit non-goals

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- 16 test files passed
- 96 tests passed

No TypeScript model logic was changed in this milestone.

## Latest Proof model design checkpoint

The Proof model design milestone was completed and merged into main.

Merge commit:

- 00238c0 Merge branch 'proof-model-design'

The milestone adds:

- implementation/proof-model-design.md
- proof layer boundary definition
- proof object categories
- canonical event key policy
- proof validation stages
- Core redeem proof direction
- XEN burn proof direction
- XNTD lock / relock proof direction
- X1 fee checkpoint proof direction
- Genesis Origin eligibility proof direction
- registrar proof pattern recommendation
- finality policy
- proof replay policy
- failure policy
- testing policy

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- 16 test files passed
- 96 tests passed

No TypeScript model logic was changed in this milestone.

## Latest Storage / serialization model checkpoint

The Storage / serialization model milestone was completed and merged into main.

Merge commit:

- 7d4add1 Merge branch 'storage-serialization-model'

The milestone adds:

- implementation/storage-serialization-model.md
- storage boundary definition
- persisted state categories
- BuildState persistence direction
- registry persistence direction
- registrar state persistence direction
- replay state persistence direction
- bigint serialization policy
- Set serialization policy
- null handling policy
- schema versioning policy
- snapshot policy
- migration policy
- atomic persistence policy
- atomic write strategy
- backup strategy
- integrity check direction
- storage adapter boundary
- future serialization tests list

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- 16 test files passed
- 96 tests passed

No TypeScript model logic was changed in this milestone.

## Latest API / CLI surface design checkpoint

The API / CLI surface design milestone was completed and merged into main.

Merge commit:

- fbce4a1 Merge branch 'api-cli-surface-design'

The milestone adds:

- implementation/api-cli-surface-design.md
- API / CLI boundary definition
- API surface categories
- build query API direction
- build creation API direction
- registrar message API direction
- proof submission API direction
- snapshot / storage API direction
- health / diagnostics API direction
- CLI command categories
- error response policy
- input validation policy
- bigint input / output policy
- idempotency policy
- security policy
- logging policy
- testing policy
- recommended implementation order

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- 16 test files passed
- 96 tests passed

No TypeScript model logic was changed in this milestone.

## Latest End-to-end scenario tests checkpoint

The End-to-end scenario tests milestone was completed and merged into main.

Merge commit:

- 464f6e1 Merge branch 'e2e-scenario-tests'

The milestone adds:

- tests/e2e-scenario.test.ts
- implementation/e2e-scenario-tests-notes.md
- full MVP Build lifecycle scenario
- registrar CORE_REDEEM flow coverage
- registrar XEN_BURN flow coverage
- Genesis Origin BLD claim coverage
- registrar LOCK_XNTD flow coverage
- registrar RELOCK_XNTD flow coverage
- registrar X1_FEE_CHECKPOINT flow coverage
- final BuildState accounting assertions
- registrar processedMessages assertions
- Core redeem event replay assertions
- XEN burn event replay assertions
- duplicate registrar message rejection checks
- duplicate Core redeem event rejection checks
- duplicate XEN burn event rejection checks

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- 17 test files passed
- 98 tests passed

This milestone added real test coverage.

## Latest External watcher / indexer integration checkpoint

The External watcher / indexer integration milestone was completed and merged into main.

Merge commit:

- 2c55df6 Merge branch 'watcher-indexer-integration'

The milestone adds:

- implementation/watcher-indexer-integration.md
- watcher / indexer boundary definition
- expected watcher categories
- watcher pipeline
- canonical event key policy
- Core redeem watcher direction
- XEN burn watcher direction
- XNTD lock / relock watcher direction
- X1 fee checkpoint indexer direction
- duplicate handling policy
- reorg handling policy
- error handling policy
- security policy
- testing policy
- recommended implementation order

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- 17 test files passed
- 98 tests passed

No TypeScript model logic was changed in this milestone.

## Latest Storage implementation checkpoint

The Storage implementation milestone was completed and merged into main.

Merge commit:

- 6a3074c Merge branch 'storage-implementation'

The milestone adds:

- src/storage/serialization.ts
- tests/storage-serialization.test.ts
- implementation/storage-implementation-notes.md
- serialization helpers for BuildState
- serialization helpers for BuildRegistry
- serialization helpers for RegistrarState
- serialization helpers for RedeemEventState
- serialization helpers for XenBurnEventState
- STORAGE_SCHEMA_VERSION = 1
- bigint decimal string serialization
- Set serialization as sorted string arrays
- Map serialization as sorted key-value arrays
- duplicate set entry rejection
- duplicate buildId rejection during registry deserialization
- invalid bigint string rejection
- round-trip tests for MVP state objects

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- 18 test files passed
- 103 tests passed

This milestone added real storage serialization code and tests.

## Latest API / CLI implementation checkpoint

The API / CLI implementation milestone was completed and merged into main.

Merge commit:

- c87bf3c Merge branch 'api-cli-implementation'

The milestone adds:

- src/app/build-service.ts
- tests/app-build-service.test.ts
- implementation/api-cli-implementation-notes.md
- BuildApplicationState
- AppResult structured result type
- application state creation helper
- registered Build creation service helper
- Build query service helper
- Genesis Origin BLD service helper
- registrar Core redeem service helper
- registrar XEN burn service helper
- registrar XNTD lock service helper
- registrar XNTD relock service helper
- registrar X1 fee checkpoint service helper
- structured BuildError conversion
- full lifecycle application service tests
- non-mutating registrar rejection service test

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- 19 test files passed
- 107 tests passed

This milestone added a real application service layer for future API / CLI use.

## Latest Proof object type implementation checkpoint

The Proof object type implementation milestone was completed and merged into main.

Merge commit:

- f23941b Merge branch 'proof-object-types'

The milestone adds:

- src/proofs/proof-types.ts
- tests/proof-types.test.ts
- implementation/proof-object-types-notes.md
- ProofValidationStatus
- ProofKind
- CanonicalEventKey
- CanonicalEventKeyInput
- ProofSourceMetadata
- BaseProof
- CoreRedeemProof
- XenBurnProof
- XntdLockProof
- XntdRelockProof
- X1FeeCheckpointProof
- GenesisOriginEligibilityProof
- BuildProof union type
- createCanonicalEventKey helper
- createProofSourceMetadata helper
- isValidatedProof helper
- assertValidatedProof helper
- canonical event key validation tests
- proof source metadata tests
- validated / non-validated proof tests

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- 20 test files passed
- 112 tests passed

This milestone added real proof object type code and tests.

## Latest Watcher candidate type implementation checkpoint

The Watcher candidate type implementation milestone was completed and merged into main.

Merge commit:

- a040bc1 Merge branch 'watcher-candidate-types'

The milestone adds:

- src/watchers/watcher-candidates.ts
- tests/watcher-candidates.test.ts
- implementation/watcher-candidate-types-notes.md
- WatcherCandidateKind
- WatcherCandidateBase
- CoreRedeemCandidate
- XenBurnCandidate
- XntdLockCandidate
- XntdRelockCandidate
- X1FeeCheckpointCandidate
- WatcherCandidate union type
- createWatcherCandidateBase helper
- createCoreRedeemCandidate helper
- createXenBurnCandidate helper
- createXntdLockCandidate helper
- createXntdRelockCandidate helper
- createX1FeeCheckpointCandidate helper
- isFinalizedWatcherCandidate helper
- assertFinalizedWatcherCandidate helper
- watcher candidate tests for canonical event key derivation
- watcher candidate finality tests

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- 21 test files passed
- 117 tests passed

This milestone added real watcher candidate type code and tests.

## Latest File snapshot storage adapter checkpoint

The File snapshot storage adapter milestone was completed and merged into main.

Merge commit:

- 72b261f Merge branch 'file-snapshot-storage'

The milestone adds:

- src/storage/snapshot.ts
- tests/storage-snapshot.test.ts
- implementation/file-snapshot-storage-notes.md
- SerializedBuildApplicationSnapshot
- serializeBuildApplicationSnapshot helper
- deserializeBuildApplicationSnapshot helper
- encodeSnapshotJson helper
- decodeSnapshotJson helper
- saveSnapshotFile helper
- loadSnapshotFile helper
- full application state snapshot support
- snapshot schema version validation
- snapshot kind validation
- local JSON snapshot file save / load tests
- temporary file + rename write pattern
- pretty JSON output with trailing newline

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- 22 test files passed
- 121 tests passed

This milestone added real file snapshot storage adapter code and tests.

## Latest CLI command implementation checkpoint

The CLI command implementation milestone was completed and merged into main.

Merge commit:

- 4ec36f1 Merge branch 'cli-command-implementation'

The milestone adds:

- src/cli/parse.ts
- src/cli/commands.ts
- tests/cli-commands.test.ts
- implementation/cli-command-implementation-notes.md
- parseCliArgs helper
- getStringFlag helper
- renderCliHelp helper
- runCliCommand dispatcher
- CLI_VERSION = 0.1.0
- help command
- version command
- snapshot:show --file <path> command
- snapshot summary JSON output
- structured CLI command result
- structured unknown command failure
- structured missing flag failure
- read-only CLI behavior

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- 23 test files passed
- 127 tests passed

This milestone added a real minimal CLI command layer and tests.

## Latest Proof-to-registrar payload builder checkpoint

The Proof-to-registrar payload builder milestone was completed and merged into main.

Merge commit:

- 1e57627 Merge branch 'proof-to-registrar-builders'

The milestone adds:

- src/proofs/registrar-builders.ts
- tests/proof-registrar-builders.test.ts
- implementation/proof-to-registrar-builders-notes.md
- CreateRegistrarPayloadInput
- CoreRedeemRegistrarPayload
- XenBurnRegistrarPayload
- XntdLockRegistrarPayload
- XntdRelockRegistrarPayload
- X1FeeCheckpointRegistrarPayload
- RegistrarPayloadFromProof union type
- buildCoreRedeemRegistrarPayload helper
- buildXenBurnRegistrarPayload helper
- buildXntdLockRegistrarPayload helper
- buildXntdRelockRegistrarPayload helper
- buildX1FeeCheckpointRegistrarPayload helper
- buildRegistrarPayloadFromProof helper
- deterministic default proof message id policy
- custom messageId support
- validated proof requirement
- Genesis Origin proof rejection for registrar payload mapping

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- 24 test files passed
- 134 tests passed

This milestone added real proof-to-registrar payload builder code and tests.

## Latest Watcher-to-proof candidate conversion checkpoint

The Watcher-to-proof candidate conversion milestone was completed and merged into main.

Merge commit:

- 05f994e Merge branch 'watcher-to-proof-conversion'

The milestone adds:

- src/watchers/proof-conversion.ts
- tests/watcher-proof-conversion.test.ts
- implementation/watcher-to-proof-conversion-notes.md
- WatcherProofConversionInput
- convertCoreRedeemCandidateToProof helper
- convertXenBurnCandidateToProof helper
- convertXntdLockCandidateToProof helper
- convertXntdRelockCandidateToProof helper
- convertX1FeeCheckpointCandidateToProof helper
- convertWatcherCandidateToProof helper
- finalized watcher candidate requirement
- validated proof creation from finalized watcher candidates
- canonical event key preservation
- Core redeem redeemKey derivation from canonical event key
- XEN burn xenBurnKey derivation from canonical event key
- watcher candidate to proof to registrar payload pipeline test

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- 25 test files passed
- 141 tests passed

This milestone added real watcher-to-proof conversion code and tests.

## Current decision

Create documentation first. Do not start code until the core spec, BuildState fields, state transitions, registrar model, indexer models, and economic assumptions are reviewed.

## Documents created

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
- docs/economics/post-mvp-bld-composability.md

### Checkpoints

- docs/checkpoints/current-design-checkpoint.md
- docs/checkpoints/mvp-implementation-sequence.md
- docs/checkpoints/documentation-consistency-review.md
- docs/checkpoints/implementation-branch-plan.md
- docs/checkpoints/implementation-risk-checklist.md
- docs/checkpoints/final-pre-implementation-checkpoint.md
- docs/checkpoints/first-implementation-milestone.md

## Current model summary

X1 Build is a voluntary NFT-like user object in X1 that records independent verified contribution layers.

The main layers are:

- BLD from redeemed Core NFT history
- XEN Burn Power from verified XEN.burn(user, amount) calls
- XNTD commitment through lock / relock
- X1 Fee Contribution from network fees paid by an address as X1 fee payer
- Build creation in X1 through burn 11 BLD

These layers must remain separate.

BLD does not come from XBP.
XBP does not come from BLD.
X1 Fee Contribution does not create BLD or XBP.
XNTD lock does not create BLD or XBP.

## BLD

BLD is the normalized Build unit derived from redeemed Core NFT history.

Display unit:

1 BLD = 100,000,000 XEN burned through redeemed Core history

Fields:

- history_bld
- available_bld
- origin_bld

history_bld is historical and does not decrease when available BLD is sold, spent, burned, or transferred.

## Genesis Origin BLD

Genesis Origin BLD is a one-time tiered allocation.

It is based on history_bld at the first valid xEnchanted Crypto history connection during the Build Genesis Epoch.

Allocation tiers:

- history_bld >= 1     -> origin_bld = 11
- history_bld >= 11    -> origin_bld = 22
- history_bld >= 121   -> origin_bld = 55
- history_bld >= 1111  -> origin_bld = 121

121 BLD is the maximum Genesis Origin cap, not the default allocation.

Genesis Origin BLD is not earned BLD.

When granted:

origin_bld += tiered_origin_bld
available_bld += tiered_origin_bld

It must not increase history_bld.

## Build creation in X1 through BLD burn

A user without XEN/XC history may create an active Build in X1 by burning BLD.

Requirement:

burn 11 BLD

This does not create fake history_bld or fake XEN burn history.

This path does not require XNTD lock or relock.

## XEN Burn Power

XEN Burn Power comes from verified official XEN burn calls.

Canonical source:

successful XEN.burn(user, amount)

Arbitrary transfers to the zero address are not counted.

Display unit:

1 XBP = 100,000,000 XEN burned through XEN.burn(user, amount)

## XNTD lock / relock

XNTD lock is a commitment requirement for Build records that received earned BLD through Core redeem.

Required lock:

required_xntd_lock = current epoch Core L1 nominal

Commitment activation:

xc_commitment_active =
  history_bld > 0
  AND locked_xntd >= required_xntd_lock

Relock is allowed only if:

available_bld >= history_bld

## X1 Fee Contribution

X1 Fee Contribution records cumulative network fees paid by an address as fee payer on X1.

It tracks fee payment activity, not necessarily user activity.

It should include:

- base fee
- priority fee

MVP model:

- trusted indexer
- cumulative checkpoints
- raw smallest X1 units on-chain
- display normalization off-chain / UI
- reader interface for other X1 projects

## Source protection

Main rule:

one source event -> one accounting action -> one Build

Protection keys / maps:

- used_redeem_events[redeem_key]
- used_xen_burn_events[xen_burn_key]
- processed_messages[message_id]
- genesis_origin_claimed[identity]
- canonical_build_by_identity[identity]

## Latest Application proof submission checkpoint

The Application service proof submission milestone was completed and merged into main.

Merge commit:

- 9ba5cad Merge branch 'app-proof-submission'

The milestone adds:

- src/app/proof-submission.ts
- tests/app-proof-submission.test.ts
- implementation/app-proof-submission-notes.md
- appSubmitProof helper
- application-level proof submission routing
- validated BuildProof to registrar payload submission path
- Build lookup by buildId before state transition
- Core redeem proof submission through registrar application service
- XEN burn proof submission through registrar application service
- XNTD lock proof submission through registrar application service
- XNTD relock proof submission through registrar application service
- X1 fee checkpoint proof submission through registrar application service
- structured rejection for non-validated proof
- structured rejection for missing Build
- structured rejection for Genesis Origin proof
- duplicate proof submission rejection through existing registrar replay protection

Important architectural result:

- the proof submission service does not duplicate accounting logic
- the proof submission service does not mutate BuildState directly
- the proof submission service does not bypass registrar replay protection
- Genesis Origin proof remains outside registrar payload submission

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- 26 test files passed
- 149 tests passed

This milestone completed the application-level bridge between validated proofs and existing registrar application service helpers.

## Latest Snapshot migration / backup policy checkpoint

The Snapshot migration / backup policy milestone was completed and merged into main.

Merge commit:

- bff811f Merge branch 'snapshot-migration-backup-policy'

The milestone adds:

- implementation/snapshot-migration-backup-policy.md
- snapshot schema version policy
- explicit migration policy
- migration test policy
- backup policy
- backup creation policy
- atomic write policy
- restore policy
- corrupted snapshot policy
- replay protection preservation policy
- snapshot timestamp policy
- concurrency policy
- CLI / API snapshot policy
- snapshot security policy
- recommended next implementation order

Important architectural result:

- this milestone is documentation-only
- no TypeScript model logic changed
- snapshot storage remains an accounting-preserving layer only
- unsupported schema versions remain rejected by default
- migration must be explicit and tested
- backup restore must not silently hide corruption in normal operation
- replay protection fields are treated as critical state

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- 26 test files passed
- 149 tests passed

This milestone completed the policy layer for future snapshot migration, backup, restore, and recovery implementation.

## Latest CLI binary entry point checkpoint

The CLI binary entry point milestone was completed and merged into main.

Merge commit:

- 96bc3c9 Merge branch 'cli-binary-entry-point'

The milestone adds:

- src/cli/main.ts
- tests/cli-main.test.ts
- implementation/cli-binary-entry-point-notes.md
- package.json build script
- package.json cli script
- package.json bin entry
- x1-build-lab binary name
- runCliMain helper
- memory-backed CLI main tests
- compiled CLI manual verification

The milestone verifies:

- help output writes to stdout and returns exit code 0
- version output writes to stdout and returns exit code 0
- unknown command writes to stderr and returns exit code 1
- compiled CLI version command works
- compiled CLI help command works

Important architectural result:

- the CLI now has a real Node.js executable entry point
- the existing command layer remains the source of command behavior
- the CLI remains read-only
- no state mutation commands were added
- binary entry path matches the current TypeScript output layout

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- node ./dist/src/cli/main.js version: passed
- node ./dist/src/cli/main.js help: passed
- 27 test files passed
- 152 tests passed

This milestone completed the executable CLI entry point for the existing read-only CLI command layer.

## Latest End-to-end watcher-proof-registrar scenario checkpoint

The End-to-end watcher-proof-registrar scenario milestone was completed and merged into main.

Merge commit:

- db012ca Merge branch 'e2e-watcher-proof-registrar-scenario'

The milestone adds:

- tests/e2e-watcher-proof-registrar-scenario.test.ts
- implementation/e2e-watcher-proof-registrar-scenario-notes.md
- full watcher candidate to proof to appSubmitProof scenario
- Core redeem watcher candidate coverage
- XEN burn watcher candidate coverage
- XNTD lock watcher candidate coverage
- XNTD relock watcher candidate coverage
- X1 fee checkpoint watcher candidate coverage
- final BuildState assertions
- registrar processedMessages assertion
- Core redeem replay set assertion
- XEN burn replay set assertion
- duplicate proof submission rejection assertion

The tested path is:

watcher candidate
  -> validated proof
  -> appSubmitProof
  -> registrar application service
  -> BuildState update
  -> replay protection state update

Important architectural result:

- existing layers compose in the intended order
- no new model logic was added
- no accounting rules were changed
- no new watcher validation rules were added
- Genesis Origin remains outside registrar payload submission

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- 28 test files passed
- 153 tests passed

This milestone completed the first full end-to-end proof submission scenario across watcher, proof, application, registrar, and replay protection layers.

## Latest Snapshot verification / backup implementation checkpoint

The Snapshot verification / backup implementation milestone was completed and merged into main.

Merge commit:

- 2aa4d9d Merge branch 'snapshot-verification-backup'

The milestone adds:

- verifySnapshotJson helper
- verifySnapshotFile helper
- saveSnapshotFileWithBackup helper
- SaveSnapshotFileWithBackupOptions
- default backup path policy using <snapshotPath>.bak
- optional custom backup path support
- temporary file verification before canonical replacement
- existing canonical verification before backup creation
- new canonical verification after replacement
- temporary file cleanup on failure
- snapshot verification tests
- invalid JSON verification rejection test
- invalid schema verification rejection test
- backup creation test
- no-backup-on-first-save test
- corrupted canonical rejection test
- implementation/snapshot-verification-backup-notes.md

Important architectural result:

- snapshot verification reuses the existing decode / deserialize path
- backup-enabled save does not replace a corrupted canonical snapshot
- corrupted canonical snapshots are preserved for investigation
- backup is created only from a verified existing canonical snapshot
- this milestone does not add migration functions
- this milestone does not add restore helpers
- this milestone does not add CLI mutation commands
- storage remains accounting-preserving only

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- 28 test files passed
- 159 tests passed

This milestone completed the first implementation step from the snapshot migration / backup policy.

## Latest Snapshot recovery load helper checkpoint

The Snapshot recovery load helper milestone was completed and merged into main.

Merge commit:

- 00fcb61 Merge branch 'snapshot-recovery-load'

The milestone adds:

- loadSnapshotFileWithRecovery helper
- LoadSnapshotFileWithRecoveryOptions
- LoadSnapshotFileWithRecoveryResult
- canonical snapshot recovery load path
- backup snapshot fallback path
- default backup path policy using <snapshotPath>.bak
- custom backup path support
- source reporting through source: canonical | backup
- loaded file path reporting
- combined canonical / backup failure error
- implementation/snapshot-recovery-load-notes.md

Important architectural result:

- recovery load does not repair canonical snapshots
- recovery load does not overwrite canonical files from backup
- recovery load does not delete corrupted files
- recovery load reports whether canonical or backup was used
- storage remains accounting-preserving only
- no migration functions were added
- no CLI recovery commands were added

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- 28 test files passed
- 163 tests passed

This milestone completed the recovery-load layer for canonical snapshot plus backup snapshot reads.

## Latest CLI snapshot verification command checkpoint

The CLI snapshot verification command milestone was completed and merged into main.

Merge commit:

- 418bcbf Merge branch 'cli-snapshot-verify-command'

The milestone adds:

- snapshot:verify --file <path> CLI command
- CLI help entry for snapshot:verify
- verifySnapshotFile integration in the CLI command layer
- valid snapshot verification JSON summary
- missing --file structured failure
- invalid snapshot structured failure
- implementation/cli-snapshot-verify-command-notes.md

The command returns a read-only JSON summary for valid snapshots:

- valid
- createdAt
- buildCount
- registrarAuthority
- processedMessageCount
- usedRedeemEventCount
- usedXenBurnEventCount

Important architectural result:

- snapshot verification is now available from the executable CLI
- the command is read-only
- the command does not mutate snapshots
- the command does not create backups
- the command does not recover from backups
- the command does not migrate snapshot files

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- node ./dist/src/cli/main.js help: passed
- 28 test files passed
- 166 tests passed

This milestone completed the first read-only CLI snapshot safety command.

## Latest CLI snapshot recovery command checkpoint

The CLI snapshot recovery command milestone was completed and merged into main.

Merge commit:

- 57f68fd Merge branch 'cli-snapshot-recovery-command'

The milestone adds:

- snapshot:recover --file <path> [--backup <path>] CLI command
- CLI help entry for snapshot:recover
- loadSnapshotFileWithRecovery integration in the CLI command layer
- canonical snapshot recovery CLI path
- backup snapshot recovery CLI path
- custom backup path support through --backup
- missing --file structured failure
- recovery failure structured stderr path
- implementation/cli-snapshot-recovery-command-notes.md

The command returns a read-only JSON summary for successful recovery:

- recovered
- source
- filePath
- createdAt
- buildCount
- registrarAuthority
- processedMessageCount
- usedRedeemEventCount
- usedXenBurnEventCount

Important architectural result:

- snapshot recovery loading is now available from the executable CLI
- the command is read-only
- the command does not repair canonical snapshots
- the command does not copy backup into canonical
- the command does not delete corrupted files
- the command does not create backups
- the command does not migrate snapshot files
- the command only reports which snapshot source was used

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- node ./dist/src/cli/main.js help: passed
- 28 test files passed
- 171 tests passed

This milestone completed the read-only CLI snapshot recovery command layer.

## Latest audit Vitest upgrade checkpoint

The audit Vitest upgrade milestone was completed and merged into main.

Merge commit:

- b815498 Merge branch 'audit-vitest-upgrade'

The milestone adds:

- Vitest upgrade from ^2.1.0 to ^4.1.8
- refreshed package-lock.json
- vitest.config.ts
- dist/** exclusion from Vitest test discovery
- implementation/audit-vitest-upgrade-notes.md

Reason:

The previous npm audit report showed vulnerabilities through the older Vitest / Vite / esbuild dependency chain.

The automatic npm audit recommendation required a breaking Vitest upgrade, so the upgrade was handled explicitly on a separate branch instead of running npm audit fix --force blindly.

Important tooling result:

- npm audit now reports 0 vulnerabilities
- Vitest 4 no longer discovers compiled tests under dist/tests
- npm test remains focused on source tests under tests/
- this milestone changes dev tooling only
- no protocol logic changed
- no application state logic changed
- no CLI behavior changed
- no snapshot behavior changed

Validation after merge:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 28 test files passed
- 171 tests passed

Runtime note:

Vitest 4 dependency tooling may require newer Node versions through its dependency graph. The current local environment uses Node 24 and passes all checks.

This milestone completed the npm audit review item.

## Latest review readiness summary checkpoint

The review readiness summary milestone was completed on the review-readiness-summary branch.

Commit:

- 4570cfd Add review readiness summary

The milestone adds:

- docs/review-readiness-summary.md

The summary document gives external reviewers a compact entry point for the repository.

It covers:

- project purpose
- current review target
- high-level watcher -> proof -> registrar -> BuildState architecture
- Build state model
- BLD terminology
- Genesis Origin model
- XNTD lock / relock model
- source event protection
- implemented layers
- snapshot safety model
- CLI safety model
- validation status
- current non-goals
- suggested review questions for Theo
- suggested review path
- validation commands

Important review result:

- the repository now has a short review-oriented document
- reviewers do not need to start from the full checkpoint history
- review can focus on model correctness, replay protection, proof flow, snapshot safety, and CLI safety boundaries
- no protocol logic changed
- no application state logic changed
- no CLI behavior changed
- no snapshot behavior changed

Validation after document creation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 28 test files passed
- 171 tests passed

This milestone prepares the repository for external design and implementation review.

## Latest registrar mutation order and assumptions checkpoint

The registrar mutation order and assumptions milestone was completed on the registrar-mutation-order-assumptions branch.

Commits:

- 621e8d4 Reorder registrar event mutations
- 5a8f5d1 Add registrar mutation order notes
- fd7b045 Document MVP assumptions and limitations

The milestone addresses external review findings from Theo.

Code changes:

- applyRegistrarCoreRedeem now validates amountBld before any mutation
- applyRegistrarXenBurn now validates amountXbp before any mutation
- successful Core Redeem registrar path now marks registrar message before redeem event key
- successful XEN Burn registrar path now marks registrar message before xen burn event key

Important safety result:

- invalid BLD amount does not mark messageId
- invalid BLD amount does not mark redeemKey
- invalid XBP amount does not mark messageId
- invalid XBP amount does not mark xenBurnKey
- successful registrar mutation order is now message first, event key second
- preconditions for message kind, registrar authority, duplicate message, and duplicate event key remain unchanged

Documentation changes:

- implementation/registrar-mutation-order-assumptions-notes.md
- docs/assumptions.md
- docs/review-readiness-summary.md now links to docs/assumptions.md

The assumptions document explicitly records:

- trusted indexer / registrar model
- Build ownership mapping assumption
- XNTD lock / relock registrar-level replay protection only
- lock / relock overwrite behavior
- requiredXntdLock accepted from registrar in the MVP
- no unlock flow in the MVP
- canonicalEventKey convention
- fee checkpoint finality assumption
- snapshot recovery read-only behavior
- snapshot content hash not implemented
- no production integration guarantees yet

Deferred production / post-MVP items remain:

- per-event replay protection for XNTD lock / relock
- epoch minimum validation against authoritative XC state
- unlock flow design
- snapshot content hash if needed
- live indexer / production integration hardening

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 28 test files passed
- 171 tests passed

This milestone keeps the MVP scope small while addressing the immediate low-effort / high-safety review findings.

## Latest README review entrypoint checkpoint

The README review entrypoint milestone was completed on the readme-review-entrypoint branch.

Commit:

- 1631649 Update README review entrypoint

The milestone replaces the outdated README that described the repository as documentation-only.

The README now presents the repository as a tested MVP implementation and design lab.

It adds a public review entrypoint to:

- docs/review-readiness-summary.md
- docs/assumptions.md
- docs/checkpoints/current-design-checkpoint.md

It also documents:

- current validation baseline
- CLI commands
- implemented MVP layers
- architecture overview
- core accounting concepts
- snapshot safety model
- MVP assumptions and limitations
- current non-goals
- document map
- current review posture

Important result:

- GitHub visitors now see the current repository state immediately
- reviewers no longer have to start from the full checkpoint history
- the README no longer says implementation has not started
- no source logic changed
- no tests changed
- no CLI behavior changed
- no snapshot behavior changed

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 28 test files passed
- 171 tests passed

This milestone improves the repository's public review entrypoint without changing runtime behavior.

## Latest XNTD lock event identity design checkpoint

The XNTD lock event identity design milestone was completed on the xntd-lock-event-identity-design branch.

Commits:

- 7165e11 Document XNTD lock event identity design
- e641569 Link XNTD lock event identity design

The milestone adds:

- docs/registrar/xntd-lock-event-identity.md

The milestone also links the design path from:

- docs/assumptions.md
- docs/review-readiness-summary.md

Design result:

- XNTD lock / relock event identity is documented before runtime implementation
- messageId is explicitly treated as registrar message identity, not source event identity
- lock / relock replay risk is documented as state regression risk, not double-counting risk
- a shared XntdLockEventKey model is recommended
- LOCK_XNTD and RELOCK_XNTD are distinguished through eventKind inside canonical source identity
- one shared usedXntdLockEvents replay set is recommended
- event identity should be derived from sourceChainId, sourceAddress, eventKind, transactionHash, and logIndex / eventIndex
- ordering guards are explicitly separated from per-event replay protection
- epoch minimum validation remains a separate integration requirement
- unlock replay protection is deferred until unlock is designed

Recommended future implementation sequence:

- add XntdLockEventKey types and replay state
- add low-level replay tests for XNTD lock event state
- add registrar handler tests for duplicate lock / relock event keys
- add proof and registrar payload fields
- add watcher candidate / proof conversion support
- update snapshot serialization if new state is stored
- update assumptions once implemented

Scope boundary:

- this milestone is design-only
- no runtime behavior changed
- no source code changed
- no tests changed
- no snapshot behavior changed
- no CLI behavior changed

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 28 test files passed
- 171 tests passed

This milestone prepares the XNTD lock / relock per-event replay protection design without expanding MVP runtime scope.

## Latest XNTD commitment event identity design polish checkpoint

The XNTD commitment event identity design polish milestone was completed on the xntd-lock-event-identity-design-polish branch.

Commit:

- 7b7047e Polish XNTD commitment event identity design

This milestone incorporates Theo's review feedback on the XNTD lock / relock event identity design.

Design refinements:

- renamed the recommended shared event key from XntdLockEventKey to XntdCommitmentEventKey
- renamed the replay state to XntdCommitmentEventState
- renamed the replay set to usedXntdCommitmentEvents
- clarified that the replay domain covers the full XNTD commitment state, not only the initial lock action
- added a snapshot serialization note for usedXntdCommitmentEvents
- added a Proposed ordering guard for MVP section
- recommended monotonic lockEpoch as the MVP ordering guard
- kept stricter production ordering guards as a separate future decision
- preserved epoch minimum validation as a separate integration requirement
- preserved unlock as a future design topic

Theo review conclusion:

- shared commitment event key model: approved
- eventKind inside canonical identity: approved
- ordering guard separated from per-event replay protection: approved
- epoch minimum validation kept separate: approved
- unlock awareness is sufficient for the current design phase

Important scope boundary:

- this milestone is design-only
- no runtime behavior changed
- no source code changed
- no tests changed
- no snapshot behavior changed
- no CLI behavior changed

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 28 test files passed
- 171 tests passed

This milestone finalizes the XNTD commitment event identity design before runtime implementation.

## Latest XNTD commitment event replay state checkpoint

The XNTD commitment event replay state milestone was completed on the xntd-commitment-event-replay-state branch.

Commits:

- d03736a Add XNTD commitment event replay state
- 2502fdb Add XNTD commitment event replay state notes
- 3552ac5 Persist XNTD commitment event replay state
- 7928b42 Update XNTD commitment event replay state notes
- 8e1334d Wire XNTD commitment event replay into registrar
- f77306b Update XNTD commitment registrar replay notes

The milestone implements the first runtime version of the XNTD commitment event replay model designed in:

- docs/registrar/xntd-lock-event-identity.md

Runtime additions:

- XntdCommitmentEventKey
- XntdCommitmentEventState
- usedXntdCommitmentEvents
- createXntdCommitmentEventState()
- acceptXntdCommitmentEvent()
- DuplicateXntdCommitmentEvent error code

Application state / persistence additions:

- BuildApplicationState now includes xntdCommitmentEvents
- createBuildApplicationState() initializes xntdCommitmentEvents
- SerializedXntdCommitmentEventState added
- serializeXntdCommitmentEventState() added
- deserializeXntdCommitmentEventState() added
- BuildApplicationSnapshot now includes xntdCommitmentEvents
- STORAGE_SCHEMA_VERSION bumped to 2
- snapshot round-trip preserves usedXntdCommitmentEvents

Registrar integration:

- XNTD lock / relock registrar payloads now include xntdCommitmentEventKey
- proof-submission flow derives xntdCommitmentEventKey from proof.canonicalEventKey
- appApplyRegistrarXntdLock() passes app.xntdCommitmentEvents
- appApplyRegistrarXntdRelock() passes app.xntdCommitmentEvents
- applyRegistrarXntdLock() checks duplicate xntdCommitmentEventKey before mutation
- applyRegistrarXntdRelock() checks duplicate xntdCommitmentEventKey before mutation

Successful XNTD lock / relock registrar mutation order is now:

1. acceptRegistrarMessage()
2. acceptXntdCommitmentEvent()
3. lockXntd() / relockXntd()

Safety results:

- duplicate source commitment event is rejected even with a different messageId
- duplicate source commitment event does not mark the new registrar message
- duplicate source commitment event does not mutate Build lock state
- lock and relock share one commitment replay domain
- invalid amount still does not mark registrar message or commitment event key
- invalid relock still does not mark registrar message or commitment event key
- appSubmitProof now records XNTD lock / relock canonicalEventKey in usedXntdCommitmentEvents

Tests added / updated:

- tests/xntd-commitment-event-replay.test.ts
- tests/storage-snapshot.test.ts
- tests/registrar-xntd-lock.test.ts
- tests/app-proof-submission.test.ts
- tests/app-build-service.test.ts
- tests/e2e-scenario.test.ts

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 29 test files passed
- 177 tests passed

Remaining future items:

- CLI summaries can expose xntdCommitmentEventCount
- lockEpoch ordering guard can be implemented as a separate milestone
- watcher/proof payload shapes may remain unchanged while canonicalEventKey is used as xntdCommitmentEventKey
- production epoch minimum validation remains separate

This milestone closes the main known limitation that XNTD lock / relock had only registrar-level replay protection.

## Latest CLI XNTD commitment event count checkpoint

The CLI XNTD commitment event count milestone was completed on the cli-xntd-commitment-event-count branch.

Commits:

- e548f1b Show XNTD commitment event count in CLI snapshots
- 2375ca0 Add CLI XNTD commitment event count notes

The milestone updates read-only CLI snapshot summaries to expose the XNTD commitment event replay count.

Updated CLI commands:

- snapshot:show
- snapshot:verify
- snapshot:recover

New output field:

- usedXntdCommitmentEventCount

Reason:

- xntdCommitmentEvents are now part of BuildApplicationState
- usedXntdCommitmentEvents are now persisted in snapshots
- CLI snapshot visibility should show the new replay-state count alongside:
  - processedMessageCount
  - usedRedeemEventCount
  - usedXenBurnEventCount

Scope boundary:

- CLI visibility only
- no protocol state transition changes
- no registrar behavior changes
- no proof submission behavior changes
- no watcher behavior changes
- no snapshot serialization changes
- no recovery behavior changes

Tests updated:

- tests/cli-commands.test.ts

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 29 test files passed
- 177 tests passed

This milestone keeps operator-facing snapshot summaries aligned with the XNTD commitment event replay-state model.

## Latest XNTD lock epoch ordering guard checkpoint

The XNTD lock epoch ordering guard milestone was completed on the xntd-lock-epoch-ordering-guard branch.

Commits:

- e2dacb3 Add XNTD lock epoch ordering guard
- 261a4e0 Add XNTD lock epoch ordering guard notes

This milestone adds a registrar-layer ordering guard for XNTD lock / relock commitment events.

Problem addressed:

- XNTD commitment event replay protection rejects repeated source events.
- It does not reject stale-but-unique source events by itself.
- A stale-but-unique event has a different xntdCommitmentEventKey, so it is not a replay.
- If accepted after a newer lock / relock, it could regress commitment state.

Runtime additions:

- NonIncreasingXntdLockEpoch error code
- assertIncreasingLockEpoch() registrar-layer helper

Ordering rule:

- if build.lockEpoch is null, any incoming lockEpoch is accepted
- if build.lockEpoch is not null, incoming lockEpoch must be greater than current build.lockEpoch

Conceptual rule:

- incomingLockEpoch > currentLockEpoch

The guard applies to:

- LOCK_XNTD
- RELOCK_XNTD

Mutation safety:

The ordering guard runs before:

- acceptRegistrarMessage()
- acceptXntdCommitmentEvent()
- lockXntd() / relockXntd()

Therefore stale-but-unique events do not mark:

- registrar message ID
- XNTD commitment event key

and do not mutate Build lock state.

Scope boundary:

- low-level lockXntd() / relockXntd() primitives were not changed
- proof payload shape was not changed
- watcher payload shape was not changed
- snapshot serialization was not changed
- CLI output was not changed
- XNTD amount / epoch minimum validation was not changed

Tests updated:

- tests/registrar-xntd-lock.test.ts

Test coverage added:

- stale unique LOCK_XNTD event is rejected
- stale unique RELOCK_XNTD event is rejected
- stale event does not mark new registrar message
- stale event does not mark new xntdCommitmentEventKey
- stale event does not mutate Build lock state
- accepted newer state remains unchanged

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 29 test files passed
- 179 tests passed

Remaining future option:

- production may later replace or strengthen lockEpoch ordering with source block number, finalized slot / block height, event timestamp, or a monotonic commitment version.

For MVP, monotonic lockEpoch is the accepted ordering guard.

## Latest XNTD commitment final state review checkpoint

The XNTD commitment final state documentation review milestone was completed on the review-xntd-commitment-final-state branch.

Commits:

- c8cb336 Sync XNTD commitment final state docs
- 7ccdb5c Add XNTD commitment final state review notes

This milestone synchronizes current review-facing documentation after the XNTD commitment replay and ordering guard runtime milestones.

Updated documents:

- README.md
- docs/assumptions.md
- docs/registrar/xntd-lock-event-identity.md
- implementation/review-xntd-commitment-final-state-notes.md

The review removed outdated wording that described XNTD lock / relock as having only registrar-level replay protection.

Current documented XNTD commitment protection model:

1. processedMessages
   - protects against replay of the same registrar messageId

2. usedXntdCommitmentEvents
   - protects against replay of the same source event under a different messageId

3. monotonic lockEpoch ordering guard
   - protects against stale-but-unique source events that are not replay events but could regress commitment state

README update:

- MVP assumptions now mention XNTD lock / relock source-event replay protection
- MVP assumptions now mention monotonic lockEpoch ordering guard
- the old registrar-level-only replay statement was removed

Assumptions update:

- docs/assumptions.md now describes the implemented replay / ordering model
- the remaining production consideration is stricter ordering source selection if needed
- requiredXntdLock epoch minimum validation remains a separate integration boundary

Event identity document update:

- docs/registrar/xntd-lock-event-identity.md now says the model is implemented
- the document now serves as a design-and-implementation reference
- it records the implemented path:
  - XntdCommitmentEventKey
  - usedXntdCommitmentEvents
  - snapshot persistence
  - registrar integration
  - proof canonicalEventKey usage
  - CLI summary visibility
  - monotonic lockEpoch ordering guard

Historical checkpoint note:

- older entries in docs/checkpoints/current-design-checkpoint.md were not rewritten
- they remain historical records of repository state at the time of each milestone
- later checkpoint sections document the implemented replay and ordering guard milestones

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 29 test files passed
- 179 tests passed

This milestone closes the documentation drift created by the XNTD commitment replay and ordering guard implementation work.

## Latest XNTD lock epoch minimum validation design checkpoint

The XNTD lock epoch minimum validation design milestone was completed on the required-xntd-lock-epoch-minimum-design branch.

Commits:

- 1bf7a6b Document XNTD lock epoch minimum validation design
- ec14119 Link XNTD epoch minimum validation design
- 759257e Add XNTD epoch minimum validation design notes

This milestone documents the intended production validation model for XNTD lock / relock required lock amounts.

New design document:

- docs/registrar/xntd-lock-epoch-minimum-validation.md

Implementation notes:

- implementation/xntd-lock-epoch-minimum-validation-design-notes.md

Linked documents:

- README.md
- docs/assumptions.md
- docs/registrar/xntd-lock-event-identity.md

Problem addressed:

- the current MVP runtime sets lockedXntd = amountXntd
- the current MVP runtime sets requiredXntdLock = amountXntd
- this is acceptable under the trusted registrar MVP assumption
- this is not production-complete because the runtime does not independently verify the XC epoch minimum

Intended production rule:

- requiredXntdLock = current epoch Core L1 nominal from xEnchanted Crypto

Future production validation should require:

- amountXntd > 0
- requiredXntdLock > 0
- amountXntd >= requiredXntdLock
- requiredXntdLock == authoritativeEpochMinimum(lockEpoch)

The design separates:

- actual locked amount
- required XNTD lock amount
- authoritative XC epoch minimum
- lockEpoch ordering
- production source of truth

Relationship to existing protections:

- processedMessages protects registrar message replay
- usedXntdCommitmentEvents protects source-event replay
- monotonic lockEpoch guard protects against stale-but-unique commitment events
- epoch minimum validation is separate and protects against under-locking

Scope boundary:

- design-only
- no runtime code changed
- no proof payload changed
- no watcher candidate changed
- no snapshot serialization changed
- no CLI output changed
- no tests changed

Future implementation decisions:

- decide authoritative XC state source
- decide whether requiredXntdLock is carried in payload or derived internally
- decide how lockEpoch maps to epoch minimum
- decide how finalized source context is represented
- add runtime tests for under-lock rejection and correct epoch minimum acceptance

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 29 test files passed
- 179 tests passed

This milestone keeps epoch minimum validation as the next production-readiness layer after XNTD replay and ordering safety.

## Latest XNTD required lock payload shape review checkpoint

The XNTD required lock payload shape review milestone was completed on the xntd-lock-required-amount-payload-review branch.

Commits:

- e0ae0da Define XNTD required lock payload shape decision
- dfd8166 Add XNTD required lock payload review notes

This milestone reviews the future watcher / proof / registrar payload shape for XNTD lock / relock epoch minimum validation.

Updated document:

- docs/registrar/xntd-lock-epoch-minimum-validation.md

Implementation notes:

- implementation/xntd-lock-required-amount-payload-review-notes.md

Question reviewed:

- Should LOCK_XNTD / RELOCK_XNTD payloads explicitly carry the required XNTD lock amount, or should the registrar derive it internally from authoritative XC state?

Decision:

- future watcher / proof / registrar payloads should carry observedRequiredXntdLock

Reason:

- the proof remains self-describing
- the watcher records what requirement it observed
- the registrar does not blindly trust the payload
- registrar / integration validation must verify observedRequiredXntdLock against authoritative XC state
- audit / debug / logs are clearer because submitted observed requirement and authoritative expected requirement can be compared

Future validation rule:

- amountXntd > 0
- observedRequiredXntdLock > 0
- amountXntd >= observedRequiredXntdLock
- observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

After successful validation:

- lockedXntd = amountXntd
- requiredXntdLock = observedRequiredXntdLock

Alternatives considered:

1. Registrar derives requirement internally only
   - less trust in watcher payload
   - but proof is less self-describing and audit trail is weaker

2. Payload carries requiredXntdLock directly
   - simple naming
   - but may imply the payload value is authoritative before validation

Chosen naming:

- observedRequiredXntdLock

This makes clear that the value is observed by the watcher and must still be verified.

Scope boundary:

- design / documentation only
- no proof types changed
- no watcher candidate types changed
- no proof conversion changed
- no registrar payload builders changed
- no proof submission changed
- no registrar handlers changed
- no lockXntd() / relockXntd() changes
- no runtime tests changed

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 29 test files passed
- 179 tests passed

This milestone finalizes the recommended payload shape direction before any runtime implementation of epoch minimum validation.

## Latest authoritative XC state source design checkpoint

The authoritative XC state source design milestone was completed on the authoritative-xc-state-source-design branch.

Commits:

- f30da32 Document authoritative XC state source design
- 13f824f Link authoritative XC state source design
- a7bf2de Add authoritative XC state source design notes

This milestone documents the source-of-truth model for XC epoch state used by XNTD lock / relock epoch minimum validation.

New design document:

- docs/registrar/authoritative-xc-state-source.md

Implementation notes:

- implementation/authoritative-xc-state-source-design-notes.md

Linked documents:

- README.md
- docs/assumptions.md
- docs/registrar/xntd-lock-epoch-minimum-validation.md

Problem addressed:

- future XNTD lock / relock validation requires observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)
- observedRequiredXntdLock is useful for self-describing proofs
- but observedRequiredXntdLock is not authoritative by itself
- the system needs a clear source-of-truth model for authoritativeEpochMinimum(lockEpoch)

Decision:

Use a pragmatic trusted integration path first:

1. Watcher observes XC state at a finalized Ethereum block.
2. Watcher creates LOCK_XNTD / RELOCK_XNTD candidate with:
   - amountXntd
   - observedRequiredXntdLock
   - lockEpoch
   - source block metadata
3. Candidate becomes a validated proof.
4. Registrar / integration layer verifies observedRequiredXntdLock against authoritative XC state for the same lockEpoch / source context.
5. After successful validation:
   - lockedXntd = amountXntd
   - requiredXntdLock = observedRequiredXntdLock

Short form:

- B now, C later

Authoritative source:

- Ethereum-side xEnchanted Crypto protocol state
- deployed XC Core contract
- deployed XC Lens contract, if it exposes protocol parameters safely
- finalized Ethereum block context

The authoritative source must provide or allow deriving:

- lockEpoch
- currentBaseNominal
- current Core L1 nominal
- epoch timestamp / epochAt(timestamp), if needed

Production-hardening path:

A stricter production model can introduce a separate XC epoch state checkpoint proof containing:

- xcEpochStateCheckpointId
- sourceChainId
- xcCoreAddress
- xcLensAddress, if used
- sourceBlockNumber
- finalized status
- lockEpoch
- currentBaseNominal
- authoritativeEpochMinimum

Then LOCK_XNTD / RELOCK_XNTD proofs can reference:

- xcEpochStateCheckpointId

This separates:

- XC state verification
- user lock / relock event verification

Relationship to existing protections:

- processedMessages protects registrar message replay
- usedXntdCommitmentEvents protects source-event replay
- monotonic lockEpoch guard protects against stale-but-unique commitment events
- observedRequiredXntdLock documents the observed requirement
- authoritative XC state validation verifies the observed requirement

Scope boundary:

- design-only
- no runtime code changed
- no exact contract ABI defined
- no exact XC Lens interface defined
- no live RPC implementation defined
- no Merkle proof format defined
- no X1 on-chain verification defined

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 29 test files passed
- 179 tests passed

This milestone completes the design chain before runtime implementation of XNTD epoch minimum validation.

## Latest XNTD epoch minimum runtime implementation plan checkpoint

The XNTD epoch minimum runtime implementation plan milestone was completed on the xntd-epoch-minimum-runtime-plan branch.

Commit:

- f014b98 Add XNTD epoch minimum runtime implementation plan

New implementation plan:

- implementation/xntd-epoch-minimum-runtime-plan.md

Purpose:

- define the planned runtime implementation sequence for XNTD lock / relock epoch minimum validation
- introduce observedRequiredXntdLock safely across the full runtime chain before enforcing authoritative XC epoch minimum validation
- keep this milestone documentation-only

Current runtime state:

- XNTD lock / relock runtime carries amountXntd, lockEpoch, and lockedAt / relockedAt
- low-level lockXntd() / relockXntd() currently set lockedXntd = amountXntd
- low-level lockXntd() / relockXntd() currently set requiredXntdLock = amountXntd

Target runtime state:

- amountXntd = actual amount locked / relocked by the user
- observedRequiredXntdLock = requirement observed by watcher for lockEpoch
- requiredXntdLock = Build state value recorded after registrar validation

After successful validation:

- lockedXntd = amountXntd
- requiredXntdLock = observedRequiredXntdLock

Runtime layers to update later:

1. proof types
2. watcher candidate types
3. watcher candidate constructors
4. watcher-to-proof conversion
5. registrar payload types
6. registrar payload builders
7. proof submission
8. app build service wrappers
9. registrar XNTD lock / relock handlers
10. low-level lockXntd() / relockXntd()
11. tests

Planned rollout:

- Phase 1: add observedRequiredXntdLock to payload shapes
- Phase 2: preserve MVP semantics initially by passing observedRequiredXntdLock = amountXntd
- Phase 3: split low-level lock state assignment
- Phase 4: add registrar-layer amount / observed required amount validation
- Phase 5: add authoritative XC state validation later
- Phase 6: update tests in layers

Planned validation rules before authoritative XC state integration:

- amountXntd > 0
- observedRequiredXntdLock > 0
- amountXntd >= observedRequiredXntdLock

Authoritative validation remains a later production-readiness milestone:

- observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

Snapshot impact:

- no snapshot schema change is required just because observedRequiredXntdLock is added to transient payloads
- Build state already stores lockedXntd, requiredXntdLock, and lockEpoch

CLI impact:

- no CLI output change is required initially

Scope boundary:

- documentation-only
- no runtime code changed
- no tests changed in this milestone

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 29 test files passed
- 179 tests passed

This milestone provides the controlled rollout plan for implementing observedRequiredXntdLock without mixing payload-shape changes, low-level state changes, registrar validation, and authoritative XC state validation into one large branch.

## Latest XNTD observed required lock low-level checkpoint

The XNTD observed required lock low-level runtime milestone was completed on the xntd-observed-required-lock-low-level branch.

Commits:

- b41ac2d Add observed required XNTD lock to low-level primitives
- 2a4a43d Add XNTD observed required lock low-level notes

This milestone implements the first runtime layer for observedRequiredXntdLock.

Updated runtime files:

- src/instructions/xntd-lock.ts
- src/instructions/registrar-xntd-lock.ts

Updated tests:

- tests/xntd-lock-relock.test.ts
- tests/registrar-x1-fee-checkpoint.test.ts
- tests/x1-fee-contribution.test.ts

Implementation notes:

- implementation/xntd-observed-required-lock-low-level-notes.md

Runtime change:

Before this milestone, low-level lockXntd() / relockXntd() used:

- lockedXntd = amountXntd
- requiredXntdLock = amountXntd

After this milestone, low-level lockXntd() / relockXntd() use:

- lockedXntd = amountXntd
- requiredXntdLock = observedRequiredXntdLock

New input field:

- observedRequiredXntdLock

Added to:

- LockXntdInput
- RelockXntdInput

Meaning:

- amountXntd is the actual user locked / relocked amount
- observedRequiredXntdLock is the observed requirement for the selected lockEpoch
- requiredXntdLock is the Build state value recorded after validation

New low-level validation:

- amountXntd > 0
- observedRequiredXntdLock > 0
- amountXntd >= observedRequiredXntdLock

Registrar compatibility:

- registrar XNTD lock / relock handlers still preserve current MVP behavior
- they pass observedRequiredXntdLock = amountXntd
- watcher / proof / registrar payload shape remains unchanged for now

Scope boundary:

This milestone does not change:

- proof types
- watcher candidate types
- watcher-to-proof conversion
- registrar payload builders
- proof submission payload shape
- snapshot schema
- CLI output
- authoritative XC state validation

Test coverage added:

- lock with amountXntd > observedRequiredXntdLock
- relock with amountXntd > observedRequiredXntdLock
- observedRequiredXntdLock = 0 rejection
- amountXntd < observedRequiredXntdLock lock rejection
- amountXntd < observedRequiredXntdLock relock rejection
- rejected invalid lock / relock does not mutate Build state

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 29 test files passed
- 184 tests passed

This milestone completes the lowest runtime layer of the observedRequiredXntdLock rollout while preserving MVP registrar behavior.

## Latest XNTD observed required lock registrar input checkpoint

The XNTD observed required lock registrar input runtime milestone was completed on the xntd-observed-required-lock-registrar-input branch.

Commits:

- 1c69914 Add observed required XNTD lock to registrar inputs
- c798210 Add XNTD observed required lock registrar input notes

This milestone lifts observedRequiredXntdLock from the low-level XNTD lock / relock primitives into the registrar input layer.

Updated runtime files:

- src/instructions/registrar-xntd-lock.ts
- src/app/build-service.ts
- src/app/proof-submission.ts

Updated tests:

- tests/registrar-xntd-lock.test.ts
- tests/app-build-service.test.ts
- tests/e2e-scenario.test.ts

Implementation notes:

- implementation/xntd-observed-required-lock-registrar-input-notes.md

Runtime change:

Added observedRequiredXntdLock to:

- ApplyRegistrarXntdLockInput
- ApplyRegistrarXntdRelockInput

Registrar handlers now pass:

- observedRequiredXntdLock = input.observedRequiredXntdLock

into:

- lockXntd()
- relockXntd()

Registrar validation now checks:

- amountXntd > 0
- observedRequiredXntdLock > 0
- amountXntd >= observedRequiredXntdLock

This is still not authoritative XC state validation.

Future validation remains:

- observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

Mutation safety:

- validation happens before acceptRegistrarMessage()
- validation happens before acceptXntdCommitmentEvent()
- rejected under-lock cases do not mark registrar message
- rejected under-lock cases do not mark XNTD commitment event key
- rejected under-lock cases do not mutate Build state

Compatibility layer:

- proof / watcher payload chain is not updated in this milestone
- appApplyRegistrarXntdLock() accepts optional observedRequiredXntdLock
- appApplyRegistrarXntdRelock() accepts optional observedRequiredXntdLock
- when omitted, both default observedRequiredXntdLock to amountXntd
- proof-submission passes observedRequiredXntdLock = amountXntd for now

Scope boundary:

This milestone does not change:

- proof types
- watcher candidate types
- watcher-to-proof conversion
- registrar payload builder types
- snapshot schema
- CLI output
- authoritative XC state validation

Test coverage added:

- LOCK_XNTD amount below observedRequiredXntdLock rejected
- RELOCK_XNTD amount below observedRequiredXntdLock rejected
- rejected under-lock does not mark registrar message
- rejected under-lock does not mark XNTD commitment event key
- rejected under-lock does not mutate Build state

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 29 test files passed
- 186 tests passed

This milestone completes the registrar input layer of the observedRequiredXntdLock rollout while preserving proof / watcher payload compatibility.

## Latest XNTD observed required lock registrar payload checkpoint

The XNTD observed required lock registrar payload runtime milestone was completed on the xntd-observed-required-lock-registrar-payload branch.

Commits:

- 27d7832 Add observed required XNTD lock to registrar payloads
- 99ffdd7 Add XNTD observed required lock registrar payload notes

This milestone lifts observedRequiredXntdLock into the registrar payload builder layer.

Updated runtime files:

- src/proofs/registrar-builders.ts
- src/app/proof-submission.ts

Updated tests:

- tests/proof-registrar-builders.test.ts

Implementation notes:

- implementation/xntd-observed-required-lock-registrar-payload-notes.md

Runtime change:

Added observedRequiredXntdLock to:

- XntdLockRegistrarPayload
- XntdRelockRegistrarPayload

Registrar payloads now carry:

- amountXntd
- observedRequiredXntdLock
- lockEpoch

Builder behavior:

- buildXntdLockRegistrarPayload() now sets observedRequiredXntdLock
- buildXntdRelockRegistrarPayload() now sets observedRequiredXntdLock

Compatibility behavior:

- if proof.payload.observedRequiredXntdLock exists and is bigint, builder uses it
- otherwise builder falls back to proof.payload.amountXntd
- current proof types / watcher candidates are not changed in this milestone

Proof submission change:

- LOCK_XNTD now uses lockPayload.observedRequiredXntdLock
- RELOCK_XNTD now uses relockPayload.observedRequiredXntdLock
- proof-submission no longer invents observedRequiredXntdLock from amountXntd directly

Scope boundary:

This milestone does not change:

- proof types
- watcher candidate types
- watcher-to-proof conversion
- app proof submission tests
- e2e tests
- snapshot schema
- CLI output
- authoritative XC state validation

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 29 test files passed
- 186 tests passed

This milestone completes the registrar payload layer of the observedRequiredXntdLock rollout while preserving proof / watcher payload compatibility.

## Latest XNTD observed required lock proof payload checkpoint

The XNTD observed required lock proof payload runtime milestone was completed on the xntd-observed-required-lock-proof-payload branch.

Commits:

- d8f7e81 Add observed required XNTD lock to proof payloads
- 0009c37 Add XNTD observed required lock proof payload notes

This milestone lifts observedRequiredXntdLock into the XNTD proof payload layer.

Updated runtime files:

- src/proofs/proof-types.ts
- src/watchers/proof-conversion.ts
- src/proofs/registrar-builders.ts

Updated tests:

- tests/watcher-proof-conversion.test.ts
- tests/proof-registrar-builders.test.ts

Implementation notes:

- implementation/xntd-observed-required-lock-proof-payload-notes.md

Runtime change:

Added observedRequiredXntdLock to:

- XntdLockProof.payload
- XntdRelockProof.payload

Proof payloads now carry:

- amountXntd
- observedRequiredXntdLock
- lockEpoch

Proof conversion behavior:

- convertXntdLockCandidateToProof() now sets observedRequiredXntdLock
- convertXntdRelockCandidateToProof() now sets observedRequiredXntdLock
- if candidate payload already contains observedRequiredXntdLock as bigint, conversion uses it
- otherwise conversion falls back to amountXntd

Compatibility reason:

- watcher candidate types are not updated in this milestone
- proof conversion keeps compatibility with current watcher candidates
- watcher candidate update remains a later layer

Registrar builder change:

- buildXntdLockRegistrarPayload() now reads proof.payload.observedRequiredXntdLock directly
- buildXntdRelockRegistrarPayload() now reads proof.payload.observedRequiredXntdLock directly
- the temporary registrar-builder unknown-field fallback helper was removed

Test coverage:

- XNTD lock proof payload contains observedRequiredXntdLock
- XNTD relock proof payload contains observedRequiredXntdLock
- registrar payload preserves separated values where amountXntd > observedRequiredXntdLock

Scope boundary:

This milestone does not change:

- watcher candidate types
- watcher candidate constructors
- app proof submission tests
- e2e tests
- snapshot schema
- CLI output
- authoritative XC state validation

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 29 test files passed
- 186 tests passed

This milestone completes the proof payload layer of the observedRequiredXntdLock rollout while preserving watcher candidate compatibility.

## Latest XNTD observed required lock watcher candidate checkpoint

The XNTD observed required lock watcher candidate runtime milestone was completed on the xntd-observed-required-lock-watcher-candidate branch.

Commits:

- 52d2113 Add observed required XNTD lock to watcher candidates
- 5285319 Add XNTD observed required lock watcher candidate notes

This milestone lifts observedRequiredXntdLock into the watcher candidate layer.

Updated runtime files:

- src/watchers/watcher-candidates.ts
- src/watchers/proof-conversion.ts

Updated tests:

- tests/watcher-candidates.test.ts
- tests/watcher-proof-conversion.test.ts
- tests/app-proof-submission.test.ts
- tests/e2e-watcher-proof-registrar-scenario.test.ts

Implementation notes:

- implementation/xntd-observed-required-lock-watcher-candidate-notes.md

Runtime change:

Added observedRequiredXntdLock to:

- XntdLockCandidate.payload
- XntdRelockCandidate.payload

Watcher candidates now carry:

- amountXntd
- observedRequiredXntdLock
- lockEpoch

Constructor change:

- createXntdLockCandidate() now includes observedRequiredXntdLock in candidate payloads
- createXntdRelockCandidate() now includes observedRequiredXntdLock in candidate payloads

Proof conversion change:

- convertXntdLockCandidateToProof() now reads candidate.payload.observedRequiredXntdLock directly
- convertXntdRelockCandidateToProof() now reads candidate.payload.observedRequiredXntdLock directly
- the temporary proof-conversion fallback helper was removed

Full runtime propagation chain after this milestone:

watcher candidate
-> proof payload
-> registrar payload
-> proof submission
-> registrar input
-> low-level lock / relock
-> Build state requiredXntdLock

Test coverage:

The app proof submission test now verifies separated values through the full chain.

LOCK_XNTD:

- amountXntd = 750
- observedRequiredXntdLock = 500
- lockedXntd = 750
- requiredXntdLock = 500

RELOCK_XNTD:

- amountXntd = 400
- observedRequiredXntdLock = 250
- lockedXntd = 400
- requiredXntdLock = 250

Scope boundary:

This milestone does not change:

- snapshot schema
- CLI output
- authoritative XC state validation
- XC state source integration
- proof source metadata
- event identity model

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 29 test files passed
- 186 tests passed

This milestone completes the runtime propagation chain for observedRequiredXntdLock from watcher candidate to Build state.

## Latest observed required XNTD lock chain review checkpoint

The observed required XNTD lock chain review milestone was completed on the review-observed-required-xntd-lock-chain branch.

Commits:

- 10e7e29 Remove observed required XNTD lock app fallback
- cef117b Add observed required XNTD lock chain review notes

This milestone reviews the completed observedRequiredXntdLock runtime propagation chain and removes the remaining app-layer compatibility fallback.

Updated runtime file:

- src/app/build-service.ts

Implementation notes:

- implementation/review-observed-required-xntd-lock-chain-notes.md

Cleanup performed:

Before this milestone, appApplyRegistrarXntdLock() and appApplyRegistrarXntdRelock() still used:

- observedRequiredXntdLock = input.observedRequiredXntdLock ?? input.amountXntd

After this milestone, they use:

- observedRequiredXntdLock = input.observedRequiredXntdLock

directly.

Reason:

- observedRequiredXntdLock now flows through the full runtime chain
- app-service should no longer silently derive observedRequiredXntdLock from amountXntd
- the field must be explicit by the time execution reaches app-level registrar wrappers

Current explicit runtime flow:

watcher candidate
-> proof payload
-> registrar payload
-> proof submission
-> app service
-> registrar input
-> low-level lock / relock
-> Build state requiredXntdLock

Current runtime invariant:

- lockedXntd = amountXntd
- requiredXntdLock = observedRequiredXntdLock
- amountXntd must be >= observedRequiredXntdLock

Historical docs note:

Some older implementation notes and checkpoint sections still mention earlier temporary states such as:

- requiredXntdLock = amountXntd
- observedRequiredXntdLock = amountXntd
- fallback behavior

Those historical notes are intentionally not rewritten because they describe past milestones and compatibility phases.

Current runtime behavior is represented by the latest checkpoints and current source code.

Scope boundary:

This review does not implement authoritative XC validation.

Future production validation still needs:

- observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

That belongs to the registrar / integration boundary using the authoritative XC state source.

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 29 test files passed
- 186 tests passed

This milestone closes the post-rollout cleanup for the observedRequiredXntdLock runtime chain.

## Latest XNTD epoch minimum docs after runtime propagation checkpoint

The XNTD epoch minimum documentation update milestone was completed on the update-xntd-epoch-minimum-docs-after-runtime-chain branch.

Commits:

- fea7521 Update XNTD epoch minimum docs after runtime propagation
- da1a9e7 Add XNTD epoch minimum docs update notes

This milestone updates active design documentation after the observedRequiredXntdLock runtime propagation chain was completed.

Updated documents:

- docs/assumptions.md
- docs/registrar/xntd-lock-epoch-minimum-validation.md

Implementation notes:

- implementation/update-xntd-epoch-minimum-docs-after-runtime-chain-notes.md

Main documentation correction:

The previous active docs still described the older MVP equality model:

- requiredXntdLock = amountXntd

That is no longer the current runtime behavior.

The active docs now describe the current runtime behavior:

- lockedXntd = amountXntd
- requiredXntdLock = observedRequiredXntdLock

Current runtime validation:

- amountXntd > 0
- observedRequiredXntdLock > 0
- amountXntd >= observedRequiredXntdLock

Current explicit runtime flow:

watcher candidate
-> proof payload
-> registrar payload
-> proof submission
-> app service
-> registrar input
-> low-level lock / relock
-> Build state

Still not implemented:

This milestone does not implement authoritative XC validation.

The remaining production-readiness rule is still:

- observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

That validation belongs to the registrar / integration boundary using the authoritative XC state source.

Historical docs note:

Older implementation notes and older checkpoint sections are not rewritten.

They intentionally preserve the history of earlier rollout phases.

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 29 test files passed
- 186 tests passed

This milestone aligns the active epoch-minimum design docs with the completed observedRequiredXntdLock runtime propagation chain.

## Latest authoritative XC epoch minimum validation runtime plan checkpoint

The authoritative XC epoch minimum validation runtime plan milestone was completed on the authoritative-xc-epoch-minimum-validation-runtime-plan branch.

Commit:

- fdf3f8e Add authoritative XC epoch minimum validation runtime plan

This milestone defines the next runtime implementation path for validating XNTD lock / relock required amounts against authoritative XC epoch state.

Implementation plan:

- implementation/authoritative-xc-epoch-minimum-validation-runtime-plan.md

This is a plan-only milestone.

It does not change runtime code.

Current completed state:

The observedRequiredXntdLock runtime propagation chain is complete.

Current explicit runtime flow:

watcher candidate
-> proof payload
-> registrar payload
-> proof submission
-> app service
-> registrar input
-> low-level lock / relock
-> Build state

Current runtime records:

- lockedXntd = amountXntd
- requiredXntdLock = observedRequiredXntdLock

Current runtime validation checks:

- amountXntd > 0
- observedRequiredXntdLock > 0
- amountXntd >= observedRequiredXntdLock
- monotonic lockEpoch ordering
- registrar message replay protection
- XNTD commitment event replay protection

Remaining production-readiness gap:

- observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

Recommended next runtime layer:

- do not start with live Ethereum RPC
- introduce a local deterministic authoritative epoch minimum source / validator
- validate at the registrar / integration boundary
- keep low-level lockXntd() / relockXntd() as deterministic state transition helpers

Conceptual source:

- authoritativeEpochMinimum(lockEpoch: number): bigint | null

Registrar validation rule:

- reject if authoritative minimum for lockEpoch is missing
- reject if observedRequiredXntdLock != authoritativeEpochMinimum(lockEpoch)
- perform this validation before acceptRegistrarMessage()
- perform this validation before acceptXntdCommitmentEvent()
- perform this validation before lockXntd() / relockXntd()

Mutation safety requirement:

Rejected authoritative epoch minimum validation must not mutate:

- registrar.processedMessages
- xntdCommitmentEvents.usedXntdCommitmentEvents
- Build state

Recommended implementation style:

- start with direct registrar-level tests
- do not update snapshot schema yet
- do not persist any XC epoch source in BuildApplicationState yet
- decide app-level injection only after registrar-level validation is correct and tested

Non-goals for the first runtime layer:

- real Ethereum RPC reads
- XC Core ABI integration
- XC Lens ABI integration
- finalized block verification
- Merkle proofs
- X1 on-chain verification
- persisted epoch checkpoint storage
- snapshot schema migration

Validation target for this plan milestone:

- npm run typecheck
- npm test
- npm run build
- npm audit --audit-level=moderate

This milestone prepares the next implementation layer without increasing runtime complexity prematurely.

## Latest authoritative XC epoch minimum source checkpoint

The authoritative XC epoch minimum source milestone was completed on the authoritative-xc-epoch-minimum-source branch.

Commits:

- 050c1cd Add authoritative XC epoch minimum source helper
- ea4f099 Add authoritative XC epoch minimum source notes

This milestone introduces a deterministic local source / helper for authoritative XC epoch minimum validation.

Runtime additions:

- src/model/xc-epoch-minimum-source.ts

Exports:

- XcEpochMinimumSource
- createStaticXcEpochMinimumSource()
- assertAuthoritativeXcEpochMinimum()

Error codes added:

- MissingAuthoritativeXcEpochMinimum
- MismatchedAuthoritativeXcEpochMinimum

These errors are intentionally separate from InvalidXntdLockAmount.

Reason:

- InvalidXntdLockAmount covers invalid amount relationships
- MissingAuthoritativeXcEpochMinimum covers unknown / unavailable authoritative source state
- MismatchedAuthoritativeXcEpochMinimum covers economically incorrect observed required lock value

Helper behavior:

assertAuthoritativeXcEpochMinimum(source, lockEpoch, observedRequiredXntdLock):

1. Reads authoritativeEpochMinimum(lockEpoch) from the source.
2. Rejects if the source returns null.
3. Rejects if observedRequiredXntdLock does not equal the authoritative minimum.
4. Accepts if the observed value matches the authoritative minimum.

Test coverage:

- tests/xc-epoch-minimum-source.test.ts

Covered cases:

- matching observed required XNTD lock is accepted
- missing authoritative epoch minimum is rejected
- mismatched observed required XNTD lock is rejected

Scope boundary:

This milestone does not change:

- registrar XNTD lock / relock handlers
- app proof submission
- app service
- watcher candidates
- proof payloads
- registrar payloads
- snapshot schema
- CLI output
- real Ethereum RPC integration
- XC Core / Lens ABI integration

Next step:

Connect assertAuthoritativeXcEpochMinimum() to the registrar XNTD lock / relock boundary.

The validator should run before:

- acceptRegistrarMessage()
- acceptXntdCommitmentEvent()
- lockXntd() / relockXntd()

Rejected authoritative validation must not mutate:

- registrar.processedMessages
- xntdCommitmentEvents.usedXntdCommitmentEvents
- Build state

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 30 test files passed
- 189 tests passed

This milestone adds the reusable validation building block without increasing registrar runtime coupling yet.

## Latest authoritative XC epoch minimum registrar validation checkpoint

The authoritative XC epoch minimum registrar validation milestone was completed on the authoritative-xc-epoch-minimum-registrar-validation branch.

Commits:

- 967070f Validate XNTD registrar locks against authoritative XC minimum
- 1445d28 Add authoritative XC epoch minimum registrar validation notes

This milestone connects authoritative XC epoch minimum validation to the registrar XNTD lock / relock boundary.

Updated runtime file:

- src/instructions/registrar-xntd-lock.ts

Updated tests:

- tests/registrar-xntd-lock.test.ts

Implementation notes:

- implementation/authoritative-xc-epoch-minimum-registrar-validation-notes.md

Runtime change:

Added optional input field to:

- ApplyRegistrarXntdLockInput
- ApplyRegistrarXntdRelockInput

Field:

- xcEpochMinimumSource?: XcEpochMinimumSource

Validation behavior:

When xcEpochMinimumSource is provided, applyRegistrarXntdLock() and applyRegistrarXntdRelock() call:

assertAuthoritativeXcEpochMinimum(
  xcEpochMinimumSource,
  lockEpoch,
  observedRequiredXntdLock
)

This validates:

- observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

Compatibility behavior:

xcEpochMinimumSource is optional in this milestone.

Reason:

- app service / proof submission / e2e call sites are not updated yet
- the registrar boundary can support authoritative validation without forcing the whole app stack to change in the same branch
- a later layer can decide how to pass the source through app service and proof submission

Mutation safety:

Authoritative XC epoch minimum validation runs before:

- acceptRegistrarMessage()
- acceptXntdCommitmentEvent()
- lockXntd() / relockXntd()

Therefore rejected authoritative validation must not mutate:

- registrar.processedMessages
- xntdCommitmentEvents.usedXntdCommitmentEvents
- Build state

Test coverage:

- LOCK_XNTD accepts when observedRequiredXntdLock matches authoritative epoch minimum
- LOCK_XNTD rejects when observedRequiredXntdLock mismatches authoritative epoch minimum
- LOCK_XNTD rejects when authoritative epoch minimum is missing
- rejection does not mutate registrar processed messages
- rejection does not mark XNTD commitment event keys
- rejection does not mutate Build lock state

Scope boundary:

This milestone does not update:

- app service source injection
- proof submission source injection
- e2e source injection
- snapshot schema
- CLI output
- real Ethereum RPC integration
- XC Core / Lens ABI integration

Next step:

The next layer should decide how to pass xcEpochMinimumSource above the registrar instruction boundary.

Likely options:

1. Add explicit source argument to appApplyRegistrarXntdLock() / appApplyRegistrarXntdRelock().
2. Add explicit source argument to appSubmitProof().
3. Later decide whether BuildApplicationState should own a source provider.

Do not persist the source in snapshots yet.

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 30 test files passed
- 192 tests passed

This milestone closes the registrar-level authoritative validation hook while preserving app-level compatibility.

## Latest authoritative XC epoch minimum app service injection checkpoint

The authoritative XC epoch minimum app service injection milestone was completed on the authoritative-xc-epoch-minimum-app-service-injection branch.

Commits:

- b6898e4 Pass authoritative XC epoch minimum source through app service
- 7a25bba Add authoritative XC epoch minimum app service notes

This milestone passes the optional authoritative XC epoch minimum source through the application service XNTD lock / relock wrappers.

Updated runtime file:

- src/app/build-service.ts

Updated tests:

- tests/app-build-service.test.ts

Implementation notes:

- implementation/authoritative-xc-epoch-minimum-app-service-injection-notes.md

Runtime change:

The following app service functions now forward xcEpochMinimumSource when provided:

- appApplyRegistrarXntdLock()
- appApplyRegistrarXntdRelock()

Forwarding uses conditional object spread:

- if xcEpochMinimumSource is provided, it is passed to the registrar handler
- if xcEpochMinimumSource is undefined, the field is omitted

This preserves compatibility with exactOptionalPropertyTypes.

Validation behavior:

When a caller provides xcEpochMinimumSource:

appApplyRegistrarXntdLock()
-> applyRegistrarXntdLock()
-> assertAuthoritativeXcEpochMinimum()

and:

appApplyRegistrarXntdRelock()
-> applyRegistrarXntdRelock()
-> assertAuthoritativeXcEpochMinimum()

Therefore app-service callers can now trigger registrar-level validation of:

- observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

Compatibility behavior:

xcEpochMinimumSource remains optional.

Existing app-service call sites that do not pass the source continue to work.

Test coverage:

- appApplyRegistrarXntdLock() succeeds when the source contains the matching epoch minimum
- appApplyRegistrarXntdRelock() returns a structured error when the source is missing the relock epoch minimum
- rejected app-service relock does not mark the registrar message as processed
- rejected app-service relock does not mark the XNTD commitment event key as used
- rejected app-service relock does not mutate Build lockedXntd, requiredXntdLock, or lockEpoch

Scope boundary:

This milestone does not update:

- appSubmitProof()
- proof submission payload flow
- watcher proof conversion
- registrar payload builders
- snapshot schema
- storage serialization
- CLI output
- real Ethereum RPC integration
- XC Core / Lens ABI integration

Next step:

The next layer should decide how proof submission receives / owns the authoritative XC epoch minimum source.

Likely next step:

- add optional xcEpochMinimumSource to appSubmitProof() input
- pass it into appApplyRegistrarXntdLock() / appApplyRegistrarXntdRelock()
- add app-proof-submission tests

Do not persist the source in snapshots yet.

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 30 test files passed
- 193 tests passed

This milestone extends authoritative validation one layer upward while keeping proof submission and persistence unchanged.

## Latest authoritative XC epoch minimum proof submission injection checkpoint

The authoritative XC epoch minimum proof submission injection milestone was completed on the authoritative-xc-epoch-minimum-proof-submission-injection branch.

Commits:

- 1c04f2d Pass authoritative XC epoch minimum source through proof submission
- 7d7581e Add authoritative XC epoch minimum proof submission notes

This milestone passes the optional authoritative XC epoch minimum source through appSubmitProof() for XNTD lock / relock proof submission.

Updated runtime file:

- src/app/proof-submission.ts

Updated tests:

- tests/app-proof-submission.test.ts

Implementation notes:

- implementation/authoritative-xc-epoch-minimum-proof-submission-injection-notes.md

Runtime change:

AppSubmitProofInput now supports:

- xcEpochMinimumSource?: XcEpochMinimumSource

For XNTD proof kinds:

- XNTD_LOCK_PROOF
- XNTD_RELOCK_PROOF

appSubmitProof() forwards xcEpochMinimumSource into:

- appApplyRegistrarXntdLock()
- appApplyRegistrarXntdRelock()

Forwarding uses conditional object spread:

- if xcEpochMinimumSource is provided, it is passed down
- if xcEpochMinimumSource is undefined, the field is omitted

This preserves compatibility with exactOptionalPropertyTypes.

Validation behavior:

When proof submission receives xcEpochMinimumSource:

appSubmitProof()
-> appApplyRegistrarXntdLock() / appApplyRegistrarXntdRelock()
-> applyRegistrarXntdLock() / applyRegistrarXntdRelock()
-> assertAuthoritativeXcEpochMinimum()

Therefore proof submission callers can now trigger validation of:

- observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

Compatibility behavior:

xcEpochMinimumSource remains optional.

Existing proof submission call sites that do not pass the source continue to work.

Test coverage:

- appSubmitProof() accepts an XNTD_LOCK_PROOF when the source contains the matching epoch minimum
- appSubmitProof() rejects an XNTD_RELOCK_PROOF when the source is missing the relock epoch minimum
- rejected relock proof submission returns a structured app error
- rejected relock proof submission does not mark the registrar message as processed
- rejected relock proof submission does not mark the XNTD commitment event key as used
- rejected relock proof submission does not mutate Build lockedXntd, requiredXntdLock, or lockEpoch

Scope boundary:

This milestone does not update:

- watcher proof conversion
- registrar payload builders
- snapshot schema
- storage serialization
- CLI output
- real Ethereum RPC integration
- XC Core / Lens ABI integration
- persistent app state source ownership

Current validation chain:

The optional authoritative source can now flow through:

appSubmitProof()
-> appApplyRegistrarXntdLock() / appApplyRegistrarXntdRelock()
-> applyRegistrarXntdLock() / applyRegistrarXntdRelock()
-> assertAuthoritativeXcEpochMinimum()

Next step:

The next layer should decide whether to add the authoritative source to the e2e watcher-proof-registrar scenario.

Potential next step:

- add source injection to tests/e2e-watcher-proof-registrar-scenario.test.ts
- keep it test-only
- do not persist the source in snapshots
- do not introduce real RPC yet

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 30 test files passed
- 194 tests passed

This milestone extends authoritative validation into proof submission while keeping persistence and external integrations unchanged.

## Latest authoritative XC epoch minimum e2e scenario checkpoint

The authoritative XC epoch minimum e2e scenario milestone was completed on the authoritative-xc-epoch-minimum-e2e-scenario branch.

Commits:

- 4d29d10 Use authoritative XC epoch minimum source in e2e proof scenario
- ed3558c Add authoritative XC epoch minimum e2e scenario notes

This milestone wires the deterministic authoritative XC epoch minimum source into the end-to-end watcher proof registrar scenario.

Updated test:

- tests/e2e-watcher-proof-registrar-scenario.test.ts

Implementation notes:

- implementation/authoritative-xc-epoch-minimum-e2e-scenario-notes.md

Runtime code changes:

This milestone does not change runtime code.

It only strengthens the existing e2e test scenario.

Test source:

The e2e scenario now creates a deterministic static source:

- epoch 1 -> 500n
- epoch 2 -> 250n

This source is passed into appSubmitProof() for:

- XNTD_LOCK_PROOF
- XNTD_RELOCK_PROOF

Verified path:

The existing e2e scenario now validates that XNTD lock / relock proof submission can flow through:

1. watcher candidate creation
2. proof conversion
3. appSubmitProof()
4. app service wrapper
5. registrar lock / relock handler
6. authoritative epoch minimum validation
7. Build state mutation

Compatibility:

Non-XNTD proof submissions in the same scenario are unchanged:

- CORE_REDEEM
- XEN_BURN
- X1_FEE_CHECKPOINT

Scope boundary:

This milestone does not update:

- app runtime code
- proof submission runtime code
- registrar runtime code
- snapshot schema
- storage serialization
- CLI output
- real Ethereum RPC integration
- XC Core / Lens ABI integration
- persistent app state source ownership

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 30 test files passed
- 194 tests passed

Current authoritative validation chain covered in tests:

watcher candidate
-> proof conversion
-> appSubmitProof(..., xcEpochMinimumSource)
-> app service
-> registrar handler
-> assertAuthoritativeXcEpochMinimum()
-> Build state

Next step:

A later production-readiness layer can decide how the authoritative source is created in real integration:

- trusted integration source
- finalized Ethereum RPC / Lens read
- checkpoint source
- bridge-provided source
- X1-native verified source

Do not introduce real RPC or persistent source ownership in this e2e test milestone.

This milestone completes deterministic test coverage for the authoritative XC epoch minimum validation chain from watcher candidate to Build state.

## Latest authoritative XC epoch minimum chain review checkpoint

The authoritative XC epoch minimum chain review milestone was completed on the review-authoritative-xc-epoch-minimum-chain branch.

Commit:

- 0fb23f8 Add authoritative XC epoch minimum chain review notes

This milestone reviews the completed authoritative XC epoch minimum validation chain after the source/helper, registrar, app-service, proof-submission, and e2e scenario milestones.

Implementation notes:

- implementation/review-authoritative-xc-epoch-minimum-chain-notes.md

This is a review-only milestone.

It does not change runtime code.

Current runtime chain:

watcher candidate
-> proof conversion
-> appSubmitProof(..., xcEpochMinimumSource)
-> appApplyRegistrarXntdLock() / appApplyRegistrarXntdRelock()
-> applyRegistrarXntdLock() / applyRegistrarXntdRelock()
-> assertAuthoritativeXcEpochMinimum()
-> Build state

Current validation rule:

When xcEpochMinimumSource is provided, XNTD lock / relock validates:

- observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

The source is optional at the integration boundary.

If provided, validation is enforced.

If not provided, compatibility behavior is preserved for existing call sites.

Mutation safety:

Authoritative validation occurs before:

- acceptRegistrarMessage()
- acceptXntdCommitmentEvent()
- lockXntd() / relockXntd()

Rejected authoritative validation does not mutate:

- registrar.processedMessages
- xntdCommitmentEvents.usedXntdCommitmentEvents
- Build lock state

Review findings:

The review grep found historical references to older stages, including:

- requiredXntdLock = amountXntd
- observedRequiredXntdLock = amountXntd
- fallback behavior
- trusted registrar MVP language

These references are mostly in historical implementation notes and older checkpoint sections.

They should remain as history unless they appear in active current-state docs.

Active runtime status:

Active runtime code no longer relies on the old equality model as the source of truth for requiredXntdLock.

Current flow separates:

- lockedXntd = amountXntd
- requiredXntdLock = observedRequiredXntdLock

And, when a source is provided, validates:

- observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

Active docs status:

The active docs already describe the current state correctly:

- docs/assumptions.md
- docs/registrar/xntd-lock-epoch-minimum-validation.md
- docs/registrar/authoritative-xc-state-source.md
- docs/checkpoints/current-design-checkpoint.md latest sections

No active doc update is required in this review layer.

Test coverage status:

Coverage now includes:

- source/helper unit tests
- registrar lock validation tests
- app-service source injection tests
- proof-submission source injection tests
- e2e watcher-proof-registrar scenario with deterministic source

Current validation count:

- 30 test files passed
- 194 tests passed

Scope boundary:

This review does not introduce:

- real Ethereum RPC integration
- XC Core / Lens ABI integration
- snapshot schema changes
- storage serialization changes
- CLI changes
- persistent app-state source ownership

Next production-readiness step:

The next meaningful step is not another propagation layer.

The next step should be a design/implementation decision for how the authoritative source is created in real integration:

- trusted integration source
- finalized Ethereum RPC / Lens read
- checkpoint source
- bridge-provided source
- X1-native verified source

Until that decision is made, the deterministic source remains the correct test/runtime boundary.

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 30 test files passed
- 194 tests passed

This milestone closes the review of the deterministic authoritative XC epoch minimum validation chain.

## Latest authoritative XC epoch minimum production source plan checkpoint

The authoritative XC epoch minimum production source plan milestone was completed on the authoritative-xc-epoch-minimum-production-source-plan branch.

Commit:

- b519fab Add authoritative XC epoch minimum production source plan

This milestone defines production-source options for authoritative XC epoch minimum validation.

Implementation plan:

- implementation/authoritative-xc-epoch-minimum-production-source-plan.md

This is a plan-only milestone.

It does not change runtime code.

Current completed validation chain:

watcher candidate
-> proof conversion
-> appSubmitProof(..., xcEpochMinimumSource)
-> app service
-> registrar handler
-> assertAuthoritativeXcEpochMinimum()
-> Build state

Current runtime validation rule:

- observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

when an XcEpochMinimumSource is provided.

Remaining production question:

The remaining question is not how to validate.

The remaining question is where the production authoritative source comes from.

Production source requirement:

A production source must answer:

What was the authoritative XC Core L1 minimum for this lockEpoch?

It must not be derived from:

- amountXntd
- observedRequiredXntdLock itself

It must come from an independent authoritative XC state source.

Production source options reviewed:

1. Trusted integration source
2. Finalized Ethereum RPC / XC Lens read
3. Checkpoint source
4. Bridge-provided source
5. X1-native verified source

Recommended first production-like path:

Use trusted integration source first.

Then evolve to:

- finalized Ethereum RPC / XC Lens read
- checkpoint source

Reason:

The current runtime already has the correct boundary:

appSubmitProof(..., xcEpochMinimumSource)

This means production can start by creating a reliable source outside the runtime and passing it in.

Do not persist the source in BuildApplicationState yet.

Do not add snapshot schema changes yet.

Do not introduce live RPC directly into core app state yet.

Recommended near-term sequence:

1. Keep deterministic source for unit/e2e tests.
2. Add a production-source adapter design document.
3. Define finalized block policy.
4. Define XC Core / Lens read fields.
5. Define failure behavior:
   - missing epoch
   - stale source
   - RPC unavailable
   - mismatched minimum
6. Add adapter tests with mocked XC state.
7. Only then decide whether to wire adapter into CLI / service runtime.

Failure policy:

If authoritative minimum is unavailable:

- reject proof submission
- do not mark registrar message as processed
- do not mark XNTD commitment event as used
- do not mutate Build lock state

If observedRequiredXntdLock mismatches authoritative source:

- reject proof submission
- preserve mutation safety
- report explicit mismatch error

Current runtime already supports these error categories:

- MissingAuthoritativeXcEpochMinimum
- MismatchedAuthoritativeXcEpochMinimum

Non-goals:

This plan does not implement:

- real Ethereum RPC reads
- XC Core / Lens ABI integration
- bridge signer logic
- X1 on-chain verification
- snapshot migration
- CLI integration
- persistent app-state source ownership

Current conclusion:

The deterministic validation chain is complete.

The next production-readiness decision is source ownership, not validation mechanics.

Recommended first path:

trusted integration source -> finalized Ethereum RPC / Lens read or checkpoint source

Long-term path:

bridge-provided or X1-native verified source

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 30 test files passed
- 194 tests passed

## Latest authoritative XC epoch minimum stage summary checkpoint

The authoritative XC epoch minimum stage summary milestone was completed on the authoritative-xc-epoch-minimum-stage-summary branch.

Commit:

- 378533c Add authoritative XC epoch minimum stage summary

This milestone summarizes the completed authoritative XC epoch minimum validation stage.

Implementation summary:

- implementation/authoritative-xc-epoch-minimum-stage-summary.md

This is a summary-only milestone.

It does not change runtime code.

Completed line:

1. observedRequiredXntdLock propagation
2. fallback cleanup
3. authoritative source/helper
4. registrar validation
5. app service source injection
6. proof submission source injection
7. e2e watcher-proof-registrar coverage
8. chain review
9. production source plan

Current deterministic validation chain:

watcher candidate
-> proof conversion
-> appSubmitProof(..., xcEpochMinimumSource)
-> appApplyRegistrarXntdLock() / appApplyRegistrarXntdRelock()
-> applyRegistrarXntdLock() / applyRegistrarXntdRelock()
-> assertAuthoritativeXcEpochMinimum()
-> Build state

Current runtime rule:

When xcEpochMinimumSource is provided, XNTD lock / relock validates:

- observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

Current state separation:

- lockedXntd = amountXntd
- requiredXntdLock = observedRequiredXntdLock

Current source interface:

- XcEpochMinimumSource
- createStaticXcEpochMinimumSource()
- assertAuthoritativeXcEpochMinimum()

Current explicit errors:

- MissingAuthoritativeXcEpochMinimum
- MismatchedAuthoritativeXcEpochMinimum

Mutation safety:

Authoritative validation runs before:

- acceptRegistrarMessage()
- acceptXntdCommitmentEvent()
- lockXntd() / relockXntd()

Rejected validation does not mutate:

- registrar.processedMessages
- xntdCommitmentEvents.usedXntdCommitmentEvents
- Build lock state

Coverage now includes:

- source/helper unit tests
- registrar lock validation tests
- app-service source injection tests
- proof-submission source injection tests
- e2e watcher-proof-registrar scenario with deterministic source

Current validation count:

- 30 test files passed
- 194 tests passed

Compatibility boundary:

xcEpochMinimumSource remains optional.

If provided:

- authoritative validation is enforced

If not provided:

- compatibility behavior is preserved for existing call sites

This keeps deterministic validation available without forcing source ownership into app state or snapshots.

This stage intentionally did not add:

- real Ethereum RPC reads
- XC Core / Lens ABI integration
- bridge signer logic
- X1 on-chain verification
- snapshot schema migration
- CLI integration
- persistent app-state source ownership

Production source conclusion:

The deterministic validation mechanics are complete.

The next production-readiness question is source ownership:

Where does authoritativeEpochMinimum(lockEpoch) come from in real integration?

Production source options documented:

1. trusted integration source
2. finalized Ethereum RPC / XC Lens read
3. checkpoint source
4. bridge-provided source
5. X1-native verified source

Recommended first production-like path:

trusted integration source
-> finalized Ethereum RPC / XC Lens read or checkpoint source

Long-term path:

bridge-provided source or X1-native verified source

Recommended next stage:

authoritative-xc-epoch-minimum-production-source-adapter-design

Scope:

- define finalized block policy
- define XC Core / Lens read fields
- define adapter interface
- define mocked adapter tests
- define failure behavior for stale / missing / mismatched source
- do not add real RPC yet
- do not change snapshot schema yet

Current conclusion:

The authoritative XC epoch minimum validation chain is now ready as a deterministic runtime boundary.

The project can safely move from validation mechanics to production source design.

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities

## Latest authoritative XC epoch minimum production source adapter design checkpoint

The authoritative XC epoch minimum production source adapter design milestone was completed on the authoritative-xc-epoch-minimum-production-source-adapter-design branch.

Commit:

- e1ec9b3 Add authoritative XC epoch minimum source adapter design

This milestone designs the next production-readiness layer for authoritative XC epoch minimum validation.

Design document:

- implementation/authoritative-xc-epoch-minimum-production-source-adapter-design.md

This is a design-only milestone.

It does not change runtime code.

Current completed boundary:

The deterministic validation chain is already complete:

watcher candidate
-> proof conversion
-> appSubmitProof(..., xcEpochMinimumSource)
-> app service
-> registrar handler
-> assertAuthoritativeXcEpochMinimum()
-> Build state

Current runtime accepts:

XcEpochMinimumSource {
  authoritativeEpochMinimum(lockEpoch: number): bigint | null;
}

If provided, the runtime validates:

- observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

Adapter goal:

The adapter should turn external XC epoch state into a deterministic XcEpochMinimumSource.

Conceptual responsibility:

external XC state
-> validated epoch minimum records
-> XcEpochMinimumSource

The adapter must not derive the required lock from:

- amountXntd
- observedRequiredXntdLock
- user-provided lock amount
- mutable Build state

The adapter must derive the epoch minimum from an independent XC state source.

Recommended first adapter type:

Start with a mocked / static production-shaped adapter.

Do not start with real RPC.

Reason:

- keeps tests deterministic
- validates source construction policy before network concerns
- avoids introducing RPC secrets / provider config
- avoids snapshot changes
- avoids CLI changes
- avoids ABI/address hardcoding too early

Proposed future adapter shape:

- XcEpochMinimumSourceAdapter
- XcEpochMinimumSourceAdapterInput
- XcEpochMinimumRecord

Record concept:

- lockEpoch
- minimumXntd
- observedAt
- sourceChainId
- sourceBlockNumber
- sourceBlockHash

Finalized block policy questions captured:

1. Which block tag is acceptable?
2. What happens if finalized state is unavailable?
3. What happens if RPC nodes disagree?
4. What happens near epoch boundaries?
5. Should the source use current epoch, historical map, current + recent epochs, or checkpointed records?

Recommended initial policy:

- do not read latest
- use finalized or explicitly confirmed block
- reject missing data
- reject stale data when policy requires freshness
- prefer deterministic checkpoint records in tests

XC Core / Lens read fields captured:

- currentEpoch
- currentBaseNominal
- currentXenBurnAmount
- genesisTs
- halvingIntervalSec
- initialNominal
- epochAt(timestamp), if available
- protocol parameters exposed through Lens

Epoch policy:

The adapter should answer the required minimum for lockEpoch.

Recommended first policy:

- accept lockEpoch from the observed event / proof payload
- validate observedRequiredXntdLock against authoritativeEpochMinimum(lockEpoch)
- later add timestamp / block cross-checks if needed

Failure policy:

If the adapter cannot produce an authoritative minimum:

- return null for that epoch
- runtime rejects with MissingAuthoritativeXcEpochMinimum

If the adapter produces a minimum and payload differs:

- runtime rejects with MismatchedAuthoritativeXcEpochMinimum

If adapter input is malformed:

- adapter creation should fail before proof submission
- do not create a source from invalid records

Mocked adapter test strategy captured:

1. builds XcEpochMinimumSource from valid records
2. rejects duplicate epoch records with conflicting minimums
3. rejects zero minimum records
4. returns null for missing epoch
5. supports multiple epoch records
6. preserves deterministic behavior
7. does not read network
8. does not require secrets

Non-goals:

This design milestone does not implement:

- adapter runtime code
- real Ethereum RPC
- XC Core ABI
- XC Lens ABI
- provider config
- private keys
- RPC URLs
- snapshot schema changes
- CLI integration
- bridge signer integration
- X1 on-chain verification

Current conclusion:

The next safe production-readiness step is a mocked production-shaped source adapter.

It should validate source records and produce an XcEpochMinimumSource without network access.

Real RPC should come later, after source policy and adapter tests are stable.

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities

## Latest authoritative XC epoch minimum mocked source adapter checkpoint

The authoritative XC epoch minimum mocked source adapter milestone was completed on the authoritative-xc-epoch-minimum-mocked-source-adapter branch.

Commits:

- 3db5b48 Add mocked XC epoch minimum source adapter
- 24b46b4 Add mocked XC epoch minimum source adapter notes

This milestone adds a mocked / production-shaped XC epoch minimum source adapter.

It is the first small runtime step after the production source adapter design.

Updated runtime file:

- src/model/xc-epoch-minimum-source.ts

Updated tests:

- tests/xc-epoch-minimum-source.test.ts

Implementation notes:

- implementation/authoritative-xc-epoch-minimum-mocked-source-adapter-notes.md

Runtime changes:

Added:

- XcEpochMinimumRecord
- createXcEpochMinimumSourceFromRecords()

The adapter converts validated epoch minimum records into an XcEpochMinimumSource.

Record shape:

XcEpochMinimumRecord includes:

- lockEpoch
- minimumXntd
- observedAt
- sourceChainId
- sourceBlockNumber
- sourceBlockHash

Only lockEpoch and minimumXntd are used by the source map.

The other fields are production-shaped metadata for future source adapters.

Validation rules:

The mocked adapter validates:

- lockEpoch must be an integer
- lockEpoch must be >= 0
- minimumXntd must be > 0
- duplicate records for the same epoch are allowed only when minimumXntd matches
- duplicate records for the same epoch with conflicting minimums are rejected

Missing epochs return null through authoritativeEpochMinimum().

Error behavior:

Invalid records currently throw BuildError with:

- InvalidXntdLockAmount

This keeps the first adapter layer small and avoids expanding the error enum before the source policy stabilizes.

A later hardening layer may introduce a dedicated invalid-source-record error code if needed.

Test coverage:

- builds XcEpochMinimumSource from production-shaped records
- returns null for missing epoch
- allows duplicate records when minimums match
- rejects conflicting duplicate records
- rejects zero minimum
- rejects negative epoch

Scope boundary:

This milestone does not implement:

- real Ethereum RPC reads
- XC Core ABI
- XC Lens ABI
- provider config
- private keys
- RPC URLs
- snapshot schema changes
- CLI integration
- bridge signer integration
- X1 on-chain verification
- persistent app-state source ownership

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 30 test files passed
- 198 tests passed

Next step:

After merge, a possible next runtime hardening step is to add a dedicated error code for invalid XC epoch minimum source records, but only if it is useful enough to justify expanding the error model.

## Latest invalid XC epoch minimum record error checkpoint

The invalid XC epoch minimum record error milestone was completed on the invalid-xc-epoch-minimum-record-error branch.

Commits:

- 6342ac8 Add invalid XC epoch minimum record error
- 5f1f066 Add invalid XC epoch minimum record error notes

This milestone separates XC epoch minimum source-record validation errors from XNTD lock amount validation errors.

Updated runtime files:

- src/errors/build-error.ts
- src/model/xc-epoch-minimum-source.ts

Updated tests:

- tests/xc-epoch-minimum-source.test.ts

Implementation notes:

- implementation/invalid-xc-epoch-minimum-record-error-notes.md

Runtime change:

Added BuildErrorCode:

- InvalidXcEpochMinimumRecord

The source adapter now uses InvalidXcEpochMinimumRecord for:

- invalid lockEpoch
- zero / negative minimumXntd
- conflicting duplicate epoch minimum records

Error model after this milestone:

XNTD lock amount errors:

- InvalidXntdLockAmount

Source availability errors:

- MissingAuthoritativeXcEpochMinimum

Source mismatch errors:

- MismatchedAuthoritativeXcEpochMinimum

Source record construction errors:

- InvalidXcEpochMinimumRecord

Test coverage:

Updated / strengthened tests verify that invalid source records throw:

- InvalidXcEpochMinimumRecord

Covered cases:

- conflicting duplicate epoch records
- zero minimumXntd
- negative lockEpoch

Scope boundary:

This milestone does not change:

- authoritative validation flow
- appSubmitProof()
- app service
- registrar handlers
- snapshot schema
- storage serialization
- CLI output
- real Ethereum RPC integration
- XC Core / Lens ABI integration

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 30 test files passed
- 198 tests passed

This milestone makes the source adapter error model more precise without changing the validation flow.

## Latest XC epoch minimum record validation hardening checkpoint

The XC epoch minimum record validation hardening milestone was completed on the xc-epoch-minimum-record-validation-hardening branch.

Commits:

- 7c57d10 Harden XC epoch minimum record validation
- 4bd8045 Add XC epoch minimum record validation hardening notes

This milestone hardens validation for production-shaped XC epoch minimum records.

Updated runtime file:

- src/model/xc-epoch-minimum-source.ts

Updated tests:

- tests/xc-epoch-minimum-source.test.ts

Implementation notes:

- implementation/xc-epoch-minimum-record-validation-hardening-notes.md

Runtime change:

The record validator now rejects:

- observedAt <= 0
- sourceBlockNumber <= 0 when sourceBlockNumber is provided

Existing validation remains:

- lockEpoch must be an integer
- lockEpoch must be >= 0
- minimumXntd must be > 0
- duplicate records for the same epoch may only repeat the same minimum
- conflicting duplicate epoch minimums are rejected

Test coverage:

Added coverage:

- rejects observedAt = 0
- rejects sourceBlockNumber = 0 when provided
- verifies both cases use InvalidXcEpochMinimumRecord

Test count changed:

- 198 tests -> 199 tests

Intentional boundary:

This milestone does not validate sourceBlockHash format.

Reason:

sourceBlockHash format requirements should be decided together with the future source adapter policy.

For now, the adapter remains production-shaped but network-agnostic.

Scope boundary:

This milestone does not implement:

- real Ethereum RPC reads
- XC Core ABI
- XC Lens ABI
- provider config
- private keys
- RPC URLs
- snapshot schema changes
- CLI integration
- bridge signer integration
- X1 on-chain verification
- persistent app-state source ownership

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 30 test files passed
- 199 tests passed

## Latest XC epoch minimum source block hash policy checkpoint

The XC epoch minimum source block hash policy milestone was completed on the xc-epoch-minimum-source-block-hash-policy branch.

Commit:

- 412f17a Add XC epoch minimum source block hash policy

This milestone defines the policy question around sourceBlockHash for XC epoch minimum records.

Policy document:

- implementation/xc-epoch-minimum-source-block-hash-policy.md

This is a design-only milestone.

It does not change runtime code.

Current state:

The mocked / production-shaped XC epoch minimum source adapter currently supports:

- lockEpoch
- minimumXntd
- observedAt
- sourceChainId
- sourceBlockNumber
- sourceBlockHash

Current generic validation enforces:

- lockEpoch must be an integer
- lockEpoch must be >= 0
- minimumXntd must be > 0
- observedAt must be > 0
- sourceBlockNumber must be > 0 when provided
- duplicate records for the same epoch may only repeat the same minimum

Current generic validation intentionally does not validate sourceBlockHash format.

Reason:

sourceBlockHash depends on source type.

Possible source types include:

- Ethereum finalized RPC / XC Lens read
- trusted integration source
- checkpoint source
- bridge-provided source
- X1-native verified source
- local deterministic test source

These do not all necessarily share Ethereum block hash semantics.

Recommended policy:

Keep sourceBlockHash optional in the generic record builder.

Do not enforce global hash format in createXcEpochMinimumSourceFromRecords().

Strict sourceBlockHash / provenance validation should be implemented later inside source-specific adapters.

Ethereum-specific future policy:

A future Ethereum / XC Lens source adapter should enforce Ethereum-specific rules:

- sourceChainId must be present
- sourceBlockNumber must be present and > 0
- sourceBlockHash must be present
- sourceBlockHash should be a 0x-prefixed 32-byte hex string
- adapter should not read latest
- adapter should use finalized / safe / explicitly confirmed block policy

Checkpoint source policy:

Checkpoint sources may use checkpointId, checkpointHash, checkpointRoot, signerSetId, signedAt, or finalizedAt instead of Ethereum block hash semantics.

Bridge-provided source policy:

Bridge sources may include bridgeMessageId, signerSetId, attestationHash, and bridge-specific provenance fields.

X1-native source policy:

X1-native sources may use slot, state root, checkpoint account, verified attestation, or canonical registry entry.

Generic adapter responsibility:

The generic createXcEpochMinimumSourceFromRecords() should continue to validate only source-agnostic invariants:

- epoch validity
- positive minimum
- positive observation time
- positive sourceBlockNumber if provided
- no conflicting duplicate epoch minimums

It should not validate source-specific provenance.

Future implementation direction:

Source-specific adapters can create records only after enforcing their own provenance rules.

Examples:

1. Ethereum XC Lens adapter
   - validates sourceBlockHash as Ethereum block hash
   - validates finalized block policy
   - produces XcEpochMinimumRecord[]

2. Checkpoint adapter
   - validates checkpoint signatures / roots
   - produces XcEpochMinimumRecord[]

3. Bridge adapter
   - validates bridge signer policy
   - produces XcEpochMinimumRecord[]

The generic record builder remains the final deterministic map builder.

Non-goals:

This milestone does not implement:

- sourceBlockHash validation
- Ethereum RPC
- XC Core ABI
- XC Lens ABI
- checkpoint verification
- bridge signer verification
- X1-native verification
- snapshot schema changes
- CLI integration

Conclusion:

Do not add global sourceBlockHash validation to the generic source adapter.

Keep sourceBlockHash optional at the generic layer.

Add strict hash/provenance validation later only inside source-specific adapters.

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities

## Latest XC epoch minimum source adapter stage summary checkpoint

The XC epoch minimum source adapter stage summary milestone was completed on the xc-epoch-minimum-source-adapter-stage-summary branch.

Commit:

- cee665f Add XC epoch minimum source adapter stage summary

This milestone summarizes the completed XC epoch minimum source adapter stage.

Summary document:

- implementation/xc-epoch-minimum-source-adapter-stage-summary.md

This is a summary-only milestone.

It does not change runtime code.

Completed adapter-stage line:

1. production source adapter design
2. mocked / production-shaped source adapter
3. dedicated invalid source record error
4. record validation hardening
5. sourceBlockHash policy

Current generic source flow:

XcEpochMinimumRecord[]
-> createXcEpochMinimumSourceFromRecords()
-> XcEpochMinimumSource
-> appSubmitProof(..., xcEpochMinimumSource)
-> registrar authoritative validation
-> Build state

Current record shape:

XcEpochMinimumRecord includes:

- lockEpoch
- minimumXntd
- observedAt
- sourceChainId
- sourceBlockNumber
- sourceBlockHash

Current generic validation:

The generic record builder validates source-agnostic invariants:

- lockEpoch must be an integer
- lockEpoch must be >= 0
- minimumXntd must be > 0
- observedAt must be > 0
- sourceBlockNumber must be > 0 when provided
- duplicate records for the same epoch are allowed only when minimumXntd matches
- conflicting duplicate epoch minimum records are rejected

Current error model:

Source-record construction errors use:

- InvalidXcEpochMinimumRecord

Authoritative source availability errors use:

- MissingAuthoritativeXcEpochMinimum

Authoritative mismatch errors use:

- MismatchedAuthoritativeXcEpochMinimum

XNTD lock amount relationship errors continue to use:

- InvalidXntdLockAmount

This separates:

- user lock/relock amount validation
- source availability validation
- source mismatch validation
- source record construction validation

Current sourceBlockHash policy:

The generic source adapter intentionally does not validate sourceBlockHash format.

Reason:

sourceBlockHash semantics depend on the source type.

Strict hash/provenance validation belongs in source-specific adapters, not in the generic record builder.

Generic adapter boundary:

The generic adapter should remain:

- deterministic
- source-agnostic
- network-free
- secret-free
- snapshot-free
- CLI-free

It should only turn validated source records into an XcEpochMinimumSource.

Source-specific future adapters:

Future adapters can validate provenance before producing XcEpochMinimumRecord[].

Possible future adapters:

1. Ethereum XC Lens adapter
   - validates Ethereum chain/source metadata
   - validates finalized/safe/confirmed block policy
   - validates sourceBlockHash as 0x-prefixed 32-byte hex
   - reads XC Core / Lens state

2. Checkpoint adapter
   - validates checkpoint records
   - validates checkpoint root/hash/signatures if applicable
   - produces deterministic epoch minimum records

3. Bridge-provided adapter
   - validates bridge signer / attestation policy
   - validates bridge message provenance
   - produces epoch minimum records

4. X1-native verified adapter
   - validates X1-native registry/checkpoint/proof source
   - produces epoch minimum records

Current tests:

- 30 test files passed
- 199 tests passed

Coverage includes:

- building source from production-shaped records
- missing epoch returns null
- duplicate matching records accepted
- conflicting duplicate records rejected
- invalid lockEpoch rejected
- invalid minimumXntd rejected
- invalid observedAt rejected
- invalid sourceBlockNumber rejected
- invalid records use InvalidXcEpochMinimumRecord
- authoritative missing epoch uses MissingAuthoritativeXcEpochMinimum
- authoritative mismatch uses MismatchedAuthoritativeXcEpochMinimum

This stage intentionally did not add:

- real Ethereum RPC reads
- XC Core ABI integration
- XC Lens ABI integration
- provider config
- private keys
- RPC URLs
- checkpoint verification
- bridge signer verification
- X1-native verification
- snapshot schema changes
- CLI integration
- persistent app-state source ownership

Current conclusion:

The generic XC epoch minimum source adapter layer is now complete enough for deterministic production-shaped testing.

The next production-readiness step should not expand the generic adapter.

The next step should be source-specific design or implementation, starting with the safest production-like path.

Recommended next stage:

xc-epoch-minimum-ethereum-lens-adapter-design

Scope:

- design only
- define XC Core / Lens fields
- define finalized / safe / confirmed block policy
- define sourceChainId policy
- define sourceBlockNumber/sourceBlockHash requirements for Ethereum records
- define mocked Ethereum read tests
- no real RPC yet
- no secrets
- no provider config
- no CLI wiring yet

Alternative next stage:

xc-epoch-minimum-checkpoint-adapter-design

Use this if checkpoint-based source ownership is preferred before live Ethereum reads.

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities

## Latest XC epoch minimum Ethereum Lens adapter design checkpoint

The XC epoch minimum Ethereum Lens adapter design milestone was completed on the xc-epoch-minimum-ethereum-lens-adapter-design branch.

Commit:

- 754ccd6 Add XC epoch minimum Ethereum Lens adapter design

This milestone designs a future Ethereum / XC Lens source-specific adapter for XC epoch minimum records.

Design document:

- implementation/xc-epoch-minimum-ethereum-lens-adapter-design.md

This is a design-only milestone.

It does not change runtime code.

Current completed foundation:

The generic XC epoch minimum source layer is already complete enough for deterministic production-shaped testing.

Current generic flow:

XcEpochMinimumRecord[]
-> createXcEpochMinimumSourceFromRecords()
-> XcEpochMinimumSource
-> appSubmitProof(..., xcEpochMinimumSource)
-> registrar authoritative validation
-> Build state

Adapter responsibility:

The Ethereum / XC Lens adapter should be source-specific.

Its responsibility is to read or receive Ethereum XC state, validate Ethereum-specific provenance, and produce XcEpochMinimumRecord[].

Conceptual flow:

Ethereum finalized/safe/confirmed XC state
-> Ethereum-specific validation
-> XcEpochMinimumRecord[]
-> createXcEpochMinimumSourceFromRecords()
-> XcEpochMinimumSource

First implementation boundary:

The first implementation should not perform real RPC reads.

The first implementation should use mocked Ethereum read results.

Reason:

- no RPC secrets
- no provider config
- deterministic tests
- no network flakiness
- no ABI/address hardcoding too early
- lets us validate policy before integration

Recommended first mocked adapter name:

- createXcEpochMinimumSourceFromEthereumLensSnapshot()

Proposed design shapes:

- EthereumXcLensEpochMinimumSnapshot
- EthereumXcEpochMinimumEntry
- EthereumFinalityPolicy

Required Ethereum-specific fields:

For Ethereum / XC Lens source records, the adapter should require:

- sourceChainId
- sourceBlockNumber
- sourceBlockHash
- observedAt
- finalizedPolicy
- epochMinimums

Unlike the generic record builder, the Ethereum-specific adapter should not treat sourceBlockHash as optional.

sourceChainId policy:

Recommended format:

- eip155-1 for Ethereum mainnet
- eip155-11155111 for Sepolia
- future EIP-155 chain IDs for other EVM chains if ever needed

The adapter should reject:

- empty sourceChainId
- non-EIP-155 chain ID format for Ethereum adapter
- unexpected chain ID when config restricts the adapter to one chain

sourceBlockNumber policy:

- sourceBlockNumber must be present and > 0
- Ethereum provenance should be tied to a specific block

sourceBlockHash policy:

sourceBlockHash must be present.

Recommended format:

- string
- starts with 0x
- exactly 66 characters
- 32 bytes hex after 0x
- lowercase normalization may be applied by adapter, or comparison may be case-insensitive

The generic record builder should not enforce this.

The Ethereum adapter should enforce it.

observedAt policy:

- observedAt must be > 0
- observedAt should be set from adapter observation time, not user payload

finalized / safe / confirmed block policy:

The adapter should not read latest.

Acceptable future policies:

1. finalized
2. safe
3. confirmed

Recommended initial production-like policy:

- support a mocked finalized snapshot first
- later support finalized block tag
- do not support latest
- reject unknown finality policy

XC Core / Lens fields:

The adapter must be able to derive:

authoritativeEpochMinimum(lockEpoch)

For the current protocol meaning, this should be the XC Core L1 minimum nominal for the relevant epoch.

Candidate XC fields:

- genesisTs
- halvingIntervalSec
- initialNominal
- currentEpoch
- currentBaseNominal
- epochAt(timestamp), if available
- base nominal by epoch, if exposed
- Lens-provided protocol parameters, if available

Historical epoch policy:

The adapter must answer for lockEpoch, not just current epoch.

Possible strategies:

1. Read direct epoch minimum from Lens if available.
2. Compute epoch minimum from initialNominal and halving rules.
3. Use checkpointed epoch minimum records generated from finalized Ethereum reads.
4. Use current epoch only for early controlled tests.

Recommended first design:

- support explicit epochMinimums in mocked snapshot
- later define whether production reads direct values or computes from protocol constants

Epoch boundary policy:

Current recommended policy:

- keep current runtime validation:
  observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

- do not add timestamp cross-check in this adapter design milestone
- document timestamp / epoch-boundary cross-check as future hardening

Failure behavior:

The Ethereum adapter should reject malformed snapshots before producing records.

Reject if:

- sourceChainId is invalid
- sourceBlockNumber is missing or <= 0
- sourceBlockHash is missing or invalid
- observedAt <= 0
- finality policy is invalid
- epochMinimums is empty when policy requires at least one epoch
- any epoch minimum entry has invalid epoch or minimum
- duplicate epoch entries conflict

Error model:

For malformed Ethereum adapter input, future implementation can use:

- InvalidXcEpochMinimumRecord

If a source cannot answer an epoch, resulting source returns null and runtime will throw:

- MissingAuthoritativeXcEpochMinimum

If payload differs from authoritative source, runtime throws:

- MismatchedAuthoritativeXcEpochMinimum

First implementation test strategy:

Use mocked Ethereum snapshots.

Recommended tests:

1. builds source from valid mocked Ethereum snapshot
2. rejects missing sourceBlockHash
3. rejects invalid sourceBlockHash format
4. rejects sourceBlockNumber <= 0
5. rejects empty sourceChainId
6. rejects non-EIP-155 sourceChainId
7. rejects invalid finality policy
8. rejects conflicting duplicate epoch entries
9. returns null for missing epoch through resulting source
10. does not read network
11. does not require secrets

Security boundary:

This adapter improves provenance validation for Ethereum-shaped source data.

It does not make the source trustless by itself.

Actual production trust still depends on:

- RPC provider trust
- finality policy
- Lens/Core address correctness
- ABI correctness
- deployment configuration
- monitoring and replay/audit process

Non-goals:

This design milestone does not implement:

- real RPC reads
- viem / ethers provider integration
- XC Core ABI
- XC Lens ABI
- address config
- provider config
- RPC URLs
- private keys
- CLI integration
- snapshot schema migration
- bridge signer verification
- X1-native verification

Recommended next implementation milestone:

xc-epoch-minimum-mocked-ethereum-lens-snapshot-adapter

Scope:

- mocked snapshot input
- Ethereum-specific source metadata validation
- sourceBlockHash validation
- finality policy validation
- produce XcEpochMinimumSource
- tests only
- no network access
- no provider config
- no ABI

Conclusion:

The Ethereum / XC Lens adapter should be source-specific and stricter than the generic record builder.

The generic builder remains source-agnostic.

Ethereum-specific provenance checks belong in the Ethereum adapter.

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities


## Latest XC epoch minimum mocked Ethereum Lens snapshot adapter checkpoint

The XC epoch minimum mocked Ethereum Lens snapshot adapter milestone was completed on the xc-epoch-minimum-mocked-ethereum-lens-snapshot-adapter branch.

Commits:

- 462b786 Add mocked Ethereum XC epoch minimum source adapter
- c2033b9 Add mocked Ethereum XC epoch minimum adapter notes

This milestone implements the first source-specific XC epoch minimum adapter layer for Ethereum / XC Lens shaped data.

Runtime additions:

- src/model/ethereum-xc-epoch-minimum-source.ts

Test additions:

- tests/ethereum-xc-epoch-minimum-source.test.ts

Documentation additions:

- implementation/xc-epoch-minimum-mocked-ethereum-lens-snapshot-adapter-notes.md

Exports added through src/index.ts:

- EthereumXcLensEpochMinimumSnapshot
- EthereumXcEpochMinimumEntry
- EthereumFinalityPolicy
- createXcEpochMinimumSourceFromEthereumLensSnapshot()

Purpose:

The authoritative XC epoch minimum runtime chain already validates:

observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

The generic source builder remains source-agnostic and accepts validated XcEpochMinimumRecord[].

This milestone adds an Ethereum-specific mocked snapshot adapter that validates Ethereum-shaped source metadata before producing generic source records.

Adapter flow:

mocked Ethereum Lens snapshot
-> Ethereum-specific metadata validation
-> XcEpochMinimumRecord[]
-> createXcEpochMinimumSourceFromRecords()
-> XcEpochMinimumSource

Snapshot shape:

- sourceChainId
- sourceBlockNumber
- sourceBlockHash
- observedAt
- finalizedPolicy
- epochMinimums

Finality policy shape:

- finalized
- safe
- confirmed with positive confirmations

Ethereum-specific validation added:

- sourceChainId must match eip155-<number>
- sourceBlockNumber must be > 0
- sourceBlockHash must be 0x-prefixed 32-byte hex
- observedAt must be > 0
- finalizedPolicy kind must be finalized, safe, or confirmed
- confirmed finality requires positive integer confirmations
- epochMinimums must be non-empty

The adapter normalizes valid Ethereum block hashes to lowercase before mapping entries into generic records.

Epoch entry validation remains delegated to the generic source builder:

- lockEpoch must be an integer and >= 0
- minimumXntd must be > 0
- duplicate epoch records are allowed only when minimumXntd matches
- conflicting duplicate epoch minimum records are rejected

Error model:

Malformed Ethereum snapshot input and invalid epoch entries use the existing source-record error:

- InvalidXcEpochMinimumRecord

No new error code was added.

Resulting source behavior:

- known epoch returns the authoritative minimum
- missing epoch returns null
- runtime assertion later converts missing epoch to MissingAuthoritativeXcEpochMinimum
- runtime assertion converts mismatched observed value to MismatchedAuthoritativeXcEpochMinimum

Tests covered:

1. valid mocked Ethereum Lens snapshot
2. mixed-case block hash acceptance / normalization
3. missing or empty sourceChainId rejection
4. non-EIP-155 sourceChainId rejection
5. non-positive sourceBlockNumber rejection
6. missing / invalid sourceBlockHash rejection
7. non-positive observedAt rejection
8. safe finality acceptance
9. confirmed finality with positive confirmations acceptance
10. invalid finality kind rejection
11. confirmed finality without positive confirmations rejection
12. empty epochMinimums rejection
13. conflicting duplicate epoch entries rejection
14. missing epoch returns null

Security / operational boundary:

This milestone intentionally does not add:

- real Ethereum RPC reads
- provider config
- RPC URLs
- private keys
- API keys
- ABIs
- CLI commands
- snapshot persistence changes
- bridge signer verification
- X1-native source verification

The branch remains deterministic and suitable for production-shaped tests without network access.

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities

Current test count:

- 31 test files passed
- 213 tests passed

Conclusion:

The mocked Ethereum Lens snapshot adapter is now implemented as the first source-specific adapter layer.

It keeps the generic XC epoch minimum source builder clean and source-agnostic while moving Ethereum-specific provenance validation into a dedicated Ethereum adapter.

Recommended next milestone:

xc-epoch-minimum-mocked-ethereum-lens-snapshot-adapter-review

Suggested scope:

- review source-specific adapter boundaries
- verify no network / secret / ABI assumptions slipped in
- verify exactOptionalPropertyTypes compatibility
- verify tests cover all intended Ethereum snapshot policy cases
- decide whether any additional invalid-shape tests are needed before moving toward real provider / ABI design


## Latest XC epoch minimum mocked Ethereum Lens snapshot adapter review checkpoint

The XC epoch minimum mocked Ethereum Lens snapshot adapter review milestone was completed on the xc-epoch-minimum-mocked-ethereum-lens-snapshot-adapter-review branch.

Commits:

- a5dea0b Add mocked Ethereum XC epoch minimum adapter review notes

This was a review-only milestone. No runtime behavior changed.

Reviewed implementation:

- 462b786 Add mocked Ethereum XC epoch minimum source adapter
- c2033b9 Add mocked Ethereum XC epoch minimum adapter notes
- 4400fc9 Update checkpoint after mocked Ethereum XC epoch minimum adapter
- 29bb14c Merge branch 'xc-epoch-minimum-mocked-ethereum-lens-snapshot-adapter'

Reviewed files:

- src/model/ethereum-xc-epoch-minimum-source.ts
- tests/ethereum-xc-epoch-minimum-source.test.ts
- implementation/xc-epoch-minimum-mocked-ethereum-lens-snapshot-adapter-notes.md
- docs/checkpoints/current-design-checkpoint.md

Boundary review conclusion:

The mocked Ethereum Lens snapshot adapter boundary is clean.

The adapter remains source-specific and deterministic:

EthereumXcLensEpochMinimumSnapshot
-> Ethereum-specific metadata validation
-> XcEpochMinimumRecord[]
-> createXcEpochMinimumSourceFromRecords()
-> XcEpochMinimumSource

The generic source builder remains source-agnostic.

Ethereum-specific validation was not pushed into:

- src/model/xc-epoch-minimum-source.ts

This preserves the intended architecture:

- generic layer validates generic record invariants
- Ethereum adapter validates Ethereum-shaped provenance metadata

Network / secret / ABI assumptions review:

The implementation does not introduce:

- real RPC reads
- provider configuration
- RPC URLs
- private keys
- API keys
- ABIs
- fetch / HTTP calls
- viem / ethers dependencies
- process.env reads
- CLI integration
- snapshot persistence changes

A targeted grep over the new runtime, test, and notes files found network / secret / ABI terms only in the notes file where they are explicitly documented as non-goals.

No secret-bearing files were inspected.

TypeScript review:

The implementation is compatible with the current TypeScript settings.

The finality policy runtime guard intentionally treats policy as an unknown-shaped object internally so tests can verify invalid runtime payload shapes while preserving the exported strict union type.

Current exported finality policy remains:

- finalized
- safe
- confirmed with confirmations

Validation policy review:

The implemented Ethereum-specific validation matches the planned mocked snapshot policy:

- sourceChainId must match eip155-<number>
- sourceBlockNumber must be > 0
- sourceBlockHash must be 0x-prefixed 32-byte hex
- observedAt must be > 0
- finalizedPolicy kind must be finalized, safe, or confirmed
- confirmed finality requires positive integer confirmations
- epochMinimums must be non-empty

The adapter lowercases valid Ethereum block hashes before mapping entries into records.

Generic epoch record validation remains delegated to the existing generic builder.

Test coverage review:

The current tests cover the intended mocked snapshot policy cases:

1. valid mocked Ethereum Lens snapshot
2. mixed-case block hash acceptance / normalization
3. missing or empty sourceChainId rejection
4. non-EIP-155 sourceChainId rejection
5. non-positive sourceBlockNumber rejection
6. missing / invalid sourceBlockHash rejection
7. non-positive observedAt rejection
8. safe finality acceptance
9. confirmed finality with positive confirmations acceptance
10. invalid finality kind rejection
11. confirmed finality without positive confirmations rejection
12. empty epochMinimums rejection
13. conflicting duplicate epoch entries rejection
14. missing epoch returns null

Additional invalid-shape tests:

No additional tests are required before merging this review milestone.

Possible future tests may be useful only if the adapter begins accepting unknown JSON-like input directly, for example:

- missing finalizedPolicy object
- finalizedPolicy = null
- epochMinimums not an array
- sourceBlockNumber not bigint
- observedAt not bigint

Those cases are not necessary now because the current function accepts the typed EthereumXcLensEpochMinimumSnapshot shape, and this milestone is not a JSON parser or RPC response decoder.

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities

Current test count:

- 31 test files passed
- 213 tests passed

Conclusion:

The mocked Ethereum Lens snapshot adapter is safe to keep as the last mocked snapshot review before a separate real provider / ABI design milestone.

Recommended next milestone:

xc-epoch-minimum-ethereum-lens-provider-adapter-design

Suggested next scope:

- design real provider / ABI integration only
- define required XC Core / Lens address inputs
- define finality source policy
- define finalized / safe / confirmed block handling
- define provider trust assumptions
- define no-secret config boundary
- do not implement real RPC yet unless design is reviewed first


## Latest XC epoch minimum Ethereum Lens provider adapter design checkpoint

The XC epoch minimum Ethereum Lens provider adapter design milestone was completed on the xc-epoch-minimum-ethereum-lens-provider-adapter-design branch.

Commits:

- 90a543e Add XC epoch minimum Ethereum Lens provider adapter design

This was a design-only milestone.

No runtime behavior changed.

Design document added:

- implementation/xc-epoch-minimum-ethereum-lens-provider-adapter-design.md

Purpose:

Design a future real Ethereum / XC Lens provider adapter for authoritative XC epoch minimum records.

The milestone does not implement:

- real RPC reads
- provider configuration
- ABI calls
- CLI commands
- snapshot persistence
- env loading
- secrets

Current completed foundation:

The runtime validation chain already supports injected XC epoch minimum source validation:

watcher candidate
-> proof conversion
-> appSubmitProof(..., xcEpochMinimumSource)
-> appApplyRegistrarXntdLock() / appApplyRegistrarXntdRelock()
-> applyRegistrarXntdLock() / applyRegistrarXntdRelock()
-> assertAuthoritativeXcEpochMinimum()
-> Build state

The runtime assertion remains:

observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

Existing generic source flow:

XcEpochMinimumRecord[]
-> createXcEpochMinimumSourceFromRecords()
-> XcEpochMinimumSource

Existing mocked Ethereum Lens snapshot adapter flow:

EthereumXcLensEpochMinimumSnapshot
-> Ethereum-specific metadata validation
-> XcEpochMinimumRecord[]
-> createXcEpochMinimumSourceFromRecords()
-> XcEpochMinimumSource

Provider adapter design goal:

Future real provider adapter should convert finalized / safe / confirmed Ethereum XC Lens or XC Core reads into the same snapshot shape already accepted by the mocked snapshot adapter.

Recommended future high-level flow:

adapter config
-> provider read at finalized / safe / confirmed block
-> XC Lens / Core calls
-> EthereumXcLensEpochMinimumSnapshot
-> createXcEpochMinimumSourceFromEthereumLensSnapshot()
-> XcEpochMinimumSource

The already-reviewed mocked snapshot adapter remains the deterministic validation boundary.

Recommended future adapter shape:

- createXcEpochMinimumSourceFromEthereumLensProvider(input)

Possible future input shape:

- provider
- chainId
- lensAddress
- optional coreAddress
- finalityPolicy
- lockEpochs
- observedAt

Provider boundary:

The provider adapter should be read-only.

It should not:

- sign transactions
- send transactions
- mutate Ethereum state
- require private keys
- require wallet accounts
- manage RPC URLs directly
- read process.env directly

The provider should be passed in from outer infrastructure.

No-secret config boundary:

Do not put RPC URLs, API keys, private keys, mnemonics, tokens, or .env reads into model code.

Allowed future pattern:

outer integration layer constructs provider
-> provider is passed into adapter
-> adapter performs read-only calls through interface

Disallowed future pattern:

- adapter reads process.env.RPC_URL
- adapter embeds provider URL
- adapter accepts private key
- adapter logs provider URL
- adapter logs request headers
- adapter prints raw config

Provider interface design:

A narrow read-only provider interface is preferred over binding directly to viem or ethers.

Possible future interface:

- getChainId()
- getBlock()
- readContract()

This keeps adapter logic testable without importing a concrete provider library.

Chain ID policy:

The provider adapter should:

- read provider chain ID
- convert it to eip155-<number>
- compare with configured chain ID
- reject mismatch before producing records

Address policy:

The provider adapter should require explicit XC Lens / Core addresses.

The adapter should not hardcode addresses.

Address validation should remain Ethereum-specific:

- 0x-prefixed
- 20-byte hex
- normalized lowercase or checksum-preserving comparison policy

ABI policy:

This design milestone does not define final ABIs.

Future implementation should keep ABI scope minimal and use only functions required by the chosen read strategy.

Epoch minimum derivation strategies:

Possible strategies documented:

1. Direct Lens epoch minimum read
2. Core protocol constants + local computation
3. Checkpointed Ethereum reads

Recommended first provider design direction:

Do not choose final ABI yet in runtime.

Design should support both:

- direct Lens epoch minimum reads
- protocol constants + local computation

Before implementation, confirm which on-chain view exists and is intended as the source of truth.

Finality policy:

The provider adapter must not read latest.

Allowed policies:

- finalized
- safe
- confirmed

Finalized policy:

- read finalized block
- use finalized block number for all contract reads
- use finalized block hash in snapshot

Safe policy:

- read safe block
- use safe block number for all contract reads
- use safe block hash in snapshot

Confirmed policy:

- read head block only to calculate an older confirmed block number
- read confirmed block by number
- use confirmed block number for all contract reads
- use confirmed block hash in snapshot

Confirmed policy requires confirmations > 0.

Do not use latest block directly as the provenance block.

Block consistency policy:

All contract reads used to produce one snapshot should be performed at one selected provenance block number.

The snapshot must include:

- sourceBlockNumber
- sourceBlockHash

The source block hash must correspond to the same block number used for reads.

observedAt policy:

For the current existing shape, selected Ethereum block timestamp is the cleanest first design for observedAt.

Requested lockEpochs policy:

The provider adapter should accept an explicit list of requested lockEpochs.

It should not infer all epochs by default.

The adapter should reject empty lockEpochs list unless a future use case requires all-current snapshot generation.

Error model:

For now, avoid new error codes unless runtime implementation demonstrates a real distinction.

Expected future failures include:

- invalid configured chain ID
- provider chain ID mismatch
- invalid Lens / Core address
- unsupported finality policy
- confirmed policy with confirmations <= 0
- selected block has no hash
- contract read result cannot be decoded
- computed / read minimum is invalid
- requested epoch missing

Logging policy:

If logging is later added, allowed logs should be limited to:

- chain ID
- selected block number
- selected block hash
- finality policy kind
- requested lockEpochs
- Lens / Core address

Do not log:

- RPC URLs
- API keys
- authorization headers
- private keys
- mnemonic
- full env config

Testing strategy:

Future provider adapter implementation should start with mocked provider tests.

Recommended tests include:

1. finalized block selection
2. safe block selection
3. confirmed block selection with positive confirmations
4. latest policy rejection
5. provider chain ID mismatch rejection
6. invalid configured chain ID rejection
7. invalid Lens address rejection
8. missing block hash rejection
9. all reads performed at selected block number
10. empty requested lockEpochs rejection
11. invalid read result rejection
12. snapshot validation propagation
13. no process.env reads
14. no private keys
15. no RPC URL in adapter input

Security / trust assumptions:

A provider adapter is not fully trustless.

Its correctness depends on:

- provider honesty and availability
- finality policy correctness
- block hash / number consistency
- Lens / Core address correctness
- ABI correctness
- adapter read strategy correctness
- monitoring and replay/audit process

Conclusion:

The future real Ethereum Lens provider adapter should be a thin, read-only provider layer that produces a reviewed Ethereum snapshot shape.

The mocked snapshot adapter remains the validation boundary.

Provider / ABI integration should be reviewed before real network implementation.

Recommended next milestone:

xc-epoch-minimum-ethereum-lens-provider-adapter-design-review

Suggested next scope:

- review provider / ABI design boundary
- verify no runtime implementation was added
- verify no secret / RPC / env coupling
- decide whether provider implementation should use a custom read provider interface first
- decide whether direct Lens read or protocol-constant computation should be the first real strategy


## Latest XC epoch minimum Ethereum Lens provider adapter design review checkpoint

The XC epoch minimum Ethereum Lens provider adapter design review milestone was completed on the xc-epoch-minimum-ethereum-lens-provider-adapter-design-review branch.

Commits:

- 4ace8bd Add XC epoch minimum Ethereum provider adapter design review notes

This was a review-only milestone.

No runtime behavior changed.

Review note added:

- implementation/xc-epoch-minimum-ethereum-lens-provider-adapter-design-review-notes.md

Reviewed design:

- implementation/xc-epoch-minimum-ethereum-lens-provider-adapter-design.md

Reviewed prior commits:

- 90a543e Add XC epoch minimum Ethereum Lens provider adapter design
- c5e4fc1 Update checkpoint after XC epoch minimum Ethereum provider adapter design
- cf44d52 Merge branch 'xc-epoch-minimum-ethereum-lens-provider-adapter-design'

Review conclusion:

The Ethereum Lens provider / ABI adapter design boundary is clean.

The design remains design-only and does not add:

- runtime code
- real RPC reads
- provider configuration
- ABI calls
- CLI commands
- snapshot persistence
- env loading
- secrets

Future provider adapter shape remains:

provider read at finalized / safe / confirmed block
-> XC Lens / Core calls
-> EthereumXcLensEpochMinimumSnapshot
-> createXcEpochMinimumSourceFromEthereumLensSnapshot()
-> XcEpochMinimumSource

The mocked snapshot adapter remains the deterministic validation boundary.

Secret / RPC / env coupling review:

The future provider adapter should not:

- read process.env directly
- accept private keys
- accept RPC URLs directly
- accept mnemonics
- accept API keys
- log provider URLs
- log headers
- print raw config

Correct future pattern:

outer integration layer constructs provider
-> provider is passed into adapter
-> adapter performs read-only calls through a narrow interface

A targeted grep over the design file found secret / RPC / ABI terms only in boundary, non-goal, and future design sections.

No secret-bearing files were inspected.

Provider interface review:

The review confirms that the first implementation should use a custom read-only provider interface before binding to a concrete provider library.

Preferred future abstraction:

- getChainId()
- getBlock()
- readContract()

This avoids direct viem / ethers dependency in the first provider adapter layer unless isolated behind the interface.

Finality policy review:

Allowed future policies remain:

- finalized
- safe
- confirmed

Latest remains unsupported.

Confirmed policy may read head only to calculate an older confirmed block number, then must read the selected confirmed block by number.

All contract reads for one snapshot must use the selected provenance block number.

The selected block must have a block hash.

Chain and address policy review:

Provider adapter implementation should:

- validate configured chain ID as eip155-<number>
- compare configured chain ID against provider chain ID
- reject mismatch before producing records
- require explicit Lens / Core addresses
- validate Ethereum addresses inside provider adapter, not generic source builder

ABI / epoch minimum strategy review:

The design intentionally does not lock final ABI yet.

Possible strategies remain:

1. Direct Lens epoch minimum read
2. Core protocol constants + local computation
3. Checkpointed Ethereum reads

Review decision:

- first implementation should prove the read-only provider boundary with mocked provider tests
- do not perform real RPC yet
- do not lock final ABI in runtime until actual XC Lens/Core view source is confirmed

Preferred first real strategy after mocked provider implementation:

- direct Lens epoch minimum read if Lens exposes historical epoch minimums

Fallback:

- protocol constants + local computation if direct historical minimums are not exposed

observedAt review:

For provider-produced snapshots, first mocked provider implementation should use selected block timestamp for observedAt.

Requested lockEpochs review:

Provider adapter implementation should require explicit non-empty lockEpochs.

No implicit unbounded epoch scans.

Error model review:

Reuse InvalidXcEpochMinimumRecord for the first mocked provider adapter implementation unless runtime implementation reveals a real need for a dedicated adapter-config error.

Testing strategy review:

The first mocked provider adapter implementation should test:

1. finalized block selection
2. safe block selection
3. confirmed block selection with positive confirmations
4. latest policy rejection
5. provider chain ID mismatch rejection
6. invalid configured chain ID rejection
7. invalid Lens address rejection
8. missing block hash rejection
9. all reads performed at selected block number
10. empty requested lockEpochs rejection
11. invalid read result rejection
12. snapshot validation propagation
13. no process.env reads
14. no private keys
15. no RPC URL in adapter input

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities

Current test count:

- 31 test files passed
- 213 tests passed

Conclusion:

The Ethereum Lens provider / ABI adapter design is ready to proceed to a mocked provider implementation milestone.

The implementation should not perform real Ethereum RPC yet.

Recommended next milestone:

xc-epoch-minimum-mocked-ethereum-lens-provider-adapter

Suggested next scope:

- implement mocked read-only provider interface
- no real RPC
- no env reads
- no secrets
- no private keys
- no direct RPC URL input
- no CLI command
- produce EthereumXcLensEpochMinimumSnapshot
- reuse createXcEpochMinimumSourceFromEthereumLensSnapshot()
- tests only with mocked provider


## Latest XC epoch minimum mocked Ethereum Lens provider adapter checkpoint

The XC epoch minimum mocked Ethereum Lens provider adapter milestone was completed on the xc-epoch-minimum-mocked-ethereum-lens-provider-adapter branch.

Commits:

- 6c7414a Add mocked Ethereum XC epoch minimum provider adapter
- ffe0087 Add mocked Ethereum XC epoch minimum provider adapter notes

This milestone implements the first mocked read-only provider adapter for Ethereum / XC Lens epoch minimum sources.

Runtime additions:

- src/model/ethereum-xc-epoch-minimum-provider-source.ts

Test additions:

- tests/ethereum-xc-epoch-minimum-provider-source.test.ts

Documentation additions:

- implementation/xc-epoch-minimum-mocked-ethereum-lens-provider-adapter-notes.md

Exports added through src/index.ts:

- EthereumReadProvider
- EthereumBlockReadInput
- EthereumBlockSnapshot
- EthereumContractReadInput
- EthereumXcLensProviderAdapterInput
- createXcEpochMinimumSourceFromEthereumLensProvider()

Purpose:

The provider adapter proves the read-only provider boundary with mocked provider tests only.

It does not perform real RPC reads, does not read env, does not require secrets, does not accept private keys, does not accept direct RPC URLs, and does not add CLI commands.

Adapter flow:

provider
-> selected finalized / safe / confirmed provenance block
-> read epoch minimums at selected block number
-> EthereumXcLensEpochMinimumSnapshot
-> createXcEpochMinimumSourceFromEthereumLensSnapshot()
-> XcEpochMinimumSource

Provider interface:

The adapter uses a custom read-only provider interface:

- getChainId()
- getBlock()
- readContract()

No direct viem / ethers dependency was introduced.

Input shape:

The provider adapter accepts:

- provider
- chainId
- lensAddress
- finalityPolicy
- lockEpochs
- optional epochMinimumFunctionName
- optional epochMinimumAbi

The adapter does not accept:

- RPC URLs
- private keys
- mnemonic
- API keys
- wallet / signer accounts
- env config

Validation policy:

The adapter validates:

- configured chainId must match eip155-<number>
- provider chain ID must match configured chain ID
- lensAddress must be 0x-prefixed 20-byte hex
- finalityPolicy must be finalized, safe, or confirmed
- confirmed finality requires positive integer confirmations
- lockEpochs must be non-empty
- selected provenance block must exist
- selected provenance block number must be > 0
- selected provenance block hash must be present
- selected provenance block timestamp must be > 0
- contract read result must decode to bigint
- minimumXntd must be > 0

Invalid provider input or read results use the existing source-record error:

- InvalidXcEpochMinimumRecord

No new error code was added.

Finality behavior:

finalized:

- getBlock({ blockTag: "finalized" })
- use finalized block number for all contract reads

safe:

- getBlock({ blockTag: "safe" })
- use safe block number for all contract reads

confirmed:

- getBlock({}) to read head only for block-number calculation
- confirmedBlockNumber = head.number - confirmations
- getBlock({ blockNumber: confirmedBlockNumber })
- use confirmed block number for all contract reads

latest is rejected.

The adapter does not use latest as the provenance block.

Snapshot conversion:

The adapter builds EthereumXcLensEpochMinimumSnapshot with:

- sourceChainId = configured chainId
- sourceBlockNumber = selected block number
- sourceBlockHash = selected block hash
- observedAt = selected block timestamp
- finalizedPolicy = input finality policy
- epochMinimums = provider read results

Then it calls:

createXcEpochMinimumSourceFromEthereumLensSnapshot(snapshot)

This preserves the snapshot adapter as the validation boundary for Ethereum-shaped snapshots.

Tests covered:

1. finalized block selection and source build
2. safe block selection and source build
3. confirmed block selection with positive confirmations
4. latest finality rejection
5. confirmed finality without positive confirmations rejection
6. provider chain ID mismatch rejection
7. invalid configured chain ID rejection
8. invalid Lens address rejection
9. selected block without hash rejection
10. empty requested lockEpochs rejection
11. invalid contract read result rejection
12. normalized Lens address and selected block number passed into reads
13. snapshot validation propagation through existing snapshot adapter
14. missing epoch returns null through resulting source

Security / operational boundary:

This milestone intentionally does not add:

- real Ethereum RPC
- env reads
- RPC URL config
- private keys
- API keys
- mnemonic
- signer support
- transaction sending
- CLI commands
- snapshot persistence migration
- bridge signer verification
- X1-native verification

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities

Current test count:

- 32 test files passed
- 227 tests passed

Conclusion:

The mocked Ethereum Lens provider adapter is now implemented as a read-only provider layer.

It proves the provider boundary, finality block selection, chain/address validation, selected-block read consistency, and snapshot conversion without real network access.

Recommended next milestone:

xc-epoch-minimum-mocked-ethereum-lens-provider-adapter-review

Suggested next scope:

- review provider adapter runtime boundary
- verify no real RPC / env / secrets / direct RPC URL input
- verify all reads use the selected provenance block number
- verify snapshot adapter remains the Ethereum-shaped validation boundary
- decide whether any extra provider edge-case tests are needed before moving toward real provider wrapper design


## Latest XC epoch minimum mocked Ethereum Lens provider adapter review checkpoint

The XC epoch minimum mocked Ethereum Lens provider adapter review milestone was completed on the xc-epoch-minimum-mocked-ethereum-lens-provider-adapter-review branch.

Commits:

- 813a423 Add mocked Ethereum XC epoch minimum provider adapter review notes

This was a review-only milestone.

No runtime behavior changed.

Review note added:

- implementation/xc-epoch-minimum-mocked-ethereum-lens-provider-adapter-review-notes.md

Reviewed implementation:

- src/model/ethereum-xc-epoch-minimum-provider-source.ts

Reviewed tests:

- tests/ethereum-xc-epoch-minimum-provider-source.test.ts

Reviewed prior commits:

- 6c7414a Add mocked Ethereum XC epoch minimum provider adapter
- ffe0087 Add mocked Ethereum XC epoch minimum provider adapter notes
- 1f9a411 Update checkpoint after mocked Ethereum XC epoch minimum provider adapter
- bcf1495 Merge branch 'xc-epoch-minimum-mocked-ethereum-lens-provider-adapter'

Review conclusion:

The mocked Ethereum Lens provider adapter runtime boundary is clean.

The implementation remains a mocked read-only provider layer.

It does not perform real Ethereum RPC and does not introduce env / secret / direct RPC URL coupling.

Runtime boundary review:

The runtime adapter does not import or call:

- process.env
- fetch
- http / https
- viem
- ethers
- wallet APIs
- signer APIs

The adapter receives only a custom read-only provider object.

The adapter input does not accept:

- RPC URL
- private key
- mnemonic
- API key
- signer
- wallet account
- env config

Provider interface review:

The runtime uses the intended custom read-only interface:

- getChainId()
- getBlock()
- readContract()

This confirms the design decision:

- keep provider construction outside model code
- keep RPC URL / API key handling outside adapter input
- use mocked provider objects in tests
- avoid direct viem / ethers dependency in this layer

Finality behavior review:

The implementation correctly supports:

- finalized
- safe
- confirmed

finalized:

- getBlock({ blockTag: "finalized" })
- all contract reads use finalized block number

safe:

- getBlock({ blockTag: "safe" })
- all contract reads use safe block number

confirmed:

- getBlock({}) is used only to get head block number
- confirmedBlockNumber = head.number - confirmations
- getBlock({ blockNumber: confirmedBlockNumber })
- all contract reads use confirmed block number

latest is not supported as a provenance policy.

Selected block consistency review:

All contract reads use the selected provenance block number.

The adapter also requires:

- selected block exists
- selected block number > 0
- selected block hash is present
- selected block timestamp > 0

Chain and address validation review:

The adapter validates:

- configured chainId matches eip155-<number>
- provider chain ID is converted to eip155-<number>
- provider chain ID must match configured chainId
- lensAddress must be 0x-prefixed 20-byte hex
- lensAddress is normalized to lowercase before contract reads

Snapshot validation boundary review:

The provider adapter builds EthereumXcLensEpochMinimumSnapshot and delegates Ethereum-shaped validation to:

createXcEpochMinimumSourceFromEthereumLensSnapshot(snapshot)

This preserves the snapshot adapter as the validation boundary.

Test coverage review:

The current provider adapter tests cover:

1. finalized block selection and source build
2. safe block selection and source build
3. confirmed block selection with positive confirmations
4. latest finality rejection
5. confirmed finality without positive confirmations rejection
6. provider chain ID mismatch rejection
7. invalid configured chain ID rejection
8. invalid Lens address rejection
9. selected block without hash rejection
10. empty requested lockEpochs rejection
11. invalid contract read result rejection
12. normalized Lens address and selected block number passed into reads
13. snapshot validation propagation through existing snapshot adapter
14. missing epoch returns null through resulting source

Additional edge-case test decision:

No additional tests are required before merging this review milestone.

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities

Current test count:

- 32 test files passed
- 227 tests passed

Conclusion:

The mocked Ethereum Lens provider adapter is safe to keep as the read-only provider boundary.

It proves finality block selection, selected-block read consistency, chain/address validation, and snapshot conversion without real network access.

Recommended next milestone:

xc-epoch-minimum-ethereum-provider-wrapper-design

Suggested next scope:

- design concrete provider wrapper boundary only
- decide whether viem or ethers wrapper should be used externally
- keep RPC URLs / env / API keys outside model code
- define how wrapper maps provider block reads to EthereumBlockSnapshot
- define how wrapper maps contract reads to unknown results
- do not implement real RPC until wrapper design is reviewed


## Latest XC epoch minimum Ethereum provider wrapper design checkpoint

The XC epoch minimum Ethereum provider wrapper design milestone was completed on the xc-epoch-minimum-ethereum-provider-wrapper-design branch.

Commits:

- bc084d9 Add XC epoch minimum Ethereum provider wrapper design

This was a design-only milestone.

No runtime behavior changed.

Design document added:

- implementation/xc-epoch-minimum-ethereum-provider-wrapper-design.md

Purpose:

Design the concrete Ethereum provider wrapper boundary for the XC epoch minimum provider adapter.

This milestone does not implement:

- real RPC reads
- viem / ethers runtime code
- env reads
- secrets
- CLI commands

Current foundation:

The model-layer provider adapter is already implemented and reviewed:

EthereumReadProvider
-> createXcEpochMinimumSourceFromEthereumLensProvider()
-> EthereumXcLensEpochMinimumSnapshot
-> createXcEpochMinimumSourceFromEthereumLensSnapshot()
-> XcEpochMinimumSource

The model-layer adapter uses only a custom read-only provider interface:

- getChainId()
- getBlock()
- readContract()

It does not import concrete provider libraries and does not accept:

- RPC URL
- private key
- mnemonic
- API key
- signer
- wallet account
- env config

Design goal:

Future concrete provider wrapper should adapt an external Ethereum client into the already-reviewed EthereumReadProvider interface.

Recommended high-level flow:

outer integration / application layer
-> constructs concrete Ethereum client
-> wraps concrete client as EthereumReadProvider
-> passes wrapper into createXcEpochMinimumSourceFromEthereumLensProvider()
-> model layer remains provider-library agnostic

Wrapper responsibility:

The concrete wrapper should be responsible for:

- mapping provider chain ID reads to bigint
- mapping block reads to EthereumBlockSnapshot
- mapping contract reads to unknown
- translating finalized / safe / blockNumber requests into concrete client calls
- normalizing provider-specific null / missing block behavior
- surfacing read errors without exposing secrets

The wrapper should not:

- decide protocol economics
- compute epoch minimums
- perform Build-state validation

Model-layer boundary:

These files should remain free from concrete provider dependency imports:

- src/model/ethereum-xc-epoch-minimum-provider-source.ts
- src/model/ethereum-xc-epoch-minimum-source.ts
- src/model/xc-epoch-minimum-source.ts

Do not import viem or ethers into model files.

If a real wrapper is added later, it should live outside the model source layer or in a clearly separated adapter / infrastructure layer.

Concrete provider library choice:

Two realistic choices were documented:

- viem
- ethers

Recommended first wrapper direction:

- viem-style read-only public client wrapper

Design decision:

- do not add either library yet
- keep implementation choice outside model layer
- prefer viem wrapper if the project later needs a concrete implementation and dependency fit is acceptable

No-secret construction boundary:

Allowed future pattern:

app / script / integration reads config
-> app / script / integration constructs concrete public client
-> wrapper receives public client object
-> wrapper implements EthereumReadProvider
-> model adapter receives wrapper

Disallowed pattern:

- wrapper reads process.env.RPC_URL
- wrapper reads process.env.ALCHEMY_KEY
- wrapper reads process.env.INFURA_KEY
- wrapper accepts private key
- wrapper accepts mnemonic
- wrapper accepts signer
- wrapper logs RPC URL
- wrapper logs authorization headers

RPC URL policy:

RPC URLs may exist only in outer infrastructure configuration.

They must not be passed into:

- createXcEpochMinimumSourceFromEthereumLensProvider()

If a concrete wrapper factory is later added, prefer:

- createEthereumReadProviderFromPublicClient(publicClient)

over:

- createEthereumReadProviderFromRpcUrl(rpcUrl)

Private key / signer policy:

The concrete provider wrapper must be read-only.

It must not support:

- private keys
- mnemonic phrases
- signers
- wallet clients
- transaction sending
- account mutation
- approvals
- writes

Block read mapping:

The wrapper must map EthereumBlockReadInput to concrete provider block reads:

- finalized -> concrete finalized block tag
- safe -> concrete safe block tag
- blockNumber -> concrete block number read
- empty input -> current head read for confirmed-policy calculation only

The wrapper should not reinterpret empty input as a provenance-safe block.

Block snapshot mapping:

The wrapper must map concrete block result to:

EthereumBlockSnapshot {
  number: bigint;
  hash: string | null;
  timestamp: bigint;
}

Required behavior:

- missing block -> null
- missing block number -> null or wrapper error
- missing hash -> hash: null
- timestamp -> bigint seconds
- timestamp conversion must be explicit if provider returns number / hex / Date-like value

Contract read mapping:

The wrapper must map EthereumContractReadInput to concrete provider readContract calls.

Required behavior:

- use exactly input.blockNumber for the contract read
- pass address as provided by model adapter
- pass abi as provided by caller / integration
- pass functionName as provided
- pass args as provided
- return raw decoded result as unknown

ABI handling:

The wrapper should not hardcode XC Lens ABI unless a later implementation milestone explicitly decides to include a minimal ABI module.

Preferred boundary:

- model adapter receives epochMinimumAbi as unknown
- wrapper passes abi through to concrete client
- outer integration chooses ABI

Testing strategy:

The concrete wrapper implementation should be tested with a mocked concrete client, not a real RPC endpoint.

Recommended tests include:

1. maps getChainId result to bigint
2. maps finalized block tag to concrete client getBlock
3. maps safe block tag to concrete client getBlock
4. maps blockNumber read to concrete client getBlock
5. maps empty getBlock input to head block read
6. maps missing block to null
7. maps block hash / number / timestamp into EthereumBlockSnapshot
8. maps readContract input with exact blockNumber
9. passes abi / functionName / args through unchanged
10. does not accept RPC URL
11. does not read process.env
12. does not require private key
13. does not require signer
14. does not expose secret-bearing config in errors

Non-goals:

This design does not add:

- real RPC execution
- env loading
- CLI command
- RPC URL factory
- private key support
- signer support
- transaction sending
- production address config
- snapshot persistence
- bridge signer verification
- X1-native verification

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities

Current test count:

- 32 test files passed
- 227 tests passed

Conclusion:

The concrete Ethereum provider wrapper should remain an outer read-only infrastructure adapter.

It should adapt a concrete public client to EthereumReadProvider without moving RPC URLs, env, API keys, signers, or provider-library dependencies into the model-layer XC epoch minimum source logic.

Recommended next milestone:

xc-epoch-minimum-ethereum-provider-wrapper-design-review

Suggested next scope:

- review concrete provider wrapper boundary
- verify design keeps model layer provider-library agnostic
- verify RPC URLs / env / API keys stay outside model code
- decide whether viem-style public client wrapper should be the first mocked implementation
- do not implement real RPC until wrapper design review is complete


## Latest XC epoch minimum Ethereum provider wrapper design review checkpoint

The XC epoch minimum Ethereum provider wrapper design review milestone was completed on the xc-epoch-minimum-ethereum-provider-wrapper-design-review branch.

Commits:

- 62ddadb Add XC epoch minimum Ethereum provider wrapper design review notes

This was a review-only milestone.

No runtime behavior changed.

Review note added:

- implementation/xc-epoch-minimum-ethereum-provider-wrapper-design-review-notes.md

Reviewed design:

- implementation/xc-epoch-minimum-ethereum-provider-wrapper-design.md

Reviewed prior commits:

- bc084d9 Add XC epoch minimum Ethereum provider wrapper design
- ac5f6ec Update checkpoint after XC epoch minimum Ethereum provider wrapper design
- b0780d5 Merge branch 'xc-epoch-minimum-ethereum-provider-wrapper-design'

Review conclusion:

The concrete Ethereum provider wrapper design boundary is clean.

The design remains design-only and correctly keeps the model-layer provider adapter independent from concrete Ethereum client libraries.

The intended future flow remains:

outer integration / application layer
-> constructs concrete Ethereum client
-> wraps concrete client as EthereumReadProvider
-> passes wrapper into createXcEpochMinimumSourceFromEthereumLensProvider()
-> model layer remains provider-library agnostic

Runtime scope review:

The design does not add:

- real RPC reads
- viem runtime code
- ethers runtime code
- env reads
- secrets
- CLI commands
- private key support
- signer support
- transaction sending

Model-layer boundary review:

The following model files must remain free from concrete provider dependency imports:

- src/model/ethereum-xc-epoch-minimum-provider-source.ts
- src/model/ethereum-xc-epoch-minimum-source.ts
- src/model/xc-epoch-minimum-source.ts

Review decision:

- do not import viem or ethers into model files
- keep concrete wrapper outside the model source layer
- keep EthereumReadProvider as the stable model-facing interface

Concrete provider library decision:

The design documents two realistic choices:

- viem
- ethers

Review decision:

- prefer viem-style public client wrapper for the first mocked wrapper implementation
- do not add real viem dependency or real RPC in the next milestone
- implement against a mocked concrete public-client shape first

No-secret boundary review:

The design correctly rejects putting RPC URLs, env, API keys, private keys, mnemonics, or signers into model code.

Allowed future pattern:

app / script / integration reads config
-> app / script / integration constructs concrete public client
-> wrapper receives public client object
-> wrapper implements EthereumReadProvider
-> model adapter receives wrapper

Disallowed pattern:

- wrapper reads process.env.RPC_URL
- wrapper reads process.env.ALCHEMY_KEY
- wrapper reads process.env.INFURA_KEY
- wrapper accepts private key
- wrapper accepts mnemonic
- wrapper accepts signer
- wrapper logs RPC URL
- wrapper logs authorization headers

RPC URL policy review:

RPC URLs may exist only in outer infrastructure configuration.

They must not be passed into:

- createXcEpochMinimumSourceFromEthereumLensProvider()

They also should not be passed into the model-layer wrapper interface.

Review decision:

- no direct RPC URL factory in the first wrapper implementation

Private key / signer policy review:

The wrapper must remain read-only.

It must not support:

- private keys
- mnemonic phrases
- signers
- wallet clients
- transaction sending
- account mutation
- approvals
- writes

Review decision:

- first wrapper implementation should accept only a read-only public-client-like object
- do not accept signer-capable client types if avoidable

Block read mapping review:

The design correctly maps EthereumBlockReadInput to concrete provider block reads:

- finalized -> concrete finalized block tag
- safe -> concrete safe block tag
- blockNumber -> concrete block number read
- empty input -> current head read for confirmed-policy calculation only

Review decision:

- keep empty input mapped to head only
- do not reinterpret empty input as finalized / safe provenance block

Block snapshot mapping review:

The wrapper must map concrete block results to EthereumBlockSnapshot:

- number: bigint
- hash: string | null
- timestamp: bigint

Review decision:

- missing block should map to null
- missing hash should map to hash: null
- timestamp conversion must be explicit
- block number conversion must be explicit

Contract read mapping review:

The wrapper must map EthereumContractReadInput to concrete readContract calls.

Review decision:

- use exactly input.blockNumber for each contract read
- pass address as provided
- pass abi as provided
- pass functionName as provided
- pass args as provided
- return raw decoded result as unknown
- do not validate epoch minimum economics in the wrapper

ABI boundary review:

The design correctly avoids hardcoding XC Lens ABI at this stage.

Review decision:

- wrapper passes abi through to concrete client
- outer integration chooses ABI
- do not add large ABI modules or unrelated contract interfaces

Error and logging review:

The wrapper should avoid leaking secrets in errors or logs.

Allowed context:

- chain ID
- missing block
- unsupported block tag
- readContract failed
- block number
- block tag
- contract address
- function name

Disallowed context:

- RPC URL
- API key
- authorization header
- full env config
- private key
- mnemonic

Review decision:

- wrapper should not log by default

Testing strategy review:

The design correctly requires mocked concrete-client tests, not real RPC tests.

Recommended first implementation tests:

1. maps getChainId result to bigint
2. maps finalized block tag to concrete client getBlock
3. maps safe block tag to concrete client getBlock
4. maps blockNumber read to concrete client getBlock
5. maps empty getBlock input to head block read
6. maps missing block to null
7. maps block hash / number / timestamp into EthereumBlockSnapshot
8. maps readContract input with exact blockNumber
9. passes abi / functionName / args through unchanged
10. does not accept RPC URL
11. does not read process.env
12. does not require private key
13. does not require signer
14. does not expose secret-bearing config in errors

Add one integration-style unit test:

mocked concrete client
-> concrete wrapper
-> createXcEpochMinimumSourceFromEthereumLensProvider()
-> source.authoritativeEpochMinimum(lockEpoch)

Still no real network.

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities

Current test count:

- 32 test files passed
- 227 tests passed

Conclusion:

The concrete Ethereum provider wrapper design is ready to proceed to a mocked wrapper implementation milestone.

The implementation should use a mocked viem-style public-client shape first and must not perform real Ethereum RPC yet.

Recommended next milestone:

xc-epoch-minimum-mocked-ethereum-provider-wrapper

Suggested next scope:

- implement wrapper against mocked viem-style public client shape
- no real RPC
- no env reads
- no secrets
- no private keys
- no signers
- no direct RPC URL factory
- test mapping into EthereumReadProvider
- test integration with existing mocked provider adapter


## Latest XC epoch minimum mocked Ethereum provider wrapper checkpoint

The XC epoch minimum mocked Ethereum provider wrapper milestone was completed on the xc-epoch-minimum-mocked-ethereum-provider-wrapper branch.

Commits:

- 14d800a Add mocked Ethereum read provider wrapper
- 8b67431 Add mocked Ethereum provider wrapper notes

This milestone implements the first mocked Ethereum provider wrapper against a viem-style public client shape.

Runtime additions:

- src/ethereum/ethereum-read-provider-wrapper.ts

Test additions:

- tests/ethereum-read-provider-wrapper.test.ts

Documentation additions:

- implementation/xc-epoch-minimum-mocked-ethereum-provider-wrapper-notes.md

Exports added through src/index.ts:

- EthereumPublicClientBlock
- EthereumPublicClientLike
- EthereumPublicClientGetBlockInput
- EthereumPublicClientReadContractInput
- createEthereumReadProviderFromPublicClient()

Purpose:

The wrapper adapts a mocked viem-style public client shape into the already-reviewed EthereumReadProvider interface.

Flow:

mocked public client shape
-> createEthereumReadProviderFromPublicClient(publicClient)
-> EthereumReadProvider
-> createXcEpochMinimumSourceFromEthereumLensProvider()

Boundary:

The wrapper lives outside src/model.

The model layer remains provider-library agnostic.

The wrapper imports the existing EthereumReadProvider model-facing types, but the model layer does not import the wrapper.

No viem or ethers dependency was introduced.

The public client interface is viem-style but mocked / dependency-free.

The wrapper does not accept:

- RPC URL
- private key
- mnemonic
- API key
- signer
- wallet account
- env config

Mapping behavior:

getChainId:

- number | bigint -> bigint

getBlock:

- { blockTag: "finalized" } -> publicClient.getBlock({ blockTag: "finalized" })
- { blockTag: "safe" } -> publicClient.getBlock({ blockTag: "safe" })
- { blockNumber } -> publicClient.getBlock({ blockNumber })
- {} -> publicClient.getBlock({ blockTag: "latest" })

The empty getBlock input maps to latest only for confirmed-policy head calculation in the existing provider adapter.

The wrapper does not reinterpret empty input as finalized or safe.

Block result mapping:

- missing block -> null
- missing block number -> null
- hash preserved as string | null
- timestamp number | bigint -> bigint

readContract:

- address passed unchanged
- abi passed unchanged
- functionName passed unchanged
- args passed unchanged
- blockNumber passed unchanged
- raw decoded result returned as unknown

Tests covered:

1. getChainId number result maps to bigint
2. getChainId bigint result maps to bigint
3. finalized block tag maps to public client getBlock
4. safe block tag maps to public client getBlock
5. number timestamp maps to bigint timestamp
6. blockNumber read maps to public client getBlock
7. empty getBlock input maps to latest head block read
8. missing block maps to null
9. missing block number maps to null
10. missing block hash maps to hash null
11. readContract input passes through unchanged
12. integration with existing Ethereum Lens provider adapter without real RPC

Security / operational boundary:

This milestone intentionally does not add:

- real Ethereum RPC
- env reads
- RPC URL factory
- private keys
- API keys
- mnemonic
- signer support
- wallet support
- transaction sending
- CLI commands
- production address config
- snapshot persistence
- bridge signer verification
- X1-native verification

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities

Current test count:

- 33 test files passed
- 238 tests passed

Conclusion:

The mocked Ethereum provider wrapper now adapts a viem-style public client shape into EthereumReadProvider without moving provider-library dependencies, RPC URLs, env, secrets, or signers into the model layer.

It proves the concrete wrapper boundary and integrates with the existing Ethereum Lens provider adapter without real network access.

Recommended next milestone:

xc-epoch-minimum-mocked-ethereum-provider-wrapper-review

Suggested next scope:

- review mocked wrapper boundary
- verify src/model remains provider-library agnostic
- verify no real RPC / env / secrets / signer support
- verify block/readContract mapping
- decide whether any extra wrapper edge-case tests are needed before real viem wrapper design


## Latest XC epoch minimum mocked Ethereum provider wrapper review checkpoint

The XC epoch minimum mocked Ethereum provider wrapper review milestone was completed on the xc-epoch-minimum-mocked-ethereum-provider-wrapper-review branch.

Commits:

- 999501f Add mocked Ethereum provider wrapper review notes

This was a review-only milestone.

No runtime behavior changed.

Review note added:

- implementation/xc-epoch-minimum-mocked-ethereum-provider-wrapper-review-notes.md

Reviewed runtime:

- src/ethereum/ethereum-read-provider-wrapper.ts

Reviewed tests:

- tests/ethereum-read-provider-wrapper.test.ts

Reviewed prior commits:

- 14d800a Add mocked Ethereum read provider wrapper
- 8b67431 Add mocked Ethereum provider wrapper notes
- 46be704 Update checkpoint after mocked Ethereum provider wrapper
- 85de67b Merge branch 'xc-epoch-minimum-mocked-ethereum-provider-wrapper'

Review conclusion:

The mocked Ethereum provider wrapper boundary is clean.

The wrapper lives in:

- src/ethereum/ethereum-read-provider-wrapper.ts

It does not live in src/model.

The model layer remains provider-library agnostic.

The model layer does not import the wrapper.

The wrapper imports model-facing EthereumReadProvider types, which is the intended dependency direction.

Runtime boundary review:

The wrapper does not import or call:

- process.env
- fetch
- http / https
- viem
- ethers
- wallet APIs
- signer APIs

The wrapper does not accept:

- RPC URL
- private key
- mnemonic
- API key
- signer
- wallet account
- env config

A targeted grep over the runtime file, test file, and notes file found RPC / secret / provider-library terms only in the notes file where they are documented as non-goals.

A targeted grep over src/model found no references to:

- ethereum-read-provider-wrapper
- createEthereumReadProviderFromPublicClient

This confirms that src/model does not depend on the wrapper.

Public client shape review:

The wrapper uses a mocked viem-style public client shape:

- getChainId()
- getBlock()
- readContract()

This remains dependency-free.

No real viem dependency was introduced.

No ethers dependency was introduced.

Mapping review:

getChainId:

- number | bigint -> bigint

getBlock:

- { blockTag: "finalized" } -> publicClient.getBlock({ blockTag: "finalized" })
- { blockTag: "safe" } -> publicClient.getBlock({ blockTag: "safe" })
- { blockNumber } -> publicClient.getBlock({ blockNumber })
- {} -> publicClient.getBlock({ blockTag: "latest" })

Review decision:

- empty input maps to latest only for confirmed-policy head calculation
- wrapper does not reinterpret empty input as finalized or safe

Block result mapping:

- missing block -> null
- missing block number -> null
- hash preserved as string | null
- timestamp number | bigint -> bigint

readContract:

- address passed unchanged
- abi passed unchanged
- functionName passed unchanged
- args passed unchanged
- blockNumber passed unchanged
- raw decoded result returned as unknown

Test coverage review:

The current tests cover:

1. getChainId number result maps to bigint
2. getChainId bigint result maps to bigint
3. finalized block tag maps to public client getBlock
4. safe block tag maps to public client getBlock
5. number timestamp maps to bigint timestamp
6. blockNumber read maps to public client getBlock
7. empty getBlock input maps to latest head block read
8. missing block maps to null
9. missing block number maps to null
10. missing block hash maps to hash null
11. readContract input passes through unchanged
12. integration with existing Ethereum Lens provider adapter without real RPC

Additional edge-case test decision:

No additional tests are required before merging this review milestone.

Possible future tests for real viem wrapper design / implementation may include:

- viem-specific block timestamp shape
- viem-specific block number shape
- viem-specific null block response
- viem-specific readContract error mapping
- finalized / safe support differences across providers

Those are not required in the current dependency-free mocked wrapper layer.

Security / operational review:

This milestone does not add:

- real Ethereum RPC
- env reads
- RPC URL factory
- private keys
- API keys
- mnemonic
- signer support
- wallet support
- transaction sending
- CLI commands
- production address config
- snapshot persistence
- bridge signer verification
- X1-native verification

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities

Current test count:

- 33 test files passed
- 238 tests passed

Conclusion:

The mocked Ethereum provider wrapper is safe to keep as the dependency-free infrastructure wrapper boundary.

It adapts a viem-style public client shape into EthereumReadProvider without moving provider-library dependencies, RPC URLs, env, secrets, or signers into the model layer.

Recommended next milestone:

xc-epoch-minimum-real-viem-wrapper-design

Suggested next scope:

- design real viem wrapper boundary only
- decide exact viem public client type shape
- define viem getBlock mapping
- define viem readContract mapping
- define error redaction policy
- keep RPC URL / env / API keys outside model code
- do not implement real RPC until design review is complete


## Latest XC epoch minimum real viem wrapper design checkpoint

The XC epoch minimum real viem wrapper design milestone was completed on the xc-epoch-minimum-real-viem-wrapper-design branch.

Commits:

- 9289deb Add XC epoch minimum real viem wrapper design

This was a design-only milestone.

No runtime behavior changed.

Design document added:

- implementation/xc-epoch-minimum-real-viem-wrapper-design.md

Purpose:

Design the real viem wrapper boundary for the XC epoch minimum Ethereum provider path.

This milestone does not implement:

- real RPC reads
- npm install viem
- runtime viem imports
- env reads
- secrets
- private keys
- signers
- direct RPC URL factory

Current foundation:

The dependency-free wrapper is already implemented and reviewed:

- src/ethereum/ethereum-read-provider-wrapper.ts

It adapts a mocked viem-style public client shape into EthereumReadProvider.

The provider path is:

public client-like object
-> createEthereumReadProviderFromPublicClient(publicClient)
-> EthereumReadProvider
-> createXcEpochMinimumSourceFromEthereumLensProvider()
-> EthereumXcLensEpochMinimumSnapshot
-> createXcEpochMinimumSourceFromEthereumLensSnapshot()
-> XcEpochMinimumSource

The model layer remains provider-library agnostic.

Design goal:

The real viem wrapper should adapt an actual viem PublicClient into the existing EthereumReadProvider interface without changing the model layer.

Future flow:

outer integration / app / script layer
-> creates viem PublicClient
-> passes PublicClient to viem wrapper
-> viem wrapper exposes EthereumReadProvider
-> existing provider adapter consumes EthereumReadProvider

Boundary rule:

The model layer must not import viem.

These files must remain free from viem imports:

- src/model/ethereum-xc-epoch-minimum-provider-source.ts
- src/model/ethereum-xc-epoch-minimum-source.ts
- src/model/xc-epoch-minimum-source.ts

Recommended future wrapper location:

- src/ethereum/ethereum-viem-read-provider-wrapper.ts

Dependency direction:

Allowed:

src/ethereum/ethereum-viem-read-provider-wrapper.ts
-> imports viem types / functions if needed
-> imports EthereumReadProvider model-facing types

Disallowed:

src/model/*
-> imports viem

src/model/*
-> imports real viem wrapper

The model layer must only know about EthereumReadProvider.

PublicClient construction boundary:

Allowed future pattern:

outer integration reads config
-> outer integration creates viem PublicClient
-> wrapper receives PublicClient
-> wrapper returns EthereumReadProvider

Disallowed in wrapper:

- createPublicClient({ transport: http(process.env.RPC_URL) })
- process.env reads
- direct RPC URL input
- API key input
- private key input
- wallet client input
- signer input

Preferred future factory shape:

- createEthereumReadProviderFromViemPublicClient(publicClient)

Avoid in first implementation:

- createEthereumReadProviderFromRpcUrl(rpcUrl)

Real viem PublicClient shape:

The real wrapper should depend only on read-only PublicClient capabilities:

- getChainId()
- getBlock()
- readContract()

It should not require:

- walletClient
- account
- signer
- sendTransaction
- writeContract
- private key
- mnemonic

Chain ID mapping:

- viem getChainId returns number
- wrapper maps number -> bigint
- provider adapter later maps bigint into eip155-<number>
- wrapper does not decide chain correctness

Block read mapping:

Existing EthereumBlockReadInput supports:

- { blockTag: "finalized" }
- { blockTag: "safe" }
- { blockNumber: bigint }
- {}

Real viem wrapper mapping:

- finalized -> publicClient.getBlock({ blockTag: "finalized" })
- safe -> publicClient.getBlock({ blockTag: "safe" })
- blockNumber -> publicClient.getBlock({ blockNumber })
- {} -> publicClient.getBlock({ blockTag: "latest" })

The empty input remains only for confirmed-policy head calculation.

The wrapper must not reinterpret empty input as finalized or safe.

Finality support caveat:

The wrapper should not silently downgrade:

- finalized -> latest
- safe -> latest

If viem / provider returns an error for unsupported finalized or safe block tags, the wrapper should surface a sanitized read error.

Block result mapping:

Viem block result should be mapped into EthereumBlockSnapshot:

- number: bigint
- hash: string | null
- timestamp: bigint

Mapping policy:

- missing block -> null
- missing block number -> null
- missing hash -> hash: null
- timestamp -> bigint

Contract read mapping:

Real viem wrapper maps EthereumContractReadInput to:

publicClient.readContract({
  address,
  abi,
  functionName,
  args,
  blockNumber
})

Required behavior:

- pass address unchanged
- pass abi unchanged
- pass functionName unchanged
- pass args unchanged
- pass blockNumber unchanged
- return decoded result as unknown
- do not validate epoch minimum economics in wrapper

Address / ABI boundary:

- model-facing address remains string
- wrapper may cast after model/provider adapter validation
- model-facing abi remains unknown
- wrapper passes abi through to viem readContract
- avoid hardcoding XC Lens ABI in wrapper

Error redaction policy:

Allowed error context:

- operation name
- block tag
- block number
- chain ID
- contract address
- function name

Disallowed error context:

- RPC URL
- API key
- authorization header
- env dump
- private key
- mnemonic
- signer object
- full transport config

The wrapper should not log by default.

Testing strategy for future implementation:

The first real viem wrapper implementation should still use mocked viem PublicClient objects.

No real RPC test in the implementation milestone.

Recommended tests include:

1. maps viem getChainId number to bigint
2. maps finalized block tag to publicClient.getBlock({ blockTag: "finalized" })
3. maps safe block tag to publicClient.getBlock({ blockTag: "safe" })
4. maps blockNumber to publicClient.getBlock({ blockNumber })
5. maps empty input to publicClient.getBlock({ blockTag: "latest" })
6. maps null block to null
7. maps null block number to null
8. maps null block hash to hash null
9. maps bigint timestamp to bigint
10. maps number timestamp to bigint if test shape allows number
11. passes readContract address / abi / functionName / args / blockNumber unchanged
12. returns readContract result as unknown
13. propagates sanitized getBlock errors
14. propagates sanitized readContract errors
15. does not read process.env
16. does not accept RPC URL
17. does not require private key
18. does not require signer / wallet client
19. integration with createXcEpochMinimumSourceFromEthereumLensProvider using mocked viem client

Dependency policy:

The design review should decide whether the next implementation adds a real viem dev/runtime dependency or continues with structural typing only.

Preferred cautious path:

- keep wrapper structurally typed first
- no npm install viem until an implementation truly needs official types
- if viem is added, keep import isolated in src/ethereum only

Non-goals:

This design does not add:

- real Ethereum RPC
- viem runtime code
- npm install viem
- env reads
- RPC URL factory
- private key support
- signer support
- wallet support
- transaction sending
- CLI commands
- production address config
- snapshot persistence
- bridge signer verification
- X1-native verification

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities

Current test count:

- 33 test files passed
- 238 tests passed

Conclusion:

The real viem wrapper should remain a read-only infrastructure adapter outside the model layer.

It should adapt a viem PublicClient into EthereumReadProvider while keeping RPC URLs, env, API keys, signers, wallets, and real network execution outside this layer.

Recommended next milestone:

xc-epoch-minimum-real-viem-wrapper-design-review

Suggested next scope:

- review real viem wrapper design
- confirm no runtime viem/RPC was added
- confirm model layer remains viem-free
- decide whether next implementation should stay structurally typed first
- do not implement real RPC until design review is complete


## Latest XC epoch minimum real viem wrapper design review checkpoint

The XC epoch minimum real viem wrapper design review milestone was completed on the xc-epoch-minimum-real-viem-wrapper-design-review branch.

Commits:

- e9e2f16 Add XC epoch minimum real viem wrapper design review notes

This was a review-only milestone.

No runtime behavior changed.

Review note added:

- implementation/xc-epoch-minimum-real-viem-wrapper-design-review-notes.md

Reviewed design:

- implementation/xc-epoch-minimum-real-viem-wrapper-design.md

Reviewed prior commits:

- 9289deb Add XC epoch minimum real viem wrapper design
- fb8f264 Update checkpoint after XC epoch minimum real viem wrapper design
- 0717d05 Merge branch 'xc-epoch-minimum-real-viem-wrapper-design'

Review conclusion:

The real viem wrapper design boundary is clean.

This remains a design-only stage.

No runtime viem imports were added.

No viem dependency was installed.

No real RPC behavior was added.

The model layer remains viem-free.

Runtime / dependency review:

A targeted grep confirmed:

- no real viem imports in src
- no ethers imports in src
- no process.env reads in runtime wrapper path
- no direct RPC URL factory
- no signer / wallet / write path in runtime wrapper path

The design document contains viem / RPC / env / secret terms only as explicit boundary rules, non-goals, and future implementation policy.

Model-layer boundary review:

The following model files must remain free from viem imports:

- src/model/ethereum-xc-epoch-minimum-provider-source.ts
- src/model/ethereum-xc-epoch-minimum-source.ts
- src/model/xc-epoch-minimum-source.ts

Review decision:

- keep src/model provider-library agnostic
- keep EthereumReadProvider as the model-facing abstraction
- place real viem wrapper outside src/model

Recommended future location remains:

- src/ethereum/ethereum-viem-read-provider-wrapper.ts

PublicClient construction boundary review:

The design correctly prevents the wrapper from constructing a viem PublicClient from RPC URL.

Allowed future pattern:

outer integration reads config
-> outer integration creates viem PublicClient
-> wrapper receives PublicClient
-> wrapper returns EthereumReadProvider

Disallowed in wrapper:

- createPublicClient({ transport: http(process.env.RPC_URL) })
- process.env reads
- direct RPC URL input
- API key input
- private key input
- wallet client input
- signer input

Review decision:

- keep createEthereumReadProviderFromViemPublicClient(publicClient)-style construction
- do not add createEthereumReadProviderFromRpcUrl(rpcUrl) in the next implementation

Real viem PublicClient shape review:

The wrapper should depend only on read-only PublicClient capabilities:

- getChainId()
- getBlock()
- readContract()

The wrapper must not require:

- walletClient
- account
- signer
- sendTransaction
- writeContract
- private key
- mnemonic

Review decision:

- next implementation should stay read-only and structurally typed first
- do not add real RPC execution yet

Mapping review:

Chain ID:

- viem getChainId number -> bigint

Block reads:

- finalized -> publicClient.getBlock({ blockTag: "finalized" })
- safe -> publicClient.getBlock({ blockTag: "safe" })
- blockNumber -> publicClient.getBlock({ blockNumber })
- {} -> publicClient.getBlock({ blockTag: "latest" })

Review decision:

- empty input remains latest only for confirmed-policy head calculation
- do not reinterpret empty input as finalized or safe
- do not silently downgrade finalized / safe to latest
- if provider does not support finalized / safe, surface a sanitized error

Block mapping:

- missing block -> null
- missing block number -> null
- missing hash -> hash: null
- timestamp -> bigint

Contract read mapping:

- pass address unchanged
- pass abi unchanged
- pass functionName unchanged
- pass args unchanged
- pass blockNumber unchanged
- return decoded result as unknown
- do not validate epoch minimum economics in the wrapper

Address / ABI boundary review:

The design keeps:

- model-facing address as string
- model-facing abi as unknown

Review decision:

- wrapper may cast after provider adapter validation
- wrapper should not loosen validation in model layer
- wrapper should not hardcode XC Lens ABI at this stage

Error redaction review:

Allowed error context:

- operation name
- block tag
- block number
- chain ID
- contract address
- function name

Disallowed error context:

- RPC URL
- API key
- authorization header
- env dump
- private key
- mnemonic
- signer object
- full transport config

Review decision:

- wrapper should not log by default
- future wrapped errors should use sanitized messages

Dependency policy decision:

The next implementation should stay structurally typed first.

Do not install viem yet unless the implementation clearly needs official types.

If viem is later added, keep imports isolated in:

- src/ethereum

and never in:

- src/model

Testing strategy review:

The next implementation should use mocked viem PublicClient objects.

No real RPC test in the implementation milestone.

Recommended tests:

1. maps viem getChainId number to bigint
2. maps finalized block tag to publicClient.getBlock({ blockTag: "finalized" })
3. maps safe block tag to publicClient.getBlock({ blockTag: "safe" })
4. maps blockNumber to publicClient.getBlock({ blockNumber })
5. maps empty input to publicClient.getBlock({ blockTag: "latest" })
6. maps null block to null
7. maps null block number to null
8. maps null block hash to hash null
9. maps bigint timestamp to bigint
10. maps number timestamp to bigint if test shape allows number
11. passes readContract address / abi / functionName / args / blockNumber unchanged
12. returns readContract result as unknown
13. propagates sanitized getBlock errors
14. propagates sanitized readContract errors
15. does not read process.env
16. does not accept RPC URL
17. does not require private key
18. does not require signer / wallet client
19. integration with createXcEpochMinimumSourceFromEthereumLensProvider using mocked viem client

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities

Current test count:

- 33 test files passed
- 238 tests passed

Conclusion:

The real viem wrapper design is ready for a mocked implementation milestone.

The next implementation should stay structurally typed, use mocked viem PublicClient objects, and still avoid real RPC, env reads, RPC URL factories, private keys, signers, wallets, and transaction sending.

Recommended next milestone:

xc-epoch-minimum-mocked-real-viem-wrapper

Suggested next scope:

- implement real viem wrapper boundary with mocked viem client
- no npm install viem unless clearly needed
- no real RPC
- no env reads
- no secrets
- no RPC URL factory
- no private keys
- no signers
- no wallet client
- no transaction sending
- tests only with mocked viem client
- integration test with existing provider adapter


## Latest XC epoch minimum mocked real viem wrapper checkpoint

The XC epoch minimum mocked real viem wrapper milestone was completed on the xc-epoch-minimum-mocked-real-viem-wrapper branch.

Commits:

- b6eb538 Add mocked real viem read provider wrapper
- e602c59 Add mocked real viem wrapper notes

This milestone implements the real viem wrapper boundary with a structurally typed mocked viem PublicClient.

Runtime additions:

- src/ethereum/ethereum-viem-read-provider-wrapper.ts

Test additions:

- tests/ethereum-viem-read-provider-wrapper.test.ts

Documentation additions:

- implementation/xc-epoch-minimum-mocked-real-viem-wrapper-notes.md

Exports added through src/index.ts:

- ViemLikeBlock
- ViemLikePublicClient
- ViemLikeGetBlockInput
- ViemLikeReadContractInput
- createEthereumReadProviderFromViemPublicClient()

Purpose:

The wrapper adapts a viem-like PublicClient shape into the already-reviewed EthereumReadProvider interface.

Flow:

viem-like PublicClient
-> createEthereumReadProviderFromViemPublicClient(publicClient)
-> EthereumReadProvider
-> createXcEpochMinimumSourceFromEthereumLensProvider()

Boundary:

The wrapper lives outside src/model.

The model layer remains provider-library agnostic.

No viem dependency was installed.

No viem runtime import was added.

No ethers dependency or import was added.

The wrapper imports only model-facing EthereumReadProvider types and exposes an infrastructure adapter from viem-like client shape to EthereumReadProvider.

Public client shape:

The wrapper accepts a structurally typed read-only public client:

- getChainId()
- getBlock()
- readContract()

The wrapper does not accept:

- RPC URL
- private key
- mnemonic
- API key
- signer
- wallet client
- account
- env config

Mapping behavior:

getChainId:

- number -> bigint

getBlock:

- { blockTag: "finalized" } -> publicClient.getBlock({ blockTag: "finalized" })
- { blockTag: "safe" } -> publicClient.getBlock({ blockTag: "safe" })
- { blockNumber } -> publicClient.getBlock({ blockNumber })
- {} -> publicClient.getBlock({ blockTag: "latest" })

The empty getBlock input maps to latest only for confirmed-policy head calculation.

The wrapper does not reinterpret empty input as finalized or safe.

The wrapper does not silently downgrade finalized or safe to latest.

Block result mapping:

- null block -> null
- null block number -> null
- null block hash -> hash: null
- timestamp number | bigint -> bigint

readContract:

- address passed unchanged
- abi passed unchanged
- functionName passed unchanged
- args passed unchanged
- blockNumber passed unchanged
- decoded result returned as unknown

The wrapper does not validate epoch minimum economics.

Tests covered:

1. viem getChainId number maps to bigint
2. finalized block tag maps to viem getBlock
3. safe block tag maps to viem getBlock
4. number timestamp maps to bigint timestamp
5. blockNumber read maps to viem getBlock
6. empty getBlock input maps to latest head block read
7. null block maps to null
8. null block number maps to null
9. null block hash maps to hash null
10. readContract input passes through unchanged
11. readContract result returns as unknown
12. getBlock errors propagate without adding secret-bearing config
13. readContract errors propagate without adding secret-bearing config
14. integration with existing Ethereum Lens provider adapter without real RPC

Security / operational boundary:

This milestone intentionally does not add:

- real Ethereum RPC
- viem dependency
- viem runtime imports
- ethers dependency
- env reads
- RPC URL factory
- private keys
- API keys
- mnemonic
- signer support
- wallet client support
- account support
- transaction sending
- CLI commands
- production address config
- snapshot persistence
- bridge signer verification
- X1-native verification

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities

Current test count:

- 34 test files passed
- 251 tests passed

Conclusion:

The mocked real viem wrapper now proves the intended real viem wrapper boundary while remaining structurally typed and dependency-free.

It adapts a viem-like PublicClient into EthereumReadProvider and integrates with the existing Ethereum Lens provider adapter without real network access.

Recommended next milestone:

xc-epoch-minimum-mocked-real-viem-wrapper-review

Suggested next scope:

- review mocked real viem wrapper boundary
- verify no viem dependency / runtime imports
- verify no real RPC / env / secrets / RPC URL factory
- verify src/model remains provider-library agnostic
- verify mapping and tests before moving toward real read-only RPC integration design


## Latest XC epoch minimum mocked real viem wrapper review checkpoint

The XC epoch minimum mocked real viem wrapper review milestone was completed on the xc-epoch-minimum-mocked-real-viem-wrapper-review branch.

Commits:

- cd0d3a3 Add mocked real viem wrapper review notes

This was a review-only milestone.

No runtime behavior changed.

Review note added:

- implementation/xc-epoch-minimum-mocked-real-viem-wrapper-review-notes.md

Reviewed runtime:

- src/ethereum/ethereum-viem-read-provider-wrapper.ts

Reviewed tests:

- tests/ethereum-viem-read-provider-wrapper.test.ts

Reviewed prior commits:

- b6eb538 Add mocked real viem read provider wrapper
- e602c59 Add mocked real viem wrapper notes
- 90ffb59 Update checkpoint after mocked real viem wrapper
- 999f6ce Merge branch 'xc-epoch-minimum-mocked-real-viem-wrapper'

Review conclusion:

The mocked real viem wrapper boundary is clean.

The wrapper lives in:

- src/ethereum/ethereum-viem-read-provider-wrapper.ts

It does not live in src/model.

The model layer remains provider-library agnostic.

The model layer does not import the viem wrapper.

The wrapper imports only model-facing EthereumReadProvider types, which is the intended dependency direction.

Runtime / dependency boundary review:

The wrapper does not import or call:

- viem
- ethers
- process.env
- fetch
- http / https
- createPublicClient
- wallet APIs
- signer APIs
- transaction APIs

The wrapper does not accept:

- RPC URL
- private key
- mnemonic
- API key
- signer
- wallet client
- account
- env config

A targeted grep over the runtime file, test file, and notes file found RPC / secret / provider-library terms only in notes and test descriptions where they are documented as boundaries or non-goals.

A targeted grep over src/model found no references to:

- ethereum-viem-read-provider-wrapper
- createEthereumReadProviderFromViemPublicClient
- viem
- ethers

This confirms that src/model remains provider-library agnostic.

Structurally typed client review:

The wrapper uses a structurally typed read-only viem-like client:

- getChainId()
- getBlock()
- readContract()

This is the correct first implementation boundary.

No npm install viem was required.

No official viem types are required yet.

Mapping review:

getChainId:

- number -> bigint

getBlock:

- { blockTag: "finalized" } -> publicClient.getBlock({ blockTag: "finalized" })
- { blockTag: "safe" } -> publicClient.getBlock({ blockTag: "safe" })
- { blockNumber } -> publicClient.getBlock({ blockNumber })
- {} -> publicClient.getBlock({ blockTag: "latest" })

Review decision:

- empty input maps to latest only for confirmed-policy head calculation
- wrapper does not reinterpret empty input as finalized or safe
- wrapper does not silently downgrade finalized or safe to latest

Block result mapping:

- null block -> null
- null block number -> null
- null block hash -> hash: null
- timestamp number | bigint -> bigint

readContract:

- address passed unchanged
- abi passed unchanged
- functionName passed unchanged
- args passed unchanged
- blockNumber passed unchanged
- decoded result returned as unknown

The wrapper does not validate epoch minimum economics.

Error propagation review:

The current implementation intentionally does not wrap errors.

It propagates getBlock and readContract errors as received from the supplied mocked client.

This is acceptable for the current dependency-free mocked milestone because the wrapper does not know about RPC URLs, env, headers, private keys, signers, or transport config.

Future real read-only RPC integration design should define whether outer integration sanitizes provider errors or whether the wrapper should wrap them with sanitized messages.

Test coverage review:

The current tests cover:

1. viem getChainId number maps to bigint
2. finalized block tag maps to viem getBlock
3. safe block tag maps to viem getBlock
4. number timestamp maps to bigint timestamp
5. blockNumber read maps to viem getBlock
6. empty getBlock input maps to latest head block read
7. null block maps to null
8. null block number maps to null
9. null block hash maps to hash null
10. readContract input passes through unchanged
11. readContract result returns as unknown
12. getBlock errors propagate without adding secret-bearing config
13. readContract errors propagate without adding secret-bearing config
14. integration with existing Ethereum Lens provider adapter without real RPC

Additional edge-case test decision:

No additional tests are required before merging this review milestone.

Possible future tests for real read-only RPC integration design / implementation may include:

- provider error redaction policy
- finalized / safe unsupported-provider behavior
- real viem block timestamp type confirmation
- real viem readContract decoded result shape
- outer config construction without exposing RPC URL
- integration-level chain mismatch handling with real provider wrapper

Those are not required in the current structurally typed mocked wrapper layer.

Security / operational review:

This milestone does not add:

- real Ethereum RPC
- viem dependency
- viem runtime imports
- ethers dependency
- env reads
- RPC URL factory
- private keys
- API keys
- mnemonic
- signer support
- wallet client support
- account support
- transaction sending
- CLI commands
- production address config
- snapshot persistence
- bridge signer verification
- X1-native verification

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities

Current test count:

- 34 test files passed
- 251 tests passed

Conclusion:

The mocked real viem wrapper is safe to keep as the structurally typed, dependency-free viem-like infrastructure boundary.

It adapts a viem-like PublicClient into EthereumReadProvider and integrates with the existing Ethereum Lens provider adapter without real network access.

Recommended next milestone:

xc-epoch-minimum-real-readonly-rpc-integration-design

Suggested next scope:

- design real read-only RPC integration boundary
- decide where public client is constructed
- define config and env ownership outside model / wrapper
- define RPC URL / API key redaction policy
- define provider error sanitization policy
- define finalized / safe unsupported-provider behavior
- keep model layer provider-library agnostic
- do not implement real RPC until design review is complete

## Current next steps

Potential next documents / design areas:

1. Design the real read-only RPC integration boundary.
2. Continue implementation only with clean typecheck and tests.




















