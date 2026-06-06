# XC epoch minimum mocked script entrypoint review notes

This document reviews the mocked script entrypoint implementation.

Reviewed branch:

```text
xc-epoch-minimum-mocked-script-entrypoint-review
```

Reviewed implementation milestone:

```text
xc-epoch-minimum-mocked-script-entrypoint
```

Reviewed files:

- `src/ethereum/ethereum-script-runner.ts`
- `tests/ethereum-script-runner.test.ts`
- `src/index.ts`
- `docs/checkpoints/current-design-checkpoint.md`

## Review summary

The mocked script entrypoint implementation preserves the intended safety boundary.

The runner is a testable provided-client orchestration layer.

It does not introduce real RPC.

It does not introduce viem.

It does not read process.env.

It does not construct a public client.

It does not create any signing, wallet, or transaction path.

## Runtime boundary review

The runner accepts:

- env-like object
- already provided public client
- output writer abstraction

The runner does not own:

- RPC URL usage
- public client construction
- HTTP transport construction
- real network setup
- filesystem ABI loading
- signer creation
- wallet client creation
- transaction sending

The runner flow is:

```text
env-like input
-> parseEthereumScriptConfig()
-> summarizeEthereumScriptConfig()
-> provided public client
-> createXcEpochMinimumSourceFromReadonlyEthereumPublicClient()
-> source.authoritativeEpochMinimum(lockEpoch)
-> safe output writer
-> safe structured result
```

This matches the design.

## Source review

`src/ethereum/ethereum-script-runner.ts` imports only existing local modules and types.

It imports:

- `EthereumScriptConfigEnv`
- `EthereumScriptSafeConfigSummary`
- `parseEthereumScriptConfig`
- `summarizeEthereumScriptConfig`
- `createXcEpochMinimumSourceFromReadonlyEthereumPublicClient`
- `ViemLikePublicClient`

It does not import:

- viem
- ethers
- process.env
- createPublicClient
- http transport
- wallet client
- signer
- transaction helpers

The full parsed config is kept as a local variable.

The returned result contains only:

- `safeConfigSummary`
- `epochMinimums`
- `completed`

The runner does not return:

- rpcUrl
- raw env
- full parsed config
- transport config
- provider internals

## Safe output review

The runner writes only safe output lines:

- `safeConfigSummary`
- chain ID
- Lens address
- finality
- lock epoch count
- epoch minimum function name
- ABI path presence boolean
- real RPC confirmation boolean
- epoch minimum result lines
- completion status

The runner does not print:

- RPC URL
- API key
- raw env
- full config
- transport config
- provider internals
- private key
- mnemonic
- signer data
- wallet client data

## ABI path review

ABI path remains metadata-only in this milestone.

The runner may report whether ABI path is present through the safe summary.

The runner does not load ABI files.

The runner does not read the filesystem.

The runner preserves default epoch minimum ABI behavior through the existing provider path.

## Test review

`tests/ethereum-script-runner.test.ts` covers:

- successful run with env-like input and provided mocked public client
- safe config summary output
- no RPC URL / API-key-like output
- no full parsed config in returned result
- confirmed finality propagation
- lock epoch propagation
- function name propagation
- ABI path as metadata only
- sanitized parser validation errors
- provided public client usage

Test count after implementation:

```text
37 test files
286 tests
```

## Import and boundary grep

Review command:

```bash
grep -RniE "from ['\"]viem['\"]|from ['\"]ethers['\"]|createPublicClient|http\\(|writeContract|sendTransaction|walletClient|privateKey|mnemonic|process\\.env" src tests package.json || true
```

The grep did not show forbidden runtime/test/package matches.

The broader implementation documents contain many deliberate references to forbidden terms because they describe safety rules, but runtime/test/package path remains clean.

## Validation baseline

Review baseline:

- `npm run typecheck` passed
- `npm test` passed: 37 test files, 286 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

## Review decision

The mocked script entrypoint implementation is accepted.

No implementation changes are required before merging this review checkpoint.

The project can proceed to the next milestone after merge.

Recommended next milestone:

```text
xc-epoch-minimum-manual-rpc-smoke-script-design
```

That next milestone should still be design-only.

It should define the future manual-only real RPC smoke script boundary before adding viem or real RPC.
