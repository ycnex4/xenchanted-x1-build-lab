# XC protocol params build validation design review

This document reviews the XC protocol params build validation design milestone.

Reviewed branch:

    xc-protocol-params-build-validation-design-review

Reviewed design milestone:

    xc-protocol-params-build-validation-design

Reviewed files:

- implementation/xc-protocol-params-build-validation-design.md
- docs/checkpoints/current-design-checkpoint.md

## Review summary

The XC protocol params build validation design is accepted.

The design correctly defines `XcProtocolParams` as the authoritative XC economic context for future X1 Build validation.

The design correctly keeps protocol params context validation separate from user action proof validation.

The design correctly requires the first implementation to remain pure and mocked.

## Design-only boundary review

The reviewed milestone is design-only.

Diff from pre-design baseline to current HEAD shows only:

- docs/checkpoints/current-design-checkpoint.md
- implementation/xc-protocol-params-build-validation-design.md

No runtime code changed.

No tests changed.

No package dependency changed.

No script changed.

No real RPC was executed in the design milestone.

## Protocol params source review

The design correctly relies on the already implemented:

    XcProtocolParamsSource

The deployed XC Lens read path remains:

    getProtocolParams()

The design does not reintroduce:

    epochMinimum(uint256)

as the deployed XC Lens read path.

## Validation architecture review

The design correctly says future validation should accept already-loaded protocol params rather than calling RPC internally.

This keeps validation deterministic and testable.

The design also correctly separates:

- protocol params context
- proof validation
- event replay
- registrar logic
- watcher proof pipelines

Protocol params are context, not proof of user action.

## Authoritative fields review

The design correctly identifies likely authoritative fields:

- currentEpoch
- currentBaseNominal
- currentXenBurnAmount
- halvingInterval
- xenBurnHalvingInterval
- nextHalvingTs
- genesisTs

These are appropriate inputs for future epoch-aware X1 Build validation.

## Build requirement direction review

The design correctly points toward deriving requirements from protocol params, especially:

- currentBaseNominal as current XC base reference
- currentXenBurnAmount as current XEN burn context
- currentEpoch as current epoch context

The design intentionally does not finalize all economic formulas.

That is correct. Formula decisions should be a later validation rules milestone.

## XNTD lock direction review

The design preserves the earlier direction that XNTD lock minimum should be tied to the current epoch minimum, not a fixed 100 XNTD.

The design suggests:

    requiredXntdLockMinimum = currentBaseNominal

or a multiplier over currentBaseNominal.

The exact multiplier is intentionally left for a later milestone.

This is accepted.

## Boundary review

The future first implementation should not add:

- real RPC execution
- viem imports
- ethers imports
- createPublicClient
- http transport
- process.env reads
- private key support
- mnemonic support
- wallet client support
- writeContract
- sendTransaction
- bridge logic
- transaction logic

The first implementation should be pure and mocked.

## Error policy review

The design correctly requires sanitized validation errors.

Allowed style:

    Invalid XC protocol params build validation: missing currentBaseNominal
    Invalid XC build requirement: currentBaseNominal must be positive
    XC build validation failed: stale protocol params

Forbidden content remains:

- RPC URL
- API key
- raw provider internals
- raw env
- transport config
- authorization headers

## Grep review

The review grep found architecture and boundary terms inside design/checkpoint text only.

That is expected.

No runtime files were added or changed by this design milestone.

## Validation baseline

Review baseline:

- npm run typecheck passed
- npm test passed: 38 test files, 296 tests
- npm run build passed
- npm audit --audit-level=moderate found 0 vulnerabilities

Manual refusal without env remained safe:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

## Review decision

The XC protocol params build validation design is accepted.

No design changes are required before merging this review checkpoint.

Recommended next milestone after merge:

    xc-protocol-params-build-validation
