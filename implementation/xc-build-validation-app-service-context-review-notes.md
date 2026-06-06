# XC Build validation app service context review

This document reviews the XC Build validation app service context implementation milestone.

Reviewed branch:

    xc-build-validation-app-service-context-review

Reviewed implementation milestone:

    xc-build-validation-app-service-context

Reviewed files:

- src/app/proof-submission.ts
- tests/app-proof-submission.test.ts

## Review summary

The XC Build validation app service context implementation is accepted.

The implementation adds optional `xcBuildValidationContext` support to app proof submission input.

The implementation is backwards-compatible.

The implementation does not add global Build validity enforcement.

The implementation does not call real RPC.

The implementation does not call `XcProtocolParamsSource`.

The implementation does not read process.env.

The implementation does not change watcher, registrar, or proof payload behavior.

## Implementation files

Updated runtime source:

    src/app/proof-submission.ts

Updated tests:

    tests/app-proof-submission.test.ts

## Runtime behavior review

`AppSubmitProofInput` now accepts:

    xcBuildValidationContext?: XcBuildValidationContext

The field is optional.

Existing callers without `xcBuildValidationContext` remain valid.

`appSubmitProof()` does not currently enforce requirements from the context.

This is intentional for this milestone.

## Test coverage review

The implementation adds a test proving that app proof submission accepts `XcBuildValidationContext` without changing Core redeem behavior.

The test verifies:

- context is created from protocol params
- context can be passed into `appSubmitProof()`
- Core redeem submission still succeeds
- Build history/available BLD behavior remains unchanged
- context protocol params reference is preserved
- derived `requiredXntdLockMinimum` remains available
- registrar processed message count remains expected

Test count increased from:

    40 files / 316 tests

to:

    40 files / 317 tests

## Boundary review

The implementation does not add:

- global Build validity enforcement
- currentEpoch enforcement
- requiredForgeMinimum enforcement
- requiredXntdLockMinimum enforcement
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
- XcProtocolParamsSource usage
- watcher candidate changes
- registrar behavior changes
- proof payload changes
- package dependency changes
- scripts

## Grep review

The review grep found only expected matches in tests:

- `currentEpoch` inside the mocked protocol params fixture
- `requiredXntdLockMinimum` inside the context availability assertion

No dangerous runtime matches were found in `appSubmitProof()`.

## Diff review

Diff from pre-implementation baseline to current HEAD shows only:

- src/app/proof-submission.ts
- tests/app-proof-submission.test.ts

No package dependency changed.

No script changed.

No real RPC script was added.

No watcher, registrar, or proof payload file changed.

## Validation baseline

Review baseline:

- npm run typecheck passed
- npm test passed: 40 test files, 317 tests
- npm run build passed
- npm audit --audit-level=moderate found 0 vulnerabilities

Manual refusal without env remained safe:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

## Review decision

The XC Build validation app service context implementation is accepted.

No changes are required before merging this review checkpoint.

Recommended next step after merge:

    complete XC Build validation app service context milestone
