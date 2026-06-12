# Stage 2.6 CPI Failure Rollback Matrix Evidence

This document records Stage 2.6 runtime evidence for CPI failure rollback safety on X1 testnet.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-2-6-cpi-failure-rollback-matrix

Runtime commit:

    de931e7 Add Stage 2.6 CPI failure rollback matrix

Base Stage 2.5 runtime commit:

    9ec4f66 Add Stage 2.5 token mint CPI runtime path

## Program

Program id:

    9tCJe4M1MJQtE1gDxNYNE75fNUGpSAKiX56rgUMR8984

Program deployed slot used by Stage 2.6 tests:

    165158591

## Test command

    ANCHOR_PROVIDER_URL=https://rpc.testnet.x1.xyz \
    ANCHOR_WALLET=~/.config/solana/id.json \
    npx ts-mocha -p ./tsconfig.json -t 120000 tests/gateway_direct_mint_skeleton.ts

## Test result

    Stage 2 direct mint gateway skeleton
      ✔ verifies guardian signatures, initializes processed burn, mints tokens, rejects replay, and rolls back failed CPI
      ✔ rejects missing guardian signature instruction
      ✔ rejects wrong xxxl mint and leaves no processed burn
      ✔ rejects recipient token account with wrong mint and leaves no processed burn
      ✔ rejects recipient token account with wrong owner and leaves no processed burn
      ✔ rejects unknown guardian even with valid Ed25519 signatures

    6 passing

## Evidence proven

Stage 2.6 proves that the Stage 2.5 CPI mint path is not only successful on the happy path, but also safe under failure.

Positive path proven:

- valid guardian signatures are accepted
- context-bound message_hash is enforced
- ProcessedBurnEntry is created on success
- SPL Token mint_to CPI succeeds on success
- recipient token account balance increases by the minted amount

Replay path proven:

- replay after success is rejected
- recipient token account balance does not increase on replay

CPI failure rollback path proven:

- failed mint_to CPI rolls back the transaction
- failed mint_to CPI leaves no ProcessedBurnEntry
- failed mint_to CPI does not change recipient token balance

Invalid account paths proven:

- wrong xxxl_mint is rejected
- recipient token account with wrong mint is rejected
- recipient token account with wrong owner is rejected
- each invalid account path leaves no ProcessedBurnEntry
- each invalid account path does not change recipient token balance

## Why this matters

The key Stage 2.6 safety property is:

    no false processed state on failed mint

If mint_to CPI fails after the handler writes ProcessedBurnEntry, the full Solana transaction must roll back.

The Stage 2.6 test matrix confirms that the gateway does not leave a burn falsely marked as processed when the token mint did not happen.

## Current conclusion

Stage 2.6 confirms CPI failure atomicity for the direct mint gateway prototype.

The gateway now has evidence for:

    successful mint
    replay rejection
    CPI failure rollback
    invalid mint rejection
    invalid recipient token account rejection
    no false ProcessedBurnEntry on failure
