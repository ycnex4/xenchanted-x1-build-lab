# XC epoch minimum real RPC smoke run notes review

This document reviews the safe real RPC smoke run notes milestone.

Reviewed branch:

```text
xc-epoch-minimum-real-rpc-smoke-run-notes-review
```

Reviewed notes milestone:

```text
xc-epoch-minimum-real-rpc-smoke-run-notes
```

Reviewed files:

- `implementation/xc-epoch-minimum-real-rpc-smoke-run-notes.md`
- `docs/checkpoints/current-design-checkpoint.md`

## Review summary

The real RPC smoke run notes are accepted.

This review confirms the milestone is notes-only.

No real RPC run was performed.

No RPC URL was used.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content was printed.

## Notes document review

The notes document defines a safe manual procedure for a future real RPC smoke run.

It explicitly warns not to paste:

- RPC URLs
- API keys
- private keys
- mnemonics
- seed phrases
- `.env` contents

It documents forbidden commands:

```text
echo $XC_ETHEREUM_RPC_URL
env
printenv
cat .env
grep RPC .env
```

These commands appear only as warnings inside the notes document.

They are not executable project scripts.

## Safe command pattern review

The notes use hidden input for the RPC URL:

```text
read -s -p "XC_ETHEREUM_RPC_URL: " XC_ETHEREUM_RPC_URL
```

This is the correct safe pattern because the typed RPC URL is not displayed in the terminal.

The notes also unset secret-like values after the run:

```text
unset XC_ETHEREUM_RPC_URL
unset XC_ETHEREUM_REAL_RPC_CONFIRM
```

## Output policy review

The notes define allowed sanitized output:

- `manualRpcSmoke=true`
- provider chain ID
- safe config summary
- chain ID
- Lens address
- finality
- lock epoch count
- function name
- ABI path presence boolean
- real RPC confirmation boolean
- epoch minimum result lines
- completion status

The notes forbid output of:

- RPC URL
- API key
- private key
- mnemonic
- seed phrase
- raw env
- full parsed config
- transport config

## Failure policy review

Allowed failure examples are sanitized.

Forbidden failure examples explicitly include URL/API-key leakage and are documented only as a warning.

If a future real RPC run produces an error containing a URL or API key, that error must not be pasted into chat or committed.

## Diff review

Diff from pre-notes baseline to current HEAD shows only:

- `docs/checkpoints/current-design-checkpoint.md`
- `implementation/xc-epoch-minimum-real-rpc-smoke-run-notes.md`

No runtime code changed.

No dependencies changed.

No script changed.

No real RPC execution result was committed.

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

This confirms the built script still refuses safely before RPC when no RPC URL is provided.

## Review decision

The real RPC smoke run notes are accepted.

No changes are required before merging this review checkpoint.

Recommended next step after merge:

```text
optional manual real RPC smoke run
```

That run must be performed locally with the RPC URL kept out of chat and out of committed files.
