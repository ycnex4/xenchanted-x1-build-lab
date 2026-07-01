use super::{
    parse_ed25519_instruction_data_evidence, Ed25519InstructionDataParserError, GuardianPublicKey,
    ParsedEd25519InstructionEvidence, CANONICAL_PAYLOAD_HASH_VALIDATOR_PHASE_34,
    ED25519_INSTRUCTION_DATA_PARSER_PHASE_38, ED25519_INSTRUCTION_EVIDENCE_LAYOUT_MODEL_PHASE_37,
    ED25519_PROGRAM_ID_REFERENCE, EXPECTED_MESSAGE_LEN,
    GUARDIAN_QUORUM_STRUCTURAL_VERIFIER_PHASE_35,
};

pub const INSTRUCTIONS_SYSVAR_EVIDENCE_SCANNER_PHASE_39: &str =
    "INSTRUCTIONS_SYSVAR_EVIDENCE_SCANNER_PHASE_39";
pub const INSTRUCTIONS_SYSVAR_EVIDENCE_SCANNER_VERSION: u8 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InstructionsSysvarInstructionView<'a> {
    pub program_id: &'a str,
    pub instruction_data: &'a [u8],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScannedEd25519InstructionEvidence {
    pub matched_instruction_index: usize,
    pub parsed_ed25519_instruction_evidence: ParsedEd25519InstructionEvidence,
    pub scanned_instruction_count: usize,
    pub ed25519_candidate_count: usize,
    pub non_ed25519_instruction_count: usize,
    pub public_key_matches_expected_guardian: bool,
    pub message_matches_expected_phase_34_hash: bool,
    pub ed25519_signature_verification_performed: bool,
    pub cryptographic_signature_proof_accepted: bool,
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
pub enum InstructionsSysvarEvidenceScannerErrorKind {
    EmptyInstructionSet,
    NoMatchingEd25519Evidence,
    DuplicateMatchingEd25519Evidence,
    CandidateInstructionParseFailed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InstructionsSysvarEvidenceScannerError {
    pub kind: InstructionsSysvarEvidenceScannerErrorKind,
    pub scanned_instruction_count: usize,
    pub ed25519_candidate_count: usize,
    pub non_ed25519_instruction_count: usize,
    pub matching_evidence_count: usize,
    pub failed_candidate_instruction_index: Option<usize>,
    pub first_matching_instruction_index: Option<usize>,
    pub duplicate_matching_instruction_index: Option<usize>,
    pub candidate_parser_error: Option<Ed25519InstructionDataParserError>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InstructionsSysvarEvidenceScannerReport {
    pub scanner_id: &'static str,
    pub scanner_version: u8,
    pub scans_prepared_read_only_instruction_entries: bool,
    pub raw_instructions_sysvar_data_parsed: bool,
    pub account_info_parser_enabled: bool,
    pub identifies_ed25519_program_instruction_candidates: bool,
    pub ed25519_program_id_reference: &'static str,
    pub phase_37_layout_model_required: bool,
    pub phase_37_layout_model_phase: &'static str,
    pub phase_38_instruction_data_parser_required: bool,
    pub phase_38_instruction_data_parser_phase: &'static str,
    pub phase_34_hash_validator_available: bool,
    pub phase_34_hash_validator_phase: &'static str,
    pub phase_34_hash_validator_recomputes_hash: bool,
    pub phase_35_quorum_phase: &'static str,
    pub phase_35_quorum_separate_and_not_counted: bool,
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

pub const INSTRUCTIONS_SYSVAR_EVIDENCE_SCANNER_REPORT: InstructionsSysvarEvidenceScannerReport =
    InstructionsSysvarEvidenceScannerReport {
        scanner_id: INSTRUCTIONS_SYSVAR_EVIDENCE_SCANNER_PHASE_39,
        scanner_version: INSTRUCTIONS_SYSVAR_EVIDENCE_SCANNER_VERSION,
        scans_prepared_read_only_instruction_entries: true,
        raw_instructions_sysvar_data_parsed: false,
        account_info_parser_enabled: false,
        identifies_ed25519_program_instruction_candidates: true,
        ed25519_program_id_reference: ED25519_PROGRAM_ID_REFERENCE,
        phase_37_layout_model_required: true,
        phase_37_layout_model_phase: ED25519_INSTRUCTION_EVIDENCE_LAYOUT_MODEL_PHASE_37,
        phase_38_instruction_data_parser_required: true,
        phase_38_instruction_data_parser_phase: ED25519_INSTRUCTION_DATA_PARSER_PHASE_38,
        phase_34_hash_validator_available: true,
        phase_34_hash_validator_phase: CANONICAL_PAYLOAD_HASH_VALIDATOR_PHASE_34,
        phase_34_hash_validator_recomputes_hash: true,
        phase_35_quorum_phase: GUARDIAN_QUORUM_STRUCTURAL_VERIFIER_PHASE_35,
        phase_35_quorum_separate_and_not_counted: true,
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

pub fn scan_instructions_sysvar_for_ed25519_evidence(
    instructions: &[InstructionsSysvarInstructionView<'_>],
    expected_guardian_public_key: GuardianPublicKey,
    expected_phase_34_payload_hash: &[u8; EXPECTED_MESSAGE_LEN],
) -> Result<ScannedEd25519InstructionEvidence, InstructionsSysvarEvidenceScannerError> {
    if instructions.is_empty() {
        return Err(error(
            InstructionsSysvarEvidenceScannerErrorKind::EmptyInstructionSet,
            0,
            0,
            0,
            0,
            None,
            None,
            None,
            None,
        ));
    }

    let mut ed25519_candidate_count = 0usize;
    let mut non_ed25519_instruction_count = 0usize;
    let mut matching_evidence_count = 0usize;
    let mut first_matching_instruction_index = None;
    let mut duplicate_matching_instruction_index = None;
    let mut matched_evidence = None;
    let mut first_candidate_failure = None;

    for (instruction_index, instruction) in instructions.iter().enumerate() {
        if instruction.program_id != ED25519_PROGRAM_ID_REFERENCE {
            non_ed25519_instruction_count += 1;
            continue;
        }

        ed25519_candidate_count += 1;

        match parse_ed25519_instruction_data_evidence(
            instruction.instruction_data,
            expected_guardian_public_key,
            expected_phase_34_payload_hash,
        ) {
            Ok(parsed_evidence) => {
                matching_evidence_count += 1;

                if first_matching_instruction_index.is_none() {
                    first_matching_instruction_index = Some(instruction_index);
                    matched_evidence = Some(parsed_evidence);
                } else if duplicate_matching_instruction_index.is_none() {
                    duplicate_matching_instruction_index = Some(instruction_index);
                }
            }
            Err(parser_error) => {
                if first_candidate_failure.is_none() {
                    first_candidate_failure = Some((instruction_index, parser_error));
                }
            }
        }
    }

    if matching_evidence_count > 1 {
        return Err(error(
            InstructionsSysvarEvidenceScannerErrorKind::DuplicateMatchingEd25519Evidence,
            instructions.len(),
            ed25519_candidate_count,
            non_ed25519_instruction_count,
            matching_evidence_count,
            None,
            first_matching_instruction_index,
            duplicate_matching_instruction_index,
            None,
        ));
    }

    if let (Some(matched_instruction_index), Some(parsed_ed25519_instruction_evidence)) =
        (first_matching_instruction_index, matched_evidence)
    {
        return Ok(ScannedEd25519InstructionEvidence {
            matched_instruction_index,
            parsed_ed25519_instruction_evidence,
            scanned_instruction_count: instructions.len(),
            ed25519_candidate_count,
            non_ed25519_instruction_count,
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
        });
    }

    if let Some((failed_instruction_index, parser_error)) = first_candidate_failure {
        return Err(error(
            InstructionsSysvarEvidenceScannerErrorKind::CandidateInstructionParseFailed,
            instructions.len(),
            ed25519_candidate_count,
            non_ed25519_instruction_count,
            matching_evidence_count,
            Some(failed_instruction_index),
            None,
            None,
            Some(parser_error),
        ));
    }

    Err(error(
        InstructionsSysvarEvidenceScannerErrorKind::NoMatchingEd25519Evidence,
        instructions.len(),
        ed25519_candidate_count,
        non_ed25519_instruction_count,
        matching_evidence_count,
        None,
        None,
        None,
        None,
    ))
}

pub fn instructions_sysvar_evidence_scanner_report(
) -> &'static InstructionsSysvarEvidenceScannerReport {
    &INSTRUCTIONS_SYSVAR_EVIDENCE_SCANNER_REPORT
}

fn error(
    kind: InstructionsSysvarEvidenceScannerErrorKind,
    scanned_instruction_count: usize,
    ed25519_candidate_count: usize,
    non_ed25519_instruction_count: usize,
    matching_evidence_count: usize,
    failed_candidate_instruction_index: Option<usize>,
    first_matching_instruction_index: Option<usize>,
    duplicate_matching_instruction_index: Option<usize>,
    candidate_parser_error: Option<Ed25519InstructionDataParserError>,
) -> InstructionsSysvarEvidenceScannerError {
    InstructionsSysvarEvidenceScannerError {
        kind,
        scanned_instruction_count,
        ed25519_candidate_count,
        non_ed25519_instruction_count,
        matching_evidence_count,
        failed_candidate_instruction_index,
        first_matching_instruction_index,
        duplicate_matching_instruction_index,
        candidate_parser_error,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::verifier::{
        canonical_payload_hash_validation_report, ed25519_evidence_layout_model_report,
        ed25519_instruction_data_parser_report, guardian_quorum_structural_report,
        read_only_verifier_boundary, Ed25519InstructionDataParserErrorKind, ED25519_SIGNATURE_LEN,
        READ_ONLY_RUNTIME_VERIFIER_SCAFFOLD_PHASE_32, XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1,
    };

    const NON_ED25519_PROGRAM_ID: &str = "OtherProgram111111111111111111111111111111111";
    const EXPECTED_GUARDIAN_PUBLIC_KEY: GuardianPublicKey = GuardianPublicKey([0x31; 32]);
    const SIGNATURE_OFFSET: u16 = 16;
    const PUBLIC_KEY_OFFSET: u16 = 80;
    const MESSAGE_OFFSET: u16 = 112;
    const VALID_INSTRUCTION_DATA_LEN: usize = 144;
    const SIGNATURE_BYTES: [u8; 64] = [0x5a; 64];

    fn write_u16_le(out: &mut [u8], offset: usize, value: u16) {
        out[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
    }

    fn valid_ed25519_instruction_data() -> Vec<u8> {
        let mut data = vec![0u8; VALID_INSTRUCTION_DATA_LEN];
        data[0] = 1;
        data[1] = 0;
        write_u16_le(&mut data, 2, SIGNATURE_OFFSET);
        write_u16_le(&mut data, 4, u16::MAX);
        write_u16_le(&mut data, 6, PUBLIC_KEY_OFFSET);
        write_u16_le(&mut data, 8, u16::MAX);
        write_u16_le(&mut data, 10, MESSAGE_OFFSET);
        write_u16_le(&mut data, 12, EXPECTED_MESSAGE_LEN as u16);
        write_u16_le(&mut data, 14, u16::MAX);
        data[SIGNATURE_OFFSET as usize..SIGNATURE_OFFSET as usize + ED25519_SIGNATURE_LEN]
            .copy_from_slice(&SIGNATURE_BYTES);
        data[PUBLIC_KEY_OFFSET as usize..PUBLIC_KEY_OFFSET as usize + 32]
            .copy_from_slice(&EXPECTED_GUARDIAN_PUBLIC_KEY.0);
        data[MESSAGE_OFFSET as usize..MESSAGE_OFFSET as usize + EXPECTED_MESSAGE_LEN]
            .copy_from_slice(&XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1);

        data
    }

    fn non_ed_instruction(data: &[u8]) -> InstructionsSysvarInstructionView<'_> {
        InstructionsSysvarInstructionView {
            program_id: NON_ED25519_PROGRAM_ID,
            instruction_data: data,
        }
    }

    fn ed25519_instruction(data: &[u8]) -> InstructionsSysvarInstructionView<'_> {
        InstructionsSysvarInstructionView {
            program_id: ED25519_PROGRAM_ID_REFERENCE,
            instruction_data: data,
        }
    }

    fn scan<'a>(
        instructions: &'a [InstructionsSysvarInstructionView<'a>],
    ) -> Result<ScannedEd25519InstructionEvidence, InstructionsSysvarEvidenceScannerError> {
        scan_instructions_sysvar_for_ed25519_evidence(
            instructions,
            EXPECTED_GUARDIAN_PUBLIC_KEY,
            &XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1,
        )
    }

    fn assert_error_kind(
        result: Result<ScannedEd25519InstructionEvidence, InstructionsSysvarEvidenceScannerError>,
        kind: InstructionsSysvarEvidenceScannerErrorKind,
    ) -> InstructionsSysvarEvidenceScannerError {
        let error = result.expect_err("instructions sysvar evidence scanner error");
        assert_eq!(error.kind, kind);
        error
    }

    #[test]
    fn scanner_finds_one_valid_ed25519_evidence_among_non_ed25519_instructions() {
        let non_ed_data = [0x99; 3];
        let ed_data = valid_ed25519_instruction_data();
        let instructions = [
            non_ed_instruction(&non_ed_data),
            ed25519_instruction(&ed_data),
            non_ed_instruction(&non_ed_data),
        ];

        let result = scan(&instructions).expect("matching ed25519 evidence");

        assert_eq!(result.matched_instruction_index, 1);
        assert_eq!(result.scanned_instruction_count, 3);
        assert_eq!(result.ed25519_candidate_count, 1);
        assert_eq!(result.non_ed25519_instruction_count, 2);
        assert_eq!(
            result.parsed_ed25519_instruction_evidence.signature_bytes,
            SIGNATURE_BYTES
        );
        assert!(result.public_key_matches_expected_guardian);
        assert!(result.message_matches_expected_phase_34_hash);
    }

    #[test]
    fn scanner_skips_non_ed25519_program_instructions() {
        let non_ed_data = [0x01, 0x02];
        let ed_data = valid_ed25519_instruction_data();
        let instructions = [
            non_ed_instruction(&non_ed_data),
            ed25519_instruction(&ed_data),
        ];

        let result = scan(&instructions).expect("matching evidence");

        assert_eq!(result.non_ed25519_instruction_count, 1);
        assert_eq!(result.ed25519_candidate_count, 1);
    }

    #[test]
    fn empty_instruction_set_is_rejected() {
        let instructions = [];

        let error = assert_error_kind(
            scan(&instructions),
            InstructionsSysvarEvidenceScannerErrorKind::EmptyInstructionSet,
        );

        assert_eq!(error.scanned_instruction_count, 0);
    }

    #[test]
    fn no_matching_ed25519_evidence_is_rejected() {
        let non_ed_data = [0x01, 0x02];
        let instructions = [non_ed_instruction(&non_ed_data)];

        let error = assert_error_kind(
            scan(&instructions),
            InstructionsSysvarEvidenceScannerErrorKind::NoMatchingEd25519Evidence,
        );

        assert_eq!(error.ed25519_candidate_count, 0);
        assert_eq!(error.non_ed25519_instruction_count, 1);
    }

    #[test]
    fn duplicate_matching_ed25519_evidence_is_rejected() {
        let first_data = valid_ed25519_instruction_data();
        let second_data = valid_ed25519_instruction_data();
        let instructions = [
            ed25519_instruction(&first_data),
            ed25519_instruction(&second_data),
        ];

        let error = assert_error_kind(
            scan(&instructions),
            InstructionsSysvarEvidenceScannerErrorKind::DuplicateMatchingEd25519Evidence,
        );

        assert_eq!(error.matching_evidence_count, 2);
        assert_eq!(error.first_matching_instruction_index, Some(0));
        assert_eq!(error.duplicate_matching_instruction_index, Some(1));
    }

    #[test]
    fn malformed_ed25519_candidate_parser_failure_is_reported() {
        let malformed_data = [0u8; 4];
        let instructions = [ed25519_instruction(&malformed_data)];

        let error = assert_error_kind(
            scan(&instructions),
            InstructionsSysvarEvidenceScannerErrorKind::CandidateInstructionParseFailed,
        );

        assert_eq!(error.failed_candidate_instruction_index, Some(0));
        assert_eq!(
            error.candidate_parser_error.expect("parser error").kind,
            Ed25519InstructionDataParserErrorKind::InstructionDataTooShort
        );
    }

    #[test]
    fn guardian_public_key_mismatch_is_rejected_through_phase_38_parser() {
        let mut ed_data = valid_ed25519_instruction_data();
        ed_data[PUBLIC_KEY_OFFSET as usize] ^= 0xff;
        let instructions = [ed25519_instruction(&ed_data)];

        let error = assert_error_kind(
            scan(&instructions),
            InstructionsSysvarEvidenceScannerErrorKind::CandidateInstructionParseFailed,
        );

        assert_eq!(
            error.candidate_parser_error.expect("parser error").kind,
            Ed25519InstructionDataParserErrorKind::GuardianPublicKeyMismatch
        );
    }

    #[test]
    fn message_hash_mismatch_is_rejected_through_phase_38_parser() {
        let mut ed_data = valid_ed25519_instruction_data();
        ed_data[MESSAGE_OFFSET as usize] ^= 0xff;
        let instructions = [ed25519_instruction(&ed_data)];

        let error = assert_error_kind(
            scan(&instructions),
            InstructionsSysvarEvidenceScannerErrorKind::CandidateInstructionParseFailed,
        );

        assert_eq!(
            error.candidate_parser_error.expect("parser error").kind,
            Ed25519InstructionDataParserErrorKind::MessageHashMismatch
        );
    }

    #[test]
    fn success_result_does_not_claim_crypto_proof_quorum_or_authorization() {
        let ed_data = valid_ed25519_instruction_data();
        let instructions = [ed25519_instruction(&ed_data)];

        let result = scan(&instructions).expect("matching evidence");

        assert!(!result.ed25519_signature_verification_performed);
        assert!(!result.cryptographic_signature_proof_accepted);
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
    fn report_preserves_all_disabled_execution_and_security_flags() {
        let report = instructions_sysvar_evidence_scanner_report();

        assert_eq!(
            report.scanner_id,
            INSTRUCTIONS_SYSVAR_EVIDENCE_SCANNER_PHASE_39
        );
        assert_eq!(
            report.scanner_version,
            INSTRUCTIONS_SYSVAR_EVIDENCE_SCANNER_VERSION
        );
        assert!(report.scans_prepared_read_only_instruction_entries);
        assert!(!report.raw_instructions_sysvar_data_parsed);
        assert!(!report.account_info_parser_enabled);
        assert!(report.identifies_ed25519_program_instruction_candidates);
        assert_eq!(
            report.ed25519_program_id_reference,
            ED25519_PROGRAM_ID_REFERENCE
        );
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
    fn phase_37_layout_model_remains_required() {
        let report = instructions_sysvar_evidence_scanner_report();
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
        assert!(phase_37_report.layout_shape_check_enabled);
    }

    #[test]
    fn phase_38_parser_remains_required() {
        let report = instructions_sysvar_evidence_scanner_report();
        let phase_38_report = ed25519_instruction_data_parser_report();

        assert!(report.phase_38_instruction_data_parser_required);
        assert_eq!(
            report.phase_38_instruction_data_parser_phase,
            ED25519_INSTRUCTION_DATA_PARSER_PHASE_38
        );
        assert_eq!(
            phase_38_report.parser_id,
            ED25519_INSTRUCTION_DATA_PARSER_PHASE_38
        );
        assert!(phase_38_report.actual_instruction_data_bytes_parsed);
    }

    #[test]
    fn phase_34_hash_validator_remains_available_and_recomputing() {
        let report = instructions_sysvar_evidence_scanner_report();
        let phase_34_report = canonical_payload_hash_validation_report();

        assert!(report.phase_34_hash_validator_available);
        assert_eq!(
            report.phase_34_hash_validator_phase,
            CANONICAL_PAYLOAD_HASH_VALIDATOR_PHASE_34
        );
        assert!(report.phase_34_hash_validator_recomputes_hash);
        assert!(phase_34_report.recomputes_hash_from_payload_bytes);
        assert!(!phase_34_report.caller_provided_payload_hash_trusted);
    }

    #[test]
    fn phase_35_quorum_remains_separate_and_not_counted() {
        let report = instructions_sysvar_evidence_scanner_report();
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
