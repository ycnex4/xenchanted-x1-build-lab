# XXXL Phase 41C0A Clarify 41C1 Sysvar Access Boundary Review Request

Status: External review request.

Audience:

- Audit demon
- Theo

## Context

Phase 41C0 was accepted with notes.

Both reviewers recommended that Phase 41C1 should not include
`load_instruction`, `load_instruction_at`, or any equivalent helper that reads a
specific transaction instruction.

Phase 41C0A records that decision before Phase 41C1 implementation starts.

## Review Scope

Please review Phase 41C0A only.

Primary files:

- `docs/xxxl/xxxl-phase-41c0a-clarify-41c1-sysvar-access-boundary.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41c0a-clarify-41c1-sysvar-access-boundary.md`
- `docs/reviews/xxxl-phase-41c0a-clarify-41c1-sysvar-access-boundary-review-request.md`
- `docs/checkpoints/current-design-checkpoint.md`

## Questions For Reviewers

1. Did Phase 41C0A remain docs-only?

2. Did Phase 41C0A correctly defer `load_instruction` out of Phase 41C1?

3. Did Phase 41C0A correctly limit Phase 41C1 to presence/readability and API
   selection?

4. Did Phase 41C0A correctly keep current instruction identity in Phase 41C2?

5. Did Phase 41C0A correctly keep prior Ed25519 lookup and strict ordering in
   Phase 41C3?

6. Did Phase 41C0A preserve Phase 41B taxonomy as authoritative?

7. Did Phase 41C0A keep proof, quorum, authorization, replay, CPI, and mint
   execution forbidden?

8. May Phase 41C1 start after this clarification?

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
