# XXXL Phase 41F.2 Ed25519 Signature Verification Boundary Plan — External Acceptance

Date: 2026-07-02

Current main under review:

`01a3f3a Merge XXXL phase 41F Ed25519 signature verification plan`

## Scope Accepted

Phase 41F.2 is accepted as a docs-only plan.

No runtime code was introduced.

Accepted scope:

- plan the Ed25519 signature verification boundary after Phase 41F.1 checked extraction;
- preserve Model A native Ed25519 verification as the preferred default;
- defer Model B local cryptographic verification unless separately reviewed;
- preserve self-reference binding;
- require model-attributed statuses;
- require SAFETY_FLAGS semantics resolution before flipping any verification flag;
- require program-id defense-in-depth re-check;
- place focused crypto-boundary audit after implementation and before proof/evidence gates;
- keep signature validity separate from proof/evidence/guardian/quorum/auth/replay/mutation/CPI/mint/live route.

## Theo Verdict

Verdict: ACCEPT

Required fixes: none.

Blocking risks: none.

Theo accepted:

- Phase 41F.2 is the correct next docs-only plan after Phase 41F.1;
- Model A soundness is correct;
- Model B is correctly deferred;
- self-reference binding is preserved;
- status attribution by model is clear;
- SAFETY_FLAGS semantics must be resolved before any flag flip;
- program-id re-check requirement is correct;
- focused crypto-boundary audit checkpoint is placed correctly;
- no trust-sensitive drift is present;
- Phase 41F.2 implementation may begin after acceptance.

Theo clarified:

- SAFETY_FLAGS semantics are blocking for any verification flag flip, but not blocking for implementation start;
- program-id re-check is a must-require, not merely a should;
- Model A should use statuses such as `NativeEd25519VerificationEstablished` and `NativeEd25519VerificationNotEstablished`;
- `Ed25519SignatureInvalid` should not be used under Model A because invalid signature is normally unreachable after native Ed25519 abort-before semantics.

## Audit Demon Verdict

Verdict: ACCEPT

Required fixes: none.

Scope violations: no.

Model A soundness acceptable: yes.

Model B deferral acceptable: yes.

Self-reference binding acceptable: yes.

Status model attribution acceptable: yes.

SAFETY_FLAGS resolution requirement acceptable: yes.

Program-id re-check requirement acceptable: yes.

Audit checkpoint acceptable: yes.

Trust-sensitive boundary drift: no.

Next phase allowed: yes.

Demon accepted:

- Phase 41F.2 is docs-only;
- Model A soundness is correctly stated;
- the plan fixes the weaker wording from earlier phases;
- Model B is correctly deferred;
- self-reference binding is preserved;
- statuses must be attributed to Model A or Model B;
- SAFETY_FLAGS semantics must be resolved before any verification flag flip;
- program-id defense-in-depth re-check is required;
- focused crypto-boundary audit is correctly placed after implementation and before proof/evidence gates;
- all active blockers remain preserved.

## Demon Note — Message Payload Correctness Remains Downstream

Demon noted one non-blocking clarification to carry forward.

Signature validity over a message does not mean that the message is the correct gateway or guardian payload.

Model A can establish that a signature is valid over the Ed25519 instruction message bytes.

But without message-to-expected-payload-hash binding, the signature can be cryptographically valid and still semantically irrelevant.

Therefore, future work must keep the following as a separate downstream gate:

- message bytes match expected guardian payload hash;
- route binding is correct;
- target mint binding is correct;
- recipient binding is correct;
- amount binding is correct;
- finality binding is correct;
- expiration binding is correct.

This belongs to the later proof/evidence/payload-binding phase and must not be collapsed into Phase 41F.2.

## Accepted Model A Soundness Argument

Model A rests on the following accepted argument:

1. A prior native Ed25519 instruction is present in the executed transaction.
2. Phase 41D/41E/41F.1 locate, parse, and extract self-contained bytes from that prior instruction.
3. The current instruction was reached.
4. Native Ed25519 verification failure would abort the transaction before the current instruction.
5. Therefore, reaching the current instruction means the prior native Ed25519 instruction already verified successfully.

Accepted wording:

- “the native Ed25519 instruction already verified the signature because the transaction reached the current instruction.”

Rejected weaker wording:

- “would have verified if the transaction reached the current instruction.”

## Accepted Self-Reference Binding

Model A must continue to require:

- signature instruction index == `u16::MAX`;
- public key instruction index == `u16::MAX`;
- message instruction index == `u16::MAX`.

This binds the bytes verified by the native Ed25519 instruction to the parsed and extracted signature/public-key/message ranges.

Any future support for non-self references must remain a separate reviewed loading and binding gate.

## Accepted Status Model Direction

Future statuses must be attributed to the selected verification model.

Under Model A:

- `NativeEd25519VerificationEstablished` can mean the native verification was structurally established;
- `NativeEd25519VerificationNotEstablished` can mean the structural conditions were not met;
- `Ed25519SignatureInvalid` should not be used because invalid signature normally aborts before the current instruction.

Under Model B:

- `Ed25519SignatureValid` can represent a direct crypto result;
- `Ed25519SignatureInvalid` can represent a direct local crypto failure.

For both models:

- `UnsupportedVerificationModel` may represent an unselected or unavailable model.

## SAFETY_FLAGS Requirement

Before any future implementation flips `ed25519_signature_verification_performed`, the project must clarify whether `SAFETY_FLAGS` are:

- cumulative pipeline capability flags; or
- local module capability flags.

This requirement closes the Phase 41F.1 non-blocking note.

The convention must be documented before any verification capability flag is flipped.

## Program ID Re-Check Requirement

Future Phase 41F.2 implementation must add or require defense-in-depth program-id re-check:

- `loaded_entry.instruction.program_id == ed25519_program::id()`.

This reduces risk from stale or mismatched pipeline pairing and keeps Phase 41F consistent with Phase 41E.

## Focused Crypto-Boundary Audit

After Phase 41F.2 implementation and before any proof/evidence gate, the project must run a focused crypto-boundary audit.

Audit checklist:

- Model A soundness;
- native Ed25519 abort-before semantics;
- self-reference binding;
- checked extraction correctness;
- program-id re-check;
- status attribution by verification model;
- SAFETY_FLAGS convention;
- message-payload correctness remains downstream;
- signature validity separation from proof/evidence/guardian/quorum/auth;
- no replay/mutation/CPI/mint/live drift.

## Still Forbidden

The following remain forbidden after Phase 41F.2 plan acceptance:

- local cryptographic verification code unless separately reviewed;
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

Phase 41F.2 plan is externally accepted.

Next allowed step:

- Phase 41F.2 implementation under a separate reviewed code boundary.

Implementation prerequisites:

- resolve and document SAFETY_FLAGS semantics before flipping any verification flag;
- add program-id defense-in-depth re-check;
- use model-attributed statuses;
- preserve self-reference binding;
- keep message-payload correctness as a downstream proof/evidence gate;
- keep signature validity separate from proof/evidence/guardian/quorum/auth.

After implementation, a focused crypto-boundary audit is required before any proof/evidence gate begins.
