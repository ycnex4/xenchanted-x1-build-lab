# Blocker H.3 — Local-validator dry-run GO form, no execution

Status:

BLOCKER_H_LOCAL_VALIDATOR_DRY_RUN_GO_FORM_DEFINED_NO_EXECUTION

Current decision:

NO-GO FOR BLOCKER_H_LOCAL_VALIDATOR_EXECUTION_TESTNET_SIGNING_SPL_UPGRADE_INIT_SUBMIT

## Purpose

Blocker H.3 defines the future explicit GO form and execution boundary for a possible actual local-validator dry-run.

H.3 is GO-form and execution-plan only.

H.3 does not execute the local-validator dry-run.

H.3 does not add an actual runnable validator execution command.

H.3 does not use testnet.

H.3 does not use live RPC.

H.3 does not enable signing.

H.3 does not use real keys.

H.3 does not construct guardian packages.

H.3 does not configure SPL mint authority.

H.3 does not perform SPL CPI minting.

H.3 does not upgrade, initialize state, or submit.

## Prior readiness

Blocker H.1 opened the local-validator dry-run lane as planning-only.

Blocker H.2 defined and verified the preflight checklist with no execution.

The B6.63 command-boundary script remains fail-closed:

scripts/gateway/b6_63_local_validator_command_boundary_no_execution.sh

The verified local fixture bundle remains:

tmp/local-validator-fixtures/phase-41k6-b6-local-only

## Future actual dry-run scope

A future actual local-validator dry-run may only be scoped to:

- local machine only
- local validator only
- local disposable ledger state only
- verified mock fixture bundle only
- local mock accounts only
- runtime-generated mock key material only if unavoidable
- no committed generated key material
- no testnet RPC
- no live RPC
- no real private keys
- no seed phrases
- no credentials
- no real guardian packages
- no SPL mint authority setup against real assets
- no SPL CPI minting against real assets
- no program upgrade
- no persistent state initialization outside the local validator
- no submit to any network

## Future explicit GO phrase

A future actual local-validator dry-run may proceed only if Sergey explicitly approves a phrase equivalent to:

I approve Blocker H actual local-validator dry-run only, scoped to local disposable validator state and the verified mock fixture bundle, with no testnet RPC, no live RPC, no real signing keys, no real guardian packages, no SPL mint authority setup against real assets, no SPL CPI minting against real assets, no program upgrade, no persistent state initialization outside the local validator, and no submit to any network.

Without this explicit scoped GO, the command-boundary must remain in refusal mode.

## Required abort checks before future execution

Before any future actual local-validator dry-run command is allowed to run, it must abort unless all of the following are true:

- current branch is the explicitly approved execution branch
- main includes Blocker H.3 or later
- working tree is clean except approved disposable tmp output
- fixture directory exists locally
- fixture directory contains exactly 10 approved files
- every generated JSON file parses
- forbidden-material taxonomy scan passes
- B6.63 command-boundary script exists
- B6.63 command-boundary script syntax is valid
- B6.63 default path still prints BLOCKER_H_NOT_CLOSED
- B6.63 default path still does not execute validator
- B6.63 --execute path still refuses with exit code 63 unless replaced by a separately approved H execution script
- no testnet fallback exists
- no real RPC endpoint is present
- no keypair path is present
- no seed phrase is present
- no private-key marker is present
- no credential marker is present
- no upgrade authority material is present
- no deploy, upgrade, init, SPL setup, or submit path exists

## Still forbidden after H.3

Still forbidden after this checkpoint:

- actual local-validator execution
- testnet action
- live RPC
- real signing
- real private keys
- seed phrases
- credentials
- guardian package construction
- SPL mint authority setup
- SPL CPI minting
- program upgrade
- state initialization
- submit

## Relationship to command-boundary script

H.3 does not change the B6.63 command-boundary script.

The script remains a refusal-mode boundary artifact.

A future H execution checkpoint may introduce a new execution-specific wrapper only after separate explicit scoped GO.

That future wrapper must not silently weaken the B6.63 fail-closed boundary.

## Blocker H status

Blocker H remains OPEN and GATED after H.3.

H.3 does not close Blocker H.

H.3 does not execute Blocker H.

H.3 only defines the future GO form and execution boundary.

Blockers A through G remain open and are not affected by H.3.

## Result

Blocker H.3 records the future dry-run GO form and execution boundary.

No validator was run.

No execution occurred.

Current status:

BLOCKER_H_LOCAL_VALIDATOR_DRY_RUN_GO_FORM_DEFINED_NO_EXECUTION

Current decision remains:

NO-GO FOR BLOCKER_H_LOCAL_VALIDATOR_EXECUTION_TESTNET_SIGNING_SPL_UPGRADE_INIT_SUBMIT

## Next safe step

The next safe step is either:

- send H.1 through H.3 to Theo for review
- or create Blocker H.4 execution-readiness review package with no execution

Actual local-validator execution remains separately gated.
