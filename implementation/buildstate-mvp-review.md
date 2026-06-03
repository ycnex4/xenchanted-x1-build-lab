# BuildState MVP Model Review / Consolidation

## Branch

buildstate-mvp-review

## Purpose

This document consolidates the current BuildState MVP model after the main accounting, replay protection, lock/relock, fee checkpoint, and registrar integration milestones.

This milestone is documentation / review only.

No TypeScript model logic is changed in this branch.

## Current implementation status

The MVP model currently includes:

- BuildState account model
- createBuild instruction
- canonical Build registry / duplicate prevention
- registrar replay protection
- Core redeem -> history_bld / available_bld transition
- used_redeem_events replay protection
- Registrar CORE_REDEEM integration
- XEN Burn Power transition
- used_xen_burn_events replay protection
- Registrar XEN_BURN integration
- Genesis Origin BLD claim model
- XNTD lock / relock model
- X1 Fee Contribution checkpoint model
- Registrar LOCK_XNTD / RELOCK_XNTD integration
- Registrar X1_FEE_CHECKPOINT integration

## BuildState accounting layers

BuildState currently separates the following accounting layers:

### BLD

Fields:

- historyBld
- availableBld
- originBld

Meaning:

- historyBld is historical, non-decreasing XC contribution from Core redeem history.
- availableBld is usable / spendable / transferable BLD.
- originBld is one-time Genesis Origin allocation and does not increase historyBld.

Implemented sources:

- Core redeem increases historyBld and availableBld.
- Genesis Origin sets originBld and increases availableBld.

### XEN Burn Power

Fields:

- earnedXbp
- availableXbp

Meaning:

- earnedXbp is historical XEN Burn Power.
- availableXbp is usable XEN Burn Power.

Implemented sources:

- XEN_BURN registrar flow applies XBP through XEN burn event replay protection.

### XNTD commitment

Fields:

- lockedXntd
- requiredXntdLock
- lockEpoch
- xcCommitmentActive

Meaning:

- lockedXntd records the current committed XNTD amount.
- requiredXntdLock records the active required lock amount.
- lockEpoch records the epoch for the lock / relock.
- xcCommitmentActive indicates active XC commitment.

Implemented transitions:

- lockXntd
- relockXntd
- Registrar LOCK_XNTD
- Registrar RELOCK_XNTD

Relock integrity rule:

- availableBld >= historyBld

This means relock requires the Build to still hold enough available BLD to cover its historical XC contribution.

Genesis Origin BLD can make availableBld greater than historyBld and does not block relock.

### X1 Fee Contribution

Fields:

- x1FeeContribution
- x1TxCount
- x1FeeCountedUntilSlot
- lastFeeUpdateAt

Meaning:

- x1FeeContribution accumulates counted X1 fee contribution.
- x1TxCount accumulates counted X1 transaction count.
- x1FeeCountedUntilSlot records checkpoint progress.
- lastFeeUpdateAt records the latest fee checkpoint timestamp.

Implemented transitions:

- applyX1FeeContributionCheckpoint
- Registrar X1_FEE_CHECKPOINT

Slot rule:

- countedUntilSlot must strictly increase.

## Replay protection layers

### Registrar replay protection

State:

- RegistrarState.processedMessages

Purpose:

- Prevents the same registrar messageId from being processed twice.

Implemented for:

- CORE_REDEEM
- XEN_BURN
- LOCK_XNTD
- RELOCK_XNTD
- X1_FEE_CHECKPOINT

### Core redeem event replay protection

State:

- RedeemEventState.usedRedeemEvents

Purpose:

- Prevents the same Core redeem event from applying BLD twice.

### XEN burn event replay protection

State:

- XenBurnEventState.usedXenBurnEvents

Purpose:

- Prevents the same XEN burn event from applying XBP twice.

## Atomicity model

The MVP follows a consistent atomicity rule:

A registrar message is recorded only after the underlying accounting transition succeeds.

This protects against partially applied invalid messages.

Failure paths should not:

- mark registrar message as processed
- mark redeem event as used
- mark XEN burn event as used
- update BuildState
- partially update accounting fields

## Implemented registrar message kinds

Current RegistrarMessageKind values:

- CORE_REDEEM
- XEN_BURN
- LOCK_XNTD
- RELOCK_XNTD
- X1_FEE_CHECKPOINT

## Current test coverage

At this checkpoint:

- npm run typecheck: passed
- npm test: passed
- 16 test files passed
- 96 tests passed

The tests cover:

- BuildState defaults
- createBuild behavior
- Build registry uniqueness
- registrar replay protection
- Core redeem BLD accounting
- Core redeem event replay protection
- Registrar CORE_REDEEM integration
- XEN Burn Power accounting
- XEN burn event replay protection
- Registrar XEN_BURN integration
- Genesis Origin BLD tiering and one-time claim
- XNTD lock / relock
- Registrar LOCK_XNTD / RELOCK_XNTD integration
- X1 Fee Contribution checkpoints
- Registrar X1_FEE_CHECKPOINT integration
- non-mutating failure paths across major flows

## Current known exclusions

The MVP model does not yet implement:

- real on-chain proof validation
- Ethereum XEN burn log proof validation
- Core redeem proof validation
- XNTD escrow proof validation
- X1 fee source transaction proof validation
- registrar signature validation
- Merkle proof logic
- bridge proof logic
- unlock flow
- BLD transfer / burn mechanics
- external registry for Genesis Origin claims
- epoch parameter source
- XNTD amount calculation policy
- fee normalization policy
- slot finality policy
- BLD minting from X1 fees
- persistent storage
- API / CLI integration

## Design confidence

The current model is coherent as an MVP state-transition model.

The separation between historyBld, availableBld, originBld, XBP, XNTD commitment, and X1 fee contribution is clear.

The most important safety property is already consistent across implemented registrar flows:

- validate registrar message
- validate replay protection
- apply underlying transition
- only then record registrar message

## Next recommended milestones

Recommended next milestones:

1. Registrar / instruction surface review
2. Implementation test matrix
3. Post-MVP integration policy
4. Proof model design
5. Storage / serialization model
6. API / CLI integration design

## Main invariant

The MVP BuildState model must keep each accounting layer separate.

No transition should create value in an unrelated layer.
