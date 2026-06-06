# XC Build validation app service context completion checkpoint

This document closes the XC Build validation app service context milestone.

This checkpoint is documentation-only.

No runtime code is changed in this milestone.

No dependencies are changed in this milestone.

No real RPC is executed in this milestone.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Completed chain

The XC Build validation app service context milestone completed the full progression:

1. app service context design
2. app service context design review
3. backwards-compatible runtime implementation
4. implementation review
5. merge to main

## Current main status

Latest completed main milestone:

    main -> edeea95 Merge branch 'xc-build-validation-app-service-context-review'

Final validation after merge:

- npm run typecheck passed
- npm test passed: 40 test files, 317 tests
- npm run build passed
- npm audit --audit-level=moderate found 0 vulnerabilities

Safe refusal without env remained active:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

## Implemented source

Runtime source:

    src/app/proof-submission.ts

Tests:

    tests/app-proof-submission.test.ts

## Runtime input

`AppSubmitProofInput` now accepts:

    xcBuildValidationContext?: XcBuildValidationContext

The field is optional.

Existing callers without `xcBuildValidationContext` remain valid.

## Behavior boundary

This milestone intentionally only wires context into the app proof submission input.

It does not enforce Build validity rules yet.

It does not use context to reject proofs.

It does not compare proof values against context requirements.

It does not modify watcher behavior.

It does not modify registrar behavior.

It does not modify proof payload behavior.

## Safety boundary

The implementation does not add:

- real RPC execution
- XcProtocolParamsSource usage
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
- package dependency changes
- scripts

## Test coverage

The implementation adds a test proving that app proof submission accepts `XcBuildValidationContext` without changing Core redeem behavior.

The test verifies:

- context can be created from protocol params
- context can be passed into appSubmitProof()
- Core redeem proof submission still succeeds
- Build history / available BLD behavior remains unchanged
- context protocol params reference is preserved
- derived requiredXntdLockMinimum remains available
- registrar processed message count remains expected

Test count increased from:

    40 files / 316 tests

to:

    40 files / 317 tests

## Completed pipeline

The protocol-context pipeline now exists in a safe, staged form:

    XcProtocolParams
    -> deriveCurrentXcBuildRequirements()
    -> XcBuildValidationContext
    -> optional xcBuildValidationContext in appSubmitProof()

This is context wiring only.

Enforcement remains a later milestone.

## Recommended next milestone

Recommended next design milestone:

    xc-build-validation-epoch-policy-design

Purpose:

- decide how current XC epoch context should relate to historical proofs
- decide whether Core redeem proof may be historical
- decide whether Forge participation must be current epoch or any epoch
- decide how lock/relock should behave across epoch changes
- avoid accidental rejection of valid historical participation
- define which future enforcement checks are allowed to use current context

## Decision

The XC Build validation app service context milestone is complete.

Next step should be design-only:

    xc-build-validation-epoch-policy-design
