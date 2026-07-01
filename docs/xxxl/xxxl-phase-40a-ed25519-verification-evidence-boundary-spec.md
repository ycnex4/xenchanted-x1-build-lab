# XXXL Phase 40A Ed25519 Verification Evidence Integration Boundary Spec

Status: Docs-only boundary specification.

## Purpose

Phase 40A defines the boundary between located Ed25519 candidate evidence and
future Ed25519 verification evidence.

Phase 39 can locate an Ed25519 instruction candidate and parse its instruction
data through the Phase 38 parser.

That is not enough to prove the signature is cryptographically valid.

Phase 40A documents what a future Rust/SVM verifier phase must prove before any
guardian signature can be treated as verified evidence.

This phase adds no Rust code.

This phase adds no runtime behavior.

This phase does not verify Ed25519 signatures.

This phase does not count quorum.

This phase does not authorize minting.

## Base Context

Previous Rust/SVM verifier phases:

- Phase 32: read-only runtime verifier scaffold.
- Phase 33: raw payload decoder.
- Phase 34: canonical payload hash/domain validation.
- Phase 35: guardian membership and quorum structural verifier.
- Phase 36: Ed25519 signature evidence boundary spec.
- Phase 37: Ed25519 instruction evidence layout model.
- Phase 38: Ed25519 instruction data parser.
- Phase 39: prepared-entry Instructions sysvar evidence scanner.

Phase 39 returns located and parsed candidate evidence only.

Phase 40A defines the next boundary: future verification evidence integration.

## Preserved Security Decision

The preserved security decision remains:

~~~text
TS layer = preflight / model / watcher-side decision
Runtime = independent verifier
No authorized=true -> execute
~~~

TypeScript authorization output is not runtime authority.

TypeScript parity results are not runtime authority.

Located Ed25519 instruction candidates are not runtime authority.

Parsed signature bytes are not runtime authority.

Public-key/message-hash byte matches are not runtime authority.

Future verification evidence alone is still not mint authorization unless it is
combined with all other required runtime checks.

## Terminology

### Located candidate evidence

Located candidate evidence means the runtime has found a candidate Ed25519
instruction entry and parsed its instruction data.

This may include:

- instruction index
- Ed25519 program id match
- signature bytes
- guardian public key bytes
- signed message bytes
- expected guardian public key match
- expected Phase 34 payload hash match

Located candidate evidence is not cryptographic proof.

### Parsed evidence

Parsed evidence means Phase 38 successfully extracted bytes from Ed25519
instruction data and compared public key/message values.

Parsed evidence is not cryptographic proof.

### Verification evidence

Verification evidence means future runtime-readable evidence that the SVM
Ed25519 verification path accepted the signature over the exact expected
message bytes for the exact expected guardian public key.

Phase 40A does not implement this.

A future phase must define how this evidence is read, bounded, and tied to the
same instruction candidate that Phase 39 located.

### Authorization

Authorization is not any single proof.

Authorization can only exist after the runtime composes all required checks:

- raw payload decoding
- canonical Phase 34 hash recomputation
- source proof identity checks
- route binding checks
- target mint legitimacy checks
- amount controls
- replay checks
- expiration/finality checks
- guardian signature verification evidence
- quorum counting over verified guardian evidence
- deployment/live-route gates

Phase 40A does not create authorization.

## Future Verification Evidence Requirements

A future verification evidence integration phase must bind all of the following:

- the expected guardian public key
- the expected Phase 34 canonical payload hash
- the exact Ed25519 signature bytes
- the Ed25519 program id
- the instruction index or equivalent instruction identity
- the candidate instruction data parsed by Phase 38
- the candidate located by Phase 39
- deterministic failure reasons for malformed or missing evidence

The future phase must not accept caller-provided claims such as:

- `signature_verified = true`
- `guardian_verified = true`
- `authorized = true`
- `quorum_reached = true`

The future phase must derive or validate all such facts from runtime-readable
evidence.

## Non-Goals

Phase 40A does not add Rust code.

Phase 40A does not add a new verifier module.

Phase 40A does not modify `programs/xxxl-svm/src/lib.rs`.

Phase 40A does not modify `programs/xxxl-svm/src/verifier/mod.rs`.

Phase 40A does not parse raw Instructions sysvar account data.

Phase 40A does not parse `AccountInfo`.

Phase 40A does not call `load_instruction`.

Phase 40A does not verify Ed25519 signatures.

Phase 40A does not accept cryptographic signature proof.

Phase 40A does not count quorum.

Phase 40A does not authorize minting.

Phase 40A does not add `process_instruction`.

Phase 40A does not add a runtime instruction handler.

Phase 40A does not add account parsing.

Phase 40A does not add CPI.

Phase 40A does not enable `invoke_signed`.

Phase 40A does not enable SPL Token `mint_to`.

Phase 40A does not add replay writes.

Phase 40A does not mark processed events.

Phase 40A does not mutate runtime/account state.

Phase 40A does not unlock live route execution.

Phase 40A does not remove deployment blockers.

Phase 40A does not select a production Program ID.

Phase 40A does not claim production readiness.

Phase 40A does not claim final immutability while upgrade authority exists.

Phase 40A does not build SBF artifacts.

Phase 40A does not touch `target/deploy`.

Phase 40A does not read or modify keypair files.

Phase 40A does not read or modify `.env`.

Phase 40A does not inspect `.local-keys`.

Phase 40A does not run deploy commands.

Phase 40A does not run network commands.

Phase 40A does not spend SOL.

## Boundary Rule For Future Code

Future code must preserve this separation:

~~~text
located candidate evidence
  != parsed evidence
  != verification evidence
  != quorum
  != authorization
  != execution
~~~

A future phase may produce verification evidence.

A later phase may count quorum over verified guardian evidence.

A still later phase may compose authorization.

Execution must remain gated separately.

## Preserved Blockers

The following blockers remain active:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker is removed, weakened, renamed, or satisfied by Phase 40A.

## Recommended Next Stage

Phase 40B should add a read-only Rust/SVM verification evidence model.

That future phase should still avoid quorum authorization, handler or account
parsing, CPI, mint execution, replay writes, and runtime unlock.
