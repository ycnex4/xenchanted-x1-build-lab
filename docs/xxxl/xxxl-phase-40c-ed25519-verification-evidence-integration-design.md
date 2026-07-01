# XXXL Phase 40C Ed25519 Verification Evidence Integration Design

Status: Docs-only integration design.

## Purpose

Phase 40C defines the future design for integrating SVM Ed25519 verification
evidence into the XXXL runtime verifier.

This phase does not add Rust code.

This phase does not parse the raw Instructions sysvar.

This phase does not call `load_instruction`.

This phase does not parse `AccountInfo`.

This phase does not verify Ed25519 signatures.

This phase does not accept cryptographic proof in code.

This phase does not count quorum.

This phase does not authorize minting.

This phase does not enable execution.

## Base Context

Previous phases:

- Phase 37 added the Ed25519 instruction evidence layout model.
- Phase 38 added the Ed25519 instruction data parser.
- Phase 39 added the prepared-entry Instructions sysvar evidence scanner.
- Phase 40A documented the verification evidence boundary.
- Phase 40B added a tiny Rust model proving located/parsed evidence is still
  not proof, quorum, authorization, or execution.

Phase 40C now documents how future code should move from candidate evidence to
runtime-readable verification evidence without collapsing boundaries.

## Core Security Rule

The preserved security decision remains:

~~~text
TS layer = preflight / model / watcher-side decision
Runtime = independent verifier
No authorized=true -> execute
~~~

The runtime must not trust any caller-provided boolean such as:

- `signature_verified`
- `guardian_verified`
- `quorum_reached`
- `authorized`
- `mint_allowed`
- `execute`

The runtime must derive every accepted fact from runtime-readable evidence.

## Boundary Rule

Future code must preserve this separation:

~~~text
located candidate evidence
  != parsed evidence
  != verification evidence
  != quorum
  != authorization
  != execution
~~~

Phase 40C is about the design boundary between parsed candidate evidence and
future verification evidence only.

## Future Transaction-Level Model

A future transaction-level verification model should require:

1. One or more SVM Ed25519 verification program instructions.
2. The Ed25519 verification instruction must execute before the XXXL runtime
   instruction that consumes the evidence.
3. The XXXL runtime instruction must read the actual Instructions sysvar.
4. The runtime must locate the relevant prior Ed25519 instruction.
5. The runtime must parse the Ed25519 instruction data using the already defined
   Phase 37 and Phase 38 constraints.
6. The runtime must compare the signed message bytes to the runtime-recomputed
   Phase 34 canonical payload hash.
7. The runtime must compare the public key bytes to a guardian public key in the
   active guardian set.
8. The runtime must treat this only as verified guardian evidence, not as quorum
   and not as mint authorization.

The Ed25519 verification instruction being present is not enough.

The Ed25519 instruction data being parseable is not enough.

The guardian public key matching is not enough.

The signed message matching the payload hash is not enough.

All required runtime bindings must hold at the same time.

## Future Evidence Binding Requirements

A future verification evidence implementation must bind:

- current XXXL instruction identity
- prior Ed25519 instruction identity
- Ed25519 program id
- Ed25519 instruction data bytes
- signature bytes
- guardian public key bytes
- signed message bytes
- Phase 34 runtime-recomputed payload hash
- route id
- source chain id
- source token
- source burn transaction hash
- canonical event key
- target mint
- recipient
- amount
- guardian set id
- expiration or finality boundary
- deterministic failure reason

No single binding is sufficient alone.

## Instruction Ordering Requirement

Future verification evidence should only accept an Ed25519 instruction that has
already executed before the XXXL instruction.

A future implementation should reject:

- missing Ed25519 instruction
- Ed25519 instruction after the XXXL instruction
- wrong program id
- malformed Ed25519 instruction data
- unsupported offset layout
- wrong guardian public key
- wrong message hash
- wrong guardian set id
- duplicated evidence where uniqueness is required
- ambiguous evidence
- expired evidence
- evidence for another route
- evidence for another target mint
- evidence for another recipient
- evidence for another amount

## Same-Instruction Ed25519 Offset Policy

The current Phase 37 and Phase 38 model is intentionally narrow:

- one signature per Ed25519 instruction
- signature/public-key/message bytes live in the same Ed25519 instruction data
- message length is exactly 32 bytes
- the message is the Phase 34 canonical payload hash
- instruction-index offsets use the same-instruction sentinel

A future phase may keep this narrow policy for safety.

If a future phase expands the policy, that expansion must be explicit, separately
documented, and separately tested.

## Verification Evidence Is Still Not Quorum

A future verification evidence result may say:

- this guardian signature evidence is accepted
- the accepted evidence binds to this payload hash
- the accepted evidence binds to this guardian public key
- the accepted evidence came from a prior Ed25519 verification instruction

It must not say:

- quorum reached
- mint authorized
- replay consumed
- recipient credited
- SPL mint executed
- live route unlocked

Quorum must remain a separate composition phase.

Authorization must remain a later composition phase.

Execution must remain separately gated.

## Deterministic Error Surface

Future code should expose deterministic errors for:

- missing Instructions sysvar
- unreadable Instructions sysvar
- missing current instruction identity
- Ed25519 instruction not found
- Ed25519 instruction appears after current instruction
- wrong Ed25519 program id
- malformed Ed25519 instruction data
- unsupported Ed25519 offset layout
- public key mismatch
- message hash mismatch
- guardian set mismatch
- duplicated guardian evidence
- ambiguous candidate evidence
- expired evidence
- evidence for wrong route
- evidence for wrong target mint
- evidence for wrong recipient
- evidence for wrong amount

No failure should silently become authorization.

## Explicit Non-Goals

Phase 40C does not add Rust code.

Phase 40C does not add a new verifier module.

Phase 40C does not modify `programs/xxxl-svm/src/lib.rs`.

Phase 40C does not modify `programs/xxxl-svm/src/verifier/mod.rs`.

Phase 40C does not parse raw Instructions sysvar account data.

Phase 40C does not parse `AccountInfo`.

Phase 40C does not call `load_instruction`.

Phase 40C does not verify Ed25519 signatures.

Phase 40C does not accept cryptographic signature proof in code.

Phase 40C does not count quorum.

Phase 40C does not authorize minting.

Phase 40C does not add `process_instruction`.

Phase 40C does not add a runtime instruction handler.

Phase 40C does not add account parsing.

Phase 40C does not add CPI.

Phase 40C does not enable `invoke_signed`.

Phase 40C does not enable SPL Token `mint_to`.

Phase 40C does not add replay writes.

Phase 40C does not mark processed events.

Phase 40C does not mutate runtime/account state.

Phase 40C does not unlock live route execution.

Phase 40C does not remove deployment blockers.

Phase 40C does not select a production Program ID.

Phase 40C does not claim production readiness.

Phase 40C does not claim final immutability while upgrade authority exists.

Phase 40C does not build SBF artifacts.

Phase 40C does not touch `target/deploy`.

Phase 40C does not read or modify keypair files.

Phase 40C does not read or modify `.env`.

Phase 40C does not inspect `.local-keys`.

Phase 40C does not run deploy commands.

Phase 40C does not run network commands.

Phase 40C does not spend SOL.

## Preserved Blockers

The following blockers remain active:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker is removed, weakened, renamed, or satisfied by Phase 40C.

## Recommended Next Stage

Phase 40D should be a tiny read-only Rust design surface for future Ed25519
verification evidence integration.

It should still avoid quorum authorization, handler or account parsing, CPI,
mint execution, replay writes, and runtime unlock.
