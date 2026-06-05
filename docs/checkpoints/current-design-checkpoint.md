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

## Current next steps

Potential next documents / design areas:

1. Continue implementation only with clean typecheck and tests.




















