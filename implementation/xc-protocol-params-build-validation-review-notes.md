# XC protocol params build validation review

This document reviews the XC protocol params build validation implementation milestone.

Reviewed branch:

    xc-protocol-params-build-validation-review

Reviewed implementation milestone:

    xc-protocol-params-build-validation

Reviewed files:

- src/model/xc-protocol-params-build-validation.ts
- tests/xc-protocol-params-build-validation.test.ts
- src/index.ts

## Review summary

The XC protocol params build validation implementation is accepted.

The implementation adds a pure mocked helper that derives current XC Build requirements from `XcProtocolParams`.

The helper does not call real RPC.

The helper does not import viem or ethers.

The helper does not read process.env.

The helper does not add wallet or transaction paths.

## Implementation files

New runtime source:

    src/model/xc-protocol-params-build-validation.ts

New tests:

    tests/xc-protocol-params-build-validation.test.ts

Export update:

    src/index.ts

## Source behavior review

The implementation exposes:

- XcBuildRequirementsFromProtocolParams
- XcProtocolParamsBuildValidationInput
- XcBuildProtocolParamsValidationResult
- deriveCurrentXcBuildRequirements()
- validateXcBuildAgainstProtocolParams()

The implementation derives:

- currentEpoch
- requiredBaseNominal
- requiredXenBurnAmount
- requiredXntdLockMinimum
- requiredForgeMinimum
- nextHalvingTs
- genesisTs
- halvingInterval
- xenBurnHalvingInterval

## Requirement derivation review

The implementation uses:

    requiredBaseNominal = currentBaseNominal

    requiredXenBurnAmount = currentXenBurnAmount

    requiredXntdLockMinimum = currentBaseNominal

    requiredForgeMinimum = currentBaseNominal * 5

This matches the current design direction:

- XNTD lock minimum starts tied to current XC base nominal
- Forge minimum follows XC current base nominal based minimum logic
- exact later rule/multiplier changes can be handled in separate milestones if needed

## Validation behavior review

The implementation rejects invalid zero values for:

- currentBaseNominal
- currentXenBurnAmount
- halvingInterval
- xenBurnHalvingInterval

The implementation requires non-negative values for:

- currentEpoch
- nextHalvingTs
- genesisTs

Errors are sanitized and do not include provider internals or secret-bearing data.

## Test coverage review

Tests cover:

- deriving current epoch
- deriving current base nominal requirement
- deriving current XEN burn amount requirement
- deriving XNTD lock minimum from current base nominal
- deriving Forge minimum from current base nominal
- preserving epoch timing metadata
- handling later epochs
- positive validation result
- rejecting zero currentBaseNominal
- rejecting zero currentXenBurnAmount
- rejecting zero halvingInterval
- rejecting zero xenBurnHalvingInterval
- no real RPC call path

Test count increased from:

    38 files / 296 tests

to:

    39 files / 309 tests

## Boundary review

The implementation does not add:

- real RPC execution
- viem import
- ethers import
- createPublicClient
- http transport
- process.env read
- private key support
- mnemonic support
- wallet client support
- writeContract
- sendTransaction

The review grep found no dangerous matches in:

- src/model/xc-protocol-params-build-validation.ts
- tests/xc-protocol-params-build-validation.test.ts
- src/index.ts
- package.json

## Diff review

Diff from pre-implementation baseline to current HEAD shows only:

- src/model/xc-protocol-params-build-validation.ts
- tests/xc-protocol-params-build-validation.test.ts
- src/index.ts

No package dependency changed.

No script changed.

No real RPC script was added.

## Validation baseline

Review baseline:

- npm run typecheck passed
- npm test passed: 39 test files, 309 tests
- npm run build passed
- npm audit --audit-level=moderate found 0 vulnerabilities

Manual refusal without env remained safe:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

## Review decision

The XC protocol params build validation implementation is accepted.

No changes are required before merging this review checkpoint.

Recommended next step after merge:

    complete XC protocol params build validation milestone
