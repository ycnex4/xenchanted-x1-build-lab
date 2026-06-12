# Stage 2.7 Runtime Account Hygiene Evidence

This document records Stage 2.7 runtime account hygiene evidence for the X1 gateway prototype.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-2-7-runtime-account-hygiene

Runtime commit:

    374b100 Clean up Stage 2 runtime account hygiene

Base Stage 2.6 runtime commit:

    de931e7 Add Stage 2.6 CPI failure rollback matrix

## Scope

Stage 2.7 is a hygiene-only runtime cleanup.

It does not change the direct mint gateway protocol logic.

It cleans up two runtime warnings observed during Stage 2.5 and Stage 2.6:

- ambiguous glob re-exports
- deprecated AccountInfo usage for the instruction sysvar account

## Runtime changes

The runtime cleanup changed:

- instruction module re-exports from public glob exports to crate-local glob exports
- submit_mint_approval instructions_sysvar account from AccountInfo to UncheckedAccount
- explicit conversion of instructions_sysvar to AccountInfo before reading prior Ed25519 instructions

## Checks

The following checks passed after cleanup:

    cargo test -p hello-x1 binding_
    cargo test -p hello-x1 parser_
    anchor build

The previous Rust / Anchor warnings were removed.

## Live matrix

The Stage 2.6 live testnet matrix remained green after cleanup.

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

Stage 2.7 confirms that the gateway runtime can be cleaned up without changing the Stage 2.5 / Stage 2.6 behavior.

The runtime keeps the proven properties:

- direct SPL Token mint_to CPI works
- replay is rejected
- failed CPI rolls back ProcessedBurnEntry
- invalid mint / token account inputs are rejected
- no false processed state is left on failure

The Rust / Anchor account hygiene warnings are removed.
