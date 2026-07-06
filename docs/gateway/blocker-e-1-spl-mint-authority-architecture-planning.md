# Blocker E.1 — SPL mint authority architecture planning

Status:

BLOCKER_E_OPEN_SPL_MINT_AUTHORITY_ARCHITECTURE_PLANNING_ONLY_NO_SPL_SETUP_NO_RPC_NO_EXECUTION

Current decision:

BLOCKER_E_NOT_CLOSED

NO-GO REMAINS_FOR_SPL_MINT_SETUP_SPL_AUTHORITY_TRANSFER_SPL_CPI_MINTING_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

## Purpose

Blocker E.1 opens the SPL mint authority architecture track.

E.1 is planning-only.

It does not create an SPL mint.

It does not configure mint authority.

It does not transfer mint authority.

It does not set freeze authority.

It does not mint tokens.

It does not initialize state.

It does not call RPC.

It does not use testnet.

It does not sign.

It does not deploy, upgrade, construct guardian packages, submit, or mutate any network.

## Why E follows D

Blocker D closed narrowly for state initialization design and invariants only.

D explicitly separated SPL mint setup and SPL mint authority architecture into Blocker E.

Therefore E must define the SPL mint authority architecture before any state-init execution, SPL setup, or deployable artifact can be accepted.

## Current repo-grounded SPL facts

Current source contains a gateway_mint_authority PDA inventory.

Current source contains a MintToCpi boundary that references:

- token_program
- mint
- recipient_token_account
- mint_authority_pda

Current source asserts that token_program is spl_token::id().

Current source asserts that mint_authority_pda matches the gateway_mint_authority PDA derived from the program id.

Current source can build a spl_token mint_to instruction.

Current source signs through gateway_mint_authority signer seeds.

Current source keeps SPL mint CPI execution disabled by default.

Current source returns CpiBoundaryNotReady when the guarded SPL CPI path is not enabled.

Current deployment status remains deployable=false.

Current program id status remains placeholder-boundary active.

## E architecture questions

Blocker E must answer:

- Is XXXL a classic SPL Token mint or Token-2022 mint?
- What exact token program is canonical?
- What exact mint account is canonical?
- What exact decimals value is canonical?
- What exact initial supply is allowed?
- Who creates the SPL mint?
- Who pays rent for the SPL mint?
- When is mint authority assigned to gateway_mint_authority PDA?
- Is there any temporary setup authority?
- How is temporary setup authority eliminated or constrained?
- Is freeze authority disabled, set to none, or assigned to a reviewed PDA?
- Can any human key ever mint after setup?
- Can any admin key ever change mint authority after setup?
- How does MintState record mint_pubkey, gateway_mint_authority_pda, and bump?
- How is SPL total supply reconciled with MintState total_supply?
- How does RecipientBalance relate to actual SPL token accounts?
- What evidence proves no manual mint path exists?
- What evidence proves SPL CPI minting is only reachable through gateway-authorized consumption?

## Initial candidate models

E.1 records candidate models but does not select a final one:

1. Human/admin mint authority retained
   - expected decision: reject
   - reason: incompatible with no hidden admin mint and immutable emission principles

2. Temporary setup authority then transfer to gateway_mint_authority PDA
   - expected decision: possible testnet model only if transfer/freeze evidence is public and bounded
   - risk: requires exact custody and handoff proof

3. Mint created with gateway_mint_authority PDA as authority from the start
   - expected decision: preferred if tool/runtime path can support it safely
   - risk: requires exact PDA/program id/freeze authority evidence before execution

4. Freeze authority retained by human/admin
   - expected decision: likely reject unless explicitly justified for testnet and disclosed

5. Freeze authority disabled / none
   - expected decision: preferred production direction if compatible with launch mechanics

## Required future E evidence

Before Blocker E can close, the repo must record:

- exact token program choice
- exact canonical mint account model
- exact decimals
- exact initial supply rule
- exact mint authority model
- exact freeze authority model
- exact setup authority model
- exact authority handoff/freeze procedure if a temporary setup authority exists
- exact proof that no human/admin mint authority remains after setup
- exact proof that SPL CPI minting is disabled by default
- exact proof that SPL CPI minting requires gateway authorization before mint
- exact MintState relationship to SPL mint pubkey and PDA authority
- exact total supply reconciliation model
- exact no-manual-mint/no-admin-supply-control boundary
- exact statement that E closure does not approve execution

## Relationship to previous blockers

Blocker A:

- upgrade authority remains accepted only for test phase
- A does not approve SPL mint authority

Blocker C:

- B1C7 handler boundary is reviewed
- C does not approve SPL mint CPI activation

Blocker D:

- state initialization design is reviewed
- D does not approve SPL mint setup

## Non-closure statement

E.1 does not close Blocker E.

E.1 does not approve:

- SPL mint creation
- SPL mint initialization
- mint authority assignment
- mint authority transfer
- freeze authority assignment
- freeze authority disablement
- SPL CPI minting
- state initialization execution
- signing
- keypair use
- deploy
- write-buffer
- set-upgrade-authority
- close
- upgrade
- guardian package construction
- transaction submit
- mutation
- production activation

## Result

Current status:

BLOCKER_E_OPEN_SPL_MINT_AUTHORITY_ARCHITECTURE_PLANNING_ONLY_NO_SPL_SETUP_NO_RPC_NO_EXECUTION

Current decision:

BLOCKER_E_NOT_CLOSED

NO-GO REMAINS_FOR_SPL_MINT_SETUP_SPL_AUTHORITY_TRANSFER_SPL_CPI_MINTING_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

## Next safe step

Blocker E.2 — repo-grounded SPL mint authority and CPI inventory.

E.2 should inspect tracked repository code only.

E.2 must not create an SPL mint, configure authority, call RPC, use testnet, sign, deploy, upgrade, initialize state, construct guardian packages, submit, or mutate.
