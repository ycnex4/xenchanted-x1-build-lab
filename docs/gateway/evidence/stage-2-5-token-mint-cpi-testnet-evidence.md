# Stage 2.5 Token Mint CPI Testnet Evidence

This document records the first successful Stage 2.5 token mint CPI runtime evidence on X1 testnet.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-2-5-token-mint-cpi-runtime

Runtime commit:

    9ec4f66 Add Stage 2.5 token mint CPI runtime path

## Program

Program id:

    9tCJe4M1MJQtE1gDxNYNE75fNUGpSAKiX56rgUMR8984

ProgramData address:

    32XqEK3cV1gySnS4gWAhEcTMfGtmNUcQrjdNkk4FVFWn

Upgrade authority:

    DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc

Successful deploy signature:

    34xtHwaBWvTj26zTicxX42aDFBHykTurwbEuvGkM2V3KFpUhNNfp4Rp2VwqBEjRcDmXvbETDCmouQhwARUjRW2yf

Previous deployed slot:

    164855038

New deployed slot:

    165158591

New deployed data length:

    261464 bytes

## What changed in runtime

Stage 2.5 added the first SPL Token mint CPI path to submit_mint_approval.

Implemented runtime additions:

- GatewayConfig.xxxl_mint
- Stage 2.5 gateway PDA seed: [b"gateway", b"stage_2_5"]
- prototype mint_authority PDA seed: [b"mint_authority"]
- xxxl_mint account validation
- recipient_token_account validation
- standard SPL Token program validation
- SPL Token mint_to CPI
- client/test ComputeBudgetProgram.setComputeUnitLimit
- test setup for XXXL mint and recipient token account

## Successful live test

Command:

    ANCHOR_PROVIDER_URL=https://rpc.testnet.x1.xyz \
    ANCHOR_WALLET=~/.config/solana/id.json \
    npx ts-mocha -p ./tsconfig.json -t 120000 tests/gateway_direct_mint_skeleton.ts

Result:

    Stage 2 direct mint gateway skeleton
      ✔ verifies guardian signatures, initializes processed burn, and rejects replay
      ✔ rejects missing guardian signature instruction
      ✔ rejects unknown guardian even with valid Ed25519 signatures

    3 passing

## Evidence proven

The successful test proves the Stage 2.5 direct mint CPI path:

    guardian signatures
    -> context-bound message_hash
    -> gateway validation
    -> replay protection
    -> ProcessedBurnEntry
    -> SPL Token mint_to CPI
    -> replay rejection after success

Negative paths also remained active:

- missing guardian signature is rejected
- unknown guardian is rejected
- replay after successful mint is rejected

## Deployment note

Initial deploy attempts failed because of expired blockhash / max retries and left deploy buffer accounts.

Old deploy buffers were closed and SOL was returned to the authority wallet.

A later deploy using solana program deploy with increased max sign attempts succeeded.

## Current conclusion

Stage 2.5 has reached live X1 testnet CPI success.

The gateway prototype no longer only records processed burns.

It now performs an atomic path that records ProcessedBurnEntry and mints XXXL through SPL Token mint_to CPI.
