# Invalid XC Epoch Minimum Record Error Notes

## Branch

invalid-xc-epoch-minimum-record-error

## Purpose

This milestone separates XC epoch minimum source-record validation errors from XNTD lock amount validation errors.

The mocked source adapter previously reused:

InvalidXntdLockAmount

for invalid XC epoch minimum records.

That worked mechanically, but it mixed two different error domains:

- user XNTD lock / relock amount validation
- authoritative XC epoch minimum source record validation

## Runtime changes

Updated:

- src/errors/build-error.ts
- src/model/xc-epoch-minimum-source.ts

Added BuildErrorCode:

- InvalidXcEpochMinimumRecord

The source adapter now uses InvalidXcEpochMinimumRecord for:

- invalid lockEpoch
- zero / negative minimumXntd
- conflicting duplicate epoch minimum records

## Error model after this milestone

XNTD lock amount errors:

- InvalidXntdLockAmount

Source availability errors:

- MissingAuthoritativeXcEpochMinimum

Source mismatch errors:

- MismatchedAuthoritativeXcEpochMinimum

Source record construction errors:

- InvalidXcEpochMinimumRecord

## Test coverage

Updated:

- tests/xc-epoch-minimum-source.test.ts

Added / strengthened checks that invalid source records throw:

- InvalidXcEpochMinimumRecord

Covered cases:

- conflicting duplicate epoch records
- zero minimumXntd
- negative lockEpoch

## Scope boundary

This milestone does not change:

- authoritative validation flow
- appSubmitProof()
- app service
- registrar handlers
- snapshot schema
- storage serialization
- CLI output
- real Ethereum RPC integration
- XC Core / Lens ABI integration

## Validation

After this milestone:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 30 test files passed
- 198 tests passed
