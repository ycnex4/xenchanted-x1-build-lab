# Authoritative XC Epoch Minimum Proof Submission Injection Notes

## Branch

authoritative-xc-epoch-minimum-proof-submission-injection

## Purpose

This milestone passes the optional authoritative XC epoch minimum source through appSubmitProof() for XNTD lock / relock proof submission.

It extends authoritative XC epoch minimum validation upward from the app service layer into the proof submission flow.

## Runtime change

Updated:

- src/app/proof-submission.ts

AppSubmitProofInput now supports:

- xcEpochMinimumSource?: XcEpochMinimumSource

For XNTD proof kinds:

- XNTD_LOCK_PROOF
- XNTD_RELOCK_PROOF

appSubmitProof() forwards xcEpochMinimumSource into:

- appApplyRegistrarXntdLock()
- appApplyRegistrarXntdRelock()

Forwarding uses conditional object spread:

- if xcEpochMinimumSource is provided, it is passed down
- if xcEpochMinimumSource is undefined, the field is omitted

This preserves compatibility with exactOptionalPropertyTypes.

## Validation behavior

When proof submission receives xcEpochMinimumSource:

appSubmitProof()
-> appApplyRegistrarXntdLock() / appApplyRegistrarXntdRelock()
-> applyRegistrarXntdLock() / applyRegistrarXntdRelock()
-> assertAuthoritativeXcEpochMinimum()

Therefore proof submission callers can now trigger validation of:

observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

## Compatibility behavior

xcEpochMinimumSource remains optional.

Existing proof submission call sites that do not pass the source continue to work.

## Test coverage

Updated:

- tests/app-proof-submission.test.ts

Added coverage:

- appSubmitProof() accepts an XNTD_LOCK_PROOF when the source contains the matching epoch minimum
- appSubmitProof() rejects an XNTD_RELOCK_PROOF when the source is missing the relock epoch minimum
- rejected relock proof submission returns a structured app error
- rejected relock proof submission does not mark the registrar message as processed
- rejected relock proof submission does not mark the XNTD commitment event key as used
- rejected relock proof submission does not mutate Build lockedXntd, requiredXntdLock, or lockEpoch

## Scope boundary

This milestone does not update:

- watcher proof conversion
- registrar payload builders
- snapshot schema
- storage serialization
- CLI output
- real Ethereum RPC integration
- XC Core / Lens ABI integration
- persistent app state source ownership

## Current validation chain

The optional authoritative source can now flow through:

appSubmitProof()
-> appApplyRegistrarXntdLock() / appApplyRegistrarXntdRelock()
-> applyRegistrarXntdLock() / applyRegistrarXntdRelock()
-> assertAuthoritativeXcEpochMinimum()

## Next step

The next layer should decide whether to add the authoritative source to the e2e watcher-proof-registrar scenario.

Potential next step:

- add source injection to tests/e2e-watcher-proof-registrar-scenario.test.ts
- keep it test-only
- do not persist the source in snapshots
- do not introduce real RPC yet

## Validation

After this milestone:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 30 test files passed
- 194 tests passed
