# Authoritative XC Epoch Minimum Mocked Source Adapter Notes

## Branch

authoritative-xc-epoch-minimum-mocked-source-adapter

## Purpose

This milestone adds a mocked / production-shaped XC epoch minimum source adapter.

It is the first small runtime step after the production source adapter design.

It does not add real Ethereum RPC, ABI integration, CLI integration, snapshot changes, or persistent app-state source ownership.

## Runtime changes

Updated:

- src/model/xc-epoch-minimum-source.ts

Added:

- XcEpochMinimumRecord
- createXcEpochMinimumSourceFromRecords()

The adapter converts validated epoch minimum records into an XcEpochMinimumSource.

## Record shape

XcEpochMinimumRecord includes:

- lockEpoch
- minimumXntd
- observedAt
- sourceChainId
- sourceBlockNumber
- sourceBlockHash

Only lockEpoch and minimumXntd are used by the source map.

The other fields are production-shaped metadata for future source adapters.

## Validation rules

The mocked adapter validates:

- lockEpoch must be an integer
- lockEpoch must be >= 0
- minimumXntd must be > 0
- duplicate records for the same epoch are allowed only when minimumXntd matches
- duplicate records for the same epoch with conflicting minimums are rejected

Missing epochs return null through authoritativeEpochMinimum().

## Error behavior

Invalid records currently throw BuildError with:

- InvalidXntdLockAmount

This keeps the first adapter layer small and avoids expanding the error enum before the source policy stabilizes.

A later hardening layer may introduce a dedicated invalid-source-record error code if needed.

## Test coverage

Updated:

- tests/xc-epoch-minimum-source.test.ts

Added coverage:

- builds XcEpochMinimumSource from production-shaped records
- returns null for missing epoch
- allows duplicate records when minimums match
- rejects conflicting duplicate records
- rejects zero minimum
- rejects negative epoch

## Scope boundary

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

## Validation

After this milestone:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 30 test files passed
- 198 tests passed

## Next step

The next layer should update the checkpoint and then merge this branch.

After merge, a possible next runtime hardening step is to add a dedicated error code for invalid XC epoch minimum source records, but only if it is useful enough to justify expanding the error model.
