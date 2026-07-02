# XXXL Phase 41D3.2.0 Prior Enumeration / Checked Loading Plan — External Acceptance

Date: 2026-07-02

Current main under review:

`d64a4e4 Merge XXXL phase 41D3 prior enumeration loading plan`

## Scope Accepted

Phase 41D3.2.0 is accepted as a docs-only plan.

No runtime code was introduced.

Accepted planning scope:

- Phase 41D3.2.1 — prior index range enumeration only;
- Phase 41D3.2.2 — checked prior instruction loading via `load_instruction_at_checked`;
- Phase 41D3.2.3 — prefilter + Phase 41C3 descriptors + explicit same/later reject.

## Theo Verdict

Verdict: ACCEPT

Required fixes: none.

Blocking risks: none.

Accepted findings:

- splitting enumeration and loading is correct;
- `current_index == 0 => empty prior range` is correct fail-closed behavior;
- strict `< current_index` is sufficient for the enumeration layer;
- same/later reject should remain explicit at the runtime boundary as defense-in-depth;
- `load_instruction_at_checked` is the correct checked loading helper;
- `load_instruction` and unchecked loading remain forbidden;
- raw Instructions sysvar byte parsing remains forbidden;
- Phase 41C3 descriptors without evidence acceptance are safe;
- expected flag flips are limited to:
  - `locates_prior_ed25519_instruction: true`;
  - `load_instruction_called: true`;
- proof/evidence/quorum/auth/replay/CPI/mint/live-route boundaries remain closed;
- Phase 41D3.2.1 code may start immediately after acceptance.

## Audit Demon Verdict

Verdict: ACCEPT

Required fixes: none.

Scope violations: no.

Split acceptable: yes.

Next code sub-step allowed: yes.

Accepted findings:

- docs-only scope is clean;
- no runtime code was introduced;
- split is acceptable:
  - 41D3.2.1: index-range enumeration only, no loading;
  - 41D3.2.2: checked loading via `load_instruction_at_checked`;
  - 41D3.2.3: prefilter + Phase 41C3 descriptors + explicit same/later reject;
- `current_index == 0 => empty prior range` is correct;
- strict `< current_index` is correct;
- explicit same/later reject is acceptable as defense-in-depth;
- `load_instruction_at_checked` is the only allowed loading helper;
- forbidden boundaries remain preserved:
  - raw sysvar parsing;
  - unchecked loading;
  - `load_instruction`;
  - cryptographic verification;
  - evidence acceptance;
  - quorum/auth/replay;
  - CPI;
  - mint;
  - live route;
- capability flips are limited and delayed until implementation and review:
  - `locates_prior_ed25519_instruction`;
  - `load_instruction_called`.

## Non-Blocking Notes Captured

Audit Demon noted that the plan document did not include the standard explicit section:

- Active blockers remain;
- no blocker weakened.

This acceptance record captures that traceability statement below.

Audit Demon also noted that explicit same/later reject in 41D3.2.3 is redundant relative to strict `< current_index` range construction, but acceptable as defense-in-depth. The future 41D3.2.3 tests must exercise the explicit reject path directly.

## Active Blockers Remain

No blocker is removed, weakened, or reinterpreted by Phase 41D3.2.0.

Active blockers remain:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Minimum Safe 41D3.2.1 Boundary

Phase 41D3.2.1 may start after this acceptance record is merged.

Allowed in 41D3.2.1:

- accept a checked current index from Phase 41D3.1;
- construct bounded prior index range `0..current_index`;
- `current_index == 0` maps to an empty prior range;
- enforce strict `< current_index` ordering;
- remain pure range construction.

Forbidden in 41D3.2.1:

- any instruction loading;
- `load_instruction`;
- `load_instruction_at`;
- `load_instruction_at_checked`;
- raw sysvar parsing;
- descriptor construction;
- Phase 41C3 descriptor usage;
- cryptographic verification;
- evidence acceptance;
- quorum counting;
- authorization;
- replay writes;
- account mutation;
- CPI;
- `invoke_signed`;
- SPL Token `mint_to`;
- handler;
- live route unlock;
- any flag flip.

## Next Gate

Phase 41D3.2.1 remains gated under its own code review before merge.
