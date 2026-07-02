# XXXL Phase 41F.2 — Ed25519 Signature Verification Boundary Plan

Date: 2026-07-02

## Status

Planning document only.

No runtime code is introduced in Phase 41F.2.

## Parent Gate

Phase 41F.1 external acceptance:

`f5c9c7f Merge XXXL phase 41F checked extraction acceptance record`

## Purpose

Phase 41F.2 plans the Ed25519 signature verification boundary after checked byte extraction.

This is the first phase that discusses establishing signature validity.

However, Phase 41F.2 itself is docs-only and does not perform verification.

## Critical Separation

Signature validity must remain separate from:

- proof acceptance;
- evidence acceptance;
- guardian validity;
- guardian set membership;
- quorum;
- authorization;
- replay writes;
- processed event marking;
- account mutation;
- CPI;
- `invoke_signed`;
- SPL Token `mint_to`;
- process instruction handler;
- live route unlock.

## Verification Models

Phase 41F.2 must explicitly select and document the verification model before code.

### Model A — Native Ed25519 Instruction Verification

Model A is the preferred default unless external review says otherwise.

Under Model A, signature verification is provided by a prior native Ed25519 instruction in the same executed transaction.

Soundness argument:

1. The prior native Ed25519 instruction is present in the executed transaction.
2. Phase 41D/41E/41F.1 locate, parse, and extract the self-contained bytes from that prior instruction.
3. The current instruction was reached.
4. Native Ed25519 verification failure would abort the transaction before the current instruction.
5. Therefore, reaching the current instruction means the prior native Ed25519 instruction already verified successfully.

Preferred wording:

- "the native Ed25519 instruction already verified the signature because the transaction reached the current instruction."

Avoid weaker wording:

- "would have verified if the transaction reached the current instruction."

### Model B — Local Cryptographic Verification

Model B means program-side local Ed25519 verification.

Model B is deferred unless separately reviewed.

Model B requires separate review for:

- crypto crate choice;
- SVM/BPF compatibility;
- deterministic behavior;
- compute cost;
- heap behavior;
- panic paths;
- side effects;
- no proof/evidence/auth drift.

Model B must not be introduced accidentally.

## Required Inputs

Future Phase 41F.2 code may consume only the accepted Phase 41F.1 extraction output.

Required input:

- checked signature bytes as `&[u8; 64]`;
- checked public key bytes as `&[u8; 32]`;
- checked message bytes as borrowed `&[u8]`;
- original parsed ranges;
- matched prior instruction index.

The message must remain borrowed.

No attacker-sized message `Vec` copy is allowed.

## Self-Reference Binding

Model A requires the Phase 41E self-reference invariant to remain preserved:

- signature instruction index == `u16::MAX`;
- public key instruction index == `u16::MAX`;
- message instruction index == `u16::MAX`.

This binds the bytes verified by the native Ed25519 instruction to the parsed and extracted signature/public-key/message ranges.

Future support for non-self references must remain a separate reviewed loading and binding gate.

## Status Model Must Be Model-Attributed

Future status names must distinguish Model A and Model B behavior.

Under Model A:

- invalid signature is normally unreachable at runtime;
- if native Ed25519 verification failed, the transaction would abort before current instruction;
- status should represent whether native verification is structurally established, not local invalid-signature checking.

Under Model B:

- invalid signature can be directly returned by local crypto verification.

Therefore, future statuses must avoid misleading dead paths.

## SAFETY_FLAGS Semantics To Resolve

Before any signature-verification flag is flipped, the project must clarify whether `SAFETY_FLAGS` are:

- cumulative pipeline capability flags; or
- local module capability flags.

Demon noted that Phase 41E.1 used cumulative-style flags while Phase 41F.1 used local-style flags.

This is not a security issue because all trust/execution flags remained false.

However, Phase 41F.2 must not flip `ed25519_signature_verification_performed` until the flag convention is clarified.

## Program ID Defense-In-Depth

Future Phase 41F.2 code should add or require a defense-in-depth re-check:

- `loaded_entry.instruction.program_id == ed25519_program::id()`.

This keeps Phase 41F consistent with Phase 41E and reduces risk from stale or mismatched pipeline pairing.

## Audit Checkpoint

Phase 41F.2 should trigger a focused crypto-boundary audit after implementation and before any later proof/evidence gate.

That audit must check:

- Model A soundness;
- self-reference binding;
- checked extraction correctness;
- program-id re-check;
- status model attribution;
- signature-validity separation from proof/evidence/guardian/quorum/auth;
- no replay/mutation/CPI/mint/live drift.

## Still Forbidden In Phase 41F.2 Plan

Phase 41F.2 plan does not permit code.

Still forbidden:

- local cryptographic verification code;
- native Ed25519 verification establishment code;
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

## Review Questions

External review should confirm:

1. Is 41F.2 the correct next docs-only plan after 41F.1?
2. Is Model A soundness stated correctly?
3. Is Model B correctly deferred?
4. Is self-reference binding preserved?
5. Is status attribution by verification model clear?
6. Must SAFETY_FLAGS semantics be resolved before code?
7. Is program-id re-check required before or inside 41F.2 implementation?
8. Is the focused crypto-boundary audit checkpoint placed correctly?
9. Are trust-sensitive gates preserved?
10. Can Phase 41F.2 implementation begin after acceptance?

## Next Gate

If accepted, the next step is Phase 41F.2 implementation under separate review.

That implementation must still not accept proof, evidence, guardian validity, quorum, authorization, replay, mutation, CPI, mint, handler, or live route.
