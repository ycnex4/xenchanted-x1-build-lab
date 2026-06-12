# Stage 2.12 Inconsistent Recovery State Evidence

This document records Stage 2.12 inconsistent recovery state handling evidence for the X1 direct mint gateway relayer prototype.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-2-12-inconsistent-recovery-state-handling

Runtime commit:

    cc07651 Add Stage 2.12 inconsistent recovery state handling test

Base runtime commit:

    3a0e1e0 Add Stage 2.11 ambiguous confirmation recovery prototype

## Scope

Stage 2.12 adds an inconsistent recovery state handling test for the TypeScript relayer prototype.

It does not change the on-chain runtime.

It covers the case where relayer recovery sees a processed burn, but the observed balance delta does not match the expected minted amount.

## Runtime changes

Runtime test added:

    tests/stage2_relayer_inconsistent_recovery.test.ts

No helper change was required.

## Recovery model

The Stage 2.11 recovery helper can return:

    inconsistent_after_ambiguous_result

when processed state and balance delta do not match the expected completed or not-completed states.

Stage 2.12 verifies this branch.

## Tested scenario

The test performs a real successful mint with:

    mintedAmount = 2222

Then recovery is intentionally called with:

    expectedMintedAmount = 3333

The on-chain state shows:

- processed_burn exists
- token balance delta is 2222
- expected delta is 3333

Therefore the relayer recovery result must be:

    inconsistent_after_ambiguous_result

## Live Stage 2.12 test

Command:

    ANCHOR_PROVIDER_URL=https://rpc.testnet.x1.xyz \
    ANCHOR_WALLET=~/.config/solana/id.json \
    npx ts-mocha -p ./tsconfig.json -t 120000 tests/stage2_relayer_inconsistent_recovery.test.ts

Result:

    Stage 2.12 relayer inconsistent recovery state handling
      ✔ classifies processed burn with unexpected balance delta as inconsistent and does not retry blindly

    1 passing

## Regression checks

Stage 2.11 ambiguous recovery remained green:

    Stage 2.11 relayer ambiguous confirmation recovery
      ✔ recovers a completed mint after an ambiguous send result by checking processed burn and balance

    1 passing

Stage 2.10 idempotency / retry remained green:

    Stage 2.10 relayer idempotency / retry prototype
      ✔ stops safely when processed burn already exists and does not mint twice

    1 passing

Stage 2.9 relayer prototype remained green:

    Stage 2.9 TypeScript relayer prototype
      ✔ builds and submits the relayer transaction shape for a direct mint

    1 passing

Stage 2.6 rollback matrix remained green:

    Stage 2 direct mint gateway skeleton
      ✔ verifies guardian signatures, initializes processed burn, mints tokens, rejects replay, and rolls back failed CPI
      ✔ rejects missing guardian signature instruction
      ✔ rejects wrong xxxl mint and leaves no processed burn
      ✔ rejects recipient token account with wrong mint and leaves no processed burn
      ✔ rejects recipient token account with wrong owner and leaves no processed burn
      ✔ rejects unknown guardian even with valid Ed25519 signatures

    6 passing

Rust / Anchor checks remained green:

    cargo test -p hello-x1 binding_
    cargo test -p hello-x1 parser_
    anchor build

## Current conclusion

Stage 2.12 confirms that the relayer can classify inconsistent recovery state and avoid blind retry.

If processed_burn exists but the recipient token balance delta does not match the expected minted amount, the relayer must treat the result as inconsistent and stop for manual/operator review.

The on-chain runtime remains unchanged.
