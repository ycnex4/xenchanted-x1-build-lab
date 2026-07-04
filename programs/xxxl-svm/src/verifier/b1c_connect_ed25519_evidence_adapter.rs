use solana_program::ed25519_program;

use super::b1c_ed25519_evidence_parser::{
    parse_b1c_single_ed25519_precompile_evidence, B1CEd25519EvidenceParsingRejectionKind,
    B1CEd25519EvidenceParsingResult, B1CParsedEd25519Evidence,
};
use super::checked_prior_instruction_loading_runtime_boundary::Phase41D3_2_2CheckedPriorInstructionLoadingResult;
use super::instructions_sysvar_live_wiring_boundary::{
    derive_phase_41k_1_from_checked_prior_loading, Phase41K1InstructionsSysvarLiveWiringStatus,
};

pub const PHASE_41K6_B1C_3_CONNECT_ED25519_EVIDENCE_PHASE: &str = "41K.6-B1C.3-connect";
pub const PHASE_41K6_B1C_3_CONNECT_ED25519_EVIDENCE_VERSION: &str = "0.1.0";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B1CConnectedEd25519EvidenceStatus {
    ParsedPriorEd25519Evidence,
    NoParsedPriorEd25519Evidence,
    SourceRejected,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct B1CConnectedEd25519EvidenceParseRejection {
    pub source_instruction_index: usize,
    pub instruction_data_len: usize,
    pub kind: B1CEd25519EvidenceParsingRejectionKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct B1CConnectedEd25519Evidence {
    pub status: B1CConnectedEd25519EvidenceStatus,
    pub source_status: Phase41K1InstructionsSysvarLiveWiringStatus,
    pub current_instruction_index: Option<usize>,
    pub loaded_prior_instruction_count: usize,
    pub inspected_prior_instruction_count: usize,
    pub discarded_non_ed25519_prior_instruction_count: usize,
    pub prior_ed25519_precompile_count: usize,
    pub source_descriptor_count: usize,
    pub parsed_evidence_count: usize,
    pub rejected_evidence_count: usize,
    pub parsed_evidence: Vec<B1CParsedEd25519Evidence>,
    pub parse_rejections: Vec<B1CConnectedEd25519EvidenceParseRejection>,
    pub consumes_checked_prior_instruction_loading_result: bool,
    pub consumes_phase_41k1_live_wiring_boundary: bool,
    pub parses_with_b1c3_pure_parser: bool,
    pub accepts_caller_provided_instruction_bytes: bool,
    pub accepts_frontend_or_watcher_ed25519_proof: bool,
    pub binds_payload_hash: bool,
    pub validates_guardian_membership: bool,
    pub deduplicates_guardians: bool,
    pub counts_unique_guardians: bool,
    pub authorization_enabled: bool,
    pub processed_event_marking_enabled: bool,
    pub cpi_enabled: bool,
    pub live_route_enabled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct B1CConnectEd25519EvidenceAdapterReport {
    pub phase: &'static str,
    pub version: &'static str,
    pub consumes_checked_prior_instruction_loading_result: bool,
    pub consumes_phase_41k1_live_wiring_boundary: bool,
    pub mutates_b1c2_descriptors: bool,
    pub reads_instructions_sysvar_directly: bool,
    pub parses_with_b1c3_pure_parser: bool,
    pub computes_expected_payload_hash: bool,
    pub binds_payload_hash: bool,
    pub validates_guardian_membership: bool,
    pub deduplicates_guardians: bool,
    pub counts_unique_guardians: bool,
    pub authorizes_handler_execution: bool,
    pub processed_event_marking_enabled: bool,
    pub cpi_enabled: bool,
    pub live_route_enabled: bool,
}

pub const B1C_CONNECT_ED25519_EVIDENCE_ADAPTER_REPORT: B1CConnectEd25519EvidenceAdapterReport =
    B1CConnectEd25519EvidenceAdapterReport {
        phase: PHASE_41K6_B1C_3_CONNECT_ED25519_EVIDENCE_PHASE,
        version: PHASE_41K6_B1C_3_CONNECT_ED25519_EVIDENCE_VERSION,
        consumes_checked_prior_instruction_loading_result: true,
        consumes_phase_41k1_live_wiring_boundary: true,
        mutates_b1c2_descriptors: false,
        reads_instructions_sysvar_directly: false,
        parses_with_b1c3_pure_parser: true,
        computes_expected_payload_hash: false,
        binds_payload_hash: false,
        validates_guardian_membership: false,
        deduplicates_guardians: false,
        counts_unique_guardians: false,
        authorizes_handler_execution: false,
        processed_event_marking_enabled: false,
        cpi_enabled: false,
        live_route_enabled: false,
    };

pub fn b1c_connect_ed25519_evidence_adapter_report(
) -> &'static B1CConnectEd25519EvidenceAdapterReport {
    &B1C_CONNECT_ED25519_EVIDENCE_ADAPTER_REPORT
}

pub fn connect_b1c_checked_prior_loading_to_parsed_ed25519_evidence(
    source: &Phase41D3_2_2CheckedPriorInstructionLoadingResult,
) -> B1CConnectedEd25519Evidence {
    let phase_41k1_result = derive_phase_41k_1_from_checked_prior_loading(source);

    if phase_41k1_result.status
        != Phase41K1InstructionsSysvarLiveWiringStatus::PriorEd25519PrecompileInstructionsLoaded
    {
        return result(
            B1CConnectedEd25519EvidenceStatus::SourceRejected,
            phase_41k1_result.status,
            phase_41k1_result.current_instruction_index,
            phase_41k1_result.loaded_prior_instruction_count,
            phase_41k1_result.inspected_prior_instruction_count,
            phase_41k1_result.discarded_non_ed25519_prior_instruction_count,
            phase_41k1_result.prior_ed25519_precompile_count,
            phase_41k1_result.prior_ed25519_precompile_count,
            Vec::new(),
            Vec::new(),
        );
    }

    let mut parsed_evidence = Vec::new();
    let mut parse_rejections = Vec::new();

    for loaded in source.loaded_prior_instructions.iter() {
        if loaded.instruction.program_id != ed25519_program::id() {
            continue;
        }

        match parse_b1c_single_ed25519_precompile_evidence(
            loaded.instruction_index,
            &loaded.instruction.data,
        ) {
            B1CEd25519EvidenceParsingResult::Parsed(parsed) => {
                parsed_evidence.push(parsed);
            }
            B1CEd25519EvidenceParsingResult::Rejected(rejected) => {
                parse_rejections.push(B1CConnectedEd25519EvidenceParseRejection {
                    source_instruction_index: rejected.source_instruction_index,
                    instruction_data_len: rejected.instruction_data_len,
                    kind: rejected.kind,
                });
            }
        }
    }

    let status = if parsed_evidence.is_empty() {
        B1CConnectedEd25519EvidenceStatus::NoParsedPriorEd25519Evidence
    } else {
        B1CConnectedEd25519EvidenceStatus::ParsedPriorEd25519Evidence
    };

    result(
        status,
        phase_41k1_result.status,
        phase_41k1_result.current_instruction_index,
        phase_41k1_result.loaded_prior_instruction_count,
        phase_41k1_result.inspected_prior_instruction_count,
        phase_41k1_result.discarded_non_ed25519_prior_instruction_count,
        phase_41k1_result.prior_ed25519_precompile_count,
        phase_41k1_result.prior_ed25519_precompile_count,
        parsed_evidence,
        parse_rejections,
    )
}

fn result(
    status: B1CConnectedEd25519EvidenceStatus,
    source_status: Phase41K1InstructionsSysvarLiveWiringStatus,
    current_instruction_index: Option<usize>,
    loaded_prior_instruction_count: usize,
    inspected_prior_instruction_count: usize,
    discarded_non_ed25519_prior_instruction_count: usize,
    prior_ed25519_precompile_count: usize,
    source_descriptor_count: usize,
    parsed_evidence: Vec<B1CParsedEd25519Evidence>,
    parse_rejections: Vec<B1CConnectedEd25519EvidenceParseRejection>,
) -> B1CConnectedEd25519Evidence {
    B1CConnectedEd25519Evidence {
        status,
        source_status,
        current_instruction_index,
        loaded_prior_instruction_count,
        inspected_prior_instruction_count,
        discarded_non_ed25519_prior_instruction_count,
        prior_ed25519_precompile_count,
        source_descriptor_count,
        parsed_evidence_count: parsed_evidence.len(),
        rejected_evidence_count: parse_rejections.len(),
        parsed_evidence,
        parse_rejections,
        consumes_checked_prior_instruction_loading_result: true,
        consumes_phase_41k1_live_wiring_boundary: true,
        parses_with_b1c3_pure_parser: true,
        accepts_caller_provided_instruction_bytes: false,
        accepts_frontend_or_watcher_ed25519_proof: false,
        binds_payload_hash: false,
        validates_guardian_membership: false,
        deduplicates_guardians: false,
        counts_unique_guardians: false,
        authorization_enabled: false,
        processed_event_marking_enabled: false,
        cpi_enabled: false,
        live_route_enabled: false,
    }
}

#[cfg(test)]
mod tests {
    use solana_program::{ed25519_program, instruction::Instruction, pubkey::Pubkey};

    use super::*;
    use crate::verifier::checked_prior_instruction_loading_runtime_boundary::{
        Phase41D3_2_2CheckedPriorInstructionLoadingResult,
        Phase41D3_2_2CheckedPriorInstructionLoadingStatus, Phase41D3_2_2LoadedPriorInstruction,
    };
    use crate::verifier::instructions_sysvar_access_contract_model::Phase41BRejectionCase;
    use crate::verifier::{
        ED25519_CURRENT_INSTRUCTION_INDEX_SENTINEL, ED25519_PUBLIC_KEY_LEN, ED25519_SIGNATURE_LEN,
        ED25519_SINGLE_SIGNATURE_HEADER_LEN,
    };

    fn valid_ed25519_instruction_data(
        message: &[u8],
        signature_byte: u8,
        pubkey_byte: u8,
    ) -> Vec<u8> {
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

        data.extend_from_slice(&[signature_byte; ED25519_SIGNATURE_LEN]);
        data.extend_from_slice(&[pubkey_byte; ED25519_PUBLIC_KEY_LEN]);
        data.extend_from_slice(message);

        data
    }

    fn malformed_ed25519_instruction_data_zero_message_len() -> Vec<u8> {
        let mut data = valid_ed25519_instruction_data(b"payload", 0x11, 0x22);
        data[12..14].copy_from_slice(&0u16.to_le_bytes());
        data
    }

    fn loaded_prior_instruction(
        instruction_index: usize,
        program_id: Pubkey,
        data: Vec<u8>,
    ) -> Phase41D3_2_2LoadedPriorInstruction {
        Phase41D3_2_2LoadedPriorInstruction {
            instruction_index,
            instruction: Instruction {
                program_id,
                accounts: Vec::new(),
                data,
            },
            loaded_instruction_is_runtime_data_only: true,
            is_evidence: false,
            authorizes_execution: false,
        }
    }

    fn checked_prior_loading_result(
        status: Phase41D3_2_2CheckedPriorInstructionLoadingStatus,
        current_instruction_index: Option<usize>,
        loaded_prior_instructions: Vec<Phase41D3_2_2LoadedPriorInstruction>,
        rejection_case: Option<Phase41BRejectionCase>,
    ) -> Phase41D3_2_2CheckedPriorInstructionLoadingResult {
        let count = loaded_prior_instructions.len();

        Phase41D3_2_2CheckedPriorInstructionLoadingResult {
            status,
            rejection_case,
            current_instruction_index,
            prior_index_count: count,
            attempted_loading_count: count,
            loaded_instruction_count: count,
            failed_instruction_index: None,
            loaded_prior_instructions,
            consumes_phase_41d3_2_1_prior_range: true,
            uses_instructions_sysvar_account_info: true,
            checks_instructions_sysvar_program_id: true,
            iterates_prior_indexes_lazily: true,
            empty_prior_range_causes_no_loading_attempt: true,
            checked_prior_instruction_loading_enabled: true,
            prior_instruction_loading_enabled: true,
            raw_instructions_sysvar_parser_implemented: false,
            load_instruction_called: true,
            load_instruction_enabled: true,
            load_instruction_at_checked_used: true,
            unchecked_instruction_loading_used: false,
            prefilter_enabled: false,
            phase_41c3_descriptor_construction_enabled: false,
            locates_prior_ed25519_instruction: false,
            accepts_verification_evidence: false,
            authorizes_execution: false,
            mutates_runtime_state: false,
        }
    }

    fn loaded_source(
        loaded_prior_instructions: Vec<Phase41D3_2_2LoadedPriorInstruction>,
    ) -> Phase41D3_2_2CheckedPriorInstructionLoadingResult {
        checked_prior_loading_result(
            Phase41D3_2_2CheckedPriorInstructionLoadingStatus::CheckedPriorInstructionsLoaded,
            Some(5),
            loaded_prior_instructions,
            None,
        )
    }

    fn assert_all_execution_flags_false(result: &B1CConnectedEd25519Evidence) {
        assert!(!result.accepts_caller_provided_instruction_bytes);
        assert!(!result.accepts_frontend_or_watcher_ed25519_proof);
        assert!(!result.binds_payload_hash);
        assert!(!result.validates_guardian_membership);
        assert!(!result.deduplicates_guardians);
        assert!(!result.counts_unique_guardians);
        assert!(!result.authorization_enabled);
        assert!(!result.processed_event_marking_enabled);
        assert!(!result.cpi_enabled);
        assert!(!result.live_route_enabled);
    }

    #[test]
    fn report_documents_b1c3_connect_adapter_scope() {
        let report = b1c_connect_ed25519_evidence_adapter_report();

        assert_eq!(
            report.phase,
            PHASE_41K6_B1C_3_CONNECT_ED25519_EVIDENCE_PHASE
        );
        assert_eq!(
            report.version,
            PHASE_41K6_B1C_3_CONNECT_ED25519_EVIDENCE_VERSION
        );
        assert!(report.consumes_checked_prior_instruction_loading_result);
        assert!(report.consumes_phase_41k1_live_wiring_boundary);
        assert!(!report.mutates_b1c2_descriptors);
        assert!(!report.reads_instructions_sysvar_directly);
        assert!(report.parses_with_b1c3_pure_parser);
        assert!(!report.computes_expected_payload_hash);
        assert!(!report.binds_payload_hash);
        assert!(!report.validates_guardian_membership);
        assert!(!report.deduplicates_guardians);
        assert!(!report.counts_unique_guardians);
        assert!(!report.authorizes_handler_execution);
        assert!(!report.processed_event_marking_enabled);
        assert!(!report.cpi_enabled);
        assert!(!report.live_route_enabled);
    }

    #[test]
    fn parses_two_valid_prior_ed25519_instructions() {
        let source = loaded_source(vec![
            loaded_prior_instruction(
                0,
                ed25519_program::id(),
                valid_ed25519_instruction_data(b"payload-a", 0xA1, 0xB1),
            ),
            loaded_prior_instruction(
                1,
                ed25519_program::id(),
                valid_ed25519_instruction_data(b"payload-b", 0xA2, 0xB2),
            ),
        ]);

        let result = connect_b1c_checked_prior_loading_to_parsed_ed25519_evidence(&source);

        assert_eq!(
            result.status,
            B1CConnectedEd25519EvidenceStatus::ParsedPriorEd25519Evidence
        );
        assert_eq!(
            result.source_status,
            Phase41K1InstructionsSysvarLiveWiringStatus::PriorEd25519PrecompileInstructionsLoaded
        );
        assert_eq!(result.current_instruction_index, Some(5));
        assert_eq!(result.loaded_prior_instruction_count, 2);
        assert_eq!(result.inspected_prior_instruction_count, 2);
        assert_eq!(result.discarded_non_ed25519_prior_instruction_count, 0);
        assert_eq!(result.prior_ed25519_precompile_count, 2);
        assert_eq!(result.source_descriptor_count, 2);
        assert_eq!(result.parsed_evidence_count, 2);
        assert_eq!(result.rejected_evidence_count, 0);
        assert_eq!(result.parsed_evidence[0].signed_message, b"payload-a");
        assert_eq!(result.parsed_evidence[0].signer_public_key, [0xB1; 32]);
        assert_eq!(result.parsed_evidence[1].signed_message, b"payload-b");
        assert_eq!(result.parsed_evidence[1].signer_public_key, [0xB2; 32]);
        assert_all_execution_flags_false(&result);
    }

    #[test]
    fn mixed_prior_instructions_parse_only_ed25519_and_discard_non_ed25519() {
        let source = loaded_source(vec![
            loaded_prior_instruction(0, Pubkey::new_unique(), vec![0x99; 12]),
            loaded_prior_instruction(
                1,
                ed25519_program::id(),
                valid_ed25519_instruction_data(b"payload", 0xC1, 0xD1),
            ),
        ]);

        let result = connect_b1c_checked_prior_loading_to_parsed_ed25519_evidence(&source);

        assert_eq!(
            result.status,
            B1CConnectedEd25519EvidenceStatus::ParsedPriorEd25519Evidence
        );
        assert_eq!(result.loaded_prior_instruction_count, 2);
        assert_eq!(result.inspected_prior_instruction_count, 2);
        assert_eq!(result.discarded_non_ed25519_prior_instruction_count, 1);
        assert_eq!(result.prior_ed25519_precompile_count, 1);
        assert_eq!(result.parsed_evidence_count, 1);
        assert_eq!(result.rejected_evidence_count, 0);
        assert_eq!(result.parsed_evidence[0].signed_message, b"payload");
        assert_all_execution_flags_false(&result);
    }

    #[test]
    fn source_rejected_when_missing_instructions_sysvar_boundary_rejects() {
        let source = checked_prior_loading_result(
            Phase41D3_2_2CheckedPriorInstructionLoadingStatus::MissingInstructionsSysvar,
            Some(5),
            Vec::new(),
            Some(Phase41BRejectionCase::MissingInstructionsSysvar),
        );

        let result = connect_b1c_checked_prior_loading_to_parsed_ed25519_evidence(&source);

        assert_eq!(
            result.status,
            B1CConnectedEd25519EvidenceStatus::SourceRejected
        );
        assert_eq!(
            result.source_status,
            Phase41K1InstructionsSysvarLiveWiringStatus::MissingInstructionsSysvar
        );
        assert_eq!(result.parsed_evidence_count, 0);
        assert_eq!(result.rejected_evidence_count, 0);
        assert_all_execution_flags_false(&result);
    }

    #[test]
    fn source_rejected_when_no_prior_instructions() {
        let source = checked_prior_loading_result(
            Phase41D3_2_2CheckedPriorInstructionLoadingStatus::EmptyPriorRangeNoLoadingAttempted,
            Some(0),
            Vec::new(),
            None,
        );

        let result = connect_b1c_checked_prior_loading_to_parsed_ed25519_evidence(&source);

        assert_eq!(
            result.status,
            B1CConnectedEd25519EvidenceStatus::SourceRejected
        );
        assert_eq!(
            result.source_status,
            Phase41K1InstructionsSysvarLiveWiringStatus::NoPriorInstructions
        );
        assert_all_execution_flags_false(&result);
    }

    #[test]
    fn source_rejected_when_no_prior_ed25519_instructions() {
        let source = loaded_source(vec![
            loaded_prior_instruction(0, Pubkey::new_unique(), vec![0x10; 16]),
            loaded_prior_instruction(1, Pubkey::new_unique(), vec![0x20; 16]),
        ]);

        let result = connect_b1c_checked_prior_loading_to_parsed_ed25519_evidence(&source);

        assert_eq!(
            result.status,
            B1CConnectedEd25519EvidenceStatus::SourceRejected
        );
        assert_eq!(
            result.source_status,
            Phase41K1InstructionsSysvarLiveWiringStatus::NoPriorEd25519PrecompileInstructions
        );
        assert_eq!(result.discarded_non_ed25519_prior_instruction_count, 2);
        assert_all_execution_flags_false(&result);
    }

    #[test]
    fn no_parsed_prior_ed25519_evidence_when_all_parser_attempts_reject() {
        let source = loaded_source(vec![
            loaded_prior_instruction(
                0,
                ed25519_program::id(),
                malformed_ed25519_instruction_data_zero_message_len(),
            ),
            loaded_prior_instruction(
                1,
                ed25519_program::id(),
                malformed_ed25519_instruction_data_zero_message_len(),
            ),
        ]);

        let result = connect_b1c_checked_prior_loading_to_parsed_ed25519_evidence(&source);

        assert_eq!(
            result.status,
            B1CConnectedEd25519EvidenceStatus::NoParsedPriorEd25519Evidence
        );
        assert_eq!(
            result.source_status,
            Phase41K1InstructionsSysvarLiveWiringStatus::PriorEd25519PrecompileInstructionsLoaded
        );
        assert_eq!(result.prior_ed25519_precompile_count, 2);
        assert_eq!(result.parsed_evidence_count, 0);
        assert_eq!(result.rejected_evidence_count, 2);
        assert_eq!(
            result.parse_rejections[0].kind,
            B1CEd25519EvidenceParsingRejectionKind::MessageLengthZero
        );
        assert_all_execution_flags_false(&result);
    }

    #[test]
    fn partial_parse_records_valid_evidence_and_parse_rejection() {
        let source = loaded_source(vec![
            loaded_prior_instruction(
                0,
                ed25519_program::id(),
                valid_ed25519_instruction_data(b"payload-ok", 0xE1, 0xF1),
            ),
            loaded_prior_instruction(
                1,
                ed25519_program::id(),
                malformed_ed25519_instruction_data_zero_message_len(),
            ),
        ]);

        let result = connect_b1c_checked_prior_loading_to_parsed_ed25519_evidence(&source);

        assert_eq!(
            result.status,
            B1CConnectedEd25519EvidenceStatus::ParsedPriorEd25519Evidence
        );
        assert_eq!(result.parsed_evidence_count, 1);
        assert_eq!(result.rejected_evidence_count, 1);
        assert_eq!(result.parsed_evidence[0].signed_message, b"payload-ok");
        assert_eq!(
            result.parse_rejections[0].kind,
            B1CEd25519EvidenceParsingRejectionKind::MessageLengthZero
        );
        assert_all_execution_flags_false(&result);
    }

    #[test]
    fn sentinel_constant_matches_current_instruction_reference() {
        assert_eq!(ED25519_CURRENT_INSTRUCTION_INDEX_SENTINEL, u16::MAX);
    }
}
