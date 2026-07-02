# XXXL Phase 41D3.0 Prior Instruction Enumeration Plan External Acceptance

Status: External review accepted.

Reviewed phase:

- Phase 41D3.0 prior-instruction enumeration runtime boundary plan

Reviewed main:

- `a79bc6b Merge XXXL phase 41D3 prior instruction enumeration plan`

Reviewers:

- Audit Demon
- Theo

## Final Review Gate Verdict

Theo verdict:

- `ACCEPT`

Theo required fixes:

- none

Theo blocking risks:

- none

Demon verdict:

- `ACCEPT`

Demon required fixes:

- none

Gate status:

- Phase 41D3.0 external review gate is closed
- Phase 41D3 code may start
- Phase 41D3 code remains gated under separate review before merge

## Theo Acceptance Summary

Theo confirmed:

1. Phase 41D3.0 as docs-only before code is correct.
2. Scope is narrow enough.
3. Real prior-instruction enumeration through Instructions sysvar is acceptable.
4. Checked instruction loading through `load_instruction_at_checked` or reviewed equivalent is the correct boundary.
5. Raw byte parsing remains excluded.
6. Prefiltering is separated from evidence acceptance.
7. Candidate descriptors are not proof, not evidence, and not authorization.
8. Same/later anomaly decision is required.
9. Minimum safe decision is acceptable:
   - same-index match: reject
   - later-index match: reject
   - prior-index match: candidate only
10. Allowed flag flips are limited.
11. Trust-sensitive flags remain false.
12. Proposed test coverage is sufficient.
13. Blockers are untouched.
14. Phase 41D3 code may start.

Theo confirmed this is a fail-closed boundary.

## Demon Acceptance Summary

Demon confirmed:

- scope violations: no
- forbidden operations detected: no
- blockers changed: no
- next phase/code allowed: yes

Demon noted that Phase 41D3 is the widest real runtime step so far.

Phase 41D3 combines:

- current-index read
- prior enumeration
- checked loading
- prefiltering
- Phase 41C3 descriptor construction
- same/later anomaly decision
- two capability flag flips

Demon classified the boundary as coherent but dense.

The Phase 41D3 code audit must check panic-safety for each sub-step separately.

Optional forward note:

- current-index acquisition may be split into a separate sub-step if the implementation becomes too dense
- this is optional, not required

## Same/Later Anomaly Requirement

Both reviewers accepted the fail-closed same/later anomaly rule.

Phase 41D3 code must explicitly implement:

- same-index match: reject
- later-index match: reject
- prior-index match: candidate only, not proof

The code must not rely on descriptor-layer behavior that merely ignores same/later cases.

The same/later reject rule must be pinned by tests.

## Minimum Safe Phase 41D3 Code Boundary

Phase 41D3 code may include:

- real prior-instruction enumeration via Instructions sysvar
- checked current-index acquisition
- checked instruction loading through `load_instruction_at_checked` or reviewed equivalent
- prefiltering unrelated instructions
- non-Ed25519 instruction discard
- Phase 41C3 candidate descriptor construction
- explicit same-index reject
- explicit later-index reject
- prior-index candidate construction only
- `locates_prior_ed25519_instruction: true` flag flip
- `load_instruction_called: true` flag flip if a checked helper is used

Phase 41D3 code must not include:

- Ed25519 cryptographic verification
- verification evidence acceptance
- guardian quorum counting
- authorization
- replay writes
- account mutation
- CPI
- `invoke_signed`
- SPL Token `mint_to`
- live route unlock
- handler or execution flag enablement

## Required Phase 41D3 Code Audit Focus

The Phase 41D3 code review must focus on:

- panic-safety of current-index acquisition
- panic-safety of checked instruction loading
- no unchecked index or slice access
- invalid index fail-closed behavior
- malformed instruction fail-closed behavior
- unrelated instruction discard
- explicit same-index reject
- explicit later-index reject
- prior-index candidate only
- no evidence acceptance
- no authorization
- no replay write
- no CPI or mint execution
- exact intended flag flips only

## Active Blockers Remain

The following blockers remain active:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

Note:

- `EXTERNAL_REVIEW_INCOMPLETE` remains a global deployment blocker.
- Phase 41D3.0 itself has received external acceptance.
- This acceptance does not unlock deployment, CPI, mint execution, live route execution, or production readiness.
