pub const PHASE_41K6_B1C_3_ED25519_EVIDENCE_PARSER_PHASE: &str = "41K.6-B1C.3";
pub const PHASE_41K6_B1C_3_ED25519_EVIDENCE_PARSER_VERSION: &str = "0.1.0";

pub const ED25519_SIGNATURE_LEN: usize = 64;
pub const ED25519_PUBLIC_KEY_LEN: usize = 32;
pub const ED25519_SINGLE_SIGNATURE_HEADER_LEN: usize = 16;
pub const ED25519_CURRENT_INSTRUCTION_INDEX_SENTINEL: u16 = u16::MAX;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B1CEd25519EvidenceParsingStatus {
    Parsed,
    Rejected,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B1CEd25519EvidenceParsingRejectionKind {
    InstructionDataTooShort,
    SignatureCountZero,
    MultipleSignaturesUnsupported,
    NonZeroPadding,
    SignatureInstructionIndexNotCurrent,
    PublicKeyInstructionIndexNotCurrent,
    MessageInstructionIndexNotCurrent,
    SignatureOffsetOverlapsHeader,
    PublicKeyOffsetOverlapsHeader,
    MessageOffsetOverlapsHeader,
    SignatureOffsetOutOfBounds,
    PublicKeyOffsetOutOfBounds,
    MessageOffsetOutOfBounds,
    MessageLengthZero,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct B1CParsedEd25519Evidence {
    pub status: B1CEd25519EvidenceParsingStatus,
    pub source_instruction_index: usize,
    pub signer_public_key: [u8; ED25519_PUBLIC_KEY_LEN],
    pub signature: [u8; ED25519_SIGNATURE_LEN],
    pub signed_message: Vec<u8>,
    pub instruction_data_len: usize,
    pub signature_offset: u16,
    pub public_key_offset: u16,
    pub message_data_offset: u16,
    pub message_data_size: u16,
    pub runtime_verified_by_ed25519_precompile: bool,
    pub parsed_from_prior_ed25519_instruction: bool,
    pub single_signature_layout: bool,
    pub self_contained_current_instruction_offsets_only: bool,
    pub accepts_caller_provided_signature_claims: bool,
    pub accepts_frontend_or_watcher_proof: bool,
    pub binds_payload_hash: bool,
    pub validates_guardian_membership: bool,
    pub counts_unique_guardians: bool,
    pub authorization_enabled: bool,
    pub processed_event_marking_enabled: bool,
    pub cpi_enabled: bool,
    pub live_route_enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct B1CEd25519EvidenceParsingRejected {
    pub status: B1CEd25519EvidenceParsingStatus,
    pub kind: B1CEd25519EvidenceParsingRejectionKind,
    pub source_instruction_index: usize,
    pub instruction_data_len: usize,
    pub runtime_verified_by_ed25519_precompile: bool,
    pub parsed_from_prior_ed25519_instruction: bool,
    pub accepts_caller_provided_signature_claims: bool,
    pub accepts_frontend_or_watcher_proof: bool,
    pub binds_payload_hash: bool,
    pub validates_guardian_membership: bool,
    pub counts_unique_guardians: bool,
    pub authorization_enabled: bool,
    pub processed_event_marking_enabled: bool,
    pub cpi_enabled: bool,
    pub live_route_enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum B1CEd25519EvidenceParsingResult {
    Parsed(B1CParsedEd25519Evidence),
    Rejected(B1CEd25519EvidenceParsingRejected),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct B1CEd25519EvidenceParserReport {
    pub phase: &'static str,
    pub version: &'static str,
    pub pure_parser_only: bool,
    pub reads_instructions_sysvar: bool,
    pub supports_single_signature_layout: bool,
    pub supports_multi_signature_layout: bool,
    pub supports_cross_instruction_offsets: bool,
    pub requires_current_instruction_sentinel: bool,
    pub rejects_offsets_overlapping_header: bool,
    pub re_verifies_ed25519_signature: bool,
    pub binds_payload_hash: bool,
    pub validates_guardian_membership: bool,
    pub counts_unique_guardians: bool,
    pub authorizes_handler_execution: bool,
    pub processed_event_marking_enabled: bool,
    pub cpi_enabled: bool,
    pub live_route_enabled: bool,
}

pub const B1C_ED25519_EVIDENCE_PARSER_REPORT: B1CEd25519EvidenceParserReport =
    B1CEd25519EvidenceParserReport {
        phase: PHASE_41K6_B1C_3_ED25519_EVIDENCE_PARSER_PHASE,
        version: PHASE_41K6_B1C_3_ED25519_EVIDENCE_PARSER_VERSION,
        pure_parser_only: true,
        reads_instructions_sysvar: false,
        supports_single_signature_layout: true,
        supports_multi_signature_layout: false,
        supports_cross_instruction_offsets: false,
        requires_current_instruction_sentinel: true,
        rejects_offsets_overlapping_header: true,
        re_verifies_ed25519_signature: false,
        binds_payload_hash: false,
        validates_guardian_membership: false,
        counts_unique_guardians: false,
        authorizes_handler_execution: false,
        processed_event_marking_enabled: false,
        cpi_enabled: false,
        live_route_enabled: false,
    };

pub fn b1c_ed25519_evidence_parser_report() -> &'static B1CEd25519EvidenceParserReport {
    &B1C_ED25519_EVIDENCE_PARSER_REPORT
}

pub fn parse_b1c_single_ed25519_precompile_evidence(
    source_instruction_index: usize,
    instruction_data: &[u8],
) -> B1CEd25519EvidenceParsingResult {
    if instruction_data.len() < ED25519_SINGLE_SIGNATURE_HEADER_LEN {
        return rejected(
            source_instruction_index,
            instruction_data.len(),
            B1CEd25519EvidenceParsingRejectionKind::InstructionDataTooShort,
        );
    }

    let signature_count = instruction_data[0];
    let padding = instruction_data[1];

    if signature_count == 0 {
        return rejected(
            source_instruction_index,
            instruction_data.len(),
            B1CEd25519EvidenceParsingRejectionKind::SignatureCountZero,
        );
    }

    if signature_count != 1 {
        return rejected(
            source_instruction_index,
            instruction_data.len(),
            B1CEd25519EvidenceParsingRejectionKind::MultipleSignaturesUnsupported,
        );
    }

    if padding != 0 {
        return rejected(
            source_instruction_index,
            instruction_data.len(),
            B1CEd25519EvidenceParsingRejectionKind::NonZeroPadding,
        );
    }

    let signature_offset = read_u16_le(instruction_data, 2);
    let signature_instruction_index = read_u16_le(instruction_data, 4);
    let public_key_offset = read_u16_le(instruction_data, 6);
    let public_key_instruction_index = read_u16_le(instruction_data, 8);
    let message_data_offset = read_u16_le(instruction_data, 10);
    let message_data_size = read_u16_le(instruction_data, 12);
    let message_instruction_index = read_u16_le(instruction_data, 14);

    if signature_instruction_index != ED25519_CURRENT_INSTRUCTION_INDEX_SENTINEL {
        return rejected(
            source_instruction_index,
            instruction_data.len(),
            B1CEd25519EvidenceParsingRejectionKind::SignatureInstructionIndexNotCurrent,
        );
    }

    if public_key_instruction_index != ED25519_CURRENT_INSTRUCTION_INDEX_SENTINEL {
        return rejected(
            source_instruction_index,
            instruction_data.len(),
            B1CEd25519EvidenceParsingRejectionKind::PublicKeyInstructionIndexNotCurrent,
        );
    }

    if message_instruction_index != ED25519_CURRENT_INSTRUCTION_INDEX_SENTINEL {
        return rejected(
            source_instruction_index,
            instruction_data.len(),
            B1CEd25519EvidenceParsingRejectionKind::MessageInstructionIndexNotCurrent,
        );
    }

    if message_data_size == 0 {
        return rejected(
            source_instruction_index,
            instruction_data.len(),
            B1CEd25519EvidenceParsingRejectionKind::MessageLengthZero,
        );
    }

    let signature_range = match checked_data_range(
        instruction_data.len(),
        signature_offset,
        ED25519_SIGNATURE_LEN,
        B1CEd25519EvidenceParsingRejectionKind::SignatureOffsetOverlapsHeader,
        B1CEd25519EvidenceParsingRejectionKind::SignatureOffsetOutOfBounds,
    ) {
        Ok(range) => range,
        Err(kind) => return rejected(source_instruction_index, instruction_data.len(), kind),
    };

    let public_key_range = match checked_data_range(
        instruction_data.len(),
        public_key_offset,
        ED25519_PUBLIC_KEY_LEN,
        B1CEd25519EvidenceParsingRejectionKind::PublicKeyOffsetOverlapsHeader,
        B1CEd25519EvidenceParsingRejectionKind::PublicKeyOffsetOutOfBounds,
    ) {
        Ok(range) => range,
        Err(kind) => return rejected(source_instruction_index, instruction_data.len(), kind),
    };

    let message_range = match checked_data_range(
        instruction_data.len(),
        message_data_offset,
        usize::from(message_data_size),
        B1CEd25519EvidenceParsingRejectionKind::MessageOffsetOverlapsHeader,
        B1CEd25519EvidenceParsingRejectionKind::MessageOffsetOutOfBounds,
    ) {
        Ok(range) => range,
        Err(kind) => return rejected(source_instruction_index, instruction_data.len(), kind),
    };

    let mut signature = [0u8; ED25519_SIGNATURE_LEN];
    signature.copy_from_slice(&instruction_data[signature_range]);

    let mut signer_public_key = [0u8; ED25519_PUBLIC_KEY_LEN];
    signer_public_key.copy_from_slice(&instruction_data[public_key_range]);

    let signed_message = instruction_data[message_range].to_vec();

    B1CEd25519EvidenceParsingResult::Parsed(B1CParsedEd25519Evidence {
        status: B1CEd25519EvidenceParsingStatus::Parsed,
        source_instruction_index,
        signer_public_key,
        signature,
        signed_message,
        instruction_data_len: instruction_data.len(),
        signature_offset,
        public_key_offset,
        message_data_offset,
        message_data_size,
        runtime_verified_by_ed25519_precompile: true,
        parsed_from_prior_ed25519_instruction: true,
        single_signature_layout: true,
        self_contained_current_instruction_offsets_only: true,
        accepts_caller_provided_signature_claims: false,
        accepts_frontend_or_watcher_proof: false,
        binds_payload_hash: false,
        validates_guardian_membership: false,
        counts_unique_guardians: false,
        authorization_enabled: false,
        processed_event_marking_enabled: false,
        cpi_enabled: false,
        live_route_enabled: false,
    })
}

fn read_u16_le(data: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([data[offset], data[offset + 1]])
}

fn checked_data_range(
    instruction_data_len: usize,
    offset: u16,
    len: usize,
    overlap_kind: B1CEd25519EvidenceParsingRejectionKind,
    out_of_bounds_kind: B1CEd25519EvidenceParsingRejectionKind,
) -> Result<std::ops::Range<usize>, B1CEd25519EvidenceParsingRejectionKind> {
    let start = usize::from(offset);

    if start < ED25519_SINGLE_SIGNATURE_HEADER_LEN {
        return Err(overlap_kind);
    }

    let Some(end) = start.checked_add(len) else {
        return Err(out_of_bounds_kind);
    };

    if end > instruction_data_len {
        return Err(out_of_bounds_kind);
    }

    Ok(start..end)
}

fn rejected(
    source_instruction_index: usize,
    instruction_data_len: usize,
    kind: B1CEd25519EvidenceParsingRejectionKind,
) -> B1CEd25519EvidenceParsingResult {
    B1CEd25519EvidenceParsingResult::Rejected(B1CEd25519EvidenceParsingRejected {
        status: B1CEd25519EvidenceParsingStatus::Rejected,
        kind,
        source_instruction_index,
        instruction_data_len,
        runtime_verified_by_ed25519_precompile: false,
        parsed_from_prior_ed25519_instruction: false,
        accepts_caller_provided_signature_claims: false,
        accepts_frontend_or_watcher_proof: false,
        binds_payload_hash: false,
        validates_guardian_membership: false,
        counts_unique_guardians: false,
        authorization_enabled: false,
        processed_event_marking_enabled: false,
        cpi_enabled: false,
        live_route_enabled: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_instruction_data(message: &[u8]) -> Vec<u8> {
        let signature_offset = ED25519_SINGLE_SIGNATURE_HEADER_LEN as u16;
        let public_key_offset = signature_offset + ED25519_SIGNATURE_LEN as u16;
        let message_data_offset = public_key_offset + ED25519_PUBLIC_KEY_LEN as u16;
        let message_data_size = message.len() as u16;

        let mut data = vec![
            1, 0, 0, 0, 0xff, 0xff, 0, 0, 0xff, 0xff, 0, 0, 0, 0, 0xff, 0xff,
        ];

        data[2..4].copy_from_slice(&signature_offset.to_le_bytes());
        data[6..8].copy_from_slice(&public_key_offset.to_le_bytes());
        data[10..12].copy_from_slice(&message_data_offset.to_le_bytes());
        data[12..14].copy_from_slice(&message_data_size.to_le_bytes());

        data.extend_from_slice(&[0x51; ED25519_SIGNATURE_LEN]);
        data.extend_from_slice(&[0xA7; ED25519_PUBLIC_KEY_LEN]);
        data.extend_from_slice(message);

        data
    }

    fn assert_rejected(
        result: B1CEd25519EvidenceParsingResult,
        expected: B1CEd25519EvidenceParsingRejectionKind,
    ) {
        match result {
            B1CEd25519EvidenceParsingResult::Rejected(rejected) => {
                assert_eq!(rejected.status, B1CEd25519EvidenceParsingStatus::Rejected);
                assert_eq!(rejected.kind, expected);
                assert!(!rejected.runtime_verified_by_ed25519_precompile);
                assert!(!rejected.parsed_from_prior_ed25519_instruction);
                assert!(!rejected.accepts_caller_provided_signature_claims);
                assert!(!rejected.accepts_frontend_or_watcher_proof);
                assert!(!rejected.binds_payload_hash);
                assert!(!rejected.validates_guardian_membership);
                assert!(!rejected.counts_unique_guardians);
                assert!(!rejected.authorization_enabled);
                assert!(!rejected.processed_event_marking_enabled);
                assert!(!rejected.cpi_enabled);
                assert!(!rejected.live_route_enabled);
            }
            B1CEd25519EvidenceParsingResult::Parsed(_) => {
                panic!("expected rejected Ed25519 evidence parsing result")
            }
        }
    }

    #[test]
    fn report_documents_b1c3_pure_parser_scope() {
        let report = b1c_ed25519_evidence_parser_report();

        assert_eq!(report.phase, PHASE_41K6_B1C_3_ED25519_EVIDENCE_PARSER_PHASE);
        assert_eq!(
            report.version,
            PHASE_41K6_B1C_3_ED25519_EVIDENCE_PARSER_VERSION
        );
        assert!(report.pure_parser_only);
        assert!(!report.reads_instructions_sysvar);
        assert!(report.supports_single_signature_layout);
        assert!(!report.supports_multi_signature_layout);
        assert!(!report.supports_cross_instruction_offsets);
        assert!(report.requires_current_instruction_sentinel);
        assert!(report.rejects_offsets_overlapping_header);
        assert!(!report.re_verifies_ed25519_signature);
        assert!(!report.binds_payload_hash);
        assert!(!report.validates_guardian_membership);
        assert!(!report.counts_unique_guardians);
        assert!(!report.authorizes_handler_execution);
        assert!(!report.processed_event_marking_enabled);
        assert!(!report.cpi_enabled);
        assert!(!report.live_route_enabled);
    }

    #[test]
    fn parses_valid_single_signature_ed25519_instruction_data() {
        let message = b"consume_gateway_mint_authorization_v1_hash";
        let data = valid_instruction_data(message);

        let result = parse_b1c_single_ed25519_precompile_evidence(7, &data);

        match result {
            B1CEd25519EvidenceParsingResult::Parsed(parsed) => {
                assert_eq!(parsed.status, B1CEd25519EvidenceParsingStatus::Parsed);
                assert_eq!(parsed.source_instruction_index, 7);
                assert_eq!(parsed.signer_public_key, [0xA7; ED25519_PUBLIC_KEY_LEN]);
                assert_eq!(parsed.signature, [0x51; ED25519_SIGNATURE_LEN]);
                assert_eq!(parsed.signed_message, message);
                assert_eq!(parsed.instruction_data_len, data.len());
                assert_eq!(
                    parsed.signature_offset,
                    ED25519_SINGLE_SIGNATURE_HEADER_LEN as u16
                );
                assert_eq!(
                    parsed.public_key_offset,
                    ED25519_SINGLE_SIGNATURE_HEADER_LEN as u16 + ED25519_SIGNATURE_LEN as u16
                );
                assert_eq!(
                    parsed.message_data_offset,
                    ED25519_SINGLE_SIGNATURE_HEADER_LEN as u16
                        + ED25519_SIGNATURE_LEN as u16
                        + ED25519_PUBLIC_KEY_LEN as u16
                );
                assert_eq!(parsed.message_data_size, message.len() as u16);
                assert!(parsed.runtime_verified_by_ed25519_precompile);
                assert!(parsed.parsed_from_prior_ed25519_instruction);
                assert!(parsed.single_signature_layout);
                assert!(parsed.self_contained_current_instruction_offsets_only);
                assert!(!parsed.accepts_caller_provided_signature_claims);
                assert!(!parsed.accepts_frontend_or_watcher_proof);
                assert!(!parsed.binds_payload_hash);
                assert!(!parsed.validates_guardian_membership);
                assert!(!parsed.counts_unique_guardians);
                assert!(!parsed.authorization_enabled);
                assert!(!parsed.processed_event_marking_enabled);
                assert!(!parsed.cpi_enabled);
                assert!(!parsed.live_route_enabled);
            }
            B1CEd25519EvidenceParsingResult::Rejected(rejected) => {
                panic!("expected parsed evidence, got {rejected:?}")
            }
        }
    }

    #[test]
    fn rejects_too_short_instruction_data() {
        assert_rejected(
            parse_b1c_single_ed25519_precompile_evidence(0, &[1, 0, 0]),
            B1CEd25519EvidenceParsingRejectionKind::InstructionDataTooShort,
        );
    }

    #[test]
    fn rejects_zero_signatures() {
        let mut data = valid_instruction_data(b"payload");
        data[0] = 0;

        assert_rejected(
            parse_b1c_single_ed25519_precompile_evidence(0, &data),
            B1CEd25519EvidenceParsingRejectionKind::SignatureCountZero,
        );
    }

    #[test]
    fn rejects_more_than_one_signature() {
        let mut data = valid_instruction_data(b"payload");
        data[0] = 2;

        assert_rejected(
            parse_b1c_single_ed25519_precompile_evidence(0, &data),
            B1CEd25519EvidenceParsingRejectionKind::MultipleSignaturesUnsupported,
        );
    }

    #[test]
    fn rejects_nonzero_padding() {
        let mut data = valid_instruction_data(b"payload");
        data[1] = 1;

        assert_rejected(
            parse_b1c_single_ed25519_precompile_evidence(0, &data),
            B1CEd25519EvidenceParsingRejectionKind::NonZeroPadding,
        );
    }

    #[test]
    fn rejects_signature_offset_out_of_bounds() {
        let mut data = valid_instruction_data(b"payload");
        let offset = (data.len() - 8) as u16;
        data[2..4].copy_from_slice(&offset.to_le_bytes());

        assert_rejected(
            parse_b1c_single_ed25519_precompile_evidence(0, &data),
            B1CEd25519EvidenceParsingRejectionKind::SignatureOffsetOutOfBounds,
        );
    }

    #[test]
    fn rejects_public_key_offset_out_of_bounds() {
        let mut data = valid_instruction_data(b"payload");
        let offset = (data.len() - 8) as u16;
        data[6..8].copy_from_slice(&offset.to_le_bytes());

        assert_rejected(
            parse_b1c_single_ed25519_precompile_evidence(0, &data),
            B1CEd25519EvidenceParsingRejectionKind::PublicKeyOffsetOutOfBounds,
        );
    }

    #[test]
    fn rejects_message_offset_out_of_bounds() {
        let mut data = valid_instruction_data(b"payload");
        let offset = (data.len() + 1) as u16;
        data[10..12].copy_from_slice(&offset.to_le_bytes());

        assert_rejected(
            parse_b1c_single_ed25519_precompile_evidence(0, &data),
            B1CEd25519EvidenceParsingRejectionKind::MessageOffsetOutOfBounds,
        );
    }

    #[test]
    fn rejects_zero_message_length() {
        let mut data = valid_instruction_data(b"payload");
        data[12..14].copy_from_slice(&0u16.to_le_bytes());

        assert_rejected(
            parse_b1c_single_ed25519_precompile_evidence(0, &data),
            B1CEd25519EvidenceParsingRejectionKind::MessageLengthZero,
        );
    }

    #[test]
    fn rejects_cross_instruction_signature_index() {
        let mut data = valid_instruction_data(b"payload");
        data[4..6].copy_from_slice(&0u16.to_le_bytes());

        assert_rejected(
            parse_b1c_single_ed25519_precompile_evidence(0, &data),
            B1CEd25519EvidenceParsingRejectionKind::SignatureInstructionIndexNotCurrent,
        );
    }

    #[test]
    fn rejects_cross_instruction_public_key_index() {
        let mut data = valid_instruction_data(b"payload");
        data[8..10].copy_from_slice(&0u16.to_le_bytes());

        assert_rejected(
            parse_b1c_single_ed25519_precompile_evidence(0, &data),
            B1CEd25519EvidenceParsingRejectionKind::PublicKeyInstructionIndexNotCurrent,
        );
    }

    #[test]
    fn rejects_cross_instruction_message_index() {
        let mut data = valid_instruction_data(b"payload");
        data[14..16].copy_from_slice(&0u16.to_le_bytes());

        assert_rejected(
            parse_b1c_single_ed25519_precompile_evidence(0, &data),
            B1CEd25519EvidenceParsingRejectionKind::MessageInstructionIndexNotCurrent,
        );
    }

    #[test]
    fn rejects_signature_offset_overlapping_header() {
        let mut data = valid_instruction_data(b"payload");
        data[2..4].copy_from_slice(&4u16.to_le_bytes());

        assert_rejected(
            parse_b1c_single_ed25519_precompile_evidence(0, &data),
            B1CEd25519EvidenceParsingRejectionKind::SignatureOffsetOverlapsHeader,
        );
    }

    #[test]
    fn rejects_public_key_offset_overlapping_header() {
        let mut data = valid_instruction_data(b"payload");
        data[6..8].copy_from_slice(&4u16.to_le_bytes());

        assert_rejected(
            parse_b1c_single_ed25519_precompile_evidence(0, &data),
            B1CEd25519EvidenceParsingRejectionKind::PublicKeyOffsetOverlapsHeader,
        );
    }

    #[test]
    fn rejects_message_offset_overlapping_header() {
        let mut data = valid_instruction_data(b"payload");
        data[10..12].copy_from_slice(&4u16.to_le_bytes());

        assert_rejected(
            parse_b1c_single_ed25519_precompile_evidence(0, &data),
            B1CEd25519EvidenceParsingRejectionKind::MessageOffsetOverlapsHeader,
        );
    }
}
