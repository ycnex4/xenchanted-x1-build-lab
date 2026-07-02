# XXXL Phase 41F.2 Ed25519 Signature Verification Boundary — External Acceptance

Date: 2026-07-02

Current main under review:

`133acb8 Merge XXXL phase 41F Ed25519 signature verification boundary`

Implementation commit:

`863ca71 Add phase 41F Ed25519 signature verification boundary`

Parent accepted checkpoint:

`326bfb9 Merge XXXL phase 41F signature verification plan acceptance record`

## Scope Accepted

Phase 41F.2 implementation is accepted as a narrow Model A native Ed25519 signature verification boundary.

Accepted implementation scope:

- establish native Ed25519 verification structurally;
- use the SVM native Ed25519 program as the verifier;
- verify that SVM already verified by relying on abort-before-current-instruction semantics;
- preserve self-reference binding;
- preserve parsed/extracted range consistency;
- add program-id defense-in-depth re-check;
- resolve SAFETY_FLAGS as cumulative pipeline capability flags;
- keep message payload correctness downstream;
- keep proof/evidence/guardian/quorum/auth/replay/mutation/CPI/mint/live disabled.

## Validation

Validation passed before merge:

- targeted Phase 41F.2 tests: 11/11 OK;
- `cargo test --manifest-path programs/xxxl-svm/Cargo.toml verifier --lib`: OK;
- `cargo test --manifest-path programs/xxxl-svm/Cargo.toml --lib --locked`: OK;
- `npm run typecheck`: OK;
- `npm run build`: OK.

## Theo Verdict

Verdict: ACCEPT

Required fixes: none.

Blocking risks: none.

Theo accepted:

- Model A establishment is correct;
- SAFETY_FLAGS cumulative semantics are acceptable;
- program-id re-check is sufficient;
- self-reference binding is preserved;
- status attribution is clear;
- message-payload correctness is deferred;
- no local crypto verification was introduced;
- no downstream trust/execution drift exists;
- tests are sufficient;
- focused crypto-boundary audit is required before 41G.

Theo architecture note:

Phase 41F.2 does not mathematically verify the signature.

The SVM is the verifier.

The XXXL program verifies that SVM verified.

## Audit Demon Verdict

Verdict: ACCEPT WITH NOTES

Required fixes: none.

Scope violations: no.

Model A establishment acceptable: yes.

SAFETY_FLAGS semantics acceptable: yes.

Program-id re-check acceptable: yes.

Self-reference binding acceptable: yes.

Status attribution acceptable: yes.

Message-payload deferral acceptable: yes.

Forbidden operations detected: no.

Trust-sensitive boundary drift: no.

Focused crypto-audit required before 41G: yes.

Next phase allowed: yes.

## Demon Note 1 — Model A Load-Bearing Precondition

Model A soundness has a load-bearing precondition.

The function `establish_native_ed25519_signature_verification` is a pure function over result structures.

It does not itself read the runtime Instructions sysvar.

It does not itself know that the current instruction is actually executing inside a real SVM transaction.

Therefore the accepted Model A argument is sound only when the boundary is called from an actually executing `process_instruction` path, with a `loading_result` derived from the real Instructions sysvar.

Accepted soundness statement:

- current instruction was reached;
- prior native Ed25519 instruction was present in the same executed transaction;
- native Ed25519 failure aborts before current instruction;
- therefore prior native Ed25519 verification already succeeded.

But this is not load-bearing if the result structures are fabricated, replayed, reconstructed, or supplied from a non-runtime source.

Future live-wiring must make this precondition explicit.

Future live-wiring must be treated as a high-risk audit point.

Mandatory future wiring-gate requirement:

- `loading_result` must be produced from the real Instructions sysvar in the actually executing transaction;
- `current_instruction_index` must be derived from runtime state, not caller input;
- the 41F.2 result must not be accepted from fabricated or reconstructed pipeline structures;
- live handler wiring must be audited before any use in an execution path.

## Demon Note 2 — Phase 41F.1 SAFETY_FLAGS Alignment

Phase 41F.2 canonizes the SAFETY_FLAGS convention as cumulative pipeline capability flags.

Under this convention:

- a true flag means the pipeline has reached a phase that establishes the capability;
- a false flag means no accepted phase has yet established the capability.

Phase 41F.2 correctly sets:

- `ed25519_signature_verification_performed: true`.

However, Phase 41F.1 still carries a conservative local-module-style `PHASE_41F_1_SAFETY_FLAGS` state.

This is not blocking because Phase 41F.1 is non-authorizing extraction and conservative false values do not enable trust.

But for consistency, Phase 41F.1 should be aligned to the cumulative convention in a later small cleanup phase.

Suggested follow-up:

- update Phase 41F.1 SAFETY_FLAGS to cumulative pipeline convention;
- document that the change is semantic consistency only;
- do not flip proof/evidence/guardian/quorum/auth/replay/mutation/CPI/mint/live flags.

## Accepted Model A Boundary

Phase 41F.2 establishes:

- `NativeEd25519VerificationEstablished`.

It does not establish:

- local cryptographic verification;
- message payload correctness;
- proof acceptance;
- evidence acceptance;
- guardian validity;
- guardian set membership;
- quorum;
- authorization.

## Accepted Defense-In-Depth Chain

Accepted checks:

- 41F.1 extraction status gate;
- matched instruction index consistency;
- parsed offsets availability;
- matched loaded entry lookup;
- program-id re-check;
- runtime-data-only re-check;
- instruction data length consistency;
- self-reference binding;
- extracted ranges match parsed ranges.

## Accepted Message Payload Deferral

Signature verification over message bytes does not mean the message is the correct gateway payload.

Message payload correctness remains downstream.

Future proof/evidence work must separately bind:

- message bytes;
- expected guardian payload hash;
- route;
- target mint;
- recipient;
- amount;
- finality;
- expiration.

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

Phase 41F.2 implementation is externally accepted.

Before Phase 41G proof/evidence acceptance begins, a focused crypto-boundary audit is required.

Carry-forward obligations before or during that audit:

- explicitly include the Model A live-wiring precondition;
- ensure no fabricated or reconstructed loading result can become load-bearing in live execution;
- keep message-payload correctness as a separate gate;
- keep signature-validity separate from proof/evidence/guardian/quorum/auth;
- consider a small Phase 41F.1 SAFETY_FLAGS alignment cleanup for cumulative consistency.

After focused crypto-boundary audit, Phase 41G may begin under a separate reviewed boundary.
