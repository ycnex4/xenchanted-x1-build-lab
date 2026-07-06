# Blocker F.4 — Guardian descriptor invariant review package

Status:

BLOCKER_F_REVIEW_READY_GUARDIAN_DESCRIPTOR_INVARIANTS_RECORDED_NO_KEYS_NO_PACKAGES_NO_EXECUTION

Current decision:

BLOCKER_F_NOT_CLOSED_PENDING_CLOSURE_DECISION

Closure candidate prepared:

GUARDIAN_DESCRIPTOR_MODEL_REVIEWED_KEYS_AND_PACKAGES_NOT_APPROVED

NO-GO REMAINS_FOR_GUARDIAN_DESCRIPTOR_FINALIZATION_GUARDIAN_KEYS_PRODUCTION_KEYS_SIGNING_PACKAGES_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Purpose

Blocker F.4 records the guardian descriptor invariant review package.

F.4 is review-only.

It does not add guardian keys.

It does not add private keys.

It does not finalize a live guardian descriptor.

It does not construct a guardian package.

It does not sign.

It does not initialize GuardianSet state.

It does not configure SPL.

It does not build a deployable artifact.

It does not call RPC.

It does not use testnet.

It does not deploy, upgrade, submit, or mutate any network.

## Evidence basis

F.4 is based on:

- F.1 guardian descriptor planning
- F.2 repo-grounded guardian/quorum inventory
- F.3 guardian descriptor decision model

## Reviewed invariants

- static public descriptor model
- public keys only
- private keys never committed, requested, printed, or stored in repo
- ed25519 public key descriptor model
- explicit threshold/quorum rule
- distinct guardian approvals
- deterministic descriptor hash/id
- guardian_set_id / route / source chain / mint token / message schema / canonical encoding binding
- rotation requires new descriptor id and separate review
- guardian package construction and signing remain blocked
- no RPC, testnet, submit, or mutation approved

## Private key material scan summary

private_key_scan_match_count: 0

private_key_material_scan_result: NO_PRIVATE_KEY_MATERIAL_PATTERNS_FOUND

## Invariant review matrix

```text
# Guardian descriptor invariant review matrix

F4_INVARIANT_01_STATIC_PUBLIC_DESCRIPTOR_MODEL
status: reviewed
result: true
meaning: Future guardian descriptor must be a public tracked repo artifact.

F4_INVARIANT_02_PUBLIC_KEYS_ONLY
status: reviewed
result: true
meaning: Descriptor may contain public keys only.

F4_INVARIANT_03_PRIVATE_KEYS_NEVER_COMMITTED_REQUESTED_PRINTED
status: reviewed
result: true
meaning: Private keys must never be committed, requested, printed, or stored in repo.

F4_INVARIANT_04_ED25519_PUBLIC_KEY_MODEL
status: reviewed
result: true
meaning: Descriptor model is ed25519 public key based.

F4_INVARIANT_05_EXPLICIT_THRESHOLD
status: reviewed
result: true
meaning: Threshold/quorum must be explicit.

F4_INVARIANT_06_DISTINCT_GUARDIAN_APPROVALS
status: reviewed
result: true
meaning: Quorum must be over distinct guardian approvals.

F4_INVARIANT_07_DETERMINISTIC_DESCRIPTOR_HASH_ID
status: reviewed
result: true
meaning: Descriptor id/hash must be deterministic.

F4_INVARIANT_08_ROUTE_STATE_MESSAGE_BINDING
status: reviewed
result: true
meaning: Descriptor must bind guardian_set_id, route_id, source chain, mint token, message schema, and canonical encoding.

F4_INVARIANT_09_ROTATION_REQUIRES_NEW_DESCRIPTOR_ID
status: reviewed
result: true
meaning: Rotation requires a new descriptor id and separate review.

F4_INVARIANT_10_NO_PACKAGE_CONSTRUCTION_OR_SIGNING_APPROVED
status: reviewed
result: true
meaning: Guardian package construction and signing remain blocked until future reviewed descriptor and final scoped GO.

F4_INVARIANT_11_NO_RPC_TESTNET_MUTATION
status: reviewed
result: true
meaning: F.4 does not approve RPC, testnet, submit, or mutation.

F4_AGGREGATE
all_invariants_reviewed: true
blocker_f_closure_ready: true
closure_type: narrow_descriptor_boundary_only
```

## Review result

all_invariants_reviewed: true

blocker_f_closure_ready: true

closure_type: narrow_descriptor_boundary_only

## Closure candidate prepared

F.4 prepares, but does not itself record, a narrow closure candidate for Blocker F:

GUARDIAN_DESCRIPTOR_MODEL_REVIEWED_KEYS_AND_PACKAGES_NOT_APPROVED

Meaning:

- guardian descriptor model has been reviewed
- future descriptor must be public and repo-tracked
- future descriptor may contain public keys only
- private keys remain forbidden
- explicit threshold/quorum is required
- distinct guardian approvals are required
- deterministic descriptor hash/id is required
- route/state/message binding is required
- no guardian package construction is approved
- no signing is approved

## Remaining open items outside F closure

- exact descriptor schema file path
- exact canonical descriptor hash rule
- exact testnet guardian public key list
- exact threshold value
- exact guardian_set_id value
- exact route/state/message binding values
- exact invalid/duplicate/unknown/under-threshold failure matrix
- future reviewed descriptor artifact
- final scoped GO before package construction or signing

## Non-closure statement

F.4 does not close Blocker F.

F.4 does not approve:

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

BLOCKER_F_REVIEW_READY_GUARDIAN_DESCRIPTOR_INVARIANTS_RECORDED_NO_KEYS_NO_PACKAGES_NO_EXECUTION

Current decision:

BLOCKER_F_NOT_CLOSED_PENDING_CLOSURE_DECISION

Closure candidate prepared:

GUARDIAN_DESCRIPTOR_MODEL_REVIEWED_KEYS_AND_PACKAGES_NOT_APPROVED

NO-GO REMAINS_FOR_GUARDIAN_DESCRIPTOR_FINALIZATION_GUARDIAN_KEYS_PRODUCTION_KEYS_SIGNING_PACKAGES_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Next safe step

Blocker F.5 — guardian descriptor closure decision record.

F.5 may close Blocker F narrowly as descriptor/invariant closure only.

F.5 must not add keys, finalize a live descriptor, construct packages, sign, call RPC, use testnet, deploy, upgrade, submit, or mutate.

## Evidence preview

metadata:

```text
phase=blocker-f-4-guardian-descriptor-invariant-review-package
timestamp_utc=2026-07-06T19:51:26Z
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
