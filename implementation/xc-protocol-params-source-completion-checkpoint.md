# XC protocol params source completion checkpoint

This document closes the XC protocol params source milestone.

This checkpoint is documentation-only.

No runtime code is changed in this milestone.

No dependencies are changed in this milestone.

No real RPC is executed in this milestone.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Completed chain

The XC protocol params source milestone completed the full progression:

1. protocol params source design
2. protocol params source design review
3. mocked/tested source implementation
4. implementation review
5. merge to main

## Current main status

Latest completed main milestone:

    main -> 1af1ff8 Merge branch 'xc-protocol-params-source-review'

Final validation after merge:

- npm run typecheck passed
- npm test passed: 38 test files, 296 tests
- npm run build passed
- npm audit --audit-level=moderate found 0 vulnerabilities

Safe refusal without env remained active:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

## Implemented source

Runtime source:

    src/ethereum/xc-protocol-params-source.ts

Tests:

    tests/xc-protocol-params-source.test.ts

Export update:

    src/index.ts

The implementation added:

- XcProtocolParamsReadProvider
- XcProtocolParams
- XcProtocolParamsSource
- XcProtocolParamsSourceConfig
- createXcProtocolParamsSourceFromEthereumReadProvider()
- normalizeXcProtocolParams()

## Read path

The source models the deployed xEnchanted Crypto Lens read path:

    getProtocolParams()

The source does not model deployed XC Lens as:

    epochMinimum(uint256)

This preserves the separation between:

- XcProtocolParamsSource
- XcEpochMinimumSource

## Source boundary

The source uses an injected minimal read provider with only:

    readContract()

The source does not require:

- getChainId
- getBlock
- real RPC construction
- viem
- ethers
- process.env
- public client construction
- private keys
- mnemonics
- wallet clients
- writeContract
- sendTransaction

## Normalization behavior

The source normalizes:

- object-like tuple returns
- array-like tuple returns

Large values remain bigint.

Small safe values are converted to number:

- maxLevel
- baseAprBpsNow

The full deployed protocol params tuple is preserved:

- genesisTs
- halvingInterval
- xenBurnHalvingInterval
- currentEpoch
- nextHalvingTs
- initialNominal
- currentBaseNominal
- initialXenBurn
- currentXenBurnAmount
- enchantMultiplier
- maxLevel
- baseAprBpsNow
- bpsDenom
- earlyPenaltyBps
- maxWalletNfts

## Test coverage

The implementation tests cover:

- readContract call path
- Lens address passthrough
- getProtocolParams function name
- empty args
- minimal ABI fragment
- object-like tuple normalization
- array-like tuple normalization
- numeric string normalization
- invalid Lens address
- missing tuple fields
- malformed tuple shape
- invalid number field
- provider read error wrapping

## Safety conclusion

No RPC URL was committed.

No API key was committed.

No private key was committed.

No mnemonic or seed phrase was committed.

No `.env` content or raw environment content was committed.

No real RPC execution was added to the source milestone.

## Recommended next milestone

Recommended next design milestone:

    xc-protocol-params-build-validation-design

Purpose:

- decide how X1 Build validation should consume XcProtocolParamsSource
- define which protocol params are authoritative for Build validation
- connect currentBaseNominal and currentXenBurnAmount to X1 Build requirements
- keep validation deterministic
- avoid hardcoded XC economic parameters where the Lens can be source of truth

Likely authoritative fields:

- currentEpoch
- currentBaseNominal
- currentXenBurnAmount
- halvingInterval
- xenBurnHalvingInterval
- nextHalvingTs
- genesisTs

## Decision

The XC protocol params source milestone is complete.

Next step should be design-only:

    xc-protocol-params-build-validation-design
