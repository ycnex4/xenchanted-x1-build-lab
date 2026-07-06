# Phase 41K.6 B6.61 — Local-validator dry-run planning-only

Status:

LOCAL_VALIDATOR_DRY_RUN_PLANNING_ONLY_DEFINED_NO_EXECUTION

Current decision:

NO-GO FOR LOCAL_VALIDATOR_EXECUTION_TESTNET_SIGNING_SPL_UPGRADE_INIT_SUBMIT

## Purpose

B6.61 converts the B6.60 GO form into a planning-only boundary for a future local-validator dry-run.

B6.61 is not an execution checkpoint.

B6.61 does not provide a runnable validator command.

B6.61 does not run a local validator.

B6.61 does not use testnet.

B6.61 does not use live RPC.

B6.61 does not enable signing.

B6.61 does not use real private keys.

B6.61 does not construct guardian packages.

B6.61 does not configure SPL mint authority.

B6.61 does not perform SPL CPI minting.

B6.61 does not upgrade a program.

B6.61 does not initialize state.

B6.61 does not submit transactions.

## Inputs

B6.58 emitted the approved local-only mock fixture bundle.

B6.59 verified the emitted fixture bundle.

B6.60 defined the explicit GO form and command boundary for a future local-validator dry-run.

The verified local fixture bundle path remains:

tmp/local-validator-fixtures/phase-41k6-b6-local-only

## Planning boundary

A future local-validator dry-run may only be considered if it remains fully local and disposable.

The future dry-run must not cross into testnet or any live network.

The future dry-run must not use real key material.

The future dry-run must not use real guardian packages.

The future dry-run must not use real upgrade authority material.

The future dry-run must not create persistent production or testnet descriptors.

The future dry-run must not perform SPL mint authority setup against real assets.

The future dry-run must not submit transactions to any network.

## Required future preflight checks

Before any future execution checkpoint, the following preflight checks must pass:

- current branch must be explicitly chosen for execution planning
- main must include B6.58, B6.59, and B6.60
- working tree must be clean except disposable tmp output
- fixture directory must exist locally
- fixture directory must contain exactly 10 approved files
- every generated JSON file must parse
- forbidden material scan must pass
- no real RPC endpoint may be present
- no keypair path may be present
- no seed phrase may be present
- no private-key marker may be present
- no credential marker may be present
- no upgrade command may be present
- no submit command may be present
- no testnet descriptor may be created

## Future local-validator isolation policy

If a later checkpoint receives explicit GO for local-validator execution, it must use:

- local-only disposable ledger directory
- local-only mock accounts
- local-only mock fixtures
- local-only runtime-generated mock key material if key material is unavoidable
- no committed runtime-generated key material
- no real keys
- no real seed phrases
- no production/testnet keypair paths
- no authenticated RPC URLs
- no network submit path

## Future command policy

B6.61 does not define a runnable shell command.

A future command must be introduced in a separate checkpoint only after explicit scoped GO.

The future command must include hard abort checks for:

- wrong branch
- unexpected working tree changes
- missing fixture bundle
- wrong fixture file count
- JSON parse failure
- forbidden material detection
- non-local RPC markers
- real keypair path markers
- upgrade/init/submit markers outside the approved local-only boundary

## Blocker H relationship

B6.61 does not close blocker H.

B6.61 does not execute blocker H.

B6.61 only prepares a planning boundary for possible future blocker H local-validator dry-run work.

Blocker H remains open and separately gated.

Blockers A through G remain open and are not affected by B6.61.

## Result

B6.61 defines the local-validator dry-run planning-only boundary.

No validator was run.

No execution occurred.

Current status:

LOCAL_VALIDATOR_DRY_RUN_PLANNING_ONLY_DEFINED_NO_EXECUTION

Current decision remains:

NO-GO FOR LOCAL_VALIDATOR_EXECUTION_TESTNET_SIGNING_SPL_UPGRADE_INIT_SUBMIT

## Next safe step

The next safe step is either:

- request Theo review of the B6.58 through B6.61 local-validator preparation lane
- or create a separate B6.62 command-boundary checkpoint with no execution

No local-validator command should be executed without a new explicit scoped GO from Sergey.
