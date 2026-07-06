# Blocker C.1 — B1C7 handler production/testnet boundary planning

Status:

BLOCKER_C_OPEN_B1C7_HANDLER_BOUNDARY_PLANNING_ONLY_NO_CODE_CHANGE_NO_RPC_NO_EXECUTION

Current decision:

BLOCKER_C_NOT_CLOSED

NO-GO REMAINS_FOR_HANDLER_ACTIVATION_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

## Purpose

Blocker C.1 opens the B1C7 handler production/testnet boundary track after Blocker A was closed narrowly.

C.1 is planning-only.

It does not activate the B1C7 handler.

It does not change runtime code.

It does not build a deployable artifact.

It does not call RPC.

It does not use testnet.

It does not sign.

It does not deploy, upgrade, initialize state, configure SPL, construct guardian packages, submit, or mutate any network.

## Why C follows A

Blocker A is now closed narrowly as:

UPGRADE_AUTHORITY_PRESENT_BUT_ACCEPTED_FOR_TEST_PHASE

That closure only resolves the authority-model question.

It does not make the runtime deployable.

The next architectural blocker is C:

B1C7 handler production/testnet boundary.

## Current repo-grounded C status

Current repo evidence shows:

- B1C7 integration exists behind feature gate: phase-41k6-b1c7-handler-integration-test-gate
- dangerous SBF build allow feature is separately named and explicit
- default consume_gateway_mint path rejects with CpiBoundaryNotReady when the B1C7 gate is not enabled
- LIVE_ROUTE_ACTIVATION_FROM_PROCESS_INSTRUCTION_ENABLED remains false
- deployment_status.rs still records deployable=false
- program_id_status.rs still records PLACEHOLDER_PROGRAM_ID_BOUNDARY
- lib.rs still records SCAFFOLD_ONLY_NOT_DEPLOYABLE

C.1 interpretation:

The repo has a meaningful B1C7 handler path, but it is not yet a production/testnet-ready live route.

## Blocker C problem statement

Blocker C asks:

Can the B1C7 guardian-authorized ConsumeGatewayMint handler be treated as a reviewed production/testnet boundary?

This requires proving:

- the handler is intentionally gated today
- any future activation removes or changes gates only through a reviewed step
- authorization happens before mark and mint
- failed authorization causes no mutation
- replay protection remains check-before-mark
- processed_event marking and SPL mint are atomic in the intended runtime path
- live route activation is explicit and separately reviewed
- testnet activation cannot accidentally include broader deployment permissions

## Current known safeguards

Current safeguards visible in repo:

- B1C7 integration is feature-gated
- dangerous SBF build allow feature is named explicitly
- default route fails closed
- live route flag remains false
- SPL CPI execution remains separately gated
- deployable=false remains active

## Current known gaps

Current gaps:

- no production/testnet handler activation decision
- no deployable handler artifact hash
- no local-validator program-load test for the handler path
- no state initialization execution package
- no SPL mint authority architecture closure
- no guardian descriptor closure
- no rollback closure
- no final testnet GO package

## Required C closure evidence

Before Blocker C can close, the repo must record:

- exact handler entrypoint being evaluated
- exact feature-gate status
- exact live route activation status
- exact authorization-before-mutation invariant
- exact replay check-before-mark invariant
- exact no-mutation-on-failure cases
- exact account contract required by the handler
- exact dependency on D/E/F/B/G blockers
- exact local test or review evidence
- explicit statement that C closure does not approve deploy/upgrade/state/SPL/submit

## Proposed C path

Recommended path:

1. C.2 — repo-grounded B1C7 handler inventory
   Inspect handler code, feature gates, account contract, verifier path, CPI gate, and tests.

2. C.3 — B1C7 production/testnet activation decision model
   Decide what it would mean to convert the test-gated path into a testnet-intended path.

3. C.4 — handler invariant review package
   Record authorization-before-mutation, no-mutation-on-failure, replay, and account contract evidence.

4. C.R — closure decision
   Close C only if the boundary is reviewed and still no broader mutation is approved.

## Non-closure statement

C.1 does not close Blocker C.

C.1 only starts the C track and records the correct boundary.

C.1 does not approve:

- handler activation
- live route activation
- signing
- keypair use
- deploy
- write-buffer
- set-upgrade-authority
- close
- upgrade
- state initialization
- SPL mint setup
- SPL CPI minting
- guardian package construction
- transaction submit
- mutation
- production activation

## Result

Current status:

BLOCKER_C_OPEN_B1C7_HANDLER_BOUNDARY_PLANNING_ONLY_NO_CODE_CHANGE_NO_RPC_NO_EXECUTION

Current decision:

BLOCKER_C_NOT_CLOSED

NO-GO REMAINS_FOR_HANDLER_ACTIVATION_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

## Next safe step

Blocker C.2 — repo-grounded B1C7 handler inventory.

C.2 should inspect tracked repository code and tests only.

C.2 must not activate the handler, call RPC, build deployable artifacts, sign, upgrade, initialize state, configure SPL, construct guardian packages, submit, or mutate.
