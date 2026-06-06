# XC epoch minimum script implementation planning

This document defines the next safe implementation step for the Ethereum XC epoch minimum source path.

The current baseline already contains:

- model-level XC epoch minimum source abstractions
- Ethereum lens provider source
- dependency-free Ethereum read provider abstraction
- mocked Ethereum provider wrapper
- viem-like read provider wrapper without a viem dependency
- read-only RPC integration helper that accepts a provided public client
- script config parser that accepts an env-like object instead of reading process.env

The current baseline does not contain:

- real RPC usage
- viem dependency
- process.env reads in model, wrapper, helper, or parser
- public client construction
- RPC URL factory
- private keys
- signers
- wallet clients
- transaction sending
- writeContract / sendTransaction path

## Current baseline

Last known baseline before this planning step:

- 36 test files passed
- 278 tests passed
- npm run build passed
- npm audit --audit-level=moderate found 0 vulnerabilities

## Goal of the next implementation step

The next implementation step should add a mocked script entrypoint layer before any real RPC implementation.

The goal is to prove the script flow shape safely:

```text
env-like input
-> parseEthereumScriptConfig()
-> summarizeEthereumScriptConfig()
-> injected mocked/provided public client
-> createXcEpochMinimumSourceFromReadonlyEthereumPublicClient()
-> read source data
-> print safe summary only
```

This keeps the future real script design clear while avoiding real RPC, viem, process.env, or secret exposure.

## Recommended next milestone

Recommended branch after this planning branch:

```text
xc-epoch-minimum-mocked-script-entrypoint-design
```

Purpose:

- design the mocked script entrypoint before implementation
- define what the script may print
- define what the script must never print
- define how dependencies are injected for tests
- define how future real RPC construction will remain outside model/wrapper/helper layers

## Why mocked script entrypoint before real RPC

A mocked script entrypoint is the safest next step because it tests the outer orchestration shape without introducing the dangerous parts:

- no real RPC endpoint
- no API key handling
- no viem dependency
- no http transport
- no process.env read requirement
- no wallet/signing path
- no accidental transaction capability

It also lets tests enforce the most important rule early:

```text
only safe summaries are printable
```

## Proposed mocked script entrypoint boundary

The mocked script entrypoint should be implemented as a testable function, not as an immediately-executing real script.

Possible future file:

```text
src/ethereum/ethereum-script-runner.ts
```

Possible exported function:

```ts
runEthereumXcEpochMinimumReadFromProvidedClient(input)
```

The function should accept:

- env-like config object
- provided public client
- output writer abstraction
- optional logger abstraction
- optional ABI loader abstraction if needed later

The function should not:

- read process.env directly
- construct a real public client
- accept an RPC URL outside parsed config needs
- print the RPC URL
- print API keys
- print raw env
- print full config
- import viem
- import ethers
- create a signer
- create a wallet client
- send transactions

## Safe output policy

The mocked script runner may print:

- chain ID
- lens address
- finality policy
- lock epoch count
- function name
- whether ABI path is present
- whether explicit real RPC confirmation is present
- sanitized source result fields that do not contain secrets

The mocked script runner must not print:

- rpcUrl
- raw env object
- full parsed config object
- API-key-like substrings
- transport config
- provider internals
- private keys
- mnemonics
- seed phrases
- authorization headers

## Error policy

Errors must be sanitized.

Validation errors from parseEthereumScriptConfig() already avoid leaking RPC URL / API-key-like values.

The mocked script runner should preserve that policy and avoid wrapping errors with raw config, raw env, provider internals, or transport details.

## Dependency policy

The next implementation step should still not install viem.

The runner should use the existing viem-like public client type and mocked clients in tests.

No dependency additions are required for the mocked script entrypoint step.

## Future real RPC milestone

Real RPC should remain a later milestone after:

1. mocked script entrypoint design
2. mocked script entrypoint implementation
3. mocked script entrypoint review
4. manual-only real RPC smoke script design
5. review of the manual-only real RPC smoke script design

Only after those steps should the project consider:

- installing viem
- creating a manual-only script under scripts/
- reading process.env at the script edge
- constructing createPublicClient/http transport
- requiring explicit confirmation

## Future real RPC script rules

The future real RPC script must be manual-only.

It must not be part of:

- npm test
- npm run build
- CI
- default package scripts that can run accidentally

It must require:

```text
XC_ETHEREUM_REAL_RPC_CONFIRM=I_UNDERSTAND_THIS_USES_REAL_RPC
```

It must use only read-only calls:

- getChainId
- getBlock
- readContract

It must not include:

- private key support
- signer support
- wallet client support
- writeContract
- sendTransaction
- contract writes
- approvals
- token transfers

## Parser hardening ideas

The existing parser is sufficient for the next mocked script entrypoint step.

Optional future hardening can be considered separately:

- duplicated lock epoch values policy
- very large lock epoch values policy
- whitespace normalization around all env values
- ABI path traversal policy if ABI path support becomes active
- explicit tests that safe summary is the only printable config object

These are not blockers for mocked script entrypoint design.

## Decision

The next safe step should be:

```text
mocked script entrypoint design
```

Not real RPC implementation.

Not viem installation.

Not process.env integration.

Not public client construction.

The project should continue in small milestones:

```text
design
review
implementation
review
checkpoint
merge
```
