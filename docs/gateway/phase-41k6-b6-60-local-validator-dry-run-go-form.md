# Phase 41K.6 B6.60 — Local-validator dry-run GO form / command boundary

Status:

LOCAL_VALIDATOR_DRY_RUN_GO_FORM_DEFINED_NOT_APPROVED

Current decision:

NO-GO FOR LOCAL_VALIDATOR_TESTNET_SIGNING_SPL_UPGRADE_INIT_SUBMIT

## Purpose

B6.60 defines the future explicit GO form and command boundary for a local-validator dry-run.

B6.60 is form-only.

B6.60 does not run a local validator.

B6.60 does not use testnet.

B6.60 does not use live RPC.

B6.60 does not enable real signing.

B6.60 does not use real private keys.

B6.60 does not construct guardian packages.

B6.60 does not configure SPL mint authority.

B6.60 does not perform SPL CPI minting.

B6.60 does not upgrade a program.

B6.60 does not initialize state.

B6.60 does not submit transactions.

## Inputs from previous checkpoints

B6.58 emitted the approved local-only mock fixture bundle.

B6.59 verified that emitted bundle.

The local fixture bundle path remains:

tmp/local-validator-fixtures/phase-41k6-b6-local-only

B6.60 only defines how a later local-validator dry-run may be approved.

## Future B6.61 scope, if separately approved

A future B6.61 local-validator dry-run may be allowed only after Sergey gives a separate explicit scoped GO.

Future B6.61 may be scoped to:

- local validator only
- local machine only
- local disposable state only
- B6.58/B6.59 verified fixture bundle only
- no testnet RPC
- no live RPC
- no real keys
- no real guardian packages
- no real SPL mint authority setup
- no real upgrade authority use
- no real submit
- no persistent credentials
- no production/testnet descriptor file creation

If future local-validator execution requires key material, it must use mock deterministic local-only runtime-generated key material.

Such material must not be real.

Such material must not be committed.

Such material must not be copied from production or testnet.

Such material must not be stored in B6.58 fixture files.

## Future explicit approval phrase

The future B6.61 dry-run may proceed only if Sergey explicitly approves a phrase equivalent to:

I approve B6.61 local-validator dry-run only, scoped to local disposable validator state and the verified B6.58/B6.59 mock fixture bundle, with no testnet RPC, no live RPC, no real signing keys, no real guardian packages, no SPL mint authority setup, no SPL CPI minting against real assets, no program upgrade, no persistent state initialization outside the local validator, and no submit to any network.

Theo approval, prior planning docs, or this B6.60 form do not replace Sergey explicit scoped GO.

## Forbidden until separate GO

Still forbidden after B6.60:

- local validator execution
- testnet action
- live RPC
- real signing
- real private keys
- seed phrases
- credentials
- keypair paths
- guardian package construction
- SPL mint authority setup
- SPL CPI minting
- program upgrade
- state initialization
- submit

## Required preconditions before any future B6.61

Before any B6.61 local-validator dry-run command is provided, the next checkpoint must verify:

- current branch and main are clean except disposable tmp output
- B6.58 emitter exists on main
- B6.59 safety checkpoint exists on main
- fixture bundle still contains exactly 10 approved files
- JSON files still parse
- forbidden material scan still passes
- command uses local validator only
- command cannot reach testnet or live RPC
- command cannot use real keypair paths
- command cannot upgrade or submit to any network

## Blocker relationship

B6.60 does not close blocker H.

B6.60 prepares the GO form for a possible future blocker H local-validator dry-run.

Blocker H remains open and separately gated.

Blockers A through G remain open and are not affected by B6.60.

## Result

B6.60 defines the local-validator dry-run approval boundary.

No execution occurred.

Current status:

LOCAL_VALIDATOR_DRY_RUN_GO_FORM_DEFINED_NOT_APPROVED

Current decision remains:

NO-GO FOR LOCAL_VALIDATOR_TESTNET_SIGNING_SPL_UPGRADE_INIT_SUBMIT

## Next safe step

The next safe step is a Sergey decision on whether to request B6.61 local-validator dry-run planning.

Without a new explicit scoped GO, no validator command should be executed.
