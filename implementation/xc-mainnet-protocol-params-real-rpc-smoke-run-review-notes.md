# XC mainnet protocol params real RPC smoke run review

This document reviews the sanitized mainnet real RPC smoke run result for the deployed xEnchanted Crypto NFT Lens.

Reviewed branch:

    xc-mainnet-protocol-params-real-rpc-smoke-run-review

Reviewed smoke run branch:

    xc-epoch-minimum-real-rpc-smoke-run

Reviewed files:

- implementation/xc-mainnet-protocol-params-real-rpc-smoke-run-result.md
- docs/checkpoints/current-design-checkpoint.md

## Review summary

The sanitized mainnet protocol params real RPC smoke run result is accepted.

The recorded result confirms:

- Ethereum mainnet RPC was reachable
- provider chain ID matched configured mainnet
- deployed NFT Lens address was valid
- getProtocolParams() read succeeded
- protocol params were returned successfully
- no RPC URL, API key, private key, mnemonic, seed phrase, .env content, or raw environment content was recorded

## Result reviewed

Network:

    chainId=eip155-1
    providerChainId=1
    network=Ethereum mainnet

Lens:

    lensAddress=0xd4b90d7392c1565d558c80122dee76b5b3bb6c23
    function=getProtocolParams()

Success marker:

    xcProtocolParamsSmoke=true
    completed=true

## ABI/function review

The previous generic epochMinimum(lockEpoch) smoke attempt reached mainnet and validated chain ID, but failed with a sanitized runtime error.

That failure is now understood as an expected ABI/function mismatch.

The deployed xEnchantedNFTLens does not expose:

    epochMinimum(uint256)

The deployed XC NFT Lens exposes the correct read path:

    getProtocolParams()

The successful mainnet smoke result uses getProtocolParams().

## Secret safety review

The reviewed result does not include:

- RPC URL
- API key
- private key
- mnemonic
- seed phrase
- .env content
- raw environment content
- provider account details
- transport config

The review grep found secret-related words only in safety statements saying that secrets were not recorded or printed.

No actual secret-like value was found in the result document.

## Diff review

Diff from pre-smoke baseline to current HEAD shows only:

- docs/checkpoints/current-design-checkpoint.md
- implementation/xc-mainnet-protocol-params-real-rpc-smoke-run-result.md

No runtime code changed.

No dependencies changed.

No script changed.

## Validation baseline

Review baseline:

- npm run typecheck passed
- npm test passed: 37 test files, 286 tests
- npm run build passed
- npm audit --audit-level=moderate found 0 vulnerabilities

Manual refusal check without env remained safe:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

## Review decision

The XC mainnet protocol params real RPC smoke run result is accepted.

No changes are required before merging this review checkpoint.

Recommended next step after merge:

    complete current XC epoch minimum / protocol params RPC smoke milestone
