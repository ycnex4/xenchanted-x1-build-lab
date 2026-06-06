# XC epoch minimum manual RPC smoke script design review notes

This document reviews the manual RPC smoke script design milestone.

Reviewed branch:

```text
xc-epoch-minimum-manual-rpc-smoke-script-design-review
```

Reviewed design milestone:

```text
xc-epoch-minimum-manual-rpc-smoke-script-design
```

Reviewed document:

- `implementation/xc-epoch-minimum-manual-rpc-smoke-script-design.md`

## Review summary

The manual RPC smoke script design preserves the current safety boundary.

The milestone is design-only.

It does not add real RPC.

It does not add viem.

It does not add a script file.

It does not add process.env runtime usage.

It does not add public client construction.

It does not add signer, wallet, or transaction capability.

## Scope review

The design only introduces documentation for a future manual-only real RPC smoke script.

The proposed future script path is:

```text
scripts/read-xc-epoch-minimum-source.ts
```

That file was not added in the design milestone.

The future script is explicitly scoped to manual smoke testing only.

It must not run as part of:

- npm test
- npm run build
- CI
- default package lifecycle scripts
- pretest
- postinstall
- prepare
- any automatic check

## Future boundary review

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

The design keeps real RPC ownership at the script edge only.

The design does not move RPC construction into:

- `src/model`
- `src/ethereum/ethereum-script-runner.ts`
- `src/ethereum/ethereum-readonly-rpc-integration.ts`
- `src/ethereum/ethereum-viem-read-provider-wrapper.ts`
- reusable exported RPC URL factory

## Confirmation review

The design correctly requires the future script to refuse running unless this exact confirmation is present:

```text
XC_ETHEREUM_REAL_RPC_CONFIRM=I_UNDERSTAND_THIS_USES_REAL_RPC
```

The design states that this confirmation must be checked through the existing parser.

The future script must not bypass parser validation.

## Dependency review

The design identifies `viem` as the recommended future dependency.

The design correctly does not add `viem` now.

The design states that viem should be introduced only in a later script-edge implementation milestone.

The existing library/model/helper layers should remain free of viem runtime imports.

## Output and secret safety review

The design allows future script output for:

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

The design forbids future script output for:

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

This matches the existing safety policy.

## Chain handling review

The design defines the first recommended real target as:

```text
eip155-1
```

Optional future Sepolia support must be explicit:

```text
eip155-11155111
```

The design correctly requires unsupported chain IDs to be refused instead of guessed.

The design also requires comparing configured chain ID with provider `getChainId()` result.

## ABI policy review

The design keeps ABI path loading out of the first real smoke script.

The recommended first behavior is:

- use default epoch minimum ABI behavior
- parse ABI path presence through existing config parser
- do not load ABI file yet

If ABI file loading becomes necessary later, it should get a separate design milestone.

## Diff review

Review diff from pre-design baseline to current HEAD shows only:

- `docs/checkpoints/current-design-checkpoint.md`
- `implementation/xc-epoch-minimum-manual-rpc-smoke-script-design.md`

No runtime source file was added or changed.

No script file was added.

No package dependency was added.

## Boundary grep review

Review command:

```bash
grep -RniE "from ['\"]viem['\"]|from ['\"]ethers['\"]|createPublicClient|http\\(|writeContract|sendTransaction|walletClient|privateKey|mnemonic|process\\.env" src tests package.json || true
```

No forbidden runtime/test/package matches were found.

## Validation baseline

Review baseline:

- `npm run typecheck` passed
- `npm test` passed: 37 test files, 286 tests
- `npm run build` passed
- `npm audit --audit-level=moderate` found 0 vulnerabilities

## Review decision

The manual RPC smoke script design is accepted.

No design changes are required before merging this review checkpoint.

Recommended next milestone after merge:

```text
xc-epoch-minimum-manual-rpc-smoke-script
```

That next milestone may implement the manual-only script, but it must still preserve the documented boundary:

- script-edge process.env only
- read-only public client only
- explicit confirmation
- safe output only
- no private keys
- no signers
- no wallet clients
- no writeContract
- no sendTransaction
- no default/CI execution
