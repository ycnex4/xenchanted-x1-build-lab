# XXXL Phase 41F — Focused Crypto-Boundary Audit Scope

Date: 2026-07-02

## Status

Audit-only checkpoint.

No runtime code is introduced.

No verification logic is changed.

No proof/evidence/guardian/quorum/auth/replay/mutation/CPI/mint/live route is enabled.

## Current Baseline

Current main checkpoint:

`2efb5aa Merge XXXL phase 41F extraction safety flags acceptance record`

## Purpose

This audit closes Phase 41F as a focused crypto-boundary before Phase 41G begins.

The audit must confirm that the Phase 41F pipeline establishes only one thing:

- native Ed25519 signature verification was already performed by the SVM.

It must also confirm what Phase 41F does not establish:

- message payload correctness;
- proof acceptance;
- evidence acceptance;
- guardian validity;
- guardian set membership;
- quorum;
- authorization;
- replay protection;
- account mutation;
- CPI;
- mint;
- process instruction handler;
- live route.

## Phase 41F Pipeline Under Audit

Accepted components:

- Phase 41D: checked prior instruction loading;
- Phase 41E: Ed25519 instruction byte parsing;
- Phase 41F.1: checked signature/pubkey/message byte extraction;
- Phase 41F.2: Model A native Ed25519 verification establishment;
- Phase 41F.1 cumulative SAFETY_FLAGS alignment.

## Core Model A Claim

The accepted Model A claim is:

1. A prior native Ed25519 instruction exists in the same executed transaction.
2. The prior instruction is located, parsed, and extracted through accepted checked boundaries.
3. The current instruction is reached.
4. Native Ed25519 verification failure aborts the transaction before the current instruction.
5. Therefore, reaching the current instruction means the prior native Ed25519 instruction already verified successfully.

Short form:

- The SVM is the verifier.
- XXXL only establishes that the SVM verified.

## Load-Bearing Live-Wiring Precondition

The Model A claim is load-bearing only when the boundary is called from an actually executing runtime path.

Future live wiring must ensure:

- `loading_result` is derived from the real Instructions sysvar;
- `current_instruction_index` is derived from runtime state;
- fabricated or reconstructed pipeline structures are not accepted as load-bearing;
- live handler wiring receives a separate high-risk audit.

This audit must carry that precondition forward.

## Self-Reference Binding

The audit must confirm that Phase 41F preserves self-reference binding:

- signature instruction index == `u16::MAX`;
- public key instruction index == `u16::MAX`;
- message instruction index == `u16::MAX`.

This prevents cross-instruction references in Phase 41F.

Any future support for cross-instruction references remains out of scope and requires separate review.

## Checked Extraction Boundary

The audit must confirm that Phase 41F.1 extracts only checked byte slices:

- signature: `&[u8; 64]`;
- public key: `&[u8; 32]`;
- message: borrowed `&[u8]`.

The boundary must not copy attacker-sized message data into a new Vec.

## Program ID Re-Check

The audit must confirm Phase 41F.2 re-checks:

- `loaded_entry.instruction.program_id == ed25519_program::id()`.

This is a defense-in-depth check against stale or mismatched pipeline pairing.

## Status Attribution

The audit must confirm that Phase 41F statuses remain Model-A-attributed.

Accepted success status:

- `NativeEd25519VerificationEstablished`.

The success status is structural.

It means the SVM already verified the native Ed25519 instruction before the current instruction was reached.

Phase 41F must not use misleading Model A statuses such as:

- `Ed25519SignatureValid`;
- `Ed25519SignatureInvalid`.

A local invalid-signature path belongs only to a separately reviewed local-crypto Model B, not to Model A.


## SAFETY_FLAGS Taxonomy

The audit must confirm cumulative pipeline capability semantics.

A true capability flag means:

- the accepted pipeline has reached a phase that establishes this capability.

A false capability flag means:

- no accepted phase has yet established this capability.

Accepted Phase 41F result:

- `ed25519_signature_verification_performed: true`.

Still false:

- proof acceptance;
- evidence acceptance;
- guardian validity;
- quorum;
- authorization;
- replay writes;
- account mutation;
- CPI;
- mint;
- handler;
- live route.

## Message Payload Correctness Remains Downstream

Phase 41F establishes verification over message bytes.

It does not establish that the message is the correct gateway payload.

The following remain downstream Phase 41G work:

- message bytes match expected guardian payload hash;
- route binding;
- source chain binding;
- source burn event binding;
- recipient binding;
- amount binding;
- mint token binding;
- finality binding;
- expiration binding.

## Still Forbidden

The following remain forbidden:

- local cryptographic verification unless separately reviewed;
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

No blocker is removed, weakened, or reinterpreted.

Active blockers remain:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Expected Outcome

If accepted, this audit becomes the gate checkpoint that allows Phase 41G proof/evidence/payload-binding design or implementation to begin under a separate reviewed boundary.
