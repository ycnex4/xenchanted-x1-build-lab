# External Watcher / Indexer Integration

## Branch

watcher-indexer-integration

## Purpose

This document defines the future watcher / indexer integration direction for the post-MVP xEnchanted X1 Build Lab.

The current MVP is an in-memory state-transition model with registrar flows, replay protection, and accounting invariants.

The watcher / indexer layer should observe external systems and submit validated facts into the existing model without becoming a second source of accounting logic.

This milestone is documentation-only.

No TypeScript model logic is changed in this branch.

## Current validation baseline

At the start of this milestone:

- npm run typecheck: passed
- npm test: passed
- 17 test files passed
- 98 tests passed

## Design boundary

The watcher / indexer layer may:

- observe external chains or systems
- normalize source events
- derive canonical event keys
- apply finality policy
- build proof candidates
- build registrar message candidates
- submit validated payloads to API / CLI / service layer

The watcher / indexer layer must not:

- mutate BuildState directly
- create BLD directly
- create XBP directly
- create XNTD commitment directly
- create X1 fee contribution directly
- bypass registrar replay protection
- bypass event replay protection
- rewrite persisted state
- treat unfinalized events as final in production

## Core principle

Watchers observe.

Indexers normalize.

Proof / registrar layers validate.

The MVP model applies state transitions.

No watcher should be trusted to directly edit BuildState.

## Expected watcher categories

The project will likely need watchers for:

- Ethereum Core redeem events
- Ethereum or source-chain XEN burn events
- XNTD lock / relock events
- X1 fee contribution checkpoints
- optional Genesis Origin snapshot / eligibility source

## Watcher pipeline

Recommended pipeline:

1. observe source event
2. normalize source event into internal candidate format
3. derive canonical event key
4. apply source-specific finality rule
5. validate expected source contract / program / event kind
6. derive accounting payload
7. submit proof or registrar message
8. model layer applies replay-protected transition
9. persist successful state update

## Canonical event key policy

Watcher-derived event keys must be deterministic.

Recommended key components:

- source chain id
- source contract / program address
- event kind
- transaction hash or source event id
- log index / instruction index / event index

The exact string format should be finalized in the proof object type implementation milestone.

## Source-specific watchers

### Core redeem watcher

Purpose:

- observe Core redeem events
- derive BLD amount
- derive redeemKey
- build registrar CORE_REDEEM payload or proof candidate

Output should eventually feed:

- applyRegistrarCoreRedeem

### XEN burn watcher

Purpose:

- observe qualifying XEN burn events
- derive XBP amount
- derive xenBurnKey
- build registrar XEN_BURN payload or proof candidate

Output should eventually feed:

- applyRegistrarXenBurn

### XNTD lock / relock watcher

Purpose:

- observe XNTD lock or relock events
- derive locked XNTD amount
- derive lock epoch
- build registrar LOCK_XNTD or RELOCK_XNTD payload

Output should eventually feed:

- applyRegistrarXntdLock
- applyRegistrarXntdRelock

### X1 fee checkpoint indexer

Purpose:

- aggregate X1 fee contribution
- count qualifying transactions
- derive feeAmount
- derive txCount
- derive countedUntilSlot
- build registrar X1_FEE_CHECKPOINT payload

Output should eventually feed:

- applyRegistrarX1FeeCheckpoint

## Duplicate handling

Duplicate watcher observations must be safe.

Possible duplicate sources:

- watcher restart
- reorg recovery
- polling overlap
- multiple watchers observing the same event
- manual resubmission

Protection layers:

- canonical event keys
- usedRedeemEvents
- usedXenBurnEvents
- processedMessages
- countedUntilSlot monotonicity for X1 fee checkpoints

## Reorg handling

Watchers must handle source-chain reorgs before production use.

Possible approach:

- observe event as pending
- wait finality threshold
- submit only finalized events
- keep raw observation log
- support replay from safe block / slot

No unfinalized event should be submitted into the production state model.

## Error handling

Watcher errors should not corrupt model state.

Failure cases:

- malformed event
- wrong source contract
- wrong event kind
- invalid amount
- unfinalized event
- duplicate canonical key
- duplicate registrar message
- storage failure
- API submission failure

Failures should be logged and retryable when appropriate.

## Security policy

Watchers should not hold unnecessary signing authority.

If registrar signing is required, prefer separation:

- watcher observes and normalizes
- validator verifies
- registrar signs or submits
- model applies

Do not combine observation, signing, storage mutation, and admin power in one unchecked component.

## Testing policy

Future watcher / indexer tests should cover:

- valid source event normalization
- canonical key derivation
- wrong source rejection
- wrong event kind rejection
- unfinalized event rejection
- duplicate event handling
- retry behavior
- reorg-safe behavior
- generated registrar payload shape
- no direct BuildState mutation
- no unrelated accounting value creation

## Recommended implementation order

Recommended order:

1. proof object types
2. canonical event key helpers
3. watcher candidate types
4. normalization tests
5. finality policy helpers
6. registrar payload builders
7. local mock watcher tests
8. service integration tests
9. real source adapters

## Current known exclusions

This milestone does not implement:

- watcher code
- indexer code
- proof validators
- real chain event parsing
- RPC integration
- finality helpers
- registrar signing
- storage persistence
- API / CLI submission
- reorg recovery logic

## Main invariant

Watchers and indexers may observe external facts.

They must not become the authority that mutates BuildState directly.
