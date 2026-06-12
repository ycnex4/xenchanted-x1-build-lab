# Stage 2.5 XXXL Mint Creation Decision

This document records the XXXL mint account creation decision for the Stage 2.5 token mint CPI prototype.

## Decision

For Stage 2.5, the XXXL SPL Token mint will be created outside the gateway program as part of test setup / deploy setup.

The gateway program will not create or initialize the XXXL mint account in the first CPI prototype.

Stage 2.5 submit_mint_approval will:

- receive the XXXL mint account
- verify that it matches the expected xxxl_mint stored in GatewayConfig
- verify the recipient token account
- perform SPL Token mint_to through CPI
- rely on the prototype gateway PDA as mint authority

## Reason

Stage 2.5 is intended to prove the atomic gateway mint path:

    guardian approval + replay protection + ProcessedBurnEntry + SPL Token mint CPI

It is not intended to prove production token setup or final authority architecture.

Keeping mint creation outside the gateway program has several benefits for the first CPI prototype:

- submit_mint_approval remains focused on the atomic mint boundary
- less gateway program logic is added before CPI evidence
- fewer setup edge cases are mixed into the mint proof
- test failures are easier to diagnose
- token setup remains replaceable when the final authority model is designed

## Rejected alternatives for Stage 2.5

### Dedicated initialize_xxxl_mint instruction

A dedicated gateway instruction could create or initialize the XXXL mint.

This is not selected for the first Stage 2.5 prototype because it adds setup logic before the CPI atomicity proof.

It may be revisited later if the final authority model keeps token setup inside the same program.

### Mint setup inside initialize_gateway_config

This is not selected.

Gateway configuration and token mint setup are separate concerns.

Mixing them would make the prototype harder to reason about.

## GatewayConfig requirement

GatewayConfig should store the expected XXXL mint pubkey.

submit_mint_approval must reject any mint account that does not match GatewayConfig.xxxl_mint.

This prevents a relayer from submitting a valid gateway approval while redirecting mint CPI to an arbitrary SPL Token mint.

## Prototype-only authority boundary

The Stage 2.5 test mint may use the gateway PDA as mint authority.

This remains prototype-only.

It is not the final XXXL production authority model.

The final authority model remains open until Stake and Forge architecture are defined.

## Expected Stage 2.5 setup flow

The expected Stage 2.5 test setup flow is:

1. Derive gateway PDA / prototype mint_authority PDA.
2. Create a test XXXL SPL Token mint outside the gateway program.
3. Set the mint authority to the gateway PDA.
4. Initialize or update GatewayConfig with the expected xxxl_mint pubkey.
5. Create or provide a valid recipient token account.
6. Call submit_mint_approval.
7. Verify that ProcessedBurnEntry creation and mint_to are atomic.

## Test requirements

Stage 2.5 CPI tests should cover:

- valid mint account succeeds
- wrong mint account is rejected
- wrong mint authority causes CPI failure and rollback
- invalid recipient token account causes CPI failure and rollback
- ProcessedBurnEntry does not remain if mint CPI fails
- replay after successful mint does not mint again

## Current conclusion

Stage 2.5 prerequisite 2 is closed for the prototype.

The XXXL mint account will be pre-created outside the gateway program for the first CPI prototype.

The gateway program must store and enforce the expected xxxl_mint pubkey.
