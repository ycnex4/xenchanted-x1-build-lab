# XNTD Lock / Relock Event Identity Design

## Purpose

This document defines the intended event identity model for XNTD lock and relock events.

The goal is to prepare a clean per-event replay protection design before changing runtime code.

This document does not implement the model yet.

## Current MVP state

In the current MVP, XNTD lock and relock are protected by registrar-level replay protection:

- processed registrar message IDs

There is no dedicated per-event replay key equivalent to:

- redeemKey for Core Redeem
- xenBurnKey for XEN Burn

This is documented as an MVP assumption / known limitation.

## Why messageId is not enough

messageId protects the registrar message.

It does not necessarily protect the source event.

If the same source lock or relock event is submitted with a different messageId, registrar-level replay protection will not detect it.

For accumulative accounting, this could create double-counting.

For XNTD lock and relock, the operation is overwrite-based, so the main risk is different:

- not double-counting
- possible state regression

Example:

1. user locks 100 XNTD
2. user later relocks 50 XNTD
3. old 100 XNTD lock event is replayed with a new messageId
4. state may regress back to lockedXntd = 100

This is not a token inflation issue, but it is still an incorrect state transition.

## How lock / relock differs from Core Redeem and XEN Burn

Core Redeem and XEN Burn are contribution events.

They are accumulative.

A duplicated Core Redeem or XEN Burn event would incorrectly increase accounting.

XNTD lock and relock are commitment-state events.

They overwrite commitment fields:

- lockedXntd
- requiredXntdLock
- lockEpoch
- updatedAt

Therefore replay protection must preserve commitment history order and prevent old source events from being accepted again under new message IDs.

## Design goal

The design should provide source-event replay protection for:

- LOCK_XNTD
- RELOCK_XNTD

without preventing legitimate future relocks.

The system must allow:

- one initial lock
- later legitimate relocks
- relocks in later epochs
- relocks with different amount requirements

The system must reject:

- replay of the same lock source event
- replay of the same relock source event
- replay of old commitment events under new message IDs

## Proposed event identity

Use a dedicated XNTD lock event key.

Possible name:

- xntdCommitmentEventKey

This key should identify the source event, not the registrar message.

The key should be derived from canonical source-event identity.

For Ethereum / XC-side events, the key should include:

- source chain ID
- source contract address
- event kind
- transaction hash
- log index / event index

Recommended conceptual format:

    xntdCommitmentEventKey = hash(
      sourceChainId,
      sourceAddress,
      eventKind,
      transactionHash,
      logIndex
    )

Where eventKind is one of:

- LOCK_XNTD
- RELOCK_XNTD

## One key type or two key types

There are two reasonable options.

Option A:

- one shared key type: XntdCommitmentEventKey
- eventKind distinguishes LOCK_XNTD from RELOCK_XNTD

Option B:

- LockEventKey
- RelockEventKey

Recommendation:

Use one shared key type:

- XntdCommitmentEventKey

Reason:

Lock and relock update the same commitment-state domain.

A shared event-key state makes it easier to guarantee that the same source event cannot be replayed across lock/relock paths.

The eventKind inside the canonical event identity still distinguishes lock from relock.

## Proposed state model

Add a new replay state:

    XntdCommitmentEventState {
      usedXntdCommitmentEvents: Set<XntdCommitmentEventKey>
    }

The name XntdCommitmentEventKey is preferred over XntdLockEventKey because the replay domain covers the full XNTD commitment state, not only the initial lock action.

Add helper:

    acceptXntdCommitmentEvent(...)

or two semantic helpers over the same state:

    acceptXntdLockEvent(...)
    acceptXntdRelockEvent(...)

Both should use the same underlying usedXntdCommitmentEvents set.

Snapshot serialization note:

When implemented, XntdCommitmentEventState must be included in snapshot serialization and deserialization.

Otherwise, replay protection would be lost after process restart or snapshot recovery.

## Proposed registrar payload changes

LOCK_XNTD payload should include:

- xntdCommitmentEventKey
- amountXntd
- lockEpoch
- lockedAt

RELOCK_XNTD payload should include:

- xntdCommitmentEventKey
- amountXntd
- lockEpoch
- relockedAt

## Proposed handler order

For LOCK_XNTD:

1. message kind precondition
2. authority precondition
3. duplicate registrar message precondition
4. duplicate xntdCommitmentEventKey precondition
5. amount precondition
6. acceptRegistrarMessage
7. acceptXntdLockEvent
8. lockXntd

For RELOCK_XNTD:

1. message kind precondition
2. authority precondition
3. duplicate registrar message precondition
4. duplicate xntdCommitmentEventKey precondition
5. amount precondition
6. commitment-active precondition
7. availableBld >= historyBld precondition
8. acceptRegistrarMessage
9. acceptXntdLockEvent
10. relockXntd

The exact implementation can combine event acceptance and state mutation, but preconditions must run before mutations.

## Ordering and state regression

Per-event replay protection prevents the same source event from being replayed under a new messageId.

It does not by itself prove that a newer event cannot be followed by an older but different event.

For production, this may require an additional ordering guard.

Possible ordering sources:

- lockEpoch
- lockedAt / relockedAt
- source block number
- finalized slot / block height
- monotonic commitment version

MVP design note:

The first implementation can add per-event replay protection without adding full production ordering guards.

Before production, decide whether lock/relock must reject stale-but-unique events.

## Proposed ordering guard for MVP

Per-event replay protection prevents the same source event from being applied twice.

It does not prevent stale-but-unique commitment events.

A simple MVP ordering guard can use monotonic lockEpoch.

Recommended MVP rule:

    newLockEpoch > currentLockEpoch

If a lock or relock event has lockEpoch less than or equal to the current Build lockEpoch, reject it.

This protects against the most obvious state regression case:

1. relock in epoch 2 is applied
2. older lock / relock from epoch 1 is submitted later
3. epoch guard rejects the stale event

This does not replace production-grade ordering.

For production, a stricter ordering source may still be needed, such as:

- source block number
- finalized slot / block height
- event timestamp
- monotonic commitment version

## Interaction with requiredXntdLock

In the current MVP, requiredXntdLock is accepted from registrar-provided amountXntd.

The intended production rule is:

    requiredXntdLock = current epoch Core L1 nominal

Event identity does not solve epoch minimum validation.

That remains a separate integration requirement.

## Interaction with unlock

The current MVP has no unlock flow.

If unlock is added later, it should either:

- use the same XntdCommitmentEventKey replay state, or
- define a separate commitment event identity model covering lock / relock / unlock together

Do not design unlock replay protection separately without considering lock / relock.

## Recommended implementation sequence

1. Keep this design document separate from runtime code.
2. Add XntdCommitmentEventKey types and event state.
3. Add low-level replay tests for XNTD commitment event state.
4. Add snapshot serialization / deserialization for usedXntdCommitmentEvents.
5. Add registrar handler tests:
   - duplicate lock event key is rejected
   - duplicate relock event key is rejected
   - same source event with different messageId is rejected
   - valid future relock still works
6. Add proof / registrar payload fields.
7. Add watcher candidate / proof conversion support.
8. Add MVP lockEpoch ordering guard if accepted for implementation.
9. Update CLI summary fields if needed.
10. Update assumptions once implemented.

## Current decision

The current branch is design-only.

No runtime behavior is changed by this document.

The preferred future model is:

- one shared XntdCommitmentEventKey
- one shared usedXntdCommitmentEvents replay set
- eventKind included in canonical source identity
- registrar message replay protection plus source-event replay protection
- monotonic lockEpoch as the recommended MVP ordering guard
- stricter ordering guard considered separately before production
