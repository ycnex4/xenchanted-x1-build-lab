# Stage 2.5 Mint Authority PDA Decision

This document records the prototype mint authority PDA decision for the Stage 2.5 token mint CPI prototype.

## Decision

Stage 2.5 will use a dedicated prototype mint authority PDA.

Seed model:

    seeds = [b"mint_authority"]

The mint authority PDA is separate from GatewayConfig.

GatewayConfig remains responsible for protocol configuration and expected state.

MintAuthority PDA is responsible only for signing the SPL Token mint CPI in the Stage 2.5 prototype.

## Scope

This is a Stage 2.5 prototype-only authority decision.

It is not the final XXXL production mint authority model.

The final XXXL authority model remains open until Stake and Forge architecture are defined.

## Reason

Using a dedicated mint authority PDA keeps roles clearer:

- GatewayConfig stores protocol configuration
- MintAuthority PDA signs mint CPI
- submit_mint_approval enforces gateway rules before minting
- token mint authority is not mixed with config storage
- future migration to a shared Core/Minter authority model remains conceptually cleaner

This is preferable to using GatewayConfig PDA as the mint authority.

## Rejected alternative

### GatewayConfig PDA as mint authority

GatewayConfig PDA could technically be used as the mint authority.

This is not selected because it mixes two roles:

- configuration account
- token mint authority signer

For Stage 2.5, a dedicated mint_authority PDA is clearer.

## Stage 2.5 expected authority flow

The expected Stage 2.5 prototype flow is:

1. Derive mint_authority PDA with seeds [b"mint_authority"].
2. Create the test XXXL SPL Token mint outside the gateway program.
3. Set XXXL mint authority to the mint_authority PDA.
4. Store expected xxxl_mint in GatewayConfig.
5. submit_mint_approval verifies gateway rules.
6. submit_mint_approval invokes SPL Token mint_to.
7. The gateway program signs the CPI using mint_authority PDA signer seeds.

## Required runtime checks

submit_mint_approval should verify:

- provided xxxl_mint matches GatewayConfig.xxxl_mint
- provided token program is the standard SPL Token program
- provided recipient_token_account is valid for recipient and xxxl_mint
- mint CPI is signed by the mint_authority PDA
- no external wallet signs as mint authority

## Current conclusion

Stage 2.5 prerequisite 3 is closed for the prototype.

The first CPI prototype will use:

    mint_authority PDA = [b"mint_authority"]

This remains prototype-only and does not finalize the production XXXL authority model.
