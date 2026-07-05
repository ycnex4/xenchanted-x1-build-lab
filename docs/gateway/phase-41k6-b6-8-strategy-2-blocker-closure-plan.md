# Phase 41K.6 B6.8 — Strategy 2 blocker closure plan

## Purpose

This document opens the Strategy 2 blocker closure plan after B6.7 manual placeholder boundary resolution.

B6.7 manual review resolved placeholder_program_id_boundary as a readiness/deployment blocker, not a structural PDA derivation constant.

Therefore, Strategy 2 remains viable for planning:

- upgrade the existing documented X1 testnet program,
- initialize required program state,
- configure SPL mint authority,
- configure guardian set,
- run one later-approved submit rehearsal.

This document does not approve signing.

This document does not approve transaction submission.

This document does not approve SOL spend.

This document does not approve private-key handling.

This document does not approve deploy.

This document does not approve program upgrade.

This document does not approve account initialization.

This document does not approve SPL mint setup.

This document does not approve guardian package construction.

This document does not approve submit rehearsal.

This document does not remove the B1C7 compile_error guard.

This document does not weaken the B1C7 feature gate.

This document does not open production or production-like activation.

## Current main checkpoint

B6.7 placeholder boundary manual resolution is merged on main:

3087f03 Merge phase 41K.6 B6.7 placeholder boundary manual resolution

Current decision remains:

NO-GO.

## Strategy status

- Strategy 1: closed_not_viable
- Strategy 2: viable_for_planning
- Strategy 3: fallback_if_later_structural_blocker_is_found
- Strategy 4: available_if_launch_path_must_stop

## Blockers before any testnet upgrade GO

All eight blockers remain open.

### A — Upgrade authority custody map

Status:

open.

Required evidence:

- public upgrade authority address confirmed
- custody owner identified outside repository
- custody mode described without secrets
- signing procedure described without key paths
- compromise handling described
- no private key, seed phrase, mnemonic, or keypair path recorded

Current known public address:

DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc

This is public-only metadata.

### B — Expected post-upgrade ProgramData hash

Status:

open.

Required evidence:

- local deployable artifact exists
- artifact path recorded without secrets
- artifact size recorded
- artifact hash recorded
- expected post-upgrade ProgramData hash recorded
- hash reproduction command recorded
- binary size remains under loader and practical limits

This cannot close before a deployable runtime artifact exists.

### C — B1C7 handler presence verification

Status:

open.

Required evidence:

- upgraded runtime includes real ConsumeGatewayMint handler path
- handler accepts expected instruction layout
- handler validates account contract
- handler validates guardian quorum
- handler checks processed event before mint
- handler marks processed event atomically with mint
- handler rejects replay
- handler rejects invalid guardian package
- handler rejects wrong SPL Token program
- handler rejects wrong mint authority PDA
- B1C7 dangerous gate remains impossible to include accidentally

### D — State initialization instruction design

Status:

open.

Required evidence:

- init instruction enum or instruction tags defined
- gateway_config PDA derivation defined
- guardian_set PDA derivation defined
- mint_state PDA derivation defined
- processed_event PDA derivation defined
- account sizes defined
- account owner expectations defined
- init order defined
- idempotency guards defined
- AlreadyInitialized errors defined
- wrong owner and wrong PDA errors defined

### E — SPL mint authority architecture

Status:

open.

Required evidence:

- mint authority target is gateway mint authority PDA or equivalent program-controlled authority
- wallet mint authority is not accepted as final architecture
- freeze authority policy defined
- decimals defined
- SPL Token program id verified
- CPI mint_to account list defined
- signer seeds and bump verification defined

### F — Guardian set testnet descriptor

Status:

open.

Required evidence:

- guardian set id defined
- guardian public keys defined
- threshold defined
- threshold satisfies 1 <= threshold <= guardian_count
- duplicate pubkeys rejected
- status active/inactive/deprecated defined
- guardian set PDA derivation defined
- testnet-only status clearly marked

No private guardian keys may be recorded.

### G — Rollback or recovery plan

Status:

open.

Required evidence:

- post-upgrade verification steps defined
- failure detection defined
- next-upgrade recovery path defined
- no downgrade assumption
- emergency no-submit condition defined
- stale or partially initialized state handling defined
- authority access requirements for recovery defined without secrets

### H — Local validator dry-run

Status:

open.

Required evidence:

- local validator build path defined
- local upgrade simulation performed
- local state initialization simulation performed
- SPL mint setup simulation performed
- guardian package simulation performed
- simulateTransaction submit rehearsal performed
- replay rejection verified
- negative cases verified

This must close before any testnet upgrade GO.

## Recommended execution order

The safe planning and implementation order is:

1. B6.8 blocker closure plan.
2. Runtime upgrade target design.
3. State account model and initialization design.
4. SPL mint authority architecture.
5. Guardian set descriptor.
6. Handler presence and account contract verification.
7. Local build artifact and hash.
8. Local validator dry-run.
9. Upgrade authority custody map.
10. GO form review.
11. Only then consider testnet upgrade GO.

## Next implementation boundary

The next implementation boundary should be local-only:

B6.9 — Runtime upgrade target design and handler inventory.

B6.9 should answer:

- what handlers must exist,
- what instruction tags must exist,
- what state accounts must exist,
- what PDA seeds must exist,
- what tests must prove before local build artifact hashing,
- which compile guards must remain intact.

B6.9 must not sign.

B6.9 must not submit.

B6.9 must not spend SOL.

B6.9 must not deploy.

B6.9 must not upgrade.

B6.9 must not initialize testnet accounts.

## Current decision

Current decision:

NO-GO.

This Strategy 2 blocker closure plan does not authorize live action.
