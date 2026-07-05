# Phase 41K.6 B6.7 — Placeholder boundary manual resolution

## Purpose

This document records the manual resolution of the B6.7 placeholder program id boundary question.

The previous automated scan reported:

- placeholder_in_pda_file: true
- placeholder_near_structural_terms: true
- source_level_conclusion: structural_placeholder_risk_detected_requires_manual_review

Manual review was required before Strategy 2 could proceed to planning.

This document records that manual review.

This document does not approve signing.

This document does not approve transaction submission.

This document does not approve SOL spend.

This document does not approve private-key handling.

This document does not approve deploy.

This document does not approve program upgrade.

This document does not approve account initialization.

This document does not approve SPL mint setup.

This document does not approve guardian package construction.

This document does not approve submit rehearsal.

This document does not remove the B1C7 compile_error guard.

This document does not weaken the B1C7 feature gate.

This document does not open production or production-like activation.

## Current main checkpoint

B6.7 placeholder boundary analysis is merged on main:

b573b88 Merge phase 41K.6 B6.7 placeholder boundary analysis

Current decision remains:

NO-GO.

## Manual review target

The critical question was:

Does placeholder_program_id_boundary mean the on-chain program id is structurally wrong for PDA derivation, or is it a replaceable readiness/deployment blocker?

## Manual review result

Manual review found that the placeholder appears in pda.rs only inside a test safety assertion:

- assert_ne!(candidate, crate::XXXL_PROGRAM_ID_PLACEHOLDER)

This assertion prevents a public testnet program id candidate from accidentally being equal to the placeholder.

It is not used as a PDA derivation input.

It is not passed into Pubkey::find_program_address.

It is not baked into runtime PDA derivation.

## PDA derivation result

The gateway mint authority PDA derivation uses:

- function input: program_id: &Pubkey
- derivation: Pubkey::find_program_address(&gateway_mint_authority_seeds(), program_id)

Therefore, the source-level PDA derivation is program-id-parametric.

The placeholder program id is not structurally embedded into the PDA derivation path.

## Program id readiness result

The placeholder program id is used in program_id_status.rs as a readiness and deployment blocker:

- status_code: PLACEHOLDER_PROGRAM_ID_BOUNDARY
- configured_program_id: XXXL_PROGRAM_ID_PLACEHOLDER
- deployable_path_ready: false
- blocker_code: PLACEHOLDER_PROGRAM_ID
- resolution: Set and review the real Program ID and regenerate all Program-ID-dependent PDA fixtures.

This means the placeholder boundary is real, but it is a deployability/readiness blocker.

It does not by itself force Strategy 3.

## Manual conclusion

The automated structural-risk result is resolved as a false positive for the currently reviewed PDA derivation source.

- previous_automated_conclusion: structural_placeholder_risk_detected_requires_manual_review
- manual_resolution: placeholder_boundary_is_readiness_blocker_not_structural_pda_constant
- strategy_1_status: closed_not_viable
- strategy_2_status: viable_for_planning
- strategy_3_status: fallback_if_later_structural_blocker_is_found

## Strategy decision implication

Strategy 2 remains the preferred planning path:

- upgrade the existing documented X1 testnet program,
- then initialize required state,
- then set up SPL mint authority,
- then prepare guardian package,
- then run exactly one later-approved submit rehearsal.

This document does not approve any of those actions.

## Remaining blockers before any upgrade GO

All blockers imported from the strategy assessment remain open:

- A: upgrade authority custody map
- B: expected post-upgrade ProgramData hash
- C: B1C7 handler presence verification
- D: state initialization instruction design
- E: SPL mint authority architecture
- F: guardian set testnet descriptor
- G: rollback or recovery plan
- H: local validator dry-run

None of these blockers are closed by this document.

## Current decision

Current decision:

NO-GO.

This manual resolution does not authorize live action.
