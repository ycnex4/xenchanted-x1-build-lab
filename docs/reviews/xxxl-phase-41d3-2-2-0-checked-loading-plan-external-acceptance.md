# XXXL Phase 41D3.2.2.0 Checked Prior Instruction Loading Plan — External Acceptance

Date: 2026-07-02

Current main under review:

`fbefba4 Merge XXXL phase 41D3 checked loading plan`

## Scope Accepted

Phase 41D3.2.2.0 is accepted as a docs-only plan before introducing checked prior instruction loading.

No runtime code was introduced.

Accepted planning scope:

- accept bounded prior indexes from Phase 41D3.2.1;
- iterate prior indexes lazily;
- call `load_instruction_at_checked` only for prior indexes;
- avoid unchecked loading;
- avoid raw sysvar parsing;
- avoid prefiltering;
- avoid Phase 41C3 descriptor construction;
- keep loaded instructions non-authorizing.

## Audit Demon Verdict

Verdict: ACCEPT

Required fixes: none.

Scope violations: no.

Loading boundary acceptable: yes.

Next code sub-step allowed: yes.

Accepted findings:

- docs-only scope is clean;
- `load_instruction_at_checked` is acceptable as the only loading helper;
- missing/wrong Instructions sysvar account must fail closed;
- empty prior range must cause no loading attempt;
- checked loading failure must be deterministic and non-panicking;
- loaded instruction is runtime data only, not proof/evidence/auth;
- `locates_prior_ed25519_instruction` remains false in Phase 41D3.2.2;
- allowed flips are loading-related only:
  - `prior_instruction_loading_enabled`;
  - `load_instruction_called`;
  - `load_instruction_enabled`;
- forbidden boundaries remain closed:
  - `load_instruction`;
  - `load_instruction_at`;
  - unchecked loading;
  - raw sysvar parsing;
  - direct sysvar byte slicing;
  - descriptors;
  - evidence acceptance;
  - crypto verification;
  - quorum/auth/replay;
  - CPI/mint/live route.

## Theo Verdict

Verdict: ACCEPT

Required fixes: none.

Blocking risks: none.

Accepted findings:

- docs-only plan is the correct gate before first real loading;
- lazy iteration is the correct pattern;
- implementation should borrow the bounded range and use `.iter().copied()`;
- implementation should avoid materializing a second large vector;
- `load_instruction_at_checked` is the only acceptable helper;
- `load_instruction` and `load_instruction_at` remain forbidden;
- raw sysvar parsing remains forbidden;
- empty prior range maps to no loading attempt;
- checked loading failure must be deterministic and non-panic;
- Phase 41D3.2.2 must avoid prefiltering and descriptors;
- `locates_prior_ed25519_instruction` remains false;
- only loading-related flags may flip;
- proof/evidence/auth/replay/CPI/mint/live boundaries remain closed;
- Phase 41D3.2.2 code may start after acceptance.

## Cap / Memory Notes

Theo noted that no additional cap is required for Phase 41D3.2.2 because the prior range from Phase 41D3.2.1 is already bounded by the checked current-index space:

- `MAX_CHECKED_CURRENT_INSTRUCTION_INDEX = u16::MAX`;
- `0..current_index`;
- maximum prior indexes: `<= 65535`.

Audit Demon noted that future code should still avoid unnecessary heap pressure:

- prefer lazy iteration over prior indexes;
- do not construct a second large index vector;
- if loaded results are collected, the buffer must be bounded by the accepted prior range;
- ideally, future 41D3.2.3 should stream loaded entries into prefiltering instead of holding all loaded instructions in memory at once.

## Minimum Safe 41D3.2.2 Boundary

Phase 41D3.2.2 may start after this acceptance record is merged.

Allowed:

- consume bounded prior range from Phase 41D3.2.1;
- borrow prior indexes and iterate with `.iter().copied()`;
- accept Instructions sysvar AccountInfo as checked loading source;
- verify Instructions sysvar account key before loading;
- call `load_instruction_at_checked(index, instructions_sysvar_account)` only for prior indexes;
- deterministic mapping of checked loading success/failure;
- empty prior range causes no loading attempt;
- checked loading failure is non-panic and non-authorizing;
- loaded instruction remains runtime data only;
- loading-related flags may flip:
  - `prior_instruction_loading_enabled: true`;
  - `load_instruction_called: true`;
  - `load_instruction_enabled: true`.

Forbidden:

- `load_instruction`;
- `load_instruction_at`;
- unchecked loading;
- raw Instructions sysvar byte parsing;
- direct sysvar byte slicing;
- prefiltering;
- Phase 41C3 descriptors;
- `locates_prior_ed25519_instruction: true`;
- Ed25519 cryptographic verification;
- verification evidence acceptance;
- quorum counting;
- authorization;
- replay writes;
- processed event marking;
- account mutation;
- CPI;
- `invoke_signed`;
- SPL Token `mint_to`;
- handler;
- live route unlock.

## Active Blockers Remain

No blocker is removed, weakened, or reinterpreted by Phase 41D3.2.2.0.

Active blockers remain:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Next Gate

Phase 41D3.2.2 remains gated under its own code review before merge.
