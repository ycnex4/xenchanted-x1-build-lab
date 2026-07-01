use super::{
    ScannedEd25519InstructionEvidence, ED25519_VERIFICATION_EVIDENCE_INTEGRATION_DESIGN_PHASE_40D,
    ED25519_VERIFICATION_EVIDENCE_MODEL_PHASE_40B, INSTRUCTIONS_SYSVAR_EVIDENCE_SCANNER_PHASE_39,
};

pub const ED25519_PRIOR_INSTRUCTION_ORDERING_MODEL_PHASE_40E: &str =
    "ED25519_PRIOR_INSTRUCTION_ORDERING_MODEL_PHASE_40E";
pub const ED25519_PRIOR_INSTRUCTION_ORDERING_MODEL_VERSION: u8 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ed25519PriorInstructionOrderingErrorKind {
    ZeroInstructionCount,
    CurrentInstructionIndexOutOfBounds,
    MatchedEd25519InstructionIndexOutOfBounds,
    Ed25519InstructionIsCurrentInstruction,
    Ed25519InstructionAfterCurrentInstruction,
    CandidateClaimsInstructionsSysvarRead,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ed25519PriorInstructionOrderingError {
    pub kind: Ed25519PriorInstructionOrderingErrorKind,
    pub scanned_instruction_count: usize,
    pub current_instruction_index: usize,
    pub matched_ed25519_instruction_index: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ed25519PriorInstructionOrderingResult {
    pub scanned_instruction_count: usize,
    pub current_instruction_index: usize,
    pub matched_ed25519_instruction_index: usize,
    pub ed25519_instruction_precedes_current_instruction: bool,
    pub current_instruction_identity_modeled_only: bool,
    pub raw_instructions_sysvar_parser_implemented: bool,
    pub account_info_parser_implemented: bool,
    pub load_instruction_called: bool,
    pub ed25519_signature_verification_performed: bool,
    pub cryptographic_signature_proof_accepted: bool,
    pub verification_evidence_accepted: bool,
    pub quorum_counted: bool,
    pub authorization_granted: bool,
    pub live_route_enabled: bool,
    pub spl_cpi_enabled: bool,
    pub invoke_signed_enabled: bool,
    pub mint_execution_enabled: bool,
    pub runtime_state_mutation_enabled: bool,
    pub replay_write_enabled: bool,
    pub processed_event_marking_enabled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ed25519PriorInstructionOrderingModelReport {
    pub model_id: &'static str,
    pub model_version: u8,
    pub phase_39_scanner_required: bool,
    pub phase_39_scanner_phase: &'static str,
    pub phase_40b_verification_evidence_boundary_required: bool,
    pub phase_40b_verification_evidence_boundary_phase: &'static str,
    pub phase_40d_integration_design_required: bool,
    pub phase_40d_integration_design_phase: &'static str,
    pub prior_instruction_ordering_required: bool,
    pub current_instruction_identity_required_future: bool,
    pub current_instruction_identity_modeled_only: bool,
    pub raw_instructions_sysvar_parser_implemented: bool,
    pub account_info_parser_implemented: bool,
    pub load_instruction_called: bool,
    pub ed25519_signature_verification_performed: bool,
    pub cryptographic_signature_proof_accepted: bool,
    pub verification_evidence_accepted: bool,
    pub quorum_counting_enabled: bool,
    pub authorization_enabled: bool,
    pub live_route_enabled: bool,
    pub spl_cpi_enabled: bool,
    pub invoke_signed_enabled: bool,
    pub mint_execution_enabled: bool,
    pub runtime_state_mutation_enabled: bool,
    pub replay_write_enabled: bool,
    pub processed_event_marking_enabled: bool,
    pub production_program_id_selected: bool,
    pub deployment_blockers_removed: bool,
}

pub const ED25519_PRIOR_INSTRUCTION_ORDERING_MODEL_REPORT:
    Ed25519PriorInstructionOrderingModelReport = Ed25519PriorInstructionOrderingModelReport {
    model_id: ED25519_PRIOR_INSTRUCTION_ORDERING_MODEL_PHASE_40E,
    model_version: ED25519_PRIOR_INSTRUCTION_ORDERING_MODEL_VERSION,
    phase_39_scanner_required: true,
    phase_39_scanner_phase: INSTRUCTIONS_SYSVAR_EVIDENCE_SCANNER_PHASE_39,
    phase_40b_verification_evidence_boundary_required: true,
    phase_40b_verification_evidence_boundary_phase: ED25519_VERIFICATION_EVIDENCE_MODEL_PHASE_40B,
    phase_40d_integration_design_required: true,
    phase_40d_integration_design_phase: ED25519_VERIFICATION_EVIDENCE_INTEGRATION_DESIGN_PHASE_40D,
    prior_instruction_ordering_required: true,
    current_instruction_identity_required_future: true,
    current_instruction_identity_modeled_only: true,
    raw_instructions_sysvar_parser_implemented: false,
    account_info_parser_implemented: false,
    load_instruction_called: false,
    ed25519_signature_verification_performed: false,
    cryptographic_signature_proof_accepted: false,
    verification_evidence_accepted: false,
    quorum_counting_enabled: false,
    authorization_enabled: false,
    live_route_enabled: false,
    spl_cpi_enabled: false,
    invoke_signed_enabled: false,
    mint_execution_enabled: false,
    runtime_state_mutation_enabled: false,
    replay_write_enabled: false,
    processed_event_marking_enabled: false,
    production_program_id_selected: false,
    deployment_blockers_removed: false,
};

pub fn model_ed25519_prior_instruction_ordering(
    scanned_evidence: &ScannedEd25519InstructionEvidence,
    current_instruction_index: usize,
) -> Result<Ed25519PriorInstructionOrderingResult, Ed25519PriorInstructionOrderingError> {
    let scanned_instruction_count = scanned_evidence.scanned_instruction_count;
    let matched_ed25519_instruction_index = scanned_evidence.matched_instruction_index;

    if scanned_instruction_count == 0 {
        return Err(error(
            Ed25519PriorInstructionOrderingErrorKind::ZeroInstructionCount,
            scanned_instruction_count,
            current_instruction_index,
            matched_ed25519_instruction_index,
        ));
    }

    if current_instruction_index >= scanned_instruction_count {
        return Err(error(
            Ed25519PriorInstructionOrderingErrorKind::CurrentInstructionIndexOutOfBounds,
            scanned_instruction_count,
            current_instruction_index,
            matched_ed25519_instruction_index,
        ));
    }

    if matched_ed25519_instruction_index >= scanned_instruction_count {
        return Err(error(
            Ed25519PriorInstructionOrderingErrorKind::MatchedEd25519InstructionIndexOutOfBounds,
            scanned_instruction_count,
            current_instruction_index,
            matched_ed25519_instruction_index,
        ));
    }

    if matched_ed25519_instruction_index == current_instruction_index {
        return Err(error(
            Ed25519PriorInstructionOrderingErrorKind::Ed25519InstructionIsCurrentInstruction,
            scanned_instruction_count,
            current_instruction_index,
            matched_ed25519_instruction_index,
        ));
    }

    if matched_ed25519_instruction_index > current_instruction_index {
        return Err(error(
            Ed25519PriorInstructionOrderingErrorKind::Ed25519InstructionAfterCurrentInstruction,
            scanned_instruction_count,
            current_instruction_index,
            matched_ed25519_instruction_index,
        ));
    }

    if scanned_evidence
        .parsed_ed25519_instruction_evidence
        .instructions_sysvar_read
    {
        return Err(error(
            Ed25519PriorInstructionOrderingErrorKind::CandidateClaimsInstructionsSysvarRead,
            scanned_instruction_count,
            current_instruction_index,
            matched_ed25519_instruction_index,
        ));
    }

    Ok(Ed25519PriorInstructionOrderingResult {
        scanned_instruction_count,
        current_instruction_index,
        matched_ed25519_instruction_index,
        ed25519_instruction_precedes_current_instruction: true,
        current_instruction_identity_modeled_only: true,
        raw_instructions_sysvar_parser_implemented: false,
        account_info_parser_implemented: false,
        load_instruction_called: false,
        ed25519_signature_verification_performed: false,
        cryptographic_signature_proof_accepted: false,
        verification_evidence_accepted: false,
        quorum_counted: false,
        authorization_granted: false,
        live_route_enabled: false,
        spl_cpi_enabled: false,
        invoke_signed_enabled: false,
        mint_execution_enabled: false,
        runtime_state_mutation_enabled: false,
        replay_write_enabled: false,
        processed_event_marking_enabled: false,
    })
}

pub fn ed25519_prior_instruction_ordering_model_report(
) -> &'static Ed25519PriorInstructionOrderingModelReport {
    &ED25519_PRIOR_INSTRUCTION_ORDERING_MODEL_REPORT
}

fn error(
    kind: Ed25519PriorInstructionOrderingErrorKind,
    scanned_instruction_count: usize,
    current_instruction_index: usize,
    matched_ed25519_instruction_index: usize,
) -> Ed25519PriorInstructionOrderingError {
    Ed25519PriorInstructionOrderingError {
        kind,
        scanned_instruction_count,
        current_instruction_index,
        matched_ed25519_instruction_index,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::verifier::{
        ed25519_verification_evidence_integration_design_report,
        ed25519_verification_evidence_model_report, instructions_sysvar_evidence_scanner_report,
        read_only_verifier_boundary, GuardianPublicKey, ParsedEd25519InstructionEvidence,
        ScannedEd25519InstructionEvidence, ED25519_SIGNATURE_LEN, EXPECTED_MESSAGE_LEN,
        READ_ONLY_RUNTIME_VERIFIER_SCAFFOLD_PHASE_32,
    };

    const EXPECTED_GUARDIAN_PUBLIC_KEY: GuardianPublicKey = GuardianPublicKey([0x31; 32]);
    const SIGNATURE_BYTES: [u8; ED25519_SIGNATURE_LEN] = [0x5a; ED25519_SIGNATURE_LEN];
    const MESSAGE_BYTES: [u8; EXPECTED_MESSAGE_LEN] = [0xab; EXPECTED_MESSAGE_LEN];

    fn parsed_evidence() -> ParsedEd25519InstructionEvidence {
        ParsedEd25519InstructionEvidence {
            signature_bytes: SIGNATURE_BYTES,
            guardian_public_key: EXPECTED_GUARDIAN_PUBLIC_KEY,
            message_bytes: MESSAGE_BYTES,
            public_key_matches_expected_guardian: true,
            message_matches_expected_phase_34_hash: true,
            ed25519_signature_verification_performed: false,
            cryptographic_signature_proof_accepted: false,
            instructions_sysvar_read: false,
            quorum_counted: false,
            authorization_granted: false,
        }
    }

    fn scanned_evidence_with_indices(
        scanned_instruction_count: usize,
        matched_instruction_index: usize,
    ) -> ScannedEd25519InstructionEvidence {
        ScannedEd25519InstructionEvidence {
            matched_instruction_index,
            parsed_ed25519_instruction_evidence: parsed_evidence(),
            scanned_instruction_count,
            ed25519_candidate_count: 1,
            non_ed25519_instruction_count: scanned_instruction_count.saturating_sub(1),
            public_key_matches_expected_guardian: true,
            message_matches_expected_phase_34_hash: true,
            ed25519_signature_verification_performed: false,
            cryptographic_signature_proof_accepted: false,
            quorum_counted: false,
            authorization_granted: false,
            live_route_enabled: false,
            spl_cpi_enabled: false,
            invoke_signed_enabled: false,
            mint_execution_enabled: false,
            runtime_state_mutation_enabled: false,
            replay_write_enabled: false,
            processed_event_marking_enabled: false,
        }
    }

    fn assert_error_kind(
        result: Result<Ed25519PriorInstructionOrderingResult, Ed25519PriorInstructionOrderingError>,
        kind: Ed25519PriorInstructionOrderingErrorKind,
    ) -> Ed25519PriorInstructionOrderingError {
        let error = result.expect_err("prior instruction ordering error");
        assert_eq!(error.kind, kind);
        error
    }

    #[test]
    fn marker_and_report_are_stable() {
        let report = ed25519_prior_instruction_ordering_model_report();

        assert_eq!(
            report.model_id,
            ED25519_PRIOR_INSTRUCTION_ORDERING_MODEL_PHASE_40E
        );
        assert_eq!(
            report.model_version,
            ED25519_PRIOR_INSTRUCTION_ORDERING_MODEL_VERSION
        );
        assert!(report.prior_instruction_ordering_required);
        assert!(report.current_instruction_identity_required_future);
        assert!(report.current_instruction_identity_modeled_only);
    }

    #[test]
    fn ed25519_instruction_before_current_instruction_is_accepted() {
        let evidence = scanned_evidence_with_indices(4, 2);
        let result = model_ed25519_prior_instruction_ordering(&evidence, 3)
            .expect("prior ed25519 instruction");

        assert_eq!(result.scanned_instruction_count, 4);
        assert_eq!(result.matched_ed25519_instruction_index, 2);
        assert_eq!(result.current_instruction_index, 3);
        assert!(result.ed25519_instruction_precedes_current_instruction);
        assert!(result.current_instruction_identity_modeled_only);
    }

    #[test]
    fn first_instruction_before_current_instruction_is_accepted() {
        let evidence = scanned_evidence_with_indices(2, 0);
        let result = model_ed25519_prior_instruction_ordering(&evidence, 1)
            .expect("first instruction is prior");

        assert_eq!(result.matched_ed25519_instruction_index, 0);
        assert_eq!(result.current_instruction_index, 1);
        assert!(result.ed25519_instruction_precedes_current_instruction);
    }

    #[test]
    fn zero_instruction_count_is_rejected() {
        let evidence = scanned_evidence_with_indices(0, 0);

        let error = assert_error_kind(
            model_ed25519_prior_instruction_ordering(&evidence, 0),
            Ed25519PriorInstructionOrderingErrorKind::ZeroInstructionCount,
        );

        assert_eq!(error.scanned_instruction_count, 0);
    }

    #[test]
    fn current_instruction_index_out_of_bounds_is_rejected() {
        let evidence = scanned_evidence_with_indices(4, 2);

        let error = assert_error_kind(
            model_ed25519_prior_instruction_ordering(&evidence, 4),
            Ed25519PriorInstructionOrderingErrorKind::CurrentInstructionIndexOutOfBounds,
        );

        assert_eq!(error.current_instruction_index, 4);
    }

    #[test]
    fn matched_ed25519_instruction_index_out_of_bounds_is_rejected() {
        let evidence = scanned_evidence_with_indices(4, 4);

        let error = assert_error_kind(
            model_ed25519_prior_instruction_ordering(&evidence, 3),
            Ed25519PriorInstructionOrderingErrorKind::MatchedEd25519InstructionIndexOutOfBounds,
        );

        assert_eq!(error.matched_ed25519_instruction_index, 4);
    }

    #[test]
    fn ed25519_instruction_equal_to_current_instruction_is_rejected() {
        let evidence = scanned_evidence_with_indices(4, 2);

        let error = assert_error_kind(
            model_ed25519_prior_instruction_ordering(&evidence, 2),
            Ed25519PriorInstructionOrderingErrorKind::Ed25519InstructionIsCurrentInstruction,
        );

        assert_eq!(error.matched_ed25519_instruction_index, 2);
        assert_eq!(error.current_instruction_index, 2);
    }

    #[test]
    fn ed25519_instruction_after_current_instruction_is_rejected() {
        let evidence = scanned_evidence_with_indices(4, 3);

        let error = assert_error_kind(
            model_ed25519_prior_instruction_ordering(&evidence, 2),
            Ed25519PriorInstructionOrderingErrorKind::Ed25519InstructionAfterCurrentInstruction,
        );

        assert_eq!(error.matched_ed25519_instruction_index, 3);
        assert_eq!(error.current_instruction_index, 2);
    }

    #[test]
    fn parsed_instruction_sysvar_claim_is_rejected() {
        let mut evidence = scanned_evidence_with_indices(4, 2);
        evidence
            .parsed_ed25519_instruction_evidence
            .instructions_sysvar_read = true;

        assert_error_kind(
            model_ed25519_prior_instruction_ordering(&evidence, 3),
            Ed25519PriorInstructionOrderingErrorKind::CandidateClaimsInstructionsSysvarRead,
        );
    }

    #[test]
    fn accepted_ordering_result_is_not_verification_evidence() {
        let evidence = scanned_evidence_with_indices(4, 2);
        let result = model_ed25519_prior_instruction_ordering(&evidence, 3)
            .expect("prior ed25519 instruction");

        assert!(!result.raw_instructions_sysvar_parser_implemented);
        assert!(!result.account_info_parser_implemented);
        assert!(!result.load_instruction_called);
        assert!(!result.ed25519_signature_verification_performed);
        assert!(!result.cryptographic_signature_proof_accepted);
        assert!(!result.verification_evidence_accepted);
        assert!(!result.quorum_counted);
        assert!(!result.authorization_granted);
        assert!(!result.live_route_enabled);
        assert!(!result.spl_cpi_enabled);
        assert!(!result.invoke_signed_enabled);
        assert!(!result.mint_execution_enabled);
        assert!(!result.runtime_state_mutation_enabled);
        assert!(!result.replay_write_enabled);
        assert!(!result.processed_event_marking_enabled);
    }

    #[test]
    fn model_report_preserves_all_disabled_runtime_surfaces() {
        let report = ed25519_prior_instruction_ordering_model_report();

        assert!(!report.raw_instructions_sysvar_parser_implemented);
        assert!(!report.account_info_parser_implemented);
        assert!(!report.load_instruction_called);
        assert!(!report.ed25519_signature_verification_performed);
        assert!(!report.cryptographic_signature_proof_accepted);
        assert!(!report.verification_evidence_accepted);
        assert!(!report.quorum_counting_enabled);
        assert!(!report.authorization_enabled);
        assert!(!report.live_route_enabled);
        assert!(!report.spl_cpi_enabled);
        assert!(!report.invoke_signed_enabled);
        assert!(!report.mint_execution_enabled);
        assert!(!report.runtime_state_mutation_enabled);
        assert!(!report.replay_write_enabled);
        assert!(!report.processed_event_marking_enabled);
        assert!(!report.production_program_id_selected);
        assert!(!report.deployment_blockers_removed);
    }

    #[test]
    fn phase_39_40b_and_40d_remain_required() {
        let report = ed25519_prior_instruction_ordering_model_report();
        let phase_39_report = instructions_sysvar_evidence_scanner_report();
        let phase_40b_report = ed25519_verification_evidence_model_report();
        let phase_40d_report = ed25519_verification_evidence_integration_design_report();

        assert!(report.phase_39_scanner_required);
        assert_eq!(
            report.phase_39_scanner_phase,
            INSTRUCTIONS_SYSVAR_EVIDENCE_SCANNER_PHASE_39
        );
        assert_eq!(
            phase_39_report.scanner_id,
            INSTRUCTIONS_SYSVAR_EVIDENCE_SCANNER_PHASE_39
        );

        assert!(report.phase_40b_verification_evidence_boundary_required);
        assert_eq!(
            report.phase_40b_verification_evidence_boundary_phase,
            ED25519_VERIFICATION_EVIDENCE_MODEL_PHASE_40B
        );
        assert_eq!(
            phase_40b_report.model_id,
            ED25519_VERIFICATION_EVIDENCE_MODEL_PHASE_40B
        );

        assert!(report.phase_40d_integration_design_required);
        assert_eq!(
            report.phase_40d_integration_design_phase,
            ED25519_VERIFICATION_EVIDENCE_INTEGRATION_DESIGN_PHASE_40D
        );
        assert_eq!(
            phase_40d_report.design_id,
            ED25519_VERIFICATION_EVIDENCE_INTEGRATION_DESIGN_PHASE_40D
        );
    }

    #[test]
    fn phase_32_boundary_safety_flags_remain_false() {
        let boundary = read_only_verifier_boundary();

        assert_eq!(
            boundary.scaffold_id,
            READ_ONLY_RUNTIME_VERIFIER_SCAFFOLD_PHASE_32
        );
        assert!(!boundary.execution_enabled());
        assert!(!boundary.deployment_unlocked());
        assert!(!boundary.live_route_enabled);
        assert!(!boundary.spl_cpi_enabled);
        assert!(!boundary.invoke_signed_enabled);
        assert!(!boundary.mint_execution_enabled);
        assert!(!boundary.runtime_state_mutation_enabled);
        assert!(!boundary.replay_write_enabled);
        assert!(!boundary.processed_event_marking_enabled);
    }
}
