# XXXL Phase 36 Ed25519 Signature Evidence Boundary Spec

Status: Docs-only boundary specification.

## Purpose

Phase 36 defines the Ed25519 signature evidence boundary for the future Rust/SVM
runtime verifier.

This phase does not implement Ed25519 verification.

This phase does not parse the Instructions sysvar.

This phase does not inspect ed25519 program instructions.

This phase does not add runtime code.

This phase does not unlock runtime execution.

## Base Context

Previous Rust/SVM verifier phases:

- Phase 32: read-only runtime verifier scaffold.
- Phase 33: raw payload decoder.
- Phase 34: canonical payload hash/domain validation.
- Phase 35: guardian membership and quorum structural verifier.

Phase 35 deliberately verifies only structural quorum:

~~~text
unique known approvals >= threshold
~~~

That is not enough to authorize minting.

Phase 35 does not prove that any guardian signed the Phase 34 canonical payload
hash.

## Preserved Security Decision

The preserved security decision remains:

~~~text
TS layer = preflight / model / watcher-side decision
Runtime = independent verifier
No authorized=true -> execute
~~~

TypeScript authorization output is not runtime authority.

TypeScript parity results are not runtime authority.

Guardian structural quorum is not runtime authority.

Caller-provided signature claims are not runtime authority.

## Selected Ed25519 Evidence Model

The intended future SVM model is:

1. The transaction includes one or more SVM ed25519 program instructions.
2. Each ed25519 instruction verifies one guardian signature.
3. The runtime verifier inspects instruction evidence through the Instructions
   sysvar.
4. The runtime verifier confirms that each ed25519 instruction binds:
   - the expected guardian public key;
   - the expected signature bytes;
   - the exact expected message;
   - the expected Phase 34 canonical payload hash.
5. Only signatures whose evidence matches the expected guardian, message, and
   payload hash may be counted.
6. Only cryptographically verified and structurally valid guardian approvals may
   contribute toward quorum.

This phase only defines the boundary.

It does not implement the future instruction evidence parser.

## Signature Message Binding

The message to be signed must be bound to the Phase 34 canonical payload hash.

Phase 34 hash model:

~~~text
keccak256(keccak256("XXXL_GUARDIAN_PAYLOAD_HASH_V1") || payload_bytes)
~~~

The future Ed25519 evidence verifier must not accept a signature over:

- arbitrary user-provided bytes;
- a non-canonical payload;
- a payload hash not recomputed by Phase 34;
- a hash from a different domain;
- a hash from a different route;
- a hash from a different recipient;
- a hash from a different target mint;
- a hash from a different amount;
- a hash from a different guardian set id;
- a hash from a different message nonce;
- a hash from an expired message.

The signature evidence boundary must bind the verified signature to the exact
Phase 34 canonical payload hash used by the runtime verifier.

## Relationship To Phase 35

Phase 35 can answer:

~~~text
Are these guardian public keys known, unique, and enough to meet threshold?
~~~

Phase 36 defines the future boundary for answering:

~~~text
Did those guardians cryptographically sign the exact Phase 34 canonical payload hash?
~~~

Both are required before a future runtime verifier can treat guardian approval as
valid.

A structurally valid quorum without Ed25519 evidence is insufficient.

Ed25519 evidence without structural guardian membership and quorum is also
insufficient.

## Future Runtime Acceptance Rule

A future runtime verifier may count a guardian approval only if all of the
following are true:

- raw payload decoding succeeds;
- canonical payload hash validation succeeds;
- guardian public key is known in the selected guardian set;
- guardian set id matches;
- approval is not duplicated;
- Ed25519 evidence proves the guardian signed the expected Phase 34 payload hash;
- the guardian evidence is not reused in a way that violates replay rules;
- quorum threshold is reached using unique cryptographically verified guardians.

Phase 36 does not implement this rule.

Phase 36 only documents the required boundary.

## Explicit Non-Goals

Phase 36 does not implement Ed25519 verification.

Phase 36 does not parse ed25519 instruction data.

Phase 36 does not parse the Instructions sysvar.

Phase 36 does not add an instruction handler.

Phase 36 does not parse runtime accounts.

Phase 36 does not read route accounts.

Phase 36 does not read target mint accounts.

Phase 36 does not implement source proof verification.

Phase 36 does not implement route config verification.

Phase 36 does not implement target mint account legitimacy verification.

Phase 36 does not implement amount cap enforcement.

Phase 36 does not implement replay storage.

Phase 36 does not implement replay checks.

Phase 36 does not implement replay writes.

Phase 36 does not enable live route execution.

Phase 36 does not enable SPL CPI.

Phase 36 does not enable `invoke_signed`.

Phase 36 does not enable SPL Token `mint_to`.

Phase 36 does not add mint execution.

Phase 36 does not mutate runtime/account state.

Phase 36 does not mark processed events.

Phase 36 does not select a production Program ID.

Phase 36 does not regenerate production PDA fixtures.

Phase 36 does not remove deployment blockers.

Phase 36 does not claim production readiness.

Phase 36 does not claim final immutability while upgrade authority exists.

Phase 36 does not change Cargo manifests.

Phase 36 does not change package manifests.

Phase 36 does not build SBF artifacts.

Phase 36 does not touch `target/deploy`.

Phase 36 does not read or modify keypair files.

Phase 36 does not read or modify `.env`.

Phase 36 does not inspect `.local-keys`.

Phase 36 does not run deploy commands.

Phase 36 does not run network commands.

Phase 36 does not spend SOL.

## Preserved Blockers

The following blockers remain active:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker is removed, weakened, renamed, or satisfied by Phase 36.

## Recommended Next Stage

Recommended next stage:

- Phase 37: Rust/SVM Ed25519 instruction evidence layout model.

That next stage should still be read-only and should not unlock runtime
execution.

It should define and test the expected instruction evidence shape before any
runtime instruction parser or account parser is added.
