use super::CANONICAL_PAYLOAD_HASH_VALIDATOR_PHASE_34;

pub const GUARDIAN_QUORUM_STRUCTURAL_VERIFIER_PHASE_35: &str =
    "GUARDIAN_QUORUM_STRUCTURAL_VERIFIER_PHASE_35";
pub const GUARDIAN_QUORUM_STRUCTURAL_VERIFIER_VERSION: u8 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GuardianPublicKey(pub [u8; 32]);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GuardianSetRef<'a> {
    pub guardian_set_id: &'a [u8; 32],
    pub threshold: u8,
    pub guardians: &'a [GuardianPublicKey],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GuardianApprovalClaim<'a> {
    pub guardian_set_id: &'a [u8; 32],
    pub guardian_public_key: GuardianPublicKey,
}

pub type GuardianApprovalRef<'a> = GuardianApprovalClaim<'a>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GuardianQuorumStructuralResult<'a> {
    pub guardian_set_id: &'a [u8; 32],
    pub threshold: u8,
    pub guardian_count: usize,
    pub unique_known_approval_count: usize,
    pub quorum_reached: bool,
    pub ed25519_signature_verification_performed: bool,
    pub cryptographic_signature_proof_accepted: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GuardianQuorumStructuralErrorKind {
    EmptyGuardianSet,
    InvalidThresholdZero,
    ThresholdExceedsGuardianSet,
    DuplicateGuardianPublicKey,
    EmptyApprovals,
    GuardianSetIdMismatch,
    UnknownGuardian,
    DuplicateGuardianApproval,
    QuorumNotReached,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GuardianQuorumStructuralError {
    pub kind: GuardianQuorumStructuralErrorKind,
    pub guardian_index: Option<usize>,
    pub approval_index: Option<usize>,
    pub threshold: u8,
    pub guardian_count: usize,
    pub unique_known_approval_count: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GuardianQuorumStructuralReport {
    pub verifier_id: &'static str,
    pub verifier_version: u8,
    pub membership_check_enabled: bool,
    pub quorum_threshold_check_enabled: bool,
    pub duplicate_guardian_public_key_rejected: bool,
    pub duplicate_guardian_approval_rejected: bool,
    pub unknown_guardian_rejected: bool,
    pub guardian_set_id_mismatch_rejected: bool,
    pub empty_guardian_set_rejected: bool,
    pub invalid_threshold_rejected: bool,
    pub empty_approvals_rejected: bool,
    pub insufficient_quorum_rejected: bool,
    pub signature_bytes_parsed: bool,
    pub ed25519_signature_verification_enabled: bool,
    pub instruction_sysvar_parsing_enabled: bool,
    pub ed25519_program_instruction_validation_enabled: bool,
    pub cryptographic_signature_proof_accepted: bool,
    pub caller_provided_signature_claim_trusted: bool,
    pub phase_34_hash_validator_available: bool,
    pub phase_34_hash_validator_phase: &'static str,
    pub phase_34_hash_validator_recomputes_hash: bool,
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

pub const GUARDIAN_QUORUM_STRUCTURAL_REPORT: GuardianQuorumStructuralReport =
    GuardianQuorumStructuralReport {
        verifier_id: GUARDIAN_QUORUM_STRUCTURAL_VERIFIER_PHASE_35,
        verifier_version: GUARDIAN_QUORUM_STRUCTURAL_VERIFIER_VERSION,
        membership_check_enabled: true,
        quorum_threshold_check_enabled: true,
        duplicate_guardian_public_key_rejected: true,
        duplicate_guardian_approval_rejected: true,
        unknown_guardian_rejected: true,
        guardian_set_id_mismatch_rejected: true,
        empty_guardian_set_rejected: true,
        invalid_threshold_rejected: true,
        empty_approvals_rejected: true,
        insufficient_quorum_rejected: true,
        signature_bytes_parsed: false,
        ed25519_signature_verification_enabled: false,
        instruction_sysvar_parsing_enabled: false,
        ed25519_program_instruction_validation_enabled: false,
        cryptographic_signature_proof_accepted: false,
        caller_provided_signature_claim_trusted: false,
        phase_34_hash_validator_available: true,
        phase_34_hash_validator_phase: CANONICAL_PAYLOAD_HASH_VALIDATOR_PHASE_34,
        phase_34_hash_validator_recomputes_hash: true,
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

pub fn verify_guardian_quorum_structural<'a>(
    guardian_set: GuardianSetRef<'a>,
    approvals: &[GuardianApprovalRef<'a>],
) -> Result<GuardianQuorumStructuralResult<'a>, GuardianQuorumStructuralError> {
    if guardian_set.guardians.is_empty() {
        return Err(error(
            GuardianQuorumStructuralErrorKind::EmptyGuardianSet,
            guardian_set,
            None,
            None,
            0,
        ));
    }

    if guardian_set.threshold == 0 {
        return Err(error(
            GuardianQuorumStructuralErrorKind::InvalidThresholdZero,
            guardian_set,
            None,
            None,
            0,
        ));
    }

    if guardian_set.threshold as usize > guardian_set.guardians.len() {
        return Err(error(
            GuardianQuorumStructuralErrorKind::ThresholdExceedsGuardianSet,
            guardian_set,
            None,
            None,
            0,
        ));
    }

    if let Some(index) = duplicate_guardian_index(guardian_set.guardians) {
        return Err(error(
            GuardianQuorumStructuralErrorKind::DuplicateGuardianPublicKey,
            guardian_set,
            Some(index),
            None,
            0,
        ));
    }

    if approvals.is_empty() {
        return Err(error(
            GuardianQuorumStructuralErrorKind::EmptyApprovals,
            guardian_set,
            None,
            None,
            0,
        ));
    }

    let mut unique_known_approval_count = 0usize;

    for (approval_index, approval) in approvals.iter().enumerate() {
        if approval.guardian_set_id != guardian_set.guardian_set_id {
            return Err(error(
                GuardianQuorumStructuralErrorKind::GuardianSetIdMismatch,
                guardian_set,
                None,
                Some(approval_index),
                unique_known_approval_count,
            ));
        }

        if !guardian_set
            .guardians
            .contains(&approval.guardian_public_key)
        {
            return Err(error(
                GuardianQuorumStructuralErrorKind::UnknownGuardian,
                guardian_set,
                None,
                Some(approval_index),
                unique_known_approval_count,
            ));
        }

        if approvals[..approval_index]
            .iter()
            .any(|previous| previous.guardian_public_key == approval.guardian_public_key)
        {
            return Err(error(
                GuardianQuorumStructuralErrorKind::DuplicateGuardianApproval,
                guardian_set,
                None,
                Some(approval_index),
                unique_known_approval_count,
            ));
        }

        unique_known_approval_count += 1;
    }

    if unique_known_approval_count < guardian_set.threshold as usize {
        return Err(error(
            GuardianQuorumStructuralErrorKind::QuorumNotReached,
            guardian_set,
            None,
            None,
            unique_known_approval_count,
        ));
    }

    Ok(GuardianQuorumStructuralResult {
        guardian_set_id: guardian_set.guardian_set_id,
        threshold: guardian_set.threshold,
        guardian_count: guardian_set.guardians.len(),
        unique_known_approval_count,
        quorum_reached: true,
        ed25519_signature_verification_performed: false,
        cryptographic_signature_proof_accepted: false,
    })
}

pub fn guardian_quorum_structural_report() -> &'static GuardianQuorumStructuralReport {
    &GUARDIAN_QUORUM_STRUCTURAL_REPORT
}

fn duplicate_guardian_index(guardians: &[GuardianPublicKey]) -> Option<usize> {
    for (index, guardian) in guardians.iter().enumerate() {
        if guardians[..index]
            .iter()
            .any(|previous| previous == guardian)
        {
            return Some(index);
        }
    }

    None
}

fn error(
    kind: GuardianQuorumStructuralErrorKind,
    guardian_set: GuardianSetRef<'_>,
    guardian_index: Option<usize>,
    approval_index: Option<usize>,
    unique_known_approval_count: usize,
) -> GuardianQuorumStructuralError {
    GuardianQuorumStructuralError {
        kind,
        guardian_index,
        approval_index,
        threshold: guardian_set.threshold,
        guardian_count: guardian_set.guardians.len(),
        unique_known_approval_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::verifier::{
        canonical_payload_hash_validation_report, read_only_verifier_boundary,
        CANONICAL_PAYLOAD_HASH_VALIDATOR_PHASE_34, READ_ONLY_RUNTIME_VERIFIER_SCAFFOLD_PHASE_32,
    };

    const GUARDIAN_SET_ID: [u8; 32] = [0x22; 32];
    const OTHER_GUARDIAN_SET_ID: [u8; 32] = [0x23; 32];

    fn guardian(byte: u8) -> GuardianPublicKey {
        GuardianPublicKey([byte; 32])
    }

    fn guardian_set<'a>(
        guardian_set_id: &'a [u8; 32],
        threshold: u8,
        guardians: &'a [GuardianPublicKey],
    ) -> GuardianSetRef<'a> {
        GuardianSetRef {
            guardian_set_id,
            threshold,
            guardians,
        }
    }

    fn approval<'a>(
        guardian_set_id: &'a [u8; 32],
        guardian_public_key: GuardianPublicKey,
    ) -> GuardianApprovalRef<'a> {
        GuardianApprovalRef {
            guardian_set_id,
            guardian_public_key,
        }
    }

    fn assert_error_kind(
        result: Result<GuardianQuorumStructuralResult<'_>, GuardianQuorumStructuralError>,
        kind: GuardianQuorumStructuralErrorKind,
    ) -> GuardianQuorumStructuralError {
        let error = result.expect_err("guardian quorum structural error");
        assert_eq!(error.kind, kind);
        error
    }

    #[test]
    fn valid_quorum_is_accepted() {
        let guardians = [guardian(1), guardian(2), guardian(3)];
        let approvals = [
            approval(&GUARDIAN_SET_ID, guardian(1)),
            approval(&GUARDIAN_SET_ID, guardian(3)),
        ];
        let result = verify_guardian_quorum_structural(
            guardian_set(&GUARDIAN_SET_ID, 2, &guardians),
            &approvals,
        )
        .expect("valid structural quorum");

        assert_eq!(result.guardian_set_id, &GUARDIAN_SET_ID);
        assert_eq!(result.threshold, 2);
        assert_eq!(result.guardian_count, 3);
        assert_eq!(result.unique_known_approval_count, 2);
        assert!(result.quorum_reached);
        assert!(!result.ed25519_signature_verification_performed);
        assert!(!result.cryptographic_signature_proof_accepted);
    }

    #[test]
    fn threshold_exactly_met_is_accepted() {
        let guardians = [guardian(1), guardian(2)];
        let approvals = [
            approval(&GUARDIAN_SET_ID, guardian(1)),
            approval(&GUARDIAN_SET_ID, guardian(2)),
        ];
        let result = verify_guardian_quorum_structural(
            guardian_set(&GUARDIAN_SET_ID, 2, &guardians),
            &approvals,
        )
        .expect("threshold exactly met");

        assert_eq!(result.unique_known_approval_count, 2);
        assert!(result.quorum_reached);
    }

    #[test]
    fn threshold_exceeded_is_accepted() {
        let guardians = [guardian(1), guardian(2), guardian(3)];
        let approvals = [
            approval(&GUARDIAN_SET_ID, guardian(1)),
            approval(&GUARDIAN_SET_ID, guardian(2)),
            approval(&GUARDIAN_SET_ID, guardian(3)),
        ];
        let result = verify_guardian_quorum_structural(
            guardian_set(&GUARDIAN_SET_ID, 2, &guardians),
            &approvals,
        )
        .expect("threshold exceeded");

        assert_eq!(result.unique_known_approval_count, 3);
        assert!(result.quorum_reached);
    }

    #[test]
    fn empty_guardian_set_is_rejected() {
        let guardians = [];
        let approvals = [approval(&GUARDIAN_SET_ID, guardian(1))];

        let error = assert_error_kind(
            verify_guardian_quorum_structural(
                guardian_set(&GUARDIAN_SET_ID, 1, &guardians),
                &approvals,
            ),
            GuardianQuorumStructuralErrorKind::EmptyGuardianSet,
        );

        assert_eq!(error.guardian_count, 0);
    }

    #[test]
    fn threshold_zero_is_rejected() {
        let guardians = [guardian(1)];
        let approvals = [approval(&GUARDIAN_SET_ID, guardian(1))];

        let error = assert_error_kind(
            verify_guardian_quorum_structural(
                guardian_set(&GUARDIAN_SET_ID, 0, &guardians),
                &approvals,
            ),
            GuardianQuorumStructuralErrorKind::InvalidThresholdZero,
        );

        assert_eq!(error.threshold, 0);
    }

    #[test]
    fn threshold_greater_than_guardian_set_size_is_rejected() {
        let guardians = [guardian(1)];
        let approvals = [approval(&GUARDIAN_SET_ID, guardian(1))];

        let error = assert_error_kind(
            verify_guardian_quorum_structural(
                guardian_set(&GUARDIAN_SET_ID, 2, &guardians),
                &approvals,
            ),
            GuardianQuorumStructuralErrorKind::ThresholdExceedsGuardianSet,
        );

        assert_eq!(error.threshold, 2);
        assert_eq!(error.guardian_count, 1);
    }

    #[test]
    fn duplicate_guardian_public_key_in_set_is_rejected() {
        let guardians = [guardian(1), guardian(1)];
        let approvals = [approval(&GUARDIAN_SET_ID, guardian(1))];

        let error = assert_error_kind(
            verify_guardian_quorum_structural(
                guardian_set(&GUARDIAN_SET_ID, 1, &guardians),
                &approvals,
            ),
            GuardianQuorumStructuralErrorKind::DuplicateGuardianPublicKey,
        );

        assert_eq!(error.guardian_index, Some(1));
    }

    #[test]
    fn empty_approvals_are_rejected() {
        let guardians = [guardian(1)];
        let approvals = [];

        assert_error_kind(
            verify_guardian_quorum_structural(
                guardian_set(&GUARDIAN_SET_ID, 1, &guardians),
                &approvals,
            ),
            GuardianQuorumStructuralErrorKind::EmptyApprovals,
        );
    }

    #[test]
    fn guardian_set_id_mismatch_is_rejected() {
        let guardians = [guardian(1)];
        let approvals = [approval(&OTHER_GUARDIAN_SET_ID, guardian(1))];

        let error = assert_error_kind(
            verify_guardian_quorum_structural(
                guardian_set(&GUARDIAN_SET_ID, 1, &guardians),
                &approvals,
            ),
            GuardianQuorumStructuralErrorKind::GuardianSetIdMismatch,
        );

        assert_eq!(error.approval_index, Some(0));
    }

    #[test]
    fn unknown_guardian_is_rejected() {
        let guardians = [guardian(1)];
        let approvals = [approval(&GUARDIAN_SET_ID, guardian(9))];

        let error = assert_error_kind(
            verify_guardian_quorum_structural(
                guardian_set(&GUARDIAN_SET_ID, 1, &guardians),
                &approvals,
            ),
            GuardianQuorumStructuralErrorKind::UnknownGuardian,
        );

        assert_eq!(error.approval_index, Some(0));
    }

    #[test]
    fn duplicate_guardian_approval_is_rejected() {
        let guardians = [guardian(1), guardian(2)];
        let approvals = [
            approval(&GUARDIAN_SET_ID, guardian(1)),
            approval(&GUARDIAN_SET_ID, guardian(1)),
        ];

        let error = assert_error_kind(
            verify_guardian_quorum_structural(
                guardian_set(&GUARDIAN_SET_ID, 2, &guardians),
                &approvals,
            ),
            GuardianQuorumStructuralErrorKind::DuplicateGuardianApproval,
        );

        assert_eq!(error.approval_index, Some(1));
        assert_eq!(error.unique_known_approval_count, 1);
    }

    #[test]
    fn insufficient_unique_approvals_are_rejected() {
        let guardians = [guardian(1), guardian(2), guardian(3)];
        let approvals = [approval(&GUARDIAN_SET_ID, guardian(1))];

        let error = assert_error_kind(
            verify_guardian_quorum_structural(
                guardian_set(&GUARDIAN_SET_ID, 2, &guardians),
                &approvals,
            ),
            GuardianQuorumStructuralErrorKind::QuorumNotReached,
        );

        assert_eq!(error.unique_known_approval_count, 1);
        assert_eq!(error.threshold, 2);
    }

    #[test]
    fn report_says_ed25519_and_signature_proof_are_not_implemented() {
        let report = guardian_quorum_structural_report();

        assert_eq!(
            report.verifier_id,
            GUARDIAN_QUORUM_STRUCTURAL_VERIFIER_PHASE_35
        );
        assert_eq!(
            report.verifier_version,
            GUARDIAN_QUORUM_STRUCTURAL_VERIFIER_VERSION
        );
        assert!(report.membership_check_enabled);
        assert!(report.quorum_threshold_check_enabled);
        assert!(!report.signature_bytes_parsed);
        assert!(!report.ed25519_signature_verification_enabled);
        assert!(!report.instruction_sysvar_parsing_enabled);
        assert!(!report.ed25519_program_instruction_validation_enabled);
        assert!(!report.cryptographic_signature_proof_accepted);
        assert!(!report.caller_provided_signature_claim_trusted);
    }

    #[test]
    fn report_keeps_unfinished_verifier_and_execution_surfaces_disabled() {
        let report = guardian_quorum_structural_report();

        assert!(report.duplicate_guardian_public_key_rejected);
        assert!(report.duplicate_guardian_approval_rejected);
        assert!(report.unknown_guardian_rejected);
        assert!(report.guardian_set_id_mismatch_rejected);
        assert!(report.empty_guardian_set_rejected);
        assert!(report.invalid_threshold_rejected);
        assert!(report.empty_approvals_rejected);
        assert!(report.insufficient_quorum_rejected);
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
        let report = guardian_quorum_structural_report();
        let phase_34_report = canonical_payload_hash_validation_report();

        assert!(report.phase_34_hash_validator_available);
        assert_eq!(
            report.phase_34_hash_validator_phase,
            CANONICAL_PAYLOAD_HASH_VALIDATOR_PHASE_34
        );
        assert!(report.phase_34_hash_validator_recomputes_hash);
        assert_eq!(
            phase_34_report.validator_id,
            CANONICAL_PAYLOAD_HASH_VALIDATOR_PHASE_34
        );
        assert!(phase_34_report.recomputes_hash_from_payload_bytes);
        assert!(!phase_34_report.caller_provided_payload_hash_trusted);
    }
}
