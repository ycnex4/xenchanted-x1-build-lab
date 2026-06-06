# xEnchanted X1 Build Lab XC protocol params RPC smoke result

This document records the successful controlled read-only mainnet RPC smoke run for XC protocol params.

This result is documentation-only.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Smoke command

Package script:

    npm run smoke:xc-protocol-params:rpc

Resolved command:

    node ./dist/scripts/read-xc-protocol-params.js

Source file:

    scripts/read-xc-protocol-params.ts

## Target

Chain:

    eip155-1

Provider chain id:

    1

Lens address:

    0xd4b90d7392c1565d558c80122dee76b5b3bb6c23

Function:

    getProtocolParams()

Finality:

    safe

## Result

The controlled read-only mainnet RPC smoke run completed successfully.

Observed output:

    manualProtocolParamsSmoke=true
    providerChainId=1
    chainId=eip155-1
    lensAddress=0xd4b90d7392c1565d558c80122dee76b5b3bb6c23
    finality=safe
    realRpcConfirmed=true
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

## Safety review

The smoke run was read-only.

It did not:

- execute transactions
- create a wallet client
- use a private key
- use a mnemonic
- call writeContract
- call sendTransaction
- print RPC URL
- print API key
- print `.env` contents

## Decision

The XC protocol params read-only mainnet RPC smoke path is confirmed working.

This does not approve production deployment.

This only confirms that the deployed mainnet XC Lens can be read through the safe read-only protocol params smoke script.
