# XXXL X1/SVM Program Skeleton

## Purpose

This document records the first X1/SVM-facing program skeleton for XXXL Program v1.

This stage follows:

- Theo approval of the model-layer runtime preparation package
- production runtime byte layout definitions

It is still not live deployable runtime code.

It is an implementation-facing skeleton that fixes the major runtime boundaries before live X1/SVM implementation.

## Status

Status:

- `MODEL_ONLY_NOT_DEPLOYABLE`

This stage does not submit transactions.

It does not deploy a program.

It does not derive real on-chain PDAs.

It does not activate any route.

## Program ID boundary

The program id is still an explicit placeholder:

- `XXXLProgram111111111111111111111111111111111`

A future live implementation stage must replace this with the real Program ID.

## SPL Token Program

The skeleton fixes the SPL Token Program ID constant:

- `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`

The `consume_gateway_mint` handler prepares a `mint_to` CPI boundary against this program.

## Gateway mint authority PDA

The skeleton fixes the seed set:

- `["xxxl", "gateway-mint-authority", "v1"]`

This stage uses a deterministic model PDA.

A future live implementation must verify the real SVM `find_program_address` output.

## Handler

Defined handler:

- `consume_gateway_mint`

The handler skeleton validates:

- model-only boundary
- Program ID placeholder boundary
- SPL Token Program ID
- gateway mint authority PDA model
- account meta order
- production byte layouts
- Stage 1 authorization-result boundary
- SPL Token `mint_to` CPI boundary
- processed-event marking boundary

## Account metas

Canonical account meta order:

1. Mint State
2. Gateway Config
3. Guardian Set
4. Processed Event
5. Recipient Balance
6. SPL Token Mint
7. Recipient Token Account
8. Mint Authority PDA
9. Token Program

Writable accounts:

- Mint State
- Processed Event
- Recipient Balance
- SPL Token Mint
- Recipient Token Account

The Mint Authority PDA is not a parent instruction signer.

It is a CPI signer only.

## Guardian signature boundary

The runtime does not verify guardian signatures.

The skeleton keeps the approved boundary:

- `STAGE_1_AUTHORIZATION_RESULT_ONLY`

## Route activation boundary

This skeleton does not activate Avalanche or any other route.

Route activation remains outside this stage.

## Non-goals

This stage does not implement live X1/SVM program code.

It does not deploy.

It does not submit transactions.

It does not derive real on-chain PDA addresses.

It does not call SPL Token.

It does not activate Avalanche.

It does not change supply policy.
