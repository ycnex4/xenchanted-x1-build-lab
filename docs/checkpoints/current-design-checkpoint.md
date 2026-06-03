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

1. Implementation test matrix.
2. Post-MVP integration policy.
3. XEN Burn Power milestone.
4. used_xen_burn_events replay protection milestone.
5. Review npm audit findings separately.
6. Continue implementation only with clean typecheck and tests.




















