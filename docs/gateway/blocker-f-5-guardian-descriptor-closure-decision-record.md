# Blocker F.5 — Guardian descriptor closure decision record

Status:

BLOCKER_F_CLOSED_NARROW_GUARDIAN_DESCRIPTOR_MODEL_REVIEWED_KEYS_AND_PACKAGES_NOT_APPROVED

Current decision:

BLOCKER_F_CLOSED_NARROW_DESCRIPTOR_INVARIANTS_ONLY

NO-GO REMAINS_FOR_GUARDIAN_DESCRIPTOR_FINALIZATION_GUARDIAN_KEYS_PRODUCTION_KEYS_SIGNING_PACKAGES_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Purpose

Blocker F.5 records the closure decision for Blocker F.

The closure is narrow.

It closes only the guardian descriptor model / invariant review blocker.

It does not approve guardian descriptor finalization.

It does not approve guardian public key selection.

It does not approve production key selection.

It does not approve private key handling.

It does not approve guardian package construction.

It does not approve signing.

It does not approve RPC, testnet, submit, or mutation.

## Closure state

Blocker F is closed as:

GUARDIAN_DESCRIPTOR_MODEL_REVIEWED_KEYS_AND_PACKAGES_NOT_APPROVED

Meaning:

- guardian descriptor model has been reviewed
- future descriptor must be public and repo-tracked
- future descriptor may contain public keys only
- private keys are forbidden
- private key material scan recorded zero matches
- ed25519 public key descriptor model is selected
- explicit threshold/quorum is required
- distinct guardian approvals are required
- deterministic descriptor hash/id is required
- guardian_set_id / route / source chain / mint token / message schema / canonical encoding binding is required
- rotation requires a new descriptor id and separate review
- guardian package construction remains blocked
- signing remains blocked
- no network mutation is approved

## Evidence chain

F.5 is based on:

1. F.1 — guardian descriptor planning
2. F.2 — repo-grounded guardian/quorum inventory
3. F.3 — guardian descriptor decision model
4. F.4 — guardian descriptor invariant review package

## Accepted F.2 inventory result

F.2 inventory accepted:

all_inventory_checks_passed: true

Accepted inventory categories:

- GuardianSet state/account inventory exists
- guardian_set account contract entry exists
- repo has guardian/quorum/approval/signature references
- descriptor inventory confirms no keys added by F.2
- descriptor inventory confirms no private keys added by F.2
- descriptor inventory confirms no package constructed by F.2
- descriptor inventory confirms no signing executed by F.2

## Accepted F.3 decision

F.3 decision accepted:

STATIC_PUBLIC_GUARDIAN_DESCRIPTOR_PUBLIC_KEYS_ONLY_EXPLICIT_THRESHOLD_NO_PRIVATE_KEYS_NO_PACKAGES_NO_SIGNING

Accepted key material boundary:

PUBLIC_KEYS_ONLY_PRIVATE_KEYS_NEVER_COMMITTED_NEVER_REQUESTED_NEVER_PRINTED

Accepted key type model:

ED25519_PUBLIC_KEY_DESCRIPTOR_MODEL

Accepted quorum model:

EXPLICIT_THRESHOLD_OVER_DISTINCT_GUARDIAN_APPROVALS

Accepted package/signing boundary:

FUTURE_REVIEWED_GUARDIAN_DESCRIPTOR_AND_FINAL_SCOPED_GO_REQUIRED_BEFORE_ANY_GUARDIAN_PACKAGE_CONSTRUCTION_OR_SIGNING

## Accepted F.4 invariant result

F.4 invariant result accepted:

all_invariants_reviewed: true

blocker_f_closure_ready: true

closure_type: narrow_descriptor_boundary_only

Private key material scan accepted:

private_key_scan_match_count: 0

private_key_material_scan_result: NO_PRIVATE_KEY_MATERIAL_PATTERNS_FOUND

Accepted invariant categories:

- static public descriptor model
- public keys only
- private keys never committed, requested, printed, or stored in repo
- ed25519 public key descriptor model
- explicit threshold/quorum rule
- distinct guardian approvals
- deterministic descriptor hash/id
- route/state/message binding
- rotation requires new descriptor id and separate review
- no package construction or signing approved
- no RPC/testnet/mutation approved

## What this closure allows

This closure allows future planning to treat Blocker F as closed for the narrow guardian descriptor model/invariant question.

It allows the project to proceed to the next separately scoped blocker.

Recommended next blockers:

- G — rollback / recovery plan
- B — expected post-upgrade ProgramData hash

## What this closure does not allow

This closure does not approve:

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

## Remaining blockers

After F.5:

- Blocker A — CLOSED narrowly: upgrade authority present but accepted for test phase
- Blocker B — OPEN: expected post-upgrade ProgramData hash
- Blocker C — CLOSED narrowly: B1C7 handler boundary / invariants only
- Blocker D — CLOSED narrowly: state initialization design / invariants only
- Blocker E — CLOSED narrowly: SPL mint authority architecture / invariants only
- Blocker F — CLOSED narrowly: guardian descriptor model / invariants only
- Blocker G — OPEN: rollback / recovery plan
- Blocker H — CLOSED narrowly: local-validator health dry-run only

## Safety invariant

Closing Blocker F must not weaken the overall NO-GO boundary.

Overall testnet mutation remains NO-GO until B and G are closed and a final scoped GO package is recorded.

A future reviewed guardian descriptor remains required before any guardian package construction or signing.

A future final scoped GO remains required before any guardian package construction or signing.

## Result

Current status:

BLOCKER_F_CLOSED_NARROW_GUARDIAN_DESCRIPTOR_MODEL_REVIEWED_KEYS_AND_PACKAGES_NOT_APPROVED

Current decision:

BLOCKER_F_CLOSED_NARROW_DESCRIPTOR_INVARIANTS_ONLY

NO-GO REMAINS_FOR_GUARDIAN_DESCRIPTOR_FINALIZATION_GUARDIAN_KEYS_PRODUCTION_KEYS_SIGNING_PACKAGES_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Next safe step

Proceed to the next separately scoped blocker.

Recommended next step:

Blocker G.1 — rollback / recovery plan planning.

Do not proceed to deploy, upgrade, state init execution, SPL setup, guardian package construction, signing, or submit.
