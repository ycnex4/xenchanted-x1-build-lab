# Phase 41K.6 B6.45 — SPL mint authority architecture map

Status:

SPL_MINT_AUTHORITY_ARCHITECTURE_MAP_DESIGN_NOT_EXECUTED

Current decision:

NO-GO

## Purpose

This document maps blocker E:

SPL mint authority architecture.

It defines the architecture requirements for future XXXL mint authority control on X1/SVM.

This is docs-only.

It does not configure SPL mint authority.

It does not perform SPL CPI minting.

It does not run a local validator.

It does not build, deploy, upgrade, sign, submit, spend SOL, initialize testnet accounts, construct guardian packages, or rehearse live submit flow.

## Current blocker E status

Blocker E:

SPL mint authority architecture

Current status:

OPEN_DESIGN_STARTED

Closure state:

NOT CLOSED

Current decision remains:

NO-GO.

## Known public baseline

Known public baseline:

- x1_testnet_program_id: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
- expected_gateway_mint_authority_pda: BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG
- expected_gateway_mint_authority_bump: 252
- spl_token_program_id: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA

These are public identifiers only.

No signing material is stored in this repository.

No signing action is approved by this checkpoint.

## Architecture principle

The XXXL mint authority should be controlled by a program-derived address.

The program should mint XXXL only through a tightly scoped SPL Token CPI path.

The CPI path must be reachable only after all gateway preconditions pass.

The mint authority PDA must not be controlled by a wallet.

The gateway must not allow arbitrary minting.

## Required mint authority model

The required model is:

- XXXL SPL mint has mint authority set to gateway mint authority PDA.
- Gateway mint authority PDA is derived by the deployed gateway program.
- PDA seeds are fixed and documented.
- PDA bump is verified.
- Program signs CPI using PDA seeds.
- CPI mint amount is derived from the verified gateway message.
- CPI recipient token account must match the verified recipient.
- Replay protection must be checked before minting.
- Replay mark and mint accounting must be atomic with successful mint.

## Required PDA boundary

The mint authority PDA must be derived from stable seeds.

Known intended seed boundary:

- seed 1: xxxl
- seed 2: gateway-mint-authority
- seed 3: v1

Known expected bump:

252

Required verification before closure:

- PDA derivation matches expected address
- bump matches expected bump
- program id matches expected program id
- seed list is documented
- runtime uses the same seeds
- tests cover wrong PDA
- tests cover wrong bump
- tests cover wrong program id

## Required SPL mint boundary

Before blocker E can close, the target mint boundary must be explicit.

Required fields:

- mint public key
- token program public key
- decimals
- mint authority
- freeze authority policy
- supply before initialization if applicable
- authority handoff policy
- read-only verification command
- abort conditions

This checkpoint does not define or configure a live mint.

## Required CPI mint boundary

Future CPI minting must require:

- gateway_config active
- mint_state active
- guardian_set active
- route id valid
- source chain id valid
- mint token valid
- recipient hash valid
- recipient token account valid
- SPL mint valid
- token program valid
- mint authority PDA valid
- mint authority bump valid
- canonical event not processed
- guardian quorum valid
- amount greater than zero
- amount does not overflow
- all mutable accounts are writable
- all readonly accounts remain readonly
- all owners match expectations

CPI must not execute before these checks pass.

## Required atomicity model

Successful ConsumeGatewayMint must be atomic.

The intended atomic mutation set is:

- mint_state.total_minted increases by xxxlMintAmount
- processed_event is marked consumed
- recipient token account balance increases by xxxlMintAmount through SPL CPI
- SPL mint supply increases by xxxlMintAmount through SPL CPI

No partial success is acceptable.

If CPI fails, replay mark and mint_state accounting must not persist.

If validation fails, no mutable account must change.

## Required no-mutation failure model

For every failure before mint, future tests must verify:

- gateway_config unchanged
- guardian_set unchanged
- mint_state unchanged
- processed_event unchanged
- SPL mint unchanged
- recipient token account unchanged

Failure leaves mutable accounts byte-identical.

## Required negative cases

Blocker E closure requires negative coverage for:

- wrong token program
- wrong SPL mint
- wrong mint authority
- wrong mint authority PDA
- wrong mint authority bump
- wrong recipient token account
- wrong recipient owner
- wrong recipient mint
- uninitialized SPL mint
- uninitialized recipient token account
- readonly SPL mint when writable is required
- readonly recipient token account when writable is required
- CPI failure
- amount overflow
- zero amount
- replayed processed_event
- mint_state inactive
- gateway_config inactive
- guardian_set inactive

## Required local-validator evidence

Before blocker E can close as execution-ready, local-validator evidence should show:

- local SPL mint fixture exists
- local recipient token account fixture exists
- local mint authority PDA fixture exists
- local CPI success path works
- local CPI failure path preserves no-mutation invariant
- local wrong-authority cases fail
- local wrong-recipient cases fail
- local replay case fails
- local success path mutates only expected accounts

This checkpoint does not run local validator.

## Required testnet readiness evidence

Before any testnet SPL mint authority action, evidence must include:

- scoped written GO
- target mint public key
- token program public key
- expected mint authority PDA
- current mint authority read-only baseline
- authority change command if needed
- maximum fee boundary
- abort conditions
- post-action read-only verification
- rollback and recovery policy

No testnet mint authority action is approved by this checkpoint.

## Forbidden until scoped GO

The following remain forbidden:

- creating a testnet SPL mint
- changing testnet SPL mint authority
- setting freeze authority
- minting XXXL on testnet
- running SPL CPI minting on testnet
- submitting gateway mint transactions
- enabling live route
- using production mint
- using production recipient accounts
- using real signing material

## Relationship to other blockers

Blocker E depends on or interacts with:

- blocker C: handler must not bypass B1C7 guard
- blocker D: state initialization must define mint_state and account layout
- blocker F: guardian descriptor must be valid before mint authorization
- blocker G: rollback and recovery must cover SPL authority and CPI failures
- blocker H: local-validator dry-run must prove CPI and no-mutation behavior

Current state:

These blockers are not closed for execution readiness.

## Explicit non-closure

This checkpoint does not close blocker E.

It maps SPL mint authority architecture requirements only.

Current blocker E state:

OPEN_DESIGN_STARTED

Current decision remains:

NO-GO.

## Next safe step

The next safe step is a docs-only guardian set testnet descriptor map for blocker F.

No SPL mint authority action is approved by this checkpoint.

No SPL CPI minting is approved by this checkpoint.

No local-validator execution is approved by this checkpoint.

No testnet action is approved by this checkpoint.

Current decision remains:

NO-GO.
