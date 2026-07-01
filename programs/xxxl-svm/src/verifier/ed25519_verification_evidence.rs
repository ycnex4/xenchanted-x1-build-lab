use super::{
    ScannedEd25519InstructionEvidence, CANONICAL_PAYLOAD_HASH_VALIDATOR_PHASE_34,
    ED25519_INSTRUCTION_DATA_PARSER_PHASE_38, ED25519_INSTRUCTION_EVIDENCE_LAYOUT_MODEL_PHASE_37,
    GUARDIAN_QUORUM_STRUCTURAL_VERIFIER_PHASE_35, INSTRUCTIONS_SYSVAR_EVIDENCE_SCANNER_PHASE_39,
};

pub const ED25519_VERIFICATION_EVIDENCE_MODEL_PHASE_40B: &str =
    "ED25519_VERIFICATION_EVIDENCE_MODEL_PHASE_40B";
pub const ED25519_VERIFICATION_EVIDENCE_MODEL_VERSION: u8 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ed25519VerificationEvidenceBoundaryResult {
    pub matched_instruction_index: usize,
    pub located_candidate_evidence_present: bool,
    pub parsed_candidate_evidence_present: bool,
    pub public_key_matches_expected_guardian: bool,
    pub message_matches_expected_phase_34_hash: bool,
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
pub enum Ed25519VerificationEvidenceBoundaryErrorKind {
    CandidatePublicKeyNotMatched,
    CandidateMessageHashNotMatched,
    CandidateClaimsInstructionsSysvarRead,
    CandidateClaimsSignatureVerification,
    CandidateClaimsCryptographicProof,
    CandidateClaimsQuorum,
    CandidateClaimsAuthorization,
    CandidateClaimsExecutionSurface,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ed25519VerificationEvidenceBoundaryError {
    pub kind: Ed25519VerificationEvidenceBoundaryErrorKind,
    pub matched_instruction_index: Option<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ed25519VerificationEvidenceModelReport {
    pub model_id: &'static str,
    pub model_version: u8,
    pub phase_39_scanner_required: bool,
    pub phase_39_scanner_phase: &'static str,
    pub phase_38_parser_required: bool,
    pub phase_38_parser_phase: &'static str,
    pub phase_37_layout_model_required: bool,
    pub phase_37_layout_model_phase: &'static str,
    pub phase_34_hash_validator_available: bool,
    pub phase_34_hash_validator_phase: &'static str,
    pub phase_35_quorum_phase: &'static str,
    pub phase_35_quorum_separate_and_not_counted: bool,
    pub located_candidate_evidence_supported: bool,
    pub parsed_candidate_evidence_supported: bool,
    pub verification_evidence_acceptance_enabled: bool,
    pub ed25519_signature_verification_enabled: bool,
    pub cryptographic_signature_proof_accepted: bool,
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

pub const ED25519_VERIFICATION_EVIDENCE_MODEL_REPORT: Ed25519VerificationEvidenceModelReport =
    Ed25519VerificationEvidenceModelReport {
        model_id: ED25519_VERIFICATION_EVIDENCE_MODEL_PHASE_40B,
        model_version: ED25519_VERIFICATION_EVIDENCE_MODEL_VERSION,
        phase_39_scanner_required: true,
        phase_39_scanner_phase: INSTRUCTIONS_SYSVAR_EVIDENCE_SCANNER_PHASE_39,
        phase_38_parser_required: true,
        phase_38_parser_phase: ED25519_INSTRUCTION_DATA_PARSER_PHASE_38,
        phase_37_layout_model_required: true,
        phase_37_layout_model_phase: ED25519_INSTRUCTION_EVIDENCE_LAYOUT_MODEL_PHASE_37,
        phase_34_hash_validator_available: true,
        phase_34_hash_validator_phase: CANONICAL_PAYLOAD_HASH_VALIDATOR_PHASE_34,
        phase_35_quorum_phase: GUARDIAN_QUORUM_STRUCTURAL_VERIFIER_PHASE_35,
        phase_35_quorum_separate_and_not_counted: true,
        located_candidate_evidence_supported: true,
        parsed_candidate_evidence_supported: true,
        verification_evidence_acceptance_enabled: false,
        ed25519_signature_verification_enabled: false,
        cryptographic_signature_proof_accepted: false,
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

pub fn model_ed25519_verification_evidence_boundary(
    scanned_evidence: ScannedEd25519InstructionEvidence,
) -> Result<Ed25519VerificationEvidenceBoundaryResult, Ed25519VerificationEvidenceBoundaryError> {
    let parsed_evidence = scanned_evidence.parsed_ed25519_instruction_evidence;
    let matched_instruction_index = Some(scanned_evidence.matched_instruction_index);

    if !scanned_evidence.public_key_matches_expected_guardian
        || !parsed_evidence.public_key_matches_expected_guardian
    {
        return Err(error(
            Ed25519VerificationEvidenceBoundaryErrorKind::CandidatePublicKeyNotMatched,
            matched_instruction_index,
        ));
    }

    if !scanned_evidence.message_matches_expected_phase_34_hash
        || !parsed_evidence.message_matches_expected_phase_34_hash
    {
        return Err(error(
            Ed25519VerificationEvidenceBoundaryErrorKind::CandidateMessageHashNotMatched,
            matched_instruction_index,
        ));
    }

    if parsed_evidence.instructions_sysvar_read {
        return Err(error(
            Ed25519VerificationEvidenceBoundaryErrorKind::CandidateClaimsInstructionsSysvarRead,
            matched_instruction_index,
        ));
    }

    if scanned_evidence.ed25519_signature_verification_performed
        || parsed_evidence.ed25519_signature_verification_performed
    {
        return Err(error(
            Ed25519VerificationEvidenceBoundaryErrorKind::CandidateClaimsSignatureVerification,
            matched_instruction_index,
        ));
    }

    if scanned_evidence.cryptographic_signature_proof_accepted
        || parsed_evidence.cryptographic_signature_proof_accepted
    {
        return Err(error(
            Ed25519VerificationEvidenceBoundaryErrorKind::CandidateClaimsCryptographicProof,
            matched_instruction_index,
        ));
    }

    if scanned_evidence.quorum_counted || parsed_evidence.quorum_counted {
        return Err(error(
            Ed25519VerificationEvidenceBoundaryErrorKind::CandidateClaimsQuorum,
            matched_instruction_index,
        ));
    }

    if scanned_evidence.authorization_granted || parsed_evidence.authorization_granted {
        return Err(error(
            Ed25519VerificationEvidenceBoundaryErrorKind::CandidateClaimsAuthorization,
            matched_instruction_index,
        ));
    }

    if scanned_evidence.live_route_enabled
        || scanned_evidence.spl_cpi_enabled
        || scanned_evidence.invoke_signed_enabled
        || scanned_evidence.mint_execution_enabled
        || scanned_evidence.runtime_state_mutation_enabled
        || scanned_evidence.replay_write_enabled
        || scanned_evidence.processed_event_marking_enabled
    {
        return Err(error(
            Ed25519VerificationEvidenceBoundaryErrorKind::CandidateClaimsExecutionSurface,
            matched_instruction_index,
        ));
    }

    Ok(Ed25519VerificationEvidenceBoundaryResult {
        matched_instruction_index: scanned_evidence.matched_instruction_index,
        located_candidate_evidence_present: true,
        parsed_candidate_evidence_present: true,
        public_key_matches_expected_guardian: true,
        message_matches_expected_phase_34_hash: true,
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

pub fn ed25519_verification_evidence_model_report(
) -> &'static Ed25519VerificationEvidenceModelReport {
    &ED25519_VERIFICATION_EVIDENCE_MODEL_REPORT
}

fn error(
    kind: Ed25519VerificationEvidenceBoundaryErrorKind,
    matched_instruction_index: Option<usize>,
) -> Ed25519VerificationEvidenceBoundaryError {
    Ed25519VerificationEvidenceBoundaryError {
        kind,
        matched_instruction_index,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::verifier::{
        canonical_payload_hash_validation_report, ed25519_evidence_layout_model_report,
        ed25519_instruction_data_parser_report, guardian_quorum_structural_report,
        instructions_sysvar_evidence_scanner_report, read_only_verifier_boundary,
        GuardianPublicKey, ParsedEd25519InstructionEvidence, ED25519_SIGNATURE_LEN,
        EXPECTED_MESSAGE_LEN, READ_ONLY_RUNTIME_VERIFIER_SCAFFOLD_PHASE_32,
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

    fn scanned_evidence() -> ScannedEd25519InstructionEvidence {
        ScannedEd25519InstructionEvidence {
            matched_instruction_index: 2,
            parsed_ed25519_instruction_evidence: parsed_evidence(),
            scanned_instruction_count: 4,
            ed25519_candidate_count: 1,
            non_ed25519_instruction_count: 3,
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
        result: Result<
            Ed25519VerificationEvidenceBoundaryResult,
            Ed25519VerificationEvidenceBoundaryError,
        >,
        kind: Ed25519VerificationEvidenceBoundaryErrorKind,
    ) -> Ed25519VerificationEvidenceBoundaryError {
        let error = result.expect_err("verification evidence boundary error");
        assert_eq!(error.kind, kind);
        error
    }

    #[test]
    fn marker_and_report_are_stable() {
        let report = ed25519_verification_evidence_model_report();

        assert_eq!(
            report.model_id,
            ED25519_VERIFICATION_EVIDENCE_MODEL_PHASE_40B
        );
        assert_eq!(
            report.model_version,
            ED25519_VERIFICATION_EVIDENCE_MODEL_VERSION
        );
        assert!(report.located_candidate_evidence_supported);
        assert!(report.parsed_candidate_evidence_supported);
        assert!(!report.verification_evidence_acceptance_enabled);
    }

    #[test]
    fn clean_scanned_candidate_maps_to_non_authorizing_boundary_result() {
        let result = model_ed25519_verification_evidence_boundary(scanned_evidence())
            .expect("clean scanned candidate evidence");

        assert_eq!(result.matched_instruction_index, 2);
        assert!(result.located_candidate_evidence_present);
        assert!(result.parsed_candidate_evidence_present);
        assert!(result.public_key_matches_expected_guardian);
        assert!(result.message_matches_expected_phase_34_hash);
        assert!(!result.ed25519_signature_verification_performed);
        assert!(!result.cryptographic_signature_proof_accepted);
        assert!(!result.verification_evidence_accepted);
        assert!(!result.quorum_counted);
        assert!(!result.authorization_granted);
    }

    #[test]
    fn candidate_public_key_mismatch_is_rejected() {
        let mut evidence = scanned_evidence();
        evidence.public_key_matches_expected_guardian = false;

        assert_error_kind(
            model_ed25519_verification_evidence_boundary(evidence),
            Ed25519VerificationEvidenceBoundaryErrorKind::CandidatePublicKeyNotMatched,
        );
    }

    #[test]
    fn parsed_public_key_mismatch_is_rejected() {
        let mut evidence = scanned_evidence();
        evidence
            .parsed_ed25519_instruction_evidence
            .public_key_matches_expected_guardian = false;

        assert_error_kind(
            model_ed25519_verification_evidence_boundary(evidence),
            Ed25519VerificationEvidenceBoundaryErrorKind::CandidatePublicKeyNotMatched,
        );
    }

    #[test]
    fn candidate_message_hash_mismatch_is_rejected() {
        let mut evidence = scanned_evidence();
        evidence.message_matches_expected_phase_34_hash = false;

        assert_error_kind(
            model_ed25519_verification_evidence_boundary(evidence),
            Ed25519VerificationEvidenceBoundaryErrorKind::CandidateMessageHashNotMatched,
        );
    }

    #[test]
    fn parsed_message_hash_mismatch_is_rejected() {
        let mut evidence = scanned_evidence();
        evidence
            .parsed_ed25519_instruction_evidence
            .message_matches_expected_phase_34_hash = false;

        assert_error_kind(
            model_ed25519_verification_evidence_boundary(evidence),
            Ed25519VerificationEvidenceBoundaryErrorKind::CandidateMessageHashNotMatched,
        );
    }

    #[test]
    fn parsed_instruction_sysvar_claim_is_rejected() {
        let mut evidence = scanned_evidence();
        evidence
            .parsed_ed25519_instruction_evidence
            .instructions_sysvar_read = true;

        assert_error_kind(
            model_ed25519_verification_evidence_boundary(evidence),
            Ed25519VerificationEvidenceBoundaryErrorKind::CandidateClaimsInstructionsSysvarRead,
        );
    }

    #[test]
    fn signature_verification_claim_is_rejected() {
        let mut evidence = scanned_evidence();
        evidence.ed25519_signature_verification_performed = true;

        assert_error_kind(
            model_ed25519_verification_evidence_boundary(evidence),
            Ed25519VerificationEvidenceBoundaryErrorKind::CandidateClaimsSignatureVerification,
        );
    }

    #[test]
    fn parsed_signature_verification_claim_is_rejected() {
        let mut evidence = scanned_evidence();
        evidence
            .parsed_ed25519_instruction_evidence
            .ed25519_signature_verification_performed = true;

        assert_error_kind(
            model_ed25519_verification_evidence_boundary(evidence),
            Ed25519VerificationEvidenceBoundaryErrorKind::CandidateClaimsSignatureVerification,
        );
    }

    #[test]
    fn cryptographic_proof_claim_is_rejected() {
        let mut evidence = scanned_evidence();
        evidence.cryptographic_signature_proof_accepted = true;

        assert_error_kind(
            model_ed25519_verification_evidence_boundary(evidence),
            Ed25519VerificationEvidenceBoundaryErrorKind::CandidateClaimsCryptographicProof,
        );
    }

    #[test]
    fn quorum_claim_is_rejected() {
        let mut evidence = scanned_evidence();
        evidence.quorum_counted = true;

        assert_error_kind(
            model_ed25519_verification_evidence_boundary(evidence),
            Ed25519VerificationEvidenceBoundaryErrorKind::CandidateClaimsQuorum,
        );
    }

    #[test]
    fn authorization_claim_is_rejected() {
        let mut evidence = scanned_evidence();
        evidence.authorization_granted = true;

        assert_error_kind(
            model_ed25519_verification_evidence_boundary(evidence),
            Ed25519VerificationEvidenceBoundaryErrorKind::CandidateClaimsAuthorization,
        );
    }

    #[test]
    fn execution_surface_claim_is_rejected() {
        let mut evidence = scanned_evidence();
        evidence.mint_execution_enabled = true;

        assert_error_kind(
            model_ed25519_verification_evidence_boundary(evidence),
            Ed25519VerificationEvidenceBoundaryErrorKind::CandidateClaimsExecutionSurface,
        );
    }

    #[test]
    fn report_preserves_all_disabled_execution_and_security_flags() {
        let report = ed25519_verification_evidence_model_report();

        assert!(!report.verification_evidence_acceptance_enabled);
        assert!(!report.ed25519_signature_verification_enabled);
        assert!(!report.cryptographic_signature_proof_accepted);
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
    fn phase_39_scanner_remains_required() {
        let report = ed25519_verification_evidence_model_report();
        let phase_39_report = instructions_sysvar_evidence_scanner_report();

        assert!(report.phase_39_scanner_required);
        assert_eq!(
            report.phase_39_scanner_phase,
            INSTRUCTIONS_SYSVAR_EVIDENCE_SCANNER_PHASE_39
        );
        assert_eq!(
            phase_39_report.scanner_id,
            INSTRUCTIONS_SYSVAR_EVIDENCE_SCANNER_PHASE_39
        );
        assert!(phase_39_report.scans_prepared_read_only_instruction_entries);
    }

    #[test]
    fn phase_38_parser_remains_required() {
        let report = ed25519_verification_evidence_model_report();
        let phase_38_report = ed25519_instruction_data_parser_report();

        assert!(report.phase_38_parser_required);
        assert_eq!(
            report.phase_38_parser_phase,
            ED25519_INSTRUCTION_DATA_PARSER_PHASE_38
        );
        assert_eq!(
            phase_38_report.parser_id,
            ED25519_INSTRUCTION_DATA_PARSER_PHASE_38
        );
        assert!(phase_38_report.actual_instruction_data_bytes_parsed);
    }

    #[test]
    fn phase_37_layout_model_remains_required() {
        let report = ed25519_verification_evidence_model_report();
        let phase_37_report = ed25519_evidence_layout_model_report();

        assert!(report.phase_37_layout_model_required);
        assert_eq!(
            report.phase_37_layout_model_phase,
            ED25519_INSTRUCTION_EVIDENCE_LAYOUT_MODEL_PHASE_37
        );
        assert_eq!(
            phase_37_report.model_id,
            ED25519_INSTRUCTION_EVIDENCE_LAYOUT_MODEL_PHASE_37
        );
    }

    #[test]
    fn phase_34_hash_validator_remains_available() {
        let report = ed25519_verification_evidence_model_report();
        let phase_34_report = canonical_payload_hash_validation_report();

        assert!(report.phase_34_hash_validator_available);
        assert_eq!(
            report.phase_34_hash_validator_phase,
            CANONICAL_PAYLOAD_HASH_VALIDATOR_PHASE_34
        );
        assert!(phase_34_report.recomputes_hash_from_payload_bytes);
        assert!(!phase_34_report.caller_provided_payload_hash_trusted);
    }

    #[test]
    fn phase_35_quorum_remains_separate_and_not_counted() {
        let report = ed25519_verification_evidence_model_report();
        let phase_35_report = guardian_quorum_structural_report();

        assert_eq!(
            report.phase_35_quorum_phase,
            GUARDIAN_QUORUM_STRUCTURAL_VERIFIER_PHASE_35
        );
        assert!(report.phase_35_quorum_separate_and_not_counted);
        assert!(!report.quorum_counting_enabled);
        assert!(phase_35_report.quorum_threshold_check_enabled);
        assert!(!phase_35_report.ed25519_signature_verification_enabled);
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
