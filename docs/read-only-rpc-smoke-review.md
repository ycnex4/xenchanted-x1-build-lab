# xEnchanted X1 Build Lab read-only RPC smoke review

This document reviews the current read-only RPC smoke path.

This review is documentation-only.

No runtime code is changed in this review.

No dependencies are changed in this review.

No real RPC is executed in this review.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Reviewed branch

    post-mvp-readonly-rpc-smoke-review

## Reviewed script

Package script:

    npm run smoke:xc-epoch-minimum:rpc

Resolved command:

    node ./dist/scripts/read-xc-epoch-minimum-source.js

Source file:

    scripts/read-xc-epoch-minimum-source.ts

## Review result

The current RPC smoke path is accepted as read-only.

The smoke path is not a deploy script.

The smoke path is not a transaction script.

The smoke path is not a production watcher runtime.

The smoke path is not a registrar runtime.

## Read-only behavior

The script creates a viem public client through:

    createPublicClient()

The script exposes only read-like methods to the internal provider wrapper:

- getChainId()
- getBlock()
- readContract()

No wallet client is created.

No write transaction method is used.

No contract write method is used.

No private key, mnemonic, seed phrase, or wallet signing path is used.

## Config handling

The script reads config from environment variables.

Required secret config:

    XC_ETHEREUM_RPC_URL

The RPC URL is treated as secret config.

Missing RPC URL produces a sanitized error:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

The value itself is not printed.

## Confirmation guard

The script requires explicit real RPC confirmation:

    XC_ETHEREUM_REAL_RPC_CONFIRM=I_UNDERSTAND_THIS_USES_REAL_RPC

Missing confirmation produces a sanitized error.

This is appropriate because the script uses real RPC even though it is read-only.

## Safe output

The script may print:

- manualRpcSmoke=true
- providerChainId
- safeConfigSummary
- chainId
- lensAddress
- finality
- lockEpochCount
- epochMinimumFunctionName
- hasEpochMinimumAbiPath
- realRpcConfirmed
- epochMinimum values
- completed=true

The script must not print:

- raw RPC URL
- API key
- private key
- mnemonic
- seed phrase
- `.env` contents
- token values

## Grep review

The reviewed files were searched for risky write/signing paths and secret output patterns.

No use was found for:

- PRIVATE_KEY
- MNEMONIC
- SEED
- walletClient
- writeContract
- sendTransaction
- createWalletClient

The only `process.env` usage is config loading.

The only `console.log` usage in the smoke script prints non-secret status and summary output.

## Test coverage review

Existing tests cover secret-safety expectations.

Tests assert that RPC URL values do not appear in:

- config errors
- runner output
- safe config summary
- serialized source state

## Boundary

This review does not approve production deployment.

This review only approves the current smoke path as a read-only RPC smoke command.

Future production deployment still requires separate readiness work for:

- live watcher runtime
- registrar authority / signature model
- production config model
- operator key management
- monitoring
- incident response
- rollback procedures

## Validation baseline

Review validation:

- npm run typecheck passed
- npm test passed: 42 test files, 328 tests
- npm run build passed
- npm audit --audit-level=moderate found 0 vulnerabilities

## Decision

The current read-only RPC smoke path is acceptable for a controlled post-MVP smoke review.

The next step may be a controlled real RPC smoke run, but only after the required environment variables are configured locally.

Do not paste RPC URLs, API keys, private keys, mnemonics, seed phrases, or `.env` contents into chat.
