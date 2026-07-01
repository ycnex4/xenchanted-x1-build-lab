# XXXL Phase 40 Ed25519 Verification Evidence Review Request

Status: External review request.

Audience:

- Audit demon
- Theo

## Context

We closed the Phase 40 Ed25519 verification evidence preparation series.

Latest expected main checkpoint:

- Phase 40F merge: `Merge XXXL phase 40F Ed25519 verification evidence coverage matrix`
- Phase 40G closes the series as a docs-only control point.

## Review Scope

Please review Phase 40A through Phase 40G.

Primary files:

- `docs/xxxl/xxxl-phase-40g-ed25519-verification-evidence-series-closure.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-40g-ed25519-verification-evidence-series-closure.md`
- `docs/reviews/xxxl-phase-40-ed25519-verification-evidence-review-request.md`
- `programs/xxxl-svm/src/verifier/ed25519_verification_evidence.rs`
- `programs/xxxl-svm/src/verifier/ed25519_verification_evidence_integration_design.rs`
- `programs/xxxl-svm/src/verifier/ed25519_prior_instruction_ordering.rs`
- `programs/xxxl-svm/src/verifier/ed25519_verification_evidence_coverage_matrix.rs`
- `programs/xxxl-svm/src/verifier/instructions_sysvar_evidence_scanner.rs`
- `programs/xxxl-svm/src/verifier/ed25519_instruction_data_parser.rs`
- `programs/xxxl-svm/src/verifier/ed25519_evidence_layout.rs`
- `programs/xxxl-svm/src/verifier/canonical_payload.rs`
- `programs/xxxl-svm/src/verifier/guardian_quorum.rs`
- `programs/xxxl-svm/src/verifier/boundary.rs`

## Core Claim To Review

The Phase 40 series should preserve this rule:

~~~text
located candidate evidence
  != parsed evidence
  != prior-instruction ordering
  != requirement coverage
  != verification evidence
  != quorum
  != authorization
  != execution
~~~

## Questions For Reviewers

1. Did any Phase 40 module accidentally imply Ed25519 proof acceptance?

2. Did any Phase 40 module accidentally imply quorum, authorization, replay
   consumption, mint execution, SPL CPI, or live route unlock?

3. Is the prior-instruction ordering model conceptually correct?

4. Is the future requirement set sufficient before raw Instructions sysvar
   integration?

5. Is the future rejection case set sufficient?

6. Does the requirement-to-rejection coverage matrix miss any important risk?

7. Should Phase 41 begin as docs-only, model-only, or implementation?

8. What is the minimum safe Phase 41 implementation boundary?

## Non-Goals Confirmed

The Phase 40 series must not be interpreted as:

- production readiness
- final immutability
- accepted Ed25519 verification proof
- guardian quorum authorization
- replay-protected mint authorization
- SPL Token mint execution
- live bridge route activation

## Requested Verdict Format

Please answer with one of:

- ACCEPT
- ACCEPT WITH NOTES
- REQUEST CHANGES
- BLOCK

Please include:

- required fixes, if any
- optional notes, if any
- recommended Phase 41 boundary
