# XC mainnet protocol params real RPC smoke run result

This document records a sanitized real RPC smoke run result for the deployed xEnchanted Crypto mainnet NFT Lens.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Context

The previous generic epoch minimum smoke script successfully reached mainnet RPC and validated chain ID, but failed with a sanitized runtime error because the deployed `xEnchantedNFTLens` does not expose:

```text
epochMinimum(uint256)
```

The deployed NFT Lens exposes:

```text
getProtocolParams()
```

Therefore a direct read-only protocol params smoke run was performed locally.

## Network

```text
chainId=eip155-1
providerChainId=1
network=Ethereum mainnet
```

## Lens

```text
lensAddress=0xd4b90d7392c1565d558c80122dee76b5b3bb6c23
function=getProtocolParams()
```

## Sanitized result

```text
xcProtocolParamsSmoke=true
providerChainId=1
lensAddress=0xd4b90d7392c1565d558c80122dee76b5b3bb6c23
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
completed=true
```

## Interpretation

The real RPC smoke run confirms:

- Ethereum mainnet RPC was reachable
- provider chain ID matched configured mainnet
- deployed NFT Lens address was valid
- `getProtocolParams()` read succeeded
- protocol params were returned successfully
- no secret values were printed or recorded

## Safety note

The RPC URL was entered locally and is not recorded.

No API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content was printed or committed.
