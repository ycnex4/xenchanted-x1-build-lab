# Blocker C.3 — B1C7 production/testnet activation decision model

Status:

BLOCKER_C_OPEN_B1C7_ACTIVATION_DECISION_MODEL_RECORDED_NO_CODE_CHANGE_NO_RPC_NO_EXECUTION

Current decision:

B1C7_DIRECT_DANGEROUS_TEST_GATE_ACTIVATION_REJECTED

Selected future model:

REVIEWED_TESTNET_INTENDED_HANDLER_ROUTE_REQUIRED_BEFORE_ANY_DEPLOYABLE_ARTIFACT

NO-GO REMAINS_FOR_HANDLER_ACTIVATION_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

## Purpose

Blocker C.3 records the activation decision model for the B1C7 handler path.

C.3 is decision-model only.

It does not activate the handler.

It does not change runtime code.

It does not build a deployable artifact.

It does not call RPC.

It does not use testnet.

It does not sign.

It does not deploy, upgrade, initialize state, configure SPL, construct guardian packages, submit, or mutate any network.

## Background

C.1 opened the B1C7 production/testnet boundary track.

C.2 inventoried the repo-grounded B1C7 handler path.

C.2 confirmed:

- the handler exists
- the handler is integration/test-gated
- the dangerous SBF allow feature is explicit
- the default path fails closed
- live route activation remains false
- SPL CPI execution remains false by default
- deployment_status remains deployable=false
- Program ID placeholder boundary remains active
- the corrected function-scoped order check shows mark before guarded CPI call inside atomic_mark_and_mint_boundary

## Activation decision

C.3 rejects direct activation of the existing dangerous test-gate feature set as a deployable or testnet-intended route.

Rejected model:

C3_MODEL_1_DIRECT_DANGEROUS_TEST_GATE_DEPLOY

Reason:

The current feature names and compile-error text intentionally describe the path as non-production integration/test-gated and dangerous for deploy artifacts.

Therefore, Blocker C must not be closed by saying that the existing dangerous test gates are acceptable for testnet deployment.

## Selected future model

Selected future model:

REVIEWED_TESTNET_INTENDED_HANDLER_ROUTE_REQUIRED_BEFORE_ANY_DEPLOYABLE_ARTIFACT

Meaning:

A future reviewed step must introduce or designate an explicit testnet-intended B1C7 handler route/boundary before any deployable artifact can be built or accepted.

That future route must not rely on the dangerous test-gate wording as the authority for deployment.

It must be separately reviewed, bounded, and tied to a final scoped GO package.

## Required properties of any future testnet-intended B1C7 route

Before any future testnet-intended B1C7 route can be accepted, it must record:

- exact feature names used for the intended testnet route
- exact compile guards preventing accidental production/deploy misuse
- exact handler entrypoint
- exact account contract
- exact authorization-before-mutation invariant
- exact fail-fast-before-mutation invariant
- exact replay check-before-mark invariant
- exact processed_event mark + SPL mint atomicity model
- exact SPL CPI gate relationship
- exact dependency on D/E/F/B/G blockers
- exact local-validator or equivalent evidence before live testnet mutation
- exact deployable artifact hash before any upgrade
- exact final scoped GO before any submit/mutation

## Relationship to SPL CPI

C.3 does not approve SPL CPI execution.

SPL CPI remains separately blocked by D/E and by the explicit CPI gate.

A future C closure may validate the handler boundary, but it must not by itself approve SPL mint authority architecture, SPL mint setup, or live CPI minting.

## Relationship to remaining blockers

C.3 does not close C and does not affect the open status of:

- B — expected post-upgrade ProgramData hash
- D — state initialization design
- E — SPL mint authority architecture
- F — guardian descriptor
- G — rollback / recovery plan

## Decision matrix

```text
# B1C7 activation decision matrix

C3_MODEL_0_CURRENT_DEFAULT_REMAIN_TEST_GATED
status: accepted_as_current_default_only
meaning: B1C7 remains non-production integration/test-gated.
allows_progress_to_next_review_step: true
allows_testnet_activation: false
allows_deployable_artifact: false
allows_mutation: false

C3_MODEL_1_DIRECT_DANGEROUS_TEST_GATE_DEPLOY
status: rejected
meaning: Build/deploy using phase-41k6-b1c7-handler-integration-test-gate plus dangerously-allow-phase-41k6-b1c7-handler-integration-test-gate-sbf-build as if it were the testnet route.
reason_rejected: the gate names and compile_error text explicitly mark this as non-production integration/test-gated and dangerous for deploy artifacts.
allows_testnet_activation: false
allows_deployable_artifact: false
allows_mutation: false

C3_MODEL_2_REVIEWED_TESTNET_INTENDED_HANDLER_ROUTE
status: selected_future_model
meaning: A future reviewed step must introduce or designate an explicit testnet-intended handler route/boundary, with non-dangerous naming, reviewed invariants, artifact hash, local evidence, dependency blockers, and final GO.
allows_testnet_activation_now: false
allows_future_testnet_activation_after_required_evidence: true
allows_mutation_now: false

C3_MODEL_3_PRODUCTION_IMMUTABLE_ACTIVATION
status: out_of_scope
meaning: Production/immutable activation is not part of Blocker C.3 and cannot be inferred from testnet boundary planning.
allows_testnet_activation: false
allows_production_activation: false
allows_mutation: false

SELECTED_C3_DECISION
B1C7_DIRECT_DANGEROUS_TEST_GATE_ACTIVATION_REJECTED

SELECTED_FUTURE_MODEL
REVIEWED_TESTNET_INTENDED_HANDLER_ROUTE_REQUIRED_BEFORE_ANY_DEPLOYABLE_ARTIFACT
```

## Non-closure statement

C.3 does not close Blocker C.

C.3 does not approve:

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

BLOCKER_C_OPEN_B1C7_ACTIVATION_DECISION_MODEL_RECORDED_NO_CODE_CHANGE_NO_RPC_NO_EXECUTION

Current decision:

B1C7_DIRECT_DANGEROUS_TEST_GATE_ACTIVATION_REJECTED

Selected future model:

REVIEWED_TESTNET_INTENDED_HANDLER_ROUTE_REQUIRED_BEFORE_ANY_DEPLOYABLE_ARTIFACT

NO-GO REMAINS_FOR_HANDLER_ACTIVATION_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

## Next safe step

Blocker C.4 — B1C7 invariant review package.

C.4 should review the B1C7 invariants and closure criteria without activating the handler, calling RPC, building deployable artifacts, signing, upgrading, initializing state, configuring SPL, constructing guardian packages, submitting, or mutating.

## Evidence preview

metadata:

```text
phase=blocker-c-3-b1c7-production-testnet-activation-decision-model
timestamp_utc=2026-07-06T17:21:42Z
repo_only=true
rpc_used=false
testnet_used=false
code_changed=false
handler_activated=false
build_executed=false
deployable_artifact_created=false
mutation_executed=false
```
