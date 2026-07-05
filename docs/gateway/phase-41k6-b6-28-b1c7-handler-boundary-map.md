# Phase 41K.6 B6.28 — B1C7 handler boundary map

Status:

LOCAL_ONLY_NOT_DEPLOYABLE

Current decision:

NO-GO

## Purpose

This document maps the boundary for blocker C:

B1C7 handler presence verification.

It is a docs-only boundary map.

It does not enable a live runtime handler.

It does not remove, weaken, bypass, or reinterpret the B1C7 guard.

It does not approve deploy, upgrade, signing, submit, SOL spend, account initialization, SPL mint setup, guardian package construction, SPL CPI minting, or submit rehearsal.

## Current B1C7 status

Current status remains:

OPEN_DESIGN_STARTED

B1C7 is not closed.

No live route is enabled.

No deployable runtime handler is approved.

Current decision remains:

NO-GO.

## What B6.11-B6.27 already provide

The local planning layer now provides local-only structure around the future handler boundary:

- reserved instruction tags
- instruction codec
- payload skeletons
- typed instruction skeleton
- account order skeleton
- account validation skeleton
- validated dispatch skeleton
- state account layout skeleton
- state initialization skeleton
- consume state transition skeleton
- initialization execution plan skeleton
- consume execution plan skeleton
- unified local execution plan skeleton
- local execution scenario skeleton
- local execution layer checkpoint
- blocker closure readiness map

These modules are local skeletons only.

They are not runtime handlers.

They do not write on-chain state.

They do not perform SPL CPI.

They do not initialize accounts on testnet.

## Handler boundary requirements

Before B1C7 can be considered for closure, a future handler boundary must prove all of the following:

1. B1C7 guard status is explicit.
2. Any handler path is gated behind an intentional feature or build boundary.
3. No accidental live route exists.
4. Instruction decode preserves the reserved tag mapping.
5. Payload parsing preserves canonical layout assumptions.
6. Account order is checked before state mutation.
7. Signer and writable expectations are checked before state mutation.
8. Account owner expectations are checked before state mutation.
9. Gateway config is checked before mint planning.
10. Guardian set is checked before authorization.
11. Mint state is checked before mint planning.
12. Processed event replay protection is checked before mint planning.
13. SPL mint and recipient token account checks are completed before CPI planning.
14. No account mutation occurs on validation failure.
15. No SPL CPI occurs on validation failure.
16. No processed event is marked before all preconditions pass.
17. Mint and processed event marking remain atomic at runtime level.
18. All failure branches preserve mutable account data.

## Required evidence before B1C7 closure

B1C7 cannot be closed with local skeleton tests alone.

Required closure evidence:

- handler boundary document
- feature gate or build gate document
- explicit B1C7 guard status
- local-validator-only runtime route design
- local-validator-only failure matrix
- local-validator-only mutation invariance tests
- proof that live testnet route remains disabled unless separately approved
- proof that SPL CPI remains disabled unless separately approved
- proof that account initialization remains disabled unless separately approved
- proof that no private key, keypair path, or signing material is introduced

## Explicit non-closure

This checkpoint does not close blocker C.

It only defines the boundary and evidence required for later closure.

Current blocker C state:

OPEN_DESIGN_STARTED

Current decision remains:

NO-GO.

## Next safe step after this map

The next safe step is a docs-only local-validator dry-run design map.

That future step must remain non-live and must not include testnet commands.

Any local-validator execution must require a separate explicit written GO.

Current decision remains:

NO-GO.
