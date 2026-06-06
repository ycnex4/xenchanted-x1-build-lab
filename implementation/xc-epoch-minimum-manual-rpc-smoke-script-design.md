# XC epoch minimum manual RPC smoke script design

This document defines the design for a future manual-only real RPC smoke script.

This is a design-only milestone.

No real RPC implementation is added in this milestone.

No viem dependency is added in this milestone.

No script file is added in this milestone.

## Current baseline

The current `main` baseline already contains:

- model-level XC epoch minimum source abstractions
- Ethereum lens provider source
- dependency-free Ethereum read provider abstraction
- viem-like read provider wrapper without a viem dependency
- read-only RPC integration helper that accepts a provided public client
- Ethereum script config parser that accepts an env-like object
- mocked/testable script runner that accepts a provided public client
- review notes confirming the mocked runner safety boundary

Latest known validation baseline before this design step:

- `npm run typecheck` passed
- `npm test` passed: 37 test files, 286 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

## Design goal

The future manual RPC smoke script should prove that the existing safe pipeline can read real Ethereum data through a read-only public client.

The intended future flow is:

```text
manual script invocation
-> read process.env at script edge only
-> parseEthereumScriptConfig(process.env-like object)
-> create read-only public client
-> runEthereumXcEpochMinimumReadFromProvidedClient({
     env,
     publicClient,
     output
   })
-> print safe output only
```

The real RPC script must remain outside model, wrapper, helper, and runner layers.

## Proposed future script file

Future file:

```text
scripts/read-xc-epoch-minimum-source.ts
```

This file should be added only in a later implementation milestone.

The script should not be added in this design milestone.

## Manual-only rule

The future script must be manual-only.

It must not run as part of:

- `npm test`
- `npm run build`
- CI
- default package lifecycle scripts
- pretest
- postinstall
- prepare
- any automatic check

If a package script is eventually added, it should be clearly manual, for example:

```text
smoke:xc-epoch-minimum:rpc
```

It must still require explicit confirmation.

## Required confirmation

The future script must refuse to run unless this exact confirmation is present:

```text
XC_ETHEREUM_REAL_RPC_CONFIRM=I_UNDERSTAND_THIS_USES_REAL_RPC
```

The confirmation must be checked through the existing parser.

The script must not bypass parser validation.

## Future dependency

The recommended future dependency is:

```text
viem
```

But viem must not be installed in this design milestone.

When viem is introduced later, it should be introduced only for the script-edge implementation.

The existing library/model/helper layers should not gain viem runtime imports.

## Future script responsibility

The future script may:

- read `process.env` at the script edge
- construct an env-like object for `parseEthereumScriptConfig()`
- create a read-only public client
- call the existing mocked/testable runner with the provided client
- write safe output to stdout
- exit non-zero on validation/runtime failure

The future script must not:

- expose RPC URL
- expose API key
- print raw env
- print full parsed config
- print transport config
- print provider internals
- accept private key
- accept mnemonic
- create signer
- create wallet client
- send transaction
- call writeContract
- call sendTransaction
- perform approvals
- perform token transfers
- perform any contract write

## Environment variables

The future script should use the existing parser variables:

```text
XC_ETHEREUM_RPC_URL
XC_ETHEREUM_CHAIN_ID
XC_ETHEREUM_LENS_ADDRESS
XC_ETHEREUM_FINALITY
XC_ETHEREUM_CONFIRMATIONS
XC_ETHEREUM_LOCK_EPOCHS
XC_ETHEREUM_EPOCH_MINIMUM_FUNCTION
XC_ETHEREUM_EPOCH_MINIMUM_ABI_PATH
XC_ETHEREUM_REAL_RPC_CONFIRM
```

Only `XC_ETHEREUM_RPC_URL` is secret-like.

The future script must not print it.

## Public client construction

The future script may construct a read-only public client using viem.

Conceptual future shape:

```ts
const publicClient = createPublicClient({
  chain,
  transport: http(config.rpcUrl)
});
```

This should exist only in the script file.

It must not be moved into:

- `src/model`
- `src/ethereum/ethereum-script-runner.ts`
- `src/ethereum/ethereum-readonly-rpc-integration.ts`
- `src/ethereum/ethereum-viem-read-provider-wrapper.ts`
- reusable exported RPC URL factory

## Chain handling

The future script must validate `XC_ETHEREUM_CHAIN_ID`.

The first supported real chain should be explicitly defined.

Recommended first target:

```text
eip155-1
```

If Sepolia support is needed later, it should be explicit:

```text
eip155-11155111
```

The script should refuse unsupported chain IDs instead of guessing.

The script should also compare:

```text
configured chain ID
provider getChainId()
```

If they differ, the script should fail with a sanitized error.

## Output policy

The future script may print:

- safe config summary
- configured chain ID
- provider chain ID
- Lens address
- finality policy
- lock epoch count
- epoch minimum function name
- ABI path presence boolean
- epoch minimum results
- completion status

The future script must not print:

- RPC URL
- API key
- raw process.env
- full config object
- transport config
- provider internals
- request headers
- authorization headers
- private keys
- mnemonics
- seed phrases
- signer objects
- wallet client objects

## Error policy

The future script must keep errors sanitized.

Allowed error style:

```text
Invalid Ethereum script config: XC_ETHEREUM_CHAIN_ID
```

Forbidden error style:

```text
Failed to connect to https://provider.example/SECRET_API_KEY
```

The script should avoid wrapping errors with raw config, raw env, RPC URL, API key, or transport details.

If viem errors include URL-like data, the script should sanitize before printing.

## ABI path policy

The current mocked runner keeps ABI path metadata-only.

For the future manual RPC script, ABI file loading should remain out of scope unless a separate design milestone is completed.

Recommended first real smoke script behavior:

- use default epoch minimum ABI behavior
- parse ABI path presence through existing config parser
- do not load ABI file yet

If ABI file loading becomes necessary later, create a separate design milestone for:

- path allowlist
- extension validation
- no traversal
- no arbitrary file read
- safe parse errors
- no printing file contents

## Test policy

The future real RPC script should not be exercised by normal tests against real network.

Implementation tests should use mocked public clients and/or mocked script dependencies.

Normal tests may verify:

- script config parsing
- confirmation requirement
- safe output formatting
- no RPC URL output
- unsupported chain refusal
- chain mismatch sanitization
- no private key/signer/wallet path
- package script is not part of default checks

Real network smoke testing should be manual-only.

## Package policy

If `viem` is added later, the package change must be reviewed carefully.

If a manual package script is added, it must be named clearly and must not be part of existing validation commands.

Allowed future package script example:

```json
{
  "scripts": {
    "smoke:xc-epoch-minimum:rpc": "tsx scripts/read-xc-epoch-minimum-source.ts"
  }
}
```

Do not add this in the design milestone.

## Future implementation milestone

Recommended next implementation milestone after this design is reviewed:

```text
xc-epoch-minimum-manual-rpc-smoke-script
```

That milestone may add:

- `viem` dependency
- manual-only script under `scripts/`
- safe process.env reading at script edge
- public client construction inside the script only
- sanitized output / error handling

That milestone must still not add:

- private key support
- signer support
- wallet client support
- transaction capability
- writeContract / sendTransaction
- automatic CI/default execution

## Review checklist for future implementation

Future implementation review should verify:

```bash
grep -RniE "privateKey|mnemonic|walletClient|writeContract|sendTransaction" src scripts tests package.json || true
```

For RPC URL and env checks, avoid commands that print actual values.

Do not grep `.env` contents directly.

Use filename-only or source-only checks.

## Decision

The future real RPC smoke script should be introduced only after this design is reviewed.

The next step after this design milestone should be review-only:

```text
xc-epoch-minimum-manual-rpc-smoke-script-design-review
```

No real RPC is added in this design milestone.

No viem dependency is added in this design milestone.

No process.env runtime script is added in this design milestone.
