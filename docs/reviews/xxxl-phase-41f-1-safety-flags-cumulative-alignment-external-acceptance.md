# XXXL Phase 41F.1 SAFETY_FLAGS Cumulative Alignment — External Acceptance

Date: 2026-07-02

Current main under review:

`be96d3e Merge XXXL phase 41F extraction safety flags alignment`

Implementation commit:

`1254356 Align phase 41F extraction safety flags with cumulative semantics`

Parent accepted checkpoint:

`6e793c9 Merge XXXL phase 41F signature verification boundary acceptance record`

## Scope Accepted

Phase 41F.1 SAFETY_FLAGS cumulative alignment is accepted as a semantic consistency cleanup.

Accepted scope:

- align Phase 41F.1 `PHASE_41F_1_SAFETY_FLAGS` with cumulative pipeline capability semantics;
- set only already-established upstream capability flags to true;
- keep `ed25519_signature_verification_performed` false in Phase 41F.1;
- keep all downstream trust/execution flags false;
- make no extraction logic changes;
- make no trust-sensitive boundary expansion.

## Validation

Validation passed before merge:

- targeted 41F.1 extraction tests: OK;
- `cargo test --manifest-path programs/xxxl-svm/Cargo.toml verifier --lib`: OK;
- `cargo test --manifest-path programs/xxxl-svm/Cargo.toml --lib --locked`: OK;
- `npm run typecheck`: OK;
- `npm run build`: OK.

## Theo Verdict

Verdict: ACCEPT

Required fixes: none.

Blocking risks: none.

Theo accepted:

- cumulative convention is applied correctly;
- upstream capabilities from 41B through 41D are now reflected in 41F.1;
- only upstream-established flags are true;
- signature verification remains false in 41F.1;
- downstream proof/evidence/guardian/quorum/auth/replay/mutation/CPI/mint/live flags remain false;
- no extraction logic changed;
- no trust-sensitive drift exists;
- focused crypto-boundary audit can proceed.

Theo summary:

- 5 flags flipped;
- zero logic changed;
- zero tests changed;
- trust boundary not expanded.

## Audit Demon Verdict

Verdict: ACCEPT

Required fixes: none.

Scope violations: no.

Cumulative semantics acceptable: yes.

Upstream true flags acceptable: yes.

Signature verification still false in 41F.1: yes.

Downstream trust flags still false: yes.

Logic changed: no.

Trust-sensitive boundary drift: no.

Focused audit can proceed: yes.

Demon accepted that this cleanup closes the Phase 41F.2 Note 2 exactly.

## Accepted Flag Changes

The following Phase 41F.1 flags were aligned from conservative local-module-style false to cumulative true:

- `account_info_parser_implemented`;
- `load_instruction_called`;
- `load_instruction_enabled`;
- `concrete_runtime_api_selected`;
- `current_instruction_identity_derived_from_runtime`.

These are upstream capabilities already established by earlier accepted pipeline phases.

The following remains false in Phase 41F.1:

- `raw_instructions_sysvar_parser_implemented`.

This remains false because no raw Instructions sysvar parser was introduced.

## Flags That Remain False

The following remain false in Phase 41F.1:

- `ed25519_signature_verification_performed`;
- `cryptographic_signature_proof_accepted`;
- `verification_evidence_accepted`;
- `quorum_counting_enabled`;
- `authorization_enabled`;
- `replay_write_enabled`;
- `processed_event_marking_enabled`;
- `account_mutation_enabled`;
- `cpi_enabled`;
- `invoke_signed_enabled`;
- `spl_token_mint_to_enabled`;
- `process_instruction_handler_added`;
- `live_route_enabled`.

## Demon Non-Blocking Note

Demon suggested an optional follow-up:

- add positive assertions for the five cumulative upstream flags inside `report_preserves_extraction_only_non_authorizing_boundary`.

This is not blocking.

Suggested future test assertions:

- `assert!(report.safety_flags.account_info_parser_implemented)`;
- `assert!(report.safety_flags.load_instruction_called)`;
- `assert!(report.safety_flags.load_instruction_enabled)`;
- `assert!(report.safety_flags.concrete_runtime_api_selected)`;
- `assert!(report.safety_flags.current_instruction_identity_derived_from_runtime)`.

## Still Forbidden

The following remain forbidden:

- local cryptographic verification;
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

No blocker is removed, weakened, or reinterpreted.

Active blockers remain:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Carry-Forward Risk

The main forward risk remains the Phase 41F.2 Model A live-wiring precondition.

Model A is load-bearing only when called from an actually executing runtime path with a `loading_result` derived from the real Instructions sysvar.

This must remain a high-risk audit point for future live wiring.

## Next Gate

Phase 41F.1 SAFETY_FLAGS cumulative alignment is externally accepted.

Focused crypto-boundary audit can proceed before Phase 41G.

Focused audit must include:

- consistent cumulative Phase 41F SAFETY_FLAGS taxonomy;
- Model A abort-before-current soundness;
- self-reference binding;
- checked extraction;
- program-id re-check;
- status attribution;
- message-payload correctness remains downstream;
- no proof/evidence/guardian/quorum/auth drift;
- no replay/mutation/CPI/mint/live drift;
- Model A live-wiring precondition.
