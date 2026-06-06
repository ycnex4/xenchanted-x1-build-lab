# XC protocol params source design review

This document reviews the XC protocol params source design milestone.

Reviewed branch:

    xc-protocol-params-source-design-review

Reviewed design milestone:

    xc-protocol-params-source-design

Reviewed files:

- implementation/xc-protocol-params-source-design.md
- docs/checkpoints/current-design-checkpoint.md

## Review summary

The XC protocol params source design is accepted.

The design correctly separates the deployed XC Lens protocol params read path from the existing epoch minimum abstraction.

The deployed mainnet xEnchantedNFTLens should be modeled through:

    getProtocolParams()

The deployed mainnet xEnchantedNFTLens should not be modeled as:

    epochMinimum(uint256)

## Design-only boundary review

The reviewed milestone is design-only.

Diff from pre-design baseline to current HEAD shows only:

- docs/checkpoints/current-design-checkpoint.md
- implementation/xc-protocol-params-source-design.md

No runtime code changed.

No tests changed.

No package dependency changed.

No script changed.

No real RPC was executed in the design milestone.

## Source design review

The proposed new source is:

    XcProtocolParamsSource

Expected future implementation file:

    src/ethereum/xc-protocol-params-source.ts

Expected future test file:

    tests/xc-protocol-params-source.test.ts

Expected export update:

    src/index.ts

The design keeps XcProtocolParamsSource separate from XcEpochMinimumSource.

This is the correct separation because epochMinimum may still be useful for future X1-native minimum sources, while deployed XC Lens uses getProtocolParams().

## Protocol params model review

The proposed model preserves uint256-like values as bigint and converts small numeric fields to number.

This is acceptable for:

- maxLevel as number
- baseAprBpsNow as number

Large protocol values remain bigint.

The design preserves the full deployed tuple:

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

## X1 Build authoritative fields review

The design correctly identifies likely authoritative fields for future X1 Build validation:

- currentEpoch
- currentBaseNominal
- currentXenBurnAmount
- halvingInterval
- xenBurnHalvingInterval
- nextHalvingTs
- genesisTs

These fields should be considered in later X1 Build validation design.

The design does not prematurely hardcode final X1 Build rules.

## Provider boundary review

The design correctly requires the future source to use an injected read-only provider.

The future source should not:

- own real RPC
- read process.env
- construct public clients
- import viem directly in runtime source
- accept private keys
- accept mnemonics
- create signers
- create wallet clients
- call writeContract
- call sendTransaction

Allowed operation for the first source:

- readContract

This preserves the existing script-edge-only secret boundary.

## ABI review

The design correctly proposes a minimal ABI fragment for:

    getProtocolParams()

The first implementation should not load ABI files from disk.

The first implementation should not add ABI path support.

This keeps the source deterministic and testable.

## Tuple normalization review

The design correctly requires support for both:

- object-like tuple returns with named fields
- array-like tuple returns ordered by ABI components

This avoids coupling implementation correctness to one provider/mock return style.

## Error policy review

The design requires sanitized errors.

Allowed style:

    Invalid XC protocol params result: missing currentBaseNominal
    Invalid XC protocol params result: invalid maxLevel
    Failed to read XC protocol params

Forbidden error content:

- RPC URL
- API key
- raw provider internals
- raw env
- transport config
- authorization headers

This is consistent with the existing safety policy.

## Boundary grep review

Review grep found provider/secret/transaction terms only in design and checkpoint boundary sections.

No runtime source file was added or changed by the design milestone.

No package dependency was added.

No script was added.

## Validation baseline

Review baseline:

- npm run typecheck passed
- npm test passed: 37 test files, 286 tests
- npm run build passed
- npm audit --audit-level=moderate found 0 vulnerabilities

Manual refusal without env remained safe:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

## Review decision

The XC protocol params source design is accepted.

No design changes are required before merging this review checkpoint.

Recommended next milestone after merge:

    xc-protocol-params-source

That milestone should add a mocked/tested reusable XC protocol params source without real RPC execution, without new dependencies, and without moving secret-bearing config out of the script edge.
