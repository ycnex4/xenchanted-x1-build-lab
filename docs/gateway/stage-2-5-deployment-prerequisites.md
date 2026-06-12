# Stage 2.5 Deployment Prerequisites

This document records deployment prerequisites for the Stage 2.5 token mint CPI prototype.

## Status

Theo reviewed the Stage 2.5 implementation boundary and confirmed that there are no blockers before runtime CPI implementation.

The boundary is accepted for prototype implementation.

## Required deployment prerequisites

Before running the Stage 2.5 CPI prototype on X1 testnet, the following must be true:

1. The test XXXL SPL Token mint exists.
2. GatewayConfig.xxxl_mint points to the expected test XXXL mint.
3. The test XXXL mint authority is the prototype mint_authority PDA.
4. The prototype mint_authority PDA uses:

       seeds = [b"mint_authority"]

5. recipient_token_account exists before submit_mint_approval.
6. recipient_token_account.owner == recipient.
7. recipient_token_account.mint == GatewayConfig.xxxl_mint.
8. The client/test transaction includes ComputeBudgetProgram.setComputeUnitLimit for the full CPI path.

## Mint authority edge case

A valid GatewayConfig.xxxl_mint is not sufficient by itself.

The mint account must also have the expected mint authority.

Edge case:

- GatewayConfig.xxxl_mint points to the correct mint.
- But the SPL Token mint authority is not the mint_authority PDA.

In that case, mint_to CPI should fail.

For Stage 2.5 this is acceptable as a deployment/setup prerequisite and test case.

It is not treated as a planning blocker.

## Runtime expectation

submit_mint_approval should still verify:

- provided xxxl_mint == GatewayConfig.xxxl_mint
- recipient_token_account belongs to recipient
- recipient_token_account uses expected xxxl_mint
- token_program is the standard SPL Token program

The SPL Token program enforces mint authority during mint_to CPI.

If the mint authority is wrong, the CPI must fail and the transaction must roll back.

ProcessedBurnEntry must not remain created if mint_to CPI fails.

## Current conclusion

Stage 2.5 can move into runtime CPI implementation.

The remaining mint authority mismatch case is a deployment prerequisite and rollback test, not a blocker.
