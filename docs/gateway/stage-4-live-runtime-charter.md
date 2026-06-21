# Stage 4 Live Runtime Charter

This document defines the Stage 4 boundary for the X1 direct mint gateway.

Stage 4 begins only after:

- Stage 1 deterministic model closure
- Stage 2 runtime / evidence closure
- Stage 3 tooling / production surface closure
- Theo review confirming Stage 3 closure

Stage 4 is not Stage 3.11.

Stage 4 is a different class of work.

## Core distinction

Stage 3 was:

    offline tooling / production surface

Stage 4 is:

    live runtime / operations layer

Stage 4 may involve live RPC, keys, wallets, SOL, deployments, relayers, watchers, and guardian operations.

Because of that, Stage 4 must be defined separately and must not be retroactively mixed into Stage 3.

## Stage 4 purpose

Stage 4 exists to move from offline proof and tooling into controlled live operation.

Its purpose is to define, test, and harden:

- watcher runtime
- relayer runtime
- guardian operations
- deploy pipeline
- live configuration handling
- live RPC access
- transaction submission boundaries
- operator safety procedures
- incident / rollback procedures
- evidence capture for live runs

Stage 4 is not a model proof stage.

Stage 4 is not an offline tooling stage.

Stage 4 is an operations stage.

## What changes in Stage 4

Stage 4 introduces risk classes that were intentionally excluded from Stage 3:

- live RPC
- wallet access
- signer / guardian key handling
- SOL balance requirements
- transaction submission
- deployment actions
- live watcher loops
- live relayer loops
- operational monitoring
- production incident response

These risk classes must be handled explicitly and separately.

## Hard safety rules

No private key, seed phrase, wallet JSON, RPC API key, guardian secret, deployer secret, or signer material may be pasted into chat.

Forbidden secret-bearing material includes values for names such as:

- PRIVATE_KEY
- MNEMONIC
- SEED_PHRASE
- SECRET_KEY
- ANCHOR_WALLET
- WALLET_JSON
- RPC_URL
- RPC_API_KEY
- GUARDIAN_PRIVATE_KEY
- DEPLOYER_PRIVATE_KEY

These names may appear in documentation as forbidden markers.

Real values must not be printed.

Real values must not be committed.

Real values must not be copied into issue text, docs, logs, terminal transcripts, or chat.

## Logging policy

Stage 4 commands and scripts must not print secret values.

Allowed logging:

- config key names
- redacted values
- public addresses
- public program IDs
- public transaction signatures
- non-sensitive status
- non-sensitive balances
- non-sensitive error summaries

Disallowed logging:

- private keys
- mnemonic phrases
- seed phrases
- full wallet JSON
- RPC API keys
- bearer tokens
- guardian signing secrets
- deployer secrets
- raw environment dumps
- unredacted `.env` files

Any diagnostic command that could print secrets must be replaced by a redacted or filename-only check.

## Environment policy

Stage 4 may require local environment files.

Allowed:

- `.env.example`
- `.env.stage4.example`
- redacted config examples
- local ignored `.env` files
- local ignored wallet paths
- documentation of required variable names

Not allowed:

- committed real `.env`
- committed real wallet file
- committed real private key
- committed real seed phrase
- committed real RPC API key
- committed guardian secret

## Git policy

Stage 4 work must preserve a clean secret boundary.

Before commits, run checks for common secret markers and inspect only filenames / marker presence where possible.

Secret scans should avoid printing values.

Preferred commands should list filenames or use redaction.

Stage 4 branches should be small and explicit.

Every live-capable stage must document whether it is:

- read-only
- dry-run
- live-send capable
- deployment capable

## Stage 4 layering rule

Stage 4 must proceed from least dangerous to most dangerous.

Recommended ordering:

1. Charter / boundary
2. redacted live config schema
3. read-only RPC connectivity
4. watcher runtime read-only observation
5. relayer dry-run / no-send path
6. guardian operation policy
7. transaction preflight boundary
8. controlled live testnet submission
9. deployment pipeline rehearsal
10. production readiness review

No later stage should be started until the previous boundary is documented and checked.

## Stage 4.0: charter boundary

Stage 4.0 is this document.

It defines the boundary before any live runtime implementation begins.

Stage 4.0 does not introduce:

- RPC calls
- wallet loading
- transaction submission
- signer usage
- SOL spending
- deployment scripts
- watcher loops
- relayer loops

Stage 4.0 is documentation only.

## Stage 4.1: redacted live config boundary

Expected purpose:

Define live config schema and redaction rules before connecting to any RPC.

Allowed:

- config field names
- required/optional field list
- validation rules
- redaction helpers
- `.env.example`
- tests proving secret values are rejected or redacted

Not allowed:

- real private keys
- real wallet JSON
- real RPC API keys
- live RPC calls
- transaction submission

## Stage 4.2: read-only RPC connectivity boundary

Expected purpose:

Introduce live RPC connectivity in read-only mode.

Allowed:

- RPC endpoint validation
- network identity check
- chain/genesis/program metadata check
- read-only account queries
- balance queries for public addresses
- redacted config use

Not allowed:

- wallet signer usage
- transaction submission
- minting
- importing
- deployment
- secret printing

## Stage 4.3: watcher runtime read-only boundary

Expected purpose:

Introduce watcher runtime as observation only.

Allowed:

- read-only chain polling
- event observation
- checkpoint writing
- health status
- retry logic
- evidence logs without secrets

Not allowed:

- signing
- submitting transactions
- changing chain state
- spending SOL

## Stage 4.4: relayer dry-run boundary

Expected purpose:

Introduce relayer logic without sending transactions.

Allowed:

- build unsigned transaction intent
- simulate or dry-run where safe
- validate message-to-instruction mapping
- compute expected accounts
- compute expected state changes
- produce dry-run evidence

Not allowed:

- live send
- wallet signing
- private key use
- SOL spending

## Stage 4.5: guardian operations policy boundary

Expected purpose:

Define guardian operations without exposing secrets.

Allowed:

- public guardian identity model
- quorum policy
- rotation policy
- incident policy
- signing ceremony checklist
- redaction rules
- no-secret evidence model

Not allowed:

- private keys in docs
- mnemonic phrases
- seed phrases
- raw signing material
- committed signer files

## Stage 4.6+: live-send boundaries

Any live-send boundary must be explicitly named and reviewed before execution.

A live-send stage must define:

- exact network
- exact program ID
- exact mint/token address
- exact payer public key
- expected SOL cost range
- failure mode
- rollback / incident response
- proof artifact
- what is signed
- what is not signed
- what is printed
- what is never printed

No live-send command should be run unless its boundary is documented first.

## Current public known X1 testnet context

Known public values may be referenced:

- RPC: https://rpc.testnet.x1.xyz
- Program id: 9tCJe4M1MJQtE1gDxNYNE75fNUGpSAKiX56rgUMR8984

Wallet paths and local signer files are not public evidence.

A wallet public key may be public.

A wallet secret file must never be printed.

## Stage 4 success definition

Stage 4 succeeds only if live operation is introduced gradually and safely.

Success requires:

- no secret leakage
- no accidental transaction submission
- no hidden wallet dependency
- explicit read-only vs dry-run vs live-send classification
- evidence for every live-capable boundary
- clear operator procedures
- review before any irreversible or value-spending action

## Current conclusion

Stage 4 is open only as a charter / boundary.

The next concrete work should be Stage 4.1: redacted live config boundary.

No live runtime code should be added before Stage 4.1 defines config and redaction rules.
