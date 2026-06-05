# Authoritative XC Epoch Minimum Chain Review Notes

## Branch

review-authoritative-xc-epoch-minimum-chain

## Purpose

This milestone reviews the completed authoritative XC epoch minimum validation chain after the source/helper, registrar, app-service, proof-submission, and e2e scenario milestones.

This is a review-only milestone.

It does not change runtime code.

## Current runtime chain

The deterministic authoritative XC epoch minimum validation chain is now available through:

watcher candidate
-> proof conversion
-> appSubmitProof(..., xcEpochMinimumSource)
-> appApplyRegistrarXntdLock() / appApplyRegistrarXntdRelock()
-> applyRegistrarXntdLock() / applyRegistrarXntdRelock()
-> assertAuthoritativeXcEpochMinimum()
-> Build state

## Current validation rule

When xcEpochMinimumSource is provided, XNTD lock / relock validates:

observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

The source is optional at the integration boundary.

If provided, validation is enforced.

If not provided, compatibility behavior is preserved for existing call sites.

## Mutation safety

Authoritative validation occurs before:

- acceptRegistrarMessage()
- acceptXntdCommitmentEvent()
- lockXntd() / relockXntd()

Rejected authoritative validation does not mutate:

- registrar.processedMessages
- xntdCommitmentEvents.usedXntdCommitmentEvents
- Build lock state

## Review findings

The review grep found historical references to older stages, including:

- requiredXntdLock = amountXntd
- observedRequiredXntdLock = amountXntd
- fallback behavior
- trusted registrar MVP language

These references are mostly in historical implementation notes and older checkpoint sections.

They should remain as history unless they appear in active current-state docs.

## Active runtime status

Active runtime code no longer relies on the old equality model as the source of truth for requiredXntdLock.

Current flow separates:

- lockedXntd = amountXntd
- requiredXntdLock = observedRequiredXntdLock

And, when a source is provided, validates:

- observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

## Active docs status

The active docs already describe the current state correctly:

- docs/assumptions.md
- docs/registrar/xntd-lock-epoch-minimum-validation.md
- docs/registrar/authoritative-xc-state-source.md
- docs/checkpoints/current-design-checkpoint.md latest sections

No active doc update is required in this review layer.

## Test coverage status

Coverage now includes:

- source/helper unit tests
- registrar lock validation tests
- app-service source injection tests
- proof-submission source injection tests
- e2e watcher-proof-registrar scenario with deterministic source

Current validation count:

- 30 test files passed
- 194 tests passed

## Scope boundary

This review does not introduce:

- real Ethereum RPC integration
- XC Core / Lens ABI integration
- snapshot schema changes
- storage serialization changes
- CLI changes
- persistent app-state source ownership

## Next production-readiness step

The next meaningful step is not another propagation layer.

The next step should be a design/implementation decision for how the authoritative source is created in real integration:

- trusted integration source
- finalized Ethereum RPC / Lens read
- checkpoint source
- bridge-provided source
- X1-native verified source

Until that decision is made, the deterministic source remains the correct test/runtime boundary.
