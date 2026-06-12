# Stage 2.9 TypeScript Relayer Prototype Evidence

This document records Stage 2.9 TypeScript relayer prototype evidence for the X1 direct mint gateway.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-2-9-typescript-relayer-prototype

Runtime commit:

    f0dbb4f Add Stage 2.9 TypeScript relayer prototype

Base runtime commit:

    374b100 Clean up Stage 2 runtime account hygiene

## Scope

Stage 2.9 adds a TypeScript relayer prototype.

It does not change the on-chain runtime.

It extracts the direct mint transaction assembly into a separate relayer/client helper.

## Added files

Runtime files added:

    tests/helpers/stage2RelayerPrototype.ts
    tests/stage2_relayer_prototype.test.ts

## Relayer helper responsibilities

The helper covers:

- Stage 2 message hash derivation
- gateway_config PDA derivation
- guardian_set PDA derivation
- processed_burn PDA derivation
- mint_authority PDA derivation
- recipient token account address derivation
- recipient token account preparation
- recipient token balance reading
- processed burn existence check
- compute budget instruction creation
- Ed25519 guardian approval instruction creation
- submit_mint_approval instruction creation
- full transaction assembly

## Transaction shape

The prototype builds the Stage 2.8 transaction shape:

    ComputeBudgetProgram.setComputeUnitLimit
    Ed25519 guardian approval instruction #1
    Ed25519 guardian approval instruction #2
    submit_mint_approval

## Prototype signer boundary

The helper currently uses prototype signer mode:

    Ed25519Program.createInstructionWithPrivateKey

This is acceptable for local/live prototype testing.

Production relayers should consume guardian approvals/signatures and must not own guardian private keys.

## Runtime checks

The following runtime checks passed:

    cargo test -p hello-x1 binding_
    cargo test -p hello-x1 parser_
    anchor build

## Live relayer prototype test

Command:

    ANCHOR_PROVIDER_URL=https://rpc.testnet.x1.xyz \
    ANCHOR_WALLET=~/.config/solana/id.json \
    npx ts-mocha -p ./tsconfig.json -t 120000 tests/stage2_relayer_prototype.test.ts

Result:

    Stage 2.9 TypeScript relayer prototype
      ✔ builds and submits the relayer transaction shape for a direct mint

    1 passing

## Existing live matrix

The existing Stage 2.6 live matrix remained green.

Command:

    ANCHOR_PROVIDER_URL=https://rpc.testnet.x1.xyz \
    ANCHOR_WALLET=~/.config/solana/id.json \
    npx ts-mocha -p ./tsconfig.json -t 120000 tests/gateway_direct_mint_skeleton.ts

Result:

    Stage 2 direct mint gateway skeleton
      ✔ verifies guardian signatures, initializes processed burn, mints tokens, rejects replay, and rolls back failed CPI
      ✔ rejects missing guardian signature instruction
      ✔ rejects wrong xxxl mint and leaves no processed burn
      ✔ rejects recipient token account with wrong mint and leaves no processed burn
      ✔ rejects recipient token account with wrong owner and leaves no processed burn
      ✔ rejects unknown guardian even with valid Ed25519 signatures

    6 passing

## Current conclusion

Stage 2.9 confirms that the relayer transaction shape can be assembled outside the matrix test as a reusable TypeScript relayer/client prototype.

The on-chain runtime behavior remains unchanged.

The direct mint path remains live-tested and the Stage 2.6 rollback matrix remains green.
