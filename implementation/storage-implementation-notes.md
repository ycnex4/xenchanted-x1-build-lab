# Storage Implementation Notes

## Branch

storage-implementation

## Purpose

This branch starts the real storage implementation layer for the xEnchanted X1 Build Lab.

The first implementation step adds pure serialization / deserialization helpers for the current MVP state objects.

This milestone does not add a file storage adapter yet.

## Implemented files

- src/storage/serialization.ts
- tests/storage-serialization.test.ts

Updated:

- src/index.ts

## Implemented serialization helpers

The branch adds serialization / deserialization support for:

- BuildState
- BuildRegistry
- RegistrarState
- RedeemEventState
- XenBurnEventState

## Schema version

The storage schema starts with:

- STORAGE_SCHEMA_VERSION = 1

Each serialized object includes:

- schemaVersion
- kind

This prepares future migration support.

## BigInt policy

All bigint fields are serialized as non-negative decimal strings.

Examples:

- historyBld -> "121"
- availableBld -> "176"
- x1FeeCountedUntilSlot -> "9000"

Deserialization rejects invalid bigint strings.

## Set policy

Set values are serialized as sorted arrays of strings.

Covered sets:

- processedMessages
- usedRedeemEvents
- usedXenBurnEvents

Deserialization rejects duplicate set entries.

## Map policy

Registry maps are serialized as sorted arrays of [key, value] pairs.

Covered maps:

- canonicalBuildByOwner
- canonicalBuildByEthereumIdentity

Deserialization rejects duplicate map keys.

## Registry policy

BuildRegistry serialization includes:

- builds
- canonicalBuildByOwner
- canonicalBuildByEthereumIdentity

Build deserialization rejects duplicate buildId values.

## Test coverage

Added test file:

- tests/storage-serialization.test.ts

Covered cases:

- BuildState round-trip
- BuildRegistry round-trip
- RegistrarState round-trip
- RedeemEventState round-trip
- XenBurnEventState round-trip
- bigint decimal string encoding
- invalid bigint string rejection
- duplicate set entry rejection

## Validation

Before commit:

- npm run typecheck: passed
- npm test: passed
- 18 test files passed
- 103 tests passed

## Current known exclusions

This milestone does not implement:

- file storage adapter
- atomic file write
- snapshot load / save
- migration helpers
- corrupted snapshot recovery
- API / CLI integration
- proof persistence
- watcher persistence

## Main invariant

Serialization must preserve the exact meaning of the in-memory MVP state.

It must not create, remove, or reinterpret accounting values.
