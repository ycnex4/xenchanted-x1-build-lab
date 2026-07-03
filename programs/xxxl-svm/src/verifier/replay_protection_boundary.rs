use super::guardian_membership_validation_boundary::AuthoritativeGuardianSetRef;
use super::quorum_authorization_boundary::{
    establish_guardian_quorum_authorization, GuardianQuorumAuthorizationAttempt,
    GuardianQuorumAuthorizationError,
};
use super::processed_registry_account_loading_boundary::Phase41K3ProcessedRegistryLoadWitness;
use super::raw_payload::{decode_guardian_payload_raw, RawPayloadDecodeError};

pub const REPLAY_PROTECTION_BOUNDARY_PHASE_41J: &str = "41J";
pub const REPLAY_PROTECTION_BOUNDARY_VERSION: &str = "0.1.0";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessedRegistryViewSource {
    ProgramControlledAbstractModel,
    CallerInstructionData,
    Unauthenticated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthoritativeProcessedRegistryViewRef<'a> {
    processed_canonical_event_keys: &'a [[u8; 32]],
    source: ProcessedRegistryViewSource,
    caller_instruction_data: bool,
}

impl<'a> AuthoritativeProcessedRegistryViewRef<'a> {
    fn from_program_controlled_abstract_model(
        processed_canonical_event_keys: &'a [[u8; 32]],
    ) -> Self {
        Self {
            processed_canonical_event_keys,
            source: ProcessedRegistryViewSource::ProgramControlledAbstractModel,
            caller_instruction_data: false,
        }
    }

    pub(crate) fn from_phase_41k_3_processed_registry_load_witness(
        witness: &'a Phase41K3ProcessedRegistryLoadWitness,
    ) -> Self {
        match witness.processed_canonical_event_key_ref() {
            Some(canonical_event_key) => Self::from_program_controlled_abstract_model(
                core::slice::from_ref(canonical_event_key),
            ),
            None => Self::from_program_controlled_abstract_model(&[]),
        }
    }

    pub fn caller_supplied_for_rejection(processed_canonical_event_keys: &'a [[u8; 32]]) -> Self {
        Self {
            processed_canonical_event_keys,
            source: ProcessedRegistryViewSource::CallerInstructionData,
            caller_instruction_data: true,
        }
    }

    pub fn unauthenticated_for_rejection(processed_canonical_event_keys: &'a [[u8; 32]]) -> Self {
        Self {
            processed_canonical_event_keys,
            source: ProcessedRegistryViewSource::Unauthenticated,
            caller_instruction_data: false,
        }
    }

    pub fn processed_canonical_event_keys(&self) -> &'a [[u8; 32]] {
        self.processed_canonical_event_keys
    }

    pub fn source(&self) -> ProcessedRegistryViewSource {
        self.source
    }

    pub fn caller_instruction_data(&self) -> bool {
        self.caller_instruction_data
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayProtectionEligibilityStatus {
    ReplayProtectionEligibilityEstablished,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayProtectionEligibilityEstablished {
    pub status: ReplayProtectionEligibilityStatus,
    pub guardian_set_id: [u8; 32],
    pub threshold: u8,
    pub successful_distinct_guardian_count: usize,
    pub canonical_event_key: [u8; 32],
    pub processed_registry_source: ProcessedRegistryViewSource,
    pub processed_registry_entry_count: usize,
    pub raw_payload_decoded_internally: bool,
    pub canonical_event_key_derived_from_raw_payload: bool,
    pub internal_41i_quorum_established: bool,
    pub replay_check_passed: bool,
    pub processed_marking_eligible: bool,
    pub processed_marking_intent: bool,
    pub replay_write_enabled: bool,
    pub processed_event_marking_enabled: bool,
    pub account_mutation_enabled: bool,
    pub runtime_account_loading_enabled: bool,
    pub sysvar_loading_enabled: bool,
    pub cpi_enabled: bool,
    pub invoke_signed_enabled: bool,
    pub spl_token_mint_to_enabled: bool,
    pub process_instruction_handler_added: bool,
    pub live_route_enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayProtectionEligibilityErrorKind {
    QuorumAuthorizationNotEstablished,
    RawPayloadDecodeFailed,
    CallerSuppliedProcessedRegistryRejected,
    UnauthenticatedProcessedRegistryRejected,
    CanonicalEventAlreadyProcessed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayProtectionEligibilityError {
    pub kind: ReplayProtectionEligibilityErrorKind,
    pub canonical_event_key: Option<[u8; 32]>,
    pub processed_registry_source: ProcessedRegistryViewSource,
    pub processed_registry_from_caller_instruction_data: bool,
    pub processed_registry_entry_count: usize,
    pub quorum_authorization_error: Option<GuardianQuorumAuthorizationError>,
    pub raw_payload_decode_error: Option<RawPayloadDecodeError>,
    pub raw_payload_decoded_internally: bool,
    pub canonical_event_key_derived_from_raw_payload: bool,
    pub internal_41i_quorum_established: bool,
    pub replay_check_passed: bool,
    pub processed_marking_eligible: bool,
    pub processed_marking_intent: bool,
    pub replay_write_enabled: bool,
    pub processed_event_marking_enabled: bool,
    pub account_mutation_enabled: bool,
    pub runtime_account_loading_enabled: bool,
    pub sysvar_loading_enabled: bool,
    pub cpi_enabled: bool,
    pub invoke_signed_enabled: bool,
    pub spl_token_mint_to_enabled: bool,
    pub process_instruction_handler_added: bool,
    pub live_route_enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReplayProtectionBoundaryReport {
    pub phase: &'static str,
    pub version: &'static str,
    pub composes_phase_41i_internally: bool,
    pub accepts_external_41i_result: bool,
    pub binds_raw_payload_to_internal_41i_authorization: bool,
    pub decodes_raw_payload_internally: bool,
    pub derives_canonical_event_key_from_raw_payload: bool,
    pub uses_canonical_event_key_as_replay_key: bool,
    pub uses_message_nonce_as_replay_key: bool,
    pub accepts_free_replay_key: bool,
    pub accepts_free_canonical_event_key: bool,
    pub accepts_free_decoded_payload: bool,
    pub requires_authoritative_processed_registry_view: bool,
    pub rejects_caller_supplied_processed_registry_view: bool,
    pub rejects_unauthenticated_processed_registry_view: bool,
    pub checks_abstract_processed_registry_view: bool,
    pub returns_replay_eligibility_only: bool,
    pub processed_marking_intent_only: bool,
    pub account_info_used: bool,
    pub sysvar_loading_enabled: bool,
    pub runtime_account_loading_enabled: bool,
    pub replay_write_enabled: bool,
    pub processed_event_marking_enabled: bool,
    pub account_mutation_enabled: bool,
    pub cpi_enabled: bool,
    pub invoke_signed_enabled: bool,
    pub spl_token_mint_to_enabled: bool,
    pub process_instruction_handler_added: bool,
    pub live_route_enabled: bool,
}

pub const REPLAY_PROTECTION_BOUNDARY_REPORT: ReplayProtectionBoundaryReport =
    ReplayProtectionBoundaryReport {
        phase: REPLAY_PROTECTION_BOUNDARY_PHASE_41J,
        version: REPLAY_PROTECTION_BOUNDARY_VERSION,
        composes_phase_41i_internally: true,
        accepts_external_41i_result: false,
        binds_raw_payload_to_internal_41i_authorization: true,
        decodes_raw_payload_internally: true,
        derives_canonical_event_key_from_raw_payload: true,
        uses_canonical_event_key_as_replay_key: true,
        uses_message_nonce_as_replay_key: false,
        accepts_free_replay_key: false,
        accepts_free_canonical_event_key: false,
        accepts_free_decoded_payload: false,
        requires_authoritative_processed_registry_view: true,
        rejects_caller_supplied_processed_registry_view: true,
        rejects_unauthenticated_processed_registry_view: true,
        checks_abstract_processed_registry_view: true,
        returns_replay_eligibility_only: true,
        processed_marking_intent_only: true,
        account_info_used: false,
        sysvar_loading_enabled: false,
        runtime_account_loading_enabled: false,
        replay_write_enabled: false,
        processed_event_marking_enabled: false,
        account_mutation_enabled: false,
        cpi_enabled: false,
        invoke_signed_enabled: false,
        spl_token_mint_to_enabled: false,
        process_instruction_handler_added: false,
        live_route_enabled: false,
    };

pub fn replay_protection_boundary_report() -> &'static ReplayProtectionBoundaryReport {
    &REPLAY_PROTECTION_BOUNDARY_REPORT
}

pub fn establish_replay_protection_eligibility<'a>(
    attempts: &[GuardianQuorumAuthorizationAttempt<'a>],
    raw_payload_bytes: &'a [u8],
    expected_configured_guardian_set_id: &'a [u8; 32],
    guardian_set: AuthoritativeGuardianSetRef<'a>,
    processed_registry_view: AuthoritativeProcessedRegistryViewRef<'a>,
) -> Result<ReplayProtectionEligibilityEstablished, ReplayProtectionEligibilityError> {
    let quorum = establish_guardian_quorum_authorization(
        attempts,
        raw_payload_bytes,
        expected_configured_guardian_set_id,
        guardian_set,
    )
    .map_err(|quorum_authorization_error| {
        error_with_quorum_authorization_error(
            ReplayProtectionEligibilityErrorKind::QuorumAuthorizationNotEstablished,
            processed_registry_view,
            quorum_authorization_error,
        )
    })?;

    let decoded_payload =
        decode_guardian_payload_raw(raw_payload_bytes).map_err(|raw_payload_decode_error| {
            error_with_raw_payload_decode_error(
                ReplayProtectionEligibilityErrorKind::RawPayloadDecodeFailed,
                processed_registry_view,
                raw_payload_decode_error,
                true,
            )
        })?;

    let canonical_event_key = *decoded_payload.canonical_event_key;

    if processed_registry_view.caller_instruction_data() {
        return Err(error_with_canonical_event_key(
            ReplayProtectionEligibilityErrorKind::CallerSuppliedProcessedRegistryRejected,
            processed_registry_view,
            canonical_event_key,
            true,
        ));
    }

    if processed_registry_view.source()
        != ProcessedRegistryViewSource::ProgramControlledAbstractModel
    {
        return Err(error_with_canonical_event_key(
            ReplayProtectionEligibilityErrorKind::UnauthenticatedProcessedRegistryRejected,
            processed_registry_view,
            canonical_event_key,
            true,
        ));
    }

    if processed_registry_view
        .processed_canonical_event_keys()
        .contains(&canonical_event_key)
    {
        return Err(error_with_canonical_event_key(
            ReplayProtectionEligibilityErrorKind::CanonicalEventAlreadyProcessed,
            processed_registry_view,
            canonical_event_key,
            true,
        ));
    }

    Ok(ReplayProtectionEligibilityEstablished {
        status: ReplayProtectionEligibilityStatus::ReplayProtectionEligibilityEstablished,
        guardian_set_id: quorum.guardian_set_id,
        threshold: quorum.threshold,
        successful_distinct_guardian_count: quorum.successful_distinct_guardian_count,
        canonical_event_key,
        processed_registry_source: processed_registry_view.source(),
        processed_registry_entry_count: processed_registry_view
            .processed_canonical_event_keys()
            .len(),
        raw_payload_decoded_internally: true,
        canonical_event_key_derived_from_raw_payload: true,
        internal_41i_quorum_established: true,
        replay_check_passed: true,
        processed_marking_eligible: true,
        processed_marking_intent: true,
        replay_write_enabled: false,
        processed_event_marking_enabled: false,
        account_mutation_enabled: false,
        runtime_account_loading_enabled: false,
        sysvar_loading_enabled: false,
        cpi_enabled: false,
        invoke_signed_enabled: false,
        spl_token_mint_to_enabled: false,
        process_instruction_handler_added: false,
        live_route_enabled: false,
    })
}

fn error_with_quorum_authorization_error(
    kind: ReplayProtectionEligibilityErrorKind,
    processed_registry_view: AuthoritativeProcessedRegistryViewRef<'_>,
    quorum_authorization_error: GuardianQuorumAuthorizationError,
) -> ReplayProtectionEligibilityError {
    error_with_details(
        kind,
        processed_registry_view,
        None,
        Some(quorum_authorization_error),
        None,
        false,
        false,
    )
}

fn error_with_raw_payload_decode_error(
    kind: ReplayProtectionEligibilityErrorKind,
    processed_registry_view: AuthoritativeProcessedRegistryViewRef<'_>,
    raw_payload_decode_error: RawPayloadDecodeError,
    internal_41i_quorum_established: bool,
) -> ReplayProtectionEligibilityError {
    error_with_details(
        kind,
        processed_registry_view,
        None,
        None,
        Some(raw_payload_decode_error),
        true,
        internal_41i_quorum_established,
    )
}

fn error_with_canonical_event_key(
    kind: ReplayProtectionEligibilityErrorKind,
    processed_registry_view: AuthoritativeProcessedRegistryViewRef<'_>,
    canonical_event_key: [u8; 32],
    internal_41i_quorum_established: bool,
) -> ReplayProtectionEligibilityError {
    error_with_details(
        kind,
        processed_registry_view,
        Some(canonical_event_key),
        None,
        None,
        true,
        internal_41i_quorum_established,
    )
}

fn error_with_details(
    kind: ReplayProtectionEligibilityErrorKind,
    processed_registry_view: AuthoritativeProcessedRegistryViewRef<'_>,
    canonical_event_key: Option<[u8; 32]>,
    quorum_authorization_error: Option<GuardianQuorumAuthorizationError>,
    raw_payload_decode_error: Option<RawPayloadDecodeError>,
    raw_payload_decoded_internally: bool,
    internal_41i_quorum_established: bool,
) -> ReplayProtectionEligibilityError {
    ReplayProtectionEligibilityError {
        kind,
        canonical_event_key,
        processed_registry_source: processed_registry_view.source(),
        processed_registry_from_caller_instruction_data: processed_registry_view
            .caller_instruction_data(),
        processed_registry_entry_count: processed_registry_view
            .processed_canonical_event_keys()
            .len(),
        quorum_authorization_error,
        raw_payload_decode_error,
        raw_payload_decoded_internally,
        canonical_event_key_derived_from_raw_payload: canonical_event_key.is_some(),
        internal_41i_quorum_established,
        replay_check_passed: false,
        processed_marking_eligible: false,
        processed_marking_intent: false,
        replay_write_enabled: false,
        processed_event_marking_enabled: false,
        account_mutation_enabled: false,
        runtime_account_loading_enabled: false,
        sysvar_loading_enabled: false,
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
    use super::super::guardian_membership_validation_boundary::AuthoritativeGuardianSetRef;
    use super::super::guardian_quorum::GuardianPublicKey;
    use super::*;

    const GUARDIAN_SET_ID: [u8; 32] = [0x22; 32];
    const OTHER_GUARDIAN_SET_ID: [u8; 32] = [0x23; 32];
    const ROUTE_ID: [u8; 32] = [0x11; 32];
    const CANONICAL_EVENT_KEY: [u8; 32] = [0x44; 32];
    const OTHER_CANONICAL_EVENT_KEY: [u8; 32] = [0x45; 32];
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

    fn raw_payload_bytes_with_event_key(
        guardian_set_id: &[u8; 32],
        canonical_event_key: &[u8; 32],
    ) -> Vec<u8> {
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
        out.extend_from_slice(canonical_event_key);
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

    fn authoritative_guardian_set<'a>(
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

    fn authoritative_registry<'a>(
        processed: &'a [[u8; 32]],
    ) -> AuthoritativeProcessedRegistryViewRef<'a> {
        AuthoritativeProcessedRegistryViewRef::from_program_controlled_abstract_model(processed)
    }

    #[test]
    fn report_documents_41j_non_mutating_boundary() {
        let report = replay_protection_boundary_report();

        assert_eq!(report.phase, REPLAY_PROTECTION_BOUNDARY_PHASE_41J);
        assert!(report.composes_phase_41i_internally);
        assert!(!report.accepts_external_41i_result);
        assert!(report.binds_raw_payload_to_internal_41i_authorization);
        assert!(report.decodes_raw_payload_internally);
        assert!(report.derives_canonical_event_key_from_raw_payload);
        assert!(report.uses_canonical_event_key_as_replay_key);
        assert!(!report.uses_message_nonce_as_replay_key);
        assert!(!report.accepts_free_replay_key);
        assert!(!report.accepts_free_canonical_event_key);
        assert!(!report.accepts_free_decoded_payload);
        assert!(report.requires_authoritative_processed_registry_view);
        assert!(report.rejects_caller_supplied_processed_registry_view);
        assert!(report.rejects_unauthenticated_processed_registry_view);
        assert!(report.checks_abstract_processed_registry_view);
        assert!(report.returns_replay_eligibility_only);
        assert!(report.processed_marking_intent_only);
        assert!(!report.account_info_used);
        assert!(!report.sysvar_loading_enabled);
        assert!(!report.runtime_account_loading_enabled);
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
    fn establishes_replay_eligibility_for_unprocessed_authorized_event() {
        let raw_payload = raw_payload_bytes_with_event_key(&GUARDIAN_SET_ID, &CANONICAL_EVENT_KEY);
        let message_hash = compute_guardian_payload_hash(&raw_payload).expect("hash");

        let phase_41f_1 = established_phase_41f_result();
        let phase_41f_2 = established_phase_41f_result();
        let extraction_1 =
            established_extraction_result(&GUARDIAN_1_PUBLIC_KEY_BYTES, &message_hash);
        let extraction_2 =
            established_extraction_result(&GUARDIAN_2_PUBLIC_KEY_BYTES, &message_hash);
        let guardians = [guardian(1), guardian(2), guardian(3)];
        let processed: [[u8; 32]; 0] = [];

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

        let result = establish_replay_protection_eligibility(
            &attempts,
            &raw_payload,
            &GUARDIAN_SET_ID,
            authoritative_guardian_set(&GUARDIAN_SET_ID, 2, &guardians),
            authoritative_registry(&processed),
        )
        .expect("eligible");

        assert_eq!(result.canonical_event_key, CANONICAL_EVENT_KEY);
        assert!(result.internal_41i_quorum_established);
        assert!(result.raw_payload_decoded_internally);
        assert!(result.canonical_event_key_derived_from_raw_payload);
        assert!(result.replay_check_passed);
        assert!(result.processed_marking_eligible);
        assert!(result.processed_marking_intent);
        assert!(!result.replay_write_enabled);
        assert!(!result.processed_event_marking_enabled);
        assert!(!result.account_mutation_enabled);
        assert!(!result.runtime_account_loading_enabled);
        assert!(!result.sysvar_loading_enabled);
        assert!(!result.cpi_enabled);
        assert!(!result.live_route_enabled);
    }

    #[test]
    fn rejects_already_processed_canonical_event_key() {
        let raw_payload = raw_payload_bytes_with_event_key(&GUARDIAN_SET_ID, &CANONICAL_EVENT_KEY);
        let message_hash = compute_guardian_payload_hash(&raw_payload).expect("hash");

        let phase_41f_1 = established_phase_41f_result();
        let phase_41f_2 = established_phase_41f_result();
        let extraction_1 =
            established_extraction_result(&GUARDIAN_1_PUBLIC_KEY_BYTES, &message_hash);
        let extraction_2 =
            established_extraction_result(&GUARDIAN_2_PUBLIC_KEY_BYTES, &message_hash);
        let guardians = [guardian(1), guardian(2), guardian(3)];
        let processed = [CANONICAL_EVENT_KEY];

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

        let err = establish_replay_protection_eligibility(
            &attempts,
            &raw_payload,
            &GUARDIAN_SET_ID,
            authoritative_guardian_set(&GUARDIAN_SET_ID, 2, &guardians),
            authoritative_registry(&processed),
        )
        .expect_err("already processed");

        assert_eq!(
            err.kind,
            ReplayProtectionEligibilityErrorKind::CanonicalEventAlreadyProcessed
        );
        assert_eq!(err.canonical_event_key, Some(CANONICAL_EVENT_KEY));
        assert!(err.internal_41i_quorum_established);
        assert!(err.raw_payload_decoded_internally);
        assert!(err.canonical_event_key_derived_from_raw_payload);
        assert!(!err.replay_check_passed);
        assert!(!err.processed_marking_eligible);
        assert!(!err.processed_marking_intent);
        assert!(!err.replay_write_enabled);
        assert!(!err.processed_event_marking_enabled);
    }

    #[test]
    fn rejects_caller_supplied_processed_registry_view() {
        let raw_payload = raw_payload_bytes_with_event_key(&GUARDIAN_SET_ID, &CANONICAL_EVENT_KEY);
        let message_hash = compute_guardian_payload_hash(&raw_payload).expect("hash");

        let phase_41f_1 = established_phase_41f_result();
        let phase_41f_2 = established_phase_41f_result();
        let extraction_1 =
            established_extraction_result(&GUARDIAN_1_PUBLIC_KEY_BYTES, &message_hash);
        let extraction_2 =
            established_extraction_result(&GUARDIAN_2_PUBLIC_KEY_BYTES, &message_hash);
        let guardians = [guardian(1), guardian(2), guardian(3)];
        let processed: [[u8; 32]; 0] = [];

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

        let err = establish_replay_protection_eligibility(
            &attempts,
            &raw_payload,
            &GUARDIAN_SET_ID,
            authoritative_guardian_set(&GUARDIAN_SET_ID, 2, &guardians),
            AuthoritativeProcessedRegistryViewRef::caller_supplied_for_rejection(&processed),
        )
        .expect_err("caller registry rejected");

        assert_eq!(
            err.kind,
            ReplayProtectionEligibilityErrorKind::CallerSuppliedProcessedRegistryRejected
        );
        assert_eq!(
            err.processed_registry_source,
            ProcessedRegistryViewSource::CallerInstructionData
        );
        assert!(err.processed_registry_from_caller_instruction_data);
        assert_eq!(err.canonical_event_key, Some(CANONICAL_EVENT_KEY));
        assert!(!err.replay_write_enabled);
        assert!(!err.processed_event_marking_enabled);
    }

    #[test]
    fn rejects_unauthenticated_processed_registry_view() {
        let raw_payload = raw_payload_bytes_with_event_key(&GUARDIAN_SET_ID, &CANONICAL_EVENT_KEY);
        let message_hash = compute_guardian_payload_hash(&raw_payload).expect("hash");

        let phase_41f_1 = established_phase_41f_result();
        let phase_41f_2 = established_phase_41f_result();
        let extraction_1 =
            established_extraction_result(&GUARDIAN_1_PUBLIC_KEY_BYTES, &message_hash);
        let extraction_2 =
            established_extraction_result(&GUARDIAN_2_PUBLIC_KEY_BYTES, &message_hash);
        let guardians = [guardian(1), guardian(2), guardian(3)];
        let processed: [[u8; 32]; 0] = [];

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

        let err = establish_replay_protection_eligibility(
            &attempts,
            &raw_payload,
            &GUARDIAN_SET_ID,
            authoritative_guardian_set(&GUARDIAN_SET_ID, 2, &guardians),
            AuthoritativeProcessedRegistryViewRef::unauthenticated_for_rejection(&processed),
        )
        .expect_err("unauthenticated registry rejected");

        assert_eq!(
            err.kind,
            ReplayProtectionEligibilityErrorKind::UnauthenticatedProcessedRegistryRejected
        );
        assert_eq!(
            err.processed_registry_source,
            ProcessedRegistryViewSource::Unauthenticated
        );
        assert_eq!(err.canonical_event_key, Some(CANONICAL_EVENT_KEY));
        assert!(!err.replay_write_enabled);
        assert!(!err.processed_event_marking_enabled);
    }

    #[test]
    fn quorum_failure_cannot_reach_replay_eligibility() {
        let raw_payload = raw_payload_bytes_with_event_key(&GUARDIAN_SET_ID, &CANONICAL_EVENT_KEY);
        let message_hash = compute_guardian_payload_hash(&raw_payload).expect("hash");

        let phase_41f = established_phase_41f_result();
        let extraction = established_extraction_result(&UNKNOWN_PUBLIC_KEY_BYTES, &message_hash);
        let guardians = [guardian(1), guardian(2), guardian(3)];
        let processed: [[u8; 32]; 0] = [];

        let attempts = [GuardianQuorumAuthorizationAttempt {
            phase_41f_result: &phase_41f,
            extraction_result: &extraction,
        }];

        let err = establish_replay_protection_eligibility(
            &attempts,
            &raw_payload,
            &GUARDIAN_SET_ID,
            authoritative_guardian_set(&GUARDIAN_SET_ID, 1, &guardians),
            authoritative_registry(&processed),
        )
        .expect_err("quorum failure");

        assert_eq!(
            err.kind,
            ReplayProtectionEligibilityErrorKind::QuorumAuthorizationNotEstablished
        );
        assert!(err.quorum_authorization_error.is_some());
        assert_eq!(err.canonical_event_key, None);
        assert!(!err.internal_41i_quorum_established);
        assert!(!err.raw_payload_decoded_internally);
        assert!(!err.canonical_event_key_derived_from_raw_payload);
        assert!(!err.processed_marking_eligible);
        assert!(!err.processed_marking_intent);
    }

    #[test]
    fn raw_payload_is_bound_to_internal_41i_before_replay_key_derivation() {
        let authorized_payload =
            raw_payload_bytes_with_event_key(&GUARDIAN_SET_ID, &OTHER_CANONICAL_EVENT_KEY);
        let replay_payload =
            raw_payload_bytes_with_event_key(&GUARDIAN_SET_ID, &CANONICAL_EVENT_KEY);
        let authorized_message_hash =
            compute_guardian_payload_hash(&authorized_payload).expect("hash");

        let phase_41f_1 = established_phase_41f_result();
        let phase_41f_2 = established_phase_41f_result();
        let extraction_1 =
            established_extraction_result(&GUARDIAN_1_PUBLIC_KEY_BYTES, &authorized_message_hash);
        let extraction_2 =
            established_extraction_result(&GUARDIAN_2_PUBLIC_KEY_BYTES, &authorized_message_hash);
        let guardians = [guardian(1), guardian(2), guardian(3)];
        let processed: [[u8; 32]; 0] = [];

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

        let err = establish_replay_protection_eligibility(
            &attempts,
            &replay_payload,
            &GUARDIAN_SET_ID,
            authoritative_guardian_set(&GUARDIAN_SET_ID, 2, &guardians),
            authoritative_registry(&processed),
        )
        .expect_err("payload substitution rejected by internal 41I/41H.2");

        assert_eq!(
            err.kind,
            ReplayProtectionEligibilityErrorKind::QuorumAuthorizationNotEstablished
        );
        assert_eq!(err.canonical_event_key, None);
        assert!(!err.raw_payload_decoded_internally);
        assert!(!err.processed_marking_eligible);
        assert!(!err.processed_marking_intent);
    }
}
