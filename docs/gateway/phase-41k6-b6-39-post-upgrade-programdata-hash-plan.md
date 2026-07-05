# Phase 41K.6 B6.39 — Expected post-upgrade ProgramData hash plan

Status:

POST_UPGRADE_PROGRAMDATA_HASH_PLAN_DEFINED_HASH_NOT_RECORDED

Current decision:

NO-GO

## Purpose

This document maps blocker B:

expected post-upgrade ProgramData hash.

It defines what must be recorded before any future program upgrade can be considered.

This is docs-only.

It does not compute a deployable artifact hash.

It does not build, deploy, upgrade, sign, submit, spend SOL, initialize testnet accounts, configure SPL mint authority, perform SPL CPI minting, construct guardian packages, run a local validator, or rehearse live submit flow.

## Current blocker B status

Blocker B:

expected post-upgrade ProgramData hash

Current status:

OPEN_DESIGN_STARTED

Closure state:

NOT CLOSED

Current decision remains:

NO-GO.

## Known public baseline

Current known public testnet baseline:

- x1_testnet_program_id: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
- x1_testnet_programdata_account: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
- observed_upgrade_authority_public_key: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc

These are public identifiers only.

No signing material is stored in this repository.

No signing action is approved by this checkpoint.

## Hash plan principle

Before any future upgrade can be approved, the expected post-upgrade ProgramData hash must be known in advance.

The expected hash must be tied to:

- exact source commit
- exact build profile
- exact program crate
- exact generated program artifact
- exact hash algorithm
- exact expected ProgramData account
- exact expected network
- exact post-upgrade read-only verification method

This checkpoint does not record the final expected hash.

It defines the plan for recording it later.

## Required future hash record

A future blocker B closure checkpoint must record:

- source_commit
- source_branch
- build_profile
- program_crate
- artifact_path
- artifact_hash_algorithm
- artifact_hash_value
- expected_program_id
- expected_programdata_account
- expected_upgrade_authority_public_key
- expected_post_upgrade_programdata_hash
- pre_upgrade_programdata_hash
- post_upgrade_programdata_hash_verification_method
- post_upgrade_verification_abort_conditions
- evidence_log_path
- decision_status

## Hash algorithm

The future expected artifact hash should use a deterministic byte hash.

Required algorithm for the plan:

SHA-256

The final closure checkpoint must specify whether the hash is calculated over:

- local built program artifact bytes
- extracted deployed ProgramData bytes
- normalized loader ProgramData bytes
- another explicitly documented byte source

The byte source must be unambiguous.

If the byte source is ambiguous, blocker B remains open.

## Required source boundary

The future expected hash must be derived from a clean source state.

Required source evidence:

- working tree clean
- branch recorded
- commit recorded
- build command recorded
- build profile recorded
- artifact path recorded
- artifact hash recorded

If the working tree is dirty, blocker B cannot close.

## Required build boundary

A future build used for blocker B must be scoped separately.

This checkpoint does not approve a build.

A future build plan must define:

- build command
- build environment
- build output path
- expected artifact name
- deterministic build assumptions
- cleanup policy
- failure handling
- whether the artifact is only local or upgrade-intended

If the build output is not reproducible or not clearly tied to the commit, blocker B cannot close.

## Required pre-upgrade read-only hash boundary

Before any future upgrade, a read-only baseline must record:

- current program id
- current ProgramData account
- current upgrade authority
- current ProgramData hash if readable
- current executable status
- current deployment slot or equivalent metadata if available

This read-only baseline must not require signing.

It must not mutate any chain state.

## Required post-upgrade read-only hash boundary

After any future explicitly approved upgrade, read-only verification must record:

- program id unchanged
- ProgramData account unchanged
- upgrade authority unchanged unless explicitly scoped
- post-upgrade ProgramData hash equals expected hash
- program remains executable
- no state initialization occurred unless separately scoped
- no SPL mint authority change occurred unless separately scoped
- no gateway submit occurred unless separately scoped
- no live route activation occurred unless separately scoped

If the post-upgrade hash differs from expected, all follow-up actions must stop.

## Required mismatch behavior

If expected and observed hashes differ:

- stop immediately
- do not initialize state
- do not set SPL mint authority
- do not submit gateway messages
- do not enable live route
- preserve logs
- record expected hash
- record actual hash
- record program id
- record ProgramData account
- record upgrade authority
- create a separate recovery decision

No blind retry is allowed.

## Relationship to blocker A

Blocker B depends on blocker A because the expected hash is only useful if the authority custody boundary is clear.

Current blocker A state:

OPEN_DESIGN_STARTED

Therefore blocker B cannot close as upgrade-ready yet.

## Relationship to blocker G

Blocker B depends on blocker G because hash mismatch recovery must already be mapped.

Current blocker G state:

OPEN_DESIGN_STARTED

Therefore blocker B cannot close as upgrade-ready yet.

## Relationship to blocker H

Blocker B may depend on blocker H if the artifact is intended to include runtime handler behavior that must first pass local-validator evidence.

Current blocker H state:

OPEN_DESIGN_STARTED

Therefore blocker B cannot close as runtime-upgrade-ready yet.

## Required closure evidence for blocker B

Before blocker B can close, the following must exist:

- clean source commit
- exact build command
- exact artifact path
- exact artifact hash
- exact expected post-upgrade ProgramData hash
- pre-upgrade read-only baseline
- post-upgrade read-only verification plan
- mismatch recovery plan
- scoped relationship to blocker A
- scoped relationship to blocker G
- scoped relationship to blocker H
- explicit statement that no upgrade is approved by hash recording alone

## Explicit non-closure

This checkpoint does not close blocker B.

It defines the expected post-upgrade ProgramData hash plan only.

Current blocker B state:

OPEN_DESIGN_STARTED

Current decision remains:

NO-GO.

## Next safe step

The next safe step is a B6 Strategy 2 closure-readiness checkpoint that summarizes blockers A through H and the remaining path to B6.40.

No build is approved by this checkpoint.

No upgrade action is approved by this checkpoint.

No testnet action is approved by this checkpoint.

Current decision remains:

NO-GO.
