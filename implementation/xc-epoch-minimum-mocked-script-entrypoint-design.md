# XC epoch minimum mocked script entrypoint design

This document defines the design for the next mocked script entrypoint milestone.

The goal is to design the outer orchestration layer for reading XC epoch minimum data through an injected mocked/provided public client, without introducing real RPC, viem, process.env reads, public client construction, or any signing/transaction path.

## Current baseline

The current `main` baseline already contains:

- model-level XC epoch minimum source abstractions
- Ethereum lens provider source
- dependency-free Ethereum read provider abstraction
- mocked Ethereum provider wrapper
- viem-like read provider wrapper without a viem dependency
- read-only RPC integration helper that accepts a provided public client
- script config parser that accepts an env-like object instead of reading process.env
- planning document for safe script implementation

Latest known validation baseline before this design step:

- `npm run typecheck` passed
- `npm test` passed: 36 test files, 278 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

## Design goal

The mocked script entrypoint should prove the future script flow shape safely:

```text
env-like input
-> parseEthereumScriptConfig()
-> summarizeEthereumScriptConfig()
-> injected mocked/provided public client
-> createXcEpochMinimumSourceFromReadonlyEthereumPublicClient()
-> read source data
-> write safe output only
```

This design deliberately keeps dangerous or irreversible capabilities out of scope.

## Non-goals

This milestone must not add:

- real RPC calls
- viem dependency
- runtime viem imports
- ethers dependency
- process.env reads
- public client construction
- RPC URL factory
- HTTP transport construction
- private key support
- mnemonic support
- signer support
- wallet client support
- transaction sending
- writeContract / sendTransaction path
- raw env printing
- full config printing
- RPC URL / API key printing

## Proposed file

Future implementation file:

```text
src/ethereum/ethereum-script-runner.ts
```

Future tests:

```text
tests/ethereum-script-runner.test.ts
```

The runner should live outside `src/model`.

The runner may live under `src/ethereum` because it orchestrates Ethereum-facing adapters and config parsing.

## Proposed exported types

Possible future exported types:

```ts
export interface EthereumScriptRunnerOutput {
  writeLine(line: string): void;
}

export interface EthereumScriptRunnerInput {
  env: EthereumScriptConfigEnv;
  publicClient: ViemLikePublicClient;
  output: EthereumScriptRunnerOutput;
}

export interface EthereumScriptRunnerResult {
  safeConfigSummary: EthereumScriptSafeConfigSummary;
  completed: boolean;
}
```

The exact names can change during implementation, but the boundary should remain the same:

- input receives env-like object
- input receives already-created public client
- input receives output writer abstraction
- function returns safe structured result
- function does not create real network clients

## Proposed exported function

Possible future function:

```ts
export async function runEthereumXcEpochMinimumReadFromProvidedClient(
  input: EthereumScriptRunnerInput,
): Promise<EthereumScriptRunnerResult>
```

The function name should make the provided-client boundary explicit.

Preferred wording:

```text
FromProvidedClient
```

Avoid names that imply the function owns RPC creation, such as:

```text
FromRpcUrl
FromEnv
CreatePublicClient
RealRpcRunner
```

## Internal flow

The function should do the following:

```text
1. parseEthereumScriptConfig(input.env)
2. summarizeEthereumScriptConfig(config)
3. write a safe config summary
4. createXcEpochMinimumSourceFromReadonlyEthereumPublicClient({
     publicClient: input.publicClient,
     chainId: config.chainId,
     lensAddress: config.lensAddress,
     finalityPolicy: config.finalityPolicy,
     lockEpochs: config.lockEpochs,
     epochMinimumFunctionName: config.epochMinimumFunctionName,
     epochMinimumAbi: optional loaded ABI only in later milestone
   })
5. read source data using the model/source API
6. write sanitized result summary
7. return a structured safe result
```

The runner must not print or return the full parsed config object.

The runner may keep the full parsed config only as a local variable.

## Safe output policy

The output writer may receive only safe strings.

Allowed output fields:

- chain ID
- lens address
- finality policy
- lock epoch count
- epoch minimum function name
- whether ABI path is present
- whether real RPC confirmation flag is present
- sanitized source result fields
- completion status

Forbidden output fields:

- rpcUrl
- raw env
- full parsed config
- API-key-like values
- authorization headers
- transport config
- provider internals
- private keys
- mnemonics
- seed phrases
- wallet client data
- signer data

## RPC URL handling

The parser currently returns `rpcUrl` because a future outer real script will need it to construct a read-only public client.

The mocked script runner must not print `rpcUrl`.

The mocked script runner should not pass `rpcUrl` into any helper.

The mocked script runner should not construct any transport from `rpcUrl`.

The only allowed use of full config inside the mocked runner is to pass safe protocol parameters into existing helpers.

## Error policy

The runner should preserve sanitized error behavior.

It should not catch and rethrow errors with:

- raw env
- full config
- RPC URL
- API key
- provider internals
- transport details

If the runner wraps errors, it should use generic sanitized messages.

Preferred approach for the first implementation:

```text
do not wrap parser validation errors unless there is a specific need
```

Provider errors should also be sanitized before output if they are written to the output abstraction.

## Dependency injection policy

The runner must receive dependencies as inputs.

Required dependencies:

- env-like object
- provided public client
- output writer

Optional future dependencies:

- ABI loader abstraction
- logger abstraction
- clock abstraction if needed for deterministic tests

The runner must not directly read global state.

Specifically, the runner must not read:

- process.env
- filesystem ABI path directly in the first mocked milestone
- network clients from globals
- wallet/signer globals

## ABI path policy

The parser already supports an optional ABI path.

For the first mocked script runner implementation, ABI path handling should remain inactive unless a separate ABI loader abstraction is explicitly designed.

Allowed first implementation behavior:

- parse and summarize whether ABI path is present
- do not load files
- use default epoch minimum ABI

If ABI file loading becomes necessary later, it should be a separate milestone with path safety rules.

## Tests to add in implementation milestone

The future implementation should add tests for:

1. runs with env-like input and provided mocked public client
2. writes safe config summary
3. does not write RPC URL
4. does not write raw env
5. does not write full config
6. does not expose API-key-like substrings in output
7. passes finality policy through to the source helper
8. passes lock epochs through to the source helper
9. uses provided public client only
10. does not import viem
11. does not read process.env
12. returns a safe structured result
13. propagates sanitized parser validation errors
14. keeps ABI path as metadata only in the first runner milestone

## Import boundary checks

Implementation review should verify:

```bash
grep -RniE "from ['\"]viem['\"]|from ['\"]ethers['\"]|createPublicClient|http\\(|writeContract|sendTransaction|walletClient|privateKey|mnemonic|process\\.env" src tests implementation package.json
```

This command is safe because it searches for filenames/lines and does not print secret values unless secrets were incorrectly committed into source files.

Before running broader secret checks, use filename-only checks and do not print actual secret values.

## Future real RPC separation

The mocked runner is not the real RPC script.

The future real RPC script should remain separate and should live under:

```text
scripts/read-xc-epoch-minimum-source.ts
```

That future script may eventually:

- read process.env at the script edge
- require explicit confirmation
- construct a read-only public client
- pass that client into the provided-client runner

But that is not part of the mocked runner implementation.

## Package script policy

The mocked runner implementation should not add a package script that runs real RPC.

If a package script is added later for real RPC, it must be manual-only and clearly named.

It must not be included in:

- npm test
- npm run build
- CI

## Decision

The next implementation milestone should be:

```text
xc-epoch-minimum-mocked-script-entrypoint
```

It should add only a mocked/testable runner around already-existing safe pieces.

It should not add real RPC, viem, process.env, public client construction, or transaction capability.

The expected implementation files are:

```text
src/ethereum/ethereum-script-runner.ts
tests/ethereum-script-runner.test.ts
```

The expected export update is:

```text
src/index.ts
```

The implementation should increase the test count while preserving the same safety boundary.
