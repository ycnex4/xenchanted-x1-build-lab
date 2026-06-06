# XC epoch minimum manual RPC smoke script review notes

This document reviews the manual RPC smoke script implementation.

Reviewed branch:

```text
xc-epoch-minimum-manual-rpc-smoke-script-review
```

Reviewed implementation milestone:

```text
xc-epoch-minimum-manual-rpc-smoke-script
```

Reviewed files:

- `scripts/read-xc-epoch-minimum-source.ts`
- `package.json`
- `package-lock.json`
- `tsconfig.json`
- `docs/checkpoints/current-design-checkpoint.md`

## Review summary

The manual RPC smoke script implementation is accepted.

The implementation preserves the intended boundary:

- real RPC exists only in the manual script
- `process.env` is read only at script edge
- viem is imported only by the manual script
- public client construction is only inside the manual script
- the script passes a provided read-only public client into the existing runner
- no private key, mnemonic, signer, wallet, or transaction path was added

## Manual-only review

The package script added is:

```text
smoke:xc-epoch-minimum:rpc
```

It runs:

```text
node ./dist/scripts/read-xc-epoch-minimum-source.js
```

The script is not part of:

- `npm test`
- `npm run build`
- CI
- default package lifecycle scripts
- pretest
- postinstall
- prepare

It is manual-only.

## Script edge review

The script file is:

```text
scripts/read-xc-epoch-minimum-source.ts
```

The script reads `process.env` only in `readEthereumScriptEnv()`.

The script constructs the viem public client only in `createReadonlyPublicClient()`.

The script supports only explicit chains:

```text
eip155-1
eip155-11155111
```

The script validates configured chain ID through the existing parser and then verifies provider `getChainId()` against the configured chain.

## Read-only behavior review

The script adapts viem to the existing `ViemLikePublicClient` interface.

It exposes only:

- `getChainId`
- `getBlock`
- `readContract`

It does not expose or call:

- `writeContract`
- `sendTransaction`
- approvals
- token transfers
- contract writes

## Secret safety review

The script keeps output sanitized.

Without required env, the built script safely refuses before RPC with:

```text
Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL
```

No RPC URL, API key, private key, mnemonic, or seed phrase was printed.

Runtime errors that are not known validation errors are sanitized as:

```text
Manual RPC smoke script failed with a sanitized runtime error.
```

## Dependency review

The implementation adds:

```text
viem
```

The dependency is used only by the manual script.

The implementation does not add `tsx` or `ts-node`.

The script is compiled by `tsc` because `tsconfig.json` now includes:

```text
scripts/**/*.ts
```

## Boundary grep review

Review command:

```bash
grep -RniE "from ['\"]viem['\"]|from ['\"]ethers['\"]|createPublicClient|http\\(|process\\.env" src scripts tests package.json || true
```

Expected and observed matches were only in:

```text
scripts/read-xc-epoch-minimum-source.ts
```

Review command:

```bash
grep -RniE "privateKey|mnemonic|walletClient|writeContract|sendTransaction" src scripts tests package.json || true
```

No matches were found.

## Validation baseline

Review baseline:

- `npm run typecheck` passed
- `npm test` passed: 37 test files, 286 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

Manual refusal check:

```text
node ./dist/scripts/read-xc-epoch-minimum-source.js
```

Result:

```text
Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL
```

## Review decision

The manual RPC smoke script implementation is accepted.

No implementation changes are required before merging this review checkpoint.

Recommended next milestone after merge:

```text
xc-epoch-minimum-real-rpc-smoke-run-notes
```

That next milestone should be manual-run documentation only unless the project is ready to run a real RPC smoke test with a real RPC URL kept out of chat.
