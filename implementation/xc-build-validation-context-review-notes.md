# XC Build validation context review

This document reviews the XC Build validation context implementation milestone.

Reviewed branch:

    xc-build-validation-context-review

Reviewed implementation milestone:

    xc-build-validation-context

Reviewed files:

- src/model/xc-build-validation-context.ts
- tests/xc-build-validation-context.test.ts
- src/index.ts

## Review summary

The XC Build validation context implementation is accepted.

The implementation adds a pure context helper that combines already-loaded `XcProtocolParams` with derived XC Build requirements.

The helper does not call real RPC.

The helper does not call `XcProtocolParamsSource`.

The helper does not read process.env.

The helper does not modify app, registrar, watcher, or proof submission behavior.

## Implementation files

New runtime source:

    src/model/xc-build-validation-context.ts

New tests:

    tests/xc-build-validation-context.test.ts

Export update:

    src/index.ts

## Source behavior review

The implementation exposes:

- XcBuildValidationContext
- CreateXcBuildValidationContextInput
- createXcBuildValidationContextFromProtocolParams()

The context shape is intentionally minimal:

    {
      protocolParams,
      requirements
    }

The implementation preserves the original protocol params reference and derives requirements through:

    deriveCurrentXcBuildRequirements()

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
- XcProtocolParamsSource usage inside pure model code
- appApply integration
- applyRegistrar integration
- WatcherCandidate integration

This preserves the intended pure model boundary.

## Test coverage review

Tests cover:

- creates validation context from protocol params
- includes derived requirements
- keeps original protocol params snapshot/reference
- preserves currentEpoch
- preserves requiredXntdLockMinimum
- preserves requiredForgeMinimum
- rejects invalid protocol params through existing requirement derivation
- does not call real RPC

Test count increased from:

    39 files / 309 tests

to:

    40 files / 316 tests

## Diff review

Diff from pre-implementation baseline to current HEAD shows only:

- src/model/xc-build-validation-context.ts
- tests/xc-build-validation-context.test.ts
- src/index.ts

No package dependency changed.

No script changed.

No real RPC script was added.

No app, registrar, watcher, or proof-submission runtime behavior changed.

## Validation baseline

Review baseline:

- npm run typecheck passed
- npm test passed: 40 test files, 316 tests
- npm run build passed
- npm audit --audit-level=moderate found 0 vulnerabilities

Manual refusal without env remained safe:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

## Review decision

The XC Build validation context implementation is accepted.

No changes are required before merging this review checkpoint.

Recommended next step after merge:

    complete XC Build validation context milestone
