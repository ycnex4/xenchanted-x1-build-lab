use super::checked_ed25519_byte_extraction_boundary::Phase41F_1CheckedByteExtractionResult;
use super::ed25519_signature_verification_boundary::Phase41F_2Ed25519SignatureVerificationResult;
use super::guardian_membership_validation_boundary::{
    establish_guardian_membership_validation, AuthoritativeGuardianSetRef,
    GuardianMembershipValidated, GuardianMembershipValidationError,
};
use super::guardian_quorum::GuardianPublicKey;

pub const QUORUM_AUTHORIZATION_BOUNDARY_PHASE_41I: &str = "41I";
pub const QUORUM_AUTHORIZATION_BOUNDARY_VERSION: &str = "0.1.0";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GuardianQuorumAuthorizationAttempt<'a> {
    pub phase_41f_result: &'a Phase41F_2Ed25519SignatureVerificationResult,
    pub extraction_result: &'a Phase41F_1CheckedByteExtractionResult<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuardianQuorumAuthorizationAttemptStatus {
    Counted,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GuardianQuorumAuthorizationAttemptOutcome {
    pub attempt_index: usize,
    pub status: GuardianQuorumAuthorizationAttemptStatus,
    pub matched_guardian_index: Option<usize>,
    pub matched_guardian_public_key: Option<GuardianPublicKey>,
    pub error: Option<GuardianMembershipValidationError>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuardianQuorumAuthorizationStatus {
    QuorumAuthorizationEstablished,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuardianQuorumAuthorizationEstablished {
    pub status: GuardianQuorumAuthorizationStatus,
    pub guardian_set_id: [u8; 32],
    pub threshold: u8,
    pub guardian_count: usize,
    pub attempt_count: usize,
    pub successful_distinct_guardian_count: usize,
    pub counted_guardian_indexes: Vec<usize>,
    pub counted_guardian_public_keys: Vec<GuardianPublicKey>,
    pub attempt_outcomes: Vec<GuardianQuorumAuthorizationAttemptOutcome>,
    pub quorum_reached: bool,
    pub logical_quorum_authorization_established: bool,
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
pub enum GuardianQuorumAuthorizationErrorKind {
    EmptyAttempts,
    QuorumNotReached,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuardianQuorumAuthorizationError {
    pub kind: GuardianQuorumAuthorizationErrorKind,
    pub guardian_set_id: [u8; 32],
    pub threshold: u8,
    pub guardian_count: usize,
    pub attempt_count: usize,
    pub successful_distinct_guardian_count: usize,
    pub counted_guardian_indexes: Vec<usize>,
    pub counted_guardian_public_keys: Vec<GuardianPublicKey>,
    pub attempt_outcomes: Vec<GuardianQuorumAuthorizationAttemptOutcome>,
    pub quorum_reached: bool,
    pub logical_quorum_authorization_established: bool,
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
pub struct GuardianQuorumAuthorizationBoundaryReport {
    pub phase: &'static str,
    pub version: &'static str,
    pub composes_phase_41h_2: bool,
    pub accepts_free_signed_message_input: bool,
    pub accepts_free_decoded_payload_input: bool,
    pub accepts_free_guardian_approval_claims: bool,
    pub requires_same_raw_payload_for_all_attempts: bool,
    pub requires_same_expected_guardian_set_id_for_all_attempts: bool,
    pub requires_same_authoritative_guardian_set_for_all_attempts: bool,
    pub counts_only_successful_41h_validations: bool,
    pub dedups_by_matched_guardian_index: bool,
    pub dedups_by_guardian_public_key: bool,
    pub preserves_per_attempt_errors: bool,
    pub failed_attempts_not_counted: bool,
    pub failed_attempts_do_not_kill_valid_m_of_n: bool,
    pub quorum_threshold_enforced: bool,
    pub logical_quorum_authorization_established: bool,
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

pub const GUARDIAN_QUORUM_AUTHORIZATION_BOUNDARY_REPORT: GuardianQuorumAuthorizationBoundaryReport =
    GuardianQuorumAuthorizationBoundaryReport {
        phase: QUORUM_AUTHORIZATION_BOUNDARY_PHASE_41I,
        version: QUORUM_AUTHORIZATION_BOUNDARY_VERSION,
        composes_phase_41h_2: true,
        accepts_free_signed_message_input: false,
        accepts_free_decoded_payload_input: false,
        accepts_free_guardian_approval_claims: false,
        requires_same_raw_payload_for_all_attempts: true,
        requires_same_expected_guardian_set_id_for_all_attempts: true,
        requires_same_authoritative_guardian_set_for_all_attempts: true,
        counts_only_successful_41h_validations: true,
        dedups_by_matched_guardian_index: true,
        dedups_by_guardian_public_key: true,
        preserves_per_attempt_errors: true,
        failed_attempts_not_counted: true,
        failed_attempts_do_not_kill_valid_m_of_n: true,
        quorum_threshold_enforced: true,
        logical_quorum_authorization_established: true,
        quorum_counting_enabled: true,
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

pub fn guardian_quorum_authorization_boundary_report(
) -> &'static GuardianQuorumAuthorizationBoundaryReport {
    &GUARDIAN_QUORUM_AUTHORIZATION_BOUNDARY_REPORT
}

pub fn establish_guardian_quorum_authorization<'a>(
    attempts: &[GuardianQuorumAuthorizationAttempt<'a>],
    raw_payload_bytes: &'a [u8],
    expected_configured_guardian_set_id: &'a [u8; 32],
    guardian_set: AuthoritativeGuardianSetRef<'a>,
) -> Result<GuardianQuorumAuthorizationEstablished, GuardianQuorumAuthorizationError> {
    if attempts.is_empty() {
        return Err(error(
            GuardianQuorumAuthorizationErrorKind::EmptyAttempts,
            guardian_set,
            0,
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ));
    }

    let mut counted_guardian_indexes = Vec::new();
    let mut counted_guardian_public_keys = Vec::new();
    let mut attempt_outcomes = Vec::with_capacity(attempts.len());

    for (attempt_index, attempt) in attempts.iter().enumerate() {
        match establish_guardian_membership_validation(
            attempt.phase_41f_result,
            attempt.extraction_result,
            raw_payload_bytes,
            expected_configured_guardian_set_id,
            guardian_set,
        ) {
            Ok(validated) => {
                if already_counted(
                    &validated,
                    &counted_guardian_indexes,
                    &counted_guardian_public_keys,
                ) {
                    attempt_outcomes.push(GuardianQuorumAuthorizationAttemptOutcome {
                        attempt_index,
                        status: GuardianQuorumAuthorizationAttemptStatus::Rejected,
                        matched_guardian_index: Some(validated.matched_guardian_index),
                        matched_guardian_public_key: Some(validated.matched_guardian_public_key),
                        error: None,
                    });
                    continue;
                }

                counted_guardian_indexes.push(validated.matched_guardian_index);
                counted_guardian_public_keys.push(validated.matched_guardian_public_key);
                attempt_outcomes.push(GuardianQuorumAuthorizationAttemptOutcome {
                    attempt_index,
                    status: GuardianQuorumAuthorizationAttemptStatus::Counted,
                    matched_guardian_index: Some(validated.matched_guardian_index),
                    matched_guardian_public_key: Some(validated.matched_guardian_public_key),
                    error: None,
                });
            }
            Err(err) => {
                attempt_outcomes.push(GuardianQuorumAuthorizationAttemptOutcome {
                    attempt_index,
                    status: GuardianQuorumAuthorizationAttemptStatus::Rejected,
                    matched_guardian_index: None,
                    matched_guardian_public_key: None,
                    error: Some(err),
                });
            }
        }
    }

    let successful_distinct_guardian_count = counted_guardian_indexes.len();

    if successful_distinct_guardian_count < guardian_set.threshold() as usize {
        return Err(error(
            GuardianQuorumAuthorizationErrorKind::QuorumNotReached,
            guardian_set,
            attempts.len(),
            counted_guardian_indexes,
            counted_guardian_public_keys,
            attempt_outcomes,
        ));
    }

    Ok(GuardianQuorumAuthorizationEstablished {
        status: GuardianQuorumAuthorizationStatus::QuorumAuthorizationEstablished,
        guardian_set_id: *guardian_set.guardian_set_id(),
        threshold: guardian_set.threshold(),
        guardian_count: guardian_set.guardians().len(),
        attempt_count: attempts.len(),
        successful_distinct_guardian_count,
        counted_guardian_indexes,
        counted_guardian_public_keys,
        attempt_outcomes,
        quorum_reached: true,
        logical_quorum_authorization_established: true,
        quorum_counting_enabled: true,
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

fn already_counted(
    validated: &GuardianMembershipValidated,
    counted_guardian_indexes: &[usize],
    counted_guardian_public_keys: &[GuardianPublicKey],
) -> bool {
    counted_guardian_indexes.contains(&validated.matched_guardian_index)
        || counted_guardian_public_keys.contains(&validated.matched_guardian_public_key)
}

fn error(
    kind: GuardianQuorumAuthorizationErrorKind,
    guardian_set: AuthoritativeGuardianSetRef<'_>,
    attempt_count: usize,
    counted_guardian_indexes: Vec<usize>,
    counted_guardian_public_keys: Vec<GuardianPublicKey>,
    attempt_outcomes: Vec<GuardianQuorumAuthorizationAttemptOutcome>,
) -> GuardianQuorumAuthorizationError {
    let successful_distinct_guardian_count = counted_guardian_indexes.len();

    GuardianQuorumAuthorizationError {
        kind,
        guardian_set_id: *guardian_set.guardian_set_id(),
        threshold: guardian_set.threshold(),
        guardian_count: guardian_set.guardians().len(),
        attempt_count,
        successful_distinct_guardian_count,
        counted_guardian_indexes,
        counted_guardian_public_keys,
        attempt_outcomes,
        quorum_reached: false,
        logical_quorum_authorization_established: false,
        quorum_counting_enabled: true,
        authorization_enabled: false,
        replay_write_enabled: false,
        processed_event_marking_enabled: false,
        account_mutation_enabled: false,
        cpi_enabled: false,
        invoke_signed_enabled: false,
        spl_token_mint_to_enabled: false,
        process_instruction_handler_added: false,
        live_route_enabled: false,
    }
}

#[cfg(test)]
mod tests {
    use super::super::canonical_payload::compute_guardian_payload_hash;
    use super::super::checked_ed25519_byte_extraction_boundary::{
        Phase41F_1CheckedByteExtractionResult, Phase41F_1CheckedByteExtractionStatus,
        Phase41F_1ExtractedEd25519ByteSlices,
    };
    use super::super::ed25519_instruction_byte_parsing_boundary::Phase41E_1ByteRange;
    use super::super::ed25519_signature_verification_boundary::{
        Phase41F_2Ed25519SignatureVerificationResult, Phase41F_2Ed25519SignatureVerificationStatus,
        Phase41F_2Ed25519VerificationModel, Phase41F_2VerifiedEd25519SignatureRanges,
    };
    use super::super::guardian_membership_validation_boundary::{
        AuthoritativeGuardianSetRef, GuardianMembershipValidationErrorKind,
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

    const SIGNATURE_BYTES: [u8; 64] = [0x11; 64];
    const GUARDIAN_1_PUBLIC_KEY_BYTES: [u8; 32] = [0x01; 32];
    const GUARDIAN_2_PUBLIC_KEY_BYTES: [u8; 32] = [0x02; 32];
    const GUARDIAN_3_PUBLIC_KEY_BYTES: [u8; 32] = [0x03; 32];
    const UNKNOWN_PUBLIC_KEY_BYTES: [u8; 32] = [0x09; 32];

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

    fn guardian(byte: u8) -> GuardianPublicKey {
        GuardianPublicKey([byte; 32])
    }

    fn signature_range() -> Phase41E_1ByteRange {
        Phase41E_1ByteRange {
            offset: 16,
            len: 64,
        }
    }

    fn public_key_range() -> Phase41E_1ByteRange {
        Phase41E_1ByteRange {
            offset: 80,
            len: 32,
        }
    }

    fn message_range() -> Phase41E_1ByteRange {
        Phase41E_1ByteRange {
            offset: 112,
            len: 32,
        }
    }

    fn verified_ranges() -> Phase41F_2VerifiedEd25519SignatureRanges {
        Phase41F_2VerifiedEd25519SignatureRanges {
            signature_range: signature_range(),
            public_key_range: public_key_range(),
            message_range: message_range(),
        }
    }

    fn established_phase_41f_result() -> Phase41F_2Ed25519SignatureVerificationResult {
        Phase41F_2Ed25519SignatureVerificationResult {
            status:
                Phase41F_2Ed25519SignatureVerificationStatus::NativeEd25519VerificationEstablished,
            rejection_case: None,
            verification_model: Some(Phase41F_2Ed25519VerificationModel::NativeEd25519Instruction),
            matched_instruction_index: Some(1),
            instruction_data_len: 144,
            verified_ranges: Some(verified_ranges()),
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

    fn established_extraction_result<'a>(
        public_key_bytes: &'a [u8; 32],
        message_bytes: &'a [u8],
    ) -> Phase41F_1CheckedByteExtractionResult<'a> {
        Phase41F_1CheckedByteExtractionResult {
            status: Phase41F_1CheckedByteExtractionStatus::CheckedEd25519ByteSlicesExtracted,
            rejection_case: None,
            matched_instruction_index: Some(1),
            instruction_data_len: 144,
            extracted_slices: Some(Phase41F_1ExtractedEd25519ByteSlices {
                signature_bytes: &SIGNATURE_BYTES,
                public_key_bytes,
                message_bytes,
                signature_range: signature_range(),
                public_key_range: public_key_range(),
                message_range: message_range(),
            }),
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

    #[test]
    fn report_documents_41i_boundary_and_forbidden_runtime_surfaces() {
        let report = guardian_quorum_authorization_boundary_report();

        assert_eq!(report.phase, QUORUM_AUTHORIZATION_BOUNDARY_PHASE_41I);
        assert!(report.composes_phase_41h_2);
        assert!(!report.accepts_free_signed_message_input);
        assert!(!report.accepts_free_decoded_payload_input);
        assert!(!report.accepts_free_guardian_approval_claims);
        assert!(report.requires_same_raw_payload_for_all_attempts);
        assert!(report.requires_same_expected_guardian_set_id_for_all_attempts);
        assert!(report.requires_same_authoritative_guardian_set_for_all_attempts);
        assert!(report.counts_only_successful_41h_validations);
        assert!(report.dedups_by_matched_guardian_index);
        assert!(report.dedups_by_guardian_public_key);
        assert!(report.preserves_per_attempt_errors);
        assert!(report.failed_attempts_not_counted);
        assert!(report.failed_attempts_do_not_kill_valid_m_of_n);
        assert!(report.quorum_threshold_enforced);
        assert!(report.logical_quorum_authorization_established);
        assert!(report.quorum_counting_enabled);
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

    #[test]
    fn establishes_quorum_from_successful_distinct_41h_validations() {
        let raw_payload = raw_payload_bytes_with_guardian_set_id(&GUARDIAN_SET_ID);
        let message_hash = compute_guardian_payload_hash(&raw_payload).expect("hash");

        let phase_41f_1 = established_phase_41f_result();
        let phase_41f_2 = established_phase_41f_result();
        let extraction_1 =
            established_extraction_result(&GUARDIAN_1_PUBLIC_KEY_BYTES, &message_hash);
        let extraction_2 =
            established_extraction_result(&GUARDIAN_2_PUBLIC_KEY_BYTES, &message_hash);
        let guardians = [guardian(1), guardian(2), guardian(3)];

        let attempts = [
            GuardianQuorumAuthorizationAttempt {
                phase_41f_result: &phase_41f_1,
                extraction_result: &extraction_1,
            },
            GuardianQuorumAuthorizationAttempt {
                phase_41f_result: &phase_41f_2,
                extraction_result: &extraction_2,
            },
        ];

        let result = establish_guardian_quorum_authorization(
            &attempts,
            &raw_payload,
            &GUARDIAN_SET_ID,
            authoritative_set(&GUARDIAN_SET_ID, 2, &guardians),
        )
        .expect("quorum");

        assert_eq!(result.successful_distinct_guardian_count, 2);
        assert_eq!(result.counted_guardian_indexes, vec![0, 1]);
        assert!(result.quorum_reached);
        assert!(result.logical_quorum_authorization_established);
        assert!(result.quorum_counting_enabled);
        assert!(!result.authorization_enabled);
        assert!(!result.replay_write_enabled);
        assert!(!result.account_mutation_enabled);
        assert!(!result.cpi_enabled);
        assert!(!result.live_route_enabled);
    }

    #[test]
    fn failed_attempt_does_not_kill_valid_m_of_n_and_error_is_preserved() {
        let raw_payload = raw_payload_bytes_with_guardian_set_id(&GUARDIAN_SET_ID);
        let message_hash = compute_guardian_payload_hash(&raw_payload).expect("hash");

        let phase_41f_1 = established_phase_41f_result();
        let phase_41f_bad = established_phase_41f_result();
        let phase_41f_2 = established_phase_41f_result();

        let extraction_1 =
            established_extraction_result(&GUARDIAN_1_PUBLIC_KEY_BYTES, &message_hash);
        let extraction_bad =
            established_extraction_result(&UNKNOWN_PUBLIC_KEY_BYTES, &message_hash);
        let extraction_2 =
            established_extraction_result(&GUARDIAN_2_PUBLIC_KEY_BYTES, &message_hash);

        let guardians = [guardian(1), guardian(2), guardian(3)];

        let attempts = [
            GuardianQuorumAuthorizationAttempt {
                phase_41f_result: &phase_41f_1,
                extraction_result: &extraction_1,
            },
            GuardianQuorumAuthorizationAttempt {
                phase_41f_result: &phase_41f_bad,
                extraction_result: &extraction_bad,
            },
            GuardianQuorumAuthorizationAttempt {
                phase_41f_result: &phase_41f_2,
                extraction_result: &extraction_2,
            },
        ];

        let result = establish_guardian_quorum_authorization(
            &attempts,
            &raw_payload,
            &GUARDIAN_SET_ID,
            authoritative_set(&GUARDIAN_SET_ID, 2, &guardians),
        )
        .expect("quorum despite failed attempt");

        assert_eq!(result.successful_distinct_guardian_count, 2);
        assert_eq!(result.attempt_outcomes.len(), 3);
        assert_eq!(
            result.attempt_outcomes[1].status,
            GuardianQuorumAuthorizationAttemptStatus::Rejected
        );
        assert_eq!(
            result.attempt_outcomes[1].error.expect("error").kind,
            GuardianMembershipValidationErrorKind::VerifiedSignerNotGuardian
        );
    }

    #[test]
    fn duplicate_guardian_attempt_is_not_counted_twice() {
        let raw_payload = raw_payload_bytes_with_guardian_set_id(&GUARDIAN_SET_ID);
        let message_hash = compute_guardian_payload_hash(&raw_payload).expect("hash");

        let phase_41f_1 = established_phase_41f_result();
        let phase_41f_duplicate = established_phase_41f_result();
        let extraction_1 =
            established_extraction_result(&GUARDIAN_1_PUBLIC_KEY_BYTES, &message_hash);
        let extraction_duplicate =
            established_extraction_result(&GUARDIAN_1_PUBLIC_KEY_BYTES, &message_hash);
        let guardians = [guardian(1), guardian(2), guardian(3)];

        let attempts = [
            GuardianQuorumAuthorizationAttempt {
                phase_41f_result: &phase_41f_1,
                extraction_result: &extraction_1,
            },
            GuardianQuorumAuthorizationAttempt {
                phase_41f_result: &phase_41f_duplicate,
                extraction_result: &extraction_duplicate,
            },
        ];

        let err = establish_guardian_quorum_authorization(
            &attempts,
            &raw_payload,
            &GUARDIAN_SET_ID,
            authoritative_set(&GUARDIAN_SET_ID, 2, &guardians),
        )
        .expect_err("duplicate guardian cannot satisfy threshold twice");

        assert_eq!(
            err.kind,
            GuardianQuorumAuthorizationErrorKind::QuorumNotReached
        );
        assert_eq!(err.successful_distinct_guardian_count, 1);
        assert_eq!(err.counted_guardian_indexes, vec![0]);
        assert_eq!(
            err.attempt_outcomes[1].status,
            GuardianQuorumAuthorizationAttemptStatus::Rejected
        );
        assert!(err.attempt_outcomes[1].error.is_none());
    }

    #[test]
    fn same_raw_payload_is_enforced_by_internal_41h_2_composition() {
        let raw_payload = raw_payload_bytes_with_guardian_set_id(&GUARDIAN_SET_ID);
        let other_raw_payload = raw_payload_bytes_with_guardian_set_id(&OTHER_GUARDIAN_SET_ID);
        let other_message_hash = compute_guardian_payload_hash(&other_raw_payload).expect("hash");

        let phase_41f = established_phase_41f_result();
        let extraction =
            established_extraction_result(&GUARDIAN_1_PUBLIC_KEY_BYTES, &other_message_hash);
        let guardians = [guardian(1), guardian(2), guardian(3)];

        let attempts = [GuardianQuorumAuthorizationAttempt {
            phase_41f_result: &phase_41f,
            extraction_result: &extraction,
        }];

        let err = establish_guardian_quorum_authorization(
            &attempts,
            &raw_payload,
            &GUARDIAN_SET_ID,
            authoritative_set(&GUARDIAN_SET_ID, 1, &guardians),
        )
        .expect_err("wrong extracted message hash for shared raw payload");

        assert_eq!(
            err.kind,
            GuardianQuorumAuthorizationErrorKind::QuorumNotReached
        );
        assert_eq!(err.successful_distinct_guardian_count, 0);
        assert_eq!(
            err.attempt_outcomes[0].error.expect("error").kind,
            GuardianMembershipValidationErrorKind::PayloadHashBindingNotEstablished
        );
    }

    #[test]
    fn same_expected_guardian_set_id_is_enforced_for_all_attempts() {
        let raw_payload = raw_payload_bytes_with_guardian_set_id(&GUARDIAN_SET_ID);
        let message_hash = compute_guardian_payload_hash(&raw_payload).expect("hash");

        let phase_41f = established_phase_41f_result();
        let extraction = established_extraction_result(&GUARDIAN_1_PUBLIC_KEY_BYTES, &message_hash);
        let guardians = [guardian(1), guardian(2), guardian(3)];

        let attempts = [GuardianQuorumAuthorizationAttempt {
            phase_41f_result: &phase_41f,
            extraction_result: &extraction,
        }];

        let err = establish_guardian_quorum_authorization(
            &attempts,
            &raw_payload,
            &OTHER_GUARDIAN_SET_ID,
            authoritative_set(&GUARDIAN_SET_ID, 1, &guardians),
        )
        .expect_err("configured guardian set id mismatch");

        assert_eq!(
            err.kind,
            GuardianQuorumAuthorizationErrorKind::QuorumNotReached
        );
        assert_eq!(
            err.attempt_outcomes[0].error.expect("error").kind,
            GuardianMembershipValidationErrorKind::GuardianSetIdMismatch
        );
    }

    #[test]
    fn empty_attempts_rejected_without_runtime_authority() {
        let guardians = [guardian(1), guardian(2), guardian(3)];

        let err = establish_guardian_quorum_authorization(
            &[],
            &[],
            &GUARDIAN_SET_ID,
            authoritative_set(&GUARDIAN_SET_ID, 2, &guardians),
        )
        .expect_err("empty attempts");

        assert_eq!(
            err.kind,
            GuardianQuorumAuthorizationErrorKind::EmptyAttempts
        );
        assert_eq!(err.successful_distinct_guardian_count, 0);
        assert!(!err.logical_quorum_authorization_established);
        assert!(err.quorum_counting_enabled);
        assert!(!err.authorization_enabled);
        assert!(!err.replay_write_enabled);
        assert!(!err.account_mutation_enabled);
        assert!(!err.cpi_enabled);
        assert!(!err.live_route_enabled);
    }
}
