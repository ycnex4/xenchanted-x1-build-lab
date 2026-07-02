# XXXL Phase 41D3.2.2 Checked Prior Instruction Loading Boundary — External Acceptance

Date: 2026-07-02

Current main under review:

`d2a87b4 Merge XXXL phase 41D3 checked prior loading boundary`

## Scope Accepted

Phase 41D3.2.2 is accepted as a code boundary for checked prior instruction loading only.

Accepted implementation scope:

- consume bounded prior range from Phase 41D3.2.1;
- verify prior indexes remain strictly before current index;
- accept Instructions sysvar AccountInfo;
- check Instructions sysvar account key before loading;
- empty prior range causes no loading attempt;
- iterate prior indexes with `.iter().copied()`;
- call `load_instruction_at_checked` only for prior indexes;
- map checked loading success to runtime-data-only entries;
- map checked loading failure to deterministic non-authorizing failure.

## Theo Verdict

Verdict: ACCEPT

Required fixes: none.

Blocking risks: none.

Accepted findings:

- code is limited to checked prior loading;
- empty prior range causes no loading and does not require the sysvar account;
- missing/wrong sysvar account fails closed;
- `load_instruction_at_checked` is the only loading helper used;
- `load_instruction` and `load_instruction_at` are absent;
- raw sysvar parsing is absent;
- direct byte slicing is absent;
- lazy iteration is used via `.iter().copied()`;
- checked failure maps deterministically and non-panic;
- loaded instruction remains runtime data only;
- prefiltering and descriptors are absent;
- `locates_prior_ed25519_instruction` remains false;
- only loading-related flags are flipped;
- all proof/evidence/auth/replay/CPI/mint/live boundaries remain closed;
- Phase 41D3.2.3 may start after acceptance.

## Audit Demon Verdict

Verdict: ACCEPT

Required fixes: none.

Scope violations: no.

Forbidden operations detected: no.

Trust-sensitive boundary drift: no.

Next code sub-step allowed: yes.

Accepted findings:

- scope is limited to checked prior loading plus docs and append-only checkpoint update;
- `mod.rs` change is one `pub mod`;
- only `load_instruction_at_checked` is used;
- loading is performed only for prior indexes from bounded range;
- iteration is lazy with `.iter().copied()`;
- `load_instruction`, `load_instruction_at`, unchecked loading, raw sysvar parsing, direct byte slicing, prefiltering, Phase 41C3 descriptors, crypto/evidence/quorum/auth/replay/CPI/mint/live behavior are absent;
- only loading capability flags are flipped:
  - `load_instruction_called: true`;
  - `load_instruction_enabled: true`;
  - `prior_instruction_loading_enabled: true`;
- `raw_instructions_sysvar_parser_implemented`, `locates_prior_ed25519_instruction`, proof/evidence/quorum/auth/replay/CPI/mint/handler/live flags remain false;
- loaded entries carry `is_evidence: false` and `authorizes_execution: false`;
- panic-safety is clean:
  - status checks;
  - `Some` handling;
  - same/later guard;
  - empty/missing/wrong-key fail-closed before loop;
  - `load_instruction_at_checked` returns `Result`;
  - failure maps deterministically;
  - no `unwrap`, `expect`, `panic!`, `unsafe`, indexing, or slicing.

## Defense-In-Depth Notes

The runtime same/later guard is accepted as defense in depth.

If a prior range result is tampered with and contains any index greater than or equal to the current instruction index:

- the boundary fails closed before loading;
- rejection maps to `Ed25519InstructionAfterCurrentInstruction`;
- no loading is attempted;
- no evidence or authorization is produced.

This is stricter than relying only on the Phase 41D3.2.1 range construction.

## Non-Blocking Heap Note

Audit Demon noted that Phase 41D3.2.2 collects all loaded prior instructions into `Vec<LoadedPriorInstruction>`.

This is accepted for Phase 41D3.2.2 because:

- the collection is bounded by the accepted prior range;
- the code is not wired into a live handler;
- real transactions are expected to be small.

Forward-looking note for Phase 41D3.2.3:

- prefer streaming `load -> prefilter -> discard non-candidates`;
- avoid holding all loaded instructions in memory if not needed;
- keep prefilter panic-safe and bounded;
- descriptors must remain non-authorizing.

## Minimum Safe Phase 41D3.2.3 Boundary

Phase 41D3.2.3 may start after this acceptance record is merged.

Allowed in Phase 41D3.2.3:

- prefilter unrelated instructions from loaded prior instructions;
- identify Ed25519 program-id candidates structurally;
- construct Phase 41C3 candidate descriptors;
- explicitly reject same-index candidates;
- explicitly reject later-index candidates;
- flip `locates_prior_ed25519_instruction: true`.

Still forbidden:

- Ed25519 cryptographic verification;
- signature proof acceptance;
- verification evidence acceptance;
- guardian quorum counting;
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

No blocker is removed, weakened, or reinterpreted by Phase 41D3.2.2.

Active blockers remain:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Next Gate

Phase 41D3.2.3 remains gated under its own code review before merge.
