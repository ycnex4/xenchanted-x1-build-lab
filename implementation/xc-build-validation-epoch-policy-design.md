# XC Build validation epoch policy design

This document defines the epoch policy for XC Build validation.

This milestone is design-only.

No runtime code is added in this milestone.

No dependencies are changed in this milestone.

No real RPC is executed in this milestone.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Background

The project now has a safe protocol-context pipeline:

    XcProtocolParams
    -> deriveCurrentXcBuildRequirements()
    -> XcBuildValidationContext
    -> optional xcBuildValidationContext in appSubmitProof()

The context exposes current XC protocol requirements, including:

- currentEpoch
- currentBaseNominal
- currentXenBurnAmount
- requiredXntdLockMinimum
- nextHalvingTs
- genesisTs
- halvingInterval
- xenBurnHalvingInterval

The app proof submission layer can now receive this context, but no enforcement has been added yet.

## Design goal

Define how current XC epoch context should relate to historical proofs and current Build validity.

The main distinction is:

    historical contribution

versus:

    current active validity

Historical contribution should not be invalidated merely because XC epoch changed.

Current active validity may depend on current epoch lock/relock requirements.

## Explicit MVP scope

The MVP epoch policy focuses on:

- Core redeem history
- history_bld
- XNTD lock
- XNTD relock
- current Build active validity

## Explicit out of scope

Forge participation is out of scope for MVP Build validity.

Forge is not required by this epoch policy.

Forge should not be used as an implicit Build activation requirement.

Forge should not be used as an implicit epoch validation requirement.

If Forge participation is ever reintroduced, it must be handled in a separate future design milestone.

Do not mix Forge rules into this milestone.

## Core redeem policy

Core redeem proof is historical.

A valid Core redeem proof should remain valid even if current XC epoch changes later.

Core redeem contributes to historical Build value.

Core redeem may affect:

- history_bld
- available_bld
- historical participation record

Core redeem should not be rejected merely because:

- currentEpoch has changed
- currentBaseNominal has changed
- currentXenBurnAmount has changed
- nextHalvingTs has moved

Reason:

Core redeem is proof of a past user action.

Changing epoch conditions should not erase or invalidate honest historical participation.

## history_bld policy

`history_bld` is historical and non-decreasing.

It should not be reduced by:

- epoch changes
- relock requirements
- temporary inactive Build status
- user sale or transfer of available BLD
- future protocol-context changes

Epoch policy may affect whether a Build is currently active, but it should not erase historical contribution.

## available_bld policy

`available_bld` is usable/spendable BLD.

Epoch policy should not automatically change `available_bld`.

Separate rules may affect available BLD through user actions, transfer, burn, sale, or relock integrity requirements.

This milestone does not change available BLD mechanics.

## XNTD lock policy

XNTD lock is current active validity context.

A Build may have historical contribution but still require an XNTD lock to be active.

The required XNTD lock minimum should be derived from current XC protocol context.

Current MVP direction:

    requiredXntdLockMinimum = currentBaseNominal

A lock proof records the requirement observed at the time of the lock/relock event.

The app/registrar validation path already distinguishes:

- observedRequiredXntdLock
- authoritative epoch minimum source
- requiredXntdLock recorded in Build state

This milestone does not replace that existing path.

## XNTD relock policy

Relock is the mechanism for updating active Build validity across epoch changes.

When XC epoch changes, the required lock amount may change.

A Build should not lose historical contribution because epoch changed.

But active validity may require relock under the new epoch requirement.

Relock should be the normal path for refreshing active validity.

## Build active validity policy

A Build should be understood as having two layers:

1. historical layer
2. active validity layer

Historical layer includes:

- Core redeem history
- history_bld
- accumulated historical contribution

Active validity layer includes:

- current or accepted XNTD lock state
- relock status
- current epoch requirement compliance when required

This prevents accidental invalidation of honest historical proofs while still allowing the system to require current commitment for active status.

## Current epoch usage

`currentEpoch` should not be used to reject historical Core redeem proof.

`currentEpoch` may be used to decide:

- whether a Build lock is current
- whether relock is needed
- what currentBaseNominal applies to new lock/relock requirements
- what current active validity means

Current epoch should be used carefully and only in focused enforcement milestones.

## Historical proof policy

Historical proofs should generally remain valid if they prove a real past event.

This includes:

- Core redeem proof
- XEN burn proof if used as historical contribution
- past lock/relock proof as evidence of past state

However, a historical lock may not be sufficient for current active validity after epoch changes.

This distinction is intentional.

## New Build activation policy

For new Build activation, the system may require:

- historical contribution proof
- current XNTD lock/relock compliance

The exact activation rule should be finalized in a future Build validity rule milestone.

This epoch policy only establishes that historical contribution and current active validity are separate layers.

## Existing XNTD lock validation relationship

Existing authoritative XNTD lock validation remains valid.

This policy should not duplicate or replace:

- observedRequiredXntdLock propagation
- authoritative XC epoch minimum source
- registrar/app lock validation
- relock validation

Future enforcement should reuse the existing lock validation chain where possible.

## What this policy must prevent

This policy should prevent accidental rejection of honest historical contribution.

Example:

    currentEpoch = 2
    user has valid Core redeem proof from epoch 0

This proof should still be valid as historical contribution.

The Build may still need current lock/relock to be active, but the history should remain.

## What this policy allows

This policy allows the system to require current commitment for active Build status.

Example:

    Build has history_bld from older Core redeem
    XC epoch changes
    currentBaseNominal changes
    active Build validity may require relock under the new requiredXntdLockMinimum

This is not a rejection of history.

It is an active-state requirement.

## Non-goals

This milestone does not:

- add runtime code
- add tests
- add dependencies
- execute real RPC
- change appSubmitProof behavior
- change watcher behavior
- change registrar behavior
- change proof payload behavior
- finalize global Build validity enforcement
- introduce Forge requirements
- enforce currentEpoch in code
- enforce requiredXntdLockMinimum in code
- change BLD transfer/sale rules

## Recommended future milestones

After this design is reviewed, recommended future milestones are:

1. epoch policy review

    xc-build-validation-epoch-policy-design-review

2. Build active validity design

    xc-build-active-validity-rule-design

3. XNTD lock active status implementation

    xc-build-xntd-lock-active-status

4. final MVP Build validation rule design

    xc-build-validation-mvp-rule-design

Forge is intentionally not included in this sequence.

## Decision

MVP epoch policy is:

    historical contribution remains historical
    active validity may require current XNTD lock/relock compliance
    currentEpoch should not invalidate Core redeem history
    Forge participation is out of scope for MVP Build validity

Next step after this design:

    xc-build-validation-epoch-policy-design-review
