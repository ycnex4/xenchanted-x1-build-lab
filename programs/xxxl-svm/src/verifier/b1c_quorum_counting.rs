use super::b1c_guardian_membership_validation::B1CGuardianMembershipValidated;
use super::guardian_quorum::GuardianPublicKey;
use super::guardian_set_account_loading_boundary::{
    Phase41K2GuardianSetAccountLoadingResult, Phase41K2GuardianSetAccountLoadingStatus,
};

pub const PHASE_41K6_B1C_6_QUORUM_COUNTING_PHASE: &str = "41K.6-B1C.6";
pub const PHASE_41K6_B1C_6_QUORUM_COUNTING_VERSION: &str = "0.1.0";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B1CQuorumCountingStatus {
    QuorumMet,
    QuorumNotMet,
    Rejected,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B1CQuorumCountingRejectionKind {
    GuardianSetNotDecoded,
    MissingGuardianSetId,
    MissingThreshold,
    GuardianSetIdMismatch,
    EmptyValidatedSigners,
    ThresholdZero,
    ThresholdExceedsGuardianCount,
    QuorumNotMet,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct B1CQuorumCountingResult {
    pub status: B1CQuorumCountingStatus,
    pub rejection_kind: Option<B1CQuorumCountingRejectionKind>,
    pub guardian_set_id: Option<[u8; 32]>,
    pub threshold: Option<u8>,
    pub guardian_count: usize,
    pub source_signer_count: usize,
    pub unique_guardian_count: usize,
    pub unique_guardians: Vec<GuardianPublicKey>,
    pub deduplicates_guardians: bool,
    pub counts_unique_guardians: bool,
    pub quorum_met: bool,
    pub authorization_enabled: bool,
    pub processed_event_marking_enabled: bool,
    pub cpi_enabled: bool,
    pub live_route_enabled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct B1CQuorumCountingReport {
    pub phase: &'static str,
    pub version: &'static str,
    pub consumes_b1c5_validated_signers: bool,
    pub consumes_b1b_guardian_threshold: bool,
    pub deduplicates_guardians: bool,
    pub counts_unique_guardians: bool,
    pub enables_authorization: bool,
    pub processed_event_marking_enabled: bool,
    pub cpi_enabled: bool,
    pub live_route_enabled: bool,
}

pub const B1C_QUORUM_COUNTING_REPORT: B1CQuorumCountingReport = B1CQuorumCountingReport {
    phase: PHASE_41K6_B1C_6_QUORUM_COUNTING_PHASE,
    version: PHASE_41K6_B1C_6_QUORUM_COUNTING_VERSION,
    consumes_b1c5_validated_signers: true,
    consumes_b1b_guardian_threshold: true,
    deduplicates_guardians: true,
    counts_unique_guardians: true,
    enables_authorization: false,
    processed_event_marking_enabled: false,
    cpi_enabled: false,
    live_route_enabled: false,
};

pub fn b1c_quorum_counting_report() -> &'static B1CQuorumCountingReport {
    &B1C_QUORUM_COUNTING_REPORT
}

pub fn count_b1c_validated_guardian_quorum(
    guardian_set: &Phase41K2GuardianSetAccountLoadingResult,
    validated: &B1CGuardianMembershipValidated,
) -> B1CQuorumCountingResult {
    if guardian_set.status
        != Phase41K2GuardianSetAccountLoadingStatus::GuardianSetAccountDataDecoded
    {
        return rejected(
            B1CQuorumCountingRejectionKind::GuardianSetNotDecoded,
            guardian_set.guardian_set_id,
            guardian_set.threshold,
            guardian_set.guardian_count,
            validated.validated_signers.len(),
            Vec::new(),
            false,
        );
    }

    let Some(guardian_set_id) = guardian_set.guardian_set_id else {
        return rejected(
            B1CQuorumCountingRejectionKind::MissingGuardianSetId,
            None,
            guardian_set.threshold,
            guardian_set.guardian_count,
            validated.validated_signers.len(),
            Vec::new(),
            false,
        );
    };

    let Some(threshold) = guardian_set.threshold else {
        return rejected(
            B1CQuorumCountingRejectionKind::MissingThreshold,
            Some(guardian_set_id),
            None,
            guardian_set.guardian_count,
            validated.validated_signers.len(),
            Vec::new(),
            false,
        );
    };

    if guardian_set_id != validated.guardian_set_id {
        return rejected(
            B1CQuorumCountingRejectionKind::GuardianSetIdMismatch,
            Some(guardian_set_id),
            Some(threshold),
            guardian_set.guardian_count,
            validated.validated_signers.len(),
            Vec::new(),
            false,
        );
    }

    if validated.validated_signers.is_empty() {
        return rejected(
            B1CQuorumCountingRejectionKind::EmptyValidatedSigners,
            Some(guardian_set_id),
            Some(threshold),
            guardian_set.guardian_count,
            0,
            Vec::new(),
            false,
        );
    }

    if threshold == 0 {
        return rejected(
            B1CQuorumCountingRejectionKind::ThresholdZero,
            Some(guardian_set_id),
            Some(threshold),
            guardian_set.guardian_count,
            validated.validated_signers.len(),
            Vec::new(),
            false,
        );
    }

    if usize::from(threshold) > guardian_set.guardian_count {
        return rejected(
            B1CQuorumCountingRejectionKind::ThresholdExceedsGuardianCount,
            Some(guardian_set_id),
            Some(threshold),
            guardian_set.guardian_count,
            validated.validated_signers.len(),
            Vec::new(),
            false,
        );
    }

    let mut unique_guardians = Vec::new();

    for signer in validated.validated_signers.iter() {
        if !unique_guardians.contains(signer) {
            unique_guardians.push(*signer);
        }
    }

    if unique_guardians.len() < usize::from(threshold) {
        return rejected(
            B1CQuorumCountingRejectionKind::QuorumNotMet,
            Some(guardian_set_id),
            Some(threshold),
            guardian_set.guardian_count,
            validated.validated_signers.len(),
            unique_guardians,
            true,
        );
    }

    B1CQuorumCountingResult {
        status: B1CQuorumCountingStatus::QuorumMet,
        rejection_kind: None,
        guardian_set_id: Some(guardian_set_id),
        threshold: Some(threshold),
        guardian_count: guardian_set.guardian_count,
        source_signer_count: validated.validated_signers.len(),
        unique_guardian_count: unique_guardians.len(),
        unique_guardians,
        deduplicates_guardians: true,
        counts_unique_guardians: true,
        quorum_met: true,
        authorization_enabled: false,
        processed_event_marking_enabled: false,
        cpi_enabled: false,
        live_route_enabled: false,
    }
}

fn rejected(
    rejection_kind: B1CQuorumCountingRejectionKind,
    guardian_set_id: Option<[u8; 32]>,
    threshold: Option<u8>,
    guardian_count: usize,
    source_signer_count: usize,
    unique_guardians: Vec<GuardianPublicKey>,
    counted: bool,
) -> B1CQuorumCountingResult {
    B1CQuorumCountingResult {
        status: if rejection_kind == B1CQuorumCountingRejectionKind::QuorumNotMet {
            B1CQuorumCountingStatus::QuorumNotMet
        } else {
            B1CQuorumCountingStatus::Rejected
        },
        rejection_kind: Some(rejection_kind),
        guardian_set_id,
        threshold,
        guardian_count,
        source_signer_count,
        unique_guardian_count: unique_guardians.len(),
        unique_guardians,
        deduplicates_guardians: counted,
        counts_unique_guardians: counted,
        quorum_met: false,
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

    const GUARDIAN_SET_ID: [u8; 32] = [0x66; 32];
    const OTHER_GUARDIAN_SET_ID: [u8; 32] = [0x67; 32];

    fn guardian(byte: u8) -> GuardianPublicKey {
        GuardianPublicKey([byte; 32])
    }

    fn validated(signers: Vec<GuardianPublicKey>) -> B1CGuardianMembershipValidated {
        B1CGuardianMembershipValidated {
            validated_signers: signers.clone(),
            guardian_set_id: GUARDIAN_SET_ID,
            source_evidence_count: signers.len(),
        }
    }

    fn guardian_set(
        threshold: Option<u8>,
        guardian_count: usize,
        status: Phase41K2GuardianSetAccountLoadingStatus,
        guardian_set_id: Option<[u8; 32]>,
    ) -> Phase41K2GuardianSetAccountLoadingResult {
        let decoded =
            status == Phase41K2GuardianSetAccountLoadingStatus::GuardianSetAccountDataDecoded;

        Phase41K2GuardianSetAccountLoadingResult {
            status,
            rejection_case: if decoded {
                None
            } else {
                Some(Phase41K2GuardianSetAccountRejectionCase::InactiveOrDeprecatedGuardianSet)
            },
            account_data_len: 320,
            guardian_set_id,
            threshold,
            guardian_count,
            guardians: vec![guardian(1), guardian(2), guardian(3)],
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

    fn decoded_guardian_set(threshold: u8) -> Phase41K2GuardianSetAccountLoadingResult {
        guardian_set(
            Some(threshold),
            3,
            Phase41K2GuardianSetAccountLoadingStatus::GuardianSetAccountDataDecoded,
            Some(GUARDIAN_SET_ID),
        )
    }

    fn assert_no_execution(result: &B1CQuorumCountingResult) {
        assert!(!result.authorization_enabled);
        assert!(!result.processed_event_marking_enabled);
        assert!(!result.cpi_enabled);
        assert!(!result.live_route_enabled);
    }

    #[test]
    fn report_documents_b1c6_scope() {
        let report = b1c_quorum_counting_report();

        assert_eq!(report.phase, PHASE_41K6_B1C_6_QUORUM_COUNTING_PHASE);
        assert!(report.consumes_b1c5_validated_signers);
        assert!(report.consumes_b1b_guardian_threshold);
        assert!(report.deduplicates_guardians);
        assert!(report.counts_unique_guardians);
        assert!(!report.enables_authorization);
        assert!(!report.processed_event_marking_enabled);
        assert!(!report.cpi_enabled);
        assert!(!report.live_route_enabled);
    }

    #[test]
    fn single_guardian_signing_once_counts_one() {
        let result = count_b1c_validated_guardian_quorum(
            &decoded_guardian_set(1),
            &validated(vec![guardian(1)]),
        );

        assert_eq!(result.status, B1CQuorumCountingStatus::QuorumMet);
        assert_eq!(result.unique_guardian_count, 1);
        assert_eq!(result.source_signer_count, 1);
        assert_eq!(result.unique_guardians, vec![guardian(1)]);
        assert_no_execution(&result);
    }

    #[test]
    fn same_guardian_signing_twice_counts_once() {
        let result = count_b1c_validated_guardian_quorum(
            &decoded_guardian_set(1),
            &validated(vec![guardian(1), guardian(1)]),
        );

        assert_eq!(result.status, B1CQuorumCountingStatus::QuorumMet);
        assert_eq!(result.source_signer_count, 2);
        assert_eq!(result.unique_guardian_count, 1);
        assert_eq!(result.unique_guardians, vec![guardian(1)]);
        assert!(result.deduplicates_guardians);
        assert!(result.counts_unique_guardians);
        assert_no_execution(&result);
    }

    #[test]
    fn two_different_guardians_count_two() {
        let result = count_b1c_validated_guardian_quorum(
            &decoded_guardian_set(2),
            &validated(vec![guardian(1), guardian(2)]),
        );

        assert_eq!(result.status, B1CQuorumCountingStatus::QuorumMet);
        assert_eq!(result.unique_guardian_count, 2);
        assert_eq!(result.unique_guardians, vec![guardian(1), guardian(2)]);
        assert_no_execution(&result);
    }

    #[test]
    fn threshold_exactly_met() {
        let result = count_b1c_validated_guardian_quorum(
            &decoded_guardian_set(2),
            &validated(vec![guardian(1), guardian(2)]),
        );

        assert_eq!(result.status, B1CQuorumCountingStatus::QuorumMet);
        assert_eq!(result.threshold, Some(2));
        assert!(result.quorum_met);
        assert!(!result.authorization_enabled);
    }

    #[test]
    fn threshold_exceeded() {
        let result = count_b1c_validated_guardian_quorum(
            &decoded_guardian_set(2),
            &validated(vec![guardian(1), guardian(2), guardian(3)]),
        );

        assert_eq!(result.status, B1CQuorumCountingStatus::QuorumMet);
        assert_eq!(result.unique_guardian_count, 3);
        assert!(result.quorum_met);
        assert_no_execution(&result);
    }

    #[test]
    fn threshold_not_met_rejects_without_authorization() {
        let result = count_b1c_validated_guardian_quorum(
            &decoded_guardian_set(2),
            &validated(vec![guardian(1), guardian(1)]),
        );

        assert_eq!(result.status, B1CQuorumCountingStatus::QuorumNotMet);
        assert_eq!(
            result.rejection_kind,
            Some(B1CQuorumCountingRejectionKind::QuorumNotMet)
        );
        assert_eq!(result.source_signer_count, 2);
        assert_eq!(result.unique_guardian_count, 1);
        assert!(!result.quorum_met);
        assert_no_execution(&result);
    }

    #[test]
    fn empty_validated_signers_rejects() {
        let result =
            count_b1c_validated_guardian_quorum(&decoded_guardian_set(1), &validated(Vec::new()));

        assert_eq!(result.status, B1CQuorumCountingStatus::Rejected);
        assert_eq!(
            result.rejection_kind,
            Some(B1CQuorumCountingRejectionKind::EmptyValidatedSigners)
        );
        assert_no_execution(&result);
    }

    #[test]
    fn threshold_zero_rejects() {
        let result = count_b1c_validated_guardian_quorum(
            &decoded_guardian_set(0),
            &validated(vec![guardian(1)]),
        );

        assert_eq!(result.status, B1CQuorumCountingStatus::Rejected);
        assert_eq!(
            result.rejection_kind,
            Some(B1CQuorumCountingRejectionKind::ThresholdZero)
        );
        assert_no_execution(&result);
    }

    #[test]
    fn threshold_greater_than_guardian_count_rejects() {
        let result = count_b1c_validated_guardian_quorum(
            &guardian_set(
                Some(4),
                3,
                Phase41K2GuardianSetAccountLoadingStatus::GuardianSetAccountDataDecoded,
                Some(GUARDIAN_SET_ID),
            ),
            &validated(vec![guardian(1), guardian(2), guardian(3)]),
        );

        assert_eq!(result.status, B1CQuorumCountingStatus::Rejected);
        assert_eq!(
            result.rejection_kind,
            Some(B1CQuorumCountingRejectionKind::ThresholdExceedsGuardianCount)
        );
        assert_no_execution(&result);
    }

    #[test]
    fn guardian_set_id_mismatch_rejects() {
        let result = count_b1c_validated_guardian_quorum(
            &guardian_set(
                Some(1),
                3,
                Phase41K2GuardianSetAccountLoadingStatus::GuardianSetAccountDataDecoded,
                Some(OTHER_GUARDIAN_SET_ID),
            ),
            &validated(vec![guardian(1)]),
        );

        assert_eq!(result.status, B1CQuorumCountingStatus::Rejected);
        assert_eq!(
            result.rejection_kind,
            Some(B1CQuorumCountingRejectionKind::GuardianSetIdMismatch)
        );
        assert_no_execution(&result);
    }

    #[test]
    fn rejected_guardian_set_rejects() {
        let result = count_b1c_validated_guardian_quorum(
            &guardian_set(
                None,
                0,
                Phase41K2GuardianSetAccountLoadingStatus::GuardianSetAccountDataRejected,
                None,
            ),
            &validated(vec![guardian(1)]),
        );

        assert_eq!(result.status, B1CQuorumCountingStatus::Rejected);
        assert_eq!(
            result.rejection_kind,
            Some(B1CQuorumCountingRejectionKind::GuardianSetNotDecoded)
        );
        assert_no_execution(&result);
    }
}
