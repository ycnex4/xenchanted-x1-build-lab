# Blocker F.3 — Guardian descriptor decision model

Status:

BLOCKER_F_OPEN_GUARDIAN_DESCRIPTOR_DECISION_MODEL_RECORDED_NO_KEYS_NO_PACKAGES_NO_EXECUTION

Current decision:

STATIC_PUBLIC_GUARDIAN_DESCRIPTOR_PUBLIC_KEYS_ONLY_EXPLICIT_THRESHOLD_NO_PRIVATE_KEYS_NO_PACKAGES_NO_SIGNING

Descriptor model:

STATIC_PUBLIC_GUARDIAN_DESCRIPTOR_IN_REPO_PUBLIC_KEYS_ONLY_NO_PRIVATE_KEYS

Descriptor scope:

TESTNET_DESCRIPTOR_ALLOWED_ONLY_AS_EXPLICIT_TESTNET_ARTIFACT_AFTER_FURTHER_REVIEW

Key material boundary:

PUBLIC_KEYS_ONLY_PRIVATE_KEYS_NEVER_COMMITTED_NEVER_REQUESTED_NEVER_PRINTED

Key type model:

ED25519_PUBLIC_KEY_DESCRIPTOR_MODEL

Quorum model:

EXPLICIT_THRESHOLD_OVER_DISTINCT_GUARDIAN_APPROVALS

Descriptor id model:

DETERMINISTIC_DESCRIPTOR_HASH_ID_REQUIRED

Binding model:

DESCRIPTOR_BOUND_TO_GUARDIAN_SET_ID_ROUTE_ID_SOURCE_CHAIN_MINT_TOKEN_MESSAGE_SCHEMA_AND_CANONICAL_ENCODING

Rotation model:

ROTATION_REQUIRES_NEW_DESCRIPTOR_ID_AND_SEPARATE_REVIEW

Package boundary:

GUARDIAN_PACKAGE_CONSTRUCTION_REQUIRES_FUTURE_REVIEWED_DESCRIPTOR_AND_FINAL_SCOPED_GO

Execution boundary:

FUTURE_REVIEWED_GUARDIAN_DESCRIPTOR_AND_FINAL_SCOPED_GO_REQUIRED_BEFORE_ANY_GUARDIAN_PACKAGE_CONSTRUCTION_OR_SIGNING

NO-GO REMAINS_FOR_GUARDIAN_DESCRIPTOR_FINALIZATION_GUARDIAN_KEYS_PRODUCTION_KEYS_SIGNING_PACKAGES_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Purpose

Blocker F.3 records the guardian descriptor decision model.

F.3 is decision-model only.

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

## Background

F.1 opened guardian descriptor planning.

F.2 completed repo-grounded guardian/quorum inventory.

F.3 selects the descriptor model and key-material boundary without selecting live keys or constructing packages.

## Selected model

STATIC_PUBLIC_GUARDIAN_DESCRIPTOR_PUBLIC_KEYS_ONLY_EXPLICIT_THRESHOLD_NO_PRIVATE_KEYS_NO_PACKAGES_NO_SIGNING

Meaning:

- the descriptor must be public and tracked in repo
- the descriptor must contain public keys only
- private keys must never be committed, requested, printed, or stored in repo
- threshold/quorum must be explicit
- approvals must be distinct guardian approvals
- descriptor id/hash must be deterministic
- descriptor must be bound to route/state/message expectations
- package construction and signing require future review and final scoped GO

## Required future descriptor fields

- descriptor_version
- descriptor_scope
- network_label
- guardian_set_id
- descriptor_id_or_hash
- key_type
- guardian_public_keys
- threshold
- distinct_approval_rule
- route_id
- source_chain_id
- mint_token
- message_schema_version
- canonical_encoding_id
- activation_boundary
- rotation_boundary
- expiry_or_supersession_rule

## Rejected models

- implicit or undocumented guardian set
- ad hoc relayer/admin signature model
- hidden off-repo guardian set
- production descriptor finalization in current phase
- private keys in repo
- guardian package construction before final scoped GO
- signing before final scoped GO

## Decision matrix

```text
# Guardian descriptor decision matrix

F3_MODEL_0_NO_DESCRIPTOR
status: rejected
meaning: Guardian set is implicit or undocumented.
reason_rejected: incompatible with deterministic authorization review.

F3_MODEL_1_AD_HOC_RELAYER_OR_ADMIN_SIGNATURE
status: rejected
meaning: A relayer/admin signs without public descriptor, key list, and quorum rule.
reason_rejected: incompatible with no hidden authorization and replay-safe gateway design.

F3_MODEL_2_HIDDEN_OFF_REPO_GUARDIAN_SET
status: rejected
meaning: Guardian set is operational/off-repo only.
reason_rejected: incompatible with public review and reproducible authorization boundary.

F3_MODEL_3_STATIC_PUBLIC_GUARDIAN_DESCRIPTOR_IN_REPO
status: selected
meaning: Guardian descriptor is a tracked public repo artifact.
required_boundary: public keys only, no private keys, deterministic descriptor hash/id, explicit testnet/production label.

F3_MODEL_4_TESTNET_ONLY_DESCRIPTOR_WITH_EXPLICIT_LABEL
status: selected_for_future_testnet_scope
meaning: A testnet descriptor may be created later only with explicit testnet label and final scoped GO before use.
required_boundary: must not be confused with production descriptor.

F3_MODEL_5_PRODUCTION_DESCRIPTOR_NOW
status: rejected_for_current_phase
meaning: Production guardian descriptor is finalized now.
reason_rejected: production activation is out of scope.

F3_MODEL_6_DESCRIPTOR_HASH_BOUND_TO_ROUTE_STATE_AND_MESSAGE
status: selected_required_property
meaning: Descriptor id/hash must bind descriptor content and must be referenced by route/state/message verification expectations.

SELECTED_F3_DESCRIPTOR_MODEL
STATIC_PUBLIC_GUARDIAN_DESCRIPTOR_IN_REPO_PUBLIC_KEYS_ONLY_NO_PRIVATE_KEYS

SELECTED_F3_DESCRIPTOR_SCOPE
TESTNET_DESCRIPTOR_ALLOWED_ONLY_AS_EXPLICIT_TESTNET_ARTIFACT_AFTER_FURTHER_REVIEW

SELECTED_F3_KEY_MATERIAL_BOUNDARY
PUBLIC_KEYS_ONLY_PRIVATE_KEYS_NEVER_COMMITTED_NEVER_REQUESTED_NEVER_PRINTED

SELECTED_F3_KEY_TYPE_MODEL
ED25519_PUBLIC_KEY_DESCRIPTOR_MODEL

SELECTED_F3_QUORUM_MODEL
EXPLICIT_THRESHOLD_OVER_DISTINCT_GUARDIAN_APPROVALS

SELECTED_F3_DESCRIPTOR_ID_MODEL
DETERMINISTIC_DESCRIPTOR_HASH_ID_REQUIRED

SELECTED_F3_BINDING_MODEL
DESCRIPTOR_BOUND_TO_GUARDIAN_SET_ID_ROUTE_ID_SOURCE_CHAIN_MINT_TOKEN_MESSAGE_SCHEMA_AND_CANONICAL_ENCODING

SELECTED_F3_ROTATION_MODEL
ROTATION_REQUIRES_NEW_DESCRIPTOR_ID_AND_SEPARATE_REVIEW

SELECTED_F3_PACKAGE_BOUNDARY
GUARDIAN_PACKAGE_CONSTRUCTION_REQUIRES_FUTURE_REVIEWED_DESCRIPTOR_AND_FINAL_SCOPED_GO

SELECTED_F3_DECISION
STATIC_PUBLIC_GUARDIAN_DESCRIPTOR_PUBLIC_KEYS_ONLY_EXPLICIT_THRESHOLD_NO_PRIVATE_KEYS_NO_PACKAGES_NO_SIGNING

SELECTED_F3_EXECUTION_BOUNDARY
FUTURE_REVIEWED_GUARDIAN_DESCRIPTOR_AND_FINAL_SCOPED_GO_REQUIRED_BEFORE_ANY_GUARDIAN_PACKAGE_CONSTRUCTION_OR_SIGNING
```

## Remaining open items before F closure

- exact descriptor schema file path
- exact descriptor hash canonicalization rule
- exact testnet guardian public key list
- exact threshold value
- exact guardian_set_id value
- exact route/state/message binding fields
- exact failure matrix for invalid/duplicate/unknown/under-threshold approvals
- no-private-key repo scan evidence
- guardian descriptor invariant review package
- final scoped GO before package construction or signing

## Non-closure statement

F.3 does not close Blocker F.

F.3 does not approve:

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

BLOCKER_F_OPEN_GUARDIAN_DESCRIPTOR_DECISION_MODEL_RECORDED_NO_KEYS_NO_PACKAGES_NO_EXECUTION

Current decision:

STATIC_PUBLIC_GUARDIAN_DESCRIPTOR_PUBLIC_KEYS_ONLY_EXPLICIT_THRESHOLD_NO_PRIVATE_KEYS_NO_PACKAGES_NO_SIGNING

Execution boundary:

FUTURE_REVIEWED_GUARDIAN_DESCRIPTOR_AND_FINAL_SCOPED_GO_REQUIRED_BEFORE_ANY_GUARDIAN_PACKAGE_CONSTRUCTION_OR_SIGNING

NO-GO REMAINS_FOR_GUARDIAN_DESCRIPTOR_FINALIZATION_GUARDIAN_KEYS_PRODUCTION_KEYS_SIGNING_PACKAGES_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Next safe step

Blocker F.4 — guardian descriptor invariant review package.

F.4 should review public-keys-only, no-private-keys, explicit threshold, descriptor hash/id, route/state/message binding, rotation boundary, and no-package/no-signing boundary.

F.4 must not add keys, finalize a live descriptor, construct packages, sign, call RPC, use testnet, deploy, upgrade, submit, or mutate.

## Evidence preview

metadata:

```text
phase=blocker-f-3-guardian-descriptor-decision-model
timestamp_utc=2026-07-06T19:48:21Z
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
