# xEnchanted X1 Build Lab deployment readiness

This document defines the post-MVP deployment readiness boundary for the xEnchanted X1 Build Lab.

This checkpoint is documentation-only.

No runtime code is changed in this checkpoint.

No dependencies are changed in this checkpoint.

No real RPC is executed in this checkpoint.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Current status

The repository is a tested MVP implementation lab.

Current validation baseline:

- npm run typecheck passed
- npm test passed: 42 test files, 328 tests
- npm run build passed
- npm audit --audit-level=moderate found 0 vulnerabilities

The MVP implementation lab is complete at the current scope.

This document does not turn the lab into a production deployment.

## Current deployable unit

The current deployable unit is:

    TypeScript implementation lab package

It includes:

- domain model
- Build accounting logic
- registrar handlers
- proof models
- watcher candidate models
- proof conversion
- snapshot serialization / recovery helpers
- read-only CLI
- read-only Ethereum provider wrappers
- XC epoch minimum read source helpers
- appGetBuildView()
- commitmentStatus model

It does not include:

- production chain deployment
- smart contract deployment
- live watcher service runtime
- bridge execution runtime
- token issuance runtime
- UI runtime
- operator production stack
- trustless proof verification

## Current package scripts

Current scripts:

    npm run typecheck
    npm test
    npm run build
    npm run cli -- <command>
    npm run smoke:xc-epoch-minimum:rpc

The normal safe validation sequence is:

    npm run typecheck
    npm test
    npm run build
    npm audit --audit-level=moderate

## CLI boundary

The CLI is read-only.

Supported CLI commands:

    npm run cli -- help
    npm run cli -- version
    npm run cli -- snapshot:show --file <path>
    npm run cli -- snapshot:verify --file <path>
    npm run cli -- snapshot:recover --file <path> [--backup <path>]

The CLI does not:

- mutate protocol state
- mutate snapshot files
- restore automatically
- migrate data
- delete corrupted files
- execute transactions
- call write RPC methods

## RPC boundary

The only current RPC-facing script is:

    npm run smoke:xc-epoch-minimum:rpc

Purpose:

    read XC epoch minimum / protocol params source data through configured RPC

The smoke script is not a deploy script.

It should remain read-only.

It must not:

- execute transactions
- use a wallet private key
- create a wallet client
- call writeContract
- call sendTransaction
- print RPC URLs
- print API keys
- print `.env` contents
- print private keys, mnemonics, seed phrases, or tokens

## Secret safety

Any future deployment or smoke command must follow these rules:

- never print raw RPC URLs
- never print API keys
- never print private keys
- never print mnemonics
- never print seed phrases
- never print `.env` contents
- never ask users to paste secrets into chat
- only report presence / absence of required config
- redact sensitive values in diagnostics

Safe wording:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

Unsafe wording:

    XC_ETHEREUM_RPC_URL=https://...

## First deployment target

The first deployment target should not be production.

Recommended order:

1. local build validation
2. read-only CLI validation
3. read-only RPC smoke validation
4. staging runtime design
5. watcher runtime design
6. registrar authority / signature design
7. production deployment readiness

Do not skip directly from MVP lab to production deployment.

## Dry-run definition

A dry-run may:

- run typecheck
- run tests
- run build
- run npm audit
- run read-only CLI commands against local files
- run read-only RPC smoke command when config is present
- validate required config presence without printing values

A dry-run must not:

- execute transactions
- mutate chain state
- mutate snapshots unless explicitly testing local snapshot helpers
- publish packages
- deploy services
- deploy contracts
- start production watchers
- create bridge messages
- issue tokens

## Real deploy definition

A real deploy would mean one of the following:

- publishing a package
- deploying a service runtime
- deploying a watcher
- deploying a registrar runtime
- deploying bridge infrastructure
- deploying smart contracts
- deploying UI/API infrastructure

None of these are currently implemented as production deployment flows.

Each real deploy target requires a separate readiness document.

## Required pre-deploy checks

Before any future deploy-like action, run:

    npm run typecheck
    npm test
    npm run build
    npm audit --audit-level=moderate

Expected current baseline:

    42 test files passed
    328 tests passed
    found 0 vulnerabilities

Also check:

    git status --short

Expected:

    clean working tree

## Current production blockers

The current repo should not be treated as production deployment-ready until these post-MVP layers are designed:

- live watcher runtime
- registrar authority / signature model
- production config model
- operator key management model
- read/write RPC boundary
- service monitoring
- incident response
- deployment rollback procedure
- trustless proof verification path, if required
- external integration policy

## Explicit non-goals for first deploy readiness

The first deployment readiness step does not include:

- Build actor
- Forge participation requirement
- unlock flow
- BLD marketplace
- UI
- bridge execution
- token issuance
- smart contract deployment
- production trustless verification
- production watcher runtime

## Recommended next milestone

Recommended next milestone:

    post-mvp-readonly-rpc-smoke-review

Purpose:

- review the existing read-only RPC smoke path
- confirm config handling is secrets-safe
- confirm the smoke command does not execute transactions
- document required env variables without printing values
- define expected success / failure output
- keep deployment scope read-only

## Decision

The current repository is ready for post-MVP deployment readiness planning.

It is not ready for production deployment.

The next practical step should be read-only RPC smoke review and staging runtime design, not new protocol features.
