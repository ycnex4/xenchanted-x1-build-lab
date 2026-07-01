use super::CANONICAL_PAYLOAD_HASH_VALIDATOR_PHASE_34;
use super::GUARDIAN_QUORUM_STRUCTURAL_VERIFIER_PHASE_35;

pub const ED25519_INSTRUCTION_EVIDENCE_LAYOUT_MODEL_PHASE_37: &str =
    "ED25519_INSTRUCTION_EVIDENCE_LAYOUT_MODEL_PHASE_37";
pub const ED25519_INSTRUCTION_EVIDENCE_LAYOUT_MODEL_VERSION: u8 = 1;

/// Length of the Ed25519 program instruction data header: `num_signatures: u8`
/// at offset 0 followed by `padding: u8` at offset 1.
pub const ED25519_INSTRUCTION_HEADER_LEN: usize = 2;

/// Length of one `Ed25519SignatureOffsets` record: 7 x `u16` little-endian
/// fields (`signature_offset`, `signature_instruction_index`,
/// `public_key_offset`, `public_key_instruction_index`, `message_data_offset`,
/// `message_data_size`, `message_instruction_index`).
pub const ED25519_SIGNATURE_OFFSETS_RECORD_LEN: usize = 14;

/// Fixed Ed25519 signature length in bytes.
pub const ED25519_SIGNATURE_LEN: usize = 64;

/// Fixed Ed25519 public key length in bytes.
pub const ED25519_PUBLIC_KEY_LEN: usize = 32;

/// Expected message length in bytes. The signed message is the 32-byte Phase 34
/// canonical payload hash, so the model requires exactly 32 bytes.
pub const EXPECTED_MESSAGE_LEN: usize = 32;

/// The single-guardian-per-instruction evidence model expects exactly one
/// signature record per ed25519 program instruction.
pub const EXPECTED_SIGNATURE_COUNT: u8 = 1;

/// Instruction-index sentinel meaning "this same ed25519 instruction's data".
pub const CURRENT_INSTRUCTION_INDEX_SENTINEL: u16 = u16::MAX;

/// Reference-only identifier of the standard Solana Ed25519 program.
///
/// This constant documents the program the future runtime parser would inspect.
/// It is NOT wired into any runtime code path in this phase.
pub const ED25519_PROGRAM_ID_REFERENCE: &str = "Ed25519SigVerify111111111111111111111111111";

/// Modeled `Ed25519SignatureOffsets` record. These are caller-provided
/// descriptions of an instruction data layout, not values read from any real
/// Instructions sysvar or instruction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ed25519SignatureOffsetsModel {
    pub signature_offset: u16,
    pub signature_instruction_index: u16,
    pub public_key_offset: u16,
    pub public_key_instruction_index: u16,
    pub message_data_offset: u16,
    pub message_data_size: u16,
    pub message_instruction_index: u16,
}

/// Caller-supplied descriptor of an ed25519 program instruction's data layout.
///
/// The validator only reasons about these declared sizes/offsets. It never
/// reads a real sysvar, a real instruction, or any account.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ed25519EvidenceLayoutDescriptor {
    pub instruction_data_len: usize,
    pub num_signatures: u8,
    pub offsets: Ed25519SignatureOffsetsModel,
}

/// Result of a successful structural shape validation.
///
/// Every field makes explicit that NO cryptographic verification, sysvar read,
/// or signature-proof acceptance happened.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ed25519EvidenceLayoutModelResult {
    pub num_signatures: u8,
    pub signature_len: usize,
    pub public_key_len: usize,
    pub message_len: usize,
    pub message_bound_to_phase_34_hash_len: bool,
    pub ed25519_signature_verification_performed: bool,
    pub instructions_sysvar_read: bool,
    pub cryptographic_signature_proof_accepted: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ed25519EvidenceLayoutErrorKind {
    UnexpectedSignatureCount,
    UnexpectedMessageSize,
    TruncatedInstructionData,
    SignatureRegionOverlapsHeader,
    SignatureRegionOutOfBounds,
    PublicKeyRegionOverlapsHeader,
    PublicKeyRegionOutOfBounds,
    MessageRegionOverlapsHeader,
    MessageRegionOutOfBounds,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ed25519EvidenceLayoutError {
    pub kind: Ed25519EvidenceLayoutErrorKind,
    pub instruction_data_len: usize,
    pub num_signatures: u8,
    pub region_start: Option<usize>,
    pub region_end: Option<usize>,
    pub expected_len: Option<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ed25519EvidenceLayoutModelReport {
    pub model_id: &'static str,
    pub model_version: u8,
    pub header_len: usize,
    pub offsets_record_len: usize,
    pub signature_len: usize,
    pub public_key_len: usize,
    pub expected_message_len: usize,
    pub expected_signature_count: u8,
    pub current_instruction_index_sentinel: u16,
    pub ed25519_program_id_reference: &'static str,
    pub layout_shape_check_enabled: bool,
    pub single_signature_model_enforced: bool,
    pub message_size_bound_to_phase_34_hash: bool,
    pub offset_bounds_checked: bool,
    pub truncated_instruction_rejected: bool,
    pub padding_byte_modeled: bool,
    pub ed25519_signature_verification_enabled: bool,
    pub instruction_sysvar_parsing_enabled: bool,
    pub ed25519_program_instruction_validation_enabled: bool,
    pub cryptographic_signature_proof_accepted: bool,
    pub caller_provided_signature_claim_trusted: bool,
    pub phase_34_hash_validator_available: bool,
    pub phase_34_hash_validator_phase: &'static str,
    pub expected_message_len_matches_phase_34_hash: bool,
    pub phase_35_structural_quorum_phase: &'static str,
    pub phase_35_structural_quorum_still_required: bool,
    pub source_proof_verified: bool,
    pub route_config_verified: bool,
    pub amount_cap_enforced: bool,
    pub target_mint_account_checked: bool,
    pub replay_check_enabled: bool,
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

pub const ED25519_INSTRUCTION_EVIDENCE_LAYOUT_MODEL_REPORT: Ed25519EvidenceLayoutModelReport =
    Ed25519EvidenceLayoutModelReport {
        model_id: ED25519_INSTRUCTION_EVIDENCE_LAYOUT_MODEL_PHASE_37,
        model_version: ED25519_INSTRUCTION_EVIDENCE_LAYOUT_MODEL_VERSION,
        header_len: ED25519_INSTRUCTION_HEADER_LEN,
        offsets_record_len: ED25519_SIGNATURE_OFFSETS_RECORD_LEN,
        signature_len: ED25519_SIGNATURE_LEN,
        public_key_len: ED25519_PUBLIC_KEY_LEN,
        expected_message_len: EXPECTED_MESSAGE_LEN,
        expected_signature_count: EXPECTED_SIGNATURE_COUNT,
        current_instruction_index_sentinel: CURRENT_INSTRUCTION_INDEX_SENTINEL,
        ed25519_program_id_reference: ED25519_PROGRAM_ID_REFERENCE,
        layout_shape_check_enabled: true,
        single_signature_model_enforced: true,
        message_size_bound_to_phase_34_hash: true,
        offset_bounds_checked: true,
        truncated_instruction_rejected: true,
        padding_byte_modeled: true,
        ed25519_signature_verification_enabled: false,
        instruction_sysvar_parsing_enabled: false,
        ed25519_program_instruction_validation_enabled: false,
        cryptographic_signature_proof_accepted: false,
        caller_provided_signature_claim_trusted: false,
        phase_34_hash_validator_available: true,
        phase_34_hash_validator_phase: CANONICAL_PAYLOAD_HASH_VALIDATOR_PHASE_34,
        expected_message_len_matches_phase_34_hash: true,
        phase_35_structural_quorum_phase: GUARDIAN_QUORUM_STRUCTURAL_VERIFIER_PHASE_35,
        phase_35_structural_quorum_still_required: true,
        source_proof_verified: false,
        route_config_verified: false,
        amount_cap_enforced: false,
        target_mint_account_checked: false,
        replay_check_enabled: false,
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

/// Validate ONLY the structural shape of a caller-described ed25519 program
/// instruction data layout against the expected single-guardian evidence model.
///
/// This performs no cryptographic verification, reads no Instructions sysvar,
/// and accepts no signature proof. It checks that:
///
/// - exactly one signature is declared (single-guardian-per-instruction);
/// - the message region is exactly the 32-byte Phase 34 canonical payload hash;
/// - the instruction data is large enough for the header plus one offsets record;
/// - every declared signature/public-key/message region lies within the declared
///   instruction data length and does not overlap the header.
pub fn validate_ed25519_evidence_layout(
    descriptor: Ed25519EvidenceLayoutDescriptor,
) -> Result<Ed25519EvidenceLayoutModelResult, Ed25519EvidenceLayoutError> {
    if descriptor.num_signatures != EXPECTED_SIGNATURE_COUNT {
        return Err(Ed25519EvidenceLayoutError {
            kind: Ed25519EvidenceLayoutErrorKind::UnexpectedSignatureCount,
            instruction_data_len: descriptor.instruction_data_len,
            num_signatures: descriptor.num_signatures,
            region_start: None,
            region_end: None,
            expected_len: Some(EXPECTED_SIGNATURE_COUNT as usize),
        });
    }

    if descriptor.offsets.message_data_size as usize != EXPECTED_MESSAGE_LEN {
        return Err(Ed25519EvidenceLayoutError {
            kind: Ed25519EvidenceLayoutErrorKind::UnexpectedMessageSize,
            instruction_data_len: descriptor.instruction_data_len,
            num_signatures: descriptor.num_signatures,
            region_start: None,
            region_end: Some(descriptor.offsets.message_data_size as usize),
            expected_len: Some(EXPECTED_MESSAGE_LEN),
        });
    }

    let minimum_instruction_data_len =
        ED25519_INSTRUCTION_HEADER_LEN + ED25519_SIGNATURE_OFFSETS_RECORD_LEN;

    if descriptor.instruction_data_len < minimum_instruction_data_len {
        return Err(Ed25519EvidenceLayoutError {
            kind: Ed25519EvidenceLayoutErrorKind::TruncatedInstructionData,
            instruction_data_len: descriptor.instruction_data_len,
            num_signatures: descriptor.num_signatures,
            region_start: None,
            region_end: None,
            expected_len: Some(minimum_instruction_data_len),
        });
    }

    check_region(
        &descriptor,
        descriptor.offsets.signature_offset as usize,
        ED25519_SIGNATURE_LEN,
        Ed25519EvidenceLayoutErrorKind::SignatureRegionOverlapsHeader,
        Ed25519EvidenceLayoutErrorKind::SignatureRegionOutOfBounds,
    )?;

    check_region(
        &descriptor,
        descriptor.offsets.public_key_offset as usize,
        ED25519_PUBLIC_KEY_LEN,
        Ed25519EvidenceLayoutErrorKind::PublicKeyRegionOverlapsHeader,
        Ed25519EvidenceLayoutErrorKind::PublicKeyRegionOutOfBounds,
    )?;

    check_region(
        &descriptor,
        descriptor.offsets.message_data_offset as usize,
        EXPECTED_MESSAGE_LEN,
        Ed25519EvidenceLayoutErrorKind::MessageRegionOverlapsHeader,
        Ed25519EvidenceLayoutErrorKind::MessageRegionOutOfBounds,
    )?;

    Ok(Ed25519EvidenceLayoutModelResult {
        num_signatures: descriptor.num_signatures,
        signature_len: ED25519_SIGNATURE_LEN,
        public_key_len: ED25519_PUBLIC_KEY_LEN,
        message_len: EXPECTED_MESSAGE_LEN,
        message_bound_to_phase_34_hash_len: true,
        ed25519_signature_verification_performed: false,
        instructions_sysvar_read: false,
        cryptographic_signature_proof_accepted: false,
    })
}

pub fn ed25519_evidence_layout_model_report() -> &'static Ed25519EvidenceLayoutModelReport {
    &ED25519_INSTRUCTION_EVIDENCE_LAYOUT_MODEL_REPORT
}

fn check_region(
    descriptor: &Ed25519EvidenceLayoutDescriptor,
    start: usize,
    len: usize,
    overlap_header_kind: Ed25519EvidenceLayoutErrorKind,
    out_of_bounds_kind: Ed25519EvidenceLayoutErrorKind,
) -> Result<(), Ed25519EvidenceLayoutError> {
    if start < ED25519_INSTRUCTION_HEADER_LEN {
        return Err(Ed25519EvidenceLayoutError {
            kind: overlap_header_kind,
            instruction_data_len: descriptor.instruction_data_len,
            num_signatures: descriptor.num_signatures,
            region_start: Some(start),
            region_end: Some(start.saturating_add(len)),
            expected_len: Some(len),
        });
    }

    let end = match start.checked_add(len) {
        Some(end) => end,
        None => {
            return Err(Ed25519EvidenceLayoutError {
                kind: out_of_bounds_kind,
                instruction_data_len: descriptor.instruction_data_len,
                num_signatures: descriptor.num_signatures,
                region_start: Some(start),
                region_end: None,
                expected_len: Some(len),
            });
        }
    };

    if end > descriptor.instruction_data_len {
        return Err(Ed25519EvidenceLayoutError {
            kind: out_of_bounds_kind,
            instruction_data_len: descriptor.instruction_data_len,
            num_signatures: descriptor.num_signatures,
            region_start: Some(start),
            region_end: Some(end),
            expected_len: Some(len),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::verifier::{
        canonical_payload_hash_validation_report, read_only_verifier_boundary,
        CANONICAL_PAYLOAD_HASH_VALIDATOR_PHASE_34, GUARDIAN_QUORUM_STRUCTURAL_VERIFIER_PHASE_35,
        READ_ONLY_RUNTIME_VERIFIER_SCAFFOLD_PHASE_32, XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1,
    };

    // A canonical single-guardian layout laid out contiguously after the header:
    //   [0..2)    header (num_signatures, padding)
    //   [2..16)   offsets record (14 bytes)
    //   [16..80)  signature (64 bytes)
    //   [80..112) public key (32 bytes)
    //   [112..144) message = 32-byte Phase 34 hash
    const SIGNATURE_OFFSET: u16 = 16;
    const PUBLIC_KEY_OFFSET: u16 = 80;
    const MESSAGE_OFFSET: u16 = 112;
    const VALID_INSTRUCTION_DATA_LEN: usize = 144;

    fn valid_offsets() -> Ed25519SignatureOffsetsModel {
        Ed25519SignatureOffsetsModel {
            signature_offset: SIGNATURE_OFFSET,
            signature_instruction_index: CURRENT_INSTRUCTION_INDEX_SENTINEL,
            public_key_offset: PUBLIC_KEY_OFFSET,
            public_key_instruction_index: CURRENT_INSTRUCTION_INDEX_SENTINEL,
            message_data_offset: MESSAGE_OFFSET,
            message_data_size: EXPECTED_MESSAGE_LEN as u16,
            message_instruction_index: CURRENT_INSTRUCTION_INDEX_SENTINEL,
        }
    }

    fn valid_descriptor() -> Ed25519EvidenceLayoutDescriptor {
        Ed25519EvidenceLayoutDescriptor {
            instruction_data_len: VALID_INSTRUCTION_DATA_LEN,
            num_signatures: EXPECTED_SIGNATURE_COUNT,
            offsets: valid_offsets(),
        }
    }

    fn assert_error_kind(
        result: Result<Ed25519EvidenceLayoutModelResult, Ed25519EvidenceLayoutError>,
        kind: Ed25519EvidenceLayoutErrorKind,
    ) -> Ed25519EvidenceLayoutError {
        let error = result.expect_err("ed25519 evidence layout error");
        assert_eq!(error.kind, kind);
        error
    }

    #[test]
    fn marker_version_and_layout_constants_are_stable() {
        assert_eq!(
            ED25519_INSTRUCTION_EVIDENCE_LAYOUT_MODEL_PHASE_37,
            "ED25519_INSTRUCTION_EVIDENCE_LAYOUT_MODEL_PHASE_37"
        );
        assert_eq!(ED25519_INSTRUCTION_EVIDENCE_LAYOUT_MODEL_VERSION, 1);
        assert_eq!(ED25519_INSTRUCTION_HEADER_LEN, 2);
        assert_eq!(ED25519_SIGNATURE_OFFSETS_RECORD_LEN, 14);
        assert_eq!(ED25519_SIGNATURE_LEN, 64);
        assert_eq!(ED25519_PUBLIC_KEY_LEN, 32);
        assert_eq!(EXPECTED_MESSAGE_LEN, 32);
        assert_eq!(EXPECTED_SIGNATURE_COUNT, 1);
        assert_eq!(CURRENT_INSTRUCTION_INDEX_SENTINEL, u16::MAX);
    }

    #[test]
    fn valid_single_signature_layout_is_accepted() {
        let result =
            validate_ed25519_evidence_layout(valid_descriptor()).expect("valid layout shape");

        assert_eq!(result.num_signatures, 1);
        assert_eq!(result.signature_len, 64);
        assert_eq!(result.public_key_len, 32);
        assert_eq!(result.message_len, 32);
        assert!(result.message_bound_to_phase_34_hash_len);
        assert!(!result.ed25519_signature_verification_performed);
        assert!(!result.instructions_sysvar_read);
        assert!(!result.cryptographic_signature_proof_accepted);
    }

    #[test]
    fn unexpected_signature_count_is_rejected() {
        let mut descriptor = valid_descriptor();
        descriptor.num_signatures = 2;

        let error = assert_error_kind(
            validate_ed25519_evidence_layout(descriptor),
            Ed25519EvidenceLayoutErrorKind::UnexpectedSignatureCount,
        );

        assert_eq!(error.num_signatures, 2);
        assert_eq!(error.expected_len, Some(1));
    }

    #[test]
    fn zero_signature_count_is_rejected() {
        let mut descriptor = valid_descriptor();
        descriptor.num_signatures = 0;

        let error = assert_error_kind(
            validate_ed25519_evidence_layout(descriptor),
            Ed25519EvidenceLayoutErrorKind::UnexpectedSignatureCount,
        );

        assert_eq!(error.num_signatures, 0);
    }

    #[test]
    fn non_32_byte_message_size_is_rejected() {
        let mut descriptor = valid_descriptor();
        descriptor.offsets.message_data_size = 31;

        let error = assert_error_kind(
            validate_ed25519_evidence_layout(descriptor),
            Ed25519EvidenceLayoutErrorKind::UnexpectedMessageSize,
        );

        assert_eq!(error.expected_len, Some(32));
        assert_eq!(error.region_end, Some(31));
    }

    #[test]
    fn truncated_instruction_data_is_rejected() {
        let mut descriptor = valid_descriptor();
        // Smaller than header (2) + offsets record (14) = 16 bytes.
        descriptor.instruction_data_len = 15;

        let error = assert_error_kind(
            validate_ed25519_evidence_layout(descriptor),
            Ed25519EvidenceLayoutErrorKind::TruncatedInstructionData,
        );

        assert_eq!(error.instruction_data_len, 15);
        assert_eq!(error.expected_len, Some(16));
    }

    #[test]
    fn signature_region_overlapping_header_is_rejected() {
        let mut descriptor = valid_descriptor();
        descriptor.offsets.signature_offset = 1;

        let error = assert_error_kind(
            validate_ed25519_evidence_layout(descriptor),
            Ed25519EvidenceLayoutErrorKind::SignatureRegionOverlapsHeader,
        );

        assert_eq!(error.region_start, Some(1));
    }

    #[test]
    fn signature_region_out_of_bounds_is_rejected() {
        let mut descriptor = valid_descriptor();
        // Signature region would run past the declared instruction data length.
        descriptor.offsets.signature_offset = (VALID_INSTRUCTION_DATA_LEN - 1) as u16;

        let error = assert_error_kind(
            validate_ed25519_evidence_layout(descriptor),
            Ed25519EvidenceLayoutErrorKind::SignatureRegionOutOfBounds,
        );

        assert_eq!(error.region_start, Some(VALID_INSTRUCTION_DATA_LEN - 1));
        assert_eq!(error.expected_len, Some(ED25519_SIGNATURE_LEN));
    }

    #[test]
    fn public_key_region_overlapping_header_is_rejected() {
        let mut descriptor = valid_descriptor();
        descriptor.offsets.public_key_offset = 0;

        let error = assert_error_kind(
            validate_ed25519_evidence_layout(descriptor),
            Ed25519EvidenceLayoutErrorKind::PublicKeyRegionOverlapsHeader,
        );

        assert_eq!(error.region_start, Some(0));
    }

    #[test]
    fn public_key_region_out_of_bounds_is_rejected() {
        let mut descriptor = valid_descriptor();
        descriptor.offsets.public_key_offset = (VALID_INSTRUCTION_DATA_LEN - 4) as u16;

        let error = assert_error_kind(
            validate_ed25519_evidence_layout(descriptor),
            Ed25519EvidenceLayoutErrorKind::PublicKeyRegionOutOfBounds,
        );

        assert_eq!(error.expected_len, Some(ED25519_PUBLIC_KEY_LEN));
    }

    #[test]
    fn message_region_overlapping_header_is_rejected() {
        let mut descriptor = valid_descriptor();
        descriptor.offsets.message_data_offset = 1;

        let error = assert_error_kind(
            validate_ed25519_evidence_layout(descriptor),
            Ed25519EvidenceLayoutErrorKind::MessageRegionOverlapsHeader,
        );

        assert_eq!(error.region_start, Some(1));
    }

    #[test]
    fn message_region_out_of_bounds_is_rejected() {
        let mut descriptor = valid_descriptor();
        descriptor.offsets.message_data_offset = (VALID_INSTRUCTION_DATA_LEN - 16) as u16;

        let error = assert_error_kind(
            validate_ed25519_evidence_layout(descriptor),
            Ed25519EvidenceLayoutErrorKind::MessageRegionOutOfBounds,
        );

        assert_eq!(error.expected_len, Some(EXPECTED_MESSAGE_LEN));
    }

    #[test]
    fn success_result_exposes_no_cryptographic_verification() {
        let result =
            validate_ed25519_evidence_layout(valid_descriptor()).expect("valid layout shape");

        assert!(!result.ed25519_signature_verification_performed);
        assert!(!result.cryptographic_signature_proof_accepted);
        assert!(!result.instructions_sysvar_read);
    }

    #[test]
    fn report_says_ed25519_and_signature_proof_are_not_implemented() {
        let report = ed25519_evidence_layout_model_report();

        assert_eq!(
            report.model_id,
            ED25519_INSTRUCTION_EVIDENCE_LAYOUT_MODEL_PHASE_37
        );
        assert_eq!(
            report.model_version,
            ED25519_INSTRUCTION_EVIDENCE_LAYOUT_MODEL_VERSION
        );
        assert!(report.layout_shape_check_enabled);
        assert!(report.single_signature_model_enforced);
        assert!(report.message_size_bound_to_phase_34_hash);
        assert!(report.offset_bounds_checked);
        assert!(report.truncated_instruction_rejected);
        assert!(report.padding_byte_modeled);
        assert!(!report.ed25519_signature_verification_enabled);
        assert!(!report.instruction_sysvar_parsing_enabled);
        assert!(!report.ed25519_program_instruction_validation_enabled);
        assert!(!report.cryptographic_signature_proof_accepted);
        assert!(!report.caller_provided_signature_claim_trusted);
    }

    #[test]
    fn report_keeps_unfinished_verifier_and_execution_surfaces_disabled() {
        let report = ed25519_evidence_layout_model_report();

        assert!(!report.source_proof_verified);
        assert!(!report.route_config_verified);
        assert!(!report.amount_cap_enforced);
        assert!(!report.target_mint_account_checked);
        assert!(!report.replay_check_enabled);
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

    #[test]
    fn phase_34_hash_validator_remains_available_and_recomputing() {
        let report = ed25519_evidence_layout_model_report();
        let phase_34_report = canonical_payload_hash_validation_report();

        assert!(report.phase_34_hash_validator_available);
        assert_eq!(
            report.phase_34_hash_validator_phase,
            CANONICAL_PAYLOAD_HASH_VALIDATOR_PHASE_34
        );
        assert!(report.expected_message_len_matches_phase_34_hash);
        assert_eq!(report.expected_message_len, 32);

        assert_eq!(
            phase_34_report.validator_id,
            CANONICAL_PAYLOAD_HASH_VALIDATOR_PHASE_34
        );
        assert!(phase_34_report.recomputes_hash_from_payload_bytes);
        assert!(!phase_34_report.caller_provided_payload_hash_trusted);

        // The Phase 34 canonical payload hash is a 32-byte value, matching the
        // modeled ed25519 message length.
        assert_eq!(
            XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1.len(),
            EXPECTED_MESSAGE_LEN
        );
        assert_eq!(
            phase_34_report.phase_23_valid_payload_hash.len(),
            EXPECTED_MESSAGE_LEN
        );
    }

    #[test]
    fn phase_35_structural_quorum_remains_a_separate_required_check() {
        let report = ed25519_evidence_layout_model_report();

        assert_eq!(
            report.phase_35_structural_quorum_phase,
            GUARDIAN_QUORUM_STRUCTURAL_VERIFIER_PHASE_35
        );
        assert!(report.phase_35_structural_quorum_still_required);
    }
}
