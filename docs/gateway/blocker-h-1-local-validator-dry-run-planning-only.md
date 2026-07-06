# Blocker H.1 — Local-validator dry-run planning-only

Status:

BLOCKER_H_LOCAL_VALIDATOR_DRY_RUN_PLANNING_ONLY_NO_EXECUTION

Current decision:

NO-GO FOR BLOCKER_H_LOCAL_VALIDATOR_EXECUTION_TESTNET_SIGNING_SPL_UPGRADE_INIT_SUBMIT

## Purpose

Blocker H.1 opens the Blocker H lane as a planning-only checkpoint.

H.1 does not execute the local-validator dry-run.

H.1 does not provide an actual runnable validator command.

H.1 does not use testnet.

H.1 does not use live RPC.

H.1 does not enable signing.

H.1 does not use real keys.

H.1 does not construct guardian packages.

H.1 does not configure SPL mint authority.

H.1 does not perform SPL CPI minting.

H.1 does not upgrade, initialize state, or submit.

## Prior lane readiness

The B6 local-validator preparation lane is complete through B6.64R.

Relevant checkpoints:

- B6.58 fixture emission, mock only
- B6.59 emitted fixture bundle safety checkpoint
- B6.60 local-validator GO form
- B6.61 planning-only boundary
- B6.62 Theo review package
- B6.62R Theo verdict record
- B6.63 command-boundary no-execution
- B6.64 command-boundary safety checkpoint / Theo review package
- B6.64R Theo verdict and exit-code comment

Theo approved B6.63/B6.64 and confirmed that the command-boundary script is fail-closed.

Theo approved proceeding to Blocker H local-validator dry-run planning step.

Theo did not approve actual local-validator execution.

## Command-boundary artifact

Existing command-boundary script:

scripts/gateway/b6_63_local_validator_command_boundary_no_execution.sh

The script remains a no-execution boundary artifact.

Default behavior:

- verifies the local fixture bundle
- prints BLOCKER_H_NOT_CLOSED
- exits without local-validator execution

With --execute or EXECUTE=true:

- still refuses execution
- prints BLOCKER_H_NOT_CLOSED
- exits with code 63

Exit code 63 is documented as:

Exit 63 = BLOCKER_H_NOT_CLOSED

## Fixture bundle

Local fixture bundle path:

tmp/local-validator-fixtures/phase-41k6-b6-local-only

Expected file count:

10

Expected files:

- README.local-only.txt
- accounts.json
- expected-snapshots.json
- failure-matrix.json
- instructions.json
- logs.json
- manifest.json
- mutation-invariance.json
- safety-report.json
- scenarios.json

The fixture bundle is local-only, mock/deterministic, disposable, and uncommitted.

## H.1 planning boundary

H.1 defines what must be planned before any actual local-validator dry-run can be considered.

The actual dry-run must be separated into a later checkpoint.

The actual dry-run must require a separate explicit scoped GO from Sergey.

The actual dry-run must not be inferred from Theo approval of planning checkpoints.

The actual dry-run must not be inferred from the existence of a command-boundary script.

## Required preflight checks before any future actual dry-run

A later execution checkpoint must verify:

- current branch is the explicitly approved execution branch
- main includes B6.64R or later
- working tree is clean except approved disposable tmp output
- fixture directory exists locally
- fixture directory contains exactly 10 approved files
- every generated JSON file parses
- forbidden-material taxonomy scan passes
- no real private keys are present
- no seed phrases are present
- no credentials or tokens are present
- no real RPC endpoints are present
- no keypair paths are present
- no real program ID markers are used as executable targets
- no upgrade authority material is present
- no testnet descriptor is created
- no testnet fallback exists
- no deploy, upgrade, init, SPL setup, or submit path exists

## Required execution isolation for a future dry-run

A later actual local-validator dry-run, if explicitly approved, must be isolated to:

- local machine only
- local validator only
- local disposable ledger directory only
- local mock accounts only
- local mock fixture data only
- runtime-generated mock key material only if unavoidable
- no committed generated key material
- no production or testnet keys
- no live network access
- no testnet fallback

## Future explicit GO phrase

A future actual local-validator dry-run may proceed only if Sergey explicitly approves a phrase equivalent to:

I approve Blocker H actual local-validator dry-run only, scoped to local disposable validator state and the verified mock fixture bundle, with no testnet RPC, no live RPC, no real signing keys, no real guardian packages, no SPL mint authority setup against real assets, no SPL CPI minting against real assets, no program upgrade, no persistent state initialization outside the local validator, and no submit to any network.

Without this explicit scoped GO, Blocker H remains open and gated.

## Blocker H status

Blocker H remains OPEN and GATED after H.1.

H.1 does not close Blocker H.

H.1 does not execute Blocker H.

H.1 only defines the planning boundary before any possible actual local-validator dry-run.

Blockers A through G remain open and are not affected by H.1.

## Result

Blocker H.1 records the planning-only boundary for the local-validator dry-run lane.

No validator was run.

No execution occurred.

Current status:

BLOCKER_H_LOCAL_VALIDATOR_DRY_RUN_PLANNING_ONLY_NO_EXECUTION

Current decision remains:

NO-GO FOR BLOCKER_H_LOCAL_VALIDATOR_EXECUTION_TESTNET_SIGNING_SPL_UPGRADE_INIT_SUBMIT

## Next safe step

The next safe step is Blocker H.2 preflight checklist definition with no execution.

Actual local-validator execution remains separately gated.
