# Stage 2.5 Recipient Token Account Policy

This document records the recipient token account policy for the Stage 2.5 token mint CPI prototype.

## Decision

Stage 2.5 will require recipient_token_account to exist before submit_mint_approval.

The gateway program will not create the recipient associated token account inside submit_mint_approval in the first CPI prototype.

submit_mint_approval will receive recipient_token_account as an account and verify that it is valid for:

- the intended recipient
- the expected XXXL mint
- the standard SPL Token program

## Reason

Stage 2.5 is intended to prove the atomic gateway mint path:

    guardian approval + replay protection + ProcessedBurnEntry + SPL Token mint CPI

It is not intended to prove associated token account creation.

Keeping recipient token account creation outside submit_mint_approval keeps the first CPI prototype smaller and easier to reason about.

Benefits:

- submit_mint_approval stays focused on validation, replay marking, and mint_to CPI
- fewer account creation edge cases
- fewer rent-payer concerns
- easier rollback testing
- easier diagnosis of token account mismatch failures

## Required runtime checks

submit_mint_approval should verify:

- recipient_token_account is owned by the SPL Token program
- recipient_token_account.mint == GatewayConfig.xxxl_mint
- recipient_token_account.owner == recipient
- xxxl_mint == GatewayConfig.xxxl_mint
- token_program == standard SPL Token program

If these checks fail:

- no XXXL should be minted
- ProcessedBurnEntry should not remain created
- the transaction should fail atomically

## Rejected alternatives for Stage 2.5

### Create associated token account inside submit_mint_approval

This is not selected for the first CPI prototype.

Reason:

- it mixes ATA creation with gateway mint proof
- it adds rent-payer and associated token program complexity
- it makes failure modes harder to isolate
- it is not necessary to prove gateway mint CPI atomicity

### Separate create_recipient_token_account instruction

This is also not selected for the first prototype.

Recipient token account creation can be handled by test setup or client setup before submit_mint_approval.

It may be revisited later if production UX requires the gateway flow to create recipient token accounts.

## Expected Stage 2.5 setup flow

The expected prototype flow is:

1. Create or derive the test XXXL mint.
2. Set mint authority to the prototype mint_authority PDA.
3. Store expected xxxl_mint in GatewayConfig.
4. Create recipient_token_account outside the gateway program.
5. Call submit_mint_approval with recipient and recipient_token_account.
6. submit_mint_approval validates the token account and performs mint_to CPI.

## Test requirements

Stage 2.5 CPI tests should cover:

- valid recipient token account succeeds
- wrong recipient owner is rejected
- wrong token mint is rejected
- non-token account is rejected
- missing/uninitialized token account fails
- mint CPI failure does not leave ProcessedBurnEntry
- replay after successful mint does not mint again

## Current conclusion

Stage 2.5 prerequisite 4 is closed for the prototype.

The recipient token account must be created outside the gateway program and passed into submit_mint_approval.

submit_mint_approval must verify that the token account belongs to the intended recipient and expected XXXL mint.
