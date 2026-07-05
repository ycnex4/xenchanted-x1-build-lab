# Phase 41K.6 B6.29 — Local-validator dry-run design map

Status:

LOCAL_VALIDATOR_ONLY_DESIGN_NOT_EXECUTED

Current decision:

NO-GO

## Purpose

This document maps the future local-validator-only dry-run boundary for blocker H.

It is a docs-only design map.

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

## Why this step exists

B6.11 through B6.25 built the local Rust execution planning layer.

B6.26 checkpointed that local execution layer.

B6.27 mapped blockers A-H.

B6.28 mapped the B1C7 handler boundary.

B6.29 defines what a future local-validator-only dry-run would need to prove before any live testnet action can even be considered.

## Dry-run boundary

The future dry-run, if separately approved later, must be limited to local validator only.

It must not use:

- X1 testnet
- live RPC
- real fee payer
- real upgrade authority signing
- production guardian keys
- production guardian packages
- production SPL mint
- production recipient accounts
- production bridge messages

The dry-run must use only local fixtures.

## Required dry-run components

A future local-validator dry-run design must define:

1. Local program build identity.
2. Local validator startup boundary.
3. Local deployment boundary.
4. Local program id boundary.
5. Local SPL mint fixture.
6. Local gateway_config account fixture.
7. Local guardian_set account fixture.
8. Local mint_state account fixture.
9. Local processed_event account fixture.
10. Local recipient token account fixture.
11. Local guardian signature fixture or disabled-signature fixture boundary.
12. Expected instruction data.
13. Expected account order.
14. Expected success state delta.
15. Expected failure matrix.
16. Expected no-mutation guarantees on failure.
17. Expected SPL CPI boundary.
18. Expected logs.
19. Abort conditions.
20. Cleanup procedure.

## Required success scenario

A future local-validator-only success scenario must prove:

- instruction decode works
- account order validation works
- signer validation works
- writable validation works
- owner validation works
- gateway_config validation works
- guardian_set validation works
- mint_state validation works
- processed_event replay protection works
- recipient token account validation works
- SPL mint validation works
- mint_state.total_minted changes only on success
- processed_event is marked only on success
- recipient token balance changes only on success
- all success mutations are atomic

## Required failure scenarios

A future local-validator-only failure matrix must prove no mutation on:

- wrong account count
- wrong account order
- wrong signer
- wrong writable flag
- wrong account owner
- wrong gateway_config discriminator
- wrong guardian_set discriminator
- wrong mint_state discriminator
- wrong processed_event discriminator
- wrong route id
- wrong source chain id
- wrong guardian set id
- wrong mint
- wrong mint authority PDA
- wrong mint authority bump
- wrong recipient token account
- wrong recipient owner
- wrong recipient mint
- replayed processed_event
- zero amount
- amount overflow
- invalid guardian quorum
- invalid signature package
- malformed instruction data
- truncated account data
- low rent account
- inactive mint_state

## Evidence required before blocker H can close

Blocker H cannot close from this document alone.

Required evidence before closure:

- explicit local-validator-only command plan
- explicit non-testnet boundary
- fixture generation plan
- expected account snapshots before execution
- expected account snapshots after execution
- focused success test result
- focused failure matrix test result
- mutation-invariance evidence
- proof that no live RPC is used
- proof that no real signing material is used
- proof that no testnet submit is performed

## Explicit non-closure

This checkpoint does not close blocker H.

It only defines the design map for a future local-validator-only dry-run.

Current blocker H state:

OPEN_DESIGN_STARTED

Current decision remains:

NO-GO.

## Next safe step

The next safe step after this document is to create a docs-only fixture inventory map for local-validator dry-run planning.

No local-validator execution is approved by this checkpoint.

Current decision remains:

NO-GO.
