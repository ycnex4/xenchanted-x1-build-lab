use core::convert::TryInto;

use super::checked_prior_instruction_loading_runtime_boundary::{
    Phase41D3_2_2CheckedPriorInstructionLoadingResult, Phase41D3_2_2LoadedPriorInstruction,
};
use super::ed25519_instruction_byte_parsing_boundary::{
    Phase41E_1ByteRange, Phase41E_1Ed25519ByteParsingResult, Phase41E_1Ed25519ByteParsingStatus,
    ED25519_PUBLIC_KEY_LEN, ED25519_SIGNATURE_LEN,
};
use super::instructions_sysvar_access_contract_model::{
    Phase41BRejectionCase, Phase41BSafetyFlags,
};

pub const PHASE_41F_1_CHECKED_BYTE_EXTRACTION_BOUNDARY_VERSION: &str = "0.1.0";
pub const PHASE_41F_1_CHECKED_BYTE_EXTRACTION_BOUNDARY_PHASE: &str = "41F.1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase41F_1CheckedByteExtractionStatus {
    Ed25519BytesNotParsed,
    MatchedInstructionIndexUnavailable,
    MatchedInstructionUnavailable,
    NonRuntimeDataInstructionEntry,
    ParsedOffsetsUnavailable,
    InstructionDataLengthMismatch,
    CheckedSignatureSliceUnavailable,
    CheckedPublicKeySliceUnavailable,
    CheckedMessageSliceUnavailable,
    CheckedEd25519ByteSlicesExtracted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phase41F_1ExtractedEd25519ByteSlices<'a> {
    pub signature_bytes: &'a [u8; ED25519_SIGNATURE_LEN],
    pub public_key_bytes: &'a [u8; ED25519_PUBLIC_KEY_LEN],
    pub message_bytes: &'a [u8],
    pub signature_range: Phase41E_1ByteRange,
    pub public_key_range: Phase41E_1ByteRange,
    pub message_range: Phase41E_1ByteRange,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Phase41F_1CheckedByteExtractionResult<'a> {
    pub status: Phase41F_1CheckedByteExtractionStatus,
    pub rejection_case: Option<Phase41BRejectionCase>,
    pub matched_instruction_index: Option<usize>,
    pub instruction_data_len: usize,
    pub extracted_slices: Option<Phase41F_1ExtractedEd25519ByteSlices<'a>>,
    pub consumes_phase_41e_1_parsed_offsets: bool,
    pub consumes_phase_41d3_2_2_loaded_prior_instructions: bool,
    pub checks_matched_instruction_index: bool,
    pub checks_loaded_entry_runtime_data_only: bool,
    pub checks_instruction_data_length_matches_parse_result: bool,
    pub extracts_signature_bytes_as_fixed_array_ref: bool,
    pub extracts_public_key_bytes_as_fixed_array_ref: bool,
    pub extracts_message_bytes_as_borrowed_slice: bool,
    pub uses_checked_slice_access: bool,
    pub uses_unchecked_indexing: bool,
    pub uses_unchecked_slicing: bool,
    pub copies_attacker_sized_message_data: bool,
    pub performs_local_cryptographic_verification: bool,
    pub establishes_native_ed25519_verification: bool,
    pub ed25519_signature_verification_performed: bool,
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
pub struct Phase41F_1CheckedByteExtractionBoundaryReport {
    pub phase: &'static str,
    pub version: &'static str,
    pub consumes_phase_41e_1_parsed_offsets: bool,
    pub consumes_phase_41d3_2_2_loaded_prior_instructions: bool,
    pub checks_matched_instruction_index: bool,
    pub checks_loaded_entry_runtime_data_only: bool,
    pub checks_instruction_data_length_matches_parse_result: bool,
    pub extracts_signature_bytes_as_fixed_array_ref: bool,
    pub extracts_public_key_bytes_as_fixed_array_ref: bool,
    pub extracts_message_bytes_as_borrowed_slice: bool,
    pub uses_checked_slice_access: bool,
    pub uses_unchecked_indexing: bool,
    pub uses_unchecked_slicing: bool,
    pub copies_attacker_sized_message_data: bool,
    pub performs_local_cryptographic_verification: bool,
    pub establishes_native_ed25519_verification: bool,
    pub ed25519_signature_verification_performed: bool,
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

pub const PHASE_41F_1_SAFETY_FLAGS: Phase41BSafetyFlags = Phase41BSafetyFlags {
    raw_instructions_sysvar_parser_implemented: false,
    account_info_parser_implemented: false,
    load_instruction_called: false,
    load_instruction_enabled: false,
    concrete_runtime_api_selected: false,
    current_instruction_identity_derived_from_runtime: false,
    ed25519_signature_verification_performed: false,
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

pub const PHASE_41F_1_CHECKED_BYTE_EXTRACTION_BOUNDARY_REPORT:
    Phase41F_1CheckedByteExtractionBoundaryReport = Phase41F_1CheckedByteExtractionBoundaryReport {
    phase: PHASE_41F_1_CHECKED_BYTE_EXTRACTION_BOUNDARY_PHASE,
    version: PHASE_41F_1_CHECKED_BYTE_EXTRACTION_BOUNDARY_VERSION,
    consumes_phase_41e_1_parsed_offsets: true,
    consumes_phase_41d3_2_2_loaded_prior_instructions: true,
    checks_matched_instruction_index: true,
    checks_loaded_entry_runtime_data_only: true,
    checks_instruction_data_length_matches_parse_result: true,
    extracts_signature_bytes_as_fixed_array_ref: true,
    extracts_public_key_bytes_as_fixed_array_ref: true,
    extracts_message_bytes_as_borrowed_slice: true,
    uses_checked_slice_access: true,
    uses_unchecked_indexing: false,
    uses_unchecked_slicing: false,
    copies_attacker_sized_message_data: false,
    performs_local_cryptographic_verification: false,
    establishes_native_ed25519_verification: false,
    ed25519_signature_verification_performed: false,
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
    safety_flags: PHASE_41F_1_SAFETY_FLAGS,
};

pub fn extract_checked_ed25519_byte_slices<'a>(
    loading_result: &'a Phase41D3_2_2CheckedPriorInstructionLoadingResult,
    parsing_result: &Phase41E_1Ed25519ByteParsingResult,
) -> Phase41F_1CheckedByteExtractionResult<'a> {
    if parsing_result.status != Phase41E_1Ed25519ByteParsingStatus::Ed25519InstructionBytesParsed
        || !parsing_result.parses_ed25519_instruction_bytes
    {
        return fail(
            Phase41F_1CheckedByteExtractionStatus::Ed25519BytesNotParsed,
            parsing_result
                .rejection_case
                .or(Some(Phase41BRejectionCase::MalformedEd25519InstructionData)),
            parsing_result.matched_instruction_index,
            parsing_result.instruction_data_len,
        );
    }

    let Some(matched_instruction_index) = parsing_result.matched_instruction_index else {
        return fail(
            Phase41F_1CheckedByteExtractionStatus::MatchedInstructionIndexUnavailable,
            Some(Phase41BRejectionCase::Ed25519InstructionNotFound),
            None,
            parsing_result.instruction_data_len,
        );
    };

    let Some(parsed_offsets) = parsing_result.parsed_offsets else {
        return fail(
            Phase41F_1CheckedByteExtractionStatus::ParsedOffsetsUnavailable,
            Some(Phase41BRejectionCase::MalformedEd25519InstructionData),
            Some(matched_instruction_index),
            parsing_result.instruction_data_len,
        );
    };

    let Some(loaded_entry) =
        find_loaded_entry_by_instruction_index(loading_result, matched_instruction_index)
    else {
        return fail(
            Phase41F_1CheckedByteExtractionStatus::MatchedInstructionUnavailable,
            Some(Phase41BRejectionCase::Ed25519InstructionNotFound),
            Some(matched_instruction_index),
            parsing_result.instruction_data_len,
        );
    };

    let instruction_data_len = loaded_entry.instruction.data.len();

    if !loaded_entry.loaded_instruction_is_runtime_data_only
        || loaded_entry.is_evidence
        || loaded_entry.authorizes_execution
    {
        return fail(
            Phase41F_1CheckedByteExtractionStatus::NonRuntimeDataInstructionEntry,
            Some(Phase41BRejectionCase::UnreadableInstructionsSysvar),
            Some(matched_instruction_index),
            instruction_data_len,
        );
    }

    if instruction_data_len != parsing_result.instruction_data_len {
        return fail(
            Phase41F_1CheckedByteExtractionStatus::InstructionDataLengthMismatch,
            Some(Phase41BRejectionCase::MalformedEd25519InstructionData),
            Some(matched_instruction_index),
            instruction_data_len,
        );
    }

    let instruction_data = loaded_entry.instruction.data.as_slice();

    let Some(signature_bytes) = checked_fixed_array_slice::<ED25519_SIGNATURE_LEN>(
        instruction_data,
        parsed_offsets.signature_range,
    ) else {
        return fail(
            Phase41F_1CheckedByteExtractionStatus::CheckedSignatureSliceUnavailable,
            Some(Phase41BRejectionCase::UnsupportedOffsetLayout),
            Some(matched_instruction_index),
            instruction_data_len,
        );
    };

    let Some(public_key_bytes) = checked_fixed_array_slice::<ED25519_PUBLIC_KEY_LEN>(
        instruction_data,
        parsed_offsets.public_key_range,
    ) else {
        return fail(
            Phase41F_1CheckedByteExtractionStatus::CheckedPublicKeySliceUnavailable,
            Some(Phase41BRejectionCase::UnsupportedOffsetLayout),
            Some(matched_instruction_index),
            instruction_data_len,
        );
    };

    let Some(message_bytes) = checked_slice(instruction_data, parsed_offsets.message_range) else {
        return fail(
            Phase41F_1CheckedByteExtractionStatus::CheckedMessageSliceUnavailable,
            Some(Phase41BRejectionCase::UnsupportedOffsetLayout),
            Some(matched_instruction_index),
            instruction_data_len,
        );
    };

    if message_bytes.is_empty() {
        return fail(
            Phase41F_1CheckedByteExtractionStatus::CheckedMessageSliceUnavailable,
            Some(Phase41BRejectionCase::MalformedEd25519InstructionData),
            Some(matched_instruction_index),
            instruction_data_len,
        );
    }

    extracted(
        Some(matched_instruction_index),
        instruction_data_len,
        Phase41F_1ExtractedEd25519ByteSlices {
            signature_bytes,
            public_key_bytes,
            message_bytes,
            signature_range: parsed_offsets.signature_range,
            public_key_range: parsed_offsets.public_key_range,
            message_range: parsed_offsets.message_range,
        },
    )
}

pub fn phase_41f_1_checked_byte_extraction_boundary_report(
) -> Phase41F_1CheckedByteExtractionBoundaryReport {
    PHASE_41F_1_CHECKED_BYTE_EXTRACTION_BOUNDARY_REPORT
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

fn checked_fixed_array_slice<const LEN: usize>(
    data: &[u8],
    range: Phase41E_1ByteRange,
) -> Option<&[u8; LEN]> {
    if range.len != LEN {
        return None;
    }

    checked_slice(data, range)?.try_into().ok()
}

fn checked_slice(data: &[u8], range: Phase41E_1ByteRange) -> Option<&[u8]> {
    let end = range.offset.checked_add(range.len)?;

    data.get(range.offset..end)
}

fn extracted<'a>(
    matched_instruction_index: Option<usize>,
    instruction_data_len: usize,
    extracted_slices: Phase41F_1ExtractedEd25519ByteSlices<'a>,
) -> Phase41F_1CheckedByteExtractionResult<'a> {
    Phase41F_1CheckedByteExtractionResult {
        status: Phase41F_1CheckedByteExtractionStatus::CheckedEd25519ByteSlicesExtracted,
        rejection_case: None,
        matched_instruction_index,
        instruction_data_len,
        extracted_slices: Some(extracted_slices),
        consumes_phase_41e_1_parsed_offsets: true,
        consumes_phase_41d3_2_2_loaded_prior_instructions: true,
        checks_matched_instruction_index: true,
        checks_loaded_entry_runtime_data_only: true,
        checks_instruction_data_length_matches_parse_result: true,
        extracts_signature_bytes_as_fixed_array_ref: true,
        extracts_public_key_bytes_as_fixed_array_ref: true,
        extracts_message_bytes_as_borrowed_slice: true,
        uses_checked_slice_access: true,
        uses_unchecked_indexing: false,
        uses_unchecked_slicing: false,
        copies_attacker_sized_message_data: false,
        performs_local_cryptographic_verification: false,
        establishes_native_ed25519_verification: false,
        ed25519_signature_verification_performed: false,
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

fn fail<'a>(
    status: Phase41F_1CheckedByteExtractionStatus,
    rejection_case: Option<Phase41BRejectionCase>,
    matched_instruction_index: Option<usize>,
    instruction_data_len: usize,
) -> Phase41F_1CheckedByteExtractionResult<'a> {
    Phase41F_1CheckedByteExtractionResult {
        status,
        rejection_case,
        matched_instruction_index,
        instruction_data_len,
        extracted_slices: None,
        consumes_phase_41e_1_parsed_offsets: true,
        consumes_phase_41d3_2_2_loaded_prior_instructions: true,
        checks_matched_instruction_index: true,
        checks_loaded_entry_runtime_data_only: true,
        checks_instruction_data_length_matches_parse_result: true,
        extracts_signature_bytes_as_fixed_array_ref: true,
        extracts_public_key_bytes_as_fixed_array_ref: true,
        extracts_message_bytes_as_borrowed_slice: true,
        uses_checked_slice_access: true,
        uses_unchecked_indexing: false,
        uses_unchecked_slicing: false,
        copies_attacker_sized_message_data: false,
        performs_local_cryptographic_verification: false,
        establishes_native_ed25519_verification: false,
        ed25519_signature_verification_performed: false,
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
    use solana_program::{ed25519_program, instruction::Instruction};

    use super::super::checked_prior_instruction_loading_runtime_boundary::{
        Phase41D3_2_2CheckedPriorInstructionLoadingResult,
        Phase41D3_2_2CheckedPriorInstructionLoadingStatus, Phase41D3_2_2LoadedPriorInstruction,
    };
    use super::super::ed25519_instruction_byte_parsing_boundary::{
        parse_located_ed25519_instruction_bytes, Phase41E_1Ed25519ByteParsingStatus,
        ED25519_CURRENT_INSTRUCTION_INDEX, ED25519_SINGLE_SIGNATURE_OFFSET_TABLE_LEN,
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

    fn ed25519_instruction_with_data(data: Vec<u8>) -> Instruction {
        Instruction {
            program_id: ed25519_program::id(),
            accounts: Vec::new(),
            data,
        }
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

    fn valid_loading_and_parsing_result(
        message_len: usize,
    ) -> (
        Phase41D3_2_2CheckedPriorInstructionLoadingResult,
        Phase41E_1Ed25519ByteParsingResult,
    ) {
        let loading_result = loading_result(Vec::from([loaded_prior_instruction(
            1,
            ed25519_instruction_with_data(valid_ed25519_instruction_data(message_len)),
        )]));
        let parsing_result =
            parse_located_ed25519_instruction_bytes(&loading_result, &prefilter_result(Some(1)));

        (loading_result, parsing_result)
    }

    #[test]
    fn extracts_signature_pubkey_and_borrowed_message_with_checked_slices() {
        let (loading_result, parsing_result) = valid_loading_and_parsing_result(8);

        let result = extract_checked_ed25519_byte_slices(&loading_result, &parsing_result);

        assert_eq!(
            result.status,
            Phase41F_1CheckedByteExtractionStatus::CheckedEd25519ByteSlicesExtracted
        );
        assert!(result.extracted_slices.is_some());
        assert!(result.uses_checked_slice_access);
        assert!(!result.uses_unchecked_indexing);
        assert!(!result.uses_unchecked_slicing);
        assert!(!result.copies_attacker_sized_message_data);
        assert!(!result.ed25519_signature_verification_performed);
        assert!(!result.accepts_verification_evidence);
        assert!(!result.authorizes_execution);

        if let Some(extracted) = result.extracted_slices {
            assert_eq!(extracted.signature_bytes.len(), ED25519_SIGNATURE_LEN);
            assert_eq!(extracted.public_key_bytes.len(), ED25519_PUBLIC_KEY_LEN);
            assert_eq!(extracted.message_bytes.len(), 8);
            assert_eq!(extracted.signature_bytes.first().copied(), Some(1));
            assert_eq!(extracted.public_key_bytes.first().copied(), Some(2));
            assert_eq!(extracted.message_bytes.first().copied(), Some(3));
        }
    }

    #[test]
    fn rejects_when_ed25519_bytes_were_not_parsed() {
        let (loading_result, mut parsing_result) = valid_loading_and_parsing_result(8);
        parsing_result.status = Phase41E_1Ed25519ByteParsingStatus::EmptyInstructionData;
        parsing_result.parses_ed25519_instruction_bytes = false;
        parsing_result.parsed_offsets = None;

        let result = extract_checked_ed25519_byte_slices(&loading_result, &parsing_result);

        assert_eq!(
            result.status,
            Phase41F_1CheckedByteExtractionStatus::Ed25519BytesNotParsed
        );
        assert!(result.extracted_slices.is_none());
        assert!(!result.ed25519_signature_verification_performed);
        assert!(!result.accepts_verification_evidence);
    }

    #[test]
    fn rejects_missing_matched_instruction_index() {
        let (loading_result, mut parsing_result) = valid_loading_and_parsing_result(8);
        parsing_result.matched_instruction_index = None;

        let result = extract_checked_ed25519_byte_slices(&loading_result, &parsing_result);

        assert_eq!(
            result.status,
            Phase41F_1CheckedByteExtractionStatus::MatchedInstructionIndexUnavailable
        );
        assert!(result.extracted_slices.is_none());
    }

    #[test]
    fn rejects_missing_parsed_offsets() {
        let (loading_result, mut parsing_result) = valid_loading_and_parsing_result(8);
        parsing_result.parsed_offsets = None;

        let result = extract_checked_ed25519_byte_slices(&loading_result, &parsing_result);

        assert_eq!(
            result.status,
            Phase41F_1CheckedByteExtractionStatus::ParsedOffsetsUnavailable
        );
        assert!(result.extracted_slices.is_none());
    }

    #[test]
    fn rejects_when_matched_instruction_is_unavailable() {
        let (_valid_loading_result, parsing_result) = valid_loading_and_parsing_result(8);
        let empty_loading_result = loading_result(Vec::new());

        let result = extract_checked_ed25519_byte_slices(&empty_loading_result, &parsing_result);

        assert_eq!(
            result.status,
            Phase41F_1CheckedByteExtractionStatus::MatchedInstructionUnavailable
        );
        assert!(result.extracted_slices.is_none());
    }

    #[test]
    fn rejects_non_runtime_data_loaded_entry() {
        let (mut loading_result, parsing_result) = valid_loading_and_parsing_result(8);

        if let Some(loaded_entry) = loading_result.loaded_prior_instructions.first_mut() {
            loaded_entry.loaded_instruction_is_runtime_data_only = false;
        }

        let result = extract_checked_ed25519_byte_slices(&loading_result, &parsing_result);

        assert_eq!(
            result.status,
            Phase41F_1CheckedByteExtractionStatus::NonRuntimeDataInstructionEntry
        );
        assert!(result.extracted_slices.is_none());
    }

    #[test]
    fn rejects_instruction_data_length_mismatch() {
        let (mut loading_result, parsing_result) = valid_loading_and_parsing_result(8);

        if let Some(loaded_entry) = loading_result.loaded_prior_instructions.first_mut() {
            loaded_entry.instruction.data.push(9);
        }

        let result = extract_checked_ed25519_byte_slices(&loading_result, &parsing_result);

        assert_eq!(
            result.status,
            Phase41F_1CheckedByteExtractionStatus::InstructionDataLengthMismatch
        );
        assert!(result.extracted_slices.is_none());
    }

    #[test]
    fn rejects_signature_slice_unavailable() {
        let (loading_result, mut parsing_result) = valid_loading_and_parsing_result(8);
        assert!(parsing_result.parsed_offsets.is_some());

        if let Some(parsed_offsets) = parsing_result.parsed_offsets.as_mut() {
            parsed_offsets.signature_range.offset = usize::MAX;
        }

        let result = extract_checked_ed25519_byte_slices(&loading_result, &parsing_result);

        assert_eq!(
            result.status,
            Phase41F_1CheckedByteExtractionStatus::CheckedSignatureSliceUnavailable
        );
        assert!(result.extracted_slices.is_none());
    }

    #[test]
    fn rejects_public_key_slice_unavailable() {
        let (loading_result, mut parsing_result) = valid_loading_and_parsing_result(8);
        assert!(parsing_result.parsed_offsets.is_some());

        if let Some(parsed_offsets) = parsing_result.parsed_offsets.as_mut() {
            parsed_offsets.public_key_range.offset = usize::MAX;
        }

        let result = extract_checked_ed25519_byte_slices(&loading_result, &parsing_result);

        assert_eq!(
            result.status,
            Phase41F_1CheckedByteExtractionStatus::CheckedPublicKeySliceUnavailable
        );
        assert!(result.extracted_slices.is_none());
    }

    #[test]
    fn rejects_message_slice_unavailable() {
        let (loading_result, mut parsing_result) = valid_loading_and_parsing_result(8);
        assert!(parsing_result.parsed_offsets.is_some());

        if let Some(parsed_offsets) = parsing_result.parsed_offsets.as_mut() {
            parsed_offsets.message_range.offset = usize::MAX;
        }

        let result = extract_checked_ed25519_byte_slices(&loading_result, &parsing_result);

        assert_eq!(
            result.status,
            Phase41F_1CheckedByteExtractionStatus::CheckedMessageSliceUnavailable
        );
        assert!(result.extracted_slices.is_none());
    }

    #[test]
    fn report_preserves_extraction_only_non_authorizing_boundary() {
        let report = phase_41f_1_checked_byte_extraction_boundary_report();

        assert_eq!(
            report.phase,
            PHASE_41F_1_CHECKED_BYTE_EXTRACTION_BOUNDARY_PHASE
        );
        assert!(report.consumes_phase_41e_1_parsed_offsets);
        assert!(report.consumes_phase_41d3_2_2_loaded_prior_instructions);
        assert!(report.uses_checked_slice_access);
        assert!(report.extracts_signature_bytes_as_fixed_array_ref);
        assert!(report.extracts_public_key_bytes_as_fixed_array_ref);
        assert!(report.extracts_message_bytes_as_borrowed_slice);
        assert!(!report.uses_unchecked_indexing);
        assert!(!report.uses_unchecked_slicing);
        assert!(!report.copies_attacker_sized_message_data);
        assert!(!report.performs_local_cryptographic_verification);
        assert!(!report.establishes_native_ed25519_verification);
        assert!(!report.ed25519_signature_verification_performed);
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
        assert!(!report.safety_flags.ed25519_signature_verification_performed);
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
