use solana_program::ed25519_program;

use super::checked_prior_instruction_loading_runtime_boundary::{
    Phase41D3_2_2CheckedPriorInstructionLoadingResult, Phase41D3_2_2LoadedPriorInstruction,
};
use super::instructions_sysvar_access_contract_model::{
    Phase41BRejectionCase, Phase41BSafetyFlags,
};
use super::prefilter_phase_41c3_candidate_descriptor_runtime_boundary::{
    Phase41D3_2_3PrefilterDescriptorResult, Phase41D3_2_3PrefilterDescriptorStatus,
};

pub const PHASE_41E_1_ED25519_BYTE_PARSING_BOUNDARY_VERSION: &str = "0.1.0";
pub const PHASE_41E_1_ED25519_BYTE_PARSING_BOUNDARY_PHASE: &str = "41E.1";

pub const ED25519_SIGNATURE_LEN: usize = 64;
pub const ED25519_PUBLIC_KEY_LEN: usize = 32;
pub const ED25519_INSTRUCTION_HEADER_LEN: usize = 2;
pub const ED25519_SIGNATURE_OFFSETS_LEN: usize = 14;
pub const ED25519_SINGLE_SIGNATURE_OFFSET_TABLE_LEN: usize =
    ED25519_INSTRUCTION_HEADER_LEN + ED25519_SIGNATURE_OFFSETS_LEN;
pub const ED25519_CURRENT_INSTRUCTION_INDEX: u16 = u16::MAX;
pub const SUPPORTED_ED25519_SIGNATURE_COUNT: u8 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase41E_1Ed25519ByteParsingStatus {
    PriorEd25519InstructionNotStructurallyLocated,
    MatchedInstructionIndexUnavailable,
    MatchedInstructionUnavailable,
    WrongEd25519ProgramId,
    NonRuntimeDataInstructionEntry,
    EmptyInstructionData,
    MalformedEd25519InstructionHeader,
    UnsupportedSignatureCount,
    OutOfBoundsSignatureOffset,
    OutOfBoundsPublicKeyOffset,
    MissingMessageByteRange,
    OutOfBoundsMessageOffset,
    UnexpectedInstructionIndexReference,
    ParsedRangeAliasesOffsetTable,
    OverlappingParsedByteRanges,
    Ed25519InstructionBytesParsed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phase41E_1ByteRange {
    pub offset: usize,
    pub len: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phase41E_1Ed25519InstructionIndexReferences {
    pub signature_instruction_index: u16,
    pub public_key_instruction_index: u16,
    pub message_instruction_index: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phase41E_1ParsedEd25519Offsets {
    pub signature_offset: usize,
    pub public_key_offset: usize,
    pub message_offset: usize,
    pub message_len: usize,
    pub instruction_index_references: Phase41E_1Ed25519InstructionIndexReferences,
    pub signature_range: Phase41E_1ByteRange,
    pub public_key_range: Phase41E_1ByteRange,
    pub message_range: Phase41E_1ByteRange,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Phase41E_1Ed25519ByteParsingResult {
    pub status: Phase41E_1Ed25519ByteParsingStatus,
    pub rejection_case: Option<Phase41BRejectionCase>,
    pub matched_instruction_index: Option<usize>,
    pub instruction_data_len: usize,
    pub signature_count: Option<u8>,
    pub padding_byte: Option<u8>,
    pub parsed_offsets: Option<Phase41E_1ParsedEd25519Offsets>,
    pub parses_ed25519_instruction_bytes: bool,
    pub entry_gate_requires_located_status: bool,
    pub entry_gate_requires_matched_instruction_index: bool,
    pub locates_prior_ed25519_instruction_used_as_gate: bool,
    pub descriptor_booleans_trusted_as_evidence: bool,
    pub consumes_phase_41d3_2_2_loaded_prior_instructions: bool,
    pub consumes_phase_41d3_2_3_prefilter_result: bool,
    pub loads_referenced_instructions: bool,
    pub rejects_cross_instruction_references: bool,
    pub rejects_offset_table_aliasing: bool,
    pub stores_message_as_bounded_indices: bool,
    pub copies_attacker_sized_message_data: bool,
    pub rejects_overlapping_parsed_ranges: bool,
    pub uses_checked_offset_arithmetic: bool,
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
pub struct Phase41E_1Ed25519ByteParsingBoundaryReport {
    pub phase: &'static str,
    pub version: &'static str,
    pub parses_ed25519_instruction_bytes: bool,
    pub entry_gate_requires_located_status: bool,
    pub entry_gate_requires_matched_instruction_index: bool,
    pub locates_prior_ed25519_instruction_used_as_gate: bool,
    pub descriptor_booleans_trusted_as_evidence: bool,
    pub consumes_phase_41d3_2_2_loaded_prior_instructions: bool,
    pub consumes_phase_41d3_2_3_prefilter_result: bool,
    pub loads_referenced_instructions: bool,
    pub rejects_cross_instruction_references: bool,
    pub rejects_offset_table_aliasing: bool,
    pub stores_message_as_bounded_indices: bool,
    pub copies_attacker_sized_message_data: bool,
    pub rejects_overlapping_parsed_ranges: bool,
    pub uses_checked_offset_arithmetic: bool,
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

pub const PHASE_41E_1_SAFETY_FLAGS: Phase41BSafetyFlags = Phase41BSafetyFlags {
    raw_instructions_sysvar_parser_implemented: false,
    account_info_parser_implemented: true,
    load_instruction_called: true,
    load_instruction_enabled: true,
    concrete_runtime_api_selected: true,
    current_instruction_identity_derived_from_runtime: true,
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

pub const PHASE_41E_1_ED25519_BYTE_PARSING_BOUNDARY_REPORT:
    Phase41E_1Ed25519ByteParsingBoundaryReport = Phase41E_1Ed25519ByteParsingBoundaryReport {
    phase: PHASE_41E_1_ED25519_BYTE_PARSING_BOUNDARY_PHASE,
    version: PHASE_41E_1_ED25519_BYTE_PARSING_BOUNDARY_VERSION,
    parses_ed25519_instruction_bytes: true,
    entry_gate_requires_located_status: true,
    entry_gate_requires_matched_instruction_index: true,
    locates_prior_ed25519_instruction_used_as_gate: false,
    descriptor_booleans_trusted_as_evidence: false,
    consumes_phase_41d3_2_2_loaded_prior_instructions: true,
    consumes_phase_41d3_2_3_prefilter_result: true,
    loads_referenced_instructions: false,
    rejects_cross_instruction_references: true,
    rejects_offset_table_aliasing: true,
    stores_message_as_bounded_indices: true,
    copies_attacker_sized_message_data: false,
    rejects_overlapping_parsed_ranges: true,
    uses_checked_offset_arithmetic: true,
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
    safety_flags: PHASE_41E_1_SAFETY_FLAGS,
};

pub fn parse_located_ed25519_instruction_bytes(
    loading_result: &Phase41D3_2_2CheckedPriorInstructionLoadingResult,
    prefilter_result: &Phase41D3_2_3PrefilterDescriptorResult,
) -> Phase41E_1Ed25519ByteParsingResult {
    if prefilter_result.status
        != Phase41D3_2_3PrefilterDescriptorStatus::PriorEd25519InstructionStructurallyLocated
    {
        return fail(
            Phase41E_1Ed25519ByteParsingStatus::PriorEd25519InstructionNotStructurallyLocated,
            prefilter_result
                .rejection_case
                .or(Some(Phase41BRejectionCase::Ed25519InstructionNotFound)),
            prefilter_result.matched_instruction_index,
            0,
        );
    }

    let Some(matched_instruction_index) = prefilter_result.matched_instruction_index else {
        return fail(
            Phase41E_1Ed25519ByteParsingStatus::MatchedInstructionIndexUnavailable,
            Some(Phase41BRejectionCase::Ed25519InstructionNotFound),
            None,
            0,
        );
    };

    let Some(loaded_entry) =
        find_loaded_entry_by_instruction_index(loading_result, matched_instruction_index)
    else {
        return fail(
            Phase41E_1Ed25519ByteParsingStatus::MatchedInstructionUnavailable,
            Some(Phase41BRejectionCase::Ed25519InstructionNotFound),
            Some(matched_instruction_index),
            0,
        );
    };

    parse_loaded_ed25519_instruction_entry(loaded_entry)
}

pub fn phase_41e_1_ed25519_byte_parsing_boundary_report(
) -> Phase41E_1Ed25519ByteParsingBoundaryReport {
    PHASE_41E_1_ED25519_BYTE_PARSING_BOUNDARY_REPORT
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

fn parse_loaded_ed25519_instruction_entry(
    loaded_entry: &Phase41D3_2_2LoadedPriorInstruction,
) -> Phase41E_1Ed25519ByteParsingResult {
    let matched_instruction_index = Some(loaded_entry.instruction_index);
    let instruction_data_len = loaded_entry.instruction.data.len();

    if loaded_entry.instruction.program_id != ed25519_program::id() {
        return fail(
            Phase41E_1Ed25519ByteParsingStatus::WrongEd25519ProgramId,
            Some(Phase41BRejectionCase::WrongEd25519ProgramId),
            matched_instruction_index,
            instruction_data_len,
        );
    }

    if !loaded_entry.loaded_instruction_is_runtime_data_only
        || loaded_entry.is_evidence
        || loaded_entry.authorizes_execution
    {
        return fail(
            Phase41E_1Ed25519ByteParsingStatus::NonRuntimeDataInstructionEntry,
            Some(Phase41BRejectionCase::UnreadableInstructionsSysvar),
            matched_instruction_index,
            instruction_data_len,
        );
    }

    parse_ed25519_instruction_data_bytes(
        &loaded_entry.instruction.data,
        loaded_entry.instruction_index,
    )
}

fn parse_ed25519_instruction_data_bytes(
    instruction_data: &[u8],
    matched_instruction_index: usize,
) -> Phase41E_1Ed25519ByteParsingResult {
    let instruction_data_len = instruction_data.len();

    if instruction_data.is_empty() {
        return fail(
            Phase41E_1Ed25519ByteParsingStatus::EmptyInstructionData,
            Some(Phase41BRejectionCase::MalformedEd25519InstructionData),
            Some(matched_instruction_index),
            instruction_data_len,
        );
    }

    let Some(signature_count) = read_u8(instruction_data, 0) else {
        return fail(
            Phase41E_1Ed25519ByteParsingStatus::MalformedEd25519InstructionHeader,
            Some(Phase41BRejectionCase::MalformedEd25519InstructionData),
            Some(matched_instruction_index),
            instruction_data_len,
        );
    };

    let Some(padding_byte) = read_u8(instruction_data, 1) else {
        return fail_with_header(
            Phase41E_1Ed25519ByteParsingStatus::MalformedEd25519InstructionHeader,
            Some(Phase41BRejectionCase::MalformedEd25519InstructionData),
            Some(matched_instruction_index),
            instruction_data_len,
            Some(signature_count),
            None,
        );
    };

    if padding_byte != 0 {
        return fail_with_header(
            Phase41E_1Ed25519ByteParsingStatus::MalformedEd25519InstructionHeader,
            Some(Phase41BRejectionCase::MalformedEd25519InstructionData),
            Some(matched_instruction_index),
            instruction_data_len,
            Some(signature_count),
            Some(padding_byte),
        );
    }

    if signature_count != SUPPORTED_ED25519_SIGNATURE_COUNT {
        return fail_with_header(
            Phase41E_1Ed25519ByteParsingStatus::UnsupportedSignatureCount,
            Some(Phase41BRejectionCase::UnsupportedOffsetLayout),
            Some(matched_instruction_index),
            instruction_data_len,
            Some(signature_count),
            Some(padding_byte),
        );
    }

    let Some(offset_table_end) =
        ED25519_INSTRUCTION_HEADER_LEN.checked_add(ED25519_SIGNATURE_OFFSETS_LEN)
    else {
        return fail_with_header(
            Phase41E_1Ed25519ByteParsingStatus::MalformedEd25519InstructionHeader,
            Some(Phase41BRejectionCase::MalformedEd25519InstructionData),
            Some(matched_instruction_index),
            instruction_data_len,
            Some(signature_count),
            Some(padding_byte),
        );
    };

    if instruction_data_len < offset_table_end {
        return fail_with_header(
            Phase41E_1Ed25519ByteParsingStatus::MalformedEd25519InstructionHeader,
            Some(Phase41BRejectionCase::MalformedEd25519InstructionData),
            Some(matched_instruction_index),
            instruction_data_len,
            Some(signature_count),
            Some(padding_byte),
        );
    }

    let Some(signature_offset) = read_u16_le_as_usize(instruction_data, 2) else {
        return malformed_header_with_header(
            matched_instruction_index,
            instruction_data_len,
            signature_count,
            padding_byte,
        );
    };
    let Some(signature_instruction_index) = read_u16_le(instruction_data, 4) else {
        return malformed_header_with_header(
            matched_instruction_index,
            instruction_data_len,
            signature_count,
            padding_byte,
        );
    };
    let Some(public_key_offset) = read_u16_le_as_usize(instruction_data, 6) else {
        return malformed_header_with_header(
            matched_instruction_index,
            instruction_data_len,
            signature_count,
            padding_byte,
        );
    };
    let Some(public_key_instruction_index) = read_u16_le(instruction_data, 8) else {
        return malformed_header_with_header(
            matched_instruction_index,
            instruction_data_len,
            signature_count,
            padding_byte,
        );
    };
    let Some(message_offset) = read_u16_le_as_usize(instruction_data, 10) else {
        return malformed_header_with_header(
            matched_instruction_index,
            instruction_data_len,
            signature_count,
            padding_byte,
        );
    };
    let Some(message_len) = read_u16_le_as_usize(instruction_data, 12) else {
        return malformed_header_with_header(
            matched_instruction_index,
            instruction_data_len,
            signature_count,
            padding_byte,
        );
    };
    let Some(message_instruction_index) = read_u16_le(instruction_data, 14) else {
        return malformed_header_with_header(
            matched_instruction_index,
            instruction_data_len,
            signature_count,
            padding_byte,
        );
    };

    let instruction_index_references = Phase41E_1Ed25519InstructionIndexReferences {
        signature_instruction_index,
        public_key_instruction_index,
        message_instruction_index,
    };

    if !all_references_are_current_instruction(instruction_index_references) {
        return fail_with_header(
            Phase41E_1Ed25519ByteParsingStatus::UnexpectedInstructionIndexReference,
            Some(Phase41BRejectionCase::UnsupportedOffsetLayout),
            Some(matched_instruction_index),
            instruction_data_len,
            Some(signature_count),
            Some(padding_byte),
        );
    }

    let Some(signature_range) = checked_byte_range(
        signature_offset,
        ED25519_SIGNATURE_LEN,
        instruction_data_len,
    ) else {
        return fail_with_header(
            Phase41E_1Ed25519ByteParsingStatus::OutOfBoundsSignatureOffset,
            Some(Phase41BRejectionCase::UnsupportedOffsetLayout),
            Some(matched_instruction_index),
            instruction_data_len,
            Some(signature_count),
            Some(padding_byte),
        );
    };

    let Some(public_key_range) = checked_byte_range(
        public_key_offset,
        ED25519_PUBLIC_KEY_LEN,
        instruction_data_len,
    ) else {
        return fail_with_header(
            Phase41E_1Ed25519ByteParsingStatus::OutOfBoundsPublicKeyOffset,
            Some(Phase41BRejectionCase::UnsupportedOffsetLayout),
            Some(matched_instruction_index),
            instruction_data_len,
            Some(signature_count),
            Some(padding_byte),
        );
    };

    if message_len == 0 {
        return fail_with_header(
            Phase41E_1Ed25519ByteParsingStatus::MissingMessageByteRange,
            Some(Phase41BRejectionCase::MalformedEd25519InstructionData),
            Some(matched_instruction_index),
            instruction_data_len,
            Some(signature_count),
            Some(padding_byte),
        );
    }

    let Some(message_range) = checked_byte_range(message_offset, message_len, instruction_data_len)
    else {
        return fail_with_header(
            Phase41E_1Ed25519ByteParsingStatus::OutOfBoundsMessageOffset,
            Some(Phase41BRejectionCase::UnsupportedOffsetLayout),
            Some(matched_instruction_index),
            instruction_data_len,
            Some(signature_count),
            Some(padding_byte),
        );
    };

    if !range_starts_after_offset_table(signature_range)
        || !range_starts_after_offset_table(public_key_range)
        || !range_starts_after_offset_table(message_range)
    {
        return fail_with_header(
            Phase41E_1Ed25519ByteParsingStatus::ParsedRangeAliasesOffsetTable,
            Some(Phase41BRejectionCase::UnsupportedOffsetLayout),
            Some(matched_instruction_index),
            instruction_data_len,
            Some(signature_count),
            Some(padding_byte),
        );
    }

    if byte_ranges_overlap(signature_range, public_key_range)
        || byte_ranges_overlap(signature_range, message_range)
        || byte_ranges_overlap(public_key_range, message_range)
    {
        return fail_with_header(
            Phase41E_1Ed25519ByteParsingStatus::OverlappingParsedByteRanges,
            Some(Phase41BRejectionCase::UnsupportedOffsetLayout),
            Some(matched_instruction_index),
            instruction_data_len,
            Some(signature_count),
            Some(padding_byte),
        );
    }

    let parsed_offsets = Phase41E_1ParsedEd25519Offsets {
        signature_offset,
        public_key_offset,
        message_offset,
        message_len,
        instruction_index_references,
        signature_range,
        public_key_range,
        message_range,
    };

    parsed(
        Some(matched_instruction_index),
        instruction_data_len,
        signature_count,
        padding_byte,
        parsed_offsets,
    )
}

fn malformed_header_with_header(
    matched_instruction_index: usize,
    instruction_data_len: usize,
    signature_count: u8,
    padding_byte: u8,
) -> Phase41E_1Ed25519ByteParsingResult {
    fail_with_header(
        Phase41E_1Ed25519ByteParsingStatus::MalformedEd25519InstructionHeader,
        Some(Phase41BRejectionCase::MalformedEd25519InstructionData),
        Some(matched_instruction_index),
        instruction_data_len,
        Some(signature_count),
        Some(padding_byte),
    )
}

fn read_u8(data: &[u8], offset: usize) -> Option<u8> {
    data.get(offset).copied()
}

fn read_u16_le(data: &[u8], offset: usize) -> Option<u16> {
    let high_offset = offset.checked_add(1)?;
    let low = read_u8(data, offset)?;
    let high = read_u8(data, high_offset)?;

    Some(u16::from_le_bytes([low, high]))
}

fn read_u16_le_as_usize(data: &[u8], offset: usize) -> Option<usize> {
    read_u16_le(data, offset).map(usize::from)
}

fn checked_byte_range(offset: usize, len: usize, data_len: usize) -> Option<Phase41E_1ByteRange> {
    let end = offset.checked_add(len)?;

    if end > data_len {
        return None;
    }

    Some(Phase41E_1ByteRange { offset, len })
}

fn range_starts_after_offset_table(range: Phase41E_1ByteRange) -> bool {
    range.offset >= ED25519_SINGLE_SIGNATURE_OFFSET_TABLE_LEN
}

fn byte_ranges_overlap(left: Phase41E_1ByteRange, right: Phase41E_1ByteRange) -> bool {
    let Some(left_end) = left.offset.checked_add(left.len) else {
        return true;
    };
    let Some(right_end) = right.offset.checked_add(right.len) else {
        return true;
    };

    left.offset < right_end && right.offset < left_end
}

fn all_references_are_current_instruction(
    references: Phase41E_1Ed25519InstructionIndexReferences,
) -> bool {
    references.signature_instruction_index == ED25519_CURRENT_INSTRUCTION_INDEX
        && references.public_key_instruction_index == ED25519_CURRENT_INSTRUCTION_INDEX
        && references.message_instruction_index == ED25519_CURRENT_INSTRUCTION_INDEX
}

fn parsed(
    matched_instruction_index: Option<usize>,
    instruction_data_len: usize,
    signature_count: u8,
    padding_byte: u8,
    parsed_offsets: Phase41E_1ParsedEd25519Offsets,
) -> Phase41E_1Ed25519ByteParsingResult {
    Phase41E_1Ed25519ByteParsingResult {
        status: Phase41E_1Ed25519ByteParsingStatus::Ed25519InstructionBytesParsed,
        rejection_case: None,
        matched_instruction_index,
        instruction_data_len,
        signature_count: Some(signature_count),
        padding_byte: Some(padding_byte),
        parsed_offsets: Some(parsed_offsets),
        parses_ed25519_instruction_bytes: true,
        entry_gate_requires_located_status: true,
        entry_gate_requires_matched_instruction_index: true,
        locates_prior_ed25519_instruction_used_as_gate: false,
        descriptor_booleans_trusted_as_evidence: false,
        consumes_phase_41d3_2_2_loaded_prior_instructions: true,
        consumes_phase_41d3_2_3_prefilter_result: true,
        loads_referenced_instructions: false,
        rejects_cross_instruction_references: true,
        rejects_offset_table_aliasing: true,
        stores_message_as_bounded_indices: true,
        copies_attacker_sized_message_data: false,
        rejects_overlapping_parsed_ranges: true,
        uses_checked_offset_arithmetic: true,
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

fn fail(
    status: Phase41E_1Ed25519ByteParsingStatus,
    rejection_case: Option<Phase41BRejectionCase>,
    matched_instruction_index: Option<usize>,
    instruction_data_len: usize,
) -> Phase41E_1Ed25519ByteParsingResult {
    fail_with_header(
        status,
        rejection_case,
        matched_instruction_index,
        instruction_data_len,
        None,
        None,
    )
}

fn fail_with_header(
    status: Phase41E_1Ed25519ByteParsingStatus,
    rejection_case: Option<Phase41BRejectionCase>,
    matched_instruction_index: Option<usize>,
    instruction_data_len: usize,
    signature_count: Option<u8>,
    padding_byte: Option<u8>,
) -> Phase41E_1Ed25519ByteParsingResult {
    Phase41E_1Ed25519ByteParsingResult {
        status,
        rejection_case,
        matched_instruction_index,
        instruction_data_len,
        signature_count,
        padding_byte,
        parsed_offsets: None,
        parses_ed25519_instruction_bytes: false,
        entry_gate_requires_located_status: true,
        entry_gate_requires_matched_instruction_index: true,
        locates_prior_ed25519_instruction_used_as_gate: false,
        descriptor_booleans_trusted_as_evidence: false,
        consumes_phase_41d3_2_2_loaded_prior_instructions: true,
        consumes_phase_41d3_2_3_prefilter_result: true,
        loads_referenced_instructions: false,
        rejects_cross_instruction_references: true,
        rejects_offset_table_aliasing: true,
        stores_message_as_bounded_indices: true,
        copies_attacker_sized_message_data: false,
        rejects_overlapping_parsed_ranges: true,
        uses_checked_offset_arithmetic: true,
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
    use solana_program::{instruction::Instruction, pubkey::Pubkey};

    use super::super::checked_prior_instruction_loading_runtime_boundary::{
        Phase41D3_2_2CheckedPriorInstructionLoadingResult,
        Phase41D3_2_2CheckedPriorInstructionLoadingStatus, Phase41D3_2_2LoadedPriorInstruction,
    };
    use super::super::prefilter_phase_41c3_candidate_descriptor_runtime_boundary::{
        Phase41D3_2_3PrefilterDescriptorResult, Phase41D3_2_3PrefilterDescriptorStatus,
    };
    use super::*;

    fn push_u16_le(data: &mut Vec<u8>, value: u16) {
        data.extend_from_slice(&value.to_le_bytes());
    }

    fn replace_u16_le(data: &mut Vec<u8>, offset: usize, value: u16) {
        let bytes = value.to_le_bytes();
        let Some(end) = offset.checked_add(2) else {
            return;
        };

        if let Some(target) = data.get_mut(offset..end) {
            for (slot, value_byte) in target.iter_mut().zip(bytes.iter()) {
                *slot = *value_byte;
            }
        }
    }

    fn replace_u8(data: &mut Vec<u8>, offset: usize, value: u8) {
        if let Some(target) = data.get_mut(offset) {
            *target = value;
        }
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

        data.extend(std::iter::repeat(1u8).take(ED25519_SIGNATURE_LEN));
        data.extend(std::iter::repeat(2u8).take(ED25519_PUBLIC_KEY_LEN));
        data.extend(std::iter::repeat(3u8).take(message_len));

        data
    }

    fn ed25519_instruction_with_data(data: Vec<u8>) -> Instruction {
        Instruction {
            program_id: ed25519_program::id(),
            accounts: Vec::new(),
            data,
        }
    }

    fn non_ed25519_instruction_with_data(data: Vec<u8>) -> Instruction {
        Instruction {
            program_id: Pubkey::new_from_array([9; 32]),
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
        status: Phase41D3_2_3PrefilterDescriptorStatus,
        matched_instruction_index: Option<usize>,
    ) -> Phase41D3_2_3PrefilterDescriptorResult {
        Phase41D3_2_3PrefilterDescriptorResult {
            status,
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
    fn rejects_when_structural_status_is_not_located_even_if_layer_flag_is_true() {
        let loading_result = loading_result(vec![loaded_prior_instruction(
            1,
            ed25519_instruction_with_data(valid_ed25519_instruction_data(8)),
        )]);
        let prefilter_result = prefilter_result(
            Phase41D3_2_3PrefilterDescriptorStatus::NoPriorEd25519CandidateDescriptors,
            Some(1),
        );

        let result = parse_located_ed25519_instruction_bytes(&loading_result, &prefilter_result);

        assert_eq!(
            result.status,
            Phase41E_1Ed25519ByteParsingStatus::PriorEd25519InstructionNotStructurallyLocated
        );
        assert!(!result.locates_prior_ed25519_instruction_used_as_gate);
        assert!(!result.parses_ed25519_instruction_bytes);
        assert!(!result.accepts_verification_evidence);
        assert!(!result.authorizes_execution);
    }

    #[test]
    fn rejects_when_matched_instruction_index_is_missing() {
        let loading_result = loading_result(vec![loaded_prior_instruction(
            1,
            ed25519_instruction_with_data(valid_ed25519_instruction_data(8)),
        )]);
        let prefilter_result = prefilter_result(
            Phase41D3_2_3PrefilterDescriptorStatus::PriorEd25519InstructionStructurallyLocated,
            None,
        );

        let result = parse_located_ed25519_instruction_bytes(&loading_result, &prefilter_result);

        assert_eq!(
            result.status,
            Phase41E_1Ed25519ByteParsingStatus::MatchedInstructionIndexUnavailable
        );
        assert!(!result.parses_ed25519_instruction_bytes);
        assert!(!result.accepts_verification_evidence);
    }

    #[test]
    fn rejects_when_matched_instruction_is_not_in_loaded_prior_set() {
        let loading_result = loading_result(vec![loaded_prior_instruction(
            1,
            ed25519_instruction_with_data(valid_ed25519_instruction_data(8)),
        )]);
        let prefilter_result = prefilter_result(
            Phase41D3_2_3PrefilterDescriptorStatus::PriorEd25519InstructionStructurallyLocated,
            Some(2),
        );

        let result = parse_located_ed25519_instruction_bytes(&loading_result, &prefilter_result);

        assert_eq!(
            result.status,
            Phase41E_1Ed25519ByteParsingStatus::MatchedInstructionUnavailable
        );
        assert!(!result.loads_referenced_instructions);
        assert!(!result.parses_ed25519_instruction_bytes);
    }

    #[test]
    fn parses_valid_single_signature_layout_as_non_authorizing_metadata() {
        let loading_result = loading_result(vec![loaded_prior_instruction(
            1,
            ed25519_instruction_with_data(valid_ed25519_instruction_data(8)),
        )]);
        let prefilter_result = prefilter_result(
            Phase41D3_2_3PrefilterDescriptorStatus::PriorEd25519InstructionStructurallyLocated,
            Some(1),
        );

        let result = parse_located_ed25519_instruction_bytes(&loading_result, &prefilter_result);

        assert_eq!(
            result.status,
            Phase41E_1Ed25519ByteParsingStatus::Ed25519InstructionBytesParsed
        );
        assert_eq!(result.rejection_case, None);
        assert_eq!(result.matched_instruction_index, Some(1));
        assert_eq!(result.signature_count, Some(1));
        assert_eq!(result.padding_byte, Some(0));
        assert!(result.parses_ed25519_instruction_bytes);
        assert!(result.stores_message_as_bounded_indices);
        assert!(result.rejects_offset_table_aliasing);
        assert!(!result.copies_attacker_sized_message_data);
        assert!(!result.ed25519_signature_verification_performed);
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

        match result.parsed_offsets {
            Some(offsets) => {
                assert_eq!(offsets.signature_range.len, ED25519_SIGNATURE_LEN);
                assert_eq!(offsets.public_key_range.len, ED25519_PUBLIC_KEY_LEN);
                assert_eq!(offsets.message_range.len, 8);
                assert_eq!(offsets.message_len, 8);
                assert_eq!(
                    offsets
                        .instruction_index_references
                        .signature_instruction_index,
                    ED25519_CURRENT_INSTRUCTION_INDEX
                );
                assert_eq!(
                    offsets
                        .instruction_index_references
                        .public_key_instruction_index,
                    ED25519_CURRENT_INSTRUCTION_INDEX
                );
                assert_eq!(
                    offsets
                        .instruction_index_references
                        .message_instruction_index,
                    ED25519_CURRENT_INSTRUCTION_INDEX
                );
            }
            None => assert!(result.parsed_offsets.is_some()),
        }
    }

    #[test]
    fn rejects_wrong_program_id_even_when_prefilter_status_claims_located() {
        let loading_result = loading_result(vec![loaded_prior_instruction(
            1,
            non_ed25519_instruction_with_data(valid_ed25519_instruction_data(8)),
        )]);
        let prefilter_result = prefilter_result(
            Phase41D3_2_3PrefilterDescriptorStatus::PriorEd25519InstructionStructurallyLocated,
            Some(1),
        );

        let result = parse_located_ed25519_instruction_bytes(&loading_result, &prefilter_result);

        assert_eq!(
            result.status,
            Phase41E_1Ed25519ByteParsingStatus::WrongEd25519ProgramId
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::WrongEd25519ProgramId)
        );
        assert!(!result.parses_ed25519_instruction_bytes);
    }

    #[test]
    fn rejects_non_runtime_data_entry_before_byte_parsing() {
        let mut entry = loaded_prior_instruction(
            1,
            ed25519_instruction_with_data(valid_ed25519_instruction_data(8)),
        );
        entry.is_evidence = true;

        let loading_result = loading_result(vec![entry]);
        let prefilter_result = prefilter_result(
            Phase41D3_2_3PrefilterDescriptorStatus::PriorEd25519InstructionStructurallyLocated,
            Some(1),
        );

        let result = parse_located_ed25519_instruction_bytes(&loading_result, &prefilter_result);

        assert_eq!(
            result.status,
            Phase41E_1Ed25519ByteParsingStatus::NonRuntimeDataInstructionEntry
        );
        assert!(!result.parses_ed25519_instruction_bytes);
        assert!(!result.accepts_verification_evidence);
    }

    #[test]
    fn rejects_empty_instruction_data() {
        let loading_result = loading_result(vec![loaded_prior_instruction(
            1,
            ed25519_instruction_with_data(Vec::new()),
        )]);
        let prefilter_result = prefilter_result(
            Phase41D3_2_3PrefilterDescriptorStatus::PriorEd25519InstructionStructurallyLocated,
            Some(1),
        );

        let result = parse_located_ed25519_instruction_bytes(&loading_result, &prefilter_result);

        assert_eq!(
            result.status,
            Phase41E_1Ed25519ByteParsingStatus::EmptyInstructionData
        );
        assert!(!result.parses_ed25519_instruction_bytes);
    }

    #[test]
    fn rejects_short_header() {
        let loading_result = loading_result(vec![loaded_prior_instruction(
            1,
            ed25519_instruction_with_data(Vec::from([1u8])),
        )]);
        let prefilter_result = prefilter_result(
            Phase41D3_2_3PrefilterDescriptorStatus::PriorEd25519InstructionStructurallyLocated,
            Some(1),
        );

        let result = parse_located_ed25519_instruction_bytes(&loading_result, &prefilter_result);

        assert_eq!(
            result.status,
            Phase41E_1Ed25519ByteParsingStatus::MalformedEd25519InstructionHeader
        );
        assert_eq!(result.signature_count, Some(1));
        assert_eq!(result.padding_byte, None);
    }

    #[test]
    fn rejects_nonzero_padding_byte() {
        let mut data = valid_ed25519_instruction_data(8);
        replace_u8(&mut data, 1, 7);

        let loading_result = loading_result(vec![loaded_prior_instruction(
            1,
            ed25519_instruction_with_data(data),
        )]);
        let prefilter_result = prefilter_result(
            Phase41D3_2_3PrefilterDescriptorStatus::PriorEd25519InstructionStructurallyLocated,
            Some(1),
        );

        let result = parse_located_ed25519_instruction_bytes(&loading_result, &prefilter_result);

        assert_eq!(
            result.status,
            Phase41E_1Ed25519ByteParsingStatus::MalformedEd25519InstructionHeader
        );
        assert_eq!(result.padding_byte, Some(7));
    }

    #[test]
    fn rejects_unsupported_signature_count() {
        let mut data = valid_ed25519_instruction_data(8);
        replace_u8(&mut data, 0, 2);

        let loading_result = loading_result(vec![loaded_prior_instruction(
            1,
            ed25519_instruction_with_data(data),
        )]);
        let prefilter_result = prefilter_result(
            Phase41D3_2_3PrefilterDescriptorStatus::PriorEd25519InstructionStructurallyLocated,
            Some(1),
        );

        let result = parse_located_ed25519_instruction_bytes(&loading_result, &prefilter_result);

        assert_eq!(
            result.status,
            Phase41E_1Ed25519ByteParsingStatus::UnsupportedSignatureCount
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::UnsupportedOffsetLayout)
        );
        assert!(!result.parses_ed25519_instruction_bytes);
    }

    #[test]
    fn rejects_cross_instruction_signature_reference_without_new_loading() {
        let mut data = valid_ed25519_instruction_data(8);
        replace_u16_le(&mut data, 4, 0);

        let loading_result = loading_result(vec![loaded_prior_instruction(
            1,
            ed25519_instruction_with_data(data),
        )]);
        let prefilter_result = prefilter_result(
            Phase41D3_2_3PrefilterDescriptorStatus::PriorEd25519InstructionStructurallyLocated,
            Some(1),
        );

        let result = parse_located_ed25519_instruction_bytes(&loading_result, &prefilter_result);

        assert_eq!(
            result.status,
            Phase41E_1Ed25519ByteParsingStatus::UnexpectedInstructionIndexReference
        );
        assert!(result.rejects_cross_instruction_references);
        assert!(!result.loads_referenced_instructions);
        assert!(!result.parses_ed25519_instruction_bytes);
    }

    #[test]
    fn rejects_cross_instruction_public_key_reference_without_new_loading() {
        let mut data = valid_ed25519_instruction_data(8);
        replace_u16_le(&mut data, 8, 0);

        let loading_result = loading_result(vec![loaded_prior_instruction(
            1,
            ed25519_instruction_with_data(data),
        )]);
        let prefilter_result = prefilter_result(
            Phase41D3_2_3PrefilterDescriptorStatus::PriorEd25519InstructionStructurallyLocated,
            Some(1),
        );

        let result = parse_located_ed25519_instruction_bytes(&loading_result, &prefilter_result);

        assert_eq!(
            result.status,
            Phase41E_1Ed25519ByteParsingStatus::UnexpectedInstructionIndexReference
        );
        assert!(!result.loads_referenced_instructions);
    }

    #[test]
    fn rejects_cross_instruction_message_reference_without_new_loading() {
        let mut data = valid_ed25519_instruction_data(8);
        replace_u16_le(&mut data, 14, 0);

        let loading_result = loading_result(vec![loaded_prior_instruction(
            1,
            ed25519_instruction_with_data(data),
        )]);
        let prefilter_result = prefilter_result(
            Phase41D3_2_3PrefilterDescriptorStatus::PriorEd25519InstructionStructurallyLocated,
            Some(1),
        );

        let result = parse_located_ed25519_instruction_bytes(&loading_result, &prefilter_result);

        assert_eq!(
            result.status,
            Phase41E_1Ed25519ByteParsingStatus::UnexpectedInstructionIndexReference
        );
        assert!(!result.loads_referenced_instructions);
    }

    #[test]
    fn rejects_out_of_bounds_signature_offset() {
        let mut data = valid_ed25519_instruction_data(8);
        replace_u16_le(&mut data, 2, u16::MAX);

        let loading_result = loading_result(vec![loaded_prior_instruction(
            1,
            ed25519_instruction_with_data(data),
        )]);
        let prefilter_result = prefilter_result(
            Phase41D3_2_3PrefilterDescriptorStatus::PriorEd25519InstructionStructurallyLocated,
            Some(1),
        );

        let result = parse_located_ed25519_instruction_bytes(&loading_result, &prefilter_result);

        assert_eq!(
            result.status,
            Phase41E_1Ed25519ByteParsingStatus::OutOfBoundsSignatureOffset
        );
        assert!(!result.parses_ed25519_instruction_bytes);
    }

    #[test]
    fn rejects_out_of_bounds_public_key_offset() {
        let mut data = valid_ed25519_instruction_data(8);
        replace_u16_le(&mut data, 6, u16::MAX);

        let loading_result = loading_result(vec![loaded_prior_instruction(
            1,
            ed25519_instruction_with_data(data),
        )]);
        let prefilter_result = prefilter_result(
            Phase41D3_2_3PrefilterDescriptorStatus::PriorEd25519InstructionStructurallyLocated,
            Some(1),
        );

        let result = parse_located_ed25519_instruction_bytes(&loading_result, &prefilter_result);

        assert_eq!(
            result.status,
            Phase41E_1Ed25519ByteParsingStatus::OutOfBoundsPublicKeyOffset
        );
        assert!(!result.parses_ed25519_instruction_bytes);
    }

    #[test]
    fn rejects_missing_message_byte_range() {
        let mut data = valid_ed25519_instruction_data(8);
        replace_u16_le(&mut data, 12, 0);

        let loading_result = loading_result(vec![loaded_prior_instruction(
            1,
            ed25519_instruction_with_data(data),
        )]);
        let prefilter_result = prefilter_result(
            Phase41D3_2_3PrefilterDescriptorStatus::PriorEd25519InstructionStructurallyLocated,
            Some(1),
        );

        let result = parse_located_ed25519_instruction_bytes(&loading_result, &prefilter_result);

        assert_eq!(
            result.status,
            Phase41E_1Ed25519ByteParsingStatus::MissingMessageByteRange
        );
        assert!(!result.parses_ed25519_instruction_bytes);
    }

    #[test]
    fn rejects_out_of_bounds_message_offset() {
        let mut data = valid_ed25519_instruction_data(8);
        replace_u16_le(&mut data, 10, u16::MAX);

        let loading_result = loading_result(vec![loaded_prior_instruction(
            1,
            ed25519_instruction_with_data(data),
        )]);
        let prefilter_result = prefilter_result(
            Phase41D3_2_3PrefilterDescriptorStatus::PriorEd25519InstructionStructurallyLocated,
            Some(1),
        );

        let result = parse_located_ed25519_instruction_bytes(&loading_result, &prefilter_result);

        assert_eq!(
            result.status,
            Phase41E_1Ed25519ByteParsingStatus::OutOfBoundsMessageOffset
        );
        assert!(!result.parses_ed25519_instruction_bytes);
    }

    #[test]
    fn rejects_signature_range_aliasing_offset_table() {
        let mut data = valid_ed25519_instruction_data(8);
        replace_u16_le(&mut data, 2, 0);

        let loading_result = loading_result(vec![loaded_prior_instruction(
            1,
            ed25519_instruction_with_data(data),
        )]);
        let prefilter_result = prefilter_result(
            Phase41D3_2_3PrefilterDescriptorStatus::PriorEd25519InstructionStructurallyLocated,
            Some(1),
        );

        let result = parse_located_ed25519_instruction_bytes(&loading_result, &prefilter_result);

        assert_eq!(
            result.status,
            Phase41E_1Ed25519ByteParsingStatus::ParsedRangeAliasesOffsetTable
        );
        assert!(result.rejects_offset_table_aliasing);
        assert!(!result.parses_ed25519_instruction_bytes);
    }

    #[test]
    fn rejects_public_key_range_aliasing_offset_table() {
        let mut data = valid_ed25519_instruction_data(8);
        replace_u16_le(&mut data, 6, 1);

        let loading_result = loading_result(vec![loaded_prior_instruction(
            1,
            ed25519_instruction_with_data(data),
        )]);
        let prefilter_result = prefilter_result(
            Phase41D3_2_3PrefilterDescriptorStatus::PriorEd25519InstructionStructurallyLocated,
            Some(1),
        );

        let result = parse_located_ed25519_instruction_bytes(&loading_result, &prefilter_result);

        assert_eq!(
            result.status,
            Phase41E_1Ed25519ByteParsingStatus::ParsedRangeAliasesOffsetTable
        );
        assert!(result.rejects_offset_table_aliasing);
        assert!(!result.parses_ed25519_instruction_bytes);
    }

    #[test]
    fn rejects_message_range_aliasing_offset_table() {
        let mut data = valid_ed25519_instruction_data(8);
        replace_u16_le(&mut data, 10, 15);
        replace_u16_le(&mut data, 12, 1);

        let loading_result = loading_result(vec![loaded_prior_instruction(
            1,
            ed25519_instruction_with_data(data),
        )]);
        let prefilter_result = prefilter_result(
            Phase41D3_2_3PrefilterDescriptorStatus::PriorEd25519InstructionStructurallyLocated,
            Some(1),
        );

        let result = parse_located_ed25519_instruction_bytes(&loading_result, &prefilter_result);

        assert_eq!(
            result.status,
            Phase41E_1Ed25519ByteParsingStatus::ParsedRangeAliasesOffsetTable
        );
        assert!(result.rejects_offset_table_aliasing);
        assert!(!result.parses_ed25519_instruction_bytes);
    }

    #[test]
    fn rejects_overlapping_parsed_ranges_deterministically() {
        let mut data = valid_ed25519_instruction_data(8);
        replace_u16_le(
            &mut data,
            10,
            ED25519_SINGLE_SIGNATURE_OFFSET_TABLE_LEN as u16,
        );
        replace_u16_le(&mut data, 12, 8);

        let loading_result = loading_result(vec![loaded_prior_instruction(
            1,
            ed25519_instruction_with_data(data),
        )]);
        let prefilter_result = prefilter_result(
            Phase41D3_2_3PrefilterDescriptorStatus::PriorEd25519InstructionStructurallyLocated,
            Some(1),
        );

        let result = parse_located_ed25519_instruction_bytes(&loading_result, &prefilter_result);

        assert_eq!(
            result.status,
            Phase41E_1Ed25519ByteParsingStatus::OverlappingParsedByteRanges
        );
        assert!(result.rejects_overlapping_parsed_ranges);
        assert!(!result.parses_ed25519_instruction_bytes);
    }

    #[test]
    fn phase_41e_1_report_preserves_non_authorizing_parser_boundary() {
        let report = phase_41e_1_ed25519_byte_parsing_boundary_report();

        assert_eq!(report.phase, "41E.1");
        assert_eq!(report.version, "0.1.0");
        assert!(report.parses_ed25519_instruction_bytes);
        assert!(report.entry_gate_requires_located_status);
        assert!(report.entry_gate_requires_matched_instruction_index);
        assert!(!report.locates_prior_ed25519_instruction_used_as_gate);
        assert!(!report.descriptor_booleans_trusted_as_evidence);
        assert!(report.consumes_phase_41d3_2_2_loaded_prior_instructions);
        assert!(report.consumes_phase_41d3_2_3_prefilter_result);
        assert!(!report.loads_referenced_instructions);
        assert!(report.rejects_cross_instruction_references);
        assert!(report.rejects_offset_table_aliasing);
        assert!(report.stores_message_as_bounded_indices);
        assert!(!report.copies_attacker_sized_message_data);
        assert!(report.rejects_overlapping_parsed_ranges);
        assert!(report.uses_checked_offset_arithmetic);
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
        assert!(!report.safety_flags.spl_token_mint_to_enabled);
        assert!(!report.safety_flags.live_route_enabled);
    }
}
