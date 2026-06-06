# XC epoch minimum / protocol params RPC smoke completion checkpoint

This document closes the current XC epoch minimum / protocol params RPC smoke milestone.

This checkpoint is documentation-only.

No runtime code is changed in this milestone.

No dependencies are changed in this milestone.

No real RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Completed chain

The project completed the full safe progression:

1. mocked XC epoch minimum source abstractions
2. Ethereum read provider abstraction
3. mocked Ethereum provider wrapper
4. viem-like read provider wrapper without viem dependency
5. read-only RPC integration helper with provided public client
6. mocked Ethereum script config parser
7. mocked/testable script runner
8. manual-only RPC smoke script
9. safe real RPC run notes
10. safe real RPC run notes review
11. sanitized mainnet protocol params smoke run
12. sanitized mainnet protocol params smoke run review

## Current main status

Latest completed main milestone:

    main -> 6e6b1ba Merge branch 'xc-mainnet-protocol-params-real-rpc-smoke-run-review'

Final validation after merge:

- npm run typecheck passed
- npm test passed: 37 test files, 286 tests
- npm run build passed
- npm audit --audit-level=moderate found 0 vulnerabilities

Safe refusal without env remained active:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

## What was proven

The milestone proved that:

- the manual RPC script can be added without default/CI execution
- process.env ownership stays at script edge
- viem ownership stays at script edge
- createPublicClient/http ownership stays at script edge
- no private key, mnemonic, signer, wallet client, writeContract, or sendTransaction path was added
- missing env fails safely before RPC
- chain mismatch fails safely before contract reads
- mainnet RPC can be reached safely when provided locally
- deployed mainnet xEnchantedNFTLens can be read
- sanitized result can be recorded without secrets

## Successful mainnet result

The successful deployed XC Lens smoke path is:

    chainId=eip155-1
    providerChainId=1
    lensAddress=0xd4b90d7392c1565d558c80122dee76b5b3bb6c23
    function=getProtocolParams()
    completed=true

The sanitized successful result was recorded in:

    implementation/xc-mainnet-protocol-params-real-rpc-smoke-run-result.md

The review was recorded in:

    implementation/xc-mainnet-protocol-params-real-rpc-smoke-run-review-notes.md

## Important ABI conclusion

The deployed mainnet xEnchantedNFTLens does not expose:

    epochMinimum(uint256)

The deployed mainnet xEnchantedNFTLens exposes:

    getProtocolParams()

Therefore the generic epochMinimum smoke script is not the correct read path for this deployed XC Lens.

The previous epochMinimum run was still useful because it confirmed:

- mainnet RPC was reachable
- provider chain ID matched configured mainnet
- Lens address format and chain path were valid
- the script reached the contract read boundary
- failure was sanitized
- no secrets were printed

But the final accepted mainnet read path for the deployed XC Lens is:

    getProtocolParams()

## Protocol params returned by mainnet Lens

The sanitized mainnet read returned:

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

## Safety conclusion

No RPC URL was committed.

No API key was committed.

No private key was committed.

No mnemonic or seed phrase was committed.

No `.env` content or raw environment content was committed.

The real RPC URL was entered locally only and was not recorded.

## Architectural conclusion

The current project should not treat the deployed xEnchantedNFTLens as an epochMinimum(uint256) source.

For xEnchanted Crypto mainnet, the authoritative deployed Lens read path is:

    getProtocolParams()

For future X1 Build logic, the next design step should decide how to use XC protocol params as a source of truth.

Important fields for future X1 Build design likely include:

- currentEpoch
- currentBaseNominal
- currentXenBurnAmount
- halvingInterval
- xenBurnHalvingInterval
- nextHalvingTs
- genesisTs

## Recommended next milestone

Recommended next branch:

    xc-protocol-params-source-design

Purpose:

- design a reusable XC protocol params source
- model getProtocolParams() as the real deployed XC Lens read path
- decide which protocol params are authoritative for X1 Build validation
- avoid overloading the old epochMinimum naming
- keep tests mocked
- keep real RPC out of unit tests
- preserve the script-edge-only secret boundary

Expected design-only files:

- implementation/xc-protocol-params-source-design.md
- docs/checkpoints/current-design-checkpoint.md

## Decision

The current XC epoch minimum / protocol params RPC smoke milestone is complete.

Next step should be design-only:

    xc-protocol-params-source-design
