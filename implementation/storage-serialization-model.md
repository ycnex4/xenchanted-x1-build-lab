# Storage / Serialization Model

## Branch

storage-serialization-model

## Purpose

This document defines the storage and serialization direction for the post-MVP xEnchanted X1 Build Lab.

The current MVP is an in-memory state-transition model.

Storage and serialization should make the model persistable without weakening its invariants.

This milestone is documentation-only.

No TypeScript model logic is changed in this branch.

## Current validation baseline

At the start of this milestone:

- npm run typecheck: passed
- npm test: passed
- 16 test files passed
- 96 tests passed

## Design boundary

Storage must not contain independent accounting logic.

Storage should only:

- load existing state
- persist updated state
- encode / decode values deterministically
- support migrations
- support backups / snapshots

Storage must not:

- create BLD
- create XBP
- create XNTD lock state
- create X1 fee contribution
- bypass replay protection
- rewrite historical accounting fields
- silently erase processed message or event keys

## Persisted state categories

The system will likely need to persist these state categories:

- BuildState records
- Build registry indexes
- RegistrarState
- RedeemEventState
- XenBurnEventState
- future proof validation records
- future storage metadata / schema version

## BuildState persistence

Each BuildState should be persisted as a versioned record.

Expected fields:

- buildId
- owner
- ethereumIdentity
- createdAt
- updatedAt
- historyBld
- availableBld
- originBld
- earnedXbp
- availableXbp
- lockedXntd
- requiredXntdLock
- lockEpoch
- xcCommitmentActive
- x1FeeContribution
- x1TxCount
- x1FeeCountedUntilSlot
- lastFeeUpdateAt

## Registry persistence

Build registry persistence should preserve all uniqueness indexes.

Expected indexes:

- buildId -> BuildState
- owner -> buildId
- ethereumIdentity -> buildId

The registry storage must preserve duplicate prevention behavior after restart.

## Registrar state persistence

RegistrarState should persist:

- registrarAuthority
- processedMessages

processedMessages must never be treated as optional in production.

If processedMessages is lost, replay protection is broken.

## Redeem event replay persistence

RedeemEventState should persist:

- usedRedeemEvents

This set protects Core redeem BLD accounting from duplicate application.

It must be stored durably.

## XEN burn event replay persistence

XenBurnEventState should persist:

- usedXenBurnEvents

This set protects XBP accounting from duplicate application.

It must be stored durably.

## BigInt serialization

All bigint values should be serialized as decimal strings.

Reason:

- JSON does not support bigint natively.
- decimal string encoding is human-readable.
- it avoids precision loss from JavaScript number conversion.

Example shape:

- historyBld: "11"
- availableBld: "33"
- lockedXntd: "500"

Deserialization must explicitly convert decimal strings back to bigint.

Invalid bigint strings must be rejected.

## Number serialization

Small numeric fields such as lockEpoch may remain JSON numbers if their range is intentionally bounded.

Current field:

- lockEpoch: number | null

Future decision:

If lockEpoch may grow beyond safe integer range, it should be converted to bigint and serialized as decimal string.

For now, keep current MVP type unless a future milestone changes it explicitly.

## Set serialization

Set values should serialize as arrays of strings.

Example shape:

- processedMessages: ["message-1", "message-2"]
- usedRedeemEvents: ["redeem-1", "redeem-2"]
- usedXenBurnEvents: ["xen-burn-1", "xen-burn-2"]

Deserialization must reconstruct Set instances.

Duplicate values in serialized arrays should be normalized or rejected.

Recommended policy:

- reject duplicate values in serialized set arrays during strict loading
- optionally normalize during recovery tooling only

## Null handling

Nullable fields should remain explicit null values.

Examples:

- ethereumIdentity
- lockEpoch
- x1FeeCountedUntilSlot
- lastFeeUpdateAt

Do not omit nullable fields in canonical snapshots.

Explicit null makes schema changes easier to review.

## Schema versioning

Every persisted file / record should include a schema version.

Recommended top-level shape:

- schemaVersion
- kind
- data

Schema version is required for:

- BuildState snapshots
- registry snapshots
- registrar state snapshots
- replay state snapshots
- full system snapshots

## Snapshot policy

A full snapshot should include:

- schemaVersion
- createdAt
- builds
- registry indexes
- registrar state
- redeem event state
- XEN burn event state
- optional metadata

The snapshot should be enough to resume processing without losing replay protection.

## Migration policy

Any storage schema change must include:

- migration document
- old schema description
- new schema description
- migration function
- migration tests
- rollback notes if possible

No silent schema change should be allowed after storage is introduced.

## Atomic persistence policy

State should only be persisted after the transition succeeds.

Recommended flow:

1. load state
2. validate command / proof / registrar message
3. apply model transition
4. persist updated state atomically
5. return result

If persistence fails after transition, the caller must treat the whole operation as failed unless atomic write guarantees are used.

## Atomic write strategy

Future storage adapters should support atomic writes.

Possible local-file approach:

1. write new snapshot to temporary file
2. fsync if needed
3. rename temporary file to final path
4. keep previous backup

This prevents partial JSON files from becoming canonical state.

## Backup strategy

Before production-like use, storage should support backups.

Recommended:

- keep latest valid snapshot
- keep previous snapshot
- optionally keep timestamped snapshots
- verify snapshot before replacing canonical state

## Integrity checks

Future snapshots may include:

- content hash
- schema version
- record count
- replay key counts
- createdAt
- previous snapshot hash

These are not required for MVP documentation but should be considered before production.

## Storage adapter boundary

Storage adapter should expose simple operations.

Possible future interface:

- loadSnapshot()
- saveSnapshot(snapshot)
- loadBuild(buildId)
- saveBuild(build)
- loadReplayState()
- saveReplayState(state)

The exact API should be decided in a later implementation milestone.

## Serialization tests to add later

Future tests should cover:

- BuildState serialization round-trip
- registrar state serialization round-trip
- replay set serialization round-trip
- bigint decimal string encoding
- invalid bigint rejection
- duplicate set entry rejection
- schema version requirement
- snapshot load / save
- migration path
- corrupted snapshot rejection

## Current known exclusions

This milestone does not implement:

- serializer functions
- deserializer functions
- storage adapter
- file storage
- database storage
- migration code
- snapshot code
- API / CLI integration
- proof persistence
- watcher persistence

## Main invariant

Storage should make the MVP durable.

It must not change the meaning of any state-transition rule.
