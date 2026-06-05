# Authoritative XC Epoch Minimum Stage Summary

## Branch

authoritative-xc-epoch-minimum-stage-summary

## Purpose

This document summarizes the completed authoritative XC epoch minimum validation stage.

It is a summary-only milestone.

It does not change runtime code.

## Completed line

The authoritative XC epoch minimum validation work completed the following sequence:

1. observedRequiredXntdLock propagation
2. fallback cleanup
3. authoritative source/helper
4. registrar validation
5. app service source injection
6. proof submission source injection
7. e2e watcher-proof-registrar coverage
8. chain review
9. production source plan

## Current validation chain

The deterministic validation chain is now covered through:

watcher candidate
-> proof conversion
-> appSubmitProof(..., xcEpochMinimumSource)
-> appApplyRegistrarXntdLock() / appApplyRegistrarXntdRelock()
-> applyRegistrarXntdLock() / applyRegistrarXntdRelock()
-> assertAuthoritativeXcEpochMinimum()
-> Build state

## Current runtime rule

When xcEpochMinimumSource is provided, XNTD lock / relock validates:

observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

Current state separation:

lockedXntd = amountXntd
requiredXntdLock = observedRequiredXntdLock

## Current source interface

XcEpochMinimumSource {
  authoritativeEpochMinimum(lockEpoch: number): bigint | null;
}

Current helper:

createStaticXcEpochMinimumSource()

Current assertion:

assertAuthoritativeXcEpochMinimum()

## Error model

Current explicit errors:

- MissingAuthoritativeXcEpochMinimum
- MismatchedAuthoritativeXcEpochMinimum

These are separate from InvalidXntdLockAmount.

Reason:

- InvalidXntdLockAmount covers invalid amount relationships.
- MissingAuthoritativeXcEpochMinimum covers unavailable source state.
- MismatchedAuthoritativeXcEpochMinimum covers economically incorrect observed required lock values.

## Mutation safety

Authoritative validation runs before:

- acceptRegistrarMessage()
- acceptXntdCommitmentEvent()
- lockXntd() / relockXntd()

Rejected validation does not mutate:

- registrar.processedMessages
- xntdCommitmentEvents.usedXntdCommitmentEvents
- Build lock state

## Test coverage

Coverage now includes:

- source/helper unit tests
- registrar lock validation tests
- app-service source injection tests
- proof-submission source injection tests
- e2e watcher-proof-registrar scenario with deterministic source

Current validation count:

- 30 test files passed
- 194 tests passed

## Compatibility boundary

xcEpochMinimumSource remains optional.

If provided:

- authoritative validation is enforced

If not provided:

- compatibility behavior is preserved for existing call sites

This keeps deterministic validation available without forcing source ownership into app state or snapshots.

## What was intentionally not added

This stage did not add:

- real Ethereum RPC reads
- XC Core / Lens ABI integration
- bridge signer logic
- X1 on-chain verification
- snapshot schema migration
- CLI integration
- persistent app-state source ownership

## Production source conclusion

The deterministic validation mechanics are complete.

The next production-readiness question is source ownership:

Where does authoritativeEpochMinimum(lockEpoch) come from in real integration?

Production source options documented:

1. trusted integration source
2. finalized Ethereum RPC / XC Lens read
3. checkpoint source
4. bridge-provided source
5. X1-native verified source

Recommended first production-like path:

trusted integration source
-> finalized Ethereum RPC / XC Lens read or checkpoint source

Long-term path:

bridge-provided source or X1-native verified source

## Recommended next stage

The next stage should be:

authoritative-xc-epoch-minimum-production-source-adapter-design

Scope:

- define finalized block policy
- define XC Core / Lens read fields
- define adapter interface
- define mocked adapter tests
- define failure behavior for stale / missing / mismatched source
- do not add real RPC yet
- do not change snapshot schema yet

## Current conclusion

The authoritative XC epoch minimum validation chain is now ready as a deterministic runtime boundary.

The project can safely move from validation mechanics to production source design.
