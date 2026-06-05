# XC Epoch Minimum Record Validation Hardening Notes

## Branch

xc-epoch-minimum-record-validation-hardening

## Purpose

This milestone hardens validation for production-shaped XC epoch minimum records.

It builds on the mocked XC epoch minimum source adapter and the dedicated InvalidXcEpochMinimumRecord error.

## Runtime changes

Updated:

- src/model/xc-epoch-minimum-source.ts

The record validator now rejects:

- observedAt <= 0
- sourceBlockNumber <= 0 when sourceBlockNumber is provided

Existing validation remains:

- lockEpoch must be an integer
- lockEpoch must be >= 0
- minimumXntd must be > 0
- duplicate records for the same epoch may only repeat the same minimum
- conflicting duplicate epoch minimums are rejected

## Test coverage

Updated:

- tests/xc-epoch-minimum-source.test.ts

Added coverage:

- rejects observedAt = 0
- rejects sourceBlockNumber = 0 when provided
- verifies both cases use InvalidXcEpochMinimumRecord

Test count changed:

- 198 tests -> 199 tests

## Intentional boundary

This milestone does not validate sourceBlockHash format.

Reason:

sourceBlockHash format requirements should be decided together with the future source adapter policy.

For now, the adapter remains production-shaped but network-agnostic.

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
- 199 tests passed
