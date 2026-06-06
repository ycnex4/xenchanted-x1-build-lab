# XC protocol params source review

This document reviews the XC protocol params source implementation milestone.

Reviewed branch:

    xc-protocol-params-source-review

Reviewed implementation milestone:

    xc-protocol-params-source

Reviewed files:

- src/ethereum/xc-protocol-params-source.ts
- tests/xc-protocol-params-source.test.ts
- src/index.ts

## Review summary

The XC protocol params source implementation is accepted.

The implementation adds a reusable mocked/tested source for reading deployed xEnchanted Crypto protocol params through:

    getProtocolParams()

The implementation correctly keeps this source separate from the existing epoch minimum abstraction.

## Implementation files

New runtime source:

    src/ethereum/xc-protocol-params-source.ts

New tests:

    tests/xc-protocol-params-source.test.ts

Export update:

    src/index.ts

## Source behavior review

The source exposes:

- XcProtocolParamsReadProvider
- XcProtocolParams
- XcProtocolParamsSource
- XcProtocolParamsSourceConfig
- createXcProtocolParamsSourceFromEthereumReadProvider()
- normalizeXcProtocolParams()

The source reads protocol params through an injected provider with only:

    readContract()

This is better than depending on a broader EthereumReadProvider because the protocol params source does not need getChainId or getBlock.

The source calls:

    getProtocolParams()

with a minimal ABI fragment.

## Normalization review

The source normalizes both:

- object-like tuple returns with named fields
- array-like tuple returns ordered by ABI components

Large integer values remain bigint.

Small safe fields are converted to number:

- maxLevel
- baseAprBpsNow

The full protocol params tuple is preserved:

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

## Error handling review

The source validates Lens address format before reads.

Invalid address error is sanitized:

    Invalid XC protocol params source config: lensAddress

Malformed tuple errors are sanitized.

Missing field errors are sanitized.

Invalid field errors are sanitized.

Provider read errors are wrapped as:

    Failed to read XC protocol params

The implementation does not expose raw provider internals in wrapped provider errors.

## Test coverage review

Tests cover:

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

Test count increased from:

    37 files / 286 tests

to:

    38 files / 296 tests

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

- src/ethereum/xc-protocol-params-source.ts
- tests/xc-protocol-params-source.test.ts
- src/index.ts
- package.json

## Diff review

Diff from pre-implementation baseline to current HEAD shows only:

- src/ethereum/xc-protocol-params-source.ts
- tests/xc-protocol-params-source.test.ts
- src/index.ts

No package dependency changed.

No script changed.

No real RPC script was added.

## Validation baseline

Review baseline:

- npm run typecheck passed
- npm test passed: 38 test files, 296 tests
- npm run build passed
- npm audit --audit-level=moderate found 0 vulnerabilities

Manual refusal without env remained safe:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

## Review decision

The XC protocol params source implementation is accepted.

No changes are required before merging this review checkpoint.

Recommended next step after merge:

    complete XC protocol params source milestone
