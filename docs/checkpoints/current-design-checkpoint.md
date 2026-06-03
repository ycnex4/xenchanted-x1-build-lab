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

## Current next steps

Potential next documents / design areas:

1. Registrar / instruction surface review milestone.
2. Post-MVP integration policy.
3. Proof model design milestone.
4. Storage / serialization model milestone.
5. Review npm audit findings separately.
6. Continue implementation only with clean typecheck and tests.




















