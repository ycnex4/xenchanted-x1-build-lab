# XC epoch minimum real RPC smoke run notes

This document defines the safe manual procedure for running the XC epoch minimum real RPC smoke script.

This milestone is notes-only unless an explicit real RPC run is performed separately.

Do not paste RPC URLs, API keys, private keys, mnemonics, seed phrases, or `.env` contents into chat.

## Current baseline

Branch:

```text
xc-epoch-minimum-real-rpc-smoke-run-notes
```

Latest known baseline before real RPC run notes:

- `npm run typecheck` passed
- `npm test` passed: 37 test files, 286 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Manual refusal check without env:

```text
node ./dist/scripts/read-xc-epoch-minimum-source.js
```

Expected safe result:

```text
Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL
```

This confirms the script refuses before RPC when the required secret-like RPC URL is absent.

## Script

Manual script:

```text
scripts/read-xc-epoch-minimum-source.ts
```

Built script:

```text
dist/scripts/read-xc-epoch-minimum-source.js
```

Package script:

```text
npm run smoke:xc-epoch-minimum:rpc
```

## Safety rules

Never print:

- RPC URL
- API key
- `.env` contents
- raw `process.env`
- private key
- mnemonic
- seed phrase
- authorization headers
- provider internals

Do not run commands like:

```text
echo $XC_ETHEREUM_RPC_URL
env
printenv
cat .env
grep RPC .env
```

Use hidden input for RPC URL when running manually.

## Required environment values

Required:

```text
XC_ETHEREUM_RPC_URL
XC_ETHEREUM_CHAIN_ID
XC_ETHEREUM_LENS_ADDRESS
XC_ETHEREUM_LOCK_EPOCHS
XC_ETHEREUM_REAL_RPC_CONFIRM
```

Optional:

```text
XC_ETHEREUM_FINALITY
XC_ETHEREUM_CONFIRMATIONS
XC_ETHEREUM_EPOCH_MINIMUM_FUNCTION
XC_ETHEREUM_EPOCH_MINIMUM_ABI_PATH
```

Required confirmation:

```text
XC_ETHEREUM_REAL_RPC_CONFIRM=I_UNDERSTAND_THIS_USES_REAL_RPC
```

Supported chains:

```text
eip155-1
eip155-11155111
```

## Safe manual run command pattern

Use this pattern so the RPC URL is typed invisibly and is not printed.

Replace only the non-secret placeholders.

```bash
read -s -p "XC_ETHEREUM_RPC_URL: " XC_ETHEREUM_RPC_URL
echo

export XC_ETHEREUM_RPC_URL
export XC_ETHEREUM_CHAIN_ID="eip155-1"
export XC_ETHEREUM_LENS_ADDRESS="<LENS_ADDRESS>"
export XC_ETHEREUM_FINALITY="finalized"
export XC_ETHEREUM_LOCK_EPOCHS="0,1,2"
export XC_ETHEREUM_REAL_RPC_CONFIRM="I_UNDERSTAND_THIS_USES_REAL_RPC"

npm run smoke:xc-epoch-minimum:rpc

unset XC_ETHEREUM_RPC_URL
unset XC_ETHEREUM_REAL_RPC_CONFIRM
```

Do not paste the actual RPC URL into chat.

Do not paste full command history if it includes the RPC URL.

## Safe output expectations

Allowed output may include:

- `manualRpcSmoke=true`
- provider chain ID
- safe config summary
- configured chain ID
- Lens address
- finality
- lock epoch count
- epoch minimum function name
- ABI path presence boolean
- real RPC confirmation boolean
- epoch minimum result lines
- completion status

Forbidden output:

- RPC URL
- API key
- private key
- mnemonic
- seed phrase
- raw env
- full parsed config
- transport config

## Result logging policy

If a real RPC run is performed, record only sanitized results.

Allowed result example:

```text
manualRpcSmoke=true
providerChainId=1
safeConfigSummary
chainId=eip155-1
lensAddress=0x...
finality=finalized
lockEpochCount=3
epochMinimumFunctionName=epochMinimum
hasEpochMinimumAbiPath=false
realRpcConfirmed=true
epochMinimum lockEpoch=0 minimumXntd=...
epochMinimum lockEpoch=1 minimumXntd=...
epochMinimum lockEpoch=2 minimumXntd=...
completed=true
```

Do not record RPC URL or provider account/API details.

## Failure logging policy

Allowed failure notes:

```text
Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL
Missing required Ethereum script confirmation: XC_ETHEREUM_REAL_RPC_CONFIRM
Unsupported Ethereum script config: XC_ETHEREUM_CHAIN_ID
Ethereum script chain mismatch: configured=1, provider=11155111
Manual RPC smoke script failed with a sanitized runtime error.
```

Forbidden failure notes:

```text
Failed to connect to https://provider.example/SECRET_API_KEY
```

If an error includes a URL or API key, do not paste it into chat or documents.

## Decision

The branch may proceed in one of two safe ways:

1. Notes-only commit:
   - commit this run procedure and checkpoint without performing real RPC.

2. Real RPC run:
   - perform the run locally using hidden input
   - record only sanitized output
   - do not paste or commit secrets

Default safe path:

```text
notes-only first
```
