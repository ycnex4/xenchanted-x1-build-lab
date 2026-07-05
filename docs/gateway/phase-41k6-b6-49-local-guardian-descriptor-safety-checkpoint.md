# Phase 41K.6 B6.49 — Local guardian descriptor safety checkpoint

Status:

LOCAL_GUARDIAN_DESCRIPTOR_SAFETY_CHECKPOINT_NO_SIGNING

Current decision:

NO-GO

## Purpose

This checkpoint records the safety boundary after B6.48 local guardian descriptor skeleton.

The descriptor exists only as a local Rust skeleton.

It models local guardian descriptor structure.

It does not create a testnet guardian descriptor.

It does not construct guardian packages.

It does not sign messages.

It does not run a local validator.

It does not build, deploy, upgrade, submit, spend SOL, initialize testnet accounts, configure SPL mint authority, or perform SPL CPI minting.

## Current implemented skeleton

B6.48 added:

programs/xxxl-svm/src/local_guardian_descriptor_skeleton.rs

The skeleton is marked:

LOCAL_ONLY_GUARDIAN_DESCRIPTOR_SKELETON_NOT_SIGNING

Current descriptor constants remain:

- LOCAL_ONLY: true
- TESTNET_ALLOWED: false
- LIVE_ROUTE_ALLOWED: false
- SIGNING_ENABLED: false
- GUARDIAN_PACKAGE_CONSTRUCTION_ENABLED: false
- PRODUCTION_KEYS_ALLOWED: false
- WRITES_TO_DISK: false
- LOCAL_VALIDATOR_EXECUTION_APPROVED: false

## Current safety state

Current state:

- local guardian descriptor skeleton exists
- deterministic local public key fixtures are modeled
- threshold behavior is modeled
- descriptor integrity label is modeled locally
- signing material markers are rejected
- production key markers are rejected
- guardian package construction is disabled
- signing is disabled
- testnet use is disabled
- live route use is disabled
- writes to disk are disabled
- local-validator execution is not approved

## What B6.49 confirms

B6.49 confirms:

1. The guardian descriptor skeleton is local-only.
2. It does not contain real guardian keys.
3. It does not enable signing.
4. It does not construct guardian packages.
5. It does not create a testnet descriptor.
6. It does not initialize guardian state.
7. It does not enable local-validator execution.
8. It does not enable testnet submit.
9. Blocker F is still not closed.
10. Blocker H is still not closed.
11. Current decision remains NO-GO.

## What B6.49 does not do

B6.49 does not:

- create descriptor json
- emit descriptor files
- write fixture files
- create guardian keys
- use real guardian public keys
- use private signing material
- construct approval packages
- sign gateway messages
- initialize guardian_set account
- run guardian verification in local validator
- run local validator
- perform testnet actions
- enable live route

## Future step required before descriptor file creation

Before any descriptor file can be created, a later checkpoint must define:

- exact descriptor output path
- exact descriptor fields
- deterministic local guardian fixture source
- descriptor integrity computation
- no-signing-material proof
- no-testnet proof
- no-production-key proof
- cleanup rule
- review rule
- abort conditions

## Future step required before guardian package construction

Before any guardian package construction can be considered, a later checkpoint must define:

- whether the package is local-only or testnet scoped
- message schema
- message hash boundary
- public key descriptor mapping
- signer material custody boundary
- approval threshold
- duplicate approval rejection
- unknown guardian rejection
- invalid signature rejection
- no-mutation failure behavior
- scoped written GO

This checkpoint does not approve that step.

## Future step required before testnet guardian descriptor

Before a testnet guardian descriptor can be created, a later checkpoint must define:

- network scope
- guardian_set_id
- threshold
- real public keys
- descriptor integrity hash
- descriptor review status
- initialization relationship
- rollback and recovery rule
- scoped written GO

This checkpoint does not approve that step.

## Blocker F status

Blocker F:

guardian set testnet descriptor

Current state:

OPEN_DESIGN_STARTED

Closure state:

NOT CLOSED

Reason:

A local-only guardian descriptor skeleton exists, but no testnet guardian descriptor is created, no testnet guardian set is initialized, no guardian package construction is enabled, and signing remains disabled.

## Blocker H status

Blocker H:

local validator dry-run

Current state:

OPEN_DESIGN_STARTED

Closure state:

NOT CLOSED

Reason:

The local guardian descriptor skeleton is not executed in a local validator. No local-validator dry-run is approved or executed.

## Checkpoint conclusion

B6.48 added a useful local-only guardian descriptor skeleton.

B6.49 confirms that it remains no-signing, no-package-construction, no-testnet, and non-executing.

This checkpoint does not approve descriptor file creation.

This checkpoint does not approve guardian package construction.

This checkpoint does not approve signing.

This checkpoint does not approve local-validator execution.

This checkpoint does not approve testnet action.

Current decision remains:

NO-GO.

## Next safe step

The next safe step is local fixture integration planning for the guardian descriptor skeleton.

No guardian descriptor is created by this checkpoint.

No guardian package is constructed by this checkpoint.

No signing is approved by this checkpoint.

No local-validator execution is approved by this checkpoint.

No testnet action is approved by this checkpoint.

Current decision remains:

NO-GO.
