# XC protocol params source design

This document designs a reusable source for reading xEnchanted Crypto protocol parameters from the deployed XC Lens.

This milestone is design-only.

No runtime code is added in this milestone.

No dependencies are changed in this milestone.

No real RPC is executed in this milestone.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Background

The previous RPC smoke milestone proved that the deployed mainnet xEnchanted Crypto NFT Lens is readable through:

    getProtocolParams()

The deployed mainnet xEnchantedNFTLens does not expose:

    epochMinimum(uint256)

Therefore the future reusable source should not model the deployed XC Lens as an `epochMinimum` source.

The correct deployed XC Lens read path is:

    xEnchantedNFTLens.getProtocolParams()

## Mainnet smoke evidence

The sanitized mainnet smoke result confirmed:

    chainId=eip155-1
    providerChainId=1
    lensAddress=0xd4b90d7392c1565d558c80122dee76b5b3bb6c23
    function=getProtocolParams()
    completed=true

The protocol params returned by mainnet were:

    genesisTs=1780166915
    halvingInterval=15552000
    xenBurnHalvingInterval=31104000
    currentEpoch=0
    nextHalvingTs=1795718915
    initialNominal=100000000000000000000
    currentBaseNominal=100000000000000000000
    initialXenBurn=100000000000000000000000000
    currentXenBurnAmount=100000000000000000000000000
    enchantMultiplier=3
    maxLevel=22
    baseAprBpsNow=1000
    bpsDenom=10000
    earlyPenaltyBps=100
    maxWalletNfts=60

## Design goal

Create a reusable XC protocol params source that can read and normalize the `getProtocolParams()` tuple from an injected read-only Ethereum provider.

The future source should be usable by X1 Build logic without hardcoding deployed XC protocol parameters.

The source should remain testable with mocked providers.

The source should not own real RPC.

The source should not read process.env.

The source should not import viem directly unless it stays behind the already-reviewed script/provider boundary.

## Proposed source name

Recommended file:

    src/ethereum/xc-protocol-params-source.ts

Recommended tests:

    tests/xc-protocol-params-source.test.ts

Recommended exported types:

    XcProtocolParams
    XcProtocolParamsSource
    createXcProtocolParamsSourceFromEthereumReadProvider()

## Proposed normalized model

The normalized model should preserve exact integer values as bigint where appropriate.

Recommended TypeScript shape:

    export interface XcProtocolParams {
      readonly genesisTs: bigint;
      readonly halvingInterval: bigint;
      readonly xenBurnHalvingInterval: bigint;
      readonly currentEpoch: bigint;
      readonly nextHalvingTs: bigint;
      readonly initialNominal: bigint;
      readonly currentBaseNominal: bigint;
      readonly initialXenBurn: bigint;
      readonly currentXenBurnAmount: bigint;
      readonly enchantMultiplier: bigint;
      readonly maxLevel: number;
      readonly baseAprBpsNow: number;
      readonly bpsDenom: bigint;
      readonly earlyPenaltyBps: bigint;
      readonly maxWalletNfts: bigint;
    }

Rationale:

- large uint256 values should remain bigint
- uint64 timestamps may also be bigint for consistency
- uint8 maxLevel can safely be number
- uint16 baseAprBpsNow can safely be number

## Proposed source interface

Recommended interface:

    export interface XcProtocolParamsSource {
      readProtocolParams(): Promise<XcProtocolParams>;
    }

The source should expose a single read method first.

Additional convenience methods should be added later only if needed.

## ABI boundary

The source should include the minimal ABI fragment for:

    getProtocolParams()

The ABI should be local to the source or a dedicated ABI module.

The source should not load ABI files from disk in the first implementation.

No ABI path support is needed for the first implementation.

## Ethereum provider boundary

The source should use the existing read-only provider abstraction or existing viem-like wrapper boundary.

The source should not construct a public client.

The source should not own RPC URL.

The source should not read environment variables.

The source should not depend on wallet clients, signers, private keys, or transaction helpers.

Allowed operations:

- readContract

Not needed for this source:

- getBlock
- getChainId

Those may remain in the outer script or provider validation layer.

## Lens address handling

The future source should accept the Lens address as an explicit constructor/config parameter.

Recommended input:

    {
      provider,
      lensAddress
    }

The source should validate the address format before calling readContract, or rely on the existing Ethereum config parser if called from script configuration.

For model-level tests, invalid address handling should be deterministic and sanitized.

## Returned tuple handling

The source should support both common viem return shapes:

1. Object-like tuple with named fields
2. Array-like tuple ordered by ABI components

The implementation should normalize either shape into `XcProtocolParams`.

This avoids brittle coupling to one provider/mock return style.

## Authoritative fields for X1 Build

The next X1 Build design should treat these fields as likely authoritative:

- currentEpoch
- currentBaseNominal
- currentXenBurnAmount
- halvingInterval
- xenBurnHalvingInterval
- nextHalvingTs
- genesisTs

Possible X1 Build uses:

- current epoch validation
- current XC base nominal validation
- current XC XEN burn amount validation
- epoch-aware lock minimum calculation
- UI/source-of-truth display
- replay/snapshot consistency checks

Fields that are useful but may not directly define X1 Build minimums:

- initialNominal
- initialXenBurn
- enchantMultiplier
- maxLevel
- baseAprBpsNow
- bpsDenom
- earlyPenaltyBps
- maxWalletNfts

These should still be preserved because they are part of the deployed protocol params tuple.

## Naming decision

Do not rename the existing epochMinimum pipeline yet.

Instead, add the new XC protocol params source separately.

Reason:

- `epochMinimum` may still be useful for future X1-native minimum sources
- deployed XC Lens uses `getProtocolParams()`
- overloading `epochMinimum` for XC Lens would hide an important ABI distinction

Recommended naming split:

- `XcEpochMinimumSource` remains for epoch minimum abstractions
- `XcProtocolParamsSource` becomes the deployed XC Lens params source

## Testing plan

The implementation milestone should use mocked providers only.

Test cases should cover:

1. reads getProtocolParams through readContract
2. passes Lens address to readContract
3. uses minimal ABI fragment
4. normalizes object-like tuple return
5. normalizes array-like tuple return
6. preserves uint256 values as bigint
7. converts uint8 maxLevel to number
8. converts uint16 baseAprBpsNow to number
9. rejects missing tuple fields with sanitized error
10. rejects malformed tuple shape with sanitized error
11. does not import viem
12. does not read process.env
13. does not create public client
14. does not expose private key / mnemonic / wallet client / tx path

## Error policy

Errors should be sanitized.

Allowed error examples:

    Invalid XC protocol params result: missing currentBaseNominal
    Invalid XC protocol params result: invalid maxLevel
    Failed to read XC protocol params

Errors must not include:

- RPC URL
- API key
- raw provider internals
- raw env
- transport config
- authorization headers

## Future script option

After the source implementation and review, a future manual script may be added:

    scripts/read-xc-protocol-params.ts

But this design milestone does not add that script.

The current manual `epochMinimum` smoke script may remain as-is for now.

A later cleanup may decide whether to:

- keep both scripts
- add a protocol params script
- rename the old script
- deprecate the generic epochMinimum smoke script

That should be a separate milestone.

## Review checklist for implementation

Future implementation review should verify:

    grep -RniE "from ['\"]viem['\"]|from ['\"]ethers['\"]|createPublicClient|http\\(|process\\.env|privateKey|mnemonic|walletClient|writeContract|sendTransaction" src tests package.json || true

Expected result:

- no matches in the new source runtime except harmless test descriptions if any
- no new script-edge real RPC
- no new dependency
- no secret-bearing config

## Decision

The next implementation milestone should add a mocked/tested reusable XC protocol params source.

Recommended next branch after design review:

    xc-protocol-params-source

Expected implementation files:

- src/ethereum/xc-protocol-params-source.ts
- tests/xc-protocol-params-source.test.ts
- src/index.ts

The milestone should not add real RPC execution.

The milestone should not add new dependencies.

The milestone should preserve the script-edge-only secret boundary.
