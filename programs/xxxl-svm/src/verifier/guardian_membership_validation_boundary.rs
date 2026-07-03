use super::checked_ed25519_byte_extraction_boundary::{
    Phase41F_1CheckedByteExtractionResult, Phase41F_1CheckedByteExtractionStatus,
};
use super::ed25519_signature_verification_boundary::{
    Phase41F_2Ed25519SignatureVerificationResult, Phase41F_2Ed25519SignatureVerificationStatus,
};
use super::guardian_quorum::GuardianPublicKey;
use super::payload_hash_binding_boundary::{
    establish_payload_hash_binding, PayloadHashBindingError,
};
use super::raw_payload::{decode_guardian_payload_raw, RawPayloadDecodeError};

pub const GUARDIAN_MEMBERSHIP_VALIDATION_BOUNDARY_PHASE_41H: &str = "41H";
pub const GUARDIAN_MEMBERSHIP_VALIDATION_BOUNDARY_VERSION: &str = "0.1.0";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthoritativeGuardianSetSource {
    ProgramControlledOnChain,
    CallerInstructionData,
    Unauthenticated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthoritativeGuardianSetRef<'a> {
    guardian_set_id: &'a [u8; 32],
    threshold: u8,
    guardians: &'a [GuardianPublicKey],
    source: AuthoritativeGuardianSetSource,
    caller_instruction_data: bool,
}

impl<'a> AuthoritativeGuardianSetRef<'a> {
    fn from_program_controlled_on_chain_source(
        guardian_set_id: &'a [u8; 32],
        threshold: u8,
        guardians: &'a [GuardianPublicKey],
    ) -> Self {
        Self {
            guardian_set_id,
            threshold,
            guardians,
            source: AuthoritativeGuardianSetSource::ProgramControlledOnChain,
            caller_instruction_data: false,
        }
    }

    pub fn caller_supplied_for_rejection(
        guardian_set_id: &'a [u8; 32],
        threshold: u8,
        guardians: &'a [GuardianPublicKey],
    ) -> Self {
        Self {
            guardian_set_id,
            threshold,
            guardians,
            source: AuthoritativeGuardianSetSource::CallerInstructionData,
            caller_instruction_data: true,
        }
    }

    pub fn unauthenticated_for_rejection(
        guardian_set_id: &'a [u8; 32],
        threshold: u8,
        guardians: &'a [GuardianPublicKey],
    ) -> Self {
        Self {
            guardian_set_id,
            threshold,
            guardians,
            source: AuthoritativeGuardianSetSource::Unauthenticated,
            caller_instruction_data: false,
        }
    }

    pub fn guardian_set_id(&self) -> &'a [u8; 32] {
        self.guardian_set_id
    }

    pub fn threshold(&self) -> u8 {
        self.threshold
    }

    pub fn guardians(&self) -> &'a [GuardianPublicKey] {
        self.guardians
    }

    pub fn source(&self) -> AuthoritativeGuardianSetSource {
        self.source
    }

    pub fn caller_instruction_data(&self) -> bool {
        self.caller_instruction_data
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuardianMembershipValidationStatus {
    GuardianMembershipValidated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GuardianMembershipValidated {
    pub status: GuardianMembershipValidationStatus,
    pub guardian_set_id: [u8; 32],
    pub matched_guardian_public_key: GuardianPublicKey,
    pub matched_guardian_index: usize,
    pub guardian_membership_validated: bool,
    pub cryptographic_signature_proof_accepted: bool,
    pub verification_evidence_accepted: bool,
    pub quorum_counting_enabled: bool,
    pub authorization_enabled: bool,
    pub replay_write_enabled: bool,
    pub processed_event_marking_enabled: bool,
    pub account_mutation_enabled: bool,
    pub cpi_enabled: bool,
    pub invoke_signed_enabled: bool,
    pub spl_token_mint_to_enabled: bool,
    pub process_instruction_handler_added: bool,
    pub live_route_enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuardianMembershipValidationErrorKind {
    Phase41FNotEstablished,
    Phase41FExtractionNotEstablished,
    VerifiedSignerPublicKeyMissing,
    VerifiedSignerRangeMissing,
    VerifiedSignerRangeMismatch,
    VerifiedMessageRangeMismatch,
    VerifiedSignatureRangeMismatch,
    ExtractedSignedMessageLengthInvalid,
    MatchedInstructionIndexMismatch,
    InstructionDataLengthMismatch,
    PayloadHashBindingNotEstablished,
    RawPayloadDecodeFailed,
    UnauthenticatedGuardianSet,
    CallerSuppliedGuardianSetRejected,
    EmptyGuardianSet,
    InvalidThresholdZero,
    ThresholdExceedsGuardianSet,
    DuplicateGuardianPublicKey,
    GuardianSetIdMismatch,
    PayloadGuardianSetIdMismatch,
    VerifiedSignerNotGuardian,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GuardianMembershipValidationError {
    pub kind: GuardianMembershipValidationErrorKind,
    pub phase_41f_status: Phase41F_2Ed25519SignatureVerificationStatus,
    pub phase_41f_establishes_native_ed25519_verification: bool,
    pub phase_41f_1_extraction_status: Phase41F_1CheckedByteExtractionStatus,
    pub guardian_set_source: AuthoritativeGuardianSetSource,
    pub guardian_set_from_caller_instruction_data: bool,
    pub guardian_count: usize,
    pub threshold: u8,
    pub payload_hash_binding_error: Option<PayloadHashBindingError>,
    pub raw_payload_decode_error: Option<RawPayloadDecodeError>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GuardianMembershipValidationBoundaryReport {
    pub phase: &'static str,
    pub version: &'static str,
    pub validates_verified_signer_membership_only: bool,
    pub requires_phase_41f_native_ed25519_verification: bool,
    pub requires_phase_41f_1_checked_extraction: bool,
    pub requires_phase_41f_2_verified_ranges: bool,
    pub checks_public_key_range_binding: bool,
    pub checks_message_range_binding: bool,
    pub checks_signature_range_binding: bool,
    pub derives_signed_message_from_41f_extraction: bool,
    pub accepts_free_signed_message_input: bool,
    pub checks_extracted_signed_message_len_32: bool,
    pub binds_41f_verified_message_to_payload_hash: bool,
    pub checks_matched_instruction_index_binding: bool,
    pub checks_instruction_data_length_binding: bool,
    pub requires_phase_41g_payload_hash_binding: bool,
    pub recomputes_phase_41g_payload_hash_binding_from_raw_payload: bool,
    pub decodes_raw_payload_internally: bool,
    pub accepts_free_decoded_payload_input: bool,
    pub requires_authoritative_guardian_set_wrapper: bool,
    pub rejects_caller_supplied_guardian_set: bool,
    pub rejects_unauthenticated_guardian_set: bool,
    pub checks_payload_guardian_set_id_binding: bool,
    pub rejects_empty_guardian_set: bool,
    pub rejects_invalid_threshold: bool,
    pub rejects_duplicate_guardian_public_key: bool,
    pub rejects_verified_signer_not_guardian: bool,
    pub wrapper_constructor_publicly_unrestricted: bool,
    pub account_info_used: bool,
    pub runtime_account_loading_enabled: bool,
    pub sysvar_loading_enabled: bool,
    pub cryptographic_signature_proof_accepted: bool,
    pub verification_evidence_accepted: bool,
    pub quorum_counting_enabled: bool,
    pub authorization_enabled: bool,
    pub replay_write_enabled: bool,
    pub processed_event_marking_enabled: bool,
    pub account_mutation_enabled: bool,
    pub cpi_enabled: bool,
    pub invoke_signed_enabled: bool,
    pub spl_token_mint_to_enabled: bool,
    pub process_instruction_handler_added: bool,
    pub live_route_enabled: bool,
}

pub const GUARDIAN_MEMBERSHIP_VALIDATION_BOUNDARY_REPORT:
    GuardianMembershipValidationBoundaryReport = GuardianMembershipValidationBoundaryReport {
    phase: GUARDIAN_MEMBERSHIP_VALIDATION_BOUNDARY_PHASE_41H,
    version: GUARDIAN_MEMBERSHIP_VALIDATION_BOUNDARY_VERSION,
    validates_verified_signer_membership_only: true,
    requires_phase_41f_native_ed25519_verification: true,
    requires_phase_41f_1_checked_extraction: true,
    requires_phase_41f_2_verified_ranges: true,
    checks_public_key_range_binding: true,
    checks_message_range_binding: true,
    checks_signature_range_binding: true,
    derives_signed_message_from_41f_extraction: true,
    accepts_free_signed_message_input: false,
    checks_extracted_signed_message_len_32: true,
    binds_41f_verified_message_to_payload_hash: true,
    checks_matched_instruction_index_binding: true,
    checks_instruction_data_length_binding: true,
    requires_phase_41g_payload_hash_binding: true,
    recomputes_phase_41g_payload_hash_binding_from_raw_payload: true,
    decodes_raw_payload_internally: true,
    accepts_free_decoded_payload_input: false,
    requires_authoritative_guardian_set_wrapper: true,
    rejects_caller_supplied_guardian_set: true,
    rejects_unauthenticated_guardian_set: true,
    checks_payload_guardian_set_id_binding: true,
    rejects_empty_guardian_set: true,
    rejects_invalid_threshold: true,
    rejects_duplicate_guardian_public_key: true,
    rejects_verified_signer_not_guardian: true,
    wrapper_constructor_publicly_unrestricted: false,
    account_info_used: false,
    runtime_account_loading_enabled: false,
    sysvar_loading_enabled: false,
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

pub fn guardian_membership_validation_boundary_report(
) -> &'static GuardianMembershipValidationBoundaryReport {
    &GUARDIAN_MEMBERSHIP_VALIDATION_BOUNDARY_REPORT
}

pub fn establish_guardian_membership_validation(
    phase_41f_result: &Phase41F_2Ed25519SignatureVerificationResult,
    extraction_result: &Phase41F_1CheckedByteExtractionResult<'_>,
    raw_payload_bytes: &[u8],
    expected_configured_guardian_set_id: &[u8; 32],
    guardian_set: AuthoritativeGuardianSetRef<'_>,
) -> Result<GuardianMembershipValidated, GuardianMembershipValidationError> {
    if phase_41f_result.status
        != Phase41F_2Ed25519SignatureVerificationStatus::NativeEd25519VerificationEstablished
        || !phase_41f_result.establishes_native_ed25519_verification
    {
        return Err(error(
            GuardianMembershipValidationErrorKind::Phase41FNotEstablished,
            phase_41f_result,
            extraction_result,
            guardian_set,
        ));
    }

    if extraction_result.status
        != Phase41F_1CheckedByteExtractionStatus::CheckedEd25519ByteSlicesExtracted
    {
        return Err(error(
            GuardianMembershipValidationErrorKind::Phase41FExtractionNotEstablished,
            phase_41f_result,
            extraction_result,
            guardian_set,
        ));
    }

    let Some(extracted_slices) = extraction_result.extracted_slices else {
        return Err(error(
            GuardianMembershipValidationErrorKind::VerifiedSignerPublicKeyMissing,
            phase_41f_result,
            extraction_result,
            guardian_set,
        ));
    };

    let Some(verified_ranges) = phase_41f_result.verified_ranges else {
        return Err(error(
            GuardianMembershipValidationErrorKind::VerifiedSignerRangeMissing,
            phase_41f_result,
            extraction_result,
            guardian_set,
        ));
    };

    if verified_ranges.public_key_range != extracted_slices.public_key_range {
        return Err(error(
            GuardianMembershipValidationErrorKind::VerifiedSignerRangeMismatch,
            phase_41f_result,
            extraction_result,
            guardian_set,
        ));
    }

    if verified_ranges.message_range != extracted_slices.message_range {
        return Err(error(
            GuardianMembershipValidationErrorKind::VerifiedMessageRangeMismatch,
            phase_41f_result,
            extraction_result,
            guardian_set,
        ));
    }

    if verified_ranges.signature_range != extracted_slices.signature_range {
        return Err(error(
            GuardianMembershipValidationErrorKind::VerifiedSignatureRangeMismatch,
            phase_41f_result,
            extraction_result,
            guardian_set,
        ));
    }

    if phase_41f_result.matched_instruction_index != extraction_result.matched_instruction_index {
        return Err(error(
            GuardianMembershipValidationErrorKind::MatchedInstructionIndexMismatch,
            phase_41f_result,
            extraction_result,
            guardian_set,
        ));
    }

    if phase_41f_result.instruction_data_len != extraction_result.instruction_data_len {
        return Err(error(
            GuardianMembershipValidationErrorKind::InstructionDataLengthMismatch,
            phase_41f_result,
            extraction_result,
            guardian_set,
        ));
    }

    if extracted_slices.message_bytes.len() != 32 {
        return Err(error(
            GuardianMembershipValidationErrorKind::ExtractedSignedMessageLengthInvalid,
            phase_41f_result,
            extraction_result,
            guardian_set,
        ));
    }

    establish_payload_hash_binding(
        raw_payload_bytes,
        extracted_slices.message_bytes,
        phase_41f_result,
    )
    .map_err(|payload_hash_binding_error| {
        error_with_payload_hash_binding_error(
            GuardianMembershipValidationErrorKind::PayloadHashBindingNotEstablished,
            phase_41f_result,
            extraction_result,
            guardian_set,
            payload_hash_binding_error,
        )
    })?;

    let decoded_payload =
        decode_guardian_payload_raw(raw_payload_bytes).map_err(|decode_error| {
            error_with_raw_payload_decode_error(
                GuardianMembershipValidationErrorKind::RawPayloadDecodeFailed,
                phase_41f_result,
                extraction_result,
                guardian_set,
                decode_error,
            )
        })?;

    if guardian_set.caller_instruction_data {
        return Err(error(
            GuardianMembershipValidationErrorKind::CallerSuppliedGuardianSetRejected,
            phase_41f_result,
            extraction_result,
            guardian_set,
        ));
    }

    if guardian_set.source != AuthoritativeGuardianSetSource::ProgramControlledOnChain {
        return Err(error(
            GuardianMembershipValidationErrorKind::UnauthenticatedGuardianSet,
            phase_41f_result,
            extraction_result,
            guardian_set,
        ));
    }

    if guardian_set.guardians.is_empty() {
        return Err(error(
            GuardianMembershipValidationErrorKind::EmptyGuardianSet,
            phase_41f_result,
            extraction_result,
            guardian_set,
        ));
    }

    if guardian_set.threshold == 0 {
        return Err(error(
            GuardianMembershipValidationErrorKind::InvalidThresholdZero,
            phase_41f_result,
            extraction_result,
            guardian_set,
        ));
    }

    if guardian_set.threshold as usize > guardian_set.guardians.len() {
        return Err(error(
            GuardianMembershipValidationErrorKind::ThresholdExceedsGuardianSet,
            phase_41f_result,
            extraction_result,
            guardian_set,
        ));
    }

    if duplicate_guardian_public_key_index(guardian_set.guardians).is_some() {
        return Err(error(
            GuardianMembershipValidationErrorKind::DuplicateGuardianPublicKey,
            phase_41f_result,
            extraction_result,
            guardian_set,
        ));
    }

    if guardian_set.guardian_set_id != expected_configured_guardian_set_id {
        return Err(error(
            GuardianMembershipValidationErrorKind::GuardianSetIdMismatch,
            phase_41f_result,
            extraction_result,
            guardian_set,
        ));
    }

    if decoded_payload.guardian_set_id != guardian_set.guardian_set_id {
        return Err(error(
            GuardianMembershipValidationErrorKind::PayloadGuardianSetIdMismatch,
            phase_41f_result,
            extraction_result,
            guardian_set,
        ));
    }

    let verified_signer_public_key = GuardianPublicKey(*extracted_slices.public_key_bytes);

    let Some(matched_guardian_index) = guardian_set
        .guardians
        .iter()
        .position(|guardian| *guardian == verified_signer_public_key)
    else {
        return Err(error(
            GuardianMembershipValidationErrorKind::VerifiedSignerNotGuardian,
            phase_41f_result,
            extraction_result,
            guardian_set,
        ));
    };

    Ok(GuardianMembershipValidated {
        status: GuardianMembershipValidationStatus::GuardianMembershipValidated,
        guardian_set_id: *guardian_set.guardian_set_id,
        matched_guardian_public_key: verified_signer_public_key,
        matched_guardian_index,
        guardian_membership_validated: true,
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
    })
}

fn duplicate_guardian_public_key_index(guardians: &[GuardianPublicKey]) -> Option<usize> {
    for (index, guardian) in guardians.iter().enumerate() {
        if guardians
            .get(..index)
            .map(|previous_guardians| {
                previous_guardians
                    .iter()
                    .any(|previous| previous == guardian)
            })
            .unwrap_or(false)
        {
            return Some(index);
        }
    }

    None
}

fn error(
    kind: GuardianMembershipValidationErrorKind,
    phase_41f_result: &Phase41F_2Ed25519SignatureVerificationResult,
    extraction_result: &Phase41F_1CheckedByteExtractionResult<'_>,
    guardian_set: AuthoritativeGuardianSetRef<'_>,
) -> GuardianMembershipValidationError {
    error_with_details(
        kind,
        phase_41f_result,
        extraction_result,
        guardian_set,
        None,
        None,
    )
}

fn error_with_payload_hash_binding_error(
    kind: GuardianMembershipValidationErrorKind,
    phase_41f_result: &Phase41F_2Ed25519SignatureVerificationResult,
    extraction_result: &Phase41F_1CheckedByteExtractionResult<'_>,
    guardian_set: AuthoritativeGuardianSetRef<'_>,
    payload_hash_binding_error: PayloadHashBindingError,
) -> GuardianMembershipValidationError {
    error_with_details(
        kind,
        phase_41f_result,
        extraction_result,
        guardian_set,
        Some(payload_hash_binding_error),
        None,
    )
}

fn error_with_raw_payload_decode_error(
    kind: GuardianMembershipValidationErrorKind,
    phase_41f_result: &Phase41F_2Ed25519SignatureVerificationResult,
    extraction_result: &Phase41F_1CheckedByteExtractionResult<'_>,
    guardian_set: AuthoritativeGuardianSetRef<'_>,
    raw_payload_decode_error: RawPayloadDecodeError,
) -> GuardianMembershipValidationError {
    error_with_details(
        kind,
        phase_41f_result,
        extraction_result,
        guardian_set,
        None,
        Some(raw_payload_decode_error),
    )
}

fn error_with_details(
    kind: GuardianMembershipValidationErrorKind,
    phase_41f_result: &Phase41F_2Ed25519SignatureVerificationResult,
    extraction_result: &Phase41F_1CheckedByteExtractionResult<'_>,
    guardian_set: AuthoritativeGuardianSetRef<'_>,
    payload_hash_binding_error: Option<PayloadHashBindingError>,
    raw_payload_decode_error: Option<RawPayloadDecodeError>,
) -> GuardianMembershipValidationError {
    GuardianMembershipValidationError {
        kind,
        phase_41f_status: phase_41f_result.status,
        phase_41f_establishes_native_ed25519_verification: phase_41f_result
            .establishes_native_ed25519_verification,
        phase_41f_1_extraction_status: extraction_result.status,
        guardian_set_source: guardian_set.source,
        guardian_set_from_caller_instruction_data: guardian_set.caller_instruction_data,
        guardian_count: guardian_set.guardians.len(),
        threshold: guardian_set.threshold,
        payload_hash_binding_error,
        raw_payload_decode_error,
    }
}

#[cfg(test)]
mod tests {
    use super::super::canonical_payload::{
        compute_guardian_payload_hash, XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1,
    };
    use super::super::checked_ed25519_byte_extraction_boundary::{
        Phase41F_1CheckedByteExtractionStatus, Phase41F_1ExtractedEd25519ByteSlices,
    };
    use super::super::ed25519_instruction_byte_parsing_boundary::Phase41E_1ByteRange;
    use super::super::ed25519_signature_verification_boundary::{
        Phase41F_2Ed25519VerificationModel, Phase41F_2VerifiedEd25519SignatureRanges,
    };
    use super::*;

    const GUARDIAN_SET_ID: [u8; 32] = [0x22; 32];
    const OTHER_GUARDIAN_SET_ID: [u8; 32] = [0x23; 32];
    const ROUTE_ID: [u8; 32] = [0x11; 32];
    const CANONICAL_EVENT_KEY: [u8; 32] = [0x44; 32];
    const X1_RECIPIENT: [u8; 32] = [0x55; 32];
    const TARGET_MINT: [u8; 32] = [0x33; 32];
    const MESSAGE_NONCE: [u8; 32] = [0x66; 32];
    const SOURCE_TOKEN: [u8; 6] = [1, 2, 3, 4, 5, 6];
    const SOURCE_SENDER: [u8; 20] = [0xaa; 20];
    const SOURCE_BURN_TX_HASH: [u8; 32] = [0xaa; 32];
    const SOURCE_BLOCK_HASH: [u8; 32] = [0xbb; 32];

    const SHORT_MESSAGE_BYTES: [u8; 31] = [0x07; 31];

    fn write_u16_le(out: &mut Vec<u8>, value: u16) {
        out.extend_from_slice(&value.to_le_bytes());
    }

    fn write_u64_le(out: &mut Vec<u8>, value: u64) {
        out.extend_from_slice(&value.to_le_bytes());
    }

    fn write_u128_le(out: &mut Vec<u8>, value: u128) {
        out.extend_from_slice(&value.to_le_bytes());
    }

    fn write_var(out: &mut Vec<u8>, bytes: &[u8]) {
        write_u16_le(out, bytes.len() as u16);
        out.extend_from_slice(bytes);
    }

    fn raw_payload_bytes_with_guardian_set_id(guardian_set_id: &[u8; 32]) -> Vec<u8> {
        let mut out = Vec::new();

        write_var(&mut out, b"XXXL_GATEWAY_MINT");
        write_u16_le(&mut out, 1);
        write_u16_le(&mut out, 2);
        out.extend_from_slice(&ROUTE_ID);
        write_u64_le(&mut out, 1);
        write_var(&mut out, &SOURCE_TOKEN);
        write_var(&mut out, &SOURCE_SENDER);
        write_var(&mut out, &SOURCE_BURN_TX_HASH);
        write_u64_le(&mut out, 7);
        write_u64_le(&mut out, 123_456_789);
        write_var(&mut out, &SOURCE_BLOCK_HASH);
        write_u64_le(&mut out, 123_456_900);
        out.extend_from_slice(&CANONICAL_EVENT_KEY);
        out.extend_from_slice(&X1_RECIPIENT);
        write_u128_le(&mut out, 1_000_000_000_000);
        write_u16_le(&mut out, 10_000);
        write_u128_le(&mut out, 1_000_000_000_000);
        out.extend_from_slice(&TARGET_MINT);
        out.extend_from_slice(guardian_set_id);
        out.extend_from_slice(&MESSAGE_NONCE);
        write_u64_le(&mut out, 987_654_321);

        out
    }

    const SIGNATURE_BYTES: [u8; 64] = [0x11; 64];
    const PUBLIC_KEY_BYTES: [u8; 32] = [0x02; 32];
    const OTHER_PUBLIC_KEY_BYTES: [u8; 32] = [0x09; 32];
    const ARBITRARY_MESSAGE_BYTES: [u8; 32] = [0x03; 32];

    fn guardian(byte: u8) -> GuardianPublicKey {
        GuardianPublicKey([byte; 32])
    }

    fn public_key_range() -> Phase41E_1ByteRange {
        Phase41E_1ByteRange {
            offset: 80,
            len: 32,
        }
    }

    fn other_public_key_range() -> Phase41E_1ByteRange {
        Phase41E_1ByteRange {
            offset: 81,
            len: 32,
        }
    }

    fn signature_range() -> Phase41E_1ByteRange {
        Phase41E_1ByteRange {
            offset: 16,
            len: 64,
        }
    }

    fn message_range() -> Phase41E_1ByteRange {
        Phase41E_1ByteRange {
            offset: 112,
            len: 32,
        }
    }

    fn other_message_range() -> Phase41E_1ByteRange {
        Phase41E_1ByteRange {
            offset: 113,
            len: 32,
        }
    }

    fn other_signature_range() -> Phase41E_1ByteRange {
        Phase41E_1ByteRange {
            offset: 17,
            len: 64,
        }
    }

    fn short_message_range() -> Phase41E_1ByteRange {
        Phase41E_1ByteRange {
            offset: 112,
            len: 31,
        }
    }

    fn verified_ranges(
        public_key_range: Phase41E_1ByteRange,
    ) -> Phase41F_2VerifiedEd25519SignatureRanges {
        verified_ranges_full(signature_range(), public_key_range, message_range())
    }

    fn verified_ranges_full(
        signature_range: Phase41E_1ByteRange,
        public_key_range: Phase41E_1ByteRange,
        message_range: Phase41E_1ByteRange,
    ) -> Phase41F_2VerifiedEd25519SignatureRanges {
        Phase41F_2VerifiedEd25519SignatureRanges {
            signature_range,
            public_key_range,
            message_range,
        }
    }

    fn established_phase_41f_result() -> Phase41F_2Ed25519SignatureVerificationResult {
        phase_41f_result(
            Phase41F_2Ed25519SignatureVerificationStatus::NativeEd25519VerificationEstablished,
            true,
            Some(verified_ranges(public_key_range())),
            Some(1),
            144,
        )
    }

    fn phase_41f_result(
        status: Phase41F_2Ed25519SignatureVerificationStatus,
        establishes_native_ed25519_verification: bool,
        verified_ranges: Option<Phase41F_2VerifiedEd25519SignatureRanges>,
        matched_instruction_index: Option<usize>,
        instruction_data_len: usize,
    ) -> Phase41F_2Ed25519SignatureVerificationResult {
        let established = status
            == Phase41F_2Ed25519SignatureVerificationStatus::NativeEd25519VerificationEstablished
            && establishes_native_ed25519_verification;

        Phase41F_2Ed25519SignatureVerificationResult {
            status,
            rejection_case: None,
            verification_model: established
                .then_some(Phase41F_2Ed25519VerificationModel::NativeEd25519Instruction),
            matched_instruction_index,
            instruction_data_len,
            verified_ranges,
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
            establishes_native_ed25519_verification,
            performs_local_cryptographic_verification: false,
            ed25519_signature_verification_performed: established,
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

    fn established_extraction_result<'a>(
        public_key_bytes: &'a [u8; 32],
    ) -> Phase41F_1CheckedByteExtractionResult<'a> {
        extraction_result(
            Phase41F_1CheckedByteExtractionStatus::CheckedEd25519ByteSlicesExtracted,
            Some(Phase41F_1ExtractedEd25519ByteSlices {
                signature_bytes: &SIGNATURE_BYTES,
                public_key_bytes,
                message_bytes: &XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1,
                signature_range: signature_range(),
                public_key_range: public_key_range(),
                message_range: message_range(),
            }),
            Some(1),
            144,
        )
    }

    fn established_extraction_result_with_ranges<'a>(
        public_key_bytes: &'a [u8; 32],
        message_bytes: &'a [u8],
        signature_range: Phase41E_1ByteRange,
        message_range: Phase41E_1ByteRange,
    ) -> Phase41F_1CheckedByteExtractionResult<'a> {
        extraction_result(
            Phase41F_1CheckedByteExtractionStatus::CheckedEd25519ByteSlicesExtracted,
            Some(Phase41F_1ExtractedEd25519ByteSlices {
                signature_bytes: &SIGNATURE_BYTES,
                public_key_bytes,
                message_bytes,
                signature_range,
                public_key_range: public_key_range(),
                message_range,
            }),
            Some(1),
            144,
        )
    }

    fn extraction_result<'a>(
        status: Phase41F_1CheckedByteExtractionStatus,
        extracted_slices: Option<Phase41F_1ExtractedEd25519ByteSlices<'a>>,
        matched_instruction_index: Option<usize>,
        instruction_data_len: usize,
    ) -> Phase41F_1CheckedByteExtractionResult<'a> {
        Phase41F_1CheckedByteExtractionResult {
            status,
            rejection_case: None,
            matched_instruction_index,
            instruction_data_len,
            extracted_slices,
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

    fn authoritative_set<'a>(
        guardian_set_id: &'a [u8; 32],
        threshold: u8,
        guardians: &'a [GuardianPublicKey],
    ) -> AuthoritativeGuardianSetRef<'a> {
        AuthoritativeGuardianSetRef::from_program_controlled_on_chain_source(
            guardian_set_id,
            threshold,
            guardians,
        )
    }

    fn validate_with<'a>(
        phase_41f: &'a Phase41F_2Ed25519SignatureVerificationResult,
        extraction: &'a Phase41F_1CheckedByteExtractionResult<'a>,
        raw_payload_bytes: &'a [u8],
        expected_set_id: &'a [u8; 32],
        guardian_set: AuthoritativeGuardianSetRef<'a>,
    ) -> Result<GuardianMembershipValidated, GuardianMembershipValidationError> {
        establish_guardian_membership_validation(
            phase_41f,
            extraction,
            raw_payload_bytes,
            expected_set_id,
            guardian_set,
        )
    }

    fn assert_error_kind(
        result: Result<GuardianMembershipValidated, GuardianMembershipValidationError>,
        expected: GuardianMembershipValidationErrorKind,
    ) -> GuardianMembershipValidationError {
        let err = result.expect_err("guardian membership validation should fail");
        assert_eq!(err.kind, expected);
        err
    }

    #[test]
    fn valid_verified_signer_is_accepted_as_guardian_member() {
        let guardians = [guardian(1), guardian(2), guardian(3)];
        let phase_41f = established_phase_41f_result();
        let extraction = established_extraction_result(&PUBLIC_KEY_BYTES);
        let raw_payload = raw_payload_bytes_with_guardian_set_id(&GUARDIAN_SET_ID);

        let result = validate_with(
            &phase_41f,
            &extraction,
            &raw_payload,
            &GUARDIAN_SET_ID,
            authoritative_set(&GUARDIAN_SET_ID, 2, &guardians),
        )
        .expect("valid guardian membership");

        assert_eq!(
            result.status,
            GuardianMembershipValidationStatus::GuardianMembershipValidated
        );
        assert_eq!(result.guardian_set_id, GUARDIAN_SET_ID);
        assert_eq!(result.matched_guardian_public_key, guardian(2));
        assert_eq!(result.matched_guardian_index, 1);
        assert!(result.guardian_membership_validated);
    }

    #[test]
    fn rejects_phase_41f_not_established() {
        let guardians = [guardian(2)];
        let phase_41f = phase_41f_result(
            Phase41F_2Ed25519SignatureVerificationStatus::WrongEd25519ProgramId,
            false,
            None,
            None,
            0,
        );
        let extraction = established_extraction_result(&PUBLIC_KEY_BYTES);
        let raw_payload = raw_payload_bytes_with_guardian_set_id(&GUARDIAN_SET_ID);

        assert_error_kind(
            validate_with(
                &phase_41f,
                &extraction,
                &raw_payload,
                &GUARDIAN_SET_ID,
                authoritative_set(&GUARDIAN_SET_ID, 1, &guardians),
            ),
            GuardianMembershipValidationErrorKind::Phase41FNotEstablished,
        );
    }

    #[test]
    fn rejects_phase_41f_established_status_with_false_flag() {
        let guardians = [guardian(2)];
        let phase_41f = phase_41f_result(
            Phase41F_2Ed25519SignatureVerificationStatus::NativeEd25519VerificationEstablished,
            false,
            Some(verified_ranges(public_key_range())),
            Some(1),
            144,
        );
        let extraction = established_extraction_result(&PUBLIC_KEY_BYTES);
        let raw_payload = raw_payload_bytes_with_guardian_set_id(&GUARDIAN_SET_ID);

        assert_error_kind(
            validate_with(
                &phase_41f,
                &extraction,
                &raw_payload,
                &GUARDIAN_SET_ID,
                authoritative_set(&GUARDIAN_SET_ID, 1, &guardians),
            ),
            GuardianMembershipValidationErrorKind::Phase41FNotEstablished,
        );
    }

    #[test]
    fn rejects_phase_41f_1_extraction_not_established() {
        let guardians = [guardian(2)];
        let phase_41f = established_phase_41f_result();
        let extraction = extraction_result(
            Phase41F_1CheckedByteExtractionStatus::CheckedPublicKeySliceUnavailable,
            None,
            Some(1),
            144,
        );
        let raw_payload = raw_payload_bytes_with_guardian_set_id(&GUARDIAN_SET_ID);

        assert_error_kind(
            validate_with(
                &phase_41f,
                &extraction,
                &raw_payload,
                &GUARDIAN_SET_ID,
                authoritative_set(&GUARDIAN_SET_ID, 1, &guardians),
            ),
            GuardianMembershipValidationErrorKind::Phase41FExtractionNotEstablished,
        );
    }

    #[test]
    fn rejects_missing_extracted_signer_public_key() {
        let guardians = [guardian(2)];
        let phase_41f = established_phase_41f_result();
        let extraction = extraction_result(
            Phase41F_1CheckedByteExtractionStatus::CheckedEd25519ByteSlicesExtracted,
            None,
            Some(1),
            144,
        );
        let raw_payload = raw_payload_bytes_with_guardian_set_id(&GUARDIAN_SET_ID);

        assert_error_kind(
            validate_with(
                &phase_41f,
                &extraction,
                &raw_payload,
                &GUARDIAN_SET_ID,
                authoritative_set(&GUARDIAN_SET_ID, 1, &guardians),
            ),
            GuardianMembershipValidationErrorKind::VerifiedSignerPublicKeyMissing,
        );
    }

    #[test]
    fn rejects_missing_verified_ranges() {
        let guardians = [guardian(2)];
        let phase_41f = phase_41f_result(
            Phase41F_2Ed25519SignatureVerificationStatus::NativeEd25519VerificationEstablished,
            true,
            None,
            Some(1),
            144,
        );
        let extraction = established_extraction_result(&PUBLIC_KEY_BYTES);
        let raw_payload = raw_payload_bytes_with_guardian_set_id(&GUARDIAN_SET_ID);

        assert_error_kind(
            validate_with(
                &phase_41f,
                &extraction,
                &raw_payload,
                &GUARDIAN_SET_ID,
                authoritative_set(&GUARDIAN_SET_ID, 1, &guardians),
            ),
            GuardianMembershipValidationErrorKind::VerifiedSignerRangeMissing,
        );
    }

    #[test]
    fn rejects_public_key_range_mismatch() {
        let guardians = [guardian(2)];
        let phase_41f = phase_41f_result(
            Phase41F_2Ed25519SignatureVerificationStatus::NativeEd25519VerificationEstablished,
            true,
            Some(verified_ranges(other_public_key_range())),
            Some(1),
            144,
        );
        let extraction = established_extraction_result(&PUBLIC_KEY_BYTES);
        let raw_payload = raw_payload_bytes_with_guardian_set_id(&GUARDIAN_SET_ID);

        assert_error_kind(
            validate_with(
                &phase_41f,
                &extraction,
                &raw_payload,
                &GUARDIAN_SET_ID,
                authoritative_set(&GUARDIAN_SET_ID, 1, &guardians),
            ),
            GuardianMembershipValidationErrorKind::VerifiedSignerRangeMismatch,
        );
    }

    #[test]
    fn rejects_message_range_mismatch() {
        let guardians = [guardian(2)];
        let phase_41f = phase_41f_result(
            Phase41F_2Ed25519SignatureVerificationStatus::NativeEd25519VerificationEstablished,
            true,
            Some(verified_ranges_full(
                signature_range(),
                public_key_range(),
                other_message_range(),
            )),
            Some(1),
            144,
        );
        let extraction = established_extraction_result(&PUBLIC_KEY_BYTES);
        let raw_payload = raw_payload_bytes_with_guardian_set_id(&GUARDIAN_SET_ID);

        assert_error_kind(
            validate_with(
                &phase_41f,
                &extraction,
                &raw_payload,
                &GUARDIAN_SET_ID,
                authoritative_set(&GUARDIAN_SET_ID, 1, &guardians),
            ),
            GuardianMembershipValidationErrorKind::VerifiedMessageRangeMismatch,
        );
    }

    #[test]
    fn rejects_signature_range_mismatch() {
        let guardians = [guardian(2)];
        let phase_41f = phase_41f_result(
            Phase41F_2Ed25519SignatureVerificationStatus::NativeEd25519VerificationEstablished,
            true,
            Some(verified_ranges_full(
                other_signature_range(),
                public_key_range(),
                message_range(),
            )),
            Some(1),
            144,
        );
        let extraction = established_extraction_result(&PUBLIC_KEY_BYTES);
        let raw_payload = raw_payload_bytes_with_guardian_set_id(&GUARDIAN_SET_ID);

        assert_error_kind(
            validate_with(
                &phase_41f,
                &extraction,
                &raw_payload,
                &GUARDIAN_SET_ID,
                authoritative_set(&GUARDIAN_SET_ID, 1, &guardians),
            ),
            GuardianMembershipValidationErrorKind::VerifiedSignatureRangeMismatch,
        );
    }

    #[test]
    fn rejects_arbitrary_message_range_pairing_attack() {
        let guardians = [guardian(2)];
        let phase_41f = phase_41f_result(
            Phase41F_2Ed25519SignatureVerificationStatus::NativeEd25519VerificationEstablished,
            true,
            Some(verified_ranges_full(
                signature_range(),
                public_key_range(),
                other_message_range(),
            )),
            Some(1),
            144,
        );
        let extraction = established_extraction_result_with_ranges(
            &PUBLIC_KEY_BYTES,
            &XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1,
            signature_range(),
            message_range(),
        );
        let raw_payload = raw_payload_bytes_with_guardian_set_id(&GUARDIAN_SET_ID);

        assert_error_kind(
            validate_with(
                &phase_41f,
                &extraction,
                &raw_payload,
                &GUARDIAN_SET_ID,
                authoritative_set(&GUARDIAN_SET_ID, 1, &guardians),
            ),
            GuardianMembershipValidationErrorKind::VerifiedMessageRangeMismatch,
        );
    }

    #[test]
    fn rejects_extracted_signed_message_length_not_32() {
        let guardians = [guardian(2)];
        let phase_41f = phase_41f_result(
            Phase41F_2Ed25519SignatureVerificationStatus::NativeEd25519VerificationEstablished,
            true,
            Some(verified_ranges_full(
                signature_range(),
                public_key_range(),
                short_message_range(),
            )),
            Some(1),
            144,
        );
        let extraction = established_extraction_result_with_ranges(
            &PUBLIC_KEY_BYTES,
            &SHORT_MESSAGE_BYTES,
            signature_range(),
            short_message_range(),
        );
        let raw_payload = raw_payload_bytes_with_guardian_set_id(&GUARDIAN_SET_ID);

        assert_error_kind(
            validate_with(
                &phase_41f,
                &extraction,
                &raw_payload,
                &GUARDIAN_SET_ID,
                authoritative_set(&GUARDIAN_SET_ID, 1, &guardians),
            ),
            GuardianMembershipValidationErrorKind::ExtractedSignedMessageLengthInvalid,
        );
    }

    #[test]
    fn rejects_matched_instruction_index_mismatch() {
        let guardians = [guardian(2)];
        let phase_41f = phase_41f_result(
            Phase41F_2Ed25519SignatureVerificationStatus::NativeEd25519VerificationEstablished,
            true,
            Some(verified_ranges(public_key_range())),
            Some(2),
            144,
        );
        let extraction = established_extraction_result(&PUBLIC_KEY_BYTES);
        let raw_payload = raw_payload_bytes_with_guardian_set_id(&GUARDIAN_SET_ID);

        assert_error_kind(
            validate_with(
                &phase_41f,
                &extraction,
                &raw_payload,
                &GUARDIAN_SET_ID,
                authoritative_set(&GUARDIAN_SET_ID, 1, &guardians),
            ),
            GuardianMembershipValidationErrorKind::MatchedInstructionIndexMismatch,
        );
    }

    #[test]
    fn rejects_instruction_data_length_mismatch() {
        let guardians = [guardian(2)];
        let phase_41f = phase_41f_result(
            Phase41F_2Ed25519SignatureVerificationStatus::NativeEd25519VerificationEstablished,
            true,
            Some(verified_ranges(public_key_range())),
            Some(1),
            145,
        );
        let extraction = established_extraction_result(&PUBLIC_KEY_BYTES);
        let raw_payload = raw_payload_bytes_with_guardian_set_id(&GUARDIAN_SET_ID);

        assert_error_kind(
            validate_with(
                &phase_41f,
                &extraction,
                &raw_payload,
                &GUARDIAN_SET_ID,
                authoritative_set(&GUARDIAN_SET_ID, 1, &guardians),
            ),
            GuardianMembershipValidationErrorKind::InstructionDataLengthMismatch,
        );
    }

    #[test]
    fn rejects_payload_hash_binding_failure_for_extracted_message_bytes() {
        let guardians = [guardian(2)];
        let phase_41f = established_phase_41f_result();
        let extraction = established_extraction_result_with_ranges(
            &PUBLIC_KEY_BYTES,
            &ARBITRARY_MESSAGE_BYTES,
            signature_range(),
            message_range(),
        );
        let raw_payload = raw_payload_bytes_with_guardian_set_id(&GUARDIAN_SET_ID);

        assert_error_kind(
            validate_with(
                &phase_41f,
                &extraction,
                &raw_payload,
                &GUARDIAN_SET_ID,
                authoritative_set(&GUARDIAN_SET_ID, 1, &guardians),
            ),
            GuardianMembershipValidationErrorKind::PayloadHashBindingNotEstablished,
        );
    }

    #[test]
    fn rejects_unauthenticated_guardian_set() {
        let guardians = [guardian(2)];
        let phase_41f = established_phase_41f_result();
        let extraction = established_extraction_result(&PUBLIC_KEY_BYTES);
        let raw_payload = raw_payload_bytes_with_guardian_set_id(&GUARDIAN_SET_ID);

        assert_error_kind(
            validate_with(
                &phase_41f,
                &extraction,
                &raw_payload,
                &GUARDIAN_SET_ID,
                AuthoritativeGuardianSetRef::unauthenticated_for_rejection(
                    &GUARDIAN_SET_ID,
                    1,
                    &guardians,
                ),
            ),
            GuardianMembershipValidationErrorKind::UnauthenticatedGuardianSet,
        );
    }

    #[test]
    fn rejects_caller_supplied_guardian_set() {
        let guardians = [guardian(2)];
        let phase_41f = established_phase_41f_result();
        let extraction = established_extraction_result(&PUBLIC_KEY_BYTES);
        let raw_payload = raw_payload_bytes_with_guardian_set_id(&GUARDIAN_SET_ID);

        assert_error_kind(
            validate_with(
                &phase_41f,
                &extraction,
                &raw_payload,
                &GUARDIAN_SET_ID,
                AuthoritativeGuardianSetRef::caller_supplied_for_rejection(
                    &GUARDIAN_SET_ID,
                    1,
                    &guardians,
                ),
            ),
            GuardianMembershipValidationErrorKind::CallerSuppliedGuardianSetRejected,
        );
    }

    #[test]
    fn rejects_empty_guardian_set() {
        let guardians = [];
        let phase_41f = established_phase_41f_result();
        let extraction = established_extraction_result(&PUBLIC_KEY_BYTES);
        let raw_payload = raw_payload_bytes_with_guardian_set_id(&GUARDIAN_SET_ID);

        assert_error_kind(
            validate_with(
                &phase_41f,
                &extraction,
                &raw_payload,
                &GUARDIAN_SET_ID,
                authoritative_set(&GUARDIAN_SET_ID, 1, &guardians),
            ),
            GuardianMembershipValidationErrorKind::EmptyGuardianSet,
        );
    }

    #[test]
    fn rejects_threshold_zero() {
        let guardians = [guardian(2)];
        let phase_41f = established_phase_41f_result();
        let extraction = established_extraction_result(&PUBLIC_KEY_BYTES);
        let raw_payload = raw_payload_bytes_with_guardian_set_id(&GUARDIAN_SET_ID);

        assert_error_kind(
            validate_with(
                &phase_41f,
                &extraction,
                &raw_payload,
                &GUARDIAN_SET_ID,
                authoritative_set(&GUARDIAN_SET_ID, 0, &guardians),
            ),
            GuardianMembershipValidationErrorKind::InvalidThresholdZero,
        );
    }

    #[test]
    fn rejects_threshold_greater_than_guardian_count() {
        let guardians = [guardian(2)];
        let phase_41f = established_phase_41f_result();
        let extraction = established_extraction_result(&PUBLIC_KEY_BYTES);
        let raw_payload = raw_payload_bytes_with_guardian_set_id(&GUARDIAN_SET_ID);

        assert_error_kind(
            validate_with(
                &phase_41f,
                &extraction,
                &raw_payload,
                &GUARDIAN_SET_ID,
                authoritative_set(&GUARDIAN_SET_ID, 2, &guardians),
            ),
            GuardianMembershipValidationErrorKind::ThresholdExceedsGuardianSet,
        );
    }

    #[test]
    fn rejects_duplicate_guardian_public_key() {
        let guardians = [guardian(2), guardian(2)];
        let phase_41f = established_phase_41f_result();
        let extraction = established_extraction_result(&PUBLIC_KEY_BYTES);
        let raw_payload = raw_payload_bytes_with_guardian_set_id(&GUARDIAN_SET_ID);

        assert_error_kind(
            validate_with(
                &phase_41f,
                &extraction,
                &raw_payload,
                &GUARDIAN_SET_ID,
                authoritative_set(&GUARDIAN_SET_ID, 1, &guardians),
            ),
            GuardianMembershipValidationErrorKind::DuplicateGuardianPublicKey,
        );
    }

    #[test]
    fn rejects_configured_guardian_set_id_mismatch() {
        let guardians = [guardian(2)];
        let phase_41f = established_phase_41f_result();
        let extraction = established_extraction_result(&PUBLIC_KEY_BYTES);
        let raw_payload = raw_payload_bytes_with_guardian_set_id(&GUARDIAN_SET_ID);

        assert_error_kind(
            validate_with(
                &phase_41f,
                &extraction,
                &raw_payload,
                &OTHER_GUARDIAN_SET_ID,
                authoritative_set(&GUARDIAN_SET_ID, 1, &guardians),
            ),
            GuardianMembershipValidationErrorKind::GuardianSetIdMismatch,
        );
    }

    #[test]
    fn rejects_payload_guardian_set_id_mismatch() {
        let guardians = [guardian(2)];
        let phase_41f = established_phase_41f_result();
        let raw_payload = raw_payload_bytes_with_guardian_set_id(&OTHER_GUARDIAN_SET_ID);
        let extracted_message_bytes =
            compute_guardian_payload_hash(&raw_payload).expect("hash for alternate valid payload");
        let extraction = established_extraction_result_with_ranges(
            &PUBLIC_KEY_BYTES,
            &extracted_message_bytes,
            signature_range(),
            message_range(),
        );

        assert_error_kind(
            validate_with(
                &phase_41f,
                &extraction,
                &raw_payload,
                &GUARDIAN_SET_ID,
                authoritative_set(&GUARDIAN_SET_ID, 1, &guardians),
            ),
            GuardianMembershipValidationErrorKind::PayloadGuardianSetIdMismatch,
        );
    }

    #[test]
    fn rejects_free_decoded_payload_substitution_by_hash_binding_raw_payload_bytes() {
        let guardians = [guardian(2)];
        let phase_41f = established_phase_41f_result();
        let extraction = established_extraction_result(&PUBLIC_KEY_BYTES);
        let raw_payload = raw_payload_bytes_with_guardian_set_id(&OTHER_GUARDIAN_SET_ID);

        assert_error_kind(
            validate_with(
                &phase_41f,
                &extraction,
                &raw_payload,
                &GUARDIAN_SET_ID,
                authoritative_set(&GUARDIAN_SET_ID, 1, &guardians),
            ),
            GuardianMembershipValidationErrorKind::PayloadHashBindingNotEstablished,
        );
    }

    #[test]
    fn rejects_verified_signer_not_in_guardian_set() {
        let guardians = [guardian(1), guardian(3)];
        let phase_41f = established_phase_41f_result();
        let extraction = established_extraction_result(&PUBLIC_KEY_BYTES);
        let raw_payload = raw_payload_bytes_with_guardian_set_id(&GUARDIAN_SET_ID);

        assert_error_kind(
            validate_with(
                &phase_41f,
                &extraction,
                &raw_payload,
                &GUARDIAN_SET_ID,
                authoritative_set(&GUARDIAN_SET_ID, 1, &guardians),
            ),
            GuardianMembershipValidationErrorKind::VerifiedSignerNotGuardian,
        );
    }

    #[test]
    fn accepts_only_verified_extracted_signer_not_arbitrary_claim_key() {
        let guardians = [GuardianPublicKey(OTHER_PUBLIC_KEY_BYTES)];
        let phase_41f = established_phase_41f_result();
        let extraction = established_extraction_result(&PUBLIC_KEY_BYTES);
        let raw_payload = raw_payload_bytes_with_guardian_set_id(&GUARDIAN_SET_ID);

        assert_error_kind(
            validate_with(
                &phase_41f,
                &extraction,
                &raw_payload,
                &GUARDIAN_SET_ID,
                authoritative_set(&GUARDIAN_SET_ID, 1, &guardians),
            ),
            GuardianMembershipValidationErrorKind::VerifiedSignerNotGuardian,
        );
    }

    #[test]
    fn success_keeps_downstream_execution_flags_false() {
        let guardians = [guardian(2)];
        let phase_41f = established_phase_41f_result();
        let extraction = established_extraction_result(&PUBLIC_KEY_BYTES);
        let raw_payload = raw_payload_bytes_with_guardian_set_id(&GUARDIAN_SET_ID);

        let result = validate_with(
            &phase_41f,
            &extraction,
            &raw_payload,
            &GUARDIAN_SET_ID,
            authoritative_set(&GUARDIAN_SET_ID, 1, &guardians),
        )
        .expect("valid guardian membership");

        assert!(!result.cryptographic_signature_proof_accepted);
        assert!(!result.verification_evidence_accepted);
        assert!(!result.quorum_counting_enabled);
        assert!(!result.authorization_enabled);
        assert!(!result.replay_write_enabled);
        assert!(!result.processed_event_marking_enabled);
        assert!(!result.account_mutation_enabled);
        assert!(!result.cpi_enabled);
        assert!(!result.invoke_signed_enabled);
        assert!(!result.spl_token_mint_to_enabled);
        assert!(!result.process_instruction_handler_added);
        assert!(!result.live_route_enabled);
    }

    #[test]
    fn report_preserves_boundary_and_forbidden_runtime_surfaces() {
        let report = guardian_membership_validation_boundary_report();

        assert_eq!(
            report.phase,
            GUARDIAN_MEMBERSHIP_VALIDATION_BOUNDARY_PHASE_41H
        );
        assert!(report.validates_verified_signer_membership_only);
        assert!(report.requires_phase_41f_native_ed25519_verification);
        assert!(report.requires_phase_41f_1_checked_extraction);
        assert!(report.requires_phase_41f_2_verified_ranges);
        assert!(report.checks_public_key_range_binding);
        assert!(report.checks_message_range_binding);
        assert!(report.checks_signature_range_binding);
        assert!(report.derives_signed_message_from_41f_extraction);
        assert!(!report.accepts_free_signed_message_input);
        assert!(report.checks_extracted_signed_message_len_32);
        assert!(report.binds_41f_verified_message_to_payload_hash);
        assert!(report.checks_matched_instruction_index_binding);
        assert!(report.checks_instruction_data_length_binding);
        assert!(report.requires_phase_41g_payload_hash_binding);
        assert!(report.recomputes_phase_41g_payload_hash_binding_from_raw_payload);
        assert!(report.decodes_raw_payload_internally);
        assert!(!report.accepts_free_decoded_payload_input);
        assert!(report.requires_authoritative_guardian_set_wrapper);
        assert!(report.rejects_caller_supplied_guardian_set);
        assert!(report.rejects_unauthenticated_guardian_set);
        assert!(report.checks_payload_guardian_set_id_binding);
        assert!(!report.wrapper_constructor_publicly_unrestricted);
        assert!(!report.account_info_used);
        assert!(!report.runtime_account_loading_enabled);
        assert!(!report.sysvar_loading_enabled);
        assert!(!report.cryptographic_signature_proof_accepted);
        assert!(!report.verification_evidence_accepted);
        assert!(!report.quorum_counting_enabled);
        assert!(!report.authorization_enabled);
        assert!(!report.replay_write_enabled);
        assert!(!report.processed_event_marking_enabled);
        assert!(!report.account_mutation_enabled);
        assert!(!report.cpi_enabled);
        assert!(!report.invoke_signed_enabled);
        assert!(!report.spl_token_mint_to_enabled);
        assert!(!report.process_instruction_handler_added);
        assert!(!report.live_route_enabled);
    }
}
