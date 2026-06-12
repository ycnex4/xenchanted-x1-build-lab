# Stage 2.5 Token Mint CPI Planning

This document defines the planning boundary for Stage 2.5 token mint CPI integration after the Stage 2.4 guardian approval layer passed live X1 testnet evidence.

## Status before Stage 2.5

Stage 2.4 is complete at the guardian approval layer.

Live X1 testnet evidence exists for:

- context-bound message_hash
- Ed25519 guardian signatures
- prior instruction scanning
- non-Ed25519 instruction interleaving
- guardian set membership
- ProcessedBurnEntry creation on success
- replay rejection
- missing signature rejection
- unknown guardian rejection

Theo review conclusion:

- guardian approval layer is closed for Stage 2
- no blockers remain before Stage 2.5 planning
- token mint CPI planning is the correct next step

## Stage 2.5 goal

Stage 2.5 introduces planning for minting XXXL through token mint CPI after guardian approval succeeds.

The goal is not to rush into production.

The goal is to design and test the next atomic boundary:

    guardian approval + replay protection + mint CPI

The mint and ProcessedBurnEntry mark must be atomic.

If mint CPI fails, ProcessedBurnEntry must not remain created.

EV-01 and EV-02 rollback evidence support this assumption, but Stage 2.5 must prove it with token mint CPI tests.

## Required design decisions before CPI code

### 1. Token program ID

Stage 2.5 must decide which token program XXXL uses on X1:

- SPL Token / TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
- Token-2022 / TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb

This decision affects:

- CPI interface
- account constraints
- mint account layout
- recipient token account layout
- client test setup
- future production compatibility

Decision:

- use standard SPL Token for the first Stage 2.5 CPI prototype
- token program id: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
- do not use Token-2022 in the first CPI prototype
- keep Token-2022 as a future compatibility option if extensions become necessary

### 2. XXXL mint account status

Stage 2.5 must decide how the XXXL mint account is created.

Options:

1. Pre-create XXXL mint before gateway deploy.
2. Add a controlled create_xxxl_mint instruction.
3. Add mint setup into initialize_gateway_config.

Preferred direction for Stage 2.5 prototype:

- keep mint creation separate from submit_mint_approval
- keep submit_mint_approval focused on approval + replay mark + mint CPI
- avoid mixing gateway configuration with token setup until the mint authority model is confirmed

Decision:

- pre-create the XXXL SPL Token mint outside the gateway program for the first Stage 2.5 CPI prototype
- do not add initialize_xxxl_mint to the gateway program in the first CPI prototype
- GatewayConfig must store expected xxxl_mint
- submit_mint_approval must reject any mint account that does not match GatewayConfig.xxxl_mint

### 3. Prototype-only gateway PDA mint authority

Stage 2.5 may use a gateway mint_authority PDA for the first CPI prototype.

This is a prototype-only authority model.

It is not the final XXXL production mint authority model.

The purpose of Stage 2.5 is to prove:

    gateway verification + replay protection + SPL Token mint CPI = atomic

Stage 2.5 does not decide all future XXXL mint sources.

Future X1-side mechanics may also mint XXXL:

- Stake redeem
- Forge redeem
- future reward/redeem mechanics

Because SPL Token has a single mint authority, final production authority cannot be assumed to be gateway-only if other protocol modules also need mint rights.

Stage 2.5 prototype authority decision:

- use a dedicated mint_authority PDA as prototype-only mint authority
- PDA seed model: [b"mint_authority"]
- mint CPI uses mint_authority PDA signer seeds
- document this as non-final
- keep final authority model open until Stake/Forge architecture is defined

Likely future production direction:

- separate XXXL Core/Minter authority program
- or another shared authority layer for multiple approved protocol mint paths

Required Stage 2.5 property:

- no external wallet should mint directly during the CPI prototype
- the gateway PDA is acceptable only as a testnet/prototype mint authority

### 4. Recipient token account

SPL Token mint_to does not mint directly to a wallet owner.

It mints to a token account.

Stage 2.5 must decide who creates or provides recipient_token_account.

Options:

1. Recipient token account must already exist.
2. Relayer creates associated token account before submit.
3. Gateway instruction creates associated token account if missing.
4. Separate create_recipient_token_account instruction.

Preferred direction for Stage 2.5 prototype:

- require recipient_token_account to be passed in
- reject if it is not valid for recipient and XXXL mint
- do not create associated token account inside submit_mint_approval in the first CPI prototype

Reason:

- keeps the first mint CPI boundary smaller
- reduces account complexity
- makes failure modes easier to test

Decision:

- recipient_token_account must be created outside the gateway program for the first Stage 2.5 CPI prototype
- submit_mint_approval receives recipient_token_account as an account
- submit_mint_approval verifies recipient_token_account.owner == recipient
- submit_mint_approval verifies recipient_token_account.mint == GatewayConfig.xxxl_mint
- do not create associated token accounts inside submit_mint_approval in the first CPI prototype

### 5. Compute budget

Stage 2.4 already uses:

- Ed25519 verification instructions
- prior instruction scanning
- context-bound keccak hash
- account creation for ProcessedBurnEntry

Stage 2.5 adds:

- token mint CPI
- token account checks
- PDA signer seeds

Client transactions may need ComputeBudgetProgram.setComputeUnitLimit.

Decision:

- add ComputeBudgetProgram.setComputeUnitLimit to the client/test transaction before submit_mint_approval for the full CPI path
- treat compute budget as a client/test execution strategy, not an on-chain protocol rule
- do not store compute budget settings in GatewayConfig

Expected Stage 2.5 test transaction shape:

1. optional ComputeBudgetProgram.setComputeUnitLimit
2. Ed25519 instruction for guardian A
3. optional non-Ed25519 interleaving instruction
4. Ed25519 instruction for guardian B
5. submit_mint_approval with mint CPI

## Stage 2.5 atomicity requirement

The following must happen atomically inside submit_mint_approval:

1. verify gateway not paused
2. verify minted_amount > 0
3. verify active guardian set
4. derive and verify context-bound message_hash
5. scan prior Ed25519 instructions
6. verify guardian membership and quorum
7. create ProcessedBurnEntry
8. mint XXXL through CPI

If any step fails, no ProcessedBurnEntry should remain and no XXXL should be minted.

## Stage 2.5 test requirements

Local/runtime tests should cover:

### Success path

- valid context-bound message_hash
- valid guardian signatures
- valid recipient token account
- ProcessedBurnEntry created
- XXXL minted to recipient token account
- replay rejected

### Mint CPI failure rollback

- valid guardian approvals
- invalid mint account or invalid recipient token account
- CPI fails
- ProcessedBurnEntry does not remain
- no token balance increase

### Replay with mint

- first submit succeeds and mints
- second submit with same canonical_event_key fails
- no second mint occurs

### Wrong context

- guardians sign context A
- relayer submits context B
- expected_message_hash mismatch
- no ProcessedBurnEntry
- no mint

### Unknown guardian

- valid Ed25519 signatures from unknown guardians
- no ProcessedBurnEntry
- no mint

### Missing signature

- approved guardian listed but no matching Ed25519 instruction
- no ProcessedBurnEntry
- no mint

### Token account mismatch

- recipient_token_account does not belong to recipient
- no ProcessedBurnEntry
- no mint

### Mint authority mismatch

- XXXL mint authority is not gateway PDA
- CPI fails
- no ProcessedBurnEntry
- no mint

## What Stage 2.5 does not prove yet

Stage 2.5 planning does not prove:

- production watcher correctness
- production relayer operations
- production guardian key management
- production fee policy
- production deployment readiness
- mainnet gateway readiness

## Current conclusion

Stage 2.5 can begin.

The guardian approval layer is closed for Stage 2.

The next implementation boundary is token mint CPI planning and then prototype implementation.

Before writing CPI code, the following decisions must be made:

1. SPL Token or Token-2022: closed, use standard SPL Token for first prototype
2. XXXL mint creation path: closed for prototype, pre-create outside gateway program
3. gateway PDA mint authority seed model: closed for prototype, use [b"mint_authority"], final authority remains open until Stake/Forge architecture
4. recipient token account creation/provision policy: closed for prototype, create outside gateway and validate in submit_mint_approval
5. compute budget strategy: closed for prototype, add ComputeBudgetProgram.setComputeUnitLimit in client/test transaction
