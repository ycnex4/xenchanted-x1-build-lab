# Blocker F.1 — Guardian descriptor planning

Status:

BLOCKER_F_OPEN_GUARDIAN_DESCRIPTOR_PLANNING_ONLY_NO_KEYS_NO_PACKAGES_NO_RPC_NO_EXECUTION

Current decision:

BLOCKER_F_NOT_CLOSED

NO-GO REMAINS_FOR_GUARDIAN_DESCRIPTOR_FINALIZATION_GUARDIAN_KEYS_PRODUCTION_KEYS_SIGNING_PACKAGES_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Purpose

Blocker F.1 opens the guardian descriptor track.

F.1 is planning-only.

It does not add guardian keys.

It does not add private keys.

It does not finalize a guardian descriptor.

It does not construct a guardian package.

It does not sign.

It does not initialize state.

It does not configure SPL.

It does not build a deployable artifact.

It does not call RPC.

It does not use testnet.

It does not deploy, upgrade, submit, or mutate any network.

## Why F follows E

Blocker A is closed narrowly for upgrade authority model.

Blocker C is closed narrowly for B1C7 handler boundary.

Blocker D is closed narrowly for state initialization design.

Blocker E is closed narrowly for SPL mint authority architecture.

Guardian descriptor design is the next separate authorization blocker.

## Guardian descriptor goal

The future guardian descriptor should make authorization reviewable before any package construction or network mutation.

It should define:

- descriptor schema
- descriptor id/hash
- guardian set id
- guardian public keys only
- key type
- weights or equal-weight model
- quorum/threshold rule
- activation boundary
- rotation boundary
- testnet/production separation
- route binding
- message binding
- state binding
- failure modes
- package construction boundary

## Planning questions

```text
# Guardian descriptor planning questions

F1_QUESTION_01_DESCRIPTOR_SCHEMA
What exact fields must the guardian descriptor contain?

F1_QUESTION_02_DESCRIPTOR_ID
How is the descriptor id/hash derived?

F1_QUESTION_03_GUARDIAN_KEY_TYPE
What key type is canonical for guardian approvals?

F1_QUESTION_04_PUBLIC_KEYS_ONLY
How do we ensure only public keys are recorded and no private/signing key material is committed?

F1_QUESTION_05_GUARDIAN_WEIGHTS
Are guardians equal-weight or weighted?

F1_QUESTION_06_THRESHOLD
What exact quorum/threshold rule is canonical?

F1_QUESTION_07_GUARDIAN_SET_ID
How is guardian_set_id represented and bound to verification?

F1_QUESTION_08_ACTIVATION_BOUNDARY
When does a descriptor become active?

F1_QUESTION_09_ROTATION_BOUNDARY
How are guardian descriptor rotations handled?

F1_QUESTION_10_TESTNET_VS_PRODUCTION
How do we separate testnet guardian descriptors from production descriptors?

F1_QUESTION_11_ROUTE_BINDING
How is the descriptor bound to routeId, source chain, mint token, and canonical message encoding?

F1_QUESTION_12_STATE_BINDING
How is the descriptor bound to GatewayConfig and GuardianSet state?

F1_QUESTION_13_REPLAY_SAFETY
How does guardian authorization compose with processed burn replay protection?

F1_QUESTION_14_FAILURE_MODES
What must fail if a signature is invalid, unknown, duplicated, under-threshold, expired, wrong route, or wrong descriptor?

F1_QUESTION_15_PACKAGE_BOUNDARY
What evidence must exist before any guardian package is constructed or submitted?
```

## Candidate models

```text
# Guardian descriptor candidate models

F1_MODEL_0_NO_DESCRIPTOR
status: rejected_candidate
meaning: Guardian set is implicit or undocumented.
reason_rejected: incompatible with deterministic review and safe testnet activation.

F1_MODEL_1_AD_HOC_RELAYER_OR_ADMIN_SIGNATURE
status: rejected_candidate
meaning: A relayer/admin signs without a public descriptor and quorum model.
reason_rejected: incompatible with quorum, replay safety, and no hidden authorization.

F1_MODEL_2_STATIC_PUBLIC_GUARDIAN_DESCRIPTOR_IN_REPO
status: preferred_candidate
meaning: Public descriptor records schema, guardian public keys, key type, threshold, route binding, activation boundary, and descriptor hash.
reason_preferred: reviewable, deterministic, auditable, and compatible with no private keys in repo.

F1_MODEL_3_TESTNET_ONLY_DESCRIPTOR_WITH_EXPLICIT_LABEL
status: acceptable_testnet_candidate
meaning: A clearly labeled testnet descriptor can be used for scoped testnet only.
required_boundary: must not be confused with production; must require final scoped GO before use.

F1_MODEL_4_HIDDEN_OFF_REPO_GUARDIAN_SET
status: rejected_candidate
meaning: Guardian set exists only operationally and is not recorded.
reason_rejected: incompatible with public review and deterministic authorization boundary.

F1_MODEL_5_DESCRIPTOR_HASH_BOUND_TO_STATE_AND_MESSAGES
status: preferred_future_property
meaning: Descriptor hash/id is bound to GatewayConfig/GuardianSet and message authorization expectations.
reason_preferred: reduces key-confusion and descriptor-confusion risk.
```

## Initial direction

F.1 does not select a final descriptor.

However, the preferred direction for later F steps is:

- static public guardian descriptor in repo
- public keys only
- no private keys
- deterministic descriptor hash/id
- explicit testnet vs production labeling
- explicit quorum rule
- descriptor binding to route/state/message expectations
- final scoped GO before any guardian package construction or submit

## Non-closure statement

F.1 does not close Blocker F.

F.1 does not approve:

- guardian descriptor finalization
- guardian public key selection
- production key selection
- private key handling
- signing
- guardian package construction
- state initialization execution
- SPL setup
- deploy
- write-buffer
- set-upgrade-authority
- close
- upgrade
- transaction submit
- mutation
- production activation

## Result

Current status:

BLOCKER_F_OPEN_GUARDIAN_DESCRIPTOR_PLANNING_ONLY_NO_KEYS_NO_PACKAGES_NO_RPC_NO_EXECUTION

Current decision:

BLOCKER_F_NOT_CLOSED

NO-GO REMAINS_FOR_GUARDIAN_DESCRIPTOR_FINALIZATION_GUARDIAN_KEYS_PRODUCTION_KEYS_SIGNING_PACKAGES_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Next safe step

Blocker F.2 — repo-grounded guardian/quorum inventory.

F.2 should inspect tracked repository code and docs only.

F.2 must not add keys, finalize descriptor, construct packages, sign, call RPC, use testnet, deploy, upgrade, submit, or mutate.

## Evidence preview

metadata:

```text
phase=blocker-f-1-guardian-descriptor-planning
timestamp_utc=2026-07-06T19:30:54Z
repo_only=true
rpc_used=false
testnet_used=false
keys_added=false
private_keys_added=false
guardian_descriptor_finalized=false
guardian_package_constructed=false
signing_executed=false
state_initialized=false
spl_setup_executed=false
deployable_artifact_created=false
mutation_executed=false
```
