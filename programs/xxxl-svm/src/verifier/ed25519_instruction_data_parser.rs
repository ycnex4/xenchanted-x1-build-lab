use core::convert::TryInto;

use super::{
    validate_ed25519_evidence_layout, Ed25519EvidenceLayoutDescriptor, Ed25519EvidenceLayoutError,
    Ed25519SignatureOffsetsModel, GuardianPublicKey, CANONICAL_PAYLOAD_HASH_VALIDATOR_PHASE_34,
    CURRENT_INSTRUCTION_INDEX_SENTINEL, ED25519_INSTRUCTION_EVIDENCE_LAYOUT_MODEL_PHASE_37,
    ED25519_INSTRUCTION_HEADER_LEN, ED25519_PUBLIC_KEY_LEN, ED25519_SIGNATURE_LEN,
    ED25519_SIGNATURE_OFFSETS_RECORD_LEN, EXPECTED_MESSAGE_LEN, EXPECTED_SIGNATURE_COUNT,
    GUARDIAN_QUORUM_STRUCTURAL_VERIFIER_PHASE_35,
};

pub const ED25519_INSTRUCTION_DATA_PARSER_PHASE_38: &str =
    "ED25519_INSTRUCTION_DATA_PARSER_PHASE_38";
pub const ED25519_INSTRUCTION_DATA_PARSER_VERSION: u8 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParsedEd25519InstructionEvidence {
    pub signature_bytes: [u8; ED25519_SIGNATURE_LEN],
    pub guardian_public_key: GuardianPublicKey,
    pub message_bytes: [u8; EXPECTED_MESSAGE_LEN],
    pub public_key_matches_expected_guardian: bool,
    pub message_matches_expected_phase_34_hash: bool,
    pub ed25519_signature_verification_performed: bool,
    pub cryptographic_signature_proof_accepted: bool,
    pub instructions_sysvar_read: bool,
    pub quorum_counted: bool,
    pub authorization_granted: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ed25519InstructionDataParserErrorKind {
    InstructionDataTooShort,
    UnexpectedSignatureCount,
    NonzeroPadding,
    SignatureInstructionIndexNotCurrent,
    PublicKeyInstructionIndexNotCurrent,
    MessageInstructionIndexNotCurrent,
    UnexpectedMessageSize,
    LayoutShapeInvalid,
    GuardianPublicKeyMismatch,
    MessageHashMismatch,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ed25519InstructionDataParserError {
    pub kind: Ed25519InstructionDataParserErrorKind,
    pub instruction_data_len: usize,
    pub offset: Option<usize>,
    pub expected: Option<usize>,
    pub actual: Option<usize>,
    pub layout_error: Option<Ed25519EvidenceLayoutError>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ed25519InstructionDataParserReport {
    pub parser_id: &'static str,
    pub parser_version: u8,
    pub phase_37_layout_model_required: bool,
    pub phase_37_layout_model_phase: &'static str,
    pub actual_instruction_data_bytes_parsed: bool,
    pub signature_bytes_extracted: bool,
    pub public_key_bytes_extracted: bool,
    pub message_bytes_extracted: bool,
    pub expected_guardian_public_key_compared: bool,
    pub expected_phase_34_payload_hash_compared: bool,
    pub phase_34_hash_validator_available: bool,
    pub phase_34_hash_validator_phase: &'static str,
    pub phase_34_hash_validator_recomputes_hash: bool,
    pub phase_35_quorum_phase: &'static str,
    pub phase_35_quorum_separate_and_not_counted: bool,
    pub ed25519_signature_verification_enabled: bool,
    pub cryptographic_signature_proof_accepted: bool,
    pub instructions_sysvar_read_enabled: bool,
    pub transaction_instruction_scan_enabled: bool,
    pub load_instruction_enabled: bool,
    pub ed25519_program_instruction_validation_enabled: bool,
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

pub const ED25519_INSTRUCTION_DATA_PARSER_REPORT: Ed25519InstructionDataParserReport =
    Ed25519InstructionDataParserReport {
        parser_id: ED25519_INSTRUCTION_DATA_PARSER_PHASE_38,
        parser_version: ED25519_INSTRUCTION_DATA_PARSER_VERSION,
        phase_37_layout_model_required: true,
        phase_37_layout_model_phase: ED25519_INSTRUCTION_EVIDENCE_LAYOUT_MODEL_PHASE_37,
        actual_instruction_data_bytes_parsed: true,
        signature_bytes_extracted: true,
        public_key_bytes_extracted: true,
        message_bytes_extracted: true,
        expected_guardian_public_key_compared: true,
        expected_phase_34_payload_hash_compared: true,
        phase_34_hash_validator_available: true,
        phase_34_hash_validator_phase: CANONICAL_PAYLOAD_HASH_VALIDATOR_PHASE_34,
        phase_34_hash_validator_recomputes_hash: true,
        phase_35_quorum_phase: GUARDIAN_QUORUM_STRUCTURAL_VERIFIER_PHASE_35,
        phase_35_quorum_separate_and_not_counted: true,
        ed25519_signature_verification_enabled: false,
        cryptographic_signature_proof_accepted: false,
        instructions_sysvar_read_enabled: false,
        transaction_instruction_scan_enabled: false,
        load_instruction_enabled: false,
        ed25519_program_instruction_validation_enabled: false,
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

pub fn parse_ed25519_instruction_data_evidence(
    instruction_data: &[u8],
    expected_guardian_public_key: GuardianPublicKey,
    expected_phase_34_payload_hash: &[u8; EXPECTED_MESSAGE_LEN],
) -> Result<ParsedEd25519InstructionEvidence, Ed25519InstructionDataParserError> {
    let minimum_instruction_data_len =
        ED25519_INSTRUCTION_HEADER_LEN + ED25519_SIGNATURE_OFFSETS_RECORD_LEN;

    if instruction_data.len() < minimum_instruction_data_len {
        return Err(error(
            Ed25519InstructionDataParserErrorKind::InstructionDataTooShort,
            instruction_data.len(),
            None,
            Some(minimum_instruction_data_len),
            Some(instruction_data.len()),
            None,
        ));
    }

    let num_signatures = instruction_data[0];
    if num_signatures != EXPECTED_SIGNATURE_COUNT {
        return Err(error(
            Ed25519InstructionDataParserErrorKind::UnexpectedSignatureCount,
            instruction_data.len(),
            Some(0),
            Some(EXPECTED_SIGNATURE_COUNT as usize),
            Some(num_signatures as usize),
            None,
        ));
    }

    let padding = instruction_data[1];
    if padding != 0 {
        return Err(error(
            Ed25519InstructionDataParserErrorKind::NonzeroPadding,
            instruction_data.len(),
            Some(1),
            Some(0),
            Some(padding as usize),
            None,
        ));
    }

    let offsets = parse_offsets_record(instruction_data);

    if offsets.signature_instruction_index != CURRENT_INSTRUCTION_INDEX_SENTINEL {
        return Err(error(
            Ed25519InstructionDataParserErrorKind::SignatureInstructionIndexNotCurrent,
            instruction_data.len(),
            Some(4),
            Some(CURRENT_INSTRUCTION_INDEX_SENTINEL as usize),
            Some(offsets.signature_instruction_index as usize),
            None,
        ));
    }

    if offsets.public_key_instruction_index != CURRENT_INSTRUCTION_INDEX_SENTINEL {
        return Err(error(
            Ed25519InstructionDataParserErrorKind::PublicKeyInstructionIndexNotCurrent,
            instruction_data.len(),
            Some(8),
            Some(CURRENT_INSTRUCTION_INDEX_SENTINEL as usize),
            Some(offsets.public_key_instruction_index as usize),
            None,
        ));
    }

    if offsets.message_instruction_index != CURRENT_INSTRUCTION_INDEX_SENTINEL {
        return Err(error(
            Ed25519InstructionDataParserErrorKind::MessageInstructionIndexNotCurrent,
            instruction_data.len(),
            Some(14),
            Some(CURRENT_INSTRUCTION_INDEX_SENTINEL as usize),
            Some(offsets.message_instruction_index as usize),
            None,
        ));
    }

    if offsets.message_data_size as usize != EXPECTED_MESSAGE_LEN {
        return Err(error(
            Ed25519InstructionDataParserErrorKind::UnexpectedMessageSize,
            instruction_data.len(),
            Some(12),
            Some(EXPECTED_MESSAGE_LEN),
            Some(offsets.message_data_size as usize),
            None,
        ));
    }

    let descriptor = Ed25519EvidenceLayoutDescriptor {
        instruction_data_len: instruction_data.len(),
        num_signatures,
        offsets,
    };

    validate_ed25519_evidence_layout(descriptor).map_err(|layout_error| {
        error(
            Ed25519InstructionDataParserErrorKind::LayoutShapeInvalid,
            instruction_data.len(),
            layout_error.region_start,
            layout_error.expected_len,
            layout_error.region_end,
            Some(layout_error),
        )
    })?;

    let signature_bytes =
        read_fixed::<ED25519_SIGNATURE_LEN>(instruction_data, offsets.signature_offset as usize);
    let public_key_bytes =
        read_fixed::<ED25519_PUBLIC_KEY_LEN>(instruction_data, offsets.public_key_offset as usize);
    let message_bytes =
        read_fixed::<EXPECTED_MESSAGE_LEN>(instruction_data, offsets.message_data_offset as usize);
    let guardian_public_key = GuardianPublicKey(public_key_bytes);

    if guardian_public_key != expected_guardian_public_key {
        return Err(error(
            Ed25519InstructionDataParserErrorKind::GuardianPublicKeyMismatch,
            instruction_data.len(),
            Some(offsets.public_key_offset as usize),
            Some(ED25519_PUBLIC_KEY_LEN),
            Some(ED25519_PUBLIC_KEY_LEN),
            None,
        ));
    }

    if &message_bytes != expected_phase_34_payload_hash {
        return Err(error(
            Ed25519InstructionDataParserErrorKind::MessageHashMismatch,
            instruction_data.len(),
            Some(offsets.message_data_offset as usize),
            Some(EXPECTED_MESSAGE_LEN),
            Some(EXPECTED_MESSAGE_LEN),
            None,
        ));
    }

    Ok(ParsedEd25519InstructionEvidence {
        signature_bytes,
        guardian_public_key,
        message_bytes,
        public_key_matches_expected_guardian: true,
        message_matches_expected_phase_34_hash: true,
        ed25519_signature_verification_performed: false,
        cryptographic_signature_proof_accepted: false,
        instructions_sysvar_read: false,
        quorum_counted: false,
        authorization_granted: false,
    })
}

pub fn ed25519_instruction_data_parser_report() -> &'static Ed25519InstructionDataParserReport {
    &ED25519_INSTRUCTION_DATA_PARSER_REPORT
}

fn parse_offsets_record(instruction_data: &[u8]) -> Ed25519SignatureOffsetsModel {
    Ed25519SignatureOffsetsModel {
        signature_offset: read_u16_le(instruction_data, 2),
        signature_instruction_index: read_u16_le(instruction_data, 4),
        public_key_offset: read_u16_le(instruction_data, 6),
        public_key_instruction_index: read_u16_le(instruction_data, 8),
        message_data_offset: read_u16_le(instruction_data, 10),
        message_data_size: read_u16_le(instruction_data, 12),
        message_instruction_index: read_u16_le(instruction_data, 14),
    }
}

fn read_u16_le(input: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes(
        input[offset..offset + 2]
            .try_into()
            .expect("u16 slice length"),
    )
}

fn read_fixed<const LEN: usize>(input: &[u8], offset: usize) -> [u8; LEN] {
    input[offset..offset + LEN]
        .try_into()
        .expect("validated fixed byte region")
}

fn error(
    kind: Ed25519InstructionDataParserErrorKind,
    instruction_data_len: usize,
    offset: Option<usize>,
    expected: Option<usize>,
    actual: Option<usize>,
    layout_error: Option<Ed25519EvidenceLayoutError>,
) -> Ed25519InstructionDataParserError {
    Ed25519InstructionDataParserError {
        kind,
        instruction_data_len,
        offset,
        expected,
        actual,
        layout_error,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::verifier::{
        canonical_payload_hash_validation_report, ed25519_evidence_layout_model_report,
        guardian_quorum_structural_report, read_only_verifier_boundary,
        Ed25519EvidenceLayoutErrorKind, GUARDIAN_QUORUM_STRUCTURAL_VERIFIER_PHASE_35,
        READ_ONLY_RUNTIME_VERIFIER_SCAFFOLD_PHASE_32, XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1,
    };

    const SIGNATURE_OFFSET: u16 = 16;
    const PUBLIC_KEY_OFFSET: u16 = 80;
    const MESSAGE_OFFSET: u16 = 112;
    const VALID_INSTRUCTION_DATA_LEN: usize = 144;
    const EXPECTED_GUARDIAN_PUBLIC_KEY: GuardianPublicKey = GuardianPublicKey([0x31; 32]);
    const SIGNATURE_BYTES: [u8; 64] = [0x5a; 64];

    fn write_u16_le(out: &mut [u8], offset: usize, value: u16) {
        out[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
    }

    fn valid_instruction_data() -> Vec<u8> {
        let mut data = vec![0u8; VALID_INSTRUCTION_DATA_LEN];
        data[0] = EXPECTED_SIGNATURE_COUNT;
        data[1] = 0;
        write_u16_le(&mut data, 2, SIGNATURE_OFFSET);
        write_u16_le(&mut data, 4, CURRENT_INSTRUCTION_INDEX_SENTINEL);
        write_u16_le(&mut data, 6, PUBLIC_KEY_OFFSET);
        write_u16_le(&mut data, 8, CURRENT_INSTRUCTION_INDEX_SENTINEL);
        write_u16_le(&mut data, 10, MESSAGE_OFFSET);
        write_u16_le(&mut data, 12, EXPECTED_MESSAGE_LEN as u16);
        write_u16_le(&mut data, 14, CURRENT_INSTRUCTION_INDEX_SENTINEL);
        data[SIGNATURE_OFFSET as usize..SIGNATURE_OFFSET as usize + ED25519_SIGNATURE_LEN]
            .copy_from_slice(&SIGNATURE_BYTES);
        data[PUBLIC_KEY_OFFSET as usize..PUBLIC_KEY_OFFSET as usize + ED25519_PUBLIC_KEY_LEN]
            .copy_from_slice(&EXPECTED_GUARDIAN_PUBLIC_KEY.0);
        data[MESSAGE_OFFSET as usize..MESSAGE_OFFSET as usize + EXPECTED_MESSAGE_LEN]
            .copy_from_slice(&XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1);

        data
    }

    fn parse_valid() -> Result<ParsedEd25519InstructionEvidence, Ed25519InstructionDataParserError>
    {
        parse_ed25519_instruction_data_evidence(
            &valid_instruction_data(),
            EXPECTED_GUARDIAN_PUBLIC_KEY,
            &XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1,
        )
    }

    fn assert_error_kind(
        result: Result<ParsedEd25519InstructionEvidence, Ed25519InstructionDataParserError>,
        kind: Ed25519InstructionDataParserErrorKind,
    ) -> Ed25519InstructionDataParserError {
        let error = result.expect_err("ed25519 instruction data parser error");
        assert_eq!(error.kind, kind);
        error
    }

    #[test]
    fn valid_instruction_data_parses_and_extracts_evidence_bytes() {
        let result = parse_valid().expect("valid ed25519 instruction data");

        assert_eq!(result.signature_bytes, SIGNATURE_BYTES);
        assert_eq!(result.guardian_public_key, EXPECTED_GUARDIAN_PUBLIC_KEY);
        assert_eq!(result.message_bytes, XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1);
        assert!(result.public_key_matches_expected_guardian);
        assert!(result.message_matches_expected_phase_34_hash);
    }

    #[test]
    fn valid_result_does_not_claim_crypto_sysvar_quorum_or_authorization() {
        let result = parse_valid().expect("valid ed25519 instruction data");

        assert!(!result.ed25519_signature_verification_performed);
        assert!(!result.cryptographic_signature_proof_accepted);
        assert!(!result.instructions_sysvar_read);
        assert!(!result.quorum_counted);
        assert!(!result.authorization_granted);
    }

    #[test]
    fn too_short_data_is_rejected() {
        let data =
            vec![0u8; ED25519_INSTRUCTION_HEADER_LEN + ED25519_SIGNATURE_OFFSETS_RECORD_LEN - 1];

        let error = assert_error_kind(
            parse_ed25519_instruction_data_evidence(
                &data,
                EXPECTED_GUARDIAN_PUBLIC_KEY,
                &XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1,
            ),
            Ed25519InstructionDataParserErrorKind::InstructionDataTooShort,
        );

        assert_eq!(error.expected, Some(16));
        assert_eq!(error.actual, Some(15));
    }

    #[test]
    fn signature_count_zero_is_rejected() {
        let mut data = valid_instruction_data();
        data[0] = 0;

        let error = assert_error_kind(
            parse_ed25519_instruction_data_evidence(
                &data,
                EXPECTED_GUARDIAN_PUBLIC_KEY,
                &XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1,
            ),
            Ed25519InstructionDataParserErrorKind::UnexpectedSignatureCount,
        );

        assert_eq!(error.actual, Some(0));
    }

    #[test]
    fn signature_count_two_is_rejected() {
        let mut data = valid_instruction_data();
        data[0] = 2;

        let error = assert_error_kind(
            parse_ed25519_instruction_data_evidence(
                &data,
                EXPECTED_GUARDIAN_PUBLIC_KEY,
                &XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1,
            ),
            Ed25519InstructionDataParserErrorKind::UnexpectedSignatureCount,
        );

        assert_eq!(error.actual, Some(2));
    }

    #[test]
    fn nonzero_padding_is_rejected() {
        let mut data = valid_instruction_data();
        data[1] = 1;

        let error = assert_error_kind(
            parse_ed25519_instruction_data_evidence(
                &data,
                EXPECTED_GUARDIAN_PUBLIC_KEY,
                &XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1,
            ),
            Ed25519InstructionDataParserErrorKind::NonzeroPadding,
        );

        assert_eq!(error.offset, Some(1));
    }

    #[test]
    fn non_sentinel_signature_instruction_index_is_rejected() {
        let mut data = valid_instruction_data();
        write_u16_le(&mut data, 4, 0);

        assert_error_kind(
            parse_ed25519_instruction_data_evidence(
                &data,
                EXPECTED_GUARDIAN_PUBLIC_KEY,
                &XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1,
            ),
            Ed25519InstructionDataParserErrorKind::SignatureInstructionIndexNotCurrent,
        );
    }

    #[test]
    fn non_sentinel_public_key_instruction_index_is_rejected() {
        let mut data = valid_instruction_data();
        write_u16_le(&mut data, 8, 0);

        assert_error_kind(
            parse_ed25519_instruction_data_evidence(
                &data,
                EXPECTED_GUARDIAN_PUBLIC_KEY,
                &XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1,
            ),
            Ed25519InstructionDataParserErrorKind::PublicKeyInstructionIndexNotCurrent,
        );
    }

    #[test]
    fn non_sentinel_message_instruction_index_is_rejected() {
        let mut data = valid_instruction_data();
        write_u16_le(&mut data, 14, 0);

        assert_error_kind(
            parse_ed25519_instruction_data_evidence(
                &data,
                EXPECTED_GUARDIAN_PUBLIC_KEY,
                &XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1,
            ),
            Ed25519InstructionDataParserErrorKind::MessageInstructionIndexNotCurrent,
        );
    }

    #[test]
    fn thirty_one_byte_message_size_is_rejected() {
        let mut data = valid_instruction_data();
        write_u16_le(&mut data, 12, 31);

        let error = assert_error_kind(
            parse_ed25519_instruction_data_evidence(
                &data,
                EXPECTED_GUARDIAN_PUBLIC_KEY,
                &XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1,
            ),
            Ed25519InstructionDataParserErrorKind::UnexpectedMessageSize,
        );

        assert_eq!(error.expected, Some(32));
        assert_eq!(error.actual, Some(31));
    }

    #[test]
    fn thirty_three_byte_message_size_is_rejected() {
        let mut data = valid_instruction_data();
        write_u16_le(&mut data, 12, 33);

        let error = assert_error_kind(
            parse_ed25519_instruction_data_evidence(
                &data,
                EXPECTED_GUARDIAN_PUBLIC_KEY,
                &XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1,
            ),
            Ed25519InstructionDataParserErrorKind::UnexpectedMessageSize,
        );

        assert_eq!(error.expected, Some(32));
        assert_eq!(error.actual, Some(33));
    }

    #[test]
    fn signature_region_overlap_header_is_rejected() {
        let mut data = valid_instruction_data();
        write_u16_le(&mut data, 2, 1);

        let error = assert_error_kind(
            parse_ed25519_instruction_data_evidence(
                &data,
                EXPECTED_GUARDIAN_PUBLIC_KEY,
                &XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1,
            ),
            Ed25519InstructionDataParserErrorKind::LayoutShapeInvalid,
        );

        assert_eq!(
            error.layout_error.expect("layout error").kind,
            Ed25519EvidenceLayoutErrorKind::SignatureRegionOverlapsHeader
        );
    }

    #[test]
    fn signature_region_out_of_bounds_is_rejected() {
        let mut data = valid_instruction_data();
        write_u16_le(&mut data, 2, (VALID_INSTRUCTION_DATA_LEN - 1) as u16);

        let error = assert_error_kind(
            parse_ed25519_instruction_data_evidence(
                &data,
                EXPECTED_GUARDIAN_PUBLIC_KEY,
                &XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1,
            ),
            Ed25519InstructionDataParserErrorKind::LayoutShapeInvalid,
        );

        assert_eq!(
            error.layout_error.expect("layout error").kind,
            Ed25519EvidenceLayoutErrorKind::SignatureRegionOutOfBounds
        );
    }

    #[test]
    fn public_key_region_overlap_header_is_rejected() {
        let mut data = valid_instruction_data();
        write_u16_le(&mut data, 6, 0);

        let error = assert_error_kind(
            parse_ed25519_instruction_data_evidence(
                &data,
                EXPECTED_GUARDIAN_PUBLIC_KEY,
                &XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1,
            ),
            Ed25519InstructionDataParserErrorKind::LayoutShapeInvalid,
        );

        assert_eq!(
            error.layout_error.expect("layout error").kind,
            Ed25519EvidenceLayoutErrorKind::PublicKeyRegionOverlapsHeader
        );
    }

    #[test]
    fn public_key_region_out_of_bounds_is_rejected() {
        let mut data = valid_instruction_data();
        write_u16_le(&mut data, 6, (VALID_INSTRUCTION_DATA_LEN - 4) as u16);

        let error = assert_error_kind(
            parse_ed25519_instruction_data_evidence(
                &data,
                EXPECTED_GUARDIAN_PUBLIC_KEY,
                &XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1,
            ),
            Ed25519InstructionDataParserErrorKind::LayoutShapeInvalid,
        );

        assert_eq!(
            error.layout_error.expect("layout error").kind,
            Ed25519EvidenceLayoutErrorKind::PublicKeyRegionOutOfBounds
        );
    }

    #[test]
    fn message_region_overlap_header_is_rejected() {
        let mut data = valid_instruction_data();
        write_u16_le(&mut data, 10, 1);

        let error = assert_error_kind(
            parse_ed25519_instruction_data_evidence(
                &data,
                EXPECTED_GUARDIAN_PUBLIC_KEY,
                &XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1,
            ),
            Ed25519InstructionDataParserErrorKind::LayoutShapeInvalid,
        );

        assert_eq!(
            error.layout_error.expect("layout error").kind,
            Ed25519EvidenceLayoutErrorKind::MessageRegionOverlapsHeader
        );
    }

    #[test]
    fn message_region_out_of_bounds_is_rejected() {
        let mut data = valid_instruction_data();
        write_u16_le(&mut data, 10, (VALID_INSTRUCTION_DATA_LEN - 16) as u16);

        let error = assert_error_kind(
            parse_ed25519_instruction_data_evidence(
                &data,
                EXPECTED_GUARDIAN_PUBLIC_KEY,
                &XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1,
            ),
            Ed25519InstructionDataParserErrorKind::LayoutShapeInvalid,
        );

        assert_eq!(
            error.layout_error.expect("layout error").kind,
            Ed25519EvidenceLayoutErrorKind::MessageRegionOutOfBounds
        );
    }

    #[test]
    fn guardian_public_key_mismatch_is_rejected() {
        let data = valid_instruction_data();

        assert_error_kind(
            parse_ed25519_instruction_data_evidence(
                &data,
                GuardianPublicKey([0x32; 32]),
                &XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1,
            ),
            Ed25519InstructionDataParserErrorKind::GuardianPublicKeyMismatch,
        );
    }

    #[test]
    fn message_hash_mismatch_is_rejected() {
        let data = valid_instruction_data();
        let mut wrong_hash = XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1;
        wrong_hash[0] ^= 0xff;

        assert_error_kind(
            parse_ed25519_instruction_data_evidence(
                &data,
                EXPECTED_GUARDIAN_PUBLIC_KEY,
                &wrong_hash,
            ),
            Ed25519InstructionDataParserErrorKind::MessageHashMismatch,
        );
    }

    #[test]
    fn report_preserves_all_disabled_execution_and_security_flags() {
        let report = ed25519_instruction_data_parser_report();

        assert_eq!(report.parser_id, ED25519_INSTRUCTION_DATA_PARSER_PHASE_38);
        assert_eq!(
            report.parser_version,
            ED25519_INSTRUCTION_DATA_PARSER_VERSION
        );
        assert!(report.actual_instruction_data_bytes_parsed);
        assert!(report.signature_bytes_extracted);
        assert!(report.public_key_bytes_extracted);
        assert!(report.message_bytes_extracted);
        assert!(report.expected_guardian_public_key_compared);
        assert!(report.expected_phase_34_payload_hash_compared);
        assert!(!report.ed25519_signature_verification_enabled);
        assert!(!report.cryptographic_signature_proof_accepted);
        assert!(!report.instructions_sysvar_read_enabled);
        assert!(!report.transaction_instruction_scan_enabled);
        assert!(!report.load_instruction_enabled);
        assert!(!report.ed25519_program_instruction_validation_enabled);
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
        let report = ed25519_instruction_data_parser_report();
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
    fn phase_34_hash_validator_remains_available_and_recomputing() {
        let report = ed25519_instruction_data_parser_report();
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
        let report = ed25519_instruction_data_parser_report();
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
