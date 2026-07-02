# XXXL Phase 41D3.2.1 Prior Index Range Boundary — External Acceptance

Date: 2026-07-02

Current main under review:

`650f605 Merge XXXL phase 41D3 prior index range boundary`

## Scope Accepted

Phase 41D3.2.1 is accepted as a narrow code boundary for prior index range construction only.

Accepted implementation:

- accepts checked current index result from Phase 41D3.1;
- requires `CurrentInstructionIndexAcquired`;
- requires `Some(current_instruction_index)`;
- constructs bounded prior range `0..current_index`;
- maps `current_index == 0` to empty prior range;
- enforces strict `< current_index`;
- excludes same index by range construction;
- excludes later indexes by range construction;
- fails closed if current index is unavailable;
- fails closed if acquired status is inconsistent with missing index;
- fails closed before allocation if a forged oversized current index is supplied.

## Theo Verdict

Verdict: ACCEPT

Required fixes: none.

Blocking risks: none.

Accepted findings:

- implementation is limited to pure prior index range construction;
- `current_index == 0` maps to empty range;
- strict `< current_index` is preserved by Rust exclusive range semantics;
- same/later indexes are excluded by construction;
- unavailable current-index path fails closed;
- forged oversized current-index path fails closed before allocation;
- no instruction loading exists;
- no raw sysvar parsing exists;
- no Phase 41C3 descriptors exist;
- no safety flags are flipped;
- proof/evidence/auth/replay/CPI/mint/live-route boundaries remain closed;
- Phase 41D3.2.2 checked loading may start after acceptance.

## Audit Demon Verdict

Verdict: ACCEPT

Required fixes: none.

Scope violations: no.

Forbidden operations detected: no.

Trust-sensitive boundary drift: no.

Next code sub-step allowed: yes.

Accepted findings:

- scope is limited to range construction over the Phase 41D3.1 result;
- `mod.rs` delta is a single `pub mod`;
- no loading, descriptors, or Phase 41C3 usage is introduced;
- no `load_instruction`, `load_instruction_at`, or `load_instruction_at_checked`;
- no raw sysvar parsing;
- no crypto/evidence/quorum/auth/replay/CPI/mint/live route;
- safety flags remain unchanged from the accepted boundary;
- `load_instruction_called` remains false;
- `locates_prior_ed25519_instruction` remains false;
- panic-safety is clean;
- unavailable/inconsistent/forged-oversized inputs fail closed.

## Non-Blocking Notes Captured

Audit Demon noted that `construct_strict_prior_index_range` materializes `(0..current).collect::<Vec<usize>>()`.

This is acceptable for Phase 41D3.2.1 because:

- the range is bounded by `u16::MAX`;
- forged values above the checked current-index space fail closed before allocation;
- this boundary is not yet integrated into a live handler;
- no instruction loading is performed;
- no descriptors are constructed;
- no state mutation occurs.

Forward-looking requirement for Phase 41D3.2.2:

- checked instruction loading should prefer lazy iteration over the prior range instead of materializing a large `Vec`;
- alternatively, a realistic transaction-instruction cap must be documented and enforced before loading;
- this is to avoid unnecessary BPF heap pressure.

Audit Demon also noted that `MAX_CHECKED_CURRENT_INSTRUCTION_INDEX = u16::MAX` is a bound for the checked index space, not a realistic transaction instruction cap.

## Active Blockers Remain

No blocker is removed, weakened, or reinterpreted by Phase 41D3.2.1.

Active blockers remain:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Minimum Safe 41D3.2.2 Boundary

Phase 41D3.2.2 may start after this acceptance record is merged.

Allowed in 41D3.2.2:

- accept bounded prior range from Phase 41D3.2.1;
- checked loading via `load_instruction_at_checked`;
- deterministic mapping of checked loading success/failure;
- `load_instruction_called: true` may flip only if checked helper is used.

Required design note for 41D3.2.2:

- prefer lazy iteration over prior indexes;
- avoid materializing large ranges for loading;
- keep memory pressure bounded.

Still forbidden in 41D3.2.2:

- `load_instruction`;
- unchecked loading;
- raw sysvar byte parsing;
- Ed25519 cryptographic verification;
- evidence acceptance;
- Phase 41C3 descriptors unless explicitly deferred to 41D3.2.3;
- quorum counting;
- authorization;
- replay writes;
- account mutation;
- CPI;
- `invoke_signed`;
- SPL Token `mint_to`;
- handler;
- live route unlock.

## Next Gate

Phase 41D3.2.2 remains gated under its own code review before merge.
