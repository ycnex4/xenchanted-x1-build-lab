# XXXL Phase 41F Focused Crypto-Boundary Audit — External Acceptance

Date: 2026-07-02

Current main under review:

`81346e2 Merge XXXL phase 41F focused audit status attribution note`

Audit package baseline:

`8933534 Merge XXXL phase 41F focused crypto boundary audit`

Status attribution addendum:

`81346e2 Merge XXXL phase 41F focused audit status attribution note`

## Scope Accepted

Phase 41F focused crypto-boundary audit is externally accepted.

Accepted scope:

- audit-only checkpoint;
- no runtime code changes;
- no verification logic changes;
- no new proof/evidence/guardian/quorum/auth/replay/mutation/CPI/mint/live behavior;
- Phase 41F closes only the native Ed25519 verification boundary;
- Phase 41G may begin after this acceptance under a separate reviewed boundary.

## Theo Verdict

Verdict: ACCEPT

Required fixes: none.

Blocking risks: none.

Theo accepted:

- the audit package is complete enough to close Phase 41F;
- Model A soundness is framed correctly;
- SVM-as-verifier framing is correct;
- live-wiring precondition is captured strongly enough;
- self-reference binding is preserved;
- checked extraction is bounded and borrowed;
- program-id re-check is present;
- status attribution is clear;
- SAFETY_FLAGS are cumulative and consistent;
- message payload correctness is deferred to Phase 41G;
- no proof/evidence/guardian/quorum/auth drift exists;
- no replay/mutation/CPI/mint/live drift exists;
- active blockers are preserved;
- Phase 41G is allowed after acceptance.

Theo summary:

Phase 41F establishes that native Ed25519 signature verification was already performed by the SVM.

Phase 41F does not establish:

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
- handler;
- live route.

## Audit Demon Verdict

Verdict: ACCEPT

Required fixes: none.

Scope violations: no.

Model A soundness acceptable: yes.

SVM-as-verifier framing acceptable: yes.

Live-wiring precondition captured: yes.

Self-reference binding acceptable: yes.

Checked extraction acceptable: yes.

Program-id re-check acceptable: yes.

Status attribution acceptable: yes.

SAFETY_FLAGS taxonomy acceptable: yes.

Message-payload deferral acceptable: yes.

Proof/evidence/guardian/quorum/auth drift: no.

Replay/mutation/CPI/mint/live drift: no.

Active blockers preserved: yes.

Phase 41G allowed after acceptance: yes.

## Demon Non-Blocking Note Resolved

Demon noted that the original audit scope did not explicitly restate status attribution.

A docs-only addendum was merged before this acceptance record.

Clarification now included:

- Phase 41F statuses are Model-A-attributed;
- success status is `NativeEd25519VerificationEstablished`;
- success is structural;
- Phase 41F must not use misleading Model A statuses such as `Ed25519SignatureValid` or `Ed25519SignatureInvalid`;
- local invalid-signature status belongs only to a separately reviewed Model B.

## Accepted Phase 41F Boundary

Phase 41F establishes only:

- native Ed25519 verification was already performed by the SVM.

Short form:

- the SVM is the verifier;
- XXXL only establishes that SVM verified.

## Accepted Model A Claim

The accepted Model A claim is:

1. A prior native Ed25519 instruction exists in the same executed transaction.
2. The prior instruction is located, parsed, and extracted through accepted checked boundaries.
3. The current instruction is reached.
4. Native Ed25519 verification failure aborts the transaction before the current instruction.
5. Therefore, reaching the current instruction means the prior native Ed25519 instruction already verified successfully.

## Carry-Forward Live-Wiring Precondition

Model A is load-bearing only when Phase 41F.2 is called from an actually executing runtime path.

Future live wiring must ensure:

- `loading_result` is derived from the real Instructions sysvar;
- `current_instruction_index` is derived from runtime state;
- fabricated or reconstructed pipeline structures are not accepted as load-bearing;
- live handler wiring receives a separate high-risk audit.

## Accepted Self-Reference Binding

Phase 41F preserves:

- signature instruction index == `u16::MAX`;
- public key instruction index == `u16::MAX`;
- message instruction index == `u16::MAX`.

Cross-instruction references remain out of scope and require separate review.

## Accepted Checked Extraction

Phase 41F.1 extracts:

- signature: `&[u8; 64]`;
- public key: `&[u8; 32]`;
- message: borrowed `&[u8]`.

No attacker-sized message Vec copy is introduced.

## Accepted Program-ID Re-Check

Phase 41F.2 re-checks:

- `loaded_entry.instruction.program_id == ed25519_program::id()`.

This remains a defense-in-depth guard against stale or mismatched pipeline pairing.

## Accepted SAFETY_FLAGS Taxonomy

SAFETY_FLAGS are cumulative pipeline capability flags.

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

## Message Payload Correctness Remains Phase 41G

Phase 41F verifies over message bytes.

It does not establish that the message is the correct gateway payload.

Phase 41G must separately address:

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

The following remain forbidden after this audit acceptance:

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

## Next Gate

Phase 41F focused crypto-boundary audit is externally accepted.

Phase 41G may begin under a separate reviewed boundary.

Phase 41G scope:

- proof/evidence/payload binding;
- message bytes to expected guardian payload hash;
- route/source/burn/recipient/amount/mint/finality/expiration binding.

Phase 41G must not enable guardian validity, quorum, authorization, replay writes, mutation, CPI, mint, handler, or live route unless separately reviewed.
