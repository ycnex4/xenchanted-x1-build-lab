# Phase 41K.6 B6.7 — Placeholder program id boundary analysis

## Purpose

This document records the B6.7 planning analysis requested after the B6.6 strategy assessment.

The key decision is whether Strategy 2 can proceed.

Strategy 2 is only viable if placeholder_program_id_boundary is a replaceable readiness or runtime boundary, not a structural PDA derivation constant that would force a new deployment.

This analysis inspects local source files only.

This analysis does not use RPC.

This analysis does not sign.

This analysis does not submit transactions.

This analysis does not spend SOL.

This analysis does not access private keys.

This analysis does not load keypair files.

This analysis does not deploy.

This analysis does not upgrade a program.

This analysis does not initialize accounts.

This analysis does not remove the B1C7 compile_error guard.

This analysis does not weaken the B1C7 feature gate.

This analysis does not open production or production-like activation.

## Current main checkpoint

B6.6 local runtime capability inventory is merged on main:

30a7284 Merge phase 41K.6 B6.6 local runtime capability inventory

Current decision remains:

NO-GO.

## Strategy assessment recorded

- Strategy 1: closed as not viable.
- Strategy 2: recommended for testnet if placeholder_program_id_boundary is not structural.
- Strategy 3: acceptable fallback if Strategy 2 is structurally blocked.
- Strategy 4: stop and redesign remains available if neither Strategy 2 nor Strategy 3 is acceptable.

## Critical question

Does placeholder_program_id_boundary mean:

1. a replaceable readiness or runtime check that an upgrade can replace,

or

2. a structural derivation constant that makes Strategy 2 incapable of supporting the correct PDA layout?

## Source-level scan result

- observed_at_utc: 2026-07-05T18:14:39+00:00
- placeholder_literal_count: 2
- placeholder_symbol_count: 7
- placeholder_in_pda_file: true
- placeholder_near_structural_terms: true
- pda_file_uses_program_id_argument_or_reference: true
- source_level_conclusion: structural_placeholder_risk_detected_requires_manual_review

## Interpretation

The local source-level scan did not conclusively clear the placeholder boundary.

Strategy 2 must not proceed to upgrade planning until the structural risk is manually resolved.

## Placeholder references

- programs/xxxl-svm/src/deployment_status.rs:L272:     use crate::{XXXL_PROGRAM_ID_PLACEHOLDER, XXXL_RUNTIME_STATUS};
- programs/xxxl-svm/src/deployment_status.rs:L560:             XXXL_PROGRAM_ID_PLACEHOLDER,
- programs/xxxl-svm/src/deployment_status.rs:L561:             "XXXLProgram111111111111111111111111111111111"
- programs/xxxl-svm/src/lib.rs:L33: pub const XXXL_PROGRAM_ID_PLACEHOLDER: &str = "XXXLProgram111111111111111111111111111111111";
- programs/xxxl-svm/src/pda.rs:L367:         assert_ne!(candidate, crate::XXXL_PROGRAM_ID_PLACEHOLDER);
- programs/xxxl-svm/src/program_id_status.rs:L3:     XXXL_PROGRAM_ID_PLACEHOLDER,
- programs/xxxl-svm/src/program_id_status.rs:L29:         status_code: "PLACEHOLDER_PROGRAM_ID_BOUNDARY",
- programs/xxxl-svm/src/program_id_status.rs:L31:         configured_program_id: XXXL_PROGRAM_ID_PLACEHOLDER,
- programs/xxxl-svm/src/program_id_status.rs:L41:             XxxlProgramIdReadinessStatus::Placeholder => "PLACEHOLDER_PROGRAM_ID_BOUNDARY",
- programs/xxxl-svm/src/program_id_status.rs:L62: pub fn xxxl_program_id_placeholder_boundary_is_active() -> bool {
- programs/xxxl-svm/src/program_id_status.rs:L73: pub fn xxxl_program_id_placeholder_blocker_is_active_in_deployment_report() -> bool {
- programs/xxxl-svm/src/program_id_status.rs:L86:         assert_eq!(status.code(), "PLACEHOLDER_PROGRAM_ID_BOUNDARY");
- programs/xxxl-svm/src/program_id_status.rs:L98:         assert_eq!(report.status_code, "PLACEHOLDER_PROGRAM_ID_BOUNDARY");
- programs/xxxl-svm/src/program_id_status.rs:L103:         assert_eq!(report.configured_program_id, XXXL_PROGRAM_ID_PLACEHOLDER);
- programs/xxxl-svm/src/program_id_status.rs:L122:         assert!(xxxl_program_id_placeholder_blocker_is_active_in_deployment_report());
- programs/xxxl-svm/src/program_id_status.rs:L127:         assert!(xxxl_program_id_placeholder_boundary_is_active());
- programs/xxxl-svm/src/safety_invariants.rs:L11:         xxxl_program_id_placeholder_blocker_is_active_in_deployment_report,
- programs/xxxl-svm/src/safety_invariants.rs:L12:         xxxl_program_id_placeholder_boundary_is_active,
- programs/xxxl-svm/src/safety_invariants.rs:L41:         program_id_placeholder_boundary_active: xxxl_program_id_placeholder_boundary_is_active(),
- programs/xxxl-svm/src/safety_invariants.rs:L43:             xxxl_program_id_placeholder_blocker_is_active_in_deployment_report(),

## Placeholder references near structural terms

- programs/xxxl-svm/src/deployment_status.rs:L272: structural_window_detected=true:     use crate::{XXXL_PROGRAM_ID_PLACEHOLDER, XXXL_RUNTIME_STATUS};
- programs/xxxl-svm/src/deployment_status.rs:L560: structural_window_detected=true:             XXXL_PROGRAM_ID_PLACEHOLDER,
- programs/xxxl-svm/src/deployment_status.rs:L561: structural_window_detected=true:             "XXXLProgram111111111111111111111111111111111"
- programs/xxxl-svm/src/lib.rs:L33: structural_window_detected=true: pub const XXXL_PROGRAM_ID_PLACEHOLDER: &str = "XXXLProgram111111111111111111111111111111111";
- programs/xxxl-svm/src/pda.rs:L367: structural_window_detected=true:         assert_ne!(candidate, crate::XXXL_PROGRAM_ID_PLACEHOLDER);
- programs/xxxl-svm/src/program_id_status.rs:L3: structural_window_detected=true:     XXXL_PROGRAM_ID_PLACEHOLDER,
- programs/xxxl-svm/src/program_id_status.rs:L29: structural_window_detected=true:         status_code: "PLACEHOLDER_PROGRAM_ID_BOUNDARY",
- programs/xxxl-svm/src/program_id_status.rs:L31: structural_window_detected=true:         configured_program_id: XXXL_PROGRAM_ID_PLACEHOLDER,
- programs/xxxl-svm/src/program_id_status.rs:L41: structural_window_detected=true:             XxxlProgramIdReadinessStatus::Placeholder => "PLACEHOLDER_PROGRAM_ID_BOUNDARY",
- programs/xxxl-svm/src/program_id_status.rs:L86: structural_window_detected=true:         assert_eq!(status.code(), "PLACEHOLDER_PROGRAM_ID_BOUNDARY");
- programs/xxxl-svm/src/program_id_status.rs:L98: structural_window_detected=true:         assert_eq!(report.status_code, "PLACEHOLDER_PROGRAM_ID_BOUNDARY");
- programs/xxxl-svm/src/program_id_status.rs:L103: structural_window_detected=true:         assert_eq!(report.configured_program_id, XXXL_PROGRAM_ID_PLACEHOLDER);

## PDA and derivation references

- programs/xxxl-svm/src/cpi.rs:L815:         assert_eq!(seeds[1], b"gateway-mint-authority");
- programs/xxxl-svm/src/execution_plan.rs:L5: // already-initialized program-owned processed-event account.
- programs/xxxl-svm/src/pda.rs:L4: pub const GATEWAY_MINT_AUTHORITY_SEED_1: &[u8] = b"gateway-mint-authority";
- programs/xxxl-svm/src/pda.rs:L63:     Pubkey::find_program_address(&gateway_mint_authority_seeds(), program_id)
- programs/xxxl-svm/src/pda.rs:L146:         assert_eq!(seeds[1], b"gateway-mint-authority");
- programs/xxxl-svm/src/pda.rs:L313:     fn gateway_mint_authority_uses_real_find_program_address() {
- programs/xxxl-svm/src/pda.rs:L317:             Pubkey::find_program_address(&gateway_mint_authority_seeds(), &program_id);
- programs/xxxl-svm/src/processed_event_marking_boundary.rs:L36:     pub uses_canonical_find_program_address_bump: bool,
- programs/xxxl-svm/src/processed_event_marking_boundary.rs:L60:     uses_canonical_find_program_address_bump: true,
- programs/xxxl-svm/src/processed_event_marking_boundary.rs:L434:         assert!(report.uses_canonical_find_program_address_bump);
- programs/xxxl-svm/src/processed_event_marking_boundary.rs:L453:         assert_eq!(seeds[1], b"processed-event");
- programs/xxxl-svm/src/processor.rs:L63:     feature = "phase-41k6-b1b-guardian-set-loading-test-gate",
- programs/xxxl-svm/src/processor.rs:L64:     not(feature = "dangerously-allow-phase-41k6-b1b-guardian-set-loading-test-gate-sbf-build")
- programs/xxxl-svm/src/processor.rs:L67:     "phase-41k6-b1b-guardian-set-loading-test-gate introduces B1B authoritative guardian set loading. It is a non-production integration gate and must never be included in deploy artifacts without the explicit dangerous test allow feature."
- programs/xxxl-svm/src/processor.rs:L70: #[cfg(feature = "phase-41k6-b1b-guardian-set-loading-test-gate")]
- programs/xxxl-svm/src/processor.rs:L129: #[cfg(feature = "phase-41k6-b1b-guardian-set-loading-test-gate")]
- programs/xxxl-svm/src/processor.rs:L148: #[cfg(feature = "phase-41k6-b1b-guardian-set-loading-test-gate")]
- programs/xxxl-svm/src/processor.rs:L796:     #[cfg(feature = "phase-41k6-b1b-guardian-set-loading-test-gate")]
- programs/xxxl-svm/src/processor.rs:L1756:     #[cfg(feature = "phase-41k6-b1b-guardian-set-loading-test-gate")]
- programs/xxxl-svm/src/processor.rs:L1789:     #[cfg(feature = "phase-41k6-b1b-guardian-set-loading-test-gate")]
- programs/xxxl-svm/src/processor.rs:L1799:     #[cfg(feature = "phase-41k6-b1b-guardian-set-loading-test-gate")]
- programs/xxxl-svm/src/processor.rs:L1809:     #[cfg(feature = "phase-41k6-b1b-guardian-set-loading-test-gate")]
- programs/xxxl-svm/src/processor.rs:L1821:     #[cfg(feature = "phase-41k6-b1b-guardian-set-loading-test-gate")]
- programs/xxxl-svm/src/processor.rs:L1833:     #[cfg(feature = "phase-41k6-b1b-guardian-set-loading-test-gate")]
- programs/xxxl-svm/src/processor.rs:L1843:     #[cfg(feature = "phase-41k6-b1b-guardian-set-loading-test-gate")]
- programs/xxxl-svm/src/processor.rs:L1859:     #[cfg(feature = "phase-41k6-b1b-guardian-set-loading-test-gate")]
- programs/xxxl-svm/src/processor.rs:L1873:     #[cfg(feature = "phase-41k6-b1b-guardian-set-loading-test-gate")]
- programs/xxxl-svm/src/processor.rs:L1895:     #[cfg(feature = "phase-41k6-b1b-guardian-set-loading-test-gate")]
- programs/xxxl-svm/src/state.rs:L197: // Assumes an already-initialized processed-event account exists with
- programs/xxxl-svm/src/state.rs:L207: #[deprecated(note = "Use the Phase 41K.4 processed-event marking boundary instead")]
- programs/xxxl-svm/src/state.rs:L328:             "41K.4/marking modules must not call legacy processed-event helper: {offenders:?}"

## Upgrade GO blockers imported from strategy assessment

All of the following must close before any testnet upgrade GO:

- A: upgrade authority custody map
- B: expected post-upgrade ProgramData hash
- C: B1C7 handler presence verification
- D: state initialization instruction design
- E: SPL mint authority architecture
- F: guardian set testnet descriptor
- G: rollback or recovery plan
- H: local validator dry-run

None of these blockers are closed by this document.

## Upgrade authority custody map status

- upgrade_authority_public_address: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc
- custody_owner: unknown
- custody_mode: unknown
- key_material_recorded: false
- keypair_path_recorded: false
- signing_procedure_approved: false
- status: open

This document records only the public address already present in prior evidence.

It does not record, request, expose, or approve any secret material.

## Expected ProgramData hash status

- expected_post_upgrade_programdata_hash: unknown
- local_upgrade_artifact_hash: unknown
- local_upgrade_artifact_path: unknown
- status: open

Expected ProgramData hash cannot be closed until a deployable upgrade artifact exists and is hashed.

## Manual resolution update

The automated structural-risk result in this document is superseded by manual review in:

docs/gateway/phase-41k6-b6-7-placeholder-boundary-manual-resolution.md

Manual review found that the placeholder reference in pda.rs is a safety assertion in an ignored candidate dry-run test, not a PDA derivation constant.

Manual conclusion:

- placeholder_boundary_is_readiness_blocker_not_structural_pda_constant
- Strategy 1: closed_not_viable
- Strategy 2: viable_for_planning
- Strategy 3: fallback_if_later_structural_blocker_is_found

Current decision remains:

NO-GO.

## Current decision

Current decision:

NO-GO.

This B6.7 placeholder boundary analysis does not authorize live action.
