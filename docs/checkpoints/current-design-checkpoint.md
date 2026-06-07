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


## Latest XC epoch minimum real read-only RPC integration design checkpoint

The XC epoch minimum real read-only RPC integration design milestone was completed on the xc-epoch-minimum-real-readonly-rpc-integration-design branch.

Commits:

- 861f1d6 Add XC epoch minimum real read-only RPC integration design

This was a design-only milestone.

No runtime behavior changed.

Design document added:

- implementation/xc-epoch-minimum-real-readonly-rpc-integration-design.md

Purpose:

Design the real read-only RPC integration boundary for the XC epoch minimum Ethereum provider path.

This milestone does not implement:

- real RPC reads
- runtime RPC execution
- env reads in model or wrapper code
- secret printing
- private keys
- signers
- wallet clients
- transaction sending

Current foundation:

The provider path currently has these completed layers:

EthereumReadProvider
-> Ethereum Lens provider adapter
-> EthereumXcLensEpochMinimumSnapshot
-> Ethereum snapshot adapter
-> XcEpochMinimumSource

The mocked real viem wrapper is already implemented and reviewed:

- src/ethereum/ethereum-viem-read-provider-wrapper.ts

It adapts a structurally typed viem-like PublicClient into EthereumReadProvider:

viem-like PublicClient
-> createEthereumReadProviderFromViemPublicClient(publicClient)
-> EthereumReadProvider

The model layer remains provider-library agnostic.

Design goal:

The real read-only RPC integration should define where a real public client is constructed and how it is passed into the already-reviewed wrapper without moving RPC URLs, env, API keys, provider construction, or secret-bearing config into the model layer or wrapper.

Future intended flow:

outer integration / script / app layer
-> reads config from a safe source
-> constructs real read-only public client
-> passes public client into createEthereumReadProviderFromViemPublicClient(publicClient)
-> passes EthereumReadProvider into createXcEpochMinimumSourceFromEthereumLensProvider()
-> receives XcEpochMinimumSource

Boundary rule:

The following layers must remain free from env reads and RPC URL construction:

- src/model/*
- src/ethereum/ethereum-viem-read-provider-wrapper.ts
- src/ethereum/ethereum-read-provider-wrapper.ts

The real RPC integration boundary should live outside src/model and outside the generic wrapper.

Recommended future location candidates:

- src/ethereum/ethereum-readonly-rpc-integration.ts
- src/integration/ethereum-readonly-rpc-integration.ts
- scripts/read-xc-epoch-minimum-source.ts

Preferred direction:

- keep reusable integration code outside src/model
- keep CLI / script env handling outside reusable library code if possible
- pass constructed public client objects inward

Config ownership:

Config ownership belongs to the outer integration layer.

Config may include:

- chainId
- lensAddress
- finalityPolicy
- lockEpochs
- epochMinimumFunctionName
- epochMinimumAbi
- public client object

Config must not include in model/wrapper input:

- RPC URL
- API key
- authorization header
- private key
- mnemonic
- signer
- wallet client
- account

Env ownership:

The model layer must not read process.env.

The wrapper layer must not read process.env.

If env is used later, it must be read only by an outer app / script / integration entrypoint.

Allowed future pattern:

script reads process.env.XC_ETHEREUM_RPC_URL
-> script creates public client
-> script passes public client to integration helper
-> integration helper passes client to wrapper
-> wrapper returns EthereumReadProvider
-> provider adapter produces XcEpochMinimumSource

Disallowed pattern:

- model reads process.env
- wrapper reads process.env
- provider adapter reads process.env
- source builder reads process.env

RPC URL / API key policy:

RPC URLs and API keys are secret-bearing or sensitive operational config.

They must not be:

- logged
- included in thrown error messages
- stored in snapshots
- stored in checkpoint records
- passed into model-layer constructors
- passed into createXcEpochMinimumSourceFromEthereumLensProvider()
- passed into createEthereumReadProviderFromViemPublicClient()

Preferred first real integration approach:

- create the public client in a script / app entrypoint
- pass the public client object inward

Avoid first:

- createEthereumReadProviderFromRpcUrl(rpcUrl)
- createXcEpochMinimumSourceFromRpcUrl(rpcUrl)

Read-only requirement:

The real integration path must be read-only.

It must not require:

- private key
- mnemonic
- signer
- wallet client
- account
- sendTransaction
- writeContract
- approve
- transaction simulation for writes

Allowed read-only calls:

- getChainId()
- getBlock()
- readContract()

Chain and address policy:

The integration should require explicit chainId and explicit Lens address.

The provider adapter already validates:

- configured chainId format
- provider chain ID match
- Lens address format
- finality policy
- lockEpochs
- selected block provenance
- read result shape

The outer integration should not bypass those checks.

Recommended future integration inputs:

- publicClient
- chainId
- lensAddress
- finalityPolicy
- lockEpochs
- epochMinimumFunctionName
- epochMinimumAbi

Finality policy:

Supported finality policies remain:

- finalized
- safe
- confirmed

Unsupported:

- latest as provenance policy

The real RPC integration should not silently change policy.

If finalized / safe is unsupported by a provider, the integration must surface a sanitized error.

Do not silently downgrade:

- finalized -> latest
- safe -> latest

If a fallback is ever added, it must be explicit in config and documented.

Confirmed policy behavior:

Confirmed policy may read the current head only to calculate an older confirmed block number.

Then all contract reads must use the selected confirmed block number.

Existing provider adapter behavior should remain the source of truth.

Provider error sanitization:

Allowed error context:

- operation name
- chain ID
- block tag
- block number
- contract address
- function name
- high-level provider failure category

Disallowed error context:

- RPC URL
- API key
- authorization header
- private key
- mnemonic
- signer object
- wallet account secret material
- full env dump
- transport internals that include URL / headers

Preferred cautious decision:

- outer integration owns secret-bearing config
- wrapper can wrap low-level errors only if it never includes transport config
- tests should verify no RPC URL / API key appears in thrown messages

Snapshot policy:

EthereumXcLensEpochMinimumSnapshot must not include:

- RPC URL
- API key
- env config
- provider object
- transport config
- private key
- signer
- wallet client

Snapshot may include:

- sourceChainId
- sourceBlockNumber
- sourceBlockHash
- observedAt
- finalizedPolicy
- epochMinimums

Logging policy:

No logging by default in reusable model / wrapper / provider layers.

Safe future script logs may include:

- chain ID
- finality policy
- selected block number
- selected block hash
- Lens address
- lockEpoch count
- function name

Unsafe fields:

- RPC URL
- API key
- authorization header
- env dump
- private key
- mnemonic
- signer / wallet internals
- transport config

ABI policy:

The real RPC integration should not hardcode a large ABI unless necessary.

Preferred initial approach:

- pass epochMinimumAbi explicitly from outer integration
- pass epochMinimumFunctionName explicitly or use a safe default

Possible later approach:

- add a minimal XC Lens ABI module
- keep it separate from model logic
- document source of ABI

Testing strategy for future implementation:

The next implementation milestone after design review should still avoid real RPC unless explicitly approved.

Recommended first implementation tests with mocked public client / mocked config:

1. constructs source from a provided public client
2. does not accept RPC URL in model-facing input
3. does not read process.env in model/wrapper/integration helper
4. passes public client into createEthereumReadProviderFromViemPublicClient()
5. passes EthereumReadProvider into createXcEpochMinimumSourceFromEthereumLensProvider()
6. preserves finalized finality policy
7. preserves safe finality policy
8. preserves confirmed finality policy
9. does not downgrade finalized / safe
10. propagates sanitized provider errors
11. error messages do not contain RPC URL
12. error messages do not contain API key
13. snapshot does not contain RPC URL / API key / env config
14. integration result works with authoritativeEpochMinimum(lockEpoch)

Real RPC tests should be a later separate milestone after design review and implementation review.

Non-goals:

This design does not add:

- real RPC execution
- viem dependency installation
- env reads
- RPC URL factory
- private key support
- signer support
- wallet client support
- transaction sending
- CLI command
- production address config
- snapshot persistence migration
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

The real read-only RPC integration should be an outer infrastructure boundary.

It should construct or receive a real public client outside the model and wrapper layers, keep RPC URL / API key / env ownership outside protocol logic, and pass only a read-only public client inward to the existing viem-like wrapper and provider adapter path.

Recommended next milestone:

xc-epoch-minimum-real-readonly-rpc-integration-design-review

Suggested next scope:

- review real read-only RPC integration design
- confirm no real RPC implementation was added
- confirm env/RPC URL ownership stays outside model and wrapper
- confirm snapshot/logging/error redaction policy
- decide whether first implementation should use a provided public client helper only
- do not implement real RPC until design review is complete


## Latest XC epoch minimum real read-only RPC integration design review checkpoint

The XC epoch minimum real read-only RPC integration design review milestone was completed on the xc-epoch-minimum-real-readonly-rpc-integration-design-review branch.

Commits:

- d3bbf9e Add real read-only RPC integration design review notes

This was a review-only milestone.

No runtime behavior changed.

Review note added:

- implementation/xc-epoch-minimum-real-readonly-rpc-integration-design-review-notes.md

Reviewed design:

- implementation/xc-epoch-minimum-real-readonly-rpc-integration-design.md

Reviewed prior commits:

- 861f1d6 Add XC epoch minimum real read-only RPC integration design
- eb35c64 Update checkpoint after real read-only RPC integration design
- 7736504 Merge branch 'xc-epoch-minimum-real-readonly-rpc-integration-design'

Review conclusion:

The real read-only RPC integration design boundary is clean.

This remains a design-only stage.

No real RPC implementation was added.

No runtime RPC execution was added.

No env reads were added in model or wrapper code.

No private keys, signers, wallet clients, or transaction-sending paths were added.

Runtime / dependency review:

A targeted grep confirmed that the design document contains RPC / env / secret / viem terms only as boundary rules, policy, conceptual examples, and non-goals.

The runtime source remains free from:

- real viem imports
- ethers imports
- process.env reads
- createPublicClient calls
- http transport construction
- RPC URL factories
- signer / wallet / write paths

The current branch remains review-only.

Model and wrapper boundary review:

The following layers must remain free from env reads and RPC URL construction:

- src/model/*
- src/ethereum/ethereum-viem-read-provider-wrapper.ts
- src/ethereum/ethereum-read-provider-wrapper.ts

Review decision:

- keep model layer provider-library agnostic
- keep wrappers free from RPC URL ownership
- keep public client construction outside model and wrapper
- pass constructed public client objects inward

Config and env ownership review:

Config ownership belongs to the outer integration layer.

Allowed future integration config:

- chainId
- lensAddress
- finalityPolicy
- lockEpochs
- epochMinimumFunctionName
- epochMinimumAbi
- public client object

Disallowed in model / wrapper input:

- RPC URL
- API key
- authorization header
- private key
- mnemonic
- signer
- wallet client
- account

If env is used later, it must be read only by an outer app / script / integration entrypoint.

Review decision:

- do not read process.env in model
- do not read process.env in wrapper
- do not read process.env in provider adapter
- do not read process.env in source builder

RPC URL / API key policy review:

RPC URLs and API keys are sensitive operational config.

They must not be:

- logged
- included in thrown error messages
- stored in snapshots
- stored in checkpoint records
- passed into model-layer constructors
- passed into createXcEpochMinimumSourceFromEthereumLensProvider()
- passed into createEthereumReadProviderFromViemPublicClient()

Review decision:

- first implementation should use a provided public client helper only
- do not add createEthereumReadProviderFromRpcUrl(rpcUrl)
- do not add createXcEpochMinimumSourceFromRpcUrl(rpcUrl)

Public client construction review:

The design correctly treats public client construction as an outer infrastructure concern.

Conceptual viem construction may happen later only outside model and wrapper code.

Review decision:

- do not add real public client construction in this design review
- do not add viem dependency in this review
- do not add real RPC tests in this review

Read-only requirement review:

The real integration path must remain read-only.

It must not require:

- private key
- mnemonic
- signer
- wallet client
- account
- sendTransaction
- writeContract
- approve
- transaction simulation for writes

Allowed read-only calls remain:

- getChainId()
- getBlock()
- readContract()

Chain and address policy review:

The integration should require explicit chainId and explicit Lens address.

The provider adapter already validates:

- configured chainId format
- provider chain ID match
- Lens address format
- finality policy
- lockEpochs
- selected block provenance
- read result shape

Review decision:

- outer integration must not bypass provider adapter checks
- future integration helper should pass inputs into existing provider adapter path

Finality policy review:

Supported finality policies remain:

- finalized
- safe
- confirmed

Unsupported as provenance policy:

- latest

Review decision:

- do not silently change finality policy
- do not silently downgrade finalized to latest
- do not silently downgrade safe to latest
- if finalized / safe is unsupported by a provider, surface a sanitized error
- any future fallback must be explicit in config and documented

Provider error sanitization review:

Allowed error context:

- operation name
- chain ID
- block tag
- block number
- contract address
- function name
- high-level provider failure category

Disallowed error context:

- RPC URL
- API key
- authorization header
- private key
- mnemonic
- signer object
- wallet account secret material
- full env dump
- transport internals that include URL / headers

Review decision:

- outer integration owns secret-bearing config
- future implementation must verify no RPC URL / API key appears in thrown messages
- wrapper may wrap low-level errors only if it never includes transport config

Snapshot policy review:

EthereumXcLensEpochMinimumSnapshot must not include:

- RPC URL
- API key
- env config
- provider object
- transport config
- private key
- signer
- wallet client

Snapshot may include:

- sourceChainId
- sourceBlockNumber
- sourceBlockHash
- observedAt
- finalizedPolicy
- epochMinimums

Review decision:

- keep snapshots portable and non-secret

Logging policy review:

No logging by default in reusable model / wrapper / provider layers.

Safe future script logs may include:

- chain ID
- finality policy
- selected block number
- selected block hash
- Lens address
- lockEpoch count
- function name

Unsafe fields include:

- RPC URL
- API key
- authorization header
- env dump
- private key
- mnemonic
- signer / wallet internals
- transport config

ABI policy review:

The real RPC integration should not hardcode a large ABI unless necessary.

Review decision:

- pass epochMinimumAbi explicitly from outer integration first
- pass epochMinimumFunctionName explicitly or use a safe default
- possible later minimal XC Lens ABI module must remain separate from model logic

Testing strategy review:

The next implementation milestone should still avoid real RPC.

Recommended mocked implementation tests:

1. constructs source from a provided public client
2. does not accept RPC URL in model-facing input
3. does not read process.env in model / wrapper / integration helper
4. passes public client into createEthereumReadProviderFromViemPublicClient()
5. passes EthereumReadProvider into createXcEpochMinimumSourceFromEthereumLensProvider()
6. preserves finalized finality policy
7. preserves safe finality policy
8. preserves confirmed finality policy
9. does not downgrade finalized / safe
10. propagates sanitized provider errors
11. error messages do not contain RPC URL
12. error messages do not contain API key
13. snapshot does not contain RPC URL / API key / env config
14. integration result works with authoritativeEpochMinimum(lockEpoch)

Real RPC tests should be a later separate milestone after mocked integration implementation and review.

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities

Current test count:

- 34 test files passed
- 251 tests passed

Conclusion:

The real read-only RPC integration design is ready for a mocked integration implementation milestone.

The next implementation should use a provided public client helper only, avoid real RPC, avoid env reads in model / wrapper / helper, avoid RPC URL factories, and verify sanitized error behavior.

Recommended next milestone:

xc-epoch-minimum-mocked-readonly-rpc-integration

Suggested next scope:

- implement integration helper using provided public client
- no real RPC
- no env reads in model / wrapper / helper
- no secrets
- no RPC URL factory
- no private keys
- no signers
- no wallet client
- tests with mocked public client
- verify sanitized errors
- verify source integration


## Latest XC epoch minimum mocked read-only RPC integration checkpoint

The XC epoch minimum mocked read-only RPC integration milestone was completed on the xc-epoch-minimum-mocked-readonly-rpc-integration branch.

Commits:

- cd29e08 Add mocked read-only RPC integration helper
- f83b1ac Add mocked read-only RPC integration notes

This milestone implements the mocked read-only RPC integration helper using a provided public client.

Runtime additions:

- src/ethereum/ethereum-readonly-rpc-integration.ts

Test additions:

- tests/ethereum-readonly-rpc-integration.test.ts

Documentation additions:

- implementation/xc-epoch-minimum-mocked-readonly-rpc-integration-notes.md

Exports added through src/index.ts:

- EthereumReadonlyRpcIntegrationInput
- createXcEpochMinimumSourceFromReadonlyEthereumPublicClient()
- createEthereumReadProviderFromReadonlyEthereumPublicClient()

Purpose:

The helper proves the intended outer integration boundary using a provided public client.

It keeps RPC URL / API key / env ownership outside model, wrapper, and helper code while integrating with the existing Ethereum provider adapter path.

Flow:

provided public client
-> createEthereumReadProviderFromReadonlyEthereumPublicClient(publicClient)
-> EthereumReadProvider
-> createXcEpochMinimumSourceFromEthereumLensProvider()
-> XcEpochMinimumSource

Boundary:

The helper lives outside src/model.

The helper does not construct a public client.

The helper receives an already-created public client object.

The helper does not accept:

- RPC URL
- API key
- authorization header
- private key
- mnemonic
- signer
- wallet client
- account
- env config

No viem dependency was installed.

No viem runtime import was added.

No ethers dependency or import was added.

No real RPC execution was added.

Input shape:

The helper accepts:

- publicClient
- chainId
- lensAddress
- finalityPolicy
- lockEpochs
- optional epochMinimumFunctionName
- optional epochMinimumAbi

The optional fields are only forwarded when present, preserving exactOptionalPropertyTypes behavior.

Function behavior:

createXcEpochMinimumSourceFromReadonlyEthereumPublicClient(input):

- creates EthereumReadProvider from input.publicClient
- passes provider and source config into createXcEpochMinimumSourceFromEthereumLensProvider()
- returns XcEpochMinimumSource

createEthereumReadProviderFromReadonlyEthereumPublicClient(publicClient):

- delegates to createEthereumReadProviderFromViemPublicClient(publicClient)

Tests covered:

1. creates EthereumReadProvider from provided public client
2. constructs source from provided public client without real RPC
3. preserves finalized finality policy
4. preserves safe finality policy
5. preserves confirmed finality policy
6. passes explicit function name and ABI through
7. uses provider adapter defaults when optional metadata is omitted
8. does not downgrade finalized to latest
9. does not downgrade safe to latest
10. propagates sanitized provider errors without adding RPC URL or API key
11. does not expose RPC URL or API key in successful source state

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

- 35 test files passed
- 261 tests passed

Conclusion:

The mocked read-only RPC integration helper now proves the intended outer integration boundary using a provided public client.

It keeps RPC URL / API key / env ownership outside model, wrapper, and helper code while integrating with the existing Ethereum provider adapter path.

Recommended next milestone:

xc-epoch-minimum-mocked-readonly-rpc-integration-review

Suggested next scope:

- review mocked read-only RPC integration helper boundary
- verify no real RPC / env / secrets / RPC URL factory
- verify no private keys / signers / wallet client
- verify model layer remains provider-library agnostic
- verify helper only accepts a provided public client
- decide whether extra sanitized-error tests are needed before real RPC integration planning


## Latest XC epoch minimum mocked read-only RPC integration review checkpoint

The XC epoch minimum mocked read-only RPC integration review milestone was completed on the xc-epoch-minimum-mocked-readonly-rpc-integration-review branch.

Commits:

- b0525a8 Add mocked read-only RPC integration review notes

This was a review-only milestone.

No runtime behavior changed.

Review note added:

- implementation/xc-epoch-minimum-mocked-readonly-rpc-integration-review-notes.md

Reviewed runtime:

- src/ethereum/ethereum-readonly-rpc-integration.ts

Reviewed tests:

- tests/ethereum-readonly-rpc-integration.test.ts

Reviewed prior commits:

- cd29e08 Add mocked read-only RPC integration helper
- f83b1ac Add mocked read-only RPC integration notes
- b3aed05 Update checkpoint after mocked read-only RPC integration
- eb2b661 Merge branch 'xc-epoch-minimum-mocked-readonly-rpc-integration'

Review conclusion:

The mocked read-only RPC integration helper boundary is clean.

The helper lives in:

- src/ethereum/ethereum-readonly-rpc-integration.ts

It does not live in src/model.

The model layer remains provider-library agnostic.

The model layer does not import the helper.

The helper imports model-facing provider adapter types and the existing viem-like wrapper, which is the intended dependency direction.

Runtime / dependency boundary review:

The helper does not import or call:

- viem
- ethers
- process.env
- fetch
- http / https
- createPublicClient
- wallet APIs
- signer APIs
- transaction APIs

The helper does not accept:

- RPC URL
- API key
- authorization header
- private key
- mnemonic
- signer
- wallet client
- account
- env config

The helper accepts only a provided public client and source configuration.

A targeted grep over the runtime helper, test file, and notes file found RPC / secret / provider-library terms only in notes and test descriptions where they are documented as boundaries or non-goals.

A targeted grep over src/model found no dependency on the integration helper.

Helper behavior review:

createXcEpochMinimumSourceFromReadonlyEthereumPublicClient(input):

- receives provided public client
- creates EthereumReadProvider from input.publicClient
- passes provider and source config into createXcEpochMinimumSourceFromEthereumLensProvider()
- returns XcEpochMinimumSource

createEthereumReadProviderFromReadonlyEthereumPublicClient(publicClient):

- delegates to createEthereumReadProviderFromViemPublicClient(publicClient)

This is the correct thin orchestration layer.

It does not construct a public client.

It does not own RPC URL / API key / env configuration.

Input shape review:

The helper accepts:

- publicClient
- chainId
- lensAddress
- finalityPolicy
- lockEpochs
- optional epochMinimumFunctionName
- optional epochMinimumAbi

The optional fields are forwarded only when present.

This correctly preserves exactOptionalPropertyTypes behavior.

Finality behavior review:

The helper preserves the existing provider adapter behavior.

Covered finality policies:

- finalized
- safe
- confirmed

Review decision:

- helper does not reinterpret finality policy
- helper does not downgrade finalized to latest
- helper does not downgrade safe to latest
- confirmed behavior remains controlled by the provider adapter

Source integration review:

The helper correctly integrates:

provided public client
-> viem-like read provider wrapper
-> EthereumReadProvider
-> Ethereum Lens provider adapter
-> XcEpochMinimumSource

The helper does not bypass provider adapter validation.

Test coverage review:

The current tests cover:

1. creates EthereumReadProvider from provided public client
2. constructs source from provided public client without real RPC
3. preserves finalized finality policy
4. preserves safe finality policy
5. preserves confirmed finality policy
6. passes explicit function name and ABI through
7. uses provider adapter defaults when optional metadata is omitted
8. does not downgrade finalized to latest
9. does not downgrade safe to latest
10. propagates sanitized provider errors without adding RPC URL or API key
11. does not expose RPC URL or API key in successful source state

Additional edge-case test decision:

No additional tests are required before merging this review milestone.

Possible future tests for real RPC integration planning may include:

- explicit provider unsupported finalized / safe behavior
- provider error redaction with secret-bearing transport config
- real public client construction owned by script / outer app
- no RPC URL leakage from outer entrypoint errors
- no API key leakage from outer entrypoint errors

Those are not required in the current mocked helper layer.

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

- 35 test files passed
- 261 tests passed

Conclusion:

The mocked read-only RPC integration helper is safe to keep as the provided-public-client integration boundary.

It proves the source integration path without moving RPC URL, API key, env, public-client construction, private keys, signers, wallet clients, or transaction sending into model, wrapper, or helper code.

Recommended next milestone:

xc-epoch-minimum-real-rpc-read-planning

Suggested next scope:

- plan real read-only RPC usage
- decide if viem dependency is actually needed
- decide where the real public client construction will live
- define safe env names without printing values
- define exact redacted error handling for outer entrypoint
- define whether real RPC smoke test should exist as manual-only
- do not implement real RPC until planning / review is complete


## Latest XC epoch minimum real RPC read planning checkpoint

The XC epoch minimum real RPC read planning milestone was completed on the xc-epoch-minimum-real-rpc-read-planning branch.

Commits:

- f2916a5 Add XC epoch minimum real RPC read planning

This was a planning-only milestone.

No runtime behavior changed.

Planning document added:

- implementation/xc-epoch-minimum-real-rpc-read-planning.md

Purpose:

Plan the future real read-only RPC usage for the XC epoch minimum Ethereum provider path.

This milestone does not implement:

- real RPC
- viem dependency installation
- runtime viem imports
- env reads
- RPC URL factories
- secrets
- private keys
- signers
- wallet clients
- transaction sending

Current completed foundation:

provided public client
-> createXcEpochMinimumSourceFromReadonlyEthereumPublicClient()
-> createEthereumReadProviderFromReadonlyEthereumPublicClient()
-> createEthereumReadProviderFromViemPublicClient()
-> EthereumReadProvider
-> createXcEpochMinimumSourceFromEthereumLensProvider()
-> XcEpochMinimumSource

The mocked read-only RPC integration helper is already implemented and reviewed.

The helper accepts only a provided public client and source configuration.

The helper does not own RPC URL / API key / env configuration.

The model layer remains provider-library agnostic.

Planning goal:

Plan how real read-only RPC should be introduced later without breaking the current boundaries.

The real RPC step should be treated as an outer infrastructure concern.

Core design remains:

- model layer: protocol validation and source construction
- wrapper layer: adapt public client to EthereumReadProvider
- integration helper: accept provided public client and source config
- outer app / script: construct real public client from env / config

Dependency decision:

Recommended direction:

- use viem for real public client construction later

Reason:

- existing wrapper is already viem-like
- viem public client is read-only by default when using public client only
- getChainId / getBlock / readContract map cleanly to current wrapper shape
- project already designed around a viem-style public client boundary

Planning decision:

- viem can be added later, but not in this planning milestone
- viem imports must stay outside src/model
- initial real viem construction should live in outer infrastructure code
- do not add ethers unless there is a strong reason

Public client construction location:

Public client construction must not live in:

- src/model/*
- src/ethereum/ethereum-viem-read-provider-wrapper.ts
- src/ethereum/ethereum-readonly-rpc-integration.ts
- src/ethereum/ethereum-read-provider-wrapper.ts

Recommended future location:

- scripts/read-xc-epoch-minimum-source.ts

Possible later reusable infrastructure module:

- src/integration/ethereum-public-client-factory.ts

Preferred first implementation:

- script-only construction first
- no reusable RPC URL factory exported from package
- keep secret-bearing config at the outermost edge

Safe env names:

Potential future env names:

- XC_ETHEREUM_RPC_URL
- XC_ETHEREUM_CHAIN_ID
- XC_ETHEREUM_LENS_ADDRESS
- XC_ETHEREUM_FINALITY
- XC_ETHEREUM_CONFIRMATIONS
- XC_ETHEREUM_LOCK_EPOCHS

Optional if needed later:

- XC_ETHEREUM_EPOCH_MINIMUM_FUNCTION
- XC_ETHEREUM_EPOCH_MINIMUM_ABI_PATH

Sensitive / never print values:

- XC_ETHEREUM_RPC_URL
- any API-key-bearing RPC URL
- any authorization header
- any token-like value

Safe to print after validation:

- XC_ETHEREUM_CHAIN_ID
- XC_ETHEREUM_LENS_ADDRESS
- XC_ETHEREUM_FINALITY
- XC_ETHEREUM_CONFIRMATIONS
- parsed lock epoch count
- selected block number
- selected block hash
- function name

Env handling policy:

Env reads may happen only in the outer script / app entrypoint.

Env reads must not happen in:

- src/model
- src/ethereum wrappers
- src/ethereum integration helper
- reusable source builders

Future script may read env, validate it, construct a public client, and pass the public client inward.

Do not print raw env values.

Do not print RPC URL.

Do not print full config object.

Redacted error policy:

Outer entrypoint errors must not include:

- RPC URL
- API key
- authorization header
- full env dump
- transport config
- private key
- mnemonic
- signer object
- wallet client internals

Allowed error context:

- operation name
- chain ID
- Lens address
- finality policy
- confirmations count
- block tag
- block number
- function name
- lock epoch count
- high-level provider failure category

Recommended pattern:

- catch provider/client construction errors at script boundary
- rethrow or print sanitized message
- never stringify full client / transport / env objects
- never include original error message if provider may include URL or headers
- optionally include original error name only

Provider unsupported finalized / safe behavior:

Do not silently downgrade:

- finalized -> latest
- safe -> latest

If a provider does not support finalized or safe:

- fail with sanitized error
- advise changing explicit finality config
- do not automatically change provenance policy

If fallback is ever allowed, it must be explicit:

- finalityPolicy: { kind: "confirmed", confirmations: N }

Manual-only real RPC smoke test decision:

Recommended first real RPC smoke test policy:

- manual-only
- not part of npm test
- not part of CI
- requires explicit env confirmation
- must not print RPC URL
- must not print secrets
- should print only safe summary fields

Possible future command:

- npm run smoke:xc-epoch-minimum:ethereum

Required confirmation env:

- XC_ETHEREUM_REAL_RPC_CONFIRM=I_UNDERSTAND_THIS_USES_REAL_RPC

The smoke test should refuse to run without explicit confirmation.

The smoke test should refuse if required safe config is missing.

The smoke test should not send transactions.

The smoke test should only perform:

- getChainId
- getBlock
- readContract

CLI / script output policy:

Allowed output:

- chain ID
- Lens address
- finality policy
- selected block number
- selected block hash
- lock epochs count
- function name
- number of epoch minimums loaded

Disallowed output:

- RPC URL
- API key
- env dump
- authorization header
- full provider transport config
- private key
- mnemonic
- signer / wallet internals

Future implementation order:

1. Review this planning document.
2. Add script-only public client construction design.
3. Review script-only public client construction design.
4. Implement mocked script/config parsing without real RPC.
5. Review mocked script/config parsing.
6. Implement manual-only real RPC smoke script.
7. Review manual-only real RPC smoke script.
8. Only then consider production operational docs.

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities

Current test count:

- 35 test files passed
- 261 tests passed

Conclusion:

Real read-only RPC should be introduced only at the outer infrastructure edge.

The safe next direction is script-only planning and design for public client construction, with RPC URL / API key ownership kept outside model, wrapper, and helper code.

No real RPC should be implemented until this planning step is reviewed.

Recommended next milestone:

xc-epoch-minimum-real-rpc-read-planning-review

Suggested next scope:

- review real RPC read planning
- confirm no real RPC / viem install / env reads were added
- confirm env and RPC URL policy
- confirm redacted error policy
- confirm manual-only smoke test direction
- do not implement real RPC until planning review is complete


## Latest XC epoch minimum real RPC read planning review checkpoint

The XC epoch minimum real RPC read planning review milestone was completed on the xc-epoch-minimum-real-rpc-read-planning-review branch.

Commits:

- ba2aa58 Add real RPC read planning review notes

This was a review-only milestone.

No runtime behavior changed.

Review note added:

- implementation/xc-epoch-minimum-real-rpc-read-planning-review-notes.md

Reviewed planning document:

- implementation/xc-epoch-minimum-real-rpc-read-planning.md

Reviewed prior commits:

- f2916a5 Add XC epoch minimum real RPC read planning
- 749fc9b Update checkpoint after real RPC read planning
- c3e43ef Merge branch 'xc-epoch-minimum-real-rpc-read-planning'

Review conclusion:

The real RPC read planning boundary is clean.

This remains a planning-only stage.

No real RPC implementation was added.

No viem dependency was installed.

No runtime viem imports were added.

No env reads were added.

No RPC URL factory was added.

No private keys, signers, wallet clients, or transaction-sending paths were added.

Runtime / dependency review:

A targeted grep confirmed that RPC / env / secret / viem terms appear only in the planning document as policy, planning, examples, and non-goals.

The runtime source remains free from:

- real viem imports
- ethers imports
- process.env reads
- createPublicClient calls
- http transport construction
- RPC URL factories
- signer / wallet / write paths

Current foundation review:

The completed provider path remains:

provided public client
-> createXcEpochMinimumSourceFromReadonlyEthereumPublicClient()
-> createEthereumReadProviderFromReadonlyEthereumPublicClient()
-> createEthereumReadProviderFromViemPublicClient()
-> EthereumReadProvider
-> createXcEpochMinimumSourceFromEthereumLensProvider()
-> XcEpochMinimumSource

The mocked read-only RPC integration helper remains the safe boundary.

The helper accepts only a provided public client and source configuration.

The helper does not own RPC URL / API key / env configuration.

The model layer remains provider-library agnostic.

Dependency planning review:

The planning document recommends viem for future real public client construction.

Review decision:

- viem is the preferred future dependency if real RPC is added
- viem should not be installed in this planning review
- viem imports must stay outside src/model
- initial real viem construction should live in outer infrastructure code
- ethers should not be added unless a strong reason appears

Public client construction location review:

Public client construction must not live in:

- src/model/*
- src/ethereum/ethereum-viem-read-provider-wrapper.ts
- src/ethereum/ethereum-readonly-rpc-integration.ts
- src/ethereum/ethereum-read-provider-wrapper.ts

Preferred first location remains:

- scripts/read-xc-epoch-minimum-source.ts

Possible later reusable infrastructure module:

- src/integration/ethereum-public-client-factory.ts

Review decision:

- script-only construction first
- no reusable RPC URL factory exported from package
- keep secret-bearing config at the outermost edge

Env policy review:

Potential future env names are acceptable:

- XC_ETHEREUM_RPC_URL
- XC_ETHEREUM_CHAIN_ID
- XC_ETHEREUM_LENS_ADDRESS
- XC_ETHEREUM_FINALITY
- XC_ETHEREUM_CONFIRMATIONS
- XC_ETHEREUM_LOCK_EPOCHS
- XC_ETHEREUM_EPOCH_MINIMUM_FUNCTION
- XC_ETHEREUM_EPOCH_MINIMUM_ABI_PATH

Sensitive values must never be printed:

- XC_ETHEREUM_RPC_URL
- API-key-bearing RPC URL
- authorization header
- token-like value

Safe values may be printed after validation:

- chain ID
- Lens address
- finality policy
- confirmations
- parsed lock epoch count
- selected block number
- selected block hash
- function name

Review decision:

- env reads may happen only in the outer script / app entrypoint
- env reads must not happen in model, wrappers, integration helper, or reusable source builders
- do not print raw env values
- do not print RPC URL
- do not print full config object

Redacted error policy review:

Outer entrypoint errors must not include:

- RPC URL
- API key
- authorization header
- full env dump
- transport config
- private key
- mnemonic
- signer object
- wallet client internals

Allowed error context:

- operation name
- chain ID
- Lens address
- finality policy
- confirmations count
- block tag
- block number
- function name
- lock epoch count
- high-level provider failure category

Review decision:

- catch provider / client construction errors at script boundary
- print or rethrow sanitized messages only
- never stringify full client / transport / env objects
- do not include original provider error messages if they may contain URL or headers
- optionally include original error name only

Finality and provider support review:

Do not silently downgrade:

- finalized -> latest
- safe -> latest

If a provider does not support finalized or safe:

- fail with sanitized error
- advise changing explicit finality config
- do not automatically change provenance policy

Fallback can only be explicit:

- finalityPolicy: { kind: "confirmed", confirmations: N }

Manual-only smoke test review:

The planned real RPC smoke test should be manual-only.

It should not be part of:

- npm test
- CI

It should require explicit confirmation:

- XC_ETHEREUM_REAL_RPC_CONFIRM=I_UNDERSTAND_THIS_USES_REAL_RPC

It must not print:

- RPC URL
- secrets
- raw env values

It should only perform read operations:

- getChainId
- getBlock
- readContract

Review decision:

- manual-only smoke direction is acceptable
- do not implement smoke test in this planning review
- design script-only construction first before smoke implementation

Output policy review:

Allowed future script output:

- chain ID
- Lens address
- finality policy
- selected block number
- selected block hash
- lock epochs count
- function name
- number of epoch minimums loaded

Disallowed output:

- RPC URL
- API key
- env dump
- authorization header
- full provider transport config
- private key
- mnemonic
- signer / wallet internals

Future implementation order review:

1. Review this planning document.
2. Add script-only public client construction design.
3. Review script-only public client construction design.
4. Implement mocked script/config parsing without real RPC.
5. Review mocked script/config parsing.
6. Implement manual-only real RPC smoke script.
7. Review manual-only real RPC smoke script.
8. Only then consider production operational docs.

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities

Current test count:

- 35 test files passed
- 261 tests passed

Conclusion:

The real RPC read planning is ready to proceed to script-only public client construction design.

The next milestone should still be design-only and should not implement real RPC.

Recommended next milestone:

xc-epoch-minimum-script-public-client-construction-design

Suggested next scope:

- design script-only public client construction
- decide exact script path
- decide exact safe env parsing rules
- decide if viem dependency is introduced in the script-design stage or later
- define sanitized error boundaries for the script
- do not implement real RPC until script construction design review is complete


## Latest XC epoch minimum script public client construction design checkpoint

The XC epoch minimum script public client construction design milestone was completed on the xc-epoch-minimum-script-public-client-construction-design branch.

Commits:

- 43fcd40 Add script public client construction design

This was a design-only milestone.

No runtime behavior changed.

Design document added:

- implementation/xc-epoch-minimum-script-public-client-construction-design.md

Purpose:

Design the script-only public client construction boundary for the XC epoch minimum Ethereum provider path.

This milestone does not implement:

- real RPC
- viem dependency installation
- runtime viem imports
- env reads
- RPC URL factory
- private keys
- signers
- wallet clients
- transaction sending

Current completed foundation:

provided public client
-> createXcEpochMinimumSourceFromReadonlyEthereumPublicClient()
-> createEthereumReadProviderFromReadonlyEthereumPublicClient()
-> createEthereumReadProviderFromViemPublicClient()
-> EthereumReadProvider
-> createXcEpochMinimumSourceFromEthereumLensProvider()
-> XcEpochMinimumSource

Design goal:

A future script may construct a real read-only public client while keeping secret-bearing configuration at the outermost edge.

The script should be the only layer allowed to read env.

The model layer, wrappers, and integration helper must remain free from:

- env reads
- RPC URL ownership
- public client construction
- private keys
- signers
- wallet clients
- transaction sending

Exact future script path:

- scripts/read-xc-epoch-minimum-source.ts

Reason:

- script-only construction keeps RPC URL / API key ownership outside exported library APIs
- script-only construction is easier to gate with explicit confirmation
- script-only construction avoids creating a reusable RPC URL factory too early
- script-only construction keeps src/model and src/ethereum helper layers clean

Dependency decision:

Recommended future dependency:

- viem

Design-stage decision:

- do not install viem in this design milestone
- do not import viem in this design milestone
- decide exact viem version only in implementation or implementation-design milestone
- keep viem imports out of src/model permanently

Future script responsibility:

A future script may be responsible for:

- reading safe env config
- validating required fields
- constructing a read-only public client
- creating XcEpochMinimumSource through the existing helper
- printing safe summary output
- refusing unsafe or incomplete configuration

A future script must not:

- print RPC URL
- print API key
- print raw env object
- print full config object
- accept private key
- accept mnemonic
- construct signer
- construct wallet client
- send transaction
- call writeContract
- call sendTransaction
- run as part of npm test or CI if it uses real RPC

Future env names:

Required future env names:

- XC_ETHEREUM_RPC_URL
- XC_ETHEREUM_CHAIN_ID
- XC_ETHEREUM_LENS_ADDRESS
- XC_ETHEREUM_FINALITY
- XC_ETHEREUM_LOCK_EPOCHS

Required only for confirmed finality:

- XC_ETHEREUM_CONFIRMATIONS

Optional future env names:

- XC_ETHEREUM_EPOCH_MINIMUM_FUNCTION
- XC_ETHEREUM_EPOCH_MINIMUM_ABI_PATH

Manual real RPC confirmation:

- XC_ETHEREUM_REAL_RPC_CONFIRM=I_UNDERSTAND_THIS_USES_REAL_RPC

Env parsing rules:

- XC_ETHEREUM_RPC_URL is required for real RPC script, but must never be printed, included in thrown errors, or stored in snapshots.
- XC_ETHEREUM_CHAIN_ID must use eip155-N format and is safe to print after validation.
- XC_ETHEREUM_LENS_ADDRESS must use 0x + 40 hex chars and is safe to print after validation.
- XC_ETHEREUM_FINALITY must be finalized, safe, or confirmed.
- XC_ETHEREUM_CONFIRMATIONS is required only when finality is confirmed.
- XC_ETHEREUM_LOCK_EPOCHS is a comma-separated positive integer list.
- XC_ETHEREUM_EPOCH_MINIMUM_FUNCTION is optional and defaults to epochMinimum.
- XC_ETHEREUM_EPOCH_MINIMUM_ABI_PATH is optional; preferred first implementation may avoid it and use minimal ABI design later.
- XC_ETHEREUM_REAL_RPC_CONFIRM must equal I_UNDERSTAND_THIS_USES_REAL_RPC.

Config object boundary:

Future script may build an internal config object, but that config object must not be printed directly.

Secret-bearing config includes:

- rpcUrl
- transport options
- authorization headers
- API-key-bearing URLs

Secret-bearing config must stay inside script-local construction scope.

It must not be passed into:

- src/model
- src/ethereum/ethereum-readonly-rpc-integration.ts
- createXcEpochMinimumSourceFromReadonlyEthereumPublicClient()
- createEthereumReadProviderFromReadonlyEthereumPublicClient()

Only the constructed public client object should be passed inward.

Error handling design:

Future script errors should be sanitized.

Allowed error context:

- operation name
- chain ID
- Lens address
- finality policy
- confirmations count
- lock epoch count
- block tag
- block number
- function name
- high-level failure category

Disallowed error context:

- RPC URL
- API key
- authorization header
- raw env object
- full config object
- transport config
- private key
- mnemonic
- signer object
- wallet client internals

Recommended future error helpers:

- sanitizeUnknownError(error)
- failWithSanitizedMessage(message)
- assertNoSecretLikeText(message)

Provider unsupported finalized / safe policy:

A future script must not silently downgrade:

- finalized -> latest
- safe -> latest

If provider does not support finalized or safe:

- fail with sanitized message
- recommend using explicit confirmed finality configuration if appropriate
- do not automatically change policy

Future script output:

Allowed output:

- real RPC confirmation accepted
- chain ID
- Lens address
- finality policy
- confirmations count if applicable
- lock epoch count
- function name
- selected block number
- selected block hash
- number of loaded epoch minimums

Disallowed output:

- RPC URL
- API key
- raw env values
- full config object
- transport config
- authorization headers
- private key
- mnemonic
- signer / wallet internals

Package script design:

Possible future package script:

- smoke:xc-epoch-minimum:ethereum

Design decision:

- do not add package script in this design milestone
- add it only when manual-only smoke implementation exists
- do not make it part of test, build, or CI

Real RPC smoke policy:

Future real RPC script should be manual-only.

It should require:

- npm run build
- XC_ETHEREUM_REAL_RPC_CONFIRM=I_UNDERSTAND_THIS_USES_REAL_RPC

It should not run during:

- npm test
- npm run build
- CI

It should only perform:

- getChainId
- getBlock
- readContract

It must not perform:

- sendTransaction
- writeContract
- approve
- signer calls
- wallet calls

Testing strategy for future implementation:

Before any real RPC script is implemented, add mocked tests for env/config parsing and error sanitization.

Recommended mocked tests:

1. parses required env into safe config without printing RPC URL
2. rejects missing RPC URL with sanitized message
3. rejects invalid chain ID
4. rejects invalid Lens address
5. rejects invalid finality
6. requires confirmations for confirmed finality
7. rejects non-positive confirmations
8. parses lock epoch list
9. rejects empty lock epoch list
10. requires explicit real RPC confirmation
11. never includes RPC URL in error message
12. never includes API-key-looking text in error message
13. does not construct signer / wallet client
14. does not expose raw env object
15. passes only constructed public client into existing helper

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities

Current test count:

- 35 test files passed
- 261 tests passed

Conclusion:

Script-only public client construction should keep real RPC at the outermost edge.

The future script may own env reading and public client construction, but model, wrappers, and integration helper must remain free from RPC URL ownership, env reads, provider construction, private keys, signers, wallet clients, and transaction sending.

Recommended next milestone:

xc-epoch-minimum-script-public-client-construction-design-review

Suggested next scope:

- review script-only public client construction design
- confirm no real RPC / viem install / env reads were added
- confirm script path and env parsing policy
- confirm sanitized error boundary
- confirm manual-only smoke policy
- do not implement real RPC until design review is complete


## Latest XC epoch minimum script public client construction design review checkpoint

The XC epoch minimum script public client construction design review milestone was completed on the xc-epoch-minimum-script-public-client-construction-design-review branch.

Commits:

- 1da16df Add script public client construction design review notes

This was a review-only milestone.

No runtime behavior changed.

Review note added:

- implementation/xc-epoch-minimum-script-public-client-construction-design-review-notes.md

Reviewed design:

- implementation/xc-epoch-minimum-script-public-client-construction-design.md

Reviewed prior commits:

- 43fcd40 Add script public client construction design
- 58de7e0 Update checkpoint after script public client construction design
- 6a47780 Merge branch 'xc-epoch-minimum-script-public-client-construction-design'

Review conclusion:

The script-only public client construction design boundary is clean.

This remains a design-only stage.

No real RPC implementation was added.

No viem dependency was installed.

No runtime viem imports were added.

No env reads were added.

No RPC URL factory was added.

No private keys, signers, wallet clients, or transaction-sending paths were added.

Runtime / dependency review:

A targeted grep confirmed that RPC / env / secret / viem terms appear only in the design document as policy, design rules, examples, and non-goals.

The runtime source remains free from:

- real viem imports
- ethers imports
- process.env reads
- createPublicClient calls
- http transport construction
- RPC URL factories
- signer / wallet / write paths

Design boundary review:

The future script path is acceptable:

- scripts/read-xc-epoch-minimum-source.ts

Review decision:

- script-only construction keeps RPC URL / API key ownership outside exported library APIs
- script-only construction is easier to gate with explicit confirmation
- script-only construction avoids creating a reusable RPC URL factory too early
- script-only construction keeps src/model and src/ethereum helper layers clean

Dependency decision review:

The design recommends viem as the future dependency for real public client construction.

Review decision:

- viem is the preferred future dependency if real RPC is added
- viem should not be installed in this design review
- viem imports must stay outside src/model
- exact viem version should be decided later
- ethers should not be added unless a strong reason appears

Env parsing policy review:

Future required env names are acceptable:

- XC_ETHEREUM_RPC_URL
- XC_ETHEREUM_CHAIN_ID
- XC_ETHEREUM_LENS_ADDRESS
- XC_ETHEREUM_FINALITY
- XC_ETHEREUM_LOCK_EPOCHS

Confirmed finality env:

- XC_ETHEREUM_CONFIRMATIONS

Optional env names:

- XC_ETHEREUM_EPOCH_MINIMUM_FUNCTION
- XC_ETHEREUM_EPOCH_MINIMUM_ABI_PATH

Manual real RPC confirmation:

- XC_ETHEREUM_REAL_RPC_CONFIRM=I_UNDERSTAND_THIS_USES_REAL_RPC

Review decision:

- env reads may happen only in the future script entrypoint
- env reads must not happen in model, wrappers, or integration helper
- RPC URL must never be printed
- full config object must never be printed
- raw env object must never be printed
- secret-bearing config must stay inside script-local construction scope

Config boundary review:

Secret-bearing config includes:

- rpcUrl
- transport options
- authorization headers
- API-key-bearing URLs

Secret-bearing config must not be passed into:

- src/model
- src/ethereum/ethereum-readonly-rpc-integration.ts
- createXcEpochMinimumSourceFromReadonlyEthereumPublicClient()
- createEthereumReadProviderFromReadonlyEthereumPublicClient()

Only the constructed public client object should be passed inward.

Error handling review:

Allowed future error context:

- operation name
- chain ID
- Lens address
- finality policy
- confirmations count
- lock epoch count
- block tag
- block number
- function name
- high-level failure category

Disallowed future error context:

- RPC URL
- API key
- authorization header
- raw env object
- full config object
- transport config
- private key
- mnemonic
- signer object
- wallet client internals

Review decision:

- sanitized error boundary is acceptable
- future implementation should add mocked tests for RPC URL / API key leakage
- do not include original provider error messages if they may contain URL or headers

Finality policy review:

A future script must not silently downgrade:

- finalized -> latest
- safe -> latest

If provider does not support finalized or safe:

- fail with sanitized message
- recommend explicit confirmed finality configuration if appropriate
- do not automatically change policy

Output policy review:

Allowed future script output:

- real RPC confirmation accepted
- chain ID
- Lens address
- finality policy
- confirmations count if applicable
- lock epoch count
- function name
- selected block number
- selected block hash
- number of loaded epoch minimums

Disallowed future script output:

- RPC URL
- API key
- raw env values
- full config object
- transport config
- authorization headers
- private key
- mnemonic
- signer / wallet internals

Manual-only smoke policy review:

The future real RPC script should be manual-only.

It should require:

- npm run build
- XC_ETHEREUM_REAL_RPC_CONFIRM=I_UNDERSTAND_THIS_USES_REAL_RPC

It should not run during:

- npm test
- npm run build
- CI

It should only perform:

- getChainId
- getBlock
- readContract

It must not perform:

- sendTransaction
- writeContract
- approve
- signer calls
- wallet calls

Testing strategy review:

The recommended mocked tests are appropriate for the next implementation phase:

1. parse required env into safe config without printing RPC URL
2. reject missing RPC URL with sanitized message
3. reject invalid chain ID
4. reject invalid Lens address
5. reject invalid finality
6. require confirmations for confirmed finality
7. reject non-positive confirmations
8. parse lock epoch list
9. reject empty lock epoch list
10. require explicit real RPC confirmation
11. never include RPC URL in error message
12. never include API-key-looking text in error message
13. do not construct signer / wallet client
14. do not expose raw env object
15. pass only constructed public client into existing helper

Additional design change decision:

No additional design changes are required before merging this review milestone.

Validation:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities

Current test count:

- 35 test files passed
- 261 tests passed

Conclusion:

The script-only public client construction design is ready to proceed to mocked script config parsing.

The next implementation should still avoid real RPC, viem dependency, public client construction, and env reads from real process.env.

Recommended next milestone:

xc-epoch-minimum-mocked-script-config-parsing

Suggested next scope:

- add mocked config parsing helpers
- no real RPC
- no viem dependency
- no public client construction
- no env reads outside test-provided objects
- no secrets printed
- no RPC URL leakage in errors
- tests only


## Latest XC epoch minimum mocked script config parsing checkpoint

The XC epoch minimum mocked script config parsing milestone was completed on the xc-epoch-minimum-mocked-script-config-parsing branch.

Commits:

- 3d4afa3 Add mocked Ethereum script config parsing
- b0d1aea Add mocked Ethereum script config parsing notes

This milestone implements mocked Ethereum script config parsing for the future XC epoch minimum real RPC script path.

Runtime additions:

- src/ethereum/ethereum-script-config.ts

Test additions:

- tests/ethereum-script-config.test.ts

Documentation additions:

- implementation/xc-epoch-minimum-mocked-script-config-parsing-notes.md

Export added through src/index.ts:

- EthereumScriptConfigEnv
- EthereumScriptConfig
- EthereumScriptSafeConfigSummary
- parseEthereumScriptConfig()
- summarizeEthereumScriptConfig()

Purpose:

The previous script-only public client construction design review concluded that the next safe implementation should add mocked config parsing helpers.

This milestone adds a parser that accepts a test-provided env-like object:

- Record<string, string | undefined>

It does not read the real process.env object.

Boundary:

The parser lives outside src/model.

The model layer remains provider-library agnostic.

The parser does not import:

- viem
- ethers
- http
- createPublicClient
- wallet clients
- signer APIs
- transaction APIs

The parser does not call:

- process.env
- fetch
- sendTransaction
- writeContract

The parser does not construct:

- public client
- wallet client
- signer
- transaction sender

Parsed config:

The parser accepts:

- XC_ETHEREUM_RPC_URL
- XC_ETHEREUM_CHAIN_ID
- XC_ETHEREUM_LENS_ADDRESS
- XC_ETHEREUM_FINALITY
- XC_ETHEREUM_CONFIRMATIONS
- XC_ETHEREUM_LOCK_EPOCHS
- XC_ETHEREUM_EPOCH_MINIMUM_FUNCTION
- XC_ETHEREUM_EPOCH_MINIMUM_ABI_PATH
- XC_ETHEREUM_REAL_RPC_CONFIRM

The returned full config includes rpcUrl because a future outer script will need it to construct a read-only public client.

The safe summary intentionally excludes rpcUrl.

Validation behavior:

The parser validates:

- required RPC URL presence
- chain ID format eip155-N
- Lens address format 0x + 40 hex chars
- finality finalized / safe / confirmed
- confirmations required for confirmed finality
- confirmations positive integer
- lock epoch list non-empty
- lock epoch values numeric
- optional function name identifier shape
- explicit real RPC confirmation

The parser normalizes:

- Lens address to lowercase
- empty optional function name to epochMinimum
- empty optional ABI path to omitted optional property

The parser preserves exactOptionalPropertyTypes behavior by omitting optional fields when absent.

Safe summary:

summarizeEthereumScriptConfig() returns:

- chainId
- lensAddress
- finalityPolicy
- lockEpochCount
- epochMinimumFunctionName
- hasEpochMinimumAbiPath
- realRpcConfirmed

It does not return:

- rpcUrl
- API key
- raw env object
- full config object
- transport config

Tests covered:

1. parses required env into config
2. normalizes Lens address to lowercase
3. parses safe finality
4. parses confirmed finality with confirmations
5. parses optional function name and ABI path
6. creates safe summary without RPC URL
7. rejects missing RPC URL with sanitized error
8. rejects invalid chain ID
9. rejects invalid Lens address
10. rejects invalid finality
11. requires confirmations for confirmed finality
12. rejects non-positive confirmations
13. rejects empty lock epoch list
14. rejects invalid lock epoch item
15. requires explicit real RPC confirmation
16. rejects invalid function name
17. does not include RPC URL or API key in validation errors

Security / operational boundary:

This milestone intentionally does not add:

- real Ethereum RPC
- viem dependency
- viem runtime imports
- ethers dependency
- process.env reads
- public client construction
- RPC URL factory
- private keys
- API keys as separate fields
- mnemonic
- signer support
- wallet client support
- account support
- transaction sending
- CLI commands
- package scripts
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

- 36 test files passed
- 278 tests passed

Conclusion:

The mocked script config parsing layer is safe to keep.

It prepares a future manual-only real RPC script while keeping env ownership mocked/test-provided, keeping real RPC out of the runtime path, and ensuring safe summaries and validation errors do not leak RPC URL or API-key-like values.

Recommended next milestone:

xc-epoch-minimum-mocked-script-config-parsing-review

Suggested next scope:

- review mocked script config parsing boundary
- verify no real RPC / viem / process.env reads / public client construction
- verify safe summary excludes rpcUrl
- verify validation errors do not leak RPC URL / API-key-like values
- decide whether additional parser hardening tests are needed before script implementation planning


## Latest XC epoch minimum mocked script config parsing review checkpoint

The XC epoch minimum mocked script config parsing review milestone was completed on the xc-epoch-minimum-mocked-script-config-parsing-review branch.

Commits:

- 84d8faf Add mocked Ethereum script config parsing review notes

This was a review-only milestone.

No runtime behavior changed.

Review note added:

- implementation/xc-epoch-minimum-mocked-script-config-parsing-review-notes.md

Reviewed runtime:

- src/ethereum/ethereum-script-config.ts

Reviewed tests:

- tests/ethereum-script-config.test.ts

Reviewed prior commits:

- 3d4afa3 Add mocked Ethereum script config parsing
- b0d1aea Add mocked Ethereum script config parsing notes
- 65eeac6 Update checkpoint after mocked script config parsing
- c83036a Merge branch 'xc-epoch-minimum-mocked-script-config-parsing'

Review conclusion:

The mocked script config parsing boundary is clean.

The parser accepts a test-provided env-like object.

The parser does not read the real process.env object.

No real RPC implementation was added.

No viem dependency was installed.

No runtime viem imports were added.

No ethers imports were added.

No public client construction was added.

No RPC URL factory was added.

No private keys, signers, wallet clients, or transaction-sending paths were added.

Runtime / dependency boundary review:

The parser lives in:

- src/ethereum/ethereum-script-config.ts

It does not live in src/model.

The model layer remains provider-library agnostic.

The parser does not import or call:

- viem
- ethers
- http
- createPublicClient
- process.env
- fetch
- sendTransaction
- writeContract
- wallet APIs
- signer APIs
- transaction APIs

The parser does not construct:

- public client
- wallet client
- signer
- transaction sender

Parser input review:

The parser accepts:

- EthereumScriptConfigEnv

This is an env-like object:

- Record<string, string | undefined>

Review decision:

- this is acceptable for the mocked layer
- it allows tests to provide config without reading real process.env
- it keeps future env ownership at the outer script boundary

Parsed config review:

The parser accepts the intended future env names:

- XC_ETHEREUM_RPC_URL
- XC_ETHEREUM_CHAIN_ID
- XC_ETHEREUM_LENS_ADDRESS
- XC_ETHEREUM_FINALITY
- XC_ETHEREUM_CONFIRMATIONS
- XC_ETHEREUM_LOCK_EPOCHS
- XC_ETHEREUM_EPOCH_MINIMUM_FUNCTION
- XC_ETHEREUM_EPOCH_MINIMUM_ABI_PATH
- XC_ETHEREUM_REAL_RPC_CONFIRM

The full config includes rpcUrl because a future script will need it to construct a read-only public client.

Review decision:

- keeping rpcUrl in the full parsed config is acceptable
- rpcUrl must stay out of safe summary
- rpcUrl must not appear in validation errors
- no public client construction is performed here

Validation review:

The parser validates:

- required RPC URL presence
- chain ID format eip155-N
- Lens address format 0x + 40 hex chars
- finality finalized / safe / confirmed
- confirmations required for confirmed finality
- confirmations positive integer
- lock epoch list non-empty
- lock epoch values numeric
- optional function name identifier shape
- explicit real RPC confirmation

The parser normalizes:

- Lens address to lowercase
- empty optional function name to epochMinimum
- empty optional ABI path to omitted optional property

The parser preserves exactOptionalPropertyTypes behavior by omitting optional fields when absent.

Safe summary review:

summarizeEthereumScriptConfig() returns:

- chainId
- lensAddress
- finalityPolicy
- lockEpochCount
- epochMinimumFunctionName
- hasEpochMinimumAbiPath
- realRpcConfirmed

It does not return:

- rpcUrl
- API key
- raw env object
- full config object
- transport config

Review decision:

- safe summary boundary is correct
- safe summary is suitable for future script output
- do not print full parsed config in future scripts

Error leakage review:

Tests verify that validation errors do not include:

- full RPC URL
- API-key-like value
- provider hostname
- https:// prefix

Review decision:

- current validation errors are sanitized enough for this mocked parser layer
- future script implementation should continue testing for RPC URL / API key leakage
- future provider errors still need a separate sanitized error boundary before real RPC

Test coverage review:

The current tests cover:

1. parses required env into config
2. normalizes Lens address to lowercase
3. parses safe finality
4. parses confirmed finality with confirmations
5. parses optional function name and ABI path
6. creates safe summary without RPC URL
7. rejects missing RPC URL with sanitized error
8. rejects invalid chain ID
9. rejects invalid Lens address
10. rejects invalid finality
11. requires confirmations for confirmed finality
12. rejects non-positive confirmations
13. rejects empty lock epoch list
14. rejects invalid lock epoch item
15. requires explicit real RPC confirmation
16. rejects invalid function name
17. does not include RPC URL or API key in validation errors

Additional hardening decision:

No additional parser hardening tests are required before merging this review milestone.

Possible future hardening tests may include:

- duplicated lock epoch values policy
- very large lock epoch values policy
- whitespace normalization around all env values
- ABI path traversal policy if ABI path support becomes active
- explicit checks that safe summary is the only object printed by future scripts

Those are not required before this review milestone is merged.

Security / operational review:

This milestone does not add:

- real Ethereum RPC
- viem dependency
- viem runtime imports
- ethers dependency
- process.env reads
- public client construction
- RPC URL factory
- private keys
- API keys as separate fields
- mnemonic
- signer support
- wallet client support
- account support
- transaction sending
- CLI commands
- package scripts
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

- 36 test files passed
- 278 tests passed

Conclusion:

The mocked script config parsing layer is safe to keep.

It prepares future script implementation while keeping config input mocked/test-provided, avoiding real RPC, avoiding viem, avoiding public client construction, avoiding process.env reads, and preventing RPC URL / API-key-like leakage in safe summaries and validation errors.

Recommended next milestone:

xc-epoch-minimum-script-implementation-planning

Suggested next scope:

- plan the next script implementation step separately
- decide whether to design mocked script entrypoint first or add more parser hardening
- keep real RPC disabled until manual-only smoke script design and review

## Current next steps

Potential next documents / design areas:

1. Plan the next script implementation step separately.
2. Continue implementation only with clean typecheck and tests.





















## Latest XC epoch minimum script implementation planning checkpoint

The XC epoch minimum script implementation planning milestone was completed on the `xc-epoch-minimum-script-implementation-planning` branch.

This milestone is planning-only.

New document:

- `implementation/xc-epoch-minimum-script-implementation-planning.md`

Purpose:

- define the next safe implementation step for the Ethereum XC epoch minimum source path
- keep real RPC disabled
- keep viem dependency out of the project for now
- keep process.env reads out of model, wrapper, helper, and parser layers
- define the mocked script entrypoint as the recommended next milestone
- preserve the rule that only safe summaries may be printed

The recommended next branch is:

```text
xc-epoch-minimum-mocked-script-entrypoint-design
```

The next safe step should be mocked script entrypoint design, not real RPC implementation.

The planning document confirms that the next milestone should still not add:

- real RPC calls
- viem installation
- process.env integration
- public client construction
- RPC URL factory
- private keys
- signers
- wallet clients
- transaction sending
- writeContract / sendTransaction paths
- raw env printing
- full config printing
- RPC URL / API key printing

The proposed future mocked script entrypoint shape is:

```text
env-like input
-> parseEthereumScriptConfig()
-> summarizeEthereumScriptConfig()
-> injected mocked/provided public client
-> createXcEpochMinimumSourceFromReadonlyEthereumPublicClient()
-> read source data
-> print safe summary only
```

Validation baseline after the planning document:

- `npm run typecheck` passed
- `npm test` passed: 36 test files, 278 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

No runtime code changed.

No dependencies changed.

No real RPC was added.

## Latest XC epoch minimum mocked script entrypoint design checkpoint

The XC epoch minimum mocked script entrypoint design milestone was completed on the `xc-epoch-minimum-mocked-script-entrypoint-design` branch.

This milestone is design-only.

New document:

- `implementation/xc-epoch-minimum-mocked-script-entrypoint-design.md`

Purpose:

- design the future mocked/testable script runner
- keep the runner outside `src/model`
- define the provided-client boundary
- define safe output policy
- define dependency injection rules
- define error sanitization expectations
- define ABI path handling for the first mocked runner milestone
- keep future real RPC separated from the mocked runner
- define the expected implementation files for the next milestone

The proposed future implementation files are:

```text
src/ethereum/ethereum-script-runner.ts
tests/ethereum-script-runner.test.ts
```

The expected export update is:

```text
src/index.ts
```

The proposed future runner function is:

```text
runEthereumXcEpochMinimumReadFromProvidedClient(input)
```

The name should preserve the important boundary:

```text
FromProvidedClient
```

The mocked runner design confirms that the next implementation milestone should still not add:

- real RPC calls
- viem dependency
- runtime viem imports
- ethers dependency
- process.env reads
- public client construction
- RPC URL factory
- HTTP transport construction
- private key support
- mnemonic support
- signer support
- wallet client support
- transaction sending
- writeContract / sendTransaction paths
- raw env printing
- full config printing
- RPC URL / API key printing

The intended safe flow remains:

```text
env-like input
-> parseEthereumScriptConfig()
-> summarizeEthereumScriptConfig()
-> injected mocked/provided public client
-> createXcEpochMinimumSourceFromReadonlyEthereumPublicClient()
-> read source data
-> write safe output only
```

The design explicitly states that the parser may continue to keep `rpcUrl` in the full parsed config for a future outer real script, but the mocked runner must not print it, pass it into helpers, or construct transport from it.

ABI path policy for the first mocked runner implementation:

- parse and summarize whether ABI path is present
- do not load files
- use default epoch minimum ABI
- leave ABI loading for a separate milestone if needed

Validation baseline after the design document:

- `npm run typecheck` passed
- `npm test` passed: 36 test files, 278 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

No runtime code changed.

No dependencies changed.

No real RPC was added.

## Latest XC epoch minimum mocked script entrypoint implementation checkpoint

The XC epoch minimum mocked script entrypoint implementation milestone was completed on the `xc-epoch-minimum-mocked-script-entrypoint` branch.

This milestone adds a mocked/testable runner only.

New runtime file:

- `src/ethereum/ethereum-script-runner.ts`

New test file:

- `tests/ethereum-script-runner.test.ts`

Updated export:

- `src/index.ts`

New exported runner:

```text
runEthereumXcEpochMinimumReadFromProvidedClient(input)
```

New exported runner types:

```text
EthereumScriptRunnerOutput
EthereumScriptRunnerInput
EthereumScriptRunnerEpochMinimumResult
EthereumScriptRunnerResult
```

The runner implements the provided-client path:

```text
env-like input
-> parseEthereumScriptConfig()
-> summarizeEthereumScriptConfig()
-> provided public client
-> createXcEpochMinimumSourceFromReadonlyEthereumPublicClient()
-> source.authoritativeEpochMinimum(lockEpoch)
-> safe output writer
-> safe structured result
```

The implementation preserves the safety boundary:

- does not add real RPC calls
- does not install viem
- does not import viem at runtime
- does not import ethers
- does not read process.env
- does not construct a public client
- does not create an RPC URL factory
- does not construct HTTP transport
- does not accept private keys
- does not accept mnemonics
- does not create signers
- does not create wallet clients
- does not send transactions
- does not call writeContract
- does not call sendTransaction
- does not print rpcUrl
- does not print raw env
- does not print full parsed config

The runner keeps the full parsed config only as a local variable and returns only:

- safeConfigSummary
- epochMinimums
- completed

The runner writes only safe output lines:

- chainId
- lensAddress
- finality
- lockEpochCount
- epochMinimumFunctionName
- hasEpochMinimumAbiPath
- realRpcConfirmed
- epoch minimum result lines
- completed=true

ABI path remains metadata-only in this milestone:

- ABI path may be parsed and summarized
- ABI files are not loaded
- default epoch minimum ABI behavior is preserved

Tests added:

- runs with env-like input and provided mocked public client
- writes safe config summary without RPC URL or API-key-like values
- does not return full parsed config object
- passes confirmed finality policy through to source helper
- passes lock epochs and function name through to contract reads
- keeps ABI path as metadata only
- propagates sanitized parser validation errors
- uses provided public client only

Validation baseline after implementation:

- `npm run typecheck` passed
- `npm test` passed: 37 test files, 286 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Test count increased from:

```text
36 test files / 278 tests
```

to:

```text
37 test files / 286 tests
```

No dependencies changed.

No real RPC was added.

## Latest XC epoch minimum mocked script entrypoint review checkpoint

The XC epoch minimum mocked script entrypoint review milestone was completed on the `xc-epoch-minimum-mocked-script-entrypoint-review` branch.

This milestone is review-only.

New document:

- `implementation/xc-epoch-minimum-mocked-script-entrypoint-review-notes.md`

Reviewed files:

- `src/ethereum/ethereum-script-runner.ts`
- `tests/ethereum-script-runner.test.ts`
- `src/index.ts`
- `docs/checkpoints/current-design-checkpoint.md`

Review conclusion:

- mocked script runner accepted
- provided-client boundary preserved
- no implementation changes required before merge

The review confirms that the runner:

- accepts env-like object
- accepts already provided public client
- accepts output writer abstraction
- calls `parseEthereumScriptConfig()`
- calls `summarizeEthereumScriptConfig()`
- calls `createXcEpochMinimumSourceFromReadonlyEthereumPublicClient()`
- reads epoch minimums through `source.authoritativeEpochMinimum(lockEpoch)`
- writes only safe output lines
- returns only safe structured result

The review confirms that the runner does not add:

- real RPC calls
- viem dependency
- runtime viem imports
- ethers imports
- process.env reads
- public client construction
- RPC URL factory
- HTTP transport construction
- private key support
- mnemonic support
- signer support
- wallet client support
- transaction sending
- writeContract / sendTransaction path
- raw env printing
- full config printing
- RPC URL / API key printing

The review command checked runtime/test/package boundaries:

```bash
grep -RniE "from ['\"]viem['\"]|from ['\"]ethers['\"]|createPublicClient|http\\(|writeContract|sendTransaction|walletClient|privateKey|mnemonic|process\\.env" src tests package.json || true
```

No forbidden runtime/test/package matches were found.

Validation baseline for review:

- `npm run typecheck` passed
- `npm test` passed: 37 test files, 286 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Recommended next milestone after merge:

```text
xc-epoch-minimum-manual-rpc-smoke-script-design
```

The next milestone should still be design-only and should define the future manual-only real RPC smoke script boundary before adding viem or real RPC.

## Latest XC epoch minimum manual RPC smoke script design checkpoint

The XC epoch minimum manual RPC smoke script design milestone was completed on the `xc-epoch-minimum-manual-rpc-smoke-script-design` branch.

This milestone is design-only.

New document:

- `implementation/xc-epoch-minimum-manual-rpc-smoke-script-design.md`

Purpose:

- design the future manual-only real RPC smoke script
- keep real RPC implementation out of this milestone
- keep viem dependency out of this milestone
- keep script file creation out of this milestone
- define the future script-edge-only process.env boundary
- define future read-only public client construction rules
- define safe output and sanitized error policy
- define manual-only execution rules
- define confirmation requirement
- define future chain handling policy
- define ABI path policy for the first real smoke script

The proposed future script file is:

```text
scripts/read-xc-epoch-minimum-source.ts
```

This file was not added in this milestone.

The intended future flow is:

```text
manual script invocation
-> read process.env at script edge only
-> parseEthereumScriptConfig(process.env-like object)
-> create read-only public client
-> runEthereumXcEpochMinimumReadFromProvidedClient({
     env,
     publicClient,
     output
   })
-> print safe output only
```

The future script must require:

```text
XC_ETHEREUM_REAL_RPC_CONFIRM=I_UNDERSTAND_THIS_USES_REAL_RPC
```

The future script may eventually add `viem`, but only at the script-edge implementation milestone.

The design confirms that the future script must not:

- print RPC URL
- print API key
- print raw process.env
- print full config object
- print transport config
- print provider internals
- accept private keys
- accept mnemonics
- create signers
- create wallet clients
- send transactions
- call writeContract
- call sendTransaction
- perform approvals
- perform token transfers
- run as part of npm test
- run as part of npm run build
- run in CI/default package lifecycle scripts

Recommended first supported real chain:

```text
eip155-1
```

Optional future Sepolia support must be explicit:

```text
eip155-11155111
```

ABI path policy for the first real smoke script:

- use default epoch minimum ABI behavior
- parse ABI path presence through existing config parser
- do not load ABI file yet
- create a separate design milestone before ABI file loading

Validation baseline after the design document:

- `npm run typecheck` passed
- `npm test` passed: 37 test files, 286 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

No runtime code changed.

No dependencies changed.

No real RPC was added.

No script file was added.

Recommended next milestone:

```text
xc-epoch-minimum-manual-rpc-smoke-script-design-review
```

## Latest XC epoch minimum manual RPC smoke script design review checkpoint

The XC epoch minimum manual RPC smoke script design review milestone was completed on the `xc-epoch-minimum-manual-rpc-smoke-script-design-review` branch.

This milestone is review-only.

New document:

- `implementation/xc-epoch-minimum-manual-rpc-smoke-script-design-review-notes.md`

Reviewed document:

- `implementation/xc-epoch-minimum-manual-rpc-smoke-script-design.md`

Review conclusion:

- manual RPC smoke script design accepted
- design-only boundary preserved
- no changes required before merge

The review confirms that the design milestone did not add:

- real RPC implementation
- viem dependency
- script file
- process.env runtime usage
- public client construction
- signer support
- wallet client support
- transaction capability
- writeContract / sendTransaction path

The review confirms the intended future script path:

```text
scripts/read-xc-epoch-minimum-source.ts
```

That file was not added in the design milestone.

The review confirms that the future script must remain manual-only and must not run as part of:

- npm test
- npm run build
- CI
- default package lifecycle scripts
- pretest
- postinstall
- prepare
- any automatic check

The review confirms that the future script must require:

```text
XC_ETHEREUM_REAL_RPC_CONFIRM=I_UNDERSTAND_THIS_USES_REAL_RPC
```

The review confirms that future real RPC ownership must stay at the script edge only.

Validation baseline for review:

- `npm run typecheck` passed
- `npm test` passed: 37 test files, 286 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Recommended next milestone after merge:

```text
xc-epoch-minimum-manual-rpc-smoke-script
```

## Latest XC epoch minimum manual RPC smoke script implementation checkpoint

The XC epoch minimum manual RPC smoke script implementation milestone was completed on the `xc-epoch-minimum-manual-rpc-smoke-script` branch.

This milestone adds a manual-only real RPC smoke script.

New script:

- `scripts/read-xc-epoch-minimum-source.ts`

Updated files:

- `package.json`
- `package-lock.json`
- `tsconfig.json`

New dependency:

- `viem`

New manual package script:

```text
npm run smoke:xc-epoch-minimum:rpc
```

The package script runs:

```text
node ./dist/scripts/read-xc-epoch-minimum-source.js
```

The script is included in TypeScript build through:

```text
scripts/**/*.ts
```

The script is manual-only and is not part of:

- `npm test`
- `npm run build`
- CI
- default package lifecycle scripts
- pretest
- postinstall
- prepare

The implementation keeps real RPC ownership at the script edge only.

The script reads `process.env` only inside:

```text
scripts/read-xc-epoch-minimum-source.ts
```

The script constructs the viem public client only inside:

```text
scripts/read-xc-epoch-minimum-source.ts
```

The script then passes the provided read-only public client into:

```text
runEthereumXcEpochMinimumReadFromProvidedClient()
```

The script requires the existing parser confirmation:

```text
XC_ETHEREUM_REAL_RPC_CONFIRM=I_UNDERSTAND_THIS_USES_REAL_RPC
```

Supported chains are explicit:

```text
eip155-1
eip155-11155111
```

The script verifies provider chain ID against configured chain ID before reading contract data.

The script uses only read-only client operations:

- `getChainId`
- `getBlock`
- `readContract`

The script does not add:

- private key support
- mnemonic support
- signer support
- wallet client support
- writeContract
- sendTransaction
- approvals
- token transfers
- any contract write path

Safety checks performed:

- `npm run typecheck` passed
- `npm test` passed: 37 test files, 286 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities
- running `node ./dist/scripts/read-xc-epoch-minimum-source.js` without env safely refused before RPC with:
  - `Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL`

Boundary grep confirmed:

- `viem`
- `createPublicClient`
- `http(`
- `process.env`

appear only in the script-edge file.

Boundary grep also confirmed no source/script/test/package matches for:

- `privateKey`
- `mnemonic`
- `walletClient`
- `writeContract`
- `sendTransaction`

No real RPC URL, API key, private key, mnemonic, or seed phrase was printed.

## Latest XC epoch minimum manual RPC smoke script review checkpoint

The XC epoch minimum manual RPC smoke script review milestone was completed on the `xc-epoch-minimum-manual-rpc-smoke-script-review` branch.

This milestone is review-only.

New document:

- `implementation/xc-epoch-minimum-manual-rpc-smoke-script-review-notes.md`

Reviewed files:

- `scripts/read-xc-epoch-minimum-source.ts`
- `package.json`
- `package-lock.json`
- `tsconfig.json`
- `docs/checkpoints/current-design-checkpoint.md`

Review conclusion:

- manual RPC smoke script implementation accepted
- manual-only boundary preserved
- no implementation changes required before merge

The review confirms:

- `viem`, `createPublicClient`, `http`, and `process.env` appear only in the script-edge file
- no `privateKey`
- no `mnemonic`
- no `walletClient`
- no `writeContract`
- no `sendTransaction`
- no signer path
- no contract write path
- no default/CI execution path

The script file is:

```text
scripts/read-xc-epoch-minimum-source.ts
```

The manual package script is:

```text
npm run smoke:xc-epoch-minimum:rpc
```

The built script safely refused without env before RPC:

```text
Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL
```

Validation baseline for review:

- `npm run typecheck` passed
- `npm test` passed: 37 test files, 286 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

No RPC URL, API key, private key, mnemonic, or seed phrase was printed.

Recommended next milestone after merge:

```text
xc-epoch-minimum-real-rpc-smoke-run-notes
```

## Latest XC epoch minimum real RPC smoke run notes checkpoint

The XC epoch minimum real RPC smoke run notes milestone was started on the `xc-epoch-minimum-real-rpc-smoke-run-notes` branch.

This checkpoint is notes-only.

New document:

- `implementation/xc-epoch-minimum-real-rpc-smoke-run-notes.md`

Purpose:

- document safe manual procedure for a future real RPC smoke run
- avoid printing RPC URLs or API keys
- avoid reading or printing `.env`
- require hidden input for RPC URL
- define allowed sanitized output
- define forbidden output
- define safe failure logging
- keep real RPC run optional and local

Baseline before notes:

- `npm run typecheck` passed
- `npm test` passed: 37 test files, 286 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Manual refusal without env remained safe:

```text
Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL
```

No RPC URL, API key, private key, mnemonic, or seed phrase was printed.

Default next step:

```text
notes-only first
```

## Latest XC epoch minimum real RPC smoke run notes review checkpoint

The XC epoch minimum real RPC smoke run notes review milestone was completed on the `xc-epoch-minimum-real-rpc-smoke-run-notes-review` branch.

This milestone is review-only.

New document:

- `implementation/xc-epoch-minimum-real-rpc-smoke-run-notes-review-notes.md`

Reviewed files:

- `implementation/xc-epoch-minimum-real-rpc-smoke-run-notes.md`
- `docs/checkpoints/current-design-checkpoint.md`

Review conclusion:

- real RPC smoke run notes accepted
- notes-only boundary preserved
- no real RPC run was performed
- no changes required before merge

The review confirms that the notes document defines a safe manual procedure for a future real RPC smoke run.

The notes explicitly prohibit printing or pasting:

- RPC URL
- API key
- private key
- mnemonic
- seed phrase
- `.env` contents
- raw environment content

The review confirms that forbidden commands appear only as warnings inside the notes document, not as executable project scripts:

```text
echo $XC_ETHEREUM_RPC_URL
env
printenv
cat .env
grep RPC .env
```

Validation baseline for review:

- `npm run typecheck` passed
- `npm test` passed: 37 test files, 286 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Manual refusal without env remained safe:

```text
Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL
```

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content was printed.

Recommended next step after merge:

```text
optional manual real RPC smoke run
```

## Latest XC mainnet protocol params real RPC smoke run result checkpoint

A sanitized real RPC smoke run result was recorded on the `xc-epoch-minimum-real-rpc-smoke-run` branch.

New document:

- `implementation/xc-mainnet-protocol-params-real-rpc-smoke-run-result.md`

The run confirmed that Ethereum mainnet RPC was reachable and the deployed xEnchanted Crypto NFT Lens could be read through `getProtocolParams()`.

Network:

```text
chainId=eip155-1
providerChainId=1
```

Lens:

```text
lensAddress=0xd4b90d7392c1565d558c80122dee76b5b3bb6c23
function=getProtocolParams()
```

Result:

```text
xcProtocolParamsSmoke=true
completed=true
```

The previous generic `epochMinimum(lockEpoch)` smoke attempt reached mainnet and validated chain ID, but failed with a sanitized runtime error because the deployed `xEnchantedNFTLens` does not expose `epochMinimum(uint256)`. The correct deployed XC Lens read path is `getProtocolParams()`.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content was printed or recorded.

## Latest XC mainnet protocol params real RPC smoke run review checkpoint

The XC mainnet protocol params real RPC smoke run review milestone was completed on the xc-mainnet-protocol-params-real-rpc-smoke-run-review branch.

This milestone is review-only.

New document:

- implementation/xc-mainnet-protocol-params-real-rpc-smoke-run-review-notes.md

Reviewed files:

- implementation/xc-mainnet-protocol-params-real-rpc-smoke-run-result.md
- docs/checkpoints/current-design-checkpoint.md

Review conclusion:

- sanitized mainnet protocol params real RPC smoke result accepted
- getProtocolParams() read path confirmed for deployed xEnchanted Crypto NFT Lens
- epochMinimum(uint256) mismatch correctly identified as ABI/function mismatch for this Lens
- no changes required before merge

The review confirms the successful sanitized result:

    xcProtocolParamsSmoke=true
    providerChainId=1
    function=getProtocolParams()
    completed=true

The review confirms that no RPC URL, API key, private key, mnemonic, seed phrase, .env content, raw environment content, provider account details, or transport config was recorded.

Validation baseline for review:

- npm run typecheck passed
- npm test passed: 37 test files, 286 tests
- npm run build passed
- npm audit --audit-level=moderate found 0 vulnerabilities

Manual refusal without env remained safe:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

Recommended next step after merge:

    complete current XC epoch minimum / protocol params RPC smoke milestone

## Latest XC epoch minimum / protocol params RPC smoke completion checkpoint

The XC epoch minimum / protocol params RPC smoke milestone was completed on the `xc-epoch-minimum-rpc-smoke-completion-checkpoint` branch.

This milestone is documentation-only.

New document:

- `implementation/xc-epoch-minimum-rpc-smoke-completion-checkpoint.md`

The checkpoint closes the completed chain:

- mocked XC epoch minimum source abstractions
- Ethereum read provider abstraction
- mocked provider wrapper
- viem-like read provider wrapper
- provided-client RPC integration helper
- mocked script config parser
- mocked/testable script runner
- manual-only RPC smoke script
- safe real RPC run notes
- notes review
- sanitized mainnet protocol params smoke run
- smoke run review

Final accepted mainnet deployed XC Lens read path:

    getProtocolParams()

The deployed mainnet xEnchantedNFTLens does not expose:

    epochMinimum(uint256)

The successful sanitized mainnet read confirmed:

    chainId=eip155-1
    providerChainId=1
    lensAddress=0xd4b90d7392c1565d558c80122dee76b5b3bb6c23
    function=getProtocolParams()
    completed=true

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content was committed.

Final validation baseline:

- `npm run typecheck` passed
- `npm test` passed: 37 test files, 286 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Safe refusal without env remained active:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

Recommended next milestone:

    xc-protocol-params-source-design

## Latest XC protocol params source design checkpoint

The XC protocol params source design milestone was completed on the `xc-protocol-params-source-design` branch.

This milestone is design-only.

New document:

- `implementation/xc-protocol-params-source-design.md`

Design conclusion:

- deployed mainnet xEnchantedNFTLens should be modeled through `getProtocolParams()`
- deployed mainnet xEnchantedNFTLens should not be treated as an `epochMinimum(uint256)` source
- a new `XcProtocolParamsSource` should be added separately from the existing `XcEpochMinimumSource`
- implementation should use mocked providers only
- implementation should not add real RPC execution
- implementation should not add dependencies
- implementation should not read process.env
- implementation should not construct public clients
- implementation should not add private key, mnemonic, wallet client, writeContract, or sendTransaction paths

Likely authoritative fields for future X1 Build validation:

- currentEpoch
- currentBaseNominal
- currentXenBurnAmount
- halvingInterval
- xenBurnHalvingInterval
- nextHalvingTs
- genesisTs

Recommended implementation branch after design review:

    xc-protocol-params-source

Expected implementation files:

- `src/ethereum/xc-protocol-params-source.ts`
- `tests/xc-protocol-params-source.test.ts`
- `src/index.ts`

Validation baseline for design:

- `npm run typecheck` passed
- `npm test` passed: 37 test files, 286 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Safe refusal without env remained active:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

## Latest XC protocol params source design review checkpoint

The XC protocol params source design review milestone was completed on the `xc-protocol-params-source-design-review` branch.

This milestone is review-only.

New document:

- `implementation/xc-protocol-params-source-design-review-notes.md`

Reviewed files:

- `implementation/xc-protocol-params-source-design.md`
- `docs/checkpoints/current-design-checkpoint.md`

Review conclusion:

- XC protocol params source design accepted
- design-only boundary preserved
- deployed XC Lens read path confirmed as `getProtocolParams()`
- deployed XC Lens should not be modeled as `epochMinimum(uint256)`
- `XcProtocolParamsSource` should remain separate from `XcEpochMinimumSource`
- no changes required before merge

The review confirms that the future implementation should use mocked providers only and should not add:

- real RPC execution
- new dependencies
- process.env reads
- public client construction
- private key support
- mnemonic support
- wallet client support
- writeContract
- sendTransaction

Likely authoritative fields for later X1 Build validation remain:

- currentEpoch
- currentBaseNominal
- currentXenBurnAmount
- halvingInterval
- xenBurnHalvingInterval
- nextHalvingTs
- genesisTs

Validation baseline for review:

- `npm run typecheck` passed
- `npm test` passed: 37 test files, 286 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Manual refusal without env remained safe:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

Recommended next milestone after merge:

    xc-protocol-params-source

## Latest XC protocol params source review checkpoint

The XC protocol params source review milestone was completed on the `xc-protocol-params-source-review` branch.

This milestone is review-only.

New document:

- `implementation/xc-protocol-params-source-review-notes.md`

Reviewed files:

- `src/ethereum/xc-protocol-params-source.ts`
- `tests/xc-protocol-params-source.test.ts`
- `src/index.ts`

Review conclusion:

- XC protocol params source implementation accepted
- `getProtocolParams()` read path implemented through injected `readContract()`
- implementation remains separate from `XcEpochMinimumSource`
- object-like tuple normalization covered
- array-like tuple normalization covered
- sanitized error handling covered
- no changes required before merge

The implementation added:

- `XcProtocolParamsReadProvider`
- `XcProtocolParams`
- `XcProtocolParamsSource`
- `XcProtocolParamsSourceConfig`
- `createXcProtocolParamsSourceFromEthereumReadProvider()`
- `normalizeXcProtocolParams()`

The review confirms that the implementation does not add:

- real RPC execution
- viem import
- ethers import
- createPublicClient
- http transport
- process.env read
- private key support
- mnemonic support
- wallet client support
- writeContract
- sendTransaction

Validation baseline for review:

- `npm run typecheck` passed
- `npm test` passed: 38 test files, 296 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Manual refusal without env remained safe:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

Recommended next step after merge:

    complete XC protocol params source milestone

## Latest XC protocol params source completion checkpoint

The XC protocol params source milestone was completed on the `xc-protocol-params-source-completion-checkpoint` branch.

This milestone is documentation-only.

New document:

- `implementation/xc-protocol-params-source-completion-checkpoint.md`

The checkpoint closes the completed chain:

- protocol params source design
- protocol params source design review
- mocked/tested source implementation
- implementation review
- merge to main

Implemented source:

- `src/ethereum/xc-protocol-params-source.ts`
- `tests/xc-protocol-params-source.test.ts`
- `src/index.ts`

The source models deployed XC Lens reads through:

    getProtocolParams()

The source remains separate from:

    XcEpochMinimumSource

The implementation uses an injected minimal provider with only:

    readContract()

The implementation does not add:

- real RPC execution
- viem import
- ethers import
- createPublicClient
- http transport
- process.env read
- private key support
- mnemonic support
- wallet client support
- writeContract
- sendTransaction

Final validation baseline:

- `npm run typecheck` passed
- `npm test` passed: 38 test files, 296 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Safe refusal without env remained active:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

Recommended next milestone:

    xc-protocol-params-build-validation-design

## Latest XC protocol params build validation design checkpoint

The XC protocol params build validation design milestone was completed on the `xc-protocol-params-build-validation-design` branch.

This milestone is design-only.

New document:

- `implementation/xc-protocol-params-build-validation-design.md`

Design conclusion:

- future X1 Build validation should consume `XcProtocolParams`
- deployed XC protocol params should be treated as authoritative XC economic context
- validation should use `getProtocolParams()` output through `XcProtocolParamsSource`
- validation should not call real RPC directly
- validation should not hardcode XC economic values when Lens params can be used
- validation should remain pure/mocked in the first implementation

Likely authoritative fields:

- currentEpoch
- currentBaseNominal
- currentXenBurnAmount
- halvingInterval
- xenBurnHalvingInterval
- nextHalvingTs
- genesisTs

Recommended next milestone after review:

    xc-protocol-params-build-validation

Expected first implementation files:

- `src/model/xc-protocol-params-build-validation.ts`
- `tests/xc-protocol-params-build-validation.test.ts`
- `src/index.ts`

Validation baseline for design:

- `npm run typecheck` passed
- `npm test` passed: 38 test files, 296 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Manual refusal without env remained safe:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

## Latest XC protocol params build validation design review checkpoint

The XC protocol params build validation design review milestone was completed on the `xc-protocol-params-build-validation-design-review` branch.

This milestone is review-only.

New document:

- `implementation/xc-protocol-params-build-validation-design-review-notes.md`

Reviewed files:

- `implementation/xc-protocol-params-build-validation-design.md`
- `docs/checkpoints/current-design-checkpoint.md`

Review conclusion:

- XC protocol params build validation design accepted
- design-only boundary preserved
- `XcProtocolParams` accepted as authoritative XC economic context for future X1 Build validation
- protocol params context remains separate from user action proof validation
- future first implementation should remain pure and mocked
- no changes required before merge

The review confirms likely authoritative fields:

- currentEpoch
- currentBaseNominal
- currentXenBurnAmount
- halvingInterval
- xenBurnHalvingInterval
- nextHalvingTs
- genesisTs

The review confirms that the future first implementation should not add:

- real RPC execution
- viem import
- ethers import
- createPublicClient
- http transport
- process.env read
- private key support
- mnemonic support
- wallet client support
- writeContract
- sendTransaction
- bridge logic
- transaction logic

Validation baseline for review:

- `npm run typecheck` passed
- `npm test` passed: 38 test files, 296 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Manual refusal without env remained safe:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

Recommended next milestone after merge:

    xc-protocol-params-build-validation

## Latest XC protocol params build validation review checkpoint

The XC protocol params build validation review milestone was completed on the `xc-protocol-params-build-validation-review` branch.

This milestone is review-only.

New document:

- `implementation/xc-protocol-params-build-validation-review-notes.md`

Reviewed files:

- `src/model/xc-protocol-params-build-validation.ts`
- `tests/xc-protocol-params-build-validation.test.ts`
- `src/index.ts`

Review conclusion:

- XC protocol params build validation implementation accepted
- helper remains pure/mocked
- helper derives Build requirements from `XcProtocolParams`
- helper does not call real RPC
- helper does not import viem or ethers
- helper does not read process.env
- helper does not add wallet or transaction paths
- no changes required before merge

The implementation derives:

- currentEpoch
- requiredBaseNominal
- requiredXenBurnAmount
- requiredXntdLockMinimum
- requiredForgeMinimum
- nextHalvingTs
- genesisTs
- halvingInterval
- xenBurnHalvingInterval

Current requirement derivation:

    requiredBaseNominal = currentBaseNominal
    requiredXenBurnAmount = currentXenBurnAmount
    requiredXntdLockMinimum = currentBaseNominal
    requiredForgeMinimum = currentBaseNominal * 5

Validation baseline for review:

- `npm run typecheck` passed
- `npm test` passed: 39 test files, 309 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Manual refusal without env remained safe:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

Recommended next step after merge:

    complete XC protocol params build validation milestone

## Latest XC protocol params build validation completion checkpoint

The XC protocol params build validation milestone was completed on the `xc-protocol-params-build-validation-completion-checkpoint` branch.

This milestone is documentation-only.

New document:

- `implementation/xc-protocol-params-build-validation-completion-checkpoint.md`

The checkpoint closes the completed chain:

- build validation design
- build validation design review
- pure/mocked implementation
- implementation review
- merge to main

Implemented source:

- `src/model/xc-protocol-params-build-validation.ts`
- `tests/xc-protocol-params-build-validation.test.ts`
- `src/index.ts`

The helper derives current XC Build requirements from `XcProtocolParams`.

Current derivation:

    requiredBaseNominal = currentBaseNominal
    requiredXenBurnAmount = currentXenBurnAmount
    requiredXntdLockMinimum = currentBaseNominal
    requiredForgeMinimum = currentBaseNominal * 5

The helper remains pure/mocked and does not add:

- real RPC execution
- viem import
- ethers import
- createPublicClient
- http transport
- process.env read
- private key support
- mnemonic support
- wallet client support
- writeContract
- sendTransaction

Final validation baseline:

- `npm run typecheck` passed
- `npm test` passed: 39 test files, 309 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Safe refusal without env remained active:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

Recommended next milestone:

    xc-build-validation-integration-design

## Latest XC Build validation integration design checkpoint

The XC Build validation integration design milestone was completed on the `xc-build-validation-integration-design` branch.

This milestone is design-only.

New document:

- `implementation/xc-build-validation-integration-design.md`

Design conclusion:

- `XcProtocolParams` and derived requirements are protocol context, not proof of user action
- first integration should not modify registrar, watcher, or app proof behavior directly
- pure validation should not call `XcProtocolParamsSource`
- derived requirements should be passed explicitly through a validation context
- future app/service integration should combine protocol context and proof validation at orchestration level
- first implementation should remain pure/mocked

Recommended next milestone after review:

    xc-build-validation-context

Expected first implementation files:

- `src/model/xc-build-validation-context.ts`
- `tests/xc-build-validation-context.test.ts`
- `src/index.ts`

Validation baseline for design:

- `npm run typecheck` passed
- `npm test` passed: 39 test files, 309 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Manual refusal without env remained safe:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

## Latest XC Build validation integration design review checkpoint

The XC Build validation integration design review milestone was completed on the `xc-build-validation-integration-design-review` branch.

This milestone is review-only.

New document:

- `implementation/xc-build-validation-integration-design-review-notes.md`

Reviewed files:

- `implementation/xc-build-validation-integration-design.md`
- `docs/checkpoints/current-design-checkpoint.md`

Review conclusion:

- XC Build validation integration design accepted
- design-only boundary preserved
- `XcProtocolParams` and derived requirements remain protocol context, not proof of user action
- first implementation should not modify registrar, watcher, or app proof behavior directly
- first implementation should introduce a pure validation context layer
- no changes required before merge

Recommended next milestone after merge:

    xc-build-validation-context

Expected first implementation files:

- `src/model/xc-build-validation-context.ts`
- `tests/xc-build-validation-context.test.ts`
- `src/index.ts`

The review confirms that the future first implementation should not add:

- real RPC execution
- scripts
- dependencies
- direct `XcProtocolParamsSource` calls inside pure model code
- registrar state transition changes
- watcher candidate generation changes
- global Build requirement enforcement
- epoch policy finalization
- lock/relock rule changes
- bridge logic
- viem import
- ethers import
- createPublicClient
- http transport
- process.env read
- private key support
- mnemonic support
- wallet client support
- writeContract
- sendTransaction

Validation baseline for review:

- `npm run typecheck` passed
- `npm test` passed: 39 test files, 309 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Manual refusal without env remained safe:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

## Latest XC Build validation context review checkpoint

The XC Build validation context review milestone was completed on the `xc-build-validation-context-review` branch.

This milestone is review-only.

New document:

- `implementation/xc-build-validation-context-review-notes.md`

Reviewed files:

- `src/model/xc-build-validation-context.ts`
- `tests/xc-build-validation-context.test.ts`
- `src/index.ts`

Review conclusion:

- XC Build validation context implementation accepted
- context helper remains pure/mocked
- context helper combines `protocolParams` and derived `requirements`
- context helper does not call real RPC
- context helper does not call `XcProtocolParamsSource`
- context helper does not read process.env
- context helper does not modify app, registrar, watcher, or proof submission behavior
- no changes required before merge

The implementation exposes:

- `XcBuildValidationContext`
- `CreateXcBuildValidationContextInput`
- `createXcBuildValidationContextFromProtocolParams()`

The context shape is intentionally minimal:

    {
      protocolParams,
      requirements
    }

Validation baseline for review:

- `npm run typecheck` passed
- `npm test` passed: 40 test files, 316 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Manual refusal without env remained safe:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

Recommended next step after merge:

    complete XC Build validation context milestone

## Latest XC Build validation context completion checkpoint

The XC Build validation context milestone was completed on the `xc-build-validation-context-completion-checkpoint` branch.

This milestone is documentation-only.

New document:

- `implementation/xc-build-validation-context-completion-checkpoint.md`

The checkpoint closes the completed chain:

- integration design
- integration design review
- pure context implementation
- implementation review
- merge to main

Implemented source:

- `src/model/xc-build-validation-context.ts`
- `tests/xc-build-validation-context.test.ts`
- `src/index.ts`

The context helper creates a minimal pure context:

    {
      protocolParams,
      requirements
    }

The helper accepts already-loaded `XcProtocolParams` and derives requirements through:

    deriveCurrentXcBuildRequirements()

The helper remains pure/mocked and does not add:

- real RPC execution
- `XcProtocolParamsSource` usage
- viem import
- ethers import
- createPublicClient
- http transport
- process.env read
- private key support
- mnemonic support
- wallet client support
- writeContract
- sendTransaction
- app behavior changes
- registrar behavior changes
- watcher behavior changes
- proof submission behavior changes

Final validation baseline:

- `npm run typecheck` passed
- `npm test` passed: 40 test files, 316 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Safe refusal without env remained active:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

Recommended next milestone:

    xc-build-validation-app-service-context-design

## Latest XC Build validation app service context design checkpoint

The XC Build validation app service context design milestone was completed on the `xc-build-validation-app-service-context-design` branch.

This milestone is design-only.

New document:

- `implementation/xc-build-validation-app-service-context-design.md`

Design conclusion:

- app/proof submission should eventually accept optional `XcBuildValidationContext`
- context remains protocol context, not proof of user action
- first implementation should be backwards-compatible
- no global Build validity enforcement should be added in the first app-service context branch
- no watcher, registrar, or proof payload behavior should be changed unless explicitly needed
- no real RPC should be added

Recommended next milestone after review:

    xc-build-validation-app-service-context

Suggested future implementation target:

- `src/app/proof-submission.ts`
- `tests/app-proof-submission.test.ts`
- possibly `tests/e2e-watcher-proof-registrar-scenario.test.ts`

Validation baseline for design:

- `npm run typecheck` passed
- `npm test` passed: 40 test files, 316 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Manual refusal without env remained safe:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

## Latest XC Build validation app service context design review checkpoint

The XC Build validation app service context design review milestone was completed on the `xc-build-validation-app-service-context-design-review` branch.

This milestone is review-only.

New document:

- `implementation/xc-build-validation-app-service-context-design-review-notes.md`

Reviewed files:

- `implementation/xc-build-validation-app-service-context-design.md`
- `docs/checkpoints/current-design-checkpoint.md`

Review conclusion:

- XC Build validation app service context design accepted
- design-only boundary preserved
- optional `XcBuildValidationContext` at app/proof submission boundary accepted
- backward compatibility requirement accepted
- first runtime implementation should not add global Build validity enforcement
- first runtime implementation should not change watcher, registrar, or proof payload behavior unless explicitly needed
- no changes required before merge

Recommended optional input name:

    xcBuildValidationContext?: XcBuildValidationContext

The review confirms that the future first implementation should not add:

- real RPC execution
- scripts
- dependencies
- direct `XcProtocolParamsSource` calls
- process.env reads
- viem import
- ethers import
- createPublicClient
- http transport
- private key support
- mnemonic support
- wallet client support
- writeContract
- sendTransaction
- watcher candidate changes
- registrar behavior changes
- proof payload changes unless explicitly needed
- global Build validity enforcement

Validation baseline for review:

- `npm run typecheck` passed
- `npm test` passed: 40 test files, 316 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Manual refusal without env remained safe:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

Recommended next milestone after merge:

    xc-build-validation-app-service-context

## Latest XC Build validation app service context review checkpoint

The XC Build validation app service context review milestone was completed on the `xc-build-validation-app-service-context-review` branch.

This milestone is review-only.

New document:

- `implementation/xc-build-validation-app-service-context-review-notes.md`

Reviewed files:

- `src/app/proof-submission.ts`
- `tests/app-proof-submission.test.ts`

Review conclusion:

- XC Build validation app service context implementation accepted
- optional `xcBuildValidationContext` support added to app proof submission input
- implementation is backwards-compatible
- no global Build validity enforcement added
- no real RPC added
- no `XcProtocolParamsSource` usage added
- no process.env read added
- no watcher, registrar, or proof payload behavior changed
- no changes required before merge

Runtime input now accepts:

    xcBuildValidationContext?: XcBuildValidationContext

The review confirms that the implementation does not add:

- currentEpoch enforcement
- requiredForgeMinimum enforcement
- requiredXntdLockMinimum enforcement
- real RPC execution
- viem import
- ethers import
- createPublicClient
- http transport
- process.env read
- private key support
- mnemonic support
- wallet client support
- writeContract
- sendTransaction
- `XcProtocolParamsSource` usage
- watcher candidate changes
- registrar behavior changes
- proof payload changes
- package dependency changes
- scripts

Validation baseline for review:

- `npm run typecheck` passed
- `npm test` passed: 40 test files, 317 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Manual refusal without env remained safe:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

Recommended next step after merge:

    complete XC Build validation app service context milestone

## Latest XC Build validation app service context completion checkpoint

The XC Build validation app service context milestone was completed on the `xc-build-validation-app-service-context-completion-checkpoint` branch.

This milestone is documentation-only.

New document:

- `implementation/xc-build-validation-app-service-context-completion-checkpoint.md`

The checkpoint closes the completed chain:

- app service context design
- app service context design review
- backwards-compatible runtime implementation
- implementation review
- merge to main

Implemented source:

- `src/app/proof-submission.ts`
- `tests/app-proof-submission.test.ts`

`AppSubmitProofInput` now accepts:

    xcBuildValidationContext?: XcBuildValidationContext

The field is optional.

Existing callers without `xcBuildValidationContext` remain valid.

The protocol-context pipeline now exists in a safe, staged form:

    XcProtocolParams
    -> deriveCurrentXcBuildRequirements()
    -> XcBuildValidationContext
    -> optional xcBuildValidationContext in appSubmitProof()

This milestone does not add:

- global Build validity enforcement
- currentEpoch enforcement
- requiredForgeMinimum enforcement
- requiredXntdLockMinimum enforcement
- real RPC execution
- `XcProtocolParamsSource` usage
- viem import
- ethers import
- createPublicClient
- http transport
- process.env read
- private key support
- mnemonic support
- wallet client support
- writeContract
- sendTransaction
- watcher candidate changes
- registrar behavior changes
- proof payload changes
- package dependency changes
- scripts

Final validation baseline:

- `npm run typecheck` passed
- `npm test` passed: 40 test files, 317 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Safe refusal without env remained active:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

Recommended next milestone:

    xc-build-validation-epoch-policy-design

## Latest XC Build validation epoch policy design checkpoint

The XC Build validation epoch policy design milestone was completed on the `xc-build-validation-epoch-policy-design` branch.

This milestone is design-only.

New document:

- `implementation/xc-build-validation-epoch-policy-design.md`

Design conclusion:

- historical contribution and current active validity are separate layers
- Core redeem proof is historical and should not be invalidated merely because current XC epoch changes
- `history_bld` is historical and non-decreasing
- XNTD lock / relock are the active validity layer
- current epoch context should mainly guide lock/relock requirements and active status
- currentEpoch should not be used to reject historical Core redeem proof
- Forge participation is out of scope for MVP Build validity
- Forge should not be used as an implicit Build activation or epoch validation requirement

MVP epoch policy:

    historical contribution remains historical
    active validity may require current XNTD lock/relock compliance
    currentEpoch should not invalidate Core redeem history
    Forge participation is out of scope for MVP Build validity

Validation baseline for design:

- `npm run typecheck` passed
- `npm test` passed: 40 test files, 317 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Manual refusal without env remained safe:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

Recommended next milestone:

    xc-build-validation-epoch-policy-design-review

## Latest XC Build validation epoch policy design review checkpoint

The XC Build validation epoch policy design review milestone was completed on the `xc-build-validation-epoch-policy-design-review` branch.

This milestone is review-only.

New document:

- `implementation/xc-build-validation-epoch-policy-design-review-notes.md`

Reviewed files:

- `implementation/xc-build-validation-epoch-policy-design.md`
- `docs/checkpoints/current-design-checkpoint.md`

Review conclusion:

- XC Build validation epoch policy design accepted
- design-only boundary preserved
- historical contribution and current active validity remain separate layers
- Core redeem proof remains historical
- `history_bld` remains historical and non-decreasing
- XNTD lock / relock are the active validity layer
- `currentEpoch` should not invalidate historical Core redeem history
- Forge participation is out of scope for MVP Build validity
- no runtime enforcement should be added before a focused implementation milestone

The review confirms that the policy should not add:

- runtime code
- tests
- dependencies
- real RPC execution
- appSubmitProof behavior changes
- watcher behavior changes
- registrar behavior changes
- proof payload behavior changes
- global Build validity enforcement
- Forge requirements
- currentEpoch enforcement in code
- requiredXntdLockMinimum enforcement in code
- BLD transfer/sale rule changes

Accepted MVP epoch policy:

    historical contribution remains historical
    active validity may require current XNTD lock/relock compliance
    currentEpoch should not invalidate Core redeem history
    Forge participation is out of scope for MVP Build validity

Validation baseline for review:

- `npm run typecheck` passed
- `npm test` passed: 40 test files, 317 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Manual refusal without env remained safe:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

Recommended next milestone after merge:

    xc-build-active-validity-rule-design

## Latest XC Build validation epoch policy completion checkpoint

The XC Build validation epoch policy milestone was completed on the `xc-build-validation-epoch-policy-completion-checkpoint` branch.

This milestone is documentation-only.

New document:

- `implementation/xc-build-validation-epoch-policy-completion-checkpoint.md`

The checkpoint closes the completed chain:

- epoch policy design
- epoch policy design review
- merge to main

Accepted MVP epoch policy:

    historical contribution remains historical
    active validity may require current XNTD lock/relock compliance
    currentEpoch should not invalidate Core redeem history
    Forge participation is out of scope for MVP Build validity

Core redeem proof is historical.

`history_bld` is historical and non-decreasing.

XNTD lock / relock are the active validity layer.

`currentEpoch` should not reject historical Core redeem proof.

Forge participation is out of scope for MVP Build validity and should not be used as an implicit Build activation or epoch validation requirement.

This milestone does not add:

- runtime code
- tests
- dependencies
- real RPC execution
- appSubmitProof behavior changes
- watcher behavior changes
- registrar behavior changes
- proof payload behavior changes
- global Build validity enforcement
- Forge requirements
- currentEpoch enforcement in code
- requiredXntdLockMinimum enforcement in code
- BLD transfer/sale rule changes

Final validation baseline:

- `npm run typecheck` passed
- `npm test` passed: 40 test files, 317 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Safe refusal without env remained active:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

Recommended next milestone:

    xc-build-active-validity-rule-design

## Latest XC Build active validity rule design checkpoint

The XC Build active validity rule design milestone was completed on the `xc-build-active-validity-rule-design` branch.

This milestone is design-only.

New document:

- `implementation/xc-build-active-validity-rule-design.md`

Design conclusion:

- active status is an optional current-commitment signal
- active status is based on XNTD lock / relock state
- inactive Build keeps historical contribution
- inactive Build does not lose `history_bld`
- inactive Build does not invalidate Core redeem proof
- external X1 projects may choose whether to use active status
- active status should not be treated as a universal punishment
- Forge participation is out of scope for MVP active validity

Accepted MVP active validity rule:

    Active status is an optional current-commitment signal.
    It is based on XNTD lock / relock state.
    Inactive Build keeps historical contribution.
    External X1 projects may choose whether to use active status.
    Forge participation is out of scope for MVP active validity.

Validation baseline for design:

- `npm run typecheck` passed
- `npm test` passed: 40 test files, 317 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Manual refusal without env remained safe:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

Recommended next milestone:

    xc-build-active-validity-rule-design-review

## Latest XC Build active validity rule design review checkpoint

The XC Build active validity rule design review milestone was completed on the `xc-build-active-validity-rule-design-review` branch.

This milestone is review-only.

New document:

- `implementation/xc-build-active-validity-rule-design-review-notes.md`

Reviewed files:

- `implementation/xc-build-active-validity-rule-design.md`
- `docs/checkpoints/current-design-checkpoint.md`

Review conclusion:

- XC Build active validity rule design accepted
- design-only boundary preserved
- active status is an optional current-commitment signal
- inactive Build keeps historical contribution
- inactive Build does not lose `history_bld`
- inactive Build does not invalidate Core redeem proof
- external X1 projects may choose whether to use active status
- active status should not be treated as universal punishment
- XNTD lock / relock are the basis of active status
- Forge participation is out of scope for MVP active validity

Accepted MVP active validity rule:

    Active status is an optional current-commitment signal.
    It is based on XNTD lock / relock state.
    Inactive Build keeps historical contribution.
    External X1 projects may choose whether to use active status.
    Forge participation is out of scope for MVP active validity.

The review confirms that the policy should not add:

- runtime code
- tests
- dependencies
- real RPC execution
- appSubmitProof behavior changes
- watcher behavior changes
- registrar behavior changes
- proof payload behavior changes
- active status enforcement
- external project policy
- inactive Build history erasure
- Forge requirements
- unlock mechanics
- BLD transfer/sale rule changes

Validation baseline for review:

- `npm run typecheck` passed
- `npm test` passed: 40 test files, 317 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Recommended next milestone after merge:

    xc-build-active-status-model-design

## Latest XC Build active validity rule completion checkpoint

The XC Build active validity rule milestone was completed on the `xc-build-active-validity-rule-completion-checkpoint` branch.

This milestone is documentation-only.

New document:

- `implementation/xc-build-active-validity-rule-completion-checkpoint.md`

The checkpoint closes the completed chain:

- active validity rule design
- active validity rule design review
- merge to main

Accepted MVP active validity rule:

    Active status is an optional current-commitment signal.
    It is based on XNTD lock / relock state.
    Inactive Build keeps historical contribution.
    External X1 projects may choose whether to use active status.
    Forge participation is out of scope for MVP active validity.

Inactive Build does not mean:

- history is invalid
- Build is deleted
- Core redeem proof is rejected
- history_bld is reduced
- available_bld is automatically reduced
- external projects must ignore the Build

External X1 projects may choose whether to use active status.

This milestone does not add:

- runtime code
- tests
- dependencies
- real RPC execution
- appSubmitProof behavior changes
- watcher behavior changes
- registrar behavior changes
- proof payload behavior changes
- active status enforcement
- external project policy
- inactive Build history erasure
- Forge requirements
- unlock mechanics
- BLD transfer/sale rule changes

Final validation baseline:

- `npm run typecheck` passed
- `npm test` passed: 40 test files, 317 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Recommended next milestone:

    xc-build-active-status-model-design

## Latest XC Build active status model design checkpoint

The XC Build active status model design milestone was completed on the `xc-build-active-status-model-design` branch.

This milestone is design-only.

New document:

- `implementation/xc-build-active-status-model-design.md`

Design conclusion:

- active status model should be read-only and non-mutating
- active status should be based on Build state and optional current context
- recommended helper name is `getBuildActiveStatus()`
- recommended status values are `ACTIVE`, `INACTIVE`, and `UNKNOWN`
- recommended model includes `isActive`, `status`, `reason`, `historyBld`, `availableBld`, `lockedXntd`, `requiredXntdLock`, `lockEpoch`, `currentEpoch`, and `needsRelock`
- inactive status does not erase historical contribution
- currentEpoch may affect active status but must not invalidate Core redeem history
- Forge participation remains out of scope for MVP active validity

Recommended reason values:

    ACTIVE_LOCK_CURRENT
    INACTIVE_NO_HISTORY
    INACTIVE_NO_LOCK
    INACTIVE_LOCK_BELOW_REQUIRED
    INACTIVE_RELOCK_REQUIRED
    UNKNOWN_NO_CURRENT_CONTEXT

Validation baseline for design:

- `npm run typecheck` passed
- `npm test` passed: 40 test files, 317 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Recommended next milestone:

    xc-build-active-status-model-design-review

## Latest XC Build active status model design review checkpoint

The XC Build active status model design review milestone was completed on the `xc-build-active-status-model-design-review` branch.

This milestone is review-only.

New document:

- `implementation/xc-build-active-status-model-design-review-notes.md`

Reviewed files:

- `implementation/xc-build-active-status-model-design.md`
- `docs/checkpoints/current-design-checkpoint.md`

Review conclusion:

- XC Build active status model design accepted
- design-only boundary preserved
- active status model is read-only and non-mutating
- recommended helper name is `getBuildActiveStatus()`
- accepted status values are `ACTIVE`, `INACTIVE`, and `UNKNOWN`
- accepted reason values cover active, no history, no lock, below required, relock required, and unknown current context
- inactive status does not erase historical contribution
- currentEpoch may affect active status but must not invalidate Core redeem history
- Forge participation remains out of scope for MVP active status

Accepted status values:

    ACTIVE
    INACTIVE
    UNKNOWN

Accepted reason values:

    ACTIVE_LOCK_CURRENT
    INACTIVE_NO_HISTORY
    INACTIVE_NO_LOCK
    INACTIVE_LOCK_BELOW_REQUIRED
    INACTIVE_RELOCK_REQUIRED
    UNKNOWN_NO_CURRENT_CONTEXT

Accepted model fields:

    isActive
    status
    reason
    historyBld
    availableBld
    lockedXntd
    requiredXntdLock
    lockEpoch
    currentEpoch
    needsRelock

The review confirms that future status calculation must not mutate:

- history_bld
- available_bld
- origin_bld
- lockedXntd
- requiredXntdLock
- lockEpoch
- replay protection state
- registrar state
- proof state

Validation baseline for review:

- `npm run typecheck` passed
- `npm test` passed: 40 test files, 317 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Recommended next milestone after merge:

    xc-build-active-status-model

## Latest XC Build active status model design completion checkpoint

The XC Build active status model design milestone was completed on the `xc-build-active-status-model-design-completion-checkpoint` branch.

This milestone is documentation-only.

New document:

- `implementation/xc-build-active-status-model-design-completion-checkpoint.md`

The checkpoint closes the completed chain:

- active status model design
- active status model design review
- merge to main

Accepted helper name:

    getBuildActiveStatus()

Accepted status values:

    ACTIVE
    INACTIVE
    UNKNOWN

Accepted reason values:

    ACTIVE_LOCK_CURRENT
    INACTIVE_NO_HISTORY
    INACTIVE_NO_LOCK
    INACTIVE_LOCK_BELOW_REQUIRED
    INACTIVE_RELOCK_REQUIRED
    UNKNOWN_NO_CURRENT_CONTEXT

Accepted model fields:

    isActive
    status
    reason
    historyBld
    availableBld
    lockedXntd
    requiredXntdLock
    lockEpoch
    currentEpoch
    needsRelock

The active status model is read-only and non-mutating.

Inactive status does not erase historical contribution.

Unknown status does not mean invalid history.

currentEpoch may affect active status, but must not invalidate Core redeem history.

Forge participation remains out of scope for MVP active status.

Final validation baseline:

- `npm run typecheck` passed
- `npm test` passed: 40 test files, 317 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Recommended next milestone:

    xc-build-active-status-model

## Latest XC Build active status model review checkpoint

The XC Build active status model review milestone was completed on the `xc-build-active-status-model-review` branch.

This milestone is review-only.

New document:

- `implementation/xc-build-active-status-model-review-notes.md`

Reviewed files:

- `src/model/build-active-status.ts`
- `tests/build-active-status.test.ts`
- `src/index.ts`

Review conclusion:

- XC Build active status model implementation accepted
- implementation adds read-only `getBuildActiveStatus()`
- implementation exports helper and types through `src/index.ts`
- implementation adds focused unit tests
- implementation is read-only and non-mutating
- implementation does not change appSubmitProof
- implementation does not change watcher behavior
- implementation does not change registrar behavior
- implementation does not change proof payload behavior
- implementation does not introduce real RPC
- implementation does not introduce Forge requirements

Accepted status values:

    ACTIVE
    INACTIVE
    UNKNOWN

Accepted reason values:

    ACTIVE_LOCK_CURRENT
    INACTIVE_NO_HISTORY
    INACTIVE_NO_LOCK
    INACTIVE_LOCK_BELOW_REQUIRED
    INACTIVE_RELOCK_REQUIRED
    UNKNOWN_NO_CURRENT_CONTEXT

Validation baseline for review:

- `npm run typecheck` passed
- `npm test` passed: 41 test files, 323 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Recommended next milestone after merge:

    xc-build-active-status-model-completion-checkpoint

## Latest XC Build active status model completion checkpoint

The XC Build active status model runtime milestone was completed on the `xc-build-active-status-model-completion-checkpoint` branch.

This milestone is documentation-only.

New document:

- `implementation/xc-build-active-status-model-completion-checkpoint.md`

The checkpoint closes the completed chain:

- active status model design
- active status model design review
- active status model design completion checkpoint
- active status model runtime implementation
- active status model runtime review
- merge to main

Runtime files added:

- `src/model/build-active-status.ts`
- `tests/build-active-status.test.ts`

Runtime export updated:

- `src/index.ts`

Implemented helper:

    getBuildActiveStatus()

Implemented status values:

    ACTIVE
    INACTIVE
    UNKNOWN

Implemented reason values:

    ACTIVE_LOCK_CURRENT
    INACTIVE_NO_HISTORY
    INACTIVE_NO_LOCK
    INACTIVE_LOCK_BELOW_REQUIRED
    INACTIVE_RELOCK_REQUIRED
    UNKNOWN_NO_CURRENT_CONTEXT

Implemented behavior:

- `INACTIVE_NO_HISTORY` when `historyBld == 0`
- `INACTIVE_NO_LOCK` when history exists but no XNTD lock exists
- `ACTIVE_LOCK_CURRENT` when history and sufficient lock exist
- `INACTIVE_LOCK_BELOW_REQUIRED` when a provided current requirement exceeds locked XNTD
- `UNKNOWN_NO_CURRENT_CONTEXT` when strict current context is required but missing

The implementation is read-only and non-mutating.

The implementation does not change:

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

Final validation baseline:

- `npm run typecheck` passed
- `npm test` passed: 41 test files, 323 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Recommended next milestone:

    xc-build-active-status-app-integration-design

## Latest XC Build commitment status rename checkpoint

The Build status terminology was renamed on the `xc-build-commitment-status-rename` branch and documented on the `xc-build-commitment-status-rename-checkpoint` branch.

The rename changed terminology from:

    active status

to:

    commitment status

Reason:

The previous active / inactive wording implied the wrong effect.

It could imply that inactive Build means invalid, disabled, or punished Build.

The intended model is different:

    Build history remains valid.
    Commitment status only describes current XNTD commitment.

Runtime rename:

- `src/model/build-active-status.ts` -> `src/model/build-commitment-status.ts`
- `tests/build-active-status.test.ts` -> `tests/build-commitment-status.test.ts`

Helper rename:

    getBuildActiveStatus()
    -> getBuildCommitmentStatus()

Type rename:

    BuildActiveStatus
    -> BuildCommitmentStatus

Status rename:

    ACTIVE -> COMMITTED
    INACTIVE -> UNCOMMITTED
    UNKNOWN remains UNKNOWN

Reason rename:

    ACTIVE_LOCK_CURRENT -> COMMITMENT_CURRENT
    INACTIVE_NO_HISTORY -> NO_HISTORY
    INACTIVE_NO_LOCK -> NO_COMMITMENT
    INACTIVE_LOCK_BELOW_REQUIRED -> COMMITMENT_BELOW_REQUIRED
    INACTIVE_RELOCK_REQUIRED -> RECOMMITMENT_REQUIRED
    UNKNOWN_NO_CURRENT_CONTEXT remains UNKNOWN_NO_CURRENT_CONTEXT

Meaning:

    commitmentStatus = current XNTD commitment signal

It does not erase or invalidate:

- historyBld
- availableBld
- originBld
- Core redeem history
- Build history

The rename did not change:

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

Validation after merge:

- `npm run typecheck` passed
- `npm test` passed: 41 test files, 323 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Important follow-up:

The previous `xc-build-active-status-app-integration-design` branch used old terminology and should not be merged.

Future app/service design should use:

    commitmentStatus

Recommended next milestone:

    xc-build-commitment-status-app-integration-design

## Latest XC Build commitment status app integration design checkpoint

The XC Build commitment status app integration design milestone was completed on the `xc-build-commitment-status-app-integration-design` branch.

This milestone is design-only.

New document:

- `implementation/xc-build-commitment-status-app-integration-design.md`

Design conclusion:

- app/service layer should expose `commitmentStatus` as optional current XNTD commitment context
- commitment status should not become global enforcement automatically
- UNCOMMITTED Build should not cause historical proof rejection
- appSubmitProof behavior should remain unchanged
- watcher behavior should remain unchanged
- registrar behavior should remain unchanged
- proof payload behavior should remain unchanged
- app/service integration should not call real RPC directly
- current context should be dependency-injected
- external X1 project usage remains optional
- Forge participation remains out of scope for MVP commitment status

Recommended future helper direction:

    appGetBuildView()

Possible future view shape:

    AppBuildView {
      build
      commitmentStatus
    }

Validation baseline for design:

- `npm run typecheck` passed
- `npm test` passed: 41 test files, 323 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Recommended next milestone:

    xc-build-commitment-status-app-integration-design-review

## Latest XC Build commitment status app integration design review checkpoint

The XC Build commitment status app integration design review milestone was completed on the `xc-build-commitment-status-app-integration-design-review` branch.

This milestone is review-only.

New document:

- `implementation/xc-build-commitment-status-app-integration-design-review-notes.md`

Reviewed files:

- `implementation/xc-build-commitment-status-app-integration-design.md`
- `docs/checkpoints/current-design-checkpoint.md`

Review conclusion:

- XC Build commitment status app integration design accepted
- design-only boundary preserved
- `commitmentStatus` remains optional current XNTD commitment context
- commitment status is not global enforcement
- UNCOMMITTED Build must not cause historical proof rejection
- appSubmitProof behavior remains unchanged
- watcher behavior remains unchanged
- registrar behavior remains unchanged
- proof payload behavior remains unchanged
- app/service view layer should not call real RPC directly
- current context should remain dependency-injected
- external X1 project usage remains optional
- Forge participation remains out of scope for MVP commitment status

Accepted future helper direction:

    appGetBuildView()

Accepted future view direction:

    AppBuildView {
      build
      commitmentStatus
    }

The review intentionally does not add future Build actor checks.

Build actor remains a separate future idea and is not part of this milestone.

Validation baseline for review:

- `npm run typecheck` passed
- `npm test` passed: 41 test files, 323 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Recommended next milestone after merge:

    xc-build-commitment-status-app-integration-completion-checkpoint

## Latest XC Build commitment status app integration completion checkpoint

The XC Build commitment status app integration design milestone was completed on the `xc-build-commitment-status-app-integration-completion-checkpoint` branch.

This milestone is documentation-only.

New document:

- `implementation/xc-build-commitment-status-app-integration-completion-checkpoint.md`

The checkpoint closes the completed chain:

- commitment status app integration design
- commitment status app integration design review
- merge to main

Accepted app/service direction:

    expose commitmentStatus as optional current XNTD commitment context

Accepted future helper direction:

    appGetBuildView()

Accepted future view direction:

    AppBuildView {
      build
      commitmentStatus
    }

Boundary preserved:

- appSubmitProof behavior remains unchanged
- watcher behavior remains unchanged
- registrar behavior remains unchanged
- proof payload behavior remains unchanged
- app/service view layer does not call real RPC directly
- current context remains dependency-injected
- external X1 project usage remains optional
- Forge participation remains out of scope for MVP commitment status

Historical safety:

- UNCOMMITTED Build does not mean invalid Build
- UNCOMMITTED Build does not erase historical contribution
- UNCOMMITTED Build does not reject Core redeem proof
- commitmentStatus means current XNTD commitment signal, not Build validity

Build actor is not part of this milestone.

Validation baseline:

- `npm run typecheck` passed
- `npm test` passed: 41 test files, 323 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Recommended next milestone:

    xc-build-commitment-status-app-view

## Latest XC Build commitment status app view review checkpoint

The XC Build commitment status app view review milestone was completed on the `xc-build-commitment-status-app-view-review` branch.

This milestone is review-only.

New document:

- `implementation/xc-build-commitment-status-app-view-review-notes.md`

Reviewed files:

- `src/app/build-view.ts`
- `tests/app-build-view.test.ts`
- `src/index.ts`

Review conclusion:

- XC Build commitment status app view implementation accepted
- implementation adds read-only `appGetBuildView()`
- implementation returns Build state plus commitmentStatus
- implementation exports helper and types through `src/index.ts`
- implementation handles optional fields correctly under `exactOptionalPropertyTypes`
- implementation adds focused unit tests
- implementation is read-only and non-mutating
- implementation does not change appSubmitProof
- implementation does not change watcher behavior
- implementation does not change registrar behavior
- implementation does not change proof payload behavior
- implementation does not introduce real RPC
- implementation does not introduce Forge requirements
- implementation does not introduce Build actor scope

Implemented app view:

    AppBuildView {
      build
      commitmentStatus
    }

Implemented helper:

    appGetBuildView()

Validation baseline for review:

- `npm run typecheck` passed
- `npm test` passed: 42 test files, 328 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Recommended next milestone after merge:

    xc-build-commitment-status-app-view-completion-checkpoint

## Latest XC Build commitment status app view completion checkpoint

The XC Build commitment status app view runtime milestone was completed on the `xc-build-commitment-status-app-view-completion-checkpoint` branch.

This milestone is documentation-only.

New document:

- `implementation/xc-build-commitment-status-app-view-completion-checkpoint.md`

The checkpoint closes the completed chain:

- commitment status app integration design
- commitment status app integration design review
- commitment status app integration completion checkpoint
- commitment status app view runtime implementation
- commitment status app view runtime review
- merge to main

Runtime files added:

- `src/app/build-view.ts`
- `tests/app-build-view.test.ts`

Runtime export updated:

- `src/index.ts`

Implemented helper:

    appGetBuildView()

Implemented app view:

    AppBuildView {
      build
      commitmentStatus
    }

The helper is read-only and non-mutating.

The helper exposes commitmentStatus as app/service context.

The helper does not enforce commitmentStatus.

The helper does not reject historical proofs.

The helper does not treat UNCOMMITTED as invalid Build history.

The milestone did not change:

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

Final validation baseline:

- `npm run typecheck` passed
- `npm test` passed: 42 test files, 328 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Recommended next milestone:

    final-mvp-readiness-checkpoint

## Latest final MVP readiness checkpoint

The final MVP readiness checkpoint was completed on the `final-mvp-readiness-checkpoint` branch.

This milestone is documentation-only.

New document:

- `docs/final-mvp-readiness-checkpoint.md`

The checkpoint records that the current repository is a tested MVP implementation lab.

Current main baseline:

    main -> 433e041 Merge branch 'xc-build-commitment-status-app-view-completion-checkpoint'

Validation baseline:

- `npm run typecheck` passed
- `npm test` passed: 42 test files, 328 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Implemented MVP flow:

    watcher candidate
    -> proof object
    -> appSubmitProof
    -> registrar handler
    -> BuildState update
    -> replay protection
    -> snapshot persistence / CLI inspection

Major implemented layers:

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
- snapshot verification / backup / recovery
- read-only CLI command layer
- Ethereum read provider wrappers
- authoritative XC epoch minimum source and provider source
- XC protocol params source
- XC protocol params build validation
- XC Build validation context
- XC Build commitment status model
- app Build view exposing commitmentStatus

Commitment status terminology is accepted:

    commitmentStatus = current XNTD commitment signal

Implemented app view:

    appGetBuildView()

with:

    AppBuildView {
      build
      commitmentStatus
    }

The MVP remains a trusted-indexer / trusted-registrar implementation lab.

The MVP is not yet:

- production chain deployment
- trustless proof verification system
- live watcher service runtime
- bridge execution system
- token issuance deployment
- UI product
- operator production stack

Build actor remains a future layer and is not part of the completed MVP scope.

Decision:

    The xEnchanted X1 Build Lab MVP implementation lab is complete at the current scope.
    The completed scope is a tested implementation lab, not a production deployment.
    Next work should be post-MVP readiness, review, and hardening, not further MVP scope expansion.

## Latest post-MVP Gateway and X1-native model checkpoint

This checkpoint records the post-MVP direction after the completed implementation lab, read-only mainnet protocol params smoke, gateway terminology update, and X1-native model design.

Commits / milestones included:

- `cb8b30a` — Merge branch `xntd-to-xxxl-gateway-terminology-risk-update`
- `c9d21bf` — Update gateway terminology and risk notes
- `1c099b8` — Merge branch `x1-native-forge-naming-sync`
- `7e16e17` — Clarify X1 Forge naming model
- `7e6567a` — Merge branch `x1-forge-stake-dual-nominal-model`
- `65851dc` — Design X1 Forge Stake dual nominal model
- `f1bb351` — Merge branch `x1-native-model-readme-sync`
- `52f4d48` — Link X1 native dual nominal model from README

Current validation baseline remains:

- `npm run typecheck` passed
- `npm test` passed: 42 test files, 328 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

### Terminology update

The previous "XNTD bridge to X1" framing has been replaced by the more accurate gateway framing:

    XNTD-to-XXXL Gateway

The reason is that the model is not a wrapped bridge and not a transfer of the same token.

The intended model is:

    source-chain XNTD burn -> X1-native XXXL mint

XNTD is destroyed on the source chain.

XXXL is minted on X1 as a new X1-native token with different origin and future utility.

### Current gateway direction

The current gateway design is documented in:

- `docs/gateway/xntd-to-xxxl-burn-to-mint-gateway-design.md`

Primary route:

    Ethereum XNTD burn -> X1 XXXL mint

Initial source-chain weight:

    Ethereum = 10000 bps

Future source chains may be added later with reduced source-chain weights.

Formula:

    xxxlMintAmount = burnedAmount * sourceChainWeightBps / 10000

The gateway should mint one unified XXXL token, not multiple origin-specific token classes.

Source-chain differences should be reflected through deterministic conversion weights and gateway history, not through separate token classes.

### Gateway guardians

The current target model is:

    5 gateway guardians
    3-of-5 threshold

The 700+ X1 validators are not the quorum.

They are a future candidate pool for finding the first small group of willing gateway / Build infrastructure operators.

Gateway guardians verify source-chain burn evidence and approve deterministic XXXL mint messages.

They must not:

- change recipient
- change amount
- choose custom coefficients for individual users
- rewrite gateway history
- mint without burn evidence
- act as protocol governance over XC core rules

### Gateway risk notes now documented

Theo's bridge-risk review was incorporated into the gateway design.

The gateway document now explicitly covers:

- irreversible burn / no undo
- reorg safety policy
- mint failure recovery
- coefficient governance principle
- guardian independence
- guardian set update timelock
- required gateway-risk items before implementation
- emergency pause as a future required design item
- transparent fee schedule as a future required design item
- audit requirement as a future required design item

The gateway remains a design-only future layer.

It is not implemented.

It is not deployed.

It is not approved for production.

### Relationship to XC core

The gateway is an optional burn-to-mint conversion layer.

It does not change immutable XC core protocol rules.

It does not give gateway operators admin power over XC core.

It does not modify Ethereum-side XC history.

Users who do not accept gateway risk can choose not to use the gateway.

### X1-native token direction

The X1-native token name is:

    XXXL

XXXL is not Ethereum XNTD.

XXXL is a new X1-native token that may be minted from:

- verified Ethereum XNTD burns
- future source-chain XNTD burns with reduced weights
- future X1-native mechanics, if designed

This naming avoids the false impression that X1 receives the same token as Ethereum XNTD.

### X1-native Forge / Stake direction

The X1-native dual nominal model is documented in:

- `docs/x1-native/x1-forge-stake-dual-nominal-model.md`

Current working names:

- Gateway: `XNTD-to-XXXL Gateway`
- Token: `XXXL`
- Forge mechanic: `X1 Forge`
- Forge object: `X1 Forged Position`
- Stake mechanic: `X1 Stake`
- Build: future memory / state layer

X1 Forge continues the Forge idea under X1-native rules.

It is not a direct copy of Ethereum XC Forge.

### Dual nominal model

Future X1 Forged Positions may use two nominal values:

    mainNominal
    stakeNominal

Meaning:

    mainNominal = redeem / conservative value
    stakeNominal = staking reward power

mainNominal may grow conservatively, for example by summing parent main nominals.

stakeNominal may grow softly with level through a mild coefficient or level bonus.

Ethereum Core-style `*3` growth is considered too aggressive for this X1-native Forge / Stake purpose.

Important rule:

    redeem must not use stakeNominal

stakeNominal is staking power only.

### Economic principle

X1 Forge should transform liquid XXXL into a long-term X1 Forged Position.

X1 Stake may give that position slow productive value.

Stake yield must not quickly neutralize the XXXL burned or committed through Forge-like actions.

If stake rewards quickly return what was burned or committed, Forge becomes delayed emission and loses its supply-discipline purpose.

### Frontend role

The existing xEnchanted frontend may later be used as the UX interface for X1 Forge and X1 Stake.

The frontend should not be the source of truth.

X1-side rules must define and enforce:

- formulas
- limits
- nominal calculations
- redeem calculations
- stake reward calculations
- APR / duration rules, if used

Frontend previews are UX helpers only.

### Strategic sequence

Current post-MVP practical direction:

    Gateway brings energy into X1.
    XXXL carries that energy.
    X1 Forge transforms liquid XXXL into long-term positions.
    X1 Stake gives those positions slow productive value.
    Build records participation / history / state later.

This means the practical focus may shift toward gateway and X1-native economic foundations before returning to Build actor or full Build program implementation.

### Current non-goals

The current repository still does not implement:

- XNTD-to-XXXL gateway runtime
- Ethereum burn contract
- X1 XXXL mint program
- gateway guardian runtime
- X1 Forge program
- X1 Stake program
- XXXL token implementation
- Build actor
- production deployment

All of these remain future layers.

### Next recommended review focus

Recommended next review focus:

1. Gateway risk review after terminology update
2. X1-native Forge / Stake dual nominal model review
3. Immutable XXXL mint core vs governed gateway layer architecture
4. Guardian / gateway operator model
5. X1-side program architecture before implementation

Do not start implementation until the gateway / XXXL / X1 Forge direction is reviewed again as a whole.


## Latest Theo post-MVP architecture review notes

Theo reviewed the full post-MVP Gateway / XXXL / X1 Forge / X1 Stake / Build direction after the terminology and checkpoint updates.

Review result:

    No architecture-level blockers.

The strategic line was considered coherent and mature for the design / hardening phase.

Reviewed line:

    Gateway brings energy into X1.
    XXXL carries that energy.
    X1 Forge transforms liquid XXXL into long-term positions.
    X1 Stake gives those positions slow productive value.
    Build records participation / history / state later.

### Main structural recommendation

The most important recommendation is:

    XXXL mint core should be immutable for Stage 1.

For Stage 1:

- Ethereum route should be immutable / hardcoded in the X1 mint core
- Ethereum source-chain weight should be `10000 bps`
- gateway guardians should not control XXXL monetary policy
- gateway guardians should only verify burn evidence and approve deterministic mint messages
- future source routes should use separately reviewed adapters / route definitions
- coefficient changes must not be retroactive

This separates:

    verification work -> gateway guardians
    monetary conversion rules -> immutable mint core / immutable route rules

### Additional review notes incorporated

The design should explicitly document:

- XXXL is a different asset from Ethereum XNTD
- gateway conversion does not create a price peg between XNTD and XXXL
- gateway is one-way
- XXXL cannot be converted back to XNTD through the gateway
- first guardian set may be bootstrapped by the project
- bootstrapped trust should be disclosed for Stage 1
- lost guardian keys require a recovery / rotation path
- Build actor can remain future scope
- a minimal Build event recorder may run in parallel if useful

### X1 Forge / Stake UX note

The technical terms remain:

    mainNominal
    stakeNominal

Recommended user-facing labels:

    mainNominal -> Redeem Value
    stakeNominal -> Staking Power

This avoids confusing users with two different "nominal" values.

### Current decision

The next design-hardening step is to preserve the current architecture while strengthening the Stage 1 boundary:

    immutable XXXL mint core
    immutable Ethereum route rules
    governed gateway operations only
    no guardian control over monetary policy

Implementation should not begin until this Stage 1 architecture is reviewed as a whole.


## Latest gateway precedent and novelty note

A follow-up review noted that no direct precedent should be assumed for the full XXXL gateway model.

The model combines:

- source-chain burn
- destination-chain mint
- one-way conversion
- future multi-source inputs
- variable source-chain weights
- one unified destination token class
- no wrapped source-token representation
- no reverse gateway redemption path

This makes the XNTD-to-XXXL Gateway different from standard lock/mint bridges, wrapped-token bridges, native multi-chain issuance, and swap-based systems.

Design consequence:

    wrapped-bridge assumptions are not sufficient.

The gateway should be treated as a novel burn-to-mint gateway pattern.

This strengthens the need for:

- narrow Stage 1 scope
- immutable Stage 1 mint rules
- independent security analysis
- explicit no-reverse-direction UX disclosure
- conservative production readiness gates

This note does not approve implementation or deployment.

## Latest Stage 1 XXXL Gateway planning checkpoint

This checkpoint records the current Stage 1 XXXL Gateway planning baseline.

Current main baseline:

- `d4a6f7f` — Merge branch `stage-1-xxxl-gateway-implementation-plan`
- `7303988` — Link Stage 1 gateway implementation plan from README
- `dc8eae9` — Add Stage 1 XXXL gateway implementation plan
- `57218f1` — Merge branch `gateway-precedent-novelty-note`
- `598846a` — Add gateway precedent and novelty note
- `0423fa5` — Merge branch `stage-1-xxxl-gateway-architecture`
- `c7def4b` — Link Stage 1 gateway architecture from README
- `26d40b2` — Define Stage 1 XXXL gateway architecture

Current validation baseline after merge:

- `npm run typecheck` passed
- `npm test` passed: 42 test files, 328 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

### Stage 1 Gateway document set

The Stage 1 Gateway design is now documented at three levels:

1. General Gateway Design

   `docs/gateway/xntd-to-xxxl-burn-to-mint-gateway-design.md`

2. Stage 1 Architecture Boundary

   `docs/gateway/stage-1-xxxl-gateway-architecture.md`

3. Stage 1 Implementation Plan

   `docs/gateway/stage-1-xxxl-gateway-implementation-plan.md`

The README document map links these gateway documents.

### Current Stage 1 boundary

The current Stage 1 boundary is:

    Ethereum XNTD burn
    -> immutable X1 XXXL mint core
    -> XXXL mint

Stage 1 is Ethereum-only.

Stage 1 source route:

    sourceChain = Ethereum
    sourceToken = XNTD
    sourceChainWeightBps = 10000

Stage 1 formula:

    xxxlMintAmount = burnedAmount

Equivalent full formula:

    xxxlMintAmount = burnedAmount * 10000 / 10000

This does not mean XXXL is the same asset as XNTD.

It means only that the Ethereum Stage 1 source route uses a full-weight conversion coefficient.

### Core architectural separation

Stage 1 must preserve the separation between:

    verification work
    monetary conversion rules

Gateway guardians handle verification work.

The immutable X1 mint core / immutable route rules define monetary conversion rules.

Gateway guardians must not control XXXL monetary policy.

Gateway guardians must not be able to:

- change the Ethereum source weight
- add source chains in Stage 1
- mint XXXL without valid burn evidence
- choose custom coefficients for individual users
- modify already-processed burn history

### Novelty / precedent note

The gateway is not a standard wrapped-token bridge.

No direct precedent is assumed for the full XXXL model.

The model combines:

- source-chain burn
- destination-chain mint
- one-way conversion
- future multi-source inputs
- variable source-chain weights
- one unified destination token class
- no wrapped source-token representation
- no reverse gateway redemption path

Design consequence:

    wrapped-bridge assumptions are not sufficient.

The model should be treated as a novel burn-to-mint gateway pattern.

Independent security analysis is required before any production deployment.

### Stage 1 implementation plan components

The implementation plan currently identifies these future components:

1. Ethereum burn contract / function
2. Ethereum burn event format
3. X1 XXXL token / mint core
4. X1 processed burn registry
5. deterministic gateway message format
6. guardian signing format
7. guardian verification runtime
8. relayer runtime
9. read-only watcher / indexer
10. frontend gateway flow
11. monitoring and incident response
12. staging test environment
13. production readiness checklist

This is planning only.

No runtime implementation has started.

### Current implementation order

Recommended future implementation order:

1. finalize Ethereum burn event schema
2. finalize deterministic mint message schema
3. design X1 XXXL mint core
4. design processed burn registry
5. design guardian key / signature model
6. design guardian runtime
7. design relayer runtime
8. design frontend flow
9. design monitoring / incident response
10. build local prototype
11. build staging prototype
12. run full staging tests
13. external review / audit
14. production readiness decision

### Stage 1 non-goals

Stage 1 still does not include:

- reverse XXXL -> XNTD conversion
- sidechain source routes
- mutable source-chain coefficients
- X1 Forge implementation
- X1 Stake implementation
- Build actor
- full Build program
- BLD marketplace
- production slashing mechanics
- multi-chain expansion

### Production readiness blockers

Before any production deployment, Stage 1 still requires:

- Ethereum burn path reviewed
- X1 mint core reviewed
- immutable Stage 1 rules confirmed
- guardian set selected
- bootstrapped trust disclosed
- guardian key management documented
- guardian rotation documented
- lost-key recovery documented
- reorg/finality policy finalized
- deterministic message format finalized
- signature verification finalized
- replay protection tested
- mint retry tested
- emergency pause designed
- fee model disclosed
- frontend disclosure implemented
- monitoring implemented
- incident response documented
- external review / audit plan completed

### Current decision

The repository now has a coherent Stage 1 Gateway planning baseline.

The next work should be review-driven hardening of the Stage 1 implementation plan before any runtime code is added.

Implementation should not begin until the following are reviewed as a whole:

- Ethereum burn event format
- X1 immutable mint core design
- guardian signature format
- processed burn replay protection
- guardian rotation / key recovery
- reorg/finality policy
- mint retry policy
- frontend disclosure flow
- incident response boundary

This checkpoint does not approve implementation or deployment.

## Latest Stage 1 Ethereum burn event schema checkpoint

This checkpoint records the first Stage 1 technical design layer after the Stage 1 Gateway planning baseline.

Current main baseline:

- `d596eb9` — Merge branch `stage-1-ethereum-burn-event-schema`
- `93d8119` — Link Ethereum burn event schema from README
- `a97ce51` — Define Stage 1 Ethereum burn event schema
- `c0fc928` — Merge branch `stage-1-gateway-planning-checkpoint`
- `d49e3a6` — Update checkpoint with Stage 1 gateway planning baseline

Current validation baseline after merge:

- `npm run typecheck` passed
- `npm test` passed: 42 test files, 328 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

### New technical design document

The first Stage 1 technical design layer is now documented in:

    docs/gateway/stage-1-ethereum-burn-event-schema.md

This document defines the Ethereum-side burn event schema for the XNTD-to-XXXL Gateway.

It is still design-only.

No runtime code was added.

No contracts or X1 programs were implemented.

No deployment was approved.

### Purpose of the burn event schema

The Ethereum burn event becomes source evidence for X1 XXXL minting.

The Stage 1 route remains:

    Ethereum XNTD burn
    -> immutable X1 XXXL mint core
    -> XXXL mint

The Ethereum burn event is only source evidence.

It does not mint XXXL by itself.

It does not authorize arbitrary minting.

It does not create wrapped XNTD.

It does not create a reverse redemption claim.

### Preferred Ethereum-side direction

Preferred function direction:

    burnForX1Gateway(x1Recipient, amount)

Preferred event direction:

    XntdBurnedForX1Gateway

The event should include:

- sourceSender
- x1RecipientHash
- x1Recipient
- burnedAmount
- sourceChainId
- sourceToken
- sourceNonce

### Canonical replay key direction

The canonical replay key should be derived from Ethereum log identity:

    canonicalEventKey = hash(sourceChainId, sourceToken, sourceBurnTxHash, sourceBurnEventIndex)

Reason:

- sourceNonce is useful for indexing and user display
- replay protection should bind to the exact Ethereum log
- transaction hash + log index ties the X1 mint to one emitted burn event

### Guardian verification scope

The burn event schema defines guardian acceptance and rejection rules.

Guardians may accept a burn only if:

- event name matches expected Stage 1 event
- source chain is Ethereum mainnet
- source token is the expected XNTD token
- burn transaction succeeded
- event exists in a canonical finalized block
- x1Recipient is present and non-empty
- burnedAmount > 0
- canonicalEventKey is derived correctly
- source burn has not already been processed on X1
- expected xxxlMintAmount is derived correctly

Guardians must reject wrong-chain, wrong-token, failed, unfinalized, reorged-out, zero-amount, empty-recipient, malformed, duplicate, or incomplete evidence.

### X1 mint message mapping direction

The burn event maps into the future deterministic X1 mint message:

    sourceSender -> sourceSender
    x1Recipient -> x1Recipient
    burnedAmount -> burnedAmount
    sourceChainId -> sourceChainId
    sourceToken -> sourceToken
    tx hash -> sourceBurnTxHash
    log index -> sourceBurnEventIndex
    canonicalEventKey -> canonicalEventKey
    sourceChainWeightBps -> 10000
    xxxlMintAmount -> burnedAmount

The next technical design step should define this deterministic gateway message schema.

### Current open questions before implementation

Open questions remain:

1. Should the burn function live in XNTD or a dedicated gateway burn contract?
2. Can the burn path be no-admin / immutable?
3. Does Stage 1 require approve + burn, or can it be one transaction?
4. What exact type should x1Recipient use?
5. Should sourceNonce be included?
6. What minimum recipient validation should Ethereum enforce?
7. Should burn amount min/max exist?
8. What finality rule should guardians use?
9. What exact canonicalEventKey encoding should X1 use?
10. How should the frontend show pending / finalized / approved / minted states?

### Next technical design document

The next recommended technical design document is:

    docs/gateway/stage-1-gateway-message-schema.md

Purpose:

    define the exact deterministic message guardians sign for X1 XXXL mint approval.

This should be done before any runtime implementation.

### Current decision

The repository now has:

- Stage 1 Gateway general design
- Stage 1 Gateway architecture boundary
- Stage 1 Gateway implementation plan
- Stage 1 Ethereum burn event schema

The next step is to define the gateway message schema.

Implementation should still not begin.

## Latest Stage 1 gateway message schema checkpoint

The Stage 1 gateway message schema milestone was completed on the stage-1-gateway-message-schema branch.

Commits:

- be7896b Define Stage 1 gateway message schema
- 457abba Merge branch 'stage-1-gateway-message-schema'

This milestone adds the next technical design layer for the XNTD-to-XXXL Gateway.

Design document added:

- docs/gateway/stage-1-gateway-message-schema.md

Purpose:

Define the deterministic message that guardians sign for X1 XXXL mint approval.

This is a design-only milestone.

It does not implement:

- Ethereum contracts
- X1 programs
- XXXL token runtime
- guardian runtime
- relayer runtime
- watcher runtime
- frontend gateway flow
- real RPC reads
- env reads
- private keys
- API keys
- mnemonic handling
- deployment logic

Current Stage 1 route:

Ethereum XNTD burn -> immutable X1 XXXL mint core -> XXXL mint

Architecture boundary remains:

gateway guardians = verification layer

immutable mint core / route rules = monetary conversion rules

Guardians must not control XXXL monetary policy.

The message schema defines:

- messageType
- schemaVersion
- routeId
- sourceChainId
- sourceToken
- sourceSender
- sourceBurnTxHash
- sourceBurnEventIndex
- sourceBlockNumber
- sourceBlockHash
- sourceNonce
- canonicalEventKey
- x1Recipient
- x1RecipientHash
- burnedAmount
- sourceChainWeightBps
- xxxlMintAmount
- mintToken
- optional deadlineOrFinalityBlock
- optional messageNonce

Preferred Stage 1 constants:

- messageType = X1_GATEWAY_MINT
- schemaVersion = 1
- routeId = ETHEREUM_XNTD_TO_X1_XXXL_STAGE_1
- sourceChainId = Ethereum mainnet
- sourceToken = expected Ethereum XNTD token
- sourceChainWeightBps = 10000
- xxxlMintAmount = burnedAmount
- mintToken = XXXL

Replay protection direction:

canonicalEventKey = hash(sourceChainId, sourceToken, sourceBurnTxHash, sourceBurnEventIndex)

The canonical replay anchor remains the exact Ethereum log:

- transaction hash
- log index

The design keeps sourceNonce useful for indexing and user display, but not as the primary replay key.

Deterministic derivation:

A finalized Ethereum burn event should derive one canonical X1GatewayMintMessage.

Any participant deriving the message from the same finalized Ethereum log should produce the same payload.

Guardian responsibility:

Guardians sign only after verifying:

- expected event name
- Ethereum mainnet source chain
- expected XNTD source token
- succeeded burn transaction
- canonical finalized block inclusion
- non-empty X1 recipient
- burnedAmount > 0
- correct x1RecipientHash
- correct canonicalEventKey
- unprocessed source burn on X1
- sourceChainWeightBps == 10000
- xxxlMintAmount == burnedAmount
- expected message type, schema version, route id, and mint token

Relayer responsibility:

The relayer transports signed messages and evidence references.

The relayer must not define monetary values.

The relayer must not be able to change:

- x1Recipient
- burnedAmount
- xxxlMintAmount
- sourceChainWeightBps
- canonicalEventKey

X1 verification responsibility:

The X1 mint path should verify the exact canonical message payload, guardian threshold, immutable route rules, and processed-burn registry before minting.

Only after successful verification should X1 mark canonicalEventKey as processed and mint XXXL to x1Recipient.

Frontend state mapping documented:

- Burn submitted
- Burn confirmed
- Burn finalized
- Guardian approval pending
- Guardian approved
- Relayer submitted
- XXXL minted
- Already processed / duplicate
- Rejected evidence

Important semantic boundary:

Stage 1 uses full-weight Ethereum conversion.

This does not mean XXXL is Ethereum XNTD.

This does not create a price peg.

This is not a standard wrapped bridge.

Open questions before implementation include:

- exact canonical binary encoding
- hash function
- signature standard
- typed message hash format
- X1 recipient type
- finality rule
- signer set epoch / version
- evidence metadata requirements
- frontend rejected / reorged evidence display
- relayer retry representation

Validation:

- docs-only change
- no runtime tests required before docs-only commit

Implementation should still not begin until message encoding, signature format, finality rule, and X1 recipient type are reviewed.

## Latest Stage 1 gateway canonical encoding checkpoint

The Stage 1 gateway canonical encoding milestone was completed on the stage-1-gateway-canonical-encoding branch.

Commits:

- 8a4b754 Define Stage 1 gateway canonical encoding
- 88bbaf2 Merge branch 'stage-1-gateway-canonical-encoding'

This milestone adds the preferred canonical encoding direction for the XNTD-to-XXXL Gateway message layer.

Design document added:

- docs/gateway/stage-1-gateway-canonical-encoding.md

Purpose:

Define how Stage 1 gateway message fields should become canonical bytes, hashes, and guardian signed payloads.

This is a design-only milestone.

It does not implement:

- Ethereum contracts
- X1 programs
- XXXL token runtime
- X1 mint core
- processed burn registry
- guardian runtime
- relayer runtime
- watcher runtime
- frontend gateway flow
- real RPC reads
- env reads
- private keys
- API keys
- mnemonic handling
- deployment logic

The main security principle:

Every participant deriving the same Stage 1 message from the same finalized Ethereum burn log must produce the same canonical bytes.

If two honest implementations can produce different bytes for the same intended message, the encoding is not acceptable.

If two different messages can produce the same bytes or hash, the encoding is not acceptable.

If signatures can be reused across routes, chains, tokens, schemas, message types, networks, or mint cores, the encoding is not acceptable.

Preferred canonical encoding direction:

- explicit field order
- fixed-width numeric fields
- fixed-width hashes
- explicit dynamic-bytes hashing
- bytes32 constants for domain fields
- no string concatenation
- no JSON canonicalization
- no locale-dependent formatting
- no decimal string amount encoding
- no implicit field omission
- no unordered maps
- no optional-field ambiguity

Preferred fixed field order:

1. messageType
2. schemaVersion
3. routeId
4. sourceChainId
5. sourceToken
6. sourceSender
7. sourceBurnTxHash
8. sourceBurnEventIndex
9. sourceBlockNumber
10. sourceBlockHash
11. sourceNonce
12. canonicalEventKey
13. x1RecipientHash
14. burnedAmount
15. sourceChainWeightBps
16. xxxlMintAmount
17. mintToken
18. deadlineOrFinalityBlock
19. messageNonce

Optional-field direction:

- include all fields in the canonical payload
- encode unused optional fields as zero
- do not omit optional fields

Preferred domain constants:

- messageType = hash("X1_GATEWAY_MINT")
- routeId = hash("ETHEREUM_XNTD_TO_X1_XXXL_STAGE_1")
- mintToken = hash("XXXL")

Preferred x1Recipient direction:

- keep x1Recipient type open until X1 runtime constraints are confirmed
- derive x1RecipientHash = hash(x1RecipientBytes)
- include x1RecipientHash in the signed payload
- carry raw x1Recipient bytes as execution / evidence payload
- verify hash(x1RecipientBytes) == x1RecipientHash before minting

canonicalEventKey direction remains:

canonicalEventKey = hash(sourceChainId, sourceToken, sourceBurnTxHash, sourceBurnEventIndex)

Preferred message hash direction:

messageHash = hash(domainSeparator, encodedGatewayMintMessage)

Guardian signing boundary:

Guardians should sign the deterministic messageHash, not loosely structured JSON.

The guardian runtime may display decoded fields for human review, but the signature must bind to exact canonical bytes.

Evidence payload vs signed payload boundary:

- signed payload should be compact, canonical, and fixed
- evidence payload may include additional verification / display metadata
- evidence payload must not change the signed monetary meaning
- X1 verifier must not trust unsigned evidence fields to override signed fields

Route rule binding:

The immutable Stage 1 route rule is:

sourceChainWeightBps = 10000
xxxlMintAmount = burnedAmount

The canonical message should include both values.

The X1 mint core should also independently verify them from immutable route rules.

This double-binding is intentional:

- the signed message states the expected value
- the mint core checks that the value matches the route rule

Processed burn registry direction:

X1 processed burn registry should key by canonicalEventKey.

The registry should not key by:

- sourceNonce alone
- sourceSender alone
- x1Recipient alone
- burnedAmount alone
- transaction hash alone without log index

Invalid encoding examples documented:

- JSON object where field order matters implicitly
- JSON object where numeric amounts are decimal strings
- hex strings without length normalization
- mixed-case address strings treated as canonical bytes
- string concatenation
- optional field omission
- sourceNonce replacing transaction hash + log index
- guardians choosing xxxlMintAmount manually
- route weight omitted from signed payload
- mintToken omitted from signed payload
- missing domain separation
- payloads reusable across testnet and mainnet
- payloads reusable across different mint cores

Test vector requirement before implementation:

No implementation should begin until at least one complete example message can be encoded and hashed identically by independent code.

Required future test vectors include:

- domain constants
- x1RecipientHash example
- canonicalEventKey example
- full message encoded bytes example
- messageHash example
- invalid field order example
- invalid optional omission example
- invalid amount string example
- invalid wrong route id example
- invalid wrong mint token example

Implementation should still not begin until encoding, hash function, signature standard, finality rule, and X1 recipient type are reviewed.

## Latest Stage 1 gateway test vectors checkpoint

The Stage 1 gateway test vectors milestone was completed on the stage-1-gateway-test-vectors branch.

Commits:

- 6aac88f Define Stage 1 gateway test vectors
- 69e2df0 Merge branch 'stage-1-gateway-test-vectors'

This milestone adds the test vector requirements layer for the XNTD-to-XXXL Gateway.

Design document added:

- docs/gateway/stage-1-gateway-test-vectors.md

Purpose:

Define the future valid and invalid vector set that should exist before implementation, so independent implementations agree on source event normalization, canonicalEventKey derivation, x1RecipientHash handling, message field order, messageHash preimage, route rule validation, replay handling, and invalid encoding rejection.

This is a design-only milestone.

It does not implement:

- Ethereum contracts
- X1 programs
- XXXL token runtime
- X1 mint core
- processed burn registry
- guardian runtime
- relayer runtime
- watcher runtime
- frontend gateway flow
- real RPC reads
- env reads
- private keys
- API keys
- mnemonic handling
- deployment logic

The document intentionally does not provide final cryptographic hashes yet.

Final numeric hashes and signatures should be added only after:

- target X1 hash function is selected
- target X1 address / recipient type is selected
- guardian signature standard is selected
- canonical binary encoding is finalized
- domain separator is finalized
- target mint core identity format is finalized

Core vector principle:

A valid test vector must allow independent implementations to derive the same result from the same input.

If two implementations produce different canonical bytes or hashes for the same vector, the design is not ready for implementation.

If an invalid vector can be accepted by any conforming implementation, the design is not ready for implementation.

Required vector categories documented:

- valid source burn event normalization
- valid canonicalEventKey derivation
- valid x1RecipientHash derivation
- valid domain constants
- valid domain separator
- valid gateway message field order
- valid messageHash preimage
- valid full mint approval message
- invalid wrong source chain
- invalid wrong source token
- invalid zero burned amount
- invalid empty X1 recipient
- invalid recipient hash mismatch
- invalid sourceChainWeightBps
- invalid xxxlMintAmount
- invalid canonicalEventKey
- invalid optional field omission
- invalid field order
- invalid string amount encoding
- invalid JSON-dependent encoding
- invalid replay / duplicate canonicalEventKey
- invalid cross-domain signature reuse

Placeholder notation documented:

- HASH(value)
- BYTES32(value)
- ADDRESS20(value)
- UINT(value)
- UINT256(value)
- ENCODE(fields...)
- DOMAIN_SEPARATOR(fields...)
- MESSAGE_HASH(domainSeparator, encodedMessage)
- SIGNATURE(messageHash)

These placeholders are not implementation syntax.

They describe what final vectors must later replace with exact bytes, hashes, and signatures.

The document defines the expected valid vector structure:

- source event
- normalized fields
- canonicalEventKey preimage
- canonicalEventKey
- x1RecipientHash preimage
- x1RecipientHash
- domain constants
- domain separator
- message fields
- encoded message bytes
- messageHash preimage
- messageHash
- guardian signatures
- expected X1 verification result
- expected processed key
- expected mint recipient
- expected mint amount

The valid vector must preserve the Stage 1 semantic boundary:

- sourceChainId = Ethereum mainnet
- sourceChainWeightBps = 10000
- xxxlMintAmount = burnedAmount
- Stage 1 full-weight conversion does not mean XXXL is Ethereum XNTD
- Stage 1 full-weight conversion does not create a price peg
- Stage 1 Gateway is not a wrapped bridge

Invalid vector cases documented:

- wrong source chain
- wrong source token
- zero burned amount
- empty recipient
- recipient hash mismatch
- wrong route weight
- wrong mint amount
- wrong canonicalEventKey
- optional field omission
- wrong field order
- amount encoded as string
- JSON-dependent encoding
- duplicate canonicalEventKey
- cross-domain replay

Future fixture file layout suggested:

- fixtures/gateway/stage-1/

Potential future fixture files documented:

- valid-ethereum-xntd-burn-to-xxxl.json
- invalid-wrong-source-chain.json
- invalid-wrong-source-token.json
- invalid-zero-burned-amount.json
- invalid-empty-recipient.json
- invalid-recipient-hash-mismatch.json
- invalid-wrong-route-weight.json
- invalid-wrong-mint-amount.json
- invalid-wrong-canonical-event-key.json
- invalid-optional-field-omission.json
- invalid-wrong-field-order.json
- invalid-string-amount.json
- invalid-json-canonicalization.json
- invalid-duplicate-canonical-event-key.json
- invalid-cross-domain-replay.json

These files should be added only when final encoding and hash choices are made.

Production readiness implication:

Production implementation should not begin until final hash function, signature standard, X1 recipient type, canonical binary encoding, domain separator, and exact test vectors are reviewed.

Current preferred direction:

- document placeholder vectors now
- add exact cryptographic vectors later
- never use live user data in vectors
- never include real secrets
- require independent implementations to match exact bytes and hashes
- require invalid vectors to fail deterministically
- use vectors as the bridge between design and implementation

Implementation should still not begin until final encoding, hash function, signature standard, finality rule, X1 recipient type, and exact test vectors are reviewed.

## Latest Stage 1 gateway Theo review notes checkpoint

The Stage 1 gateway Theo review notes milestone was completed on the stage-1-gateway-theo-review-notes branch.

Commits:

- 99d2014 Add Stage 1 gateway Theo review notes
- 1d5447e Merge branch 'stage-1-gateway-theo-review-notes'

This milestone records Theo's review of the Stage 1 XNTD-to-XXXL Gateway design chain.

Design document added:

- docs/gateway/stage-1-gateway-theo-review-notes.md

Reviewed design chain:

- docs/gateway/stage-1-ethereum-burn-event-schema.md
- docs/gateway/stage-1-gateway-message-schema.md
- docs/gateway/stage-1-gateway-canonical-encoding.md
- docs/gateway/stage-1-gateway-test-vectors.md
- docs/checkpoints/current-design-checkpoint.md

Theo's high-level conclusion:

Stage 1 design is architecturally mature.

The main structural decisions are correct.

Confirmed strengths:

- immutable mint core as a structural guard against guardian overreach
- canonical encoding with double-binding
- route rules included in signed message and independently checked by mint core
- gateway framing as burn-to-mint, not a standard wrapped bridge
- domainSeparator to prevent cross-environment signature reuse
- keeping sourceNonce outside the replay key

Confirmed architecture boundary:

- guardians verify burn evidence and sign deterministic messages
- immutable mint core owns route rules and conversion rules
- relayer transports approvals but cannot define monetary values
- guardians must not control XXXL monetary policy

Confirmed replay anchor:

canonicalEventKey = hash(sourceChainId, sourceToken, sourceBurnTxHash, sourceBurnEventIndex)

Theo confirmed that transaction hash plus log index is the right replay anchor for the Ethereum Stage 1 route.

Confirmed sourceNonce role:

- sourceNonce is useful for indexing, user display, and event ordering visibility
- sourceNonce should not be the primary replay key
- replay anchor should remain the exact emitted Ethereum log

Confirmed x1Recipient direction:

- x1RecipientHash inside signed payload
- raw x1Recipient bytes in execution / evidence payload
- X1 verifier checks hash(rawRecipientBytes) == signed x1RecipientHash

Theo warning:

Recipient encoding must be normalized before implementation.

If X1 accepts multiple byte encodings for the same recipient, this can create recipient malleability.

Confirmed double-binding route rule model:

- signed message includes sourceChainWeightBps = 10000
- signed message includes xxxlMintAmount = burnedAmount
- immutable mint core independently verifies Stage 1 route rules
- guardian message values must not override immutable route rules

Key remaining risk:

messageHash encoding must be finalized before implementation.

If domainSeparator or preimage encoding remains ambiguous, independent implementations may derive different messageHash values from the same burn event.

Test vectors are mandatory before implementation.

Theo recommended making sourceBlockHash mandatory in the signed message.

sourceBlockNumber and sourceBlockHash should be treated as required signed fields, not optional fields.

Reason:

- guardians should sign only finalized canonical Ethereum evidence
- signed message should bind to the observed canonical block
- reorged-out evidence must not remain silently valid
- production design must make finality assumptions explicit

Theo identified X1 mint core immutability as a blocker before implementation.

Open questions:

- how is the X1 mint core deployed?
- can route rules be changed after deployment?
- can mint authority be upgraded?
- can deployer key update the program / contract?
- is deployer authority removed?
- is there a timelock or governance path?
- if governance exists, which parts are mutable and which are not?

Theo identified atomic processed-burn check-and-mint as a blocker before implementation.

The X1 mint path must atomically:

1. verify the message and signatures
2. check canonicalEventKey is unprocessed
3. mark canonicalEventKey as processed
4. mint XXXL

The processed registry must not allow a race where two executions pass the unprocessed check before either marks the key.

Theo identified zero / burn recipient policy as an open question.

If a user provides a null or burn recipient:

- Ethereum XNTD is burned
- XXXL may be minted to an unusable recipient
- the user may permanently lose the X1-side mint result

Open question:

Should Stage 1 reject zero / burn recipients?

Theo identified burn amount min/max as an open policy question.

The current design rejects zero burned amount.

Open question:

Should Stage 1 define minimum or maximum burned amount?

Pre-implementation blockers from Theo review:

1. final hash function choice
2. final signature standard
3. final X1 recipient type
4. sourceBlockHash and sourceBlockNumber as mandatory signed fields
5. X1 mint core immutability mechanism
6. atomic processed-burn check-and-mint model
7. exact test vectors after hash / signature / recipient choices

Production blockers from Theo review:

1. finality rule
2. recipient normalization
3. zero / burn recipient policy
4. burn amount min/max policy
5. independent implementation agreement on exact test vectors

Current conclusion:

Stage 1 Gateway design is ready to move from broad architecture into pre-implementation blocker resolution.

Implementation should still not begin yet.

The next recommended design document is:

- docs/gateway/stage-1-gateway-pre-implementation-blockers.md

## Latest Stage 1 gateway pre-implementation blockers checkpoint

The Stage 1 gateway pre-implementation blockers milestone was completed on the stage-1-gateway-pre-implementation-blockers branch.

Commits:

- 7b9b76d Define Stage 1 gateway pre-implementation blockers
- 72a70b6 Merge branch 'stage-1-gateway-pre-implementation-blockers'

This milestone converts Theo's Stage 1 Gateway review into an explicit pre-implementation blocker gate.

Design document added:

- docs/gateway/stage-1-gateway-pre-implementation-blockers.md

Purpose:

Define the remaining decisions that must be resolved before Stage 1 Gateway implementation begins.

This is a design / readiness milestone only.

It does not implement:

- Ethereum contracts
- X1 programs
- XXXL token runtime
- X1 mint core
- processed burn registry
- guardian runtime
- relayer runtime
- watcher runtime
- frontend gateway flow
- real RPC reads
- env reads
- private keys
- API keys
- mnemonic handling
- deployment logic

The Stage 1 route remains:

Ethereum XNTD burn -> immutable X1 XXXL mint core -> XXXL mint

The core boundary remains:

- gateway guardians = verification layer
- immutable mint core / route rules = monetary conversion rules
- relayer = execution / transport layer without discretion

Guardians must not control XXXL monetary policy.

Pre-implementation blockers documented:

1. final hash function choice
2. final signature standard
3. final X1 recipient type and normalization
4. sourceBlockNumber and sourceBlockHash as mandatory signed fields
5. X1 mint core immutability mechanism
6. atomic processed-burn check-and-mint
7. finality rule
8. zero / burn recipient policy
9. burn amount min/max policy
10. exact cryptographic test vectors

Blocker 1: final hash function choice.

The final hash function must be selected for:

- canonicalEventKey
- x1RecipientHash
- domain constants
- domainSeparator
- messageHash

Blocker 2: final signature standard.

The guardian signature standard must define:

- signature algorithm
- public key / signer identity format
- signature byte format
- guardian set representation
- threshold rule
- signature ordering / deduplication rules
- malleability rejection rules
- verification behavior on X1

Blocker 3: final X1 recipient type and normalization.

The design must define:

- exact X1 recipient type
- exact recipient byte encoding
- exact normalization rule
- exact invalid recipient cases
- exact zero / burn recipient policy
- exact hash preimage for x1RecipientHash

Blocker 4: sourceBlockNumber and sourceBlockHash as mandatory signed fields.

Decision accepted from Theo review:

Stage 1 signed message should treat sourceBlockNumber and sourceBlockHash as required fields.

Required updates before implementation:

- message schema must clearly mark sourceBlockNumber and sourceBlockHash as mandatory
- canonical encoding must include both in fixed field order
- test vectors must include both
- guardian acceptance rules must reject missing block number or block hash
- finality rule must reference these fields

Blocker 5: X1 mint core immutability mechanism.

The design must define:

- how X1 mint core is deployed
- whether code can be upgraded
- whether route rules can be changed
- whether mint authority can be changed
- whether deployer authority exists after deployment
- how deployer authority is removed or disabled
- whether any governance / timelock path exists
- which parameters are immutable forever
- which parameters, if any, are operationally configurable

Stage 1 immutable route rules include:

- source chain is Ethereum mainnet
- source token is expected Ethereum XNTD token
- sourceChainWeightBps = 10000
- xxxlMintAmount = burnedAmount
- mint token is XXXL
- replay key is canonicalEventKey

Blocker 6: atomic processed-burn check-and-mint.

The X1 mint path must atomically:

1. verify message and signatures
2. check canonicalEventKey is unprocessed
3. mark canonicalEventKey as processed
4. mint XXXL

The processed registry must not allow duplicate minting for the same canonicalEventKey.

Blocker 7: finality rule.

The design must define:

- finality model
- minimum confirmation rule, if used
- whether finalized block tag is used
- whether multiple providers are required
- behavior during reorgs
- behavior if providers disagree
- handling of sourceBlockHash mismatch
- handling of source burn event disappearing after reorg

Blocker 8: zero / burn recipient policy.

The design must decide whether Stage 1 rejects:

- empty recipients
- zero recipients
- known burn recipients
- malformed recipients

Blocker 9: burn amount min/max policy.

The design must decide:

- zero amount rejection
- optional minimum burn amount
- optional maximum burn amount
- where amount rules are enforced
- whether min/max is immutable route policy

Preferred direction:

Reject zero amount.

Do not add arbitrary min/max unless there is a clear security, UX, or spam-control reason.

Blocker 10: exact cryptographic test vectors.

Exact vectors must be created after finalizing:

- hash function
- signature standard
- X1 recipient type
- canonical binary encoding
- domain separator
- target mint core identity format

Implementation gate:

Implementation is blocked until all blockers are resolved, documented, and reviewed.

Current conclusion:

Stage 1 Gateway design is strong enough to move into pre-implementation decision resolution.

It is not ready for code yet.

The next recommended document is a decision document for hash function, signature standard, and X1 recipient type.

## Latest Stage 1 gateway hash, signature, and recipient decisions checkpoint

The Stage 1 gateway hash, signature, and recipient decisions milestone was completed on the stage-1-gateway-hash-signature-recipient-decisions branch.

Commits:

- ca4b787 Define Stage 1 gateway hash signature recipient decisions
- 89b8357 Merge branch 'stage-1-gateway-hash-signature-recipient-decisions'

This milestone resolves the first three Stage 1 Gateway pre-implementation blockers:

1. final hash function choice
2. final guardian signature standard
3. final X1 recipient type and normalization

Design document added:

- docs/gateway/stage-1-gateway-hash-signature-recipient-decisions.md

Purpose:

Define the Stage 1 Gateway decision for hash function, guardian signature standard, and X1 recipient canonical encoding.

This is a design decision milestone only.

It does not implement:

- Ethereum contracts
- X1 programs
- XXXL token runtime
- X1 mint core
- processed burn registry
- guardian runtime
- relayer runtime
- watcher runtime
- frontend gateway flow
- real RPC reads
- env reads
- private keys
- API keys
- mnemonic handling
- deployment logic

Decision context:

X1 is SVM-compatible.

Stage 1 verification choices should respect both environments:

- Ethereum is the source chain and uses keccak256 naturally
- X1 is the execution / mint environment and uses SVM-native account and signature assumptions

The goal is not to emulate EVM on X1.

The goal is to use each environment where it is strongest.

Final Stage 1 decisions:

- hash function = keccak256
- guardian signature standard = Ed25519
- X1 recipient type = 32 raw bytes X1 / SVM public key
- x1RecipientHash = keccak256(x1RecipientBytes)
- base58 is display-only and not canonical protocol encoding
- 32 zero bytes recipient must be rejected

Hash decision:

Stage 1 gateway commitments use keccak256 for:

- canonicalEventKey
- x1RecipientHash
- domain constants
- domainSeparator
- messageHash

Rationale:

- Stage 1 source chain is Ethereum
- Ethereum burn evidence and tooling naturally use keccak256
- canonicalEventKey is derived from Ethereum burn event identity
- viem / ethers / Ethereum indexers can generate vectors easily
- message payloads are small enough for practical SVM keccak verification
- using one hash model avoids mixed-hash complexity

SHA-256 is not selected for Stage 1 gateway commitments.

Reason:

- it is cheaper on SVM, but less natural for Ethereum-source event commitments
- it would create a mixed Ethereum/SVM hash model
- test vectors and tooling are simpler if one hash function is used
- Stage 1 payload sizes are small enough that keccak256 cost is acceptable

Guardian signature decision:

Stage 1 guardians use Ed25519 signatures.

Rationale:

- X1 is SVM-compatible
- Ed25519 is native to SVM
- Ed25519 verification is cheaper and simpler than secp256k1 recovery on SVM
- guardians are infrastructure operators, not ordinary Ethereum users
- guardians can use fresh X1-native keys
- there is no hard requirement to reuse existing EVM guardian keys
- this avoids unnecessary EVM emulation in the X1 mint path

secp256k1 is not selected as the Stage 1 default.

It remains a documented fallback only if reusing existing EVM guardian keys becomes a hard requirement.

Recipient decision:

Stage 1 X1 recipient is a 32-byte raw X1 / SVM public key.

Canonical protocol encoding:

- exactly 32 raw bytes
- no base58 in signed payload
- no display string in signed payload
- no variable-length recipient bytes
- no checksum/casing rules
- no alternate encodings for the same recipient

Display encoding:

- base58 may be used in UI
- base58 may be used in logs or human-readable views
- base58 is not canonical protocol encoding
- base58 must be decoded to exactly 32 bytes before hashing or verification

Recipient rejection rules:

Stage 1 must reject:

- empty recipient
- recipient length not equal to 32 bytes
- malformed recipient bytes
- x1RecipientHash mismatch
- 32 zero bytes recipient

Message schema implications:

- x1RecipientHash is mandatory
- sourceBlockNumber is mandatory
- sourceBlockHash is mandatory
- canonicalEventKey uses keccak256
- x1RecipientHash uses keccak256
- domainSeparator uses keccak256
- messageHash uses keccak256
- guardian signatures are Ed25519 signatures over messageHash
- x1RecipientBytes are supplied as execution / evidence payload

Test vector implications:

Exact test vectors must include:

- keccak256 domain constants
- keccak256 x1RecipientHash for a 32-byte dummy recipient
- keccak256 canonicalEventKey
- keccak256 domainSeparator
- full encoded message bytes
- keccak256 messageHash
- Ed25519 test guardian public key
- Ed25519 signature over messageHash
- valid verification case
- invalid wrong hash case
- invalid wrong signature case
- invalid recipient length case
- invalid all-zero recipient case
- invalid base58-as-canonical case

Resolved blockers:

- final hash function choice
- final signature standard
- final X1 recipient type and normalization

Remaining blockers:

1. sourceBlockNumber and sourceBlockHash mandatory field updates
2. X1 mint core immutability mechanism
3. atomic processed-burn check-and-mint model
4. finality rule
5. zero / burn recipient policy beyond 32 zero bytes
6. burn amount min/max policy
7. exact cryptographic test vectors

Current conclusion:

Stage 1 adopts:

keccak256 + Ed25519 + 32-byte X1 / SVM recipient

This is the preferred balance between Ethereum-native source evidence and X1/SVM-native execution verification.

Implementation should still not begin until the remaining blockers are resolved and exact test vectors are produced.

## Latest Stage 1 gateway mandatory source block fields checkpoint

The Stage 1 gateway mandatory source block fields milestone was completed on the stage-1-gateway-mandatory-source-block-fields branch.

Commits:

- 70c456a Define Stage 1 gateway mandatory source block fields
- 2769087 Merge branch 'stage-1-gateway-mandatory-source-block-fields'

This milestone closes the Stage 1 Gateway pre-implementation blocker for mandatory source block fields.

Design document added:

- docs/gateway/stage-1-gateway-mandatory-source-block-fields.md

Purpose:

Define that sourceBlockNumber and sourceBlockHash are mandatory signed fields for Stage 1 Gateway messages.

This is a design decision milestone only.

It does not implement:

- Ethereum contracts
- X1 programs
- XXXL token runtime
- X1 mint core
- processed burn registry
- guardian runtime
- relayer runtime
- watcher runtime
- frontend gateway flow
- real RPC reads
- env reads
- private keys
- API keys
- mnemonic handling
- deployment logic

Decision:

sourceBlockNumber and sourceBlockHash are mandatory signed fields for Stage 1.

They must be included in:

- gateway message schema
- canonical field order
- canonical encoded message
- messageHash preimage
- guardian signed payload
- test vectors
- guardian acceptance rules
- finality rule design

Reason:

Guardians must sign only finalized canonical Ethereum burn evidence.

The signed message should bind to the observed canonical Ethereum block.

Mandatory source block fields help prevent ambiguity around:

- reorged-out events
- non-finalized evidence
- provider disagreement
- stale event observations
- evidence replay with incomplete source context

Mandatory fields:

- sourceBlockNumber
- sourceBlockHash

Field meaning:

- sourceBlockNumber is the Ethereum block number containing the accepted burn event
- sourceBlockHash is the Ethereum block hash containing the accepted burn event

Both values must come from the same canonical finalized Ethereum block that contains the accepted burn event.

Replay protection remains based on:

canonicalEventKey = keccak256(ENCODE(sourceChainId, sourceToken, sourceBurnTxHash, sourceBurnEventIndex))

sourceBlockNumber and sourceBlockHash are not the primary replay key.

Their role is to bind the signed message to the finalized source block context that guardians accepted.

Guardian rejection rules added:

Guardians must reject evidence if:

- sourceBlockNumber is missing
- sourceBlockHash is missing
- sourceBlockHash is not a 32-byte Ethereum block hash
- sourceBlockNumber does not match the block containing the burn event
- sourceBlockHash does not match the block containing the burn event
- the block is not canonical according to the chosen finality rule
- the block is not finalized enough according to the chosen finality rule
- the burn event is not present in that block
- the transaction receipt block hash differs from sourceBlockHash
- providers disagree and the finality policy cannot resolve the disagreement

Message schema implication:

sourceBlockNumber and sourceBlockHash are required fields.

They are not optional.

They are not display-only metadata.

They are part of the signed message.

Canonical encoding implication:

The existing canonical field order already includes sourceBlockNumber and sourceBlockHash.

No field order change is needed.

The decision is that fields 9 and 10 are mandatory and cannot be omitted or zero-filled as unused optional fields.

Invalid cases documented:

- sourceBlockNumber omitted
- sourceBlockHash omitted
- sourceBlockNumber encoded as a decimal string
- sourceBlockHash encoded as a hex display string instead of bytes
- sourceBlockHash with wrong length
- sourceBlockHash not matching transaction receipt block hash
- sourceBlockNumber not matching transaction receipt block number
- sourceBlockHash from a reorged-out block
- sourceBlockHash from a non-canonical block
- sourceBlockNumber and sourceBlockHash from different blocks

Test vector implications:

Exact test vectors must include:

- sourceBlockNumber
- sourceBlockHash
- sourceBurnTxHash
- sourceBurnEventIndex
- canonicalEventKey
- full encoded message bytes
- messageHash

Invalid vectors must include:

- missing sourceBlockNumber
- missing sourceBlockHash
- wrong sourceBlockHash
- wrong sourceBlockNumber
- sourceBlockHash wrong length
- sourceBlockHash from different block
- reorged-out sourceBlockHash scenario note

Finality rule dependency:

This milestone does not define the finality rule.

It prepares for the finality rule by making the signed message bind to block identity.

Current conclusion:

Stage 1 requires sourceBlockNumber and sourceBlockHash as mandatory signed fields.

This closes the mandatory source block field blocker.

Implementation should still not begin until the remaining blockers are resolved and exact test vectors are produced.

## Latest Stage 1 X1 mint core immutability checkpoint

The Stage 1 X1 mint core immutability milestone was completed on the stage-1-x1-mint-core-immutability branch.

Commits:

- 6f587e5 Define Stage 1 X1 mint core immutability
- 9b4d54b Merge branch 'stage-1-x1-mint-core-immutability'

This milestone defines the Stage 1 immutability requirements for the X1 XXXL mint core.

Design document added:

- docs/gateway/stage-1-x1-mint-core-immutability.md

Purpose:

Define the immutability requirements that keep Stage 1 monetary conversion rules outside guardian, relayer, deployer, administrator, or mutable governance control.

This is a design / readiness milestone only.

It does not implement:

- Ethereum contracts
- X1 programs
- XXXL token runtime
- X1 mint core
- processed burn registry
- guardian runtime
- relayer runtime
- watcher runtime
- frontend gateway flow
- real RPC reads
- env reads
- private keys
- API keys
- mnemonic handling
- deployment logic

Core boundary:

- gateway guardians = verification layer
- immutable mint core / route rules = monetary conversion rules
- relayer = execution / transport layer without discretion

Core principle:

The X1 mint core must enforce monetary conversion rules independently from guardian signatures.

Guardian signatures prove that source burn evidence was verified.

Guardian signatures must not define monetary policy.

The mint core must reject any message that violates immutable Stage 1 route rules, even if guardians signed it.

Immutable Stage 1 route rules documented:

- source chain is Ethereum mainnet
- source token is the expected Ethereum XNTD token
- routeId is the Stage 1 Ethereum XNTD to X1 XXXL route
- sourceChainWeightBps = 10000
- xxxlMintAmount = burnedAmount
- mint token is XXXL
- canonicalEventKey is derived from sourceChainId, sourceToken, sourceBurnTxHash, and sourceBurnEventIndex
- hash function is keccak256
- guardian signature standard is Ed25519
- X1 recipient is 32 raw bytes
- x1RecipientHash = keccak256(x1RecipientBytes)
- sourceBlockNumber and sourceBlockHash are mandatory signed fields

Mint core verification requirements documented:

Before minting XXXL, the X1 mint core must verify:

- messageType is expected
- schemaVersion is supported
- routeId is expected Stage 1 route
- sourceChainId is Ethereum mainnet
- sourceToken is expected Ethereum XNTD token
- sourceChainWeightBps equals 10000
- xxxlMintAmount equals burnedAmount
- mintToken is XXXL
- burnedAmount is greater than zero
- x1RecipientBytes length is exactly 32 bytes
- x1RecipientBytes is not 32 zero bytes
- keccak256(x1RecipientBytes) equals x1RecipientHash
- sourceBlockNumber is present
- sourceBlockHash is present and exactly 32 bytes
- canonicalEventKey is derived correctly
- guardian signatures satisfy the required Ed25519 threshold
- canonicalEventKey has not been processed before

Guardian overreach rejection documented:

The mint core must reject guardian-approved messages if any immutable route value is wrong.

Guardian threshold is necessary but not sufficient.

A valid guardian threshold cannot override route rules.

Relayer limitation documented:

The relayer may transport signed messages, signatures, raw x1RecipientBytes, evidence references, and execution metadata.

The relayer must not be able to change monetary values or route identity.

Deployment authority requirement documented:

Route rules and monetary conversion logic must not remain controllable by a deployer, admin, guardian set, relayer, or mutable governance path after production deployment.

Before implementation or production approval, the project must define:

- whether X1 programs / contracts are upgradeable
- whether deployer authority exists
- how deployer authority is removed, disabled, or constrained
- whether mint authority exists separately from program authority
- whether mint authority can be changed
- whether any governance / timelock path exists
- which values are immutable forever
- which values, if any, can be operationally configured
- how users can verify the deployed immutability state

Acceptable production outcomes may include:

- non-upgradeable mint core
- upgrade authority permanently removed
- route rules hardcoded in deployed code
- mint authority constrained to the immutable mint core
- mutable operational settings separated from monetary route rules
- public verification procedure showing no mutable authority can alter route rules

Unacceptable production outcomes documented:

- guardians can change route weight
- guardians can change mint formula
- relayer can choose mint amount
- deployer can upgrade route rules after launch
- admin can change source token
- admin can change source chain
- admin can change mint token
- governance can change monetary conversion without a new explicit route / deployment
- mint authority can mint XXXL outside verified gateway messages
- processed burn registry can be bypassed by privileged authority

Operational configuration boundary documented:

Some operational configuration may be acceptable only if it does not affect monetary conversion.

Guardian set mutability distinction documented:

Guardian set management is not the same as monetary policy.

Guardian rotation may be acceptable only if it cannot alter route rules or mint without valid burn evidence.

User verification requirement documented:

Before production, users should be able to verify deployed mint core identity, immutable route parameters, authority state, and whether any admin can alter route rules.

Current conclusion:

Stage 1 requires an immutable X1 mint core whose route rules and monetary conversion logic cannot be changed by guardians, relayers, deployers, administrators, or mutable governance after production deployment.

This closes the immutability requirement-definition blocker.

Implementation should still not begin until the exact X1 deployment and authority model is documented and exact test vectors are produced.

## Latest Stage 1 processed burn atomicity checkpoint

The Stage 1 processed burn atomicity milestone was completed on the stage-1-processed-burn-atomicity branch.

Commits:

- c5e56a8 Define Stage 1 processed burn atomicity
- 51ef8be Merge branch 'stage-1-processed-burn-atomicity'

This milestone defines the Stage 1 processed-burn registry and atomic check-and-mint requirements for the XNTD-to-XXXL Gateway.

Design document added:

- docs/gateway/stage-1-processed-burn-atomicity.md

Purpose:

Prevent one Ethereum XNTD burn event from minting XXXL more than once on X1.

This is a design / readiness milestone only.

It does not implement:

- Ethereum contracts
- X1 programs
- XXXL token runtime
- X1 mint core
- processed burn registry
- guardian runtime
- relayer runtime
- watcher runtime
- frontend gateway flow
- real RPC reads
- env reads
- private keys
- API keys
- mnemonic handling
- deployment logic

Core rule:

One canonicalEventKey can produce at most one successful XXXL mint.

Atomic sequence requirement:

The X1 mint path must atomically:

1. verify the message and guardian signatures
2. check canonicalEventKey is unprocessed
3. mark canonicalEventKey as processed
4. mint XXXL to x1RecipientBytes

These steps must be atomic or protected by X1 runtime guarantees.

Processed registry key:

The processed-burn registry must key by canonicalEventKey.

The registry must not key by:

- sourceNonce alone
- sourceSender alone
- x1RecipientHash alone
- burnedAmount alone
- sourceBurnTxHash alone without sourceBurnEventIndex
- sourceBlockHash alone
- guardian messageHash alone
- relayer transaction id

The replay anchor remains the exact Ethereum log identity.

Required processed record direction:

A successful processed-burn record should include enough data for verification, indexing, and audit.

Recommended processed record fields include:

- canonicalEventKey
- sourceChainId
- sourceToken
- sourceSender
- sourceBurnTxHash
- sourceBurnEventIndex
- sourceBlockNumber
- sourceBlockHash
- sourceNonce
- x1RecipientHash
- x1RecipientBytes
- burnedAmount
- sourceChainWeightBps
- xxxlMintAmount
- mintToken
- messageHash
- guardianSetId or signerSetVersion if used
- processedAtSlot or processedAtBlock
- x1MintTxId or execution id if available

Check-before-mark anti-pattern rejected:

The design rejects non-atomic check-then-later-mark flow because two relayers could pass the unprocessed check before either marks the key.

Mark-before-mint risk documented:

If mint fails, the processed mark must also fail / roll back.

There must be no stuck processed record without the corresponding mint.

Duplicate submission behavior documented:

- first valid execution may mint
- every later execution for the same canonicalEventKey must fail or return already processed without minting
- duplicate rejection must not alter mint amount
- duplicate rejection must not alter recipient
- duplicate rejection must not overwrite processed record
- duplicate rejection must not require guardian intervention

Relayer race behavior documented:

If two relayers submit the same approval at nearly the same time:

- at most one succeeds
- at most one mint occurs
- processed registry ends in exactly one processed state
- losing submission receives duplicate / already processed result
- no partial mint is possible
- no double mint is possible

Failure behavior documented:

If verification, route rule validation, guardian threshold, or recipient validation fails before processed mark:

- no processed record is written
- no mint occurs

If mint fails:

- no processed record remains unless the mint also succeeded
- failed execution must be retryable after the issue is corrected, if correction is possible
- no burned Ethereum event should be permanently blocked by a failed X1-side partial state update

No privileged bypass requirement documented:

There must be no privileged path that can:

- mark arbitrary canonicalEventKey as processed without verification
- unmark processed burns
- overwrite processed records
- mint XXXL outside verified gateway messages
- mint again for an already processed canonicalEventKey
- bypass guardian threshold
- bypass immutable route rules

Processed record immutability documented:

After a canonicalEventKey is processed, its record should be immutable.

A processed record must not be overwritten to change recipient, amount, source transaction, source event index, source block identity, route rule values, messageHash, or mint token.

Event / log requirements documented:

The X1 mint core should emit or record a successful mint event containing enough fields for frontend display, watcher reconciliation, audit, incident investigation, and user support.

Test vector implications:

Future exact test vectors and tests must include:

- valid first mint for canonicalEventKey
- duplicate submission rejected
- duplicate relayer race scenario note
- wrong canonicalEventKey rejected
- already processed canonicalEventKey rejected
- failed signature does not mark processed
- failed route rule does not mark processed
- failed recipient validation does not mark processed
- failed mint rolls back processed mark
- processed record cannot be overwritten
- privileged bypass impossible or not present

Current conclusion:

Stage 1 requires an atomic processed-burn check-and-mint model.

The processed registry must key by canonicalEventKey.

For each canonicalEventKey, at most one successful XXXL mint may occur.

This closes the atomic processed-burn requirement-definition blocker.

Implementation should still not begin until the exact X1 runtime atomicity model, finality rule, deployment authority model, and exact test vectors are documented.

## Latest Stage 1 Ethereum finality rule checkpoint

The Stage 1 Ethereum finality rule milestone was completed on the stage-1-ethereum-finality-rule branch.

Commits:

- ee3a7ce Define Stage 1 Ethereum finality rule
- 6b42fbe Merge branch 'stage-1-ethereum-finality-rule'

This milestone defines the Stage 1 Ethereum finality rule for the XNTD-to-XXXL Gateway.

Design document added:

- docs/gateway/stage-1-ethereum-finality-rule.md

Purpose:

Define when Ethereum-side XNTD burn evidence is canonical and finalized enough for guardians to approve X1 XXXL minting.

This is a design / readiness milestone only.

It does not implement:

- Ethereum contracts
- X1 programs
- XXXL token runtime
- X1 mint core
- processed burn registry
- guardian runtime
- relayer runtime
- watcher runtime
- frontend gateway flow
- real RPC reads
- env reads
- private keys
- API keys
- mnemonic handling
- deployment logic

Core rule:

Guardians must not sign burn evidence until the source Ethereum block is finalized enough under the Stage 1 finality policy.

Finality objective:

The finality rule protects against:

- signing reorged-out burn events
- signing non-canonical burn events
- signing events before sufficient Ethereum finality
- provider disagreement
- stale receipt data
- sourceBlockHash mismatch
- transaction hash / log index evidence without stable block context

Required source block binding:

Every signed Stage 1 message must include:

- sourceBlockNumber
- sourceBlockHash

Guardian verification must confirm:

- transaction receipt exists
- transaction succeeded
- receipt block number equals sourceBlockNumber
- receipt block hash equals sourceBlockHash
- burn log exists in that receipt
- burn log index equals sourceBurnEventIndex
- block is canonical
- block satisfies Stage 1 finality rule

Preferred finality direction:

Use Ethereum finalized block status when reliable RPC support exists.

A guardian may accept a burn event as finalized if:

- the burn receipt block is at or before the latest finalized Ethereum block
- sourceBlockHash matches the canonical block hash at sourceBlockNumber
- the burn transaction succeeded
- the expected event exists in the receipt
- all Stage 1 guardian acceptance rules pass

Conservative fallback direction:

If finalized block tag support is unavailable, inconsistent, or unreliable, guardians may use a conservative confirmation-depth fallback.

Fallback requirement:

- source block must be at least N confirmations deep
- N must be chosen conservatively before implementation
- provider responses must agree on sourceBlockHash
- provider responses must agree that the source block is canonical
- provider responses must agree that the transaction receipt is in that block

The exact N is not fixed by this document.

Production implementation must choose and document the exact confirmation depth if fallback mode is used.

Provider agreement direction:

Guardians should use at least two independent Ethereum RPC providers for finality-critical checks.

Guardians should reject or delay evidence if providers disagree about:

- source block identity
- receipt inclusion
- receipt block hash
- finalized status
- fallback confirmation status

Reorg handling:

Guardians must reject evidence if:

- sourceBlockHash is no longer canonical
- transaction receipt is no longer found
- receipt blockHash differs from signed sourceBlockHash
- burn log is missing from the canonical receipt
- sourceBurnEventIndex no longer identifies the expected burn event
- source block was reorged out before approval

Relayer responsibility:

The relayer should not attempt to submit an approval for evidence that is known to be non-final, reorged, rejected, or disputed.

However, finality verification is primarily guardian responsibility.

For Stage 1, finality is enforced by guardian verification policy and signed message discipline.

Watcher / indexer states documented:

- observed
- confirmed
- waiting for finality
- finalized
- guardian approval pending
- guardian approved
- relayer submitted
- minted on X1
- rejected
- reorged out
- provider disagreement
- already processed

Frontend states documented:

- Burn submitted
- Burn confirmed
- Waiting for Ethereum finality
- Finalized
- Guardian approval pending
- Guardian approved
- Relayer submitted
- XXXL minted
- Rejected evidence
- Reorged out
- Already processed

Invalid finality cases documented:

- burn transaction failed
- burn transaction not found
- expected burn event not found
- sourceBlockNumber missing
- sourceBlockHash missing
- sourceBlockHash wrong length
- receipt blockHash differs from sourceBlockHash
- canonical block hash at sourceBlockNumber differs from sourceBlockHash
- source block is newer than finalized block
- source block has insufficient fallback confirmations
- providers disagree about source block identity
- burn event is reorged out
- burn log index points to a different event
- source chain is not Ethereum mainnet
- source token is not expected XNTD token

Relationship to processed registry:

Finality determines whether a burn can be approved.

Processed registry determines whether an approved burn has already minted.

Both are required.

Production decision still required:

Before implementation, the project must still choose the exact operational rule:

- finalized block tag only
- finalized block tag preferred with confirmation fallback
- fixed confirmation depth
- multi-provider policy
- exact number of providers
- exact confirmation depth N if fallback is used
- exact behavior for provider disagreement

Recommended production direction:

- prefer finalized block tag when available and reliable
- use conservative confirmation-depth fallback only when finalized block data is unavailable or unreliable
- require provider agreement for finality-critical fields

Current conclusion:

Stage 1 guardians must sign only canonical Ethereum burn evidence that satisfies the Stage 1 finality rule.

The signed message must include sourceBlockNumber and sourceBlockHash.

The preferred finality direction is Ethereum finalized block status, with a conservative confirmation-depth fallback only if finalized block support is unavailable or unreliable.

This closes the Ethereum finality rule requirement-definition blocker.

Implementation should still not begin until exact provider policy, fallback confirmation depth, X1 authority model, and exact test vectors are documented.

## Latest Stage 1 recipient safety policy checkpoint

The Stage 1 recipient safety policy milestone was completed on the stage-1-recipient-safety-policy branch.

Commits:

- 1f17e62 Define Stage 1 recipient safety policy
- 5995517 Merge branch 'stage-1-recipient-safety-policy'

This milestone defines the Stage 1 recipient safety policy for the XNTD-to-XXXL Gateway.

Design document added:

- docs/gateway/stage-1-recipient-safety-policy.md

Purpose:

Prevent users from burning Ethereum XNTD with an invalid, malformed, empty, zero, or unusable X1 recipient.

This is a design / readiness milestone only.

It does not implement:

- Ethereum contracts
- X1 programs
- XXXL token runtime
- X1 mint core
- processed burn registry
- guardian runtime
- relayer runtime
- watcher runtime
- frontend gateway flow
- real RPC reads
- env reads
- private keys
- API keys
- mnemonic handling
- deployment logic

Core rule:

Invalid X1 recipients must be rejected before guardian approval and before X1 mint execution.

Frontend validation should also reject invalid recipients before the Ethereum burn transaction whenever possible.

Canonical recipient format:

- exactly 32 raw bytes
- X1 / SVM public key format
- not a string
- not base58 text
- not variable-length bytes
- not an Ethereum address
- not an EIP-55 address
- not checksum-casing dependent

Base58 may be used only as display / input format.

Protocol-level recipient bytes must be exactly 32 raw bytes.

Recipient hash:

x1RecipientHash = keccak256(x1RecipientBytes)

A hash match is necessary but not sufficient.

The recipient bytes must also pass recipient safety validation.

Mandatory rejection cases:

Stage 1 must reject:

- empty recipient
- missing recipient
- recipient length not equal to 32 bytes
- malformed recipient bytes
- base58 string used directly as canonical bytes
- Ethereum address used as X1 recipient bytes
- 32 zero bytes recipient
- x1RecipientHash mismatch
- recipient format that cannot be decoded into exactly 32 bytes
- recipient value forbidden by the final X1 runtime policy

32 zero bytes policy:

Stage 1 must reject the 32-byte all-zero recipient.

Known burn / blackhole recipient policy:

Stage 1 must reject known protocol-forbidden burn / blackhole recipients if the X1 runtime or community standard defines such addresses.

Current Stage 1 minimum policy:

- reject 32 zero bytes
- treat additional known burn / blackhole recipient list as an implementation-time policy item

The recipient safety policy must not become a discretionary blacklist.

No discretionary recipient censorship:

Recipient safety checks exist to prevent user loss and malformed execution.

They must not become a general censorship mechanism.

Frontend validation:

Frontend should validate recipient before the user burns Ethereum XNTD.

Frontend should reject:

- empty input
- invalid base58 input
- decoded recipient not exactly 32 bytes
- all-zero recipient
- known forbidden burn / blackhole recipients if policy exists
- Ethereum address pasted as recipient
- malformed copied value

Frontend validation is a UX safety layer.

It does not replace guardian or mint core validation.

Ethereum burn function validation:

Preferred direction:

- accept recipient input in a format that can be validated before burn
- reject empty recipient
- reject malformed recipient if format permits
- reject all-zero 32-byte recipient if raw bytes are supplied
- emit both x1Recipient and x1RecipientHash according to the event schema

Minimum Ethereum-side requirement:

Do not allow obviously empty recipient evidence.

Preferred Ethereum-side requirement:

Reject anything that cannot be decoded into exactly 32 recipient bytes before burning.

Guardian validation:

Guardians must reject burn evidence if:

- x1Recipient is missing
- x1Recipient cannot be decoded into exactly 32 raw bytes
- x1RecipientBytes are 32 zero bytes
- x1RecipientHash does not equal keccak256(x1RecipientBytes)
- x1Recipient appears to be a malformed display string
- event payload is incomplete
- event payload is ambiguous
- final forbidden-recipient policy rejects the recipient

X1 mint core validation:

X1 mint core must reject execution if:

- raw x1RecipientBytes are missing
- raw x1RecipientBytes length is not exactly 32 bytes
- raw x1RecipientBytes are 32 zero bytes
- keccak256(x1RecipientBytes) does not equal signed x1RecipientHash
- recipient violates final X1 runtime recipient policy

Relayer behavior:

Relayers must not modify recipient data.

If relayer-submitted x1RecipientBytes do not match the signed x1RecipientHash, X1 mint core must reject.

Event schema implication:

The Ethereum burn event direction remains:

- x1RecipientHash
- x1Recipient

Preferred production direction:

Emit data in a way that allows guardians to derive exactly one x1RecipientBytes value.

Test vector implications:

Future tests and vectors must include:

- valid 32-byte recipient
- valid base58 display decoded into 32 bytes
- empty recipient rejected
- missing recipient rejected
- wrong-length recipient rejected
- 32 zero bytes rejected
- Ethereum address used as X1 recipient rejected
- base58 string used directly as canonical bytes rejected
- x1RecipientHash mismatch rejected
- known forbidden recipient rejected if policy list exists
- relayer recipient substitution rejected
- frontend invalid recipient before burn scenario

Current conclusion:

Stage 1 recipient safety policy requires canonical 32-byte X1 / SVM recipient bytes.

Stage 1 must reject empty, malformed, non-32-byte, all-zero, hash-mismatched, and policy-forbidden recipients.

Frontend should prevent clearly invalid recipient burns before the Ethereum transaction.

Guardians must reject invalid recipient evidence.

X1 mint core must reject invalid recipient execution payloads.

This closes the recipient safety policy requirement-definition blocker.

Implementation should still not begin until burn amount min/max policy, exact X1 deployment authority model, and exact cryptographic test vectors are documented.

## Latest Stage 1 burn amount policy checkpoint

The Stage 1 burn amount policy milestone was completed on the stage-1-burn-amount-policy branch.

Commits:

- 99267dc Define Stage 1 burn amount policy
- 55d1697 Merge branch 'stage-1-burn-amount-policy'

This milestone defines the Stage 1 burn amount policy for the XNTD-to-XXXL Gateway.

Design document added:

- docs/gateway/stage-1-burn-amount-policy.md

Purpose:

Define the accepted burn amount policy for converting Ethereum XNTD burn evidence into X1 XXXL mint approval.

This is a design / readiness milestone only.

It does not implement:

- Ethereum contracts
- X1 programs
- XXXL token runtime
- X1 mint core
- processed burn registry
- guardian runtime
- relayer runtime
- watcher runtime
- frontend gateway flow
- real RPC reads
- env reads
- private keys
- API keys
- mnemonic handling
- deployment logic

Core rule:

For Stage 1, XXXL mint amount equals the verified Ethereum XNTD burned amount.

Stage 1 amount rule:

- burnedAmount must be greater than zero
- sourceChainWeightBps must equal 10000
- xxxlMintAmount must equal burnedAmount
- mintToken must be XXXL
- burnedAmount must match the amount emitted by the accepted Ethereum XNTD burn event

This is a full-weight Stage 1 conversion rule.

It does not create a price peg.

It does not mean XXXL is wrapped Ethereum XNTD.

It means Stage 1 uses a 1:1 accounting conversion from verified Ethereum XNTD burn evidence into X1-native XXXL mint amount.

Zero amount policy:

Stage 1 must reject zero burnedAmount.

Reason:

- zero burn has no economic meaning
- zero burn can create spam or meaningless gateway messages
- zero mint approval should not create processed registry entries
- zero amount can hide malformed event handling

Minimum amount policy:

Stage 1 does not define an arbitrary protocol minimum above zero at the requirement-definition layer.

Reason:

- arbitrary minimums can exclude small users
- arbitrary minimums create unnecessary policy surface
- the gateway should not silently alter the meaning of a valid burn
- spam control should be considered separately from monetary correctness

Frontend may warn users about very small burns if fees make the action economically irrational.

Frontend warning is not the same as protocol rejection.

Maximum amount policy:

Stage 1 does not define an arbitrary protocol maximum at the requirement-definition layer.

Reason:

- arbitrary maximums can distort verified burn-to-mint accounting
- maximums create additional policy surface
- Stage 1 sourceChainWeightBps already defines conversion
- overflow and runtime limits should be handled as technical safety checks, not discretionary monetary policy

Implementation must still ensure numeric safety:

- burnedAmount must fit the selected integer type
- xxxlMintAmount must fit the selected integer type
- encoding must be fixed-width and non-ambiguous
- overflow must be impossible or rejected
- mint core must reject amounts outside representable bounds

Amount source of truth:

The source of truth for burnedAmount is the accepted Ethereum XNTD burn event.

Guardians must verify:

- burn transaction succeeded
- expected XNTD burn event exists
- burnedAmount is emitted by the expected event
- burnedAmount is greater than zero
- burnedAmount matches the canonical message field
- xxxlMintAmount equals burnedAmount
- sourceChainWeightBps equals 10000

Guardians must not choose burnedAmount manually.

Guardians must not choose xxxlMintAmount manually.

X1 mint core amount verification:

Before minting, X1 mint core must verify:

- burnedAmount > 0
- sourceChainWeightBps == 10000
- xxxlMintAmount == burnedAmount
- mintToken == XXXL
- encoded amount is canonical
- amount fits supported range
- no overflow is possible
- guardian signatures are over the exact amount fields

Mint core must reject:

- burnedAmount == 0
- xxxlMintAmount == 0
- xxxlMintAmount != burnedAmount
- sourceChainWeightBps != 10000
- amount encoded as decimal string
- amount encoded with ambiguous precision
- amount outside supported numeric range
- guardian-approved amount that does not match route rule

No fee subtraction in Stage 1 amount:

Stage 1 gateway amount policy does not subtract relayer fees from xxxlMintAmount.

If relayer fees exist later, they must be handled separately from the burn-to-mint amount rule.

Stage 1 route rule remains:

xxxlMintAmount = burnedAmount

No hidden fee, haircut, spread, premium, or multiplier should be applied by guardians or relayers.

Frontend behavior:

Frontend should show:

- burnedAmount
- expected xxxlMintAmount
- sourceChainWeightBps
- warning for zero amount
- optional warning for economically tiny burn amounts
- finality status
- recipient validation status

Frontend must not imply that Stage 1 guarantees market value parity between XNTD and XXXL.

Guardian rejection cases:

Guardians must reject evidence if:

- burnedAmount is missing
- burnedAmount is zero
- burnedAmount is malformed
- burnedAmount is encoded ambiguously
- burnedAmount does not match the Ethereum burn event
- xxxlMintAmount does not equal burnedAmount
- sourceChainWeightBps does not equal 10000
- amount exceeds supported encoding / runtime bounds
- amount field appears to be decimal text instead of canonical integer encoding

Relayer behavior:

Relayers must not modify amount fields.

Relayers must not submit alternate xxxlMintAmount.

Relayers must not subtract fees from mint amount.

Relayers must not round amount.

Relayers must not convert decimals.

Relayers must submit the signed message exactly as approved.

Processed registry implication:

Processed records should store:

- burnedAmount
- sourceChainWeightBps
- xxxlMintAmount

Test vector implications:

Future tests and vectors must include:

- valid positive burnedAmount
- zero burnedAmount rejected
- xxxlMintAmount equals burnedAmount
- xxxlMintAmount higher than burnedAmount rejected
- xxxlMintAmount lower than burnedAmount rejected
- sourceChainWeightBps not 10000 rejected
- amount encoded as decimal string rejected
- amount overflow / out-of-range rejected
- guardian-signed wrong amount rejected
- relayer-modified amount rejected
- processed record stores burnedAmount and xxxlMintAmount

Current conclusion:

Stage 1 burn amount policy is:

- reject zero burnedAmount
- do not define arbitrary min/max at the requirement-definition layer
- require burnedAmount to match the accepted Ethereum XNTD burn event
- require sourceChainWeightBps = 10000
- require xxxlMintAmount = burnedAmount
- reject any mismatch or ambiguous amount encoding

This closes the burn amount policy requirement-definition blocker.

Implementation should still not begin until exact cryptographic test vectors and exact X1 deployment authority model are documented.

## Latest Stage 1 X1 deployment authority model checkpoint

The Stage 1 X1 deployment authority model milestone was completed on the stage-1-x1-deployment-authority-model branch.

Commit:

- pending

This milestone defines the Stage 1 deployment authority requirements for the X1 XXXL mint core.

Design document added:

- docs/gateway/stage-1-x1-deployment-authority-model.md

Purpose:

Define the deployment authority model requirements that must be satisfied before implementation or production approval.

This is a design / readiness milestone only.

It does not implement:

- Ethereum contracts
- X1 programs
- XXXL token runtime
- X1 mint core
- processed burn registry
- guardian runtime
- relayer runtime
- watcher runtime
- frontend gateway flow
- real RPC reads
- env reads
- private keys
- API keys
- mnemonic handling
- deployment logic

Core authority principle:

Stage 1 authority model must preserve this boundary:

- guardians verify Ethereum burn evidence
- relayers transport approved messages
- mint core enforces immutable monetary route rules
- no operator can mint XXXL outside verified gateway messages

The mint core must be the only path that can mint Stage 1 XXXL from Ethereum XNTD burn evidence.

Immutable route rules:

The following route rules must not be changeable after production deployment:

- source chain is Ethereum mainnet
- source token is expected Ethereum XNTD token
- routeId is the Stage 1 Ethereum XNTD to X1 XXXL route
- sourceChainWeightBps = 10000
- xxxlMintAmount = burnedAmount
- mint token is XXXL
- hash function is keccak256
- guardian signature standard is Ed25519
- X1 recipient is 32 raw bytes
- x1RecipientHash = keccak256(x1RecipientBytes)
- sourceBlockNumber and sourceBlockHash are mandatory signed fields
- replay protection uses canonicalEventKey
- processed registry prevents duplicate minting

Deployment authority requirement:

Before production approval, the project must know exactly:

- how X1 programs / contracts are deployed
- whether deployed programs / contracts are upgradeable
- who controls upgrade authority at deployment
- how upgrade authority is removed, disabled, or constrained
- whether token mint authority is separate from program authority
- who controls token mint authority at deployment
- how token mint authority is constrained to the mint core
- whether any admin, deployer, guardian, relayer, or governance path can mint outside the verified route
- how users can independently verify the authority state

Acceptable production outcomes may include:

- non-upgradeable mint core
- upgrade authority permanently removed
- route rules hardcoded in deployed code
- mint authority owned only by immutable mint core
- mint authority unable to mint outside verified gateway messages
- public verification procedure for authority state
- separate operational configuration that cannot change monetary route rules

Unacceptable production outcomes:

- deployer can upgrade route rules after launch
- admin can change source token
- admin can change source chain
- admin can change sourceChainWeightBps
- admin can change xxxlMintAmount formula
- admin can change mint token
- guardian set can change monetary policy
- relayer can choose mint amount
- token mint authority can mint XXXL outside the mint core
- governance can silently change Stage 1 route rules
- processed registry can be bypassed by privileged authority
- emergency function can create supply outside verified burn evidence

Mint authority model:

Stage 1 must define how XXXL mint authority works.

Required properties:

- mint authority cannot be used by a human operator to mint arbitrary XXXL
- mint authority cannot be used by relayers to mint arbitrary XXXL
- mint authority cannot be used by guardians to mint arbitrary XXXL
- mint authority is constrained to verified Stage 1 gateway execution
- mint authority cannot bypass processed registry
- mint authority cannot bypass immutable route validation
- mint authority cannot bypass guardian threshold verification

Preferred direction:

The X1 XXXL token mint authority should be controlled by the mint core or by an authority mechanism that only the mint core can exercise under verified message rules.

Upgrade authority model:

Stage 1 must define whether the X1 mint core is upgradeable.

Preferred production direction:

No production upgrade authority should be able to alter Stage 1 monetary route rules.

Guardian authority boundary:

Guardians may sign approvals only after verifying source burn evidence.

Guardians must not be able to change route rules, choose arbitrary mint amounts, bypass processed registry, or mint without burn evidence.

Relayer authority boundary:

Relayers may submit approved messages and execution payloads.

Relayers must not be able to change recipient, amount, source chain, source token, routeId, canonicalEventKey, signature verification, processed registry, or mint without valid guardian approval.

Emergency controls:

Emergency controls, if any, must not create XXXL supply outside verified gateway messages.

Potentially acceptable:

- pause new mint executions
- pause relayer frontend submission
- pause guardian signing
- publish incident status

Not acceptable:

- admin mint
- admin route rewrite
- admin processed-registry bypass
- admin recipient rewrite
- admin amount rewrite
- admin replay override
- admin mint after failed verification

Public verification checklist:

Before production, the project must publish a public verification checklist covering:

- deployed mint core identity
- deployed XXXL mint identity
- routeId
- source chain
- source token
- mint token
- sourceChainWeightBps
- mint formula
- guardian signature standard
- guardian threshold model
- processed registry identity
- upgrade authority status
- mint authority status
- whether admin mint exists
- whether route rules are mutable
- whether emergency controls can mint
- whether processed registry can be bypassed

Test implications:

Future tests or verification scripts should include:

- mint core rejects wrong sourceChainWeightBps
- mint core rejects wrong xxxlMintAmount
- mint core rejects wrong source token
- mint core rejects wrong source chain
- mint core rejects wrong mint token
- mint core rejects duplicate canonicalEventKey
- no admin mint path exists
- no relayer amount override exists
- no guardian monetary override exists
- upgrade authority status is verifiable
- mint authority status is verifiable

Current conclusion:

Stage 1 requires a deployment authority model where route rules and monetary conversion cannot be changed by deployers, administrators, guardians, relayers, or mutable governance after production deployment.

The exact X1 runtime deployment mechanism still must be confirmed before implementation.

Production readiness requires public verification of upgrade authority, mint authority, route rule immutability, and absence of admin mint paths.

This closes the deployment authority model requirement-definition layer.

Implementation should still not begin until exact cryptographic test vectors are documented and the X1 runtime authority mechanics are confirmed.
