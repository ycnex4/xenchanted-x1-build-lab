# Phase 41K.6 B6.27 — Blocker closure readiness map

Status:

LOCAL_ONLY_NOT_DEPLOYABLE

Current decision:

NO-GO

## Purpose

This document maps the current B6 Strategy 2 readiness blockers to the local execution layer checkpoint produced in B6.26.

It does not close any GO blocker by itself.

It defines what is already covered locally and what evidence is still required before any blocker can be considered closed.

## Safety boundary

This is a docs-only readiness map.

It does not approve:

- deploy
- upgrade
- signing
- submit
- SOL spend
- account initialization on testnet
- SPL mint setup
- SPL CPI minting
- guardian package construction
- submit rehearsal
- production activation

Current decision remains:

NO-GO.

## Current local evidence

The local execution layer checkpoint is recorded in:

docs/gateway/phase-41k6-b6-26-local-execution-layer-checkpoint.md

The current local layer covers:

- instruction tags
- instruction codec
- instruction payload skeletons
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
- local end-to-end execution scenario skeleton

All of the above remains:

LOCAL_ONLY_NOT_DEPLOYABLE

## Blocker A — upgrade authority custody map

Status:

OPEN

Current coverage:

- No B6.11-B6.26 local skeleton closes custody.
- No private key or keypair handling is included.
- No signing procedure is approved.

Required closure evidence:

- Explicit custody map for the upgrade authority.
- Confirmation of who controls signing.
- Confirmation that no private key is exposed in docs, scripts, logs, prompts, or repo.
- Explicit max-risk action boundary.
- Written GO for upgrade action, if upgrade is ever attempted.

Closure state:

NOT CLOSED.

## Blocker B — expected post-upgrade ProgramData hash

Status:

OPEN

Current coverage:

- B6.11-B6.26 did not produce a deployable binary.
- B6.11-B6.26 did not produce an expected post-upgrade ProgramData hash.
- B6.11-B6.26 did not approve upgrade.

Required closure evidence:

- Reproducible local build procedure.
- Explicit artifact identity.
- Expected post-upgrade ProgramData hash.
- Verification command for post-upgrade read-only inspection.
- Rollback boundary.
- Written GO for upgrade action, if upgrade is ever attempted.

Closure state:

NOT CLOSED.

## Blocker C — B1C7 handler presence verification

Status:

OPEN_DESIGN_STARTED

Current coverage:

- Local instruction tags exist.
- Local codec exists.
- Local typed instruction skeleton exists.
- Local dispatch and validated dispatch exist.
- Unified local execution planning exists.
- Local scenario test exists.

Remaining gap:

- No live runtime handler is enabled.
- No B1C7 guard is removed.
- No handler presence is verified inside a deployable runtime path.
- No live route is enabled.

Required closure evidence:

- Explicit handler boundary document.
- Proof that B1C7 guard status is intentional.
- Proof that any future handler path preserves existing safety gates.
- Local validator evidence only after separate dry-run GO.
- No testnet action without explicit written GO.

Closure state:

NOT CLOSED.

## Blocker D — state initialization instruction design

Status:

OPEN_DESIGN_STARTED

Current coverage:

- Local state account layout skeleton exists.
- Local state initialization skeleton exists.
- Initialization execution plan skeleton exists.
- Local scenario includes gateway_config, guardian_set, and mint_state initialization planning.

Remaining gap:

- No on-chain account creation.
- No rent-funded account initialization.
- No live runtime handler.
- No testnet account initialization.
- No deployed state accounts.

Required closure evidence:

- Account creation procedure design.
- Rent and account size verification.
- Owner and PDA verification procedure.
- Local-validator-only dry-run plan.
- Explicit written GO before any testnet initialization.

Closure state:

NOT CLOSED.

## Blocker E — SPL mint authority architecture

Status:

OPEN_DESIGN_STARTED

Current coverage:

- Local mint_state layout includes mint authority PDA fields.
- Local execution planning carries mint authority PDA and bump.
- Local consume transition updates local total_minted.
- Documentation explicitly keeps SPL CPI minting disabled.

Remaining gap:

- No SPL mint authority transfer.
- No SPL mint setup.
- No SPL CPI mint_to.
- No live token minting.
- No recipient token account mutation.

Required closure evidence:

- SPL mint authority ownership model.
- PDA signer seed verification.
- Mint authority handoff procedure.
- SPL Token program ID verification.
- Local-validator-only CPI test design.
- Explicit written GO before any SPL mint setup or CPI.

Closure state:

NOT CLOSED.

## Blocker F — guardian set testnet descriptor

Status:

OPEN_DESIGN_STARTED

Current coverage:

- Guardian set header payload and account layout skeletons exist.
- Local initialization scenario includes guardian_set initialization planning.
- Threshold and guardian count validation exist locally.

Remaining gap:

- No production guardian set.
- No testnet guardian descriptor.
- No guardian public key package.
- No live guardian signature path.
- No guardian package construction.

Required closure evidence:

- Testnet guardian descriptor.
- Guardian public key list.
- Threshold policy.
- Rotation policy.
- Descriptor hash or equivalent integrity marker.
- Explicit written GO before guardian package construction or live signature use.

Closure state:

NOT CLOSED.

## Blocker G — rollback and recovery plan

Status:

OPEN

Current coverage:

- No B6.11-B6.26 local skeleton closes rollback.
- No upgrade is approved.

Required closure evidence:

- Upgrade abort conditions.
- Read-only post-action verification commands.
- Recovery path for failed upgrade.
- Recovery path for mismatched ProgramData hash.
- Recovery path for incorrect state initialization.
- Explicit stop conditions.

Closure state:

NOT CLOSED.

## Blocker H — local validator dry-run

Status:

OPEN

Current coverage:

- Local skeleton tests exist.
- Local execution scenario exists.
- No validator dry-run is performed.

Remaining gap:

- No local validator deployment.
- No local validator account initialization.
- No local validator SPL CPI test.
- No local transaction execution path.

Required closure evidence:

- Local-validator-only plan.
- Explicit non-testnet command boundary.
- Fixture accounts.
- Expected logs and state deltas.
- Abort conditions.
- Separate written GO for local-validator dry-run.

Closure state:

NOT CLOSED.

## Summary table

| Blocker | Status | Closed |
|---|---|---|
| A — upgrade authority custody map | OPEN | no |
| B — expected post-upgrade ProgramData hash | OPEN | no |
| C — B1C7 handler presence verification | OPEN_DESIGN_STARTED | no |
| D — state initialization instruction design | OPEN_DESIGN_STARTED | no |
| E — SPL mint authority architecture | OPEN_DESIGN_STARTED | no |
| F — guardian set testnet descriptor | OPEN_DESIGN_STARTED | no |
| G — rollback/recovery plan | OPEN | no |
| H — local validator dry-run | OPEN | no |

## Checkpoint conclusion

B6.11-B6.26 created a coherent local execution planning layer.

B6.27 maps that layer against the open GO blockers.

No GO blocker is closed by this checkpoint.

Current decision remains:

NO-GO.
