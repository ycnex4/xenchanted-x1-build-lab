# Authoritative XC Epoch Minimum App Service Injection Notes

## Branch

authoritative-xc-epoch-minimum-app-service-injection

## Purpose

This milestone passes the optional authoritative XC epoch minimum source through the application service XNTD lock / relock wrappers.

It extends the registrar-level authoritative validation hook upward by one layer without changing proof submission, snapshots, CLI, or persisted app state.

## Runtime change

Updated:

- src/app/build-service.ts

The following app service functions now forward xcEpochMinimumSource when provided:

- appApplyRegistrarXntdLock()
- appApplyRegistrarXntdRelock()

Forwarding uses conditional object spread:

- if xcEpochMinimumSource is provided, it is passed to the registrar handler
- if xcEpochMinimumSource is undefined, the field is omitted

This preserves compatibility with exactOptionalPropertyTypes.

## Validation behavior

When a caller provides xcEpochMinimumSource:

appApplyRegistrarXntdLock()
-> applyRegistrarXntdLock()
-> assertAuthoritativeXcEpochMinimum()

and:

appApplyRegistrarXntdRelock()
-> applyRegistrarXntdRelock()
-> assertAuthoritativeXcEpochMinimum()

Therefore app-service callers can now trigger registrar-level validation of:

observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

## Compatibility behavior

xcEpochMinimumSource remains optional.

Existing app-service call sites that do not pass the source continue to work.

## Test coverage

Updated:

- tests/app-build-service.test.ts

Added coverage:

- appApplyRegistrarXntdLock() succeeds when the source contains the matching epoch minimum
- appApplyRegistrarXntdRelock() returns a structured error when the source is missing the relock epoch minimum
- rejected app-service relock does not mark the registrar message as processed
- rejected app-service relock does not mark the XNTD commitment event key as used
- rejected app-service relock does not mutate Build lockedXntd, requiredXntdLock, or lockEpoch

## Scope boundary

This milestone does not update:

- appSubmitProof()
- proof submission payload flow
- watcher proof conversion
- registrar payload builders
- snapshot schema
- storage serialization
- CLI output
- real Ethereum RPC integration
- XC Core / Lens ABI integration

## Next step

The next layer should decide how proof submission receives / owns the authoritative XC epoch minimum source.

Likely next step:

- add optional xcEpochMinimumSource to appSubmitProof() input
- pass it into appApplyRegistrarXntdLock() / appApplyRegistrarXntdRelock()
- add app-proof-submission tests

Do not persist the source in snapshots yet.

## Validation

After this milestone:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 30 test files passed
- 193 tests passed
