use super::b1c_connect_ed25519_evidence_adapter::{
    connect_b1c_checked_prior_loading_to_parsed_ed25519_evidence, B1CConnectedEd25519EvidenceStatus,
};
use super::b1c_guardian_membership_validation::{
    validate_b1c_payload_bound_evidence_guardian_membership, B1CGuardianMembershipValidationStatus,
};
use super::b1c_payload_hash_binding::{
    bind_b1c_parsed_ed25519_evidence_to_expected_payload_hash, B1CAuthorizationPayloadContext,
    B1CPayloadHashBindingStatus,
};
use super::b1c_quorum_counting::{count_b1c_validated_guardian_quorum, B1CQuorumCountingStatus};
use super::checked_prior_instruction_loading_runtime_boundary::Phase41D3_2_2CheckedPriorInstructionLoadingResult;
use super::guardian_set_account_loading_boundary::{
    Phase41K2GuardianSetAccountLoadingResult, Phase41K2GuardianSetAccountLoadingStatus,
};

pub const PHASE_41K6_B1C_7_HANDLER_AUTHORIZATION_PHASE: &str = "41K.6-B1C.7";
pub const PHASE_41K6_B1C_7_HANDLER_AUTHORIZATION_VERSION: &str = "0.1.0";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B1C7HandlerAuthorizationStatus {
    Authorized,
    Rejected,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B1C7HandlerAuthorizationRejectionKind {
    GuardianSetNotDecoded,
    EvidenceConnectionRejected,
    PayloadHashBindingRejected,
    GuardianMembershipRejected,
    QuorumRejected,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct B1C7HandlerAuthorizationResult {
    pub status: B1C7HandlerAuthorizationStatus,
    pub rejection_kind: Option<B1C7HandlerAuthorizationRejectionKind>,
    pub parsed_evidence_count: usize,
    pub payload_bound_evidence_count: usize,
    pub membership_validated_signer_count: usize,
    pub unique_guardian_count: usize,
    pub threshold: Option<u8>,
    pub fail_fast_before_mutation: bool,
    pub evidence_from_prior_ed25519_instructions: bool,
    pub payload_hash_bound: bool,
    pub guardian_membership_validated: bool,
    pub quorum_met: bool,
    pub authorization_enabled: bool,
    pub processed_event_marking_enabled: bool,
    pub cpi_enabled: bool,
    pub live_route_enabled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct B1C7HandlerAuthorizationBoundaryReport {
    pub phase: &'static str,
    pub version: &'static str,
    pub wires_b1b_guardian_set: bool,
    pub wires_b1c3_connect: bool,
    pub wires_b1c4_payload_binding: bool,
    pub wires_b1c5_membership: bool,
    pub wires_b1c6_quorum: bool,
    pub fail_fast_before_mutation: bool,
    pub authorization_enabled_on_success: bool,
    pub processed_event_marking_enabled: bool,
    pub cpi_enabled: bool,
    pub live_route_enabled: bool,
}

pub const B1C7_HANDLER_AUTHORIZATION_BOUNDARY_REPORT: B1C7HandlerAuthorizationBoundaryReport =
    B1C7HandlerAuthorizationBoundaryReport {
        phase: PHASE_41K6_B1C_7_HANDLER_AUTHORIZATION_PHASE,
        version: PHASE_41K6_B1C_7_HANDLER_AUTHORIZATION_VERSION,
        wires_b1b_guardian_set: true,
        wires_b1c3_connect: true,
        wires_b1c4_payload_binding: true,
        wires_b1c5_membership: true,
        wires_b1c6_quorum: true,
        fail_fast_before_mutation: true,
        authorization_enabled_on_success: true,
        processed_event_marking_enabled: false,
        cpi_enabled: false,
        live_route_enabled: false,
    };

pub fn b1c7_handler_authorization_boundary_report(
) -> &'static B1C7HandlerAuthorizationBoundaryReport {
    &B1C7_HANDLER_AUTHORIZATION_BOUNDARY_REPORT
}

pub fn establish_b1c7_handler_authorization_before_mark_and_mint(
    guardian_set: &Phase41K2GuardianSetAccountLoadingResult,
    checked_prior_loading: &Phase41D3_2_2CheckedPriorInstructionLoadingResult,
    payload_context: &B1CAuthorizationPayloadContext,
) -> B1C7HandlerAuthorizationResult {
    if guardian_set.status
        != Phase41K2GuardianSetAccountLoadingStatus::GuardianSetAccountDataDecoded
    {
        return rejected(B1C7HandlerAuthorizationRejectionKind::GuardianSetNotDecoded);
    }

    let connected =
        connect_b1c_checked_prior_loading_to_parsed_ed25519_evidence(checked_prior_loading);

    if connected.status != B1CConnectedEd25519EvidenceStatus::ParsedPriorEd25519Evidence {
        return rejected(B1C7HandlerAuthorizationRejectionKind::EvidenceConnectionRejected);
    }

    let payload_binding = bind_b1c_parsed_ed25519_evidence_to_expected_payload_hash(
        payload_context,
        &connected.parsed_evidence,
    );

    if payload_binding.status != B1CPayloadHashBindingStatus::Bound {
        return rejected_with_counts(
            B1C7HandlerAuthorizationRejectionKind::PayloadHashBindingRejected,
            connected.parsed_evidence_count,
            0,
            0,
            0,
            guardian_set.threshold,
        );
    }

    let membership = validate_b1c_payload_bound_evidence_guardian_membership(
        guardian_set,
        &payload_binding.bound_evidence,
    );

    if membership.status != B1CGuardianMembershipValidationStatus::Validated {
        return rejected_with_counts(
            B1C7HandlerAuthorizationRejectionKind::GuardianMembershipRejected,
            connected.parsed_evidence_count,
            payload_binding.bound_evidence_count,
            0,
            0,
            guardian_set.threshold,
        );
    }

    let validated = membership
        .validated
        .as_ref()
        .expect("validated membership result must include validated signers");

    let quorum = count_b1c_validated_guardian_quorum(guardian_set, validated);

    if quorum.status != B1CQuorumCountingStatus::QuorumMet {
        return rejected_with_counts(
            B1C7HandlerAuthorizationRejectionKind::QuorumRejected,
            connected.parsed_evidence_count,
            payload_binding.bound_evidence_count,
            validated.validated_signers.len(),
            quorum.unique_guardian_count,
            guardian_set.threshold,
        );
    }

    B1C7HandlerAuthorizationResult {
        status: B1C7HandlerAuthorizationStatus::Authorized,
        rejection_kind: None,
        parsed_evidence_count: connected.parsed_evidence_count,
        payload_bound_evidence_count: payload_binding.bound_evidence_count,
        membership_validated_signer_count: validated.validated_signers.len(),
        unique_guardian_count: quorum.unique_guardian_count,
        threshold: guardian_set.threshold,
        fail_fast_before_mutation: true,
        evidence_from_prior_ed25519_instructions: true,
        payload_hash_bound: true,
        guardian_membership_validated: true,
        quorum_met: true,
        authorization_enabled: true,
        processed_event_marking_enabled: false,
        cpi_enabled: false,
        live_route_enabled: false,
    }
}

fn rejected(kind: B1C7HandlerAuthorizationRejectionKind) -> B1C7HandlerAuthorizationResult {
    rejected_with_counts(kind, 0, 0, 0, 0, None)
}

fn rejected_with_counts(
    kind: B1C7HandlerAuthorizationRejectionKind,
    parsed_evidence_count: usize,
    payload_bound_evidence_count: usize,
    membership_validated_signer_count: usize,
    unique_guardian_count: usize,
    threshold: Option<u8>,
) -> B1C7HandlerAuthorizationResult {
    B1C7HandlerAuthorizationResult {
        status: B1C7HandlerAuthorizationStatus::Rejected,
        rejection_kind: Some(kind),
        parsed_evidence_count,
        payload_bound_evidence_count,
        membership_validated_signer_count,
        unique_guardian_count,
        threshold,
        fail_fast_before_mutation: true,
        evidence_from_prior_ed25519_instructions: false,
        payload_hash_bound: false,
        guardian_membership_validated: false,
        quorum_met: false,
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
    use crate::verifier::b1c_ed25519_evidence_parser::{
        ED25519_PUBLIC_KEY_LEN, ED25519_SIGNATURE_LEN, ED25519_SINGLE_SIGNATURE_HEADER_LEN,
    };
    use crate::verifier::b1c_payload_hash_binding::compute_b1c_expected_authorization_payload_hash;
    use crate::verifier::checked_prior_instruction_loading_runtime_boundary::{
        Phase41D3_2_2CheckedPriorInstructionLoadingResult,
        Phase41D3_2_2CheckedPriorInstructionLoadingStatus, Phase41D3_2_2LoadedPriorInstruction,
    };
    use crate::verifier::guardian_quorum::GuardianPublicKey;
    use crate::verifier::guardian_set_account_loading_boundary::{
        Phase41K2GuardianSetAccountLoadingResult, Phase41K2GuardianSetAccountLoadingStatus,
    };

    const GUARDIAN_SET_ID: [u8; 32] = [0x44; 32];

    fn guardian(byte: u8) -> GuardianPublicKey {
        GuardianPublicKey([byte; 32])
    }

    fn payload_context() -> B1CAuthorizationPayloadContext {
        B1CAuthorizationPayloadContext {
            processed_event: Pubkey::new_from_array([1; 32]),
            route_id: [0x33; 32],
            mint: Pubkey::new_from_array([2; 32]),
            recipient: Pubkey::new_from_array([3; 32]),
            amount: 123,
            guardian_set_id: GUARDIAN_SET_ID,
        }
    }

    fn guardian_set(
        threshold: u8,
        guardians: Vec<GuardianPublicKey>,
    ) -> Phase41K2GuardianSetAccountLoadingResult {
        let guardian_count = guardians.len();

        Phase41K2GuardianSetAccountLoadingResult {
            status: Phase41K2GuardianSetAccountLoadingStatus::GuardianSetAccountDataDecoded,
            rejection_case: None,
            account_data_len: 320,
            guardian_set_id: Some(GUARDIAN_SET_ID),
            threshold: Some(threshold),
            guardian_count,
            guardians,
            account_key: None,
            expected_account_key: None,
            account_owner: None,
            expected_program_id: None,
            pda_bump: None,
            active: true,
            discriminator_checked: true,
            zero_discriminator_rejected: true,
            wrong_discriminator_rejected: true,
            schema_version_checked: true,
            threshold_checked: true,
            guardian_count_checked: true,
            duplicate_guardian_public_key_rejected: true,
            guardian_set_id_checked: true,
            active_status_checked: true,
            guardian_set_account_readonly: true,
            guardian_set_account_non_signer: true,
            source_marker_program_controlled_on_chain: true,
            caller_supplied_guardian_set_rejected: true,
            account_info_used: true,
            account_key_checked: true,
            account_owner_checked: true,
            pda_checked: true,
            authoritative_wrapper_constructed: false,
            guardian_set_runtime_loading_enabled: true,
            processed_registry_runtime_loading_enabled: false,
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

    fn checked_prior_loading(
        signers: &[u8],
        message: [u8; 32],
    ) -> Phase41D3_2_2CheckedPriorInstructionLoadingResult {
        let loaded_prior_instructions = signers
            .iter()
            .enumerate()
            .map(|(i, signer)| Phase41D3_2_2LoadedPriorInstruction {
                instruction_index: i,
                instruction: Instruction {
                    program_id: ed25519_program::id(),
                    accounts: Vec::new(),
                    data: ed25519_instruction_data(&message, *signer),
                },
                loaded_instruction_is_runtime_data_only: true,
                is_evidence: false,
                authorizes_execution: false,
            })
            .collect::<Vec<_>>();

        Phase41D3_2_2CheckedPriorInstructionLoadingResult {
            status:
                Phase41D3_2_2CheckedPriorInstructionLoadingStatus::CheckedPriorInstructionsLoaded,
            rejection_case: None,
            current_instruction_index: Some(signers.len() + 1),
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
            prefilter_enabled: false,
            phase_41c3_descriptor_construction_enabled: false,
            locates_prior_ed25519_instruction: false,
            accepts_verification_evidence: false,
            authorizes_execution: false,
            mutates_runtime_state: false,
        }
    }

    fn ed25519_instruction_data(message: &[u8; 32], signer: u8) -> Vec<u8> {
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

        data.extend_from_slice(&[0x55; ED25519_SIGNATURE_LEN]);
        data.extend_from_slice(&[signer; ED25519_PUBLIC_KEY_LEN]);
        data.extend_from_slice(message);

        data
    }

    fn assert_no_mutation_flags(result: &B1C7HandlerAuthorizationResult) {
        assert!(!result.processed_event_marking_enabled);
        assert!(!result.cpi_enabled);
        assert!(!result.live_route_enabled);
    }

    #[test]
    fn report_documents_b1c7_authorization_boundary_scope() {
        let report = b1c7_handler_authorization_boundary_report();

        assert_eq!(report.phase, PHASE_41K6_B1C_7_HANDLER_AUTHORIZATION_PHASE);
        assert!(report.wires_b1b_guardian_set);
        assert!(report.wires_b1c3_connect);
        assert!(report.wires_b1c4_payload_binding);
        assert!(report.wires_b1c5_membership);
        assert!(report.wires_b1c6_quorum);
        assert!(report.fail_fast_before_mutation);
        assert!(report.authorization_enabled_on_success);
        assert!(!report.processed_event_marking_enabled);
        assert!(!report.cpi_enabled);
        assert!(!report.live_route_enabled);
    }

    #[test]
    fn full_pipeline_authorizes_before_mutation_when_quorum_met() {
        let context = payload_context();
        let hash = compute_b1c_expected_authorization_payload_hash(&context);

        let result = establish_b1c7_handler_authorization_before_mark_and_mint(
            &guardian_set(2, vec![guardian(1), guardian(2), guardian(3)]),
            &checked_prior_loading(&[1, 2], hash),
            &context,
        );

        assert_eq!(result.status, B1C7HandlerAuthorizationStatus::Authorized);
        assert_eq!(result.rejection_kind, None);
        assert_eq!(result.parsed_evidence_count, 2);
        assert_eq!(result.payload_bound_evidence_count, 2);
        assert_eq!(result.membership_validated_signer_count, 2);
        assert_eq!(result.unique_guardian_count, 2);
        assert_eq!(result.threshold, Some(2));
        assert!(result.fail_fast_before_mutation);
        assert!(result.evidence_from_prior_ed25519_instructions);
        assert!(result.payload_hash_bound);
        assert!(result.guardian_membership_validated);
        assert!(result.quorum_met);
        assert!(result.authorization_enabled);
        assert_no_mutation_flags(&result);
    }

    #[test]
    fn payload_hash_mismatch_rejects_before_mutation() {
        let context = payload_context();
        let wrong_hash = [0x99; 32];

        let result = establish_b1c7_handler_authorization_before_mark_and_mint(
            &guardian_set(1, vec![guardian(1)]),
            &checked_prior_loading(&[1], wrong_hash),
            &context,
        );

        assert_eq!(result.status, B1C7HandlerAuthorizationStatus::Rejected);
        assert_eq!(
            result.rejection_kind,
            Some(B1C7HandlerAuthorizationRejectionKind::PayloadHashBindingRejected)
        );
        assert!(!result.authorization_enabled);
        assert_no_mutation_flags(&result);
    }

    #[test]
    fn unauthorized_guardian_rejects_before_mutation() {
        let context = payload_context();
        let hash = compute_b1c_expected_authorization_payload_hash(&context);

        let result = establish_b1c7_handler_authorization_before_mark_and_mint(
            &guardian_set(1, vec![guardian(1)]),
            &checked_prior_loading(&[9], hash),
            &context,
        );

        assert_eq!(result.status, B1C7HandlerAuthorizationStatus::Rejected);
        assert_eq!(
            result.rejection_kind,
            Some(B1C7HandlerAuthorizationRejectionKind::GuardianMembershipRejected)
        );
        assert!(!result.authorization_enabled);
        assert_no_mutation_flags(&result);
    }

    #[test]
    fn duplicate_signer_cannot_fake_quorum() {
        let context = payload_context();
        let hash = compute_b1c_expected_authorization_payload_hash(&context);

        let result = establish_b1c7_handler_authorization_before_mark_and_mint(
            &guardian_set(2, vec![guardian(1), guardian(2)]),
            &checked_prior_loading(&[1, 1], hash),
            &context,
        );

        assert_eq!(result.status, B1C7HandlerAuthorizationStatus::Rejected);
        assert_eq!(
            result.rejection_kind,
            Some(B1C7HandlerAuthorizationRejectionKind::QuorumRejected)
        );
        assert_eq!(result.unique_guardian_count, 1);
        assert!(!result.authorization_enabled);
        assert_no_mutation_flags(&result);
    }

    #[test]
    fn no_prior_ed25519_evidence_rejects_before_mutation() {
        let context = payload_context();

        let result = establish_b1c7_handler_authorization_before_mark_and_mint(
            &guardian_set(1, vec![guardian(1)]),
            &checked_prior_loading(&[], [0; 32]),
            &context,
        );

        assert_eq!(result.status, B1C7HandlerAuthorizationStatus::Rejected);
        assert_eq!(
            result.rejection_kind,
            Some(B1C7HandlerAuthorizationRejectionKind::EvidenceConnectionRejected)
        );
        assert!(!result.authorization_enabled);
        assert_no_mutation_flags(&result);
    }
}
