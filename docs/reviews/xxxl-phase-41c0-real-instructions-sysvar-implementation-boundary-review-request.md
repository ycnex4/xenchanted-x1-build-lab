# XXXL Phase 41C0 Real Instructions Sysvar Implementation Boundary Review Request

Status: External review request.

Audience:

- Audit demon
- Theo

## Context

Phase 41B was accepted.

Phase 41C may start, but it is the first real runtime boundary.

Phase 41C0 is docs-only.

It splits Phase 41C before any real runtime implementation is written.

## Review Scope

Please review Phase 41C0 only.

Primary files:

- `docs/xxxl/xxxl-phase-41c0-real-instructions-sysvar-implementation-boundary.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41c0-real-instructions-sysvar-implementation-boundary.md`
- `docs/reviews/xxxl-phase-41c0-real-instructions-sysvar-implementation-boundary-review-request.md`
- `docs/checkpoints/current-design-checkpoint.md`

## Core Boundary To Review

Phase 41C0 must preserve:

~~~text
located candidate evidence
  != parsed evidence
  != prior-instruction ordering
  != requirement coverage
  != runtime sysvar read
  != structural candidate evidence
  != verification evidence
  != cryptographic proof
  != quorum
  != authorization
  != replay consumption
  != execution
~~~

## Questions For Reviewers

1. Did Phase 41C0 remain docs-only?

2. Did Phase 41C0 avoid selecting a concrete runtime API?

3. Did Phase 41C0 avoid raw Instructions sysvar parsing?

4. Did Phase 41C0 avoid `AccountInfo` parsing?

5. Did Phase 41C0 avoid calling `load_instruction`?

6. Did Phase 41C0 avoid current instruction identity derivation?

7. Did Phase 41C0 avoid prior Ed25519 lookup?

8. Did Phase 41C0 avoid proof, quorum, authorization, replay, CPI, and mint
   execution?

9. Is the 41C split safe?

10. Is Phase 41C1 narrow enough?

11. Should Phase 41C1 be allowed to select a concrete runtime API?

12. Which flags may Phase 41C1 set to true, if any?

13. Should Phase 41C1 include `load_instruction` / `load_instruction_at`, or
    should that be deferred to a later subphase?

14. Does Phase 41C0 correctly state that Phase 41B taxonomy is authoritative?

15. What must remain prohibited in Phase 41C1?

## Requested Verdict Format

Please answer with one of:

- ACCEPT
- ACCEPT WITH NOTES
- REQUEST CHANGES
- BLOCK

Please include:

- required fixes, if any
- blocking risks, if any
- optional notes, if any
- whether Phase 41C1 may start
- minimum safe Phase 41C1 boundary
- what must remain prohibited in Phase 41C1
