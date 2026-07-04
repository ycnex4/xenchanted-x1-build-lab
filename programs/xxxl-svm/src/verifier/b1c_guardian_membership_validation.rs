use super::b1c_payload_hash_binding::B1CPayloadBoundEvidence;
use super::guardian_quorum::GuardianPublicKey;
use super::guardian_set_account_loading_boundary::{
    Phase41K2GuardianSetAccountLoadingResult, Phase41K2GuardianSetAccountLoadingStatus,
};

pub const PHASE_41K6_B1C_5_GUARDIAN_MEMBERSHIP_VALIDATION_PHASE: &str = "41K.6-B1C.5";
pub const PHASE_41K6_B1C_5_GUARDIAN_MEMBERSHIP_VALIDATION_VERSION: &str = "0.1.0";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B1CGuardianMembershipValidationStatus {
    Validated,
    Rejected,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B1CGuardianMembershipValidationRejectionKind {
    GuardianSetNotDecoded,
    MissingGuardianSetId,
    EmptyEvidence,
    UnauthorizedGuardian,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct B1CGuardianMembershipValidated {
    pub validated_signers: Vec<GuardianPublicKey>,
    pub guardian_set_id: [u8; 32],
    pub source_evidence_count: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct B1CGuardianMembershipValidationResult {
    pub status: B1CGuardianMembershipValidationStatus,
    pub rejection_kind: Option<B1CGuardianMembershipValidationRejectionKind>,
    pub rejected_evidence_index: Option<usize>,
    pub rejected_signer: Option<GuardianPublicKey>,
    pub validated: Option<B1CGuardianMembershipValidated>,
    pub source_guardian_count: usize,
    pub source_evidence_count: usize,
    pub validates_guardian_membership: bool,
    pub deduplicates_guardians: bool,
    pub counts_unique_guardians: bool,
    pub authorization_enabled: bool,
    pub processed_event_marking_enabled: bool,
    pub cpi_enabled: bool,
    pub live_route_enabled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct B1CGuardianMembershipValidationReport {
    pub phase: &'static str,
    pub version: &'static str,
    pub consumes_b1b_authoritative_guardian_set: bool,
    pub consumes_b1c4_payload_bound_evidence: bool,
    pub rejects_first_unauthorized_guardian: bool,
    pub validates_guardian_membership: bool,
    pub deduplicates_guardians: bool,
    pub counts_unique_guardians: bool,
    pub authorizes_handler_execution: bool,
    pub processed_event_marking_enabled: bool,
    pub cpi_enabled: bool,
    pub live_route_enabled: bool,
}

pub const B1C_GUARDIAN_MEMBERSHIP_VALIDATION_REPORT: B1CGuardianMembershipValidationReport =
    B1CGuardianMembershipValidationReport {
        phase: PHASE_41K6_B1C_5_GUARDIAN_MEMBERSHIP_VALIDATION_PHASE,
        version: PHASE_41K6_B1C_5_GUARDIAN_MEMBERSHIP_VALIDATION_VERSION,
        consumes_b1b_authoritative_guardian_set: true,
        consumes_b1c4_payload_bound_evidence: true,
        rejects_first_unauthorized_guardian: true,
        validates_guardian_membership: true,
        deduplicates_guardians: false,
        counts_unique_guardians: false,
        authorizes_handler_execution: false,
        processed_event_marking_enabled: false,
        cpi_enabled: false,
        live_route_enabled: false,
    };

pub fn b1c_guardian_membership_validation_report() -> &'static B1CGuardianMembershipValidationReport
{
    &B1C_GUARDIAN_MEMBERSHIP_VALIDATION_REPORT
}

pub fn validate_b1c_payload_bound_evidence_guardian_membership(
    guardian_set: &Phase41K2GuardianSetAccountLoadingResult,
    evidence: &[B1CPayloadBoundEvidence],
) -> B1CGuardianMembershipValidationResult {
    if guardian_set.status
        != Phase41K2GuardianSetAccountLoadingStatus::GuardianSetAccountDataDecoded
    {
        return rejected(
            B1CGuardianMembershipValidationRejectionKind::GuardianSetNotDecoded,
            None,
            None,
            guardian_set.guardian_count,
            evidence.len(),
        );
    }

    let Some(guardian_set_id) = guardian_set.guardian_set_id else {
        return rejected(
            B1CGuardianMembershipValidationRejectionKind::MissingGuardianSetId,
            None,
            None,
            guardian_set.guardian_count,
            evidence.len(),
        );
    };

    if evidence.is_empty() {
        return rejected(
            B1CGuardianMembershipValidationRejectionKind::EmptyEvidence,
            None,
            None,
            guardian_set.guardian_count,
            evidence.len(),
        );
    }

    let mut validated_signers = Vec::with_capacity(evidence.len());

    for (index, item) in evidence.iter().enumerate() {
        let signer = GuardianPublicKey(item.signer_public_key);

        if !guardian_set.guardians.contains(&signer) {
            return rejected(
                B1CGuardianMembershipValidationRejectionKind::UnauthorizedGuardian,
                Some(index),
                Some(signer),
                guardian_set.guardian_count,
                evidence.len(),
            );
        }

        validated_signers.push(signer);
    }

    B1CGuardianMembershipValidationResult {
        status: B1CGuardianMembershipValidationStatus::Validated,
        rejection_kind: None,
        rejected_evidence_index: None,
        rejected_signer: None,
        validated: Some(B1CGuardianMembershipValidated {
            validated_signers,
            guardian_set_id,
            source_evidence_count: evidence.len(),
        }),
        source_guardian_count: guardian_set.guardian_count,
        source_evidence_count: evidence.len(),
        validates_guardian_membership: true,
        deduplicates_guardians: false,
        counts_unique_guardians: false,
        authorization_enabled: false,
        processed_event_marking_enabled: false,
        cpi_enabled: false,
        live_route_enabled: false,
    }
}

fn rejected(
    rejection_kind: B1CGuardianMembershipValidationRejectionKind,
    rejected_evidence_index: Option<usize>,
    rejected_signer: Option<GuardianPublicKey>,
    source_guardian_count: usize,
    source_evidence_count: usize,
) -> B1CGuardianMembershipValidationResult {
    B1CGuardianMembershipValidationResult {
        status: B1CGuardianMembershipValidationStatus::Rejected,
        rejection_kind: Some(rejection_kind),
        rejected_evidence_index,
        rejected_signer,
        validated: None,
        source_guardian_count,
        source_evidence_count,
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
    use super::super::guardian_set_account_loading_boundary::Phase41K2GuardianSetAccountRejectionCase;
    use super::*;

    const GUARDIAN_SET_ID: [u8; 32] = [0x44; 32];

    fn guardian(byte: u8) -> GuardianPublicKey {
        GuardianPublicKey([byte; 32])
    }

    fn evidence(byte: u8) -> B1CPayloadBoundEvidence {
        B1CPayloadBoundEvidence {
            signer_public_key: [byte; 32],
            source_instruction_index: byte as usize,
            signed_message: [0xAA; 32],
            matches_expected_payload_hash: true,
        }
    }

    fn guardian_set(
        status: Phase41K2GuardianSetAccountLoadingStatus,
        guardians: Vec<GuardianPublicKey>,
    ) -> Phase41K2GuardianSetAccountLoadingResult {
        let decoded =
            status == Phase41K2GuardianSetAccountLoadingStatus::GuardianSetAccountDataDecoded;
        let guardian_count = guardians.len();

        Phase41K2GuardianSetAccountLoadingResult {
            status,
            rejection_case: if decoded {
                None
            } else {
                Some(Phase41K2GuardianSetAccountRejectionCase::InactiveOrDeprecatedGuardianSet)
            },
            account_data_len: 320,
            guardian_set_id: if decoded { Some(GUARDIAN_SET_ID) } else { None },
            threshold: if decoded { Some(2) } else { None },
            guardian_count,
            guardians,
            account_key: None,
            expected_account_key: None,
            account_owner: None,
            expected_program_id: None,
            pda_bump: None,
            active: decoded,
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
            source_marker_program_controlled_on_chain: decoded,
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

    fn decoded_guardian_set() -> Phase41K2GuardianSetAccountLoadingResult {
        guardian_set(
            Phase41K2GuardianSetAccountLoadingStatus::GuardianSetAccountDataDecoded,
            vec![guardian(1), guardian(2), guardian(3)],
        )
    }

    fn assert_execution_flags_false(result: &B1CGuardianMembershipValidationResult) {
        assert!(!result.deduplicates_guardians);
        assert!(!result.counts_unique_guardians);
        assert!(!result.authorization_enabled);
        assert!(!result.processed_event_marking_enabled);
        assert!(!result.cpi_enabled);
        assert!(!result.live_route_enabled);
    }

    #[test]
    fn report_documents_b1c5_scope() {
        let report = b1c_guardian_membership_validation_report();

        assert_eq!(
            report.phase,
            PHASE_41K6_B1C_5_GUARDIAN_MEMBERSHIP_VALIDATION_PHASE
        );
        assert!(report.consumes_b1b_authoritative_guardian_set);
        assert!(report.consumes_b1c4_payload_bound_evidence);
        assert!(report.rejects_first_unauthorized_guardian);
        assert!(report.validates_guardian_membership);
        assert!(!report.deduplicates_guardians);
        assert!(!report.counts_unique_guardians);
        assert!(!report.authorizes_handler_execution);
        assert!(!report.processed_event_marking_enabled);
        assert!(!report.cpi_enabled);
        assert!(!report.live_route_enabled);
    }

    #[test]
    fn all_signers_are_guardians_passes() {
        let result = validate_b1c_payload_bound_evidence_guardian_membership(
            &decoded_guardian_set(),
            &[evidence(1), evidence(2)],
        );

        assert_eq!(
            result.status,
            B1CGuardianMembershipValidationStatus::Validated
        );
        assert_eq!(result.rejection_kind, None);
        assert!(result.validates_guardian_membership);
        let validated = result.validated.as_ref().expect("validated signers");
        assert_eq!(validated.guardian_set_id, GUARDIAN_SET_ID);
        assert_eq!(validated.source_evidence_count, 2);
        assert_eq!(validated.validated_signers, vec![guardian(1), guardian(2)]);
        assert_execution_flags_false(&result);
    }

    #[test]
    fn first_unauthorized_signer_rejects() {
        let result = validate_b1c_payload_bound_evidence_guardian_membership(
            &decoded_guardian_set(),
            &[evidence(9), evidence(1)],
        );

        assert_eq!(
            result.status,
            B1CGuardianMembershipValidationStatus::Rejected
        );
        assert_eq!(
            result.rejection_kind,
            Some(B1CGuardianMembershipValidationRejectionKind::UnauthorizedGuardian)
        );
        assert_eq!(result.rejected_evidence_index, Some(0));
        assert_eq!(result.rejected_signer, Some(guardian(9)));
        assert!(!result.validates_guardian_membership);
        assert_execution_flags_false(&result);
    }

    #[test]
    fn later_unauthorized_signer_rejects() {
        let result = validate_b1c_payload_bound_evidence_guardian_membership(
            &decoded_guardian_set(),
            &[evidence(1), evidence(9)],
        );

        assert_eq!(
            result.status,
            B1CGuardianMembershipValidationStatus::Rejected
        );
        assert_eq!(
            result.rejection_kind,
            Some(B1CGuardianMembershipValidationRejectionKind::UnauthorizedGuardian)
        );
        assert_eq!(result.rejected_evidence_index, Some(1));
        assert_eq!(result.rejected_signer, Some(guardian(9)));
        assert_execution_flags_false(&result);
    }

    #[test]
    fn empty_evidence_rejects() {
        let result =
            validate_b1c_payload_bound_evidence_guardian_membership(&decoded_guardian_set(), &[]);

        assert_eq!(
            result.status,
            B1CGuardianMembershipValidationStatus::Rejected
        );
        assert_eq!(
            result.rejection_kind,
            Some(B1CGuardianMembershipValidationRejectionKind::EmptyEvidence)
        );
        assert_execution_flags_false(&result);
    }

    #[test]
    fn duplicate_guardian_signatures_are_accepted_but_not_deduplicated_here() {
        let result = validate_b1c_payload_bound_evidence_guardian_membership(
            &decoded_guardian_set(),
            &[evidence(1), evidence(1), evidence(2)],
        );

        assert_eq!(
            result.status,
            B1CGuardianMembershipValidationStatus::Validated
        );
        let validated = result.validated.as_ref().expect("validated signers");
        assert_eq!(
            validated.validated_signers,
            vec![guardian(1), guardian(1), guardian(2)]
        );
        assert!(!result.deduplicates_guardians);
        assert!(!result.counts_unique_guardians);
        assert_execution_flags_false(&result);
    }

    #[test]
    fn rejected_guardian_set_rejects() {
        let source = guardian_set(
            Phase41K2GuardianSetAccountLoadingStatus::GuardianSetAccountDataRejected,
            Vec::new(),
        );

        let result =
            validate_b1c_payload_bound_evidence_guardian_membership(&source, &[evidence(1)]);

        assert_eq!(
            result.status,
            B1CGuardianMembershipValidationStatus::Rejected
        );
        assert_eq!(
            result.rejection_kind,
            Some(B1CGuardianMembershipValidationRejectionKind::GuardianSetNotDecoded)
        );
        assert_execution_flags_false(&result);
    }
}
