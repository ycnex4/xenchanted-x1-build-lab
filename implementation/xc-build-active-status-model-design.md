# XC Build active status model design

This document defines the read-only active status model for XC Build.

This milestone is design-only.

No runtime code is added in this milestone.

No dependencies are changed in this milestone.

No real RPC is executed in this milestone.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Background

The accepted active validity rule defines active status as:

    optional current-commitment signal

The accepted rule also states:

    Active status is based on XNTD lock / relock state.
    Inactive Build keeps historical contribution.
    External X1 projects may choose whether to use active status.
    Forge participation is out of scope for MVP active validity.

This document defines the future read-only status model shape.

## Design goal

Define the exact active status model that a future helper can return.

The model should answer:

- is this Build currently active?
- why is it active or inactive?
- what XNTD lock state was used?
- is relock needed?
- what current context was used?
- what historical contribution remains visible?

The model must be read-only and non-mutating.

## Core principle

Active status calculation must not mutate Build state.

It must not change:

- history_bld
- available_bld
- origin_bld
- lockedXntd
- requiredXntdLock
- lockEpoch
- replay protection state
- registrar state
- proof state

It only reads state and returns interpretation.

## Proposed helper name

Recommended helper name:

    getBuildActiveStatus()

Alternative acceptable name:

    deriveBuildActiveStatus()

Avoid names that imply mutation or enforcement.

Do not use names like:

    activateBuild()
    validateAndActivateBuild()
    enforceBuildActive()

## Proposed output shape

Recommended output shape:

    interface BuildActiveStatus {
      readonly isActive: boolean;
      readonly status: BuildActiveStatusValue;
      readonly reason: BuildActiveStatusReason;
      readonly historyBld: bigint;
      readonly availableBld: bigint;
      readonly lockedXntd: bigint;
      readonly requiredXntdLock: bigint;
      readonly lockEpoch: bigint | null;
      readonly currentEpoch: bigint | null;
      readonly needsRelock: boolean;
    }

Recommended status values:

    "ACTIVE"
    "INACTIVE"
    "UNKNOWN"

Recommended reason values:

    "ACTIVE_LOCK_CURRENT"
    "INACTIVE_NO_HISTORY"
    "INACTIVE_NO_LOCK"
    "INACTIVE_LOCK_BELOW_REQUIRED"
    "INACTIVE_RELOCK_REQUIRED"
    "UNKNOWN_NO_CURRENT_CONTEXT"

## Status value semantics

### ACTIVE

A Build is active when it has historical contribution and current accepted XNTD commitment.

For MVP, this means:

- historyBld > 0
- lockedXntd >= requiredXntdLock
- requiredXntdLock > 0
- lockEpoch exists
- if current context is provided, lock/relock is accepted under current active policy

### INACTIVE

A Build is inactive when historical contribution may exist, but current commitment is missing, insufficient, or stale.

Inactive does not erase history.

Inactive does not reject Core redeem proof.

Inactive does not reduce history_bld.

Inactive does not force external projects to ignore the Build.

### UNKNOWN

Unknown is used when the helper cannot determine active status because current context is required but not provided.

Unknown should not be treated as invalid history.

Unknown should be treated as:

    status cannot be determined from available inputs

## Reason semantics

### ACTIVE_LOCK_CURRENT

The Build has historical contribution and the lock/relock state satisfies the active status requirement.

### INACTIVE_NO_HISTORY

The Build has no historical contribution.

Recommended condition:

    historyBld == 0

This reason does not mean the Build record is invalid.

It only means there is no historical contribution for active status.

### INACTIVE_NO_LOCK

The Build has historical contribution but no XNTD commitment.

Recommended condition:

    historyBld > 0
    lockedXntd == 0

### INACTIVE_LOCK_BELOW_REQUIRED

The Build has XNTD commitment, but lockedXntd is below requiredXntdLock.

Recommended condition:

    lockedXntd > 0
    requiredXntdLock > 0
    lockedXntd < requiredXntdLock

### INACTIVE_RELOCK_REQUIRED

The Build has historical contribution and previous lock/relock, but current epoch policy requires refresh.

Recommended condition depends on future epoch policy.

Example:

    lockEpoch < currentEpoch

if the accepted policy requires relock every epoch.

This must be finalized before runtime implementation.

### UNKNOWN_NO_CURRENT_CONTEXT

The helper needs current epoch / current required lock context but it was not provided.

This is useful when a caller wants strict current status, but only historical Build state is available.

## Input model

Recommended input shape:

    interface GetBuildActiveStatusInput {
      readonly build: BuildState;
      readonly currentEpoch?: bigint;
      readonly currentRequiredXntdLock?: bigint;
    }

A future implementation may also accept:

    readonly requireCurrentEpoch?: boolean;

If requireCurrentEpoch is true and current context is missing, return UNKNOWN.

If requireCurrentEpoch is false or absent, the helper may derive status from stored Build lock state only.

## Current context relationship

Current context should come from already designed XC protocol context paths.

Possible sources:

- XcBuildValidationContext
- XcProtocolParams
- authoritative XC epoch minimum source
- static test source

This model does not introduce real RPC.

This model does not read process.env.

This model does not create a source.

It only defines what a future helper may receive.

## XNTD fields

The helper should use existing Build state fields:

- lockedXntd
- requiredXntdLock
- lockEpoch

The model should not invent separate lock balances.

The model should not mutate lock values.

The model should not create unlock mechanics.

## Epoch relationship

Epoch handling must follow the accepted epoch policy:

    currentEpoch should not invalidate historical Core redeem.
    currentEpoch may help decide whether active status is current.

Therefore:

- currentEpoch can affect active status
- currentEpoch must not erase history_bld
- currentEpoch must not reject Core redeem proof
- currentEpoch must not change available_bld automatically

## Relock relationship

needsRelock should be true when the Build has historical contribution and lock state exists, but active status requires refresh.

The exact condition should be decided before implementation.

Possible MVP options:

1. Strict epoch match

    needsRelock = lockEpoch < currentEpoch

2. Accepted lock amount only

    needsRelock = lockedXntd < currentRequiredXntdLock

3. Hybrid

    needsRelock = lockEpoch < currentEpoch || lockedXntd < currentRequiredXntdLock

Recommended direction:

    Use amount-based current requirement first.
    Add strict epoch freshness only if there is a clear product reason.

Reason:

Current active status is a signal, not punishment.

Strict epoch expiration may create unnecessary friction if currentRequiredXntdLock is already satisfied.

## External project interpretation

External X1 projects may use any part of the status model.

They may use:

- isActive
- status
- reason
- historyBld
- availableBld
- lockedXntd
- requiredXntdLock
- lockEpoch
- currentEpoch
- needsRelock

They may also ignore the model entirely.

The model should expose context without forcing policy.

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
- enforce active status
- define external project policy
- erase inactive Build history
- introduce Forge requirements
- add unlock mechanics
- change BLD transfer/sale rules
- implement getBuildActiveStatus()
- add CLI commands

## Recommended future milestones

After this design is reviewed, recommended future milestones are:

1. active status model design review

    xc-build-active-status-model-design-review

2. active status model implementation

    xc-build-active-status-model

3. active status app/service integration design

    xc-build-active-status-app-integration-design

4. final MVP Build validation rule design

    xc-build-validation-mvp-rule-design

Forge is intentionally not included in this sequence.

## Decision

MVP active status model should be:

    read-only
    non-mutating
    based on Build state and optional current context
    able to return ACTIVE / INACTIVE / UNKNOWN
    able to explain status through reason codes
    explicit that inactive status does not erase history

Recommended next step:

    xc-build-active-status-model-design-review
