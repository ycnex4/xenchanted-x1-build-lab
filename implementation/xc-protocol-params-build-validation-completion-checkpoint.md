# XC protocol params build validation completion checkpoint

This document closes the XC protocol params build validation milestone.

This checkpoint is documentation-only.

No runtime code is changed in this milestone.

No dependencies are changed in this milestone.

No real RPC is executed in this milestone.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Completed chain

The XC protocol params build validation milestone completed the full progression:

1. build validation design
2. build validation design review
3. pure/mocked implementation
4. implementation review
5. merge to main

## Current main status

Latest completed main milestone:

    main -> 8fe5833 Merge branch 'xc-protocol-params-build-validation-review'

Final validation after merge:

- npm run typecheck passed
- npm test passed: 39 test files, 309 tests
- npm run build passed
- npm audit --audit-level=moderate found 0 vulnerabilities

Safe refusal without env remained active:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

## Implemented source

Runtime source:

    src/model/xc-protocol-params-build-validation.ts

Tests:

    tests/xc-protocol-params-build-validation.test.ts

Export update:

    src/index.ts

The implementation added:

- XcBuildRequirementsFromProtocolParams
- XcProtocolParamsBuildValidationInput
- XcBuildProtocolParamsValidationResult
- deriveCurrentXcBuildRequirements()
- validateXcBuildAgainstProtocolParams()

## Requirement derivation

The implementation derives current XC Build requirements from `XcProtocolParams`.

Current derivation:

    requiredBaseNominal = currentBaseNominal
    requiredXenBurnAmount = currentXenBurnAmount
    requiredXntdLockMinimum = currentBaseNominal
    requiredForgeMinimum = currentBaseNominal * 5

The implementation also preserves:

- currentEpoch
- nextHalvingTs
- genesisTs
- halvingInterval
- xenBurnHalvingInterval

## Boundary

The helper is pure and mocked.

It does not:

- call real RPC
- import viem
- import ethers
- create public clients
- use http transport
- read process.env
- accept private keys
- accept mnemonics
- create wallet clients
- call writeContract
- call sendTransaction

## Test coverage

The implementation tests cover:

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

## Safety conclusion

No RPC URL was committed.

No API key was committed.

No private key was committed.

No mnemonic or seed phrase was committed.

No `.env` content or raw environment content was committed.

No real RPC execution was added to this milestone.

## Recommended next milestone

Recommended next design milestone:

    xc-build-validation-integration-design

Purpose:

- decide where `deriveCurrentXcBuildRequirements()` should be used in the broader Build validation flow
- connect protocol params derived requirements to existing proof / registrar / app service logic
- preserve separation between protocol context and user action proof
- avoid premature real RPC integration
- keep the next step design-only before wiring into runtime flows

## Decision

The XC protocol params build validation milestone is complete.

Next step should be design-only:

    xc-build-validation-integration-design
