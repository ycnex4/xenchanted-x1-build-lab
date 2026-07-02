use core::convert::TryInto;

use super::canonical_payload::{
    validate_guardian_payload_hash, CanonicalPayloadHashValidationError,
};
use super::ed25519_signature_verification_boundary::{
    Phase41F_2Ed25519SignatureVerificationResult, Phase41F_2Ed25519SignatureVerificationStatus,
};

pub const PAYLOAD_HASH_BINDING_BOUNDARY_PHASE_41G_2: &str = "41G.2";
pub const PAYLOAD_HASH_BINDING_BOUNDARY_VERSION: &str = "0.1.0";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PayloadHashBindingStatus {
    PayloadHashBindingEstablished,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PayloadHashBindingEstablished {
    pub status: PayloadHashBindingStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PayloadHashBindingErrorKind {
    Phase41FNativeEd25519VerificationNotEstablished,
    SignedMessageLengthInvalid,
    SignedMessageHashConversionFailed,
    CanonicalPayloadHashValidationFailed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PayloadHashBindingError {
    pub kind: PayloadHashBindingErrorKind,
    pub phase_41f_status: Phase41F_2Ed25519SignatureVerificationStatus,
    pub establishes_native_ed25519_verification: bool,
    pub signed_message_len: usize,
    pub canonical_payload_hash_error: Option<CanonicalPayloadHashValidationError>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PayloadHashBindingBoundaryReport {
    pub phase: &'static str,
    pub version: &'static str,
    pub requires_phase_41f_native_ed25519_verification_established: bool,
    pub requires_phase_41f_establishes_native_ed25519_verification: bool,
    pub requires_signed_message_bytes_len_32: bool,
    pub checked_converts_signed_message_to_hash_32: bool,
    pub delegates_to_phase_34_canonical_payload_hash_validator: bool,
    pub success_status: PayloadHashBindingStatus,
    pub cryptographic_signature_proof_accepted: bool,
    pub verification_evidence_accepted: bool,
    pub guardian_validation_enabled: bool,
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

pub const PAYLOAD_HASH_BINDING_BOUNDARY_REPORT: PayloadHashBindingBoundaryReport =
    PayloadHashBindingBoundaryReport {
        phase: PAYLOAD_HASH_BINDING_BOUNDARY_PHASE_41G_2,
        version: PAYLOAD_HASH_BINDING_BOUNDARY_VERSION,
        requires_phase_41f_native_ed25519_verification_established: true,
        requires_phase_41f_establishes_native_ed25519_verification: true,
        requires_signed_message_bytes_len_32: true,
        checked_converts_signed_message_to_hash_32: true,
        delegates_to_phase_34_canonical_payload_hash_validator: true,
        success_status: PayloadHashBindingStatus::PayloadHashBindingEstablished,
        cryptographic_signature_proof_accepted: false,
        verification_evidence_accepted: false,
        guardian_validation_enabled: false,
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

pub fn payload_hash_binding_boundary_report() -> &'static PayloadHashBindingBoundaryReport {
    &PAYLOAD_HASH_BINDING_BOUNDARY_REPORT
}

pub fn establish_payload_hash_binding(
    raw_payload_bytes: &[u8],
    signed_message_bytes: &[u8],
    phase_41f_result: &Phase41F_2Ed25519SignatureVerificationResult,
) -> Result<PayloadHashBindingEstablished, PayloadHashBindingError> {
    if phase_41f_result.status
        != Phase41F_2Ed25519SignatureVerificationStatus::NativeEd25519VerificationEstablished
        || !phase_41f_result.establishes_native_ed25519_verification
    {
        return Err(error(
            PayloadHashBindingErrorKind::Phase41FNativeEd25519VerificationNotEstablished,
            phase_41f_result,
            signed_message_bytes.len(),
            None,
        ));
    }

    if signed_message_bytes.len() != 32 {
        return Err(error(
            PayloadHashBindingErrorKind::SignedMessageLengthInvalid,
            phase_41f_result,
            signed_message_bytes.len(),
            None,
        ));
    }

    let signed_hash_32: &[u8; 32] = signed_message_bytes.try_into().map_err(|_| {
        error(
            PayloadHashBindingErrorKind::SignedMessageHashConversionFailed,
            phase_41f_result,
            signed_message_bytes.len(),
            None,
        )
    })?;

    validate_guardian_payload_hash(raw_payload_bytes, signed_hash_32).map_err(|hash_error| {
        error(
            PayloadHashBindingErrorKind::CanonicalPayloadHashValidationFailed,
            phase_41f_result,
            signed_message_bytes.len(),
            Some(hash_error),
        )
    })?;

    Ok(PayloadHashBindingEstablished {
        status: PayloadHashBindingStatus::PayloadHashBindingEstablished,
    })
}

fn error(
    kind: PayloadHashBindingErrorKind,
    phase_41f_result: &Phase41F_2Ed25519SignatureVerificationResult,
    signed_message_len: usize,
    canonical_payload_hash_error: Option<CanonicalPayloadHashValidationError>,
) -> PayloadHashBindingError {
    PayloadHashBindingError {
        kind,
        phase_41f_status: phase_41f_result.status,
        establishes_native_ed25519_verification: phase_41f_result
            .establishes_native_ed25519_verification,
        signed_message_len,
        canonical_payload_hash_error,
    }
}

#[cfg(test)]
mod tests {
    use super::super::canonical_payload::{
        canonical_payload_hash_validation_report, compute_guardian_payload_hash_domain_separator,
        validate_guardian_payload_hash, CanonicalPayloadHashValidationErrorKind,
        XXXL_GUARDIAN_PAYLOAD_HASH_DOMAIN_SEPARATOR_V1, XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1,
    };
    use super::super::ed25519_instruction_byte_parsing_boundary::Phase41E_1ByteRange;
    use super::super::ed25519_signature_verification_boundary::{
        Phase41F_2Ed25519SignatureVerificationResult, Phase41F_2Ed25519VerificationModel,
        Phase41F_2VerifiedEd25519SignatureRanges,
    };
    use super::super::instructions_sysvar_access_contract_model::PHASE_41B_SAFETY_FLAGS;
    use super::*;

    const VALID_PAYLOAD_HEX: &str = "11005858584c5f474154455741595f4d494e5401000200\
         1111111111111111111111111111111111111111111111111111111111111111\
         010000000000000006000102030405061400\
         aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
         2000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
         070000000000000015cd5b07000000002000\
         bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\
         84cd5b0700000000\
         4444444444444444444444444444444444444444444444444444444444444444\
         5555555555555555555555555555555555555555555555555555555555555555\
         0010a5d4e8000000000000000000000010270010a5d4e80000000000000000000000\
         3333333333333333333333333333333333333333333333333333333333333333\
         2222222222222222222222222222222222222222222222222222222222222222\
         6666666666666666666666666666666666666666666666666666666666666666\
         b168de3a00000000";

    fn hex_val(byte: u8) -> u8 {
        match byte {
            b'0'..=b'9' => byte - b'0',
            b'a'..=b'f' => byte - b'a' + 10,
            b'A'..=b'F' => byte - b'A' + 10,
            _ => panic!("invalid hex byte"),
        }
    }

    fn hex_bytes(hex: &str) -> Vec<u8> {
        let compact: Vec<u8> = hex
            .bytes()
            .filter(|byte| !byte.is_ascii_whitespace())
            .collect();
        assert_eq!(compact.len() % 2, 0);

        compact
            .chunks_exact(2)
            .map(|pair| (hex_val(pair[0]) << 4) | hex_val(pair[1]))
            .collect()
    }

    fn valid_payload() -> Vec<u8> {
        hex_bytes(VALID_PAYLOAD_HEX)
    }

    fn established_phase_41f_result() -> Phase41F_2Ed25519SignatureVerificationResult {
        phase_41f_result(
            Phase41F_2Ed25519SignatureVerificationStatus::NativeEd25519VerificationEstablished,
            true,
        )
    }

    fn phase_41f_result(
        status: Phase41F_2Ed25519SignatureVerificationStatus,
        establishes_native_ed25519_verification: bool,
    ) -> Phase41F_2Ed25519SignatureVerificationResult {
        let established = status
            == Phase41F_2Ed25519SignatureVerificationStatus::NativeEd25519VerificationEstablished
            && establishes_native_ed25519_verification;

        Phase41F_2Ed25519SignatureVerificationResult {
            status,
            rejection_case: None,
            verification_model: established
                .then_some(Phase41F_2Ed25519VerificationModel::NativeEd25519Instruction),
            matched_instruction_index: established.then_some(1),
            instruction_data_len: 112,
            verified_ranges: established.then_some(Phase41F_2VerifiedEd25519SignatureRanges {
                signature_range: Phase41E_1ByteRange {
                    offset: 16,
                    len: 64,
                },
                public_key_range: Phase41E_1ByteRange {
                    offset: 80,
                    len: 32,
                },
                message_range: Phase41E_1ByteRange {
                    offset: 112,
                    len: 32,
                },
            }),
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

    #[test]
    fn establishes_payload_hash_binding_for_valid_canonical_payload_hash() {
        let payload = valid_payload();
        let phase_41f_result = established_phase_41f_result();

        let result = establish_payload_hash_binding(
            &payload,
            &XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1,
            &phase_41f_result,
        )
        .expect("canonical payload hash binding should be established");

        assert_eq!(
            result.status,
            PayloadHashBindingStatus::PayloadHashBindingEstablished
        );
    }

    #[test]
    fn rejects_when_phase_41f_native_ed25519_verification_is_not_established() {
        let payload = valid_payload();

        for phase_41f_result in [
            phase_41f_result(
                Phase41F_2Ed25519SignatureVerificationStatus::Ed25519ByteSlicesNotExtracted,
                false,
            ),
            phase_41f_result(
                Phase41F_2Ed25519SignatureVerificationStatus::NativeEd25519VerificationEstablished,
                false,
            ),
        ] {
            let error = establish_payload_hash_binding(
                &payload,
                &XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1,
                &phase_41f_result,
            )
            .expect_err("phase 41f must gate payload hash binding");

            assert_eq!(
                error.kind,
                PayloadHashBindingErrorKind::Phase41FNativeEd25519VerificationNotEstablished
            );
            assert!(error.canonical_payload_hash_error.is_none());
        }
    }

    #[test]
    fn rejects_signed_message_lengths_other_than_32() {
        let payload = valid_payload();
        let phase_41f_result = established_phase_41f_result();

        for len in [0usize, 31, 33] {
            let signed_message = vec![0xab; len];
            let error =
                establish_payload_hash_binding(&payload, &signed_message, &phase_41f_result)
                    .expect_err("signed ed25519 message must be exactly a 32-byte hash");

            assert_eq!(
                error.kind,
                PayloadHashBindingErrorKind::SignedMessageLengthInvalid
            );
            assert_eq!(error.signed_message_len, len);
            assert!(error.canonical_payload_hash_error.is_none());
        }
    }

    #[test]
    fn rejects_wrong_32_byte_signed_hash() {
        let payload = valid_payload();
        let phase_41f_result = established_phase_41f_result();
        let mut wrong_hash = XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1;
        wrong_hash[0] ^= 0xff;

        let error = establish_payload_hash_binding(&payload, &wrong_hash, &phase_41f_result)
            .expect_err("wrong signed hash must fail closed");

        assert_eq!(
            error.kind,
            PayloadHashBindingErrorKind::CanonicalPayloadHashValidationFailed
        );
        assert_eq!(
            error
                .canonical_payload_hash_error
                .map(|hash_error| hash_error.kind),
            Some(CanonicalPayloadHashValidationErrorKind::HashMismatch)
        );
    }

    #[test]
    fn rejects_malformed_raw_payload_before_authenticity() {
        let malformed_payload = [0u8];
        let phase_41f_result = established_phase_41f_result();

        let error = establish_payload_hash_binding(
            &malformed_payload,
            &XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1,
            &phase_41f_result,
        )
        .expect_err("malformed raw payload must fail before hash authenticity can bind");

        assert_eq!(
            error.kind,
            PayloadHashBindingErrorKind::CanonicalPayloadHashValidationFailed
        );
        assert_eq!(
            error
                .canonical_payload_hash_error
                .map(|hash_error| hash_error.kind),
            Some(CanonicalPayloadHashValidationErrorKind::RawPayloadDecode)
        );
    }

    #[test]
    fn keeps_execution_acceptance_and_mutation_flags_disabled() {
        let report = payload_hash_binding_boundary_report();

        assert_eq!(report.phase, PAYLOAD_HASH_BINDING_BOUNDARY_PHASE_41G_2);
        assert_eq!(report.version, PAYLOAD_HASH_BINDING_BOUNDARY_VERSION);
        assert!(report.requires_phase_41f_native_ed25519_verification_established);
        assert!(report.requires_phase_41f_establishes_native_ed25519_verification);
        assert!(report.requires_signed_message_bytes_len_32);
        assert!(report.checked_converts_signed_message_to_hash_32);
        assert!(report.delegates_to_phase_34_canonical_payload_hash_validator);
        assert_eq!(
            report.success_status,
            PayloadHashBindingStatus::PayloadHashBindingEstablished
        );
        assert!(!report.cryptographic_signature_proof_accepted);
        assert!(!report.verification_evidence_accepted);
        assert!(!report.guardian_validation_enabled);
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
        assert!(!PHASE_41B_SAFETY_FLAGS.cryptographic_signature_proof_accepted);
        assert!(!PHASE_41B_SAFETY_FLAGS.verification_evidence_accepted);
        assert!(!PHASE_41B_SAFETY_FLAGS.quorum_counting_enabled);
        assert!(!PHASE_41B_SAFETY_FLAGS.authorization_enabled);
        assert!(!PHASE_41B_SAFETY_FLAGS.replay_write_enabled);
        assert!(!PHASE_41B_SAFETY_FLAGS.processed_event_marking_enabled);
        assert!(!PHASE_41B_SAFETY_FLAGS.account_mutation_enabled);
        assert!(!PHASE_41B_SAFETY_FLAGS.cpi_enabled);
        assert!(!PHASE_41B_SAFETY_FLAGS.invoke_signed_enabled);
        assert!(!PHASE_41B_SAFETY_FLAGS.spl_token_mint_to_enabled);
        assert!(!PHASE_41B_SAFETY_FLAGS.process_instruction_handler_added);
        assert!(!PHASE_41B_SAFETY_FLAGS.live_route_enabled);
    }

    #[test]
    fn preserves_domain_separator_and_vector_parity_with_canonical_payload_validator() {
        let payload = valid_payload();
        let canonical_report = canonical_payload_hash_validation_report();

        assert_eq!(
            compute_guardian_payload_hash_domain_separator(),
            XXXL_GUARDIAN_PAYLOAD_HASH_DOMAIN_SEPARATOR_V1
        );
        assert_eq!(
            canonical_report.phase_23_domain_separator,
            XXXL_GUARDIAN_PAYLOAD_HASH_DOMAIN_SEPARATOR_V1
        );
        assert_eq!(
            canonical_report.phase_23_valid_payload_hash,
            XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1
        );
        assert!(canonical_report.hash_domain_matches_phase_23);
        validate_guardian_payload_hash(&payload, &XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1)
            .expect("phase 34 validator remains the canonical hash authority");
    }
}
