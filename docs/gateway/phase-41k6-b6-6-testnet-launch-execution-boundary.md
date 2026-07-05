# Phase 41K.6 B6.6 — Testnet launch and test execution boundary

## Purpose

This document opens the planning boundary for a real X1 testnet launch and test execution sequence after B6.5 discovery.

B6.5 confirmed that the documented X1 testnet program exists, ProgramData matches, upgrade authority decoding matches after correction, and the SPL Token program is executable.

B6.5 also confirmed that the documented X1 testnet program currently has zero program-owned state accounts.

Therefore, a real testnet launch and test sequence cannot be completed by read-only discovery alone.

A real testnet test sequence requires explicit later approval for one or more live actions.

This document does not approve signing.

This document does not approve transaction submission.

This document does not approve SOL spend.

This document does not approve private-key handling.

This document does not approve deploy.

This document does not approve program upgrade.

This document does not approve account initialization.

This document does not approve SPL mint initialization.

This document does not approve route activation.

This document does not remove the B1C7 compile_error guard.

This document does not weaken the B1C7 feature gate.

This document does not open production or production-like activation.

## Current main checkpoint

Account initialization requirement is merged on main:

d456366 Merge phase 41K.6 B6.5 account initialization requirement

Current decision remains:

NO-GO.

## What B6.5 established

B6.5 established the following public baseline facts:

- documented X1 testnet program exists
- program account is executable
- ProgramData account matches the expected ProgramData address
- upgrade authority matches after decoder correction
- SPL Token program account exists
- SPL Token program account is executable
- no program-owned runtime state accounts were observed for the documented program

B6.5 did not establish submit readiness.

B6.5 did not establish route readiness.

B6.5 did not establish guardian readiness.

B6.5 did not establish mint readiness.

B6.5 did not establish recipient readiness.

B6.5 did not establish processed-event readiness.

## Required live testnet work

A complete testnet launch and test sequence requires the following work categories:

1. Build or select the testnet runtime artifact.

2. Decide whether the existing deployed program can be used as-is.

3. If the current deployed program cannot initialize the required accounts, prepare a testnet program upgrade boundary.

4. Create or verify the target SPL mint.

5. Set or verify the gateway mint authority PDA as mint authority.

6. Create or initialize mint_state.

7. Create or initialize gateway_config.

8. Create or initialize guardian_set.

9. Create or verify recipient owner.

10. Create or verify recipient token account.

11. Select a canonical test event.

12. Derive or create processed_event PDA for that event.

13. Build a no-send package using real testnet values.

14. Obtain guardian evidence for the real testnet package.

15. Prepare a signed submit rehearsal.

16. Submit exactly one approved testnet transaction.

17. Perform read-only post-submit observation.

18. Record outcome and close B6.

No item above is approved by this document.

## Required future approval classes

Before any live action, a future decision must explicitly approve the relevant class.

### Class A — Build-only local artifact

This class may include:

- local build
- local test
- local artifact hash
- local artifact size
- local artifact path recording

This class must not include deploy, upgrade, signing, submit, SOL spend, or private-key handling.

### Class B — Testnet program upgrade

This class may include:

- program upgrade transaction
- upgrade authority signing
- SOL spend for upgrade fees
- post-upgrade read-only verification

This class requires explicit written GO.

### Class C — Testnet state initialization

This class may include:

- account creation
- rent funding
- mint_state initialization
- gateway_config initialization
- guardian_set initialization
- recipient account preparation
- processed_event account preparation if required

This class requires explicit written GO.

### Class D — Testnet SPL mint setup

This class may include:

- SPL mint creation
- mint authority assignment
- recipient token account creation
- token account verification

This class requires explicit written GO.

### Class E — Testnet guardian evidence package

This class may include:

- real payload construction
- guardian evidence collection
- quorum package construction
- no-send rehearsal with real values

This class does not by itself approve submit.

### Class F — Testnet submit rehearsal

This class may include:

- one explicitly approved transaction
- one explicitly approved fee payer
- one explicitly approved max SOL spend
- one explicitly approved package hash
- post-submit observation

This class requires explicit written GO.

## Minimum GO fields for any signed step

Any signed step must include:

- approved_by
- approved_at_utc
- approved_commit
- approved_scope
- approved_network
- approved_program_id
- approved_fee_payer_public_address
- approved_max_sol_spend
- approved_commands_or_procedure
- approved_abort_conditions
- approved_post_action_observation
- explicit_no_production_activation

Missing means not approved.

Null means not approved.

Empty means not approved.

## Required testnet launch strategy decision

Before any live work, we must choose one strategy:

### Strategy 1 — Existing program, state initialization only

Use the existing documented X1 testnet program and initialize missing runtime state if the deployed runtime supports it.

Required proof before GO:

- instruction support exists
- account initialization procedure exists
- state layout is compatible
- no upgrade required
- post-initialization verification is defined

### Strategy 2 — Program upgrade then state initialization

Upgrade the existing documented X1 testnet program to a runtime that can initialize and test the required state.

Required proof before GO:

- local build passes
- artifact hash is recorded
- upgrade boundary is reviewed
- upgrade authority is confirmed
- post-upgrade verification is defined
- state initialization boundary is defined

### Strategy 3 — New testnet deployment

Deploy a new testnet program and use that baseline for the test sequence.

Required proof before GO:

- new program id is selected
- new ProgramData address is recorded after deploy
- upgrade authority is recorded
- PDA derivations are regenerated
- old baseline is not reused accidentally

### Strategy 4 — Stop and redesign

Do not run a live testnet sequence yet.

Use B6 findings to redesign the runtime/init/deploy path.

## Local runtime capability inventory

Local runtime capability inventory is recorded in:

docs/gateway/phase-41k6-b6-6-local-runtime-capability-inventory.md

This inventory inspects local source files only.

It does not use RPC, sign, submit, spend SOL, load private keys, deploy, upgrade, or initialize accounts.

Current decision remains:

NO-GO.

## Recommended next step

The next safe step after this boundary is a local capability inventory:

- inspect current Rust/SVM source
- inspect instruction support
- inspect whether state initialization instructions exist
- inspect whether current deployed baseline can possibly create the required state
- inspect whether an upgrade is required
- record the result as docs-only evidence

That next step must not sign.

That next step must not submit.

That next step must not spend SOL.

That next step must not load private keys.

That next step must not deploy.

## B6.7 placeholder program id boundary analysis

B6.7 placeholder program id boundary analysis is recorded in:

docs/gateway/phase-41k6-b6-7-placeholder-boundary-analysis.md

This analysis records the Strategy 1 closure, the Strategy 2 conditional recommendation, and the placeholder boundary question that decides Strategy 2 vs Strategy 3.

It does not approve signing, submission, SOL spend, private-key handling, deploy, upgrade, account initialization, SPL mint setup, guardian package, or submit rehearsal.

Current decision remains:

NO-GO.

## B6.7 placeholder boundary manual resolution

B6.7 placeholder boundary manual resolution is recorded in:

docs/gateway/phase-41k6-b6-7-placeholder-boundary-manual-resolution.md

Manual review resolved the automated structural-risk result as a false positive for the reviewed PDA derivation path.

Strategy 2 remains viable for planning, but no live action is approved.

Current decision remains:

NO-GO.

## B6.8 Strategy 2 blocker closure plan

B6.8 Strategy 2 blocker closure plan is recorded in:

docs/gateway/phase-41k6-b6-8-strategy-2-blocker-closure-plan.md

This plan records the eight blockers that must close before any testnet upgrade GO:

- upgrade authority custody map
- expected post-upgrade ProgramData hash
- B1C7 handler presence verification
- state initialization instruction design
- SPL mint authority architecture
- guardian set testnet descriptor
- rollback or recovery plan
- local validator dry-run

Strategy 2 remains viable for planning, but no live action is approved.

Current decision remains:

NO-GO.

## Current decision

Current decision:

NO-GO.

This testnet launch execution boundary does not authorize live action.
