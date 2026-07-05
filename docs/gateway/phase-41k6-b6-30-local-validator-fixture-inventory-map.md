# Phase 41K.6 B6.30 — Local-validator fixture inventory map

Status:

LOCAL_VALIDATOR_ONLY_FIXTURE_DESIGN_NOT_EXECUTED

Current decision:

NO-GO

## Purpose

This document defines the fixture inventory required for a future local-validator-only dry-run.

It extends the B6.29 local-validator dry-run design map.

It is docs-only.

It does not run a local validator.

It does not build, deploy, upgrade, sign, submit, spend SOL, initialize testnet accounts, configure SPL mint authority, perform SPL CPI minting, construct guardian packages, or rehearse live submit flow.

## Current blocker H status

Blocker H:

local validator dry-run

Current status:

OPEN_DESIGN_STARTED

Closure state:

NOT CLOSED

Current decision remains:

NO-GO.

## Fixture boundary

All fixtures described here are for a future local validator only.

They must not use:

- X1 testnet
- live RPC
- real fee payer
- real upgrade authority signing
- production guardian keys
- production guardian packages
- production SPL mint
- production recipient accounts
- production bridge messages

All fixture values must be deterministic, local, disposable, and non-production.

## Required fixture groups

A future local-validator-only dry-run needs the following fixture groups:

1. Program fixture.
2. SPL Token fixture.
3. Gateway config fixture.
4. Guardian set fixture.
5. Mint state fixture.
6. Processed event fixture.
7. Recipient token account fixture.
8. Instruction data fixture.
9. Account metas fixture.
10. Guardian authorization fixture.
11. Success snapshot fixture.
12. Failure snapshot fixture.
13. Log expectation fixture.
14. Mutation-invariance fixture.

## Program fixture

Purpose:

Represent the local test program under local validator.

Required fields:

- local_program_id
- local_program_binary_identity
- local_program_build_profile
- local_deploy_boundary
- b1c7_guard_status
- live_route_enabled: false unless separately approved for local validator only

Required evidence before execution:

- local-only program id
- no testnet program id
- no production program id
- no upgrade authority signing
- no live RPC

## SPL Token fixture

Purpose:

Represent a local SPL mint and local recipient token account.

Required fields:

- local_token_program_id
- local_mint
- local_mint_authority_pda
- local_mint_authority_bump
- local_decimals
- local_recipient_owner
- local_recipient_token_account
- initial_recipient_balance
- expected_recipient_balance_after_success

Required negative variants:

- wrong mint owner
- wrong mint authority
- wrong mint authority PDA
- wrong mint authority bump
- wrong recipient token owner
- wrong recipient token mint
- uninitialized mint
- uninitialized recipient token account
- low-rent token account

## Gateway config fixture

Purpose:

Represent local gateway route configuration.

Required fields:

- discriminator
- version
- bump
- route_id
- source_chain_id
- guardian_set_id
- gateway_mint_authority_pda
- is_active

Required negative variants:

- wrong discriminator
- wrong owner
- wrong route_id
- wrong source_chain_id
- wrong guardian_set_id
- inactive config
- truncated data
- low-rent account

## Guardian set fixture

Purpose:

Represent local guardian set metadata.

Required fields:

- discriminator
- version
- bump
- guardian_set_id
- threshold
- guardian_count
- status
- local guardian public key list or local disabled-signature boundary

Required negative variants:

- wrong discriminator
- wrong owner
- wrong guardian_set_id
- threshold greater than guardian_count
- zero threshold
- inactive status
- truncated data
- low-rent account

## Mint state fixture

Purpose:

Represent local mint accounting and mint authority state.

Required fields:

- discriminator
- version
- bump
- mint
- mint_authority_pda
- mint_authority_bump
- token_program
- decimals
- total_minted
- is_active

Required negative variants:

- wrong discriminator
- wrong owner
- wrong mint
- wrong mint_authority_pda
- wrong mint_authority_bump
- wrong token_program
- inactive state
- total_minted overflow boundary
- truncated data
- low-rent account

## Processed event fixture

Purpose:

Represent replay protection for a canonical event.

Required fields:

- discriminator
- version
- bump
- canonical_event_key
- message_hash
- source_burn_tx_hash
- source_burn_event_index
- recipient_hash
- minted_amount
- processed_at_slot

Required variants:

- empty uninitialized processed_event account
- already consumed processed_event account
- wrong canonical_event_key
- wrong message_hash
- wrong recipient_hash
- wrong route id binding if represented
- wrong source burn fields if represented
- truncated data
- low-rent account

## Instruction data fixture

Purpose:

Represent local instruction bytes for init and consume paths.

Required instruction fixtures:

- InitializeGatewayConfig
- InitializeGuardianSet
- InitializeMintState
- ConsumeGatewayMint

Required negative variants:

- empty instruction data
- invalid tag
- wrong tag for account context
- truncated payload
- oversized payload
- malformed amount
- zero amount boundary
- amount above allowed runtime boundary if applicable

## Account metas fixture

Purpose:

Represent ordered local account metadata.

Required account meta sets:

- InitializeGatewayConfig account order
- InitializeGuardianSet account order
- InitializeMintState account order
- ConsumeGatewayMint account order

Required negative variants:

- wrong account count
- wrong account order
- missing signer
- unexpected signer
- readonly where writable is required
- writable where readonly is required
- wrong owner expectation
- wrong program-derived address expectation

## Guardian authorization fixture

Purpose:

Represent local guardian authorization without using production guardian material.

Allowed fixture models:

- deterministic local test guardian keys
- disabled-signature boundary if still docs-only
- mocked guardian verification boundary for local-validator-only planning

Required negative variants:

- insufficient quorum
- duplicate guardian
- unknown guardian
- wrong guardian_set_id
- malformed signature package
- signature for wrong message
- expired or invalid finality boundary if represented

## Success snapshot fixture

Purpose:

Define expected state deltas after successful local consume.

Required before snapshot:

- mint_state.total_minted before
- recipient token balance before
- processed_event unconsumed before

Required after snapshot:

- mint_state.total_minted increased by mint_amount
- recipient token balance increased by mint_amount
- processed_event marked consumed
- gateway_config unchanged
- guardian_set unchanged
- SPL mint metadata unchanged except supply if CPI is enabled in local-only future step

## Failure snapshot fixture

Purpose:

Define no-mutation expectations on failure.

Required invariant:

For every negative test, mutable accounts must remain byte-identical unless a future design explicitly scopes a safe pre-validation write. The current expected invariant is no mutation on failure.

Required checked accounts:

- gateway_config
- guardian_set
- mint_state
- processed_event
- SPL mint
- recipient token account

## Log expectation fixture

Purpose:

Define expected logs for local-validator-only dry-run.

Required log categories:

- instruction tag decoded
- account order checked
- account validation result
- route validation result
- guardian validation result
- replay validation result
- mint planning result
- CPI boundary result
- success completion
- failure reason

Logs must not include:

- private signing material
- keypair paths
- secrets
- live RPC URLs
- production guardian material

## Mutation-invariance fixture

Purpose:

Prove no mutation on failure.

Required method:

- capture byte snapshots before instruction
- run local instruction
- compare all mutable account bytes after failure
- assert exact equality
- report the failing error code
- report account index and reason when relevant

## Evidence required before blocker H can close

This document does not close blocker H.

Required evidence before closure:

- concrete local fixture generator
- local-validator-only command plan
- no-testnet proof
- no-live-RPC proof
- no-real-signing-material proof
- success scenario execution result
- failure matrix execution result
- mutation-invariance result
- local logs
- cleanup result

## Explicit non-closure

This checkpoint does not close blocker H.

Current blocker H state:

OPEN_DESIGN_STARTED

Current decision remains:

NO-GO.

## Next safe step

The next safe step is a docs-only local-validator fixture generator design.

No local-validator execution is approved by this checkpoint.

Current decision remains:

NO-GO.
