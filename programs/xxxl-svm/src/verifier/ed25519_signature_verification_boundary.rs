use solana_program::ed25519_program;

use super::checked_ed25519_byte_extraction_boundary::{
    Phase41F_1CheckedByteExtractionResult, Phase41F_1CheckedByteExtractionStatus,
};
use super::checked_prior_instruction_loading_runtime_boundary::{
    Phase41D3_2_2CheckedPriorInstructionLoadingResult, Phase41D3_2_2LoadedPriorInstruction,
};
use super::ed25519_instruction_byte_parsing_boundary::{
    Phase41E_1ByteRange, Phase41E_1Ed25519ByteParsingResult,
    Phase41E_1Ed25519InstructionIndexReferences, ED25519_CURRENT_INSTRUCTION_INDEX,
};
use super::instructions_sysvar_access_contract_model::{
    Phase41BRejectionCase, Phase41BSafetyFlags,
};

pub const PHASE_41F_2_ED25519_SIGNATURE_VERIFICATION_BOUNDARY_VERSION: &str = "0.1.0";
pub const PHASE_41F_2_ED25519_SIGNATURE_VERIFICATION_BOUNDARY_PHASE: &str = "41F.2";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase41F_2Ed25519VerificationModel {
    NativeEd25519Instruction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase41F_2Ed25519SignatureVerificationStatus {
    Ed25519ByteSlicesNotExtracted,
    MatchedInstructionIndexUnavailable,
    MatchedInstructionIndexMismatch,
    ParsedOffsetsUnavailable,
    MatchedInstructionUnavailable,
    WrongEd25519ProgramId,
    NonRuntimeDataInstructionEntry,
    InstructionDataLengthMismatch,
    SelfReferenceBindingUnavailable,
    ExtractedRangesMismatchParsedRanges,
    NativeEd25519VerificationEstablished,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phase41F_2VerifiedEd25519SignatureRanges {
    pub signature_range: Phase41E_1ByteRange,
    pub public_key_range: Phase41E_1ByteRange,
    pub message_range: Phase41E_1ByteRange,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Phase41F_2Ed25519SignatureVerificationResult {
    pub status: Phase41F_2Ed25519SignatureVerificationStatus,
    pub rejection_case: Option<Phase41BRejectionCase>,
    pub verification_model: Option<Phase41F_2Ed25519VerificationModel>,
    pub matched_instruction_index: Option<usize>,
    pub instruction_data_len: usize,
    pub verified_ranges: Option<Phase41F_2VerifiedEd25519SignatureRanges>,
    pub safety_flags_are_cumulative_pipeline_capabilities: bool,
    pub safety_flags_are_local_module_capabilities: bool,
    pub consumes_phase_41f_1_checked_extraction: bool,
    pub consumes_phase_41e_1_parsed_offsets: bool,
    pub consumes_phase_41d3_2_2_loaded_prior_instructions: bool,
    pub checks_matched_instruction_index: bool,
    pub rechecks_ed25519_program_id: bool,
    pub rechecks_loaded_entry_runtime_data_only: bool,
    pub checks_instruction_data_length_matches_parse_result: bool,
    pub checks_extracted_ranges_match_parsed_ranges: bool,
    pub preserves_self_reference_binding: bool,
    pub model_a_native_abort_before_current_soundness_documented: bool,
    pub establishes_native_ed25519_verification: bool,
    pub performs_local_cryptographic_verification: bool,
    pub ed25519_signature_verification_performed: bool,
    pub message_payload_correctness_checked: bool,
    pub message_payload_correctness_deferred_to_later_gate: bool,
    pub cryptographic_signature_proof_accepted: bool,
    pub accepts_verification_evidence: bool,
    pub guardian_validity_accepted: bool,
    pub quorum_counting_enabled: bool,
    pub authorizes_execution: bool,
    pub replay_write_enabled: bool,
    pub mutates_runtime_state: bool,
    pub cpi_enabled: bool,
    pub spl_token_mint_to_enabled: bool,
    pub live_route_enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phase41F_2Ed25519SignatureVerificationBoundaryReport {
    pub phase: &'static str,
    pub version: &'static str,
    pub implemented_model: Phase41F_2Ed25519VerificationModel,
    pub safety_flags_are_cumulative_pipeline_capabilities: bool,
    pub safety_flags_are_local_module_capabilities: bool,
    pub consumes_phase_41f_1_checked_extraction: bool,
    pub consumes_phase_41e_1_parsed_offsets: bool,
    pub consumes_phase_41d3_2_2_loaded_prior_instructions: bool,
    pub checks_matched_instruction_index: bool,
    pub rechecks_ed25519_program_id: bool,
    pub rechecks_loaded_entry_runtime_data_only: bool,
    pub checks_instruction_data_length_matches_parse_result: bool,
    pub checks_extracted_ranges_match_parsed_ranges: bool,
    pub preserves_self_reference_binding: bool,
    pub model_a_native_abort_before_current_soundness_documented: bool,
    pub establishes_native_ed25519_verification: bool,
    pub performs_local_cryptographic_verification: bool,
    pub ed25519_signature_verification_performed: bool,
    pub message_payload_correctness_checked: bool,
    pub message_payload_correctness_deferred_to_later_gate: bool,
    pub cryptographic_signature_proof_accepted: bool,
    pub accepts_verification_evidence: bool,
    pub guardian_validity_accepted: bool,
    pub quorum_counting_enabled: bool,
    pub authorizes_execution: bool,
    pub replay_write_enabled: bool,
    pub mutates_runtime_state: bool,
    pub cpi_enabled: bool,
    pub spl_token_mint_to_enabled: bool,
    pub live_route_enabled: bool,
    pub safety_flags: Phase41BSafetyFlags,
}

pub const PHASE_41F_2_SAFETY_FLAGS: Phase41BSafetyFlags = Phase41BSafetyFlags {
    raw_instructions_sysvar_parser_implemented: false,
    account_info_parser_implemented: true,
    load_instruction_called: true,
    load_instruction_enabled: true,
    concrete_runtime_api_selected: true,
    current_instruction_identity_derived_from_runtime: true,
    ed25519_signature_verification_performed: true,
    cryptographic_signature_proof_accepted: false,
    verification_evidence_accepted: false,
    quorum_counting_enabled: false,
    authorization_enabled: false,
    replay_write_enabled: false,
    processed_event_marking_enabled: false,
    account_mutation_enabled: false,
    cpi_enabled: false,
    invoke_signed_enabled: false,
    spl_token_mint_to_enabled: false,
    process_instruction_handler_added: false,
    live_route_enabled: false,
};

pub const PHASE_41F_2_ED25519_SIGNATURE_VERIFICATION_BOUNDARY_REPORT:
    Phase41F_2Ed25519SignatureVerificationBoundaryReport =
    Phase41F_2Ed25519SignatureVerificationBoundaryReport {
        phase: PHASE_41F_2_ED25519_SIGNATURE_VERIFICATION_BOUNDARY_PHASE,
        version: PHASE_41F_2_ED25519_SIGNATURE_VERIFICATION_BOUNDARY_VERSION,
        implemented_model: Phase41F_2Ed25519VerificationModel::NativeEd25519Instruction,
        safety_flags_are_cumulative_pipeline_capabilities: true,
        safety_flags_are_local_module_capabilities: false,
        consumes_phase_41f_1_checked_extraction: true,
        consumes_phase_41e_1_parsed_offsets: true,
        consumes_phase_41d3_2_2_loaded_prior_instructions: true,
        checks_matched_instruction_index: true,
        rechecks_ed25519_program_id: true,
        rechecks_loaded_entry_runtime_data_only: true,
        checks_instruction_data_length_matches_parse_result: true,
        checks_extracted_ranges_match_parsed_ranges: true,
        preserves_self_reference_binding: true,
        model_a_native_abort_before_current_soundness_documented: true,
        establishes_native_ed25519_verification: true,
        performs_local_cryptographic_verification: false,
        ed25519_signature_verification_performed: true,
        message_payload_correctness_checked: false,
        message_payload_correctness_deferred_to_later_gate: true,
        cryptographic_signature_proof_accepted: false,
        accepts_verification_evidence: false,
        guardian_validity_accepted: false,
        quorum_counting_enabled: false,
        authorizes_execution: false,
        replay_write_enabled: false,
        mutates_runtime_state: false,
        cpi_enabled: false,
        spl_token_mint_to_enabled: false,
        live_route_enabled: false,
        safety_flags: PHASE_41F_2_SAFETY_FLAGS,
    };

pub fn establish_native_ed25519_signature_verification(
    loading_result: &Phase41D3_2_2CheckedPriorInstructionLoadingResult,
    parsing_result: &Phase41E_1Ed25519ByteParsingResult,
    extraction_result: &Phase41F_1CheckedByteExtractionResult,
) -> Phase41F_2Ed25519SignatureVerificationResult {
    if extraction_result.status
        != Phase41F_1CheckedByteExtractionStatus::CheckedEd25519ByteSlicesExtracted
        || extraction_result.extracted_slices.is_none()
    {
        return fail(
            Phase41F_2Ed25519SignatureVerificationStatus::Ed25519ByteSlicesNotExtracted,
            extraction_result
                .rejection_case
                .or(Some(Phase41BRejectionCase::MalformedEd25519InstructionData)),
            extraction_result.matched_instruction_index,
            extraction_result.instruction_data_len,
        );
    }

    let Some(matched_instruction_index) = parsing_result.matched_instruction_index else {
        return fail(
            Phase41F_2Ed25519SignatureVerificationStatus::MatchedInstructionIndexUnavailable,
            Some(Phase41BRejectionCase::Ed25519InstructionNotFound),
            None,
            parsing_result.instruction_data_len,
        );
    };

    if extraction_result.matched_instruction_index != Some(matched_instruction_index) {
        return fail(
            Phase41F_2Ed25519SignatureVerificationStatus::MatchedInstructionIndexMismatch,
            Some(Phase41BRejectionCase::MalformedEd25519InstructionData),
            Some(matched_instruction_index),
            parsing_result.instruction_data_len,
        );
    }

    let Some(parsed_offsets) = parsing_result.parsed_offsets else {
        return fail(
            Phase41F_2Ed25519SignatureVerificationStatus::ParsedOffsetsUnavailable,
            Some(Phase41BRejectionCase::MalformedEd25519InstructionData),
            Some(matched_instruction_index),
            parsing_result.instruction_data_len,
        );
    };

    let Some(loaded_entry) =
        find_loaded_entry_by_instruction_index(loading_result, matched_instruction_index)
    else {
        return fail(
            Phase41F_2Ed25519SignatureVerificationStatus::MatchedInstructionUnavailable,
            Some(Phase41BRejectionCase::Ed25519InstructionNotFound),
            Some(matched_instruction_index),
            parsing_result.instruction_data_len,
        );
    };

    let instruction_data_len = loaded_entry.instruction.data.len();

    if loaded_entry.instruction.program_id != ed25519_program::id() {
        return fail(
            Phase41F_2Ed25519SignatureVerificationStatus::WrongEd25519ProgramId,
            Some(Phase41BRejectionCase::WrongEd25519ProgramId),
            Some(matched_instruction_index),
            instruction_data_len,
        );
    }

    if !loaded_entry.loaded_instruction_is_runtime_data_only
        || loaded_entry.is_evidence
        || loaded_entry.authorizes_execution
    {
        return fail(
            Phase41F_2Ed25519SignatureVerificationStatus::NonRuntimeDataInstructionEntry,
            Some(Phase41BRejectionCase::UnreadableInstructionsSysvar),
            Some(matched_instruction_index),
            instruction_data_len,
        );
    }

    if instruction_data_len != parsing_result.instruction_data_len
        || instruction_data_len != extraction_result.instruction_data_len
    {
        return fail(
            Phase41F_2Ed25519SignatureVerificationStatus::InstructionDataLengthMismatch,
            Some(Phase41BRejectionCase::MalformedEd25519InstructionData),
            Some(matched_instruction_index),
            instruction_data_len,
        );
    }

    if !all_references_are_current_instruction(parsed_offsets.instruction_index_references) {
        return fail(
            Phase41F_2Ed25519SignatureVerificationStatus::SelfReferenceBindingUnavailable,
            Some(Phase41BRejectionCase::UnsupportedOffsetLayout),
            Some(matched_instruction_index),
            instruction_data_len,
        );
    }

    let Some(extracted_slices) = extraction_result.extracted_slices else {
        return fail(
            Phase41F_2Ed25519SignatureVerificationStatus::Ed25519ByteSlicesNotExtracted,
            Some(Phase41BRejectionCase::MalformedEd25519InstructionData),
            Some(matched_instruction_index),
            instruction_data_len,
        );
    };

    if extracted_slices.signature_range != parsed_offsets.signature_range
        || extracted_slices.public_key_range != parsed_offsets.public_key_range
        || extracted_slices.message_range != parsed_offsets.message_range
    {
        return fail(
            Phase41F_2Ed25519SignatureVerificationStatus::ExtractedRangesMismatchParsedRanges,
            Some(Phase41BRejectionCase::UnsupportedOffsetLayout),
            Some(matched_instruction_index),
            instruction_data_len,
        );
    }

    established(
        Some(matched_instruction_index),
        instruction_data_len,
        Phase41F_2VerifiedEd25519SignatureRanges {
            signature_range: parsed_offsets.signature_range,
            public_key_range: parsed_offsets.public_key_range,
            message_range: parsed_offsets.message_range,
        },
    )
}

pub fn phase_41f_2_ed25519_signature_verification_boundary_report(
) -> Phase41F_2Ed25519SignatureVerificationBoundaryReport {
    PHASE_41F_2_ED25519_SIGNATURE_VERIFICATION_BOUNDARY_REPORT
}

fn find_loaded_entry_by_instruction_index(
    loading_result: &Phase41D3_2_2CheckedPriorInstructionLoadingResult,
    matched_instruction_index: usize,
) -> Option<&Phase41D3_2_2LoadedPriorInstruction> {
    loading_result
        .loaded_prior_instructions
        .iter()
        .find(|loaded_entry| loaded_entry.instruction_index == matched_instruction_index)
}

fn all_references_are_current_instruction(
    references: Phase41E_1Ed25519InstructionIndexReferences,
) -> bool {
    references.signature_instruction_index == ED25519_CURRENT_INSTRUCTION_INDEX
        && references.public_key_instruction_index == ED25519_CURRENT_INSTRUCTION_INDEX
        && references.message_instruction_index == ED25519_CURRENT_INSTRUCTION_INDEX
}

fn established(
    matched_instruction_index: Option<usize>,
    instruction_data_len: usize,
    verified_ranges: Phase41F_2VerifiedEd25519SignatureRanges,
) -> Phase41F_2Ed25519SignatureVerificationResult {
    Phase41F_2Ed25519SignatureVerificationResult {
        status: Phase41F_2Ed25519SignatureVerificationStatus::NativeEd25519VerificationEstablished,
        rejection_case: None,
        verification_model: Some(Phase41F_2Ed25519VerificationModel::NativeEd25519Instruction),
        matched_instruction_index,
        instruction_data_len,
        verified_ranges: Some(verified_ranges),
        safety_flags_are_cumulative_pipeline_capabilities: true,
        safety_flags_are_local_module_capabilities: false,
        consumes_phase_41f_1_checked_extraction: true,
        consumes_phase_41e_1_parsed_offsets: true,
        consumes_phase_41d3_2_2_loaded_prior_instructions: true,
        checks_matched_instruction_index: true,
        rechecks_ed25519_program_id: true,
        rechecks_loaded_entry_runtime_data_only: true,
        checks_instruction_data_length_matches_parse_result: true,
        checks_extracted_ranges_match_parsed_ranges: true,
        preserves_self_reference_binding: true,
        model_a_native_abort_before_current_soundness_documented: true,
        establishes_native_ed25519_verification: true,
        performs_local_cryptographic_verification: false,
        ed25519_signature_verification_performed: true,
        message_payload_correctness_checked: false,
        message_payload_correctness_deferred_to_later_gate: true,
        cryptographic_signature_proof_accepted: false,
        accepts_verification_evidence: false,
        guardian_validity_accepted: false,
        quorum_counting_enabled: false,
        authorizes_execution: false,
        replay_write_enabled: false,
        mutates_runtime_state: false,
        cpi_enabled: false,
        spl_token_mint_to_enabled: false,
        live_route_enabled: false,
    }
}

fn fail(
    status: Phase41F_2Ed25519SignatureVerificationStatus,
    rejection_case: Option<Phase41BRejectionCase>,
    matched_instruction_index: Option<usize>,
    instruction_data_len: usize,
) -> Phase41F_2Ed25519SignatureVerificationResult {
    Phase41F_2Ed25519SignatureVerificationResult {
        status,
        rejection_case,
        verification_model: None,
        matched_instruction_index,
        instruction_data_len,
        verified_ranges: None,
        safety_flags_are_cumulative_pipeline_capabilities: true,
        safety_flags_are_local_module_capabilities: false,
        consumes_phase_41f_1_checked_extraction: true,
        consumes_phase_41e_1_parsed_offsets: true,
        consumes_phase_41d3_2_2_loaded_prior_instructions: true,
        checks_matched_instruction_index: true,
        rechecks_ed25519_program_id: true,
        rechecks_loaded_entry_runtime_data_only: true,
        checks_instruction_data_length_matches_parse_result: true,
        checks_extracted_ranges_match_parsed_ranges: true,
        preserves_self_reference_binding: true,
        model_a_native_abort_before_current_soundness_documented: true,
        establishes_native_ed25519_verification: false,
        performs_local_cryptographic_verification: false,
        ed25519_signature_verification_performed: false,
        message_payload_correctness_checked: false,
        message_payload_correctness_deferred_to_later_gate: true,
        cryptographic_signature_proof_accepted: false,
        accepts_verification_evidence: false,
        guardian_validity_accepted: false,
        quorum_counting_enabled: false,
        authorizes_execution: false,
        replay_write_enabled: false,
        mutates_runtime_state: false,
        cpi_enabled: false,
        spl_token_mint_to_enabled: false,
        live_route_enabled: false,
    }
}

#[cfg(test)]
mod tests {
    use solana_program::{ed25519_program, instruction::Instruction, pubkey::Pubkey};

    use super::super::checked_ed25519_byte_extraction_boundary::{
        extract_checked_ed25519_byte_slices, Phase41F_1CheckedByteExtractionStatus,
    };
    use super::super::checked_prior_instruction_loading_runtime_boundary::{
        Phase41D3_2_2CheckedPriorInstructionLoadingResult,
        Phase41D3_2_2CheckedPriorInstructionLoadingStatus, Phase41D3_2_2LoadedPriorInstruction,
    };
    use super::super::ed25519_instruction_byte_parsing_boundary::{
        parse_located_ed25519_instruction_bytes, ED25519_CURRENT_INSTRUCTION_INDEX,
        ED25519_PUBLIC_KEY_LEN, ED25519_SIGNATURE_LEN, ED25519_SINGLE_SIGNATURE_OFFSET_TABLE_LEN,
        SUPPORTED_ED25519_SIGNATURE_COUNT,
    };
    use super::super::prefilter_phase_41c3_candidate_descriptor_runtime_boundary::{
        Phase41D3_2_3PrefilterDescriptorResult, Phase41D3_2_3PrefilterDescriptorStatus,
    };
    use super::*;

    fn push_u16_le(data: &mut Vec<u8>, value: u16) {
        data.extend_from_slice(&value.to_le_bytes());
    }

    fn valid_ed25519_instruction_data(message_len: usize) -> Vec<u8> {
        let mut data = Vec::new();

        data.push(SUPPORTED_ED25519_SIGNATURE_COUNT);
        data.push(0);

        push_u16_le(&mut data, ED25519_SINGLE_SIGNATURE_OFFSET_TABLE_LEN as u16);
        push_u16_le(&mut data, ED25519_CURRENT_INSTRUCTION_INDEX);
        push_u16_le(
            &mut data,
            (ED25519_SINGLE_SIGNATURE_OFFSET_TABLE_LEN + ED25519_SIGNATURE_LEN) as u16,
        );
        push_u16_le(&mut data, ED25519_CURRENT_INSTRUCTION_INDEX);
        push_u16_le(
            &mut data,
            (ED25519_SINGLE_SIGNATURE_OFFSET_TABLE_LEN
                + ED25519_SIGNATURE_LEN
                + ED25519_PUBLIC_KEY_LEN) as u16,
        );
        push_u16_le(&mut data, message_len as u16);
        push_u16_le(&mut data, ED25519_CURRENT_INSTRUCTION_INDEX);

        data.extend(core::iter::repeat(1u8).take(ED25519_SIGNATURE_LEN));
        data.extend(core::iter::repeat(2u8).take(ED25519_PUBLIC_KEY_LEN));
        data.extend(core::iter::repeat(3u8).take(message_len));

        data
    }

    fn instruction_with_program_id(program_id: Pubkey, data: Vec<u8>) -> Instruction {
        Instruction {
            program_id,
            accounts: Vec::new(),
            data,
        }
    }

    fn ed25519_instruction_with_data(data: Vec<u8>) -> Instruction {
        instruction_with_program_id(ed25519_program::id(), data)
    }

    fn loaded_prior_instruction(
        instruction_index: usize,
        instruction: Instruction,
    ) -> Phase41D3_2_2LoadedPriorInstruction {
        Phase41D3_2_2LoadedPriorInstruction {
            instruction_index,
            instruction,
            loaded_instruction_is_runtime_data_only: true,
            is_evidence: false,
            authorizes_execution: false,
        }
    }

    fn loading_result(
        loaded_prior_instructions: Vec<Phase41D3_2_2LoadedPriorInstruction>,
    ) -> Phase41D3_2_2CheckedPriorInstructionLoadingResult {
        Phase41D3_2_2CheckedPriorInstructionLoadingResult {
            status:
                Phase41D3_2_2CheckedPriorInstructionLoadingStatus::CheckedPriorInstructionsLoaded,
            rejection_case: None,
            current_instruction_index: Some(3),
            prior_index_count: loaded_prior_instructions.len(),
            attempted_loading_count: loaded_prior_instructions.len(),
            loaded_instruction_count: loaded_prior_instructions.len(),
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
            prefilter_enabled: true,
            phase_41c3_descriptor_construction_enabled: true,
            locates_prior_ed25519_instruction: true,
            accepts_verification_evidence: false,
            authorizes_execution: false,
            mutates_runtime_state: false,
        }
    }

    fn valid_loading_result(
        message_len: usize,
    ) -> Phase41D3_2_2CheckedPriorInstructionLoadingResult {
        loading_result(Vec::from([loaded_prior_instruction(
            1,
            ed25519_instruction_with_data(valid_ed25519_instruction_data(message_len)),
        )]))
    }

    fn prefilter_result(
        matched_instruction_index: Option<usize>,
    ) -> Phase41D3_2_3PrefilterDescriptorResult {
        Phase41D3_2_3PrefilterDescriptorResult {
            status:
                Phase41D3_2_3PrefilterDescriptorStatus::PriorEd25519InstructionStructurallyLocated,
            rejection_case: None,
            current_instruction_index: Some(3),
            loaded_prior_instruction_count: 1,
            inspected_loaded_entry_count: 1,
            discarded_non_candidate_count: 0,
            candidate_descriptor_count: 1,
            matched_instruction_index,
            phase_41c3_result: None,
            phase_41c3_candidate_descriptors: Vec::new(),
            consumes_phase_41d3_2_2_loaded_prior_instructions: true,
            processes_runtime_data_only_entries: true,
            iterates_loaded_entries_by_reference: true,
            prefilter_enabled: true,
            prefilter_by_ed25519_program_id_only: true,
            discards_non_candidates_immediately: true,
            stores_candidate_metadata_only: true,
            delegates_ordering_ambiguity_to_phase_41c3: true,
            explicit_same_index_reject_boundary: true,
            explicit_later_index_reject_boundary: true,
            phase_41c3_descriptor_construction_enabled: true,
            locates_prior_ed25519_instruction: true,
            ed25519_signature_verification_performed: false,
            cryptographic_signature_proof_accepted: false,
            accepts_verification_evidence: false,
            authorizes_execution: false,
            mutates_runtime_state: false,
        }
    }

    #[test]
    fn establishes_native_ed25519_verification_without_accepting_proof_or_authorization() {
        let loading_result = valid_loading_result(8);
        let parsing_result =
            parse_located_ed25519_instruction_bytes(&loading_result, &prefilter_result(Some(1)));
        let extraction_result =
            extract_checked_ed25519_byte_slices(&loading_result, &parsing_result);

        let result = establish_native_ed25519_signature_verification(
            &loading_result,
            &parsing_result,
            &extraction_result,
        );

        assert_eq!(
            result.status,
            Phase41F_2Ed25519SignatureVerificationStatus::NativeEd25519VerificationEstablished
        );
        assert_eq!(
            result.verification_model,
            Some(Phase41F_2Ed25519VerificationModel::NativeEd25519Instruction)
        );
        assert!(result.establishes_native_ed25519_verification);
        assert!(result.ed25519_signature_verification_performed);
        assert!(result.safety_flags_are_cumulative_pipeline_capabilities);
        assert!(!result.safety_flags_are_local_module_capabilities);
        assert!(result.message_payload_correctness_deferred_to_later_gate);
        assert!(!result.message_payload_correctness_checked);
        assert!(!result.performs_local_cryptographic_verification);
        assert!(!result.cryptographic_signature_proof_accepted);
        assert!(!result.accepts_verification_evidence);
        assert!(!result.guardian_validity_accepted);
        assert!(!result.quorum_counting_enabled);
        assert!(!result.authorizes_execution);
        assert!(!result.replay_write_enabled);
        assert!(!result.mutates_runtime_state);
        assert!(!result.cpi_enabled);
        assert!(!result.spl_token_mint_to_enabled);
        assert!(!result.live_route_enabled);
    }

    #[test]
    fn rejects_when_checked_slices_were_not_extracted() {
        let loading_result = valid_loading_result(8);
        let parsing_result =
            parse_located_ed25519_instruction_bytes(&loading_result, &prefilter_result(Some(1)));
        let mut extraction_result =
            extract_checked_ed25519_byte_slices(&loading_result, &parsing_result);
        extraction_result.status = Phase41F_1CheckedByteExtractionStatus::Ed25519BytesNotParsed;
        extraction_result.extracted_slices = None;

        let result = establish_native_ed25519_signature_verification(
            &loading_result,
            &parsing_result,
            &extraction_result,
        );

        assert_eq!(
            result.status,
            Phase41F_2Ed25519SignatureVerificationStatus::Ed25519ByteSlicesNotExtracted
        );
        assert!(!result.establishes_native_ed25519_verification);
        assert!(!result.ed25519_signature_verification_performed);
        assert!(!result.accepts_verification_evidence);
    }

    #[test]
    fn rejects_matched_instruction_index_mismatch() {
        let loading_result = valid_loading_result(8);
        let parsing_result =
            parse_located_ed25519_instruction_bytes(&loading_result, &prefilter_result(Some(1)));
        let mut extraction_result =
            extract_checked_ed25519_byte_slices(&loading_result, &parsing_result);
        extraction_result.matched_instruction_index = Some(2);

        let result = establish_native_ed25519_signature_verification(
            &loading_result,
            &parsing_result,
            &extraction_result,
        );

        assert_eq!(
            result.status,
            Phase41F_2Ed25519SignatureVerificationStatus::MatchedInstructionIndexMismatch
        );
        assert!(!result.establishes_native_ed25519_verification);
    }

    #[test]
    fn rejects_missing_parsed_offsets() {
        let loading_result = valid_loading_result(8);
        let mut parsing_result =
            parse_located_ed25519_instruction_bytes(&loading_result, &prefilter_result(Some(1)));
        let extraction_result =
            extract_checked_ed25519_byte_slices(&loading_result, &parsing_result);
        parsing_result.parsed_offsets = None;

        let result = establish_native_ed25519_signature_verification(
            &loading_result,
            &parsing_result,
            &extraction_result,
        );

        assert_eq!(
            result.status,
            Phase41F_2Ed25519SignatureVerificationStatus::ParsedOffsetsUnavailable
        );
        assert!(!result.establishes_native_ed25519_verification);
    }

    #[test]
    fn rejects_when_matched_instruction_is_unavailable() {
        let valid_loading_state = valid_loading_result(8);
        let parsing_result = parse_located_ed25519_instruction_bytes(
            &valid_loading_state,
            &prefilter_result(Some(1)),
        );
        let extraction_result =
            extract_checked_ed25519_byte_slices(&valid_loading_state, &parsing_result);
        let empty_loading_result = loading_result(Vec::new());

        let result = establish_native_ed25519_signature_verification(
            &empty_loading_result,
            &parsing_result,
            &extraction_result,
        );

        assert_eq!(
            result.status,
            Phase41F_2Ed25519SignatureVerificationStatus::MatchedInstructionUnavailable
        );
        assert!(!result.establishes_native_ed25519_verification);
    }

    #[test]
    fn rejects_wrong_program_id_as_defense_in_depth() {
        let valid_loading = valid_loading_result(8);
        let parsing_result =
            parse_located_ed25519_instruction_bytes(&valid_loading, &prefilter_result(Some(1)));
        let extraction_result =
            extract_checked_ed25519_byte_slices(&valid_loading, &parsing_result);
        let wrong_program_loading = loading_result(Vec::from([loaded_prior_instruction(
            1,
            instruction_with_program_id(
                Pubkey::new_from_array([9; 32]),
                valid_ed25519_instruction_data(8),
            ),
        )]));

        let result = establish_native_ed25519_signature_verification(
            &wrong_program_loading,
            &parsing_result,
            &extraction_result,
        );

        assert_eq!(
            result.status,
            Phase41F_2Ed25519SignatureVerificationStatus::WrongEd25519ProgramId
        );
        assert!(!result.establishes_native_ed25519_verification);
    }

    #[test]
    fn rejects_non_runtime_data_loaded_entry() {
        let valid_loading = valid_loading_result(8);
        let parsing_result =
            parse_located_ed25519_instruction_bytes(&valid_loading, &prefilter_result(Some(1)));
        let extraction_result =
            extract_checked_ed25519_byte_slices(&valid_loading, &parsing_result);
        let mut non_runtime_loading = valid_loading_result(8);
        if let Some(loaded_entry) = non_runtime_loading.loaded_prior_instructions.first_mut() {
            loaded_entry.loaded_instruction_is_runtime_data_only = false;
        }

        let result = establish_native_ed25519_signature_verification(
            &non_runtime_loading,
            &parsing_result,
            &extraction_result,
        );

        assert_eq!(
            result.status,
            Phase41F_2Ed25519SignatureVerificationStatus::NonRuntimeDataInstructionEntry
        );
        assert!(!result.establishes_native_ed25519_verification);
    }

    #[test]
    fn rejects_instruction_data_length_mismatch() {
        let valid_loading = valid_loading_result(8);
        let parsing_result =
            parse_located_ed25519_instruction_bytes(&valid_loading, &prefilter_result(Some(1)));
        let extraction_result =
            extract_checked_ed25519_byte_slices(&valid_loading, &parsing_result);
        let mut mismatched_loading = valid_loading_result(8);
        if let Some(loaded_entry) = mismatched_loading.loaded_prior_instructions.first_mut() {
            loaded_entry.instruction.data.push(9);
        }

        let result = establish_native_ed25519_signature_verification(
            &mismatched_loading,
            &parsing_result,
            &extraction_result,
        );

        assert_eq!(
            result.status,
            Phase41F_2Ed25519SignatureVerificationStatus::InstructionDataLengthMismatch
        );
        assert!(!result.establishes_native_ed25519_verification);
    }

    #[test]
    fn rejects_when_self_reference_binding_is_unavailable() {
        let loading_result = valid_loading_result(8);
        let mut parsing_result =
            parse_located_ed25519_instruction_bytes(&loading_result, &prefilter_result(Some(1)));
        let extraction_result =
            extract_checked_ed25519_byte_slices(&loading_result, &parsing_result);

        if let Some(parsed_offsets) = parsing_result.parsed_offsets.as_mut() {
            parsed_offsets
                .instruction_index_references
                .message_instruction_index = 0;
        }

        let result = establish_native_ed25519_signature_verification(
            &loading_result,
            &parsing_result,
            &extraction_result,
        );

        assert_eq!(
            result.status,
            Phase41F_2Ed25519SignatureVerificationStatus::SelfReferenceBindingUnavailable
        );
        assert!(!result.establishes_native_ed25519_verification);
    }

    #[test]
    fn rejects_when_extracted_ranges_do_not_match_parsed_ranges() {
        let loading_result = valid_loading_result(8);
        let mut parsing_result =
            parse_located_ed25519_instruction_bytes(&loading_result, &prefilter_result(Some(1)));
        let extraction_result =
            extract_checked_ed25519_byte_slices(&loading_result, &parsing_result);

        if let Some(parsed_offsets) = parsing_result.parsed_offsets.as_mut() {
            parsed_offsets.message_range.len = 7;
        }

        let result = establish_native_ed25519_signature_verification(
            &loading_result,
            &parsing_result,
            &extraction_result,
        );

        assert_eq!(
            result.status,
            Phase41F_2Ed25519SignatureVerificationStatus::ExtractedRangesMismatchParsedRanges
        );
        assert!(!result.establishes_native_ed25519_verification);
    }

    #[test]
    fn report_documents_model_a_without_downstream_trust_drift() {
        let report = phase_41f_2_ed25519_signature_verification_boundary_report();

        assert_eq!(
            report.phase,
            PHASE_41F_2_ED25519_SIGNATURE_VERIFICATION_BOUNDARY_PHASE
        );
        assert_eq!(
            report.implemented_model,
            Phase41F_2Ed25519VerificationModel::NativeEd25519Instruction
        );
        assert!(report.safety_flags_are_cumulative_pipeline_capabilities);
        assert!(!report.safety_flags_are_local_module_capabilities);
        assert!(report.rechecks_ed25519_program_id);
        assert!(report.preserves_self_reference_binding);
        assert!(report.model_a_native_abort_before_current_soundness_documented);
        assert!(report.establishes_native_ed25519_verification);
        assert!(report.ed25519_signature_verification_performed);
        assert!(!report.performs_local_cryptographic_verification);
        assert!(!report.message_payload_correctness_checked);
        assert!(report.message_payload_correctness_deferred_to_later_gate);
        assert!(!report.cryptographic_signature_proof_accepted);
        assert!(!report.accepts_verification_evidence);
        assert!(!report.guardian_validity_accepted);
        assert!(!report.quorum_counting_enabled);
        assert!(!report.authorizes_execution);
        assert!(!report.replay_write_enabled);
        assert!(!report.mutates_runtime_state);
        assert!(!report.cpi_enabled);
        assert!(!report.spl_token_mint_to_enabled);
        assert!(!report.live_route_enabled);
        assert!(report.safety_flags.ed25519_signature_verification_performed);
        assert!(!report.safety_flags.cryptographic_signature_proof_accepted);
        assert!(!report.safety_flags.verification_evidence_accepted);
        assert!(!report.safety_flags.quorum_counting_enabled);
        assert!(!report.safety_flags.authorization_enabled);
        assert!(!report.safety_flags.replay_write_enabled);
        assert!(!report.safety_flags.account_mutation_enabled);
        assert!(!report.safety_flags.cpi_enabled);
        assert!(!report.safety_flags.invoke_signed_enabled);
        assert!(!report.safety_flags.spl_token_mint_to_enabled);
        assert!(!report.safety_flags.process_instruction_handler_added);
        assert!(!report.safety_flags.live_route_enabled);
    }
}
