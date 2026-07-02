# XXXL Phase 41F.2 — Ed25519 Signature Verification Boundary

Date: 2026-07-02

## Status

Implemented as a narrow Model A signature verification boundary.

## Parent Gate

Phase 41F.2 plan external acceptance:

`326bfb9 Merge XXXL phase 41F signature verification plan acceptance record`

## Scope

Phase 41F.2 implementation establishes Model A native Ed25519 verification structurally.

It does not perform local cryptographic verification.

It does not accept proof, evidence, guardian validity, quorum, authorization, replay, mutation, CPI, mint, handler, or live route.

## Model A Semantics

Model A means:

- a prior native Ed25519 instruction is present in the executed transaction;
- the prior instruction was located, parsed, and checked-extracted by earlier phases;
- the current instruction was reached;
- native Ed25519 verification failure would abort the transaction before the current instruction;
- therefore reaching the current instruction means the prior native Ed25519 instruction already verified successfully.

## SAFETY_FLAGS Convention

Phase 41F.2 resolves the flag convention as cumulative pipeline capability flags.

A true capability flag means:

- the pipeline has reached a phase that establishes this capability.

A false capability flag means:

- no accepted phase in the pipeline has yet established this capability.

Therefore Phase 41F.2 sets:

- `ed25519_signature_verification_performed: true`.

But all downstream trust/execution flags remain false.

## Implemented Checks

Phase 41F.2 checks:

- Phase 41F.1 extracted slices are present;
- matched instruction index exists;
- extraction matched instruction index agrees with parsing matched index;
- parsed offsets are present;
- matched loaded prior instruction exists;
- loaded instruction program id is `ed25519_program::id()`;
- loaded entry remains runtime-data-only;
- loaded data length matches parse and extraction results;
- self-reference binding is preserved;
- extracted ranges match parsed ranges.

## Message Payload Correctness Deferred

Phase 41F.2 does not check whether the signed message is the expected gateway payload.

The following remain downstream proof/evidence gates:

- message bytes match expected guardian payload hash;
- route binding;
- target mint binding;
- recipient binding;
- amount binding;
- finality binding;
- expiration binding.

## Status Model

Phase 41F.2 implements Model A statuses.

Accepted success status:

- `NativeEd25519VerificationEstablished`.

Failure statuses are structural, not local crypto invalid-signature results.

Phase 41F.2 does not introduce `Ed25519SignatureInvalid`, because invalid native signature normally aborts before the current instruction.

## Tests

Tests cover:

- native Ed25519 verification establishment;
- checked extraction missing;
- matched index mismatch;
- missing parsed offsets;
- matched instruction unavailable;
- wrong program id;
- non-runtime-data entry;
- instruction data length mismatch;
- missing self-reference binding;
- extracted ranges mismatching parsed ranges;
- report flags preserving downstream trust separation.

## Still Forbidden

The following remain forbidden:

- local cryptographic verification;
- proof acceptance;
- verification evidence acceptance;
- guardian validity acceptance;
- guardian set membership acceptance;
- quorum counting;
- authorization;
- replay writes;
- processed event marking;
- account mutation;
- CPI;
- `invoke_signed`;
- SPL Token `mint_to`;
- process instruction handler;
- live route unlock.

## Active Blockers Remain

No blocker is removed, weakened, or reinterpreted by Phase 41F.2.

Active blockers remain:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Next Gate

After external acceptance of Phase 41F.2 implementation, a focused crypto-boundary audit is required before any proof/evidence gate begins.
