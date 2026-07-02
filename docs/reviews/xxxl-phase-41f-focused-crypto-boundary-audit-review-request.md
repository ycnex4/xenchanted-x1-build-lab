# XXXL Phase 41F Focused Crypto-Boundary Audit — Review Request

Date: 2026-07-02

## Current Main

`2efb5aa Merge XXXL phase 41F extraction safety flags acceptance record`

## Audit Scope

Focused crypto-boundary audit before Phase 41G.

No code changes.

No new runtime behavior.

## Review Targets

Audit scope:

`docs/xxxl/xxxl-phase-41f-focused-crypto-boundary-audit-scope.md`

Checkpoint:

`docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41f-focused-crypto-boundary-audit.md`

Relevant accepted code boundaries:

- `programs/xxxl-svm/src/verifier/checked_prior_instruction_loading_runtime_boundary.rs`
- `programs/xxxl-svm/src/verifier/ed25519_instruction_byte_parsing_boundary.rs`
- `programs/xxxl-svm/src/verifier/checked_ed25519_byte_extraction_boundary.rs`
- `programs/xxxl-svm/src/verifier/ed25519_signature_verification_boundary.rs`

Relevant accepted review records:

- `docs/reviews/xxxl-phase-41f-2-ed25519-signature-verification-boundary-external-acceptance.md`
- `docs/reviews/xxxl-phase-41f-1-safety-flags-cumulative-alignment-external-acceptance.md`

## Requested Audit

Please check:

1. Is the Model A soundness argument acceptable?
2. Is it clear that the SVM is the verifier and XXXL only establishes that SVM verified?
3. Is the load-bearing live-wiring precondition captured correctly?
4. Is self-reference binding preserved?
5. Is checked extraction bounded and borrowed?
6. Is program-id defense-in-depth re-check present?
7. Is status attribution clear and not misleading?
8. Are SAFETY_FLAGS cumulative and consistent across Phase 41F?
9. Is message payload correctness clearly deferred to Phase 41G?
10. Is signature-validity kept separate from proof/evidence/guardian/quorum/auth?
11. Are replay/mutation/CPI/mint/live still forbidden?
12. Are active blockers preserved?
13. Can Phase 41G begin after this audit is accepted?

## Expected Verdict Format

- Verdict: ACCEPT / ACCEPT WITH NOTES / REQUIRES FIXES / BLOCKED
- Required fixes:
- Non-blocking notes:
- Scope violations: yes/no
- Model A soundness acceptable: yes/no
- SVM-as-verifier framing acceptable: yes/no
- Live-wiring precondition captured: yes/no
- Self-reference binding acceptable: yes/no
- Checked extraction acceptable: yes/no
- Program-id re-check acceptable: yes/no
- Status attribution acceptable: yes/no
- SAFETY_FLAGS taxonomy acceptable: yes/no
- Message-payload deferral acceptable: yes/no
- Proof/evidence/guardian/quorum/auth drift: yes/no
- Replay/mutation/CPI/mint/live drift: yes/no
- Active blockers preserved: yes/no
- Phase 41G allowed after acceptance: yes/no
