use super::{
    FutureEd25519VerificationEvidenceRejectionCase, FutureEd25519VerificationEvidenceRequirement,
    ED25519_VERIFICATION_EVIDENCE_INTEGRATION_DESIGN_PHASE_40D,
    FUTURE_ED25519_VERIFICATION_EVIDENCE_REJECTION_CASES,
    FUTURE_ED25519_VERIFICATION_EVIDENCE_REQUIREMENTS,
};

pub const ED25519_VERIFICATION_EVIDENCE_COVERAGE_MATRIX_PHASE_40F: &str =
    "ED25519_VERIFICATION_EVIDENCE_COVERAGE_MATRIX_PHASE_40F";
pub const ED25519_VERIFICATION_EVIDENCE_COVERAGE_MATRIX_VERSION: u8 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ed25519VerificationEvidenceCoverageCategory {
    FutureInstructionPresence,
    FutureRuntimeSysvarBinding,
    FutureInstructionIdentityBinding,
    FutureEd25519ProgramBinding,
    FutureLayoutBinding,
    FuturePayloadHashBinding,
    FutureGuardianBinding,
    FutureRouteBinding,
    FutureMintBinding,
    FutureRecipientBinding,
    FutureAmountBinding,
    FutureFinalityOrExpirationBinding,
    FutureDeterministicErrorSurface,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ed25519VerificationEvidenceRequirementCoverage {
    pub requirement: FutureEd25519VerificationEvidenceRequirement,
    pub category: Ed25519VerificationEvidenceCoverageCategory,
    pub primary_rejection_case: Option<FutureEd25519VerificationEvidenceRejectionCase>,
    pub declared_by_phase_40d: bool,
    pub implemented_by_phase_40f: bool,
    pub requires_future_runtime_integration: bool,
    pub raw_instructions_sysvar_parser_implemented: bool,
    pub account_info_parser_implemented: bool,
    pub ed25519_signature_verification_performed: bool,
    pub cryptographic_signature_proof_accepted: bool,
    pub verification_evidence_accepted: bool,
    pub quorum_counted: bool,
    pub authorization_granted: bool,
    pub execution_enabled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ed25519VerificationEvidenceCoverageMatrixReport {
    pub matrix_id: &'static str,
    pub matrix_version: u8,
    pub phase_40d_integration_design_required: bool,
    pub phase_40d_integration_design_phase: &'static str,
    pub requirement_count: usize,
    pub rejection_case_count: usize,
    pub matrix_entry_count: usize,
    pub every_phase_40d_requirement_has_matrix_entry: bool,
    pub raw_instructions_sysvar_parser_implemented: bool,
    pub account_info_parser_implemented: bool,
    pub ed25519_signature_verification_performed: bool,
    pub cryptographic_signature_proof_accepted: bool,
    pub verification_evidence_accepted: bool,
    pub quorum_counting_enabled: bool,
    pub authorization_enabled: bool,
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

pub const ED25519_VERIFICATION_EVIDENCE_REQUIREMENT_COVERAGE_MATRIX:
    [Ed25519VerificationEvidenceRequirementCoverage; 16] = [
    coverage(
        FutureEd25519VerificationEvidenceRequirement::PriorEd25519Instruction,
        Ed25519VerificationEvidenceCoverageCategory::FutureInstructionPresence,
        Some(FutureEd25519VerificationEvidenceRejectionCase::Ed25519InstructionNotFound),
    ),
    coverage(
        FutureEd25519VerificationEvidenceRequirement::InstructionsSysvarRuntimeRead,
        Ed25519VerificationEvidenceCoverageCategory::FutureRuntimeSysvarBinding,
        Some(FutureEd25519VerificationEvidenceRejectionCase::MissingInstructionsSysvar),
    ),
    coverage(
        FutureEd25519VerificationEvidenceRequirement::CurrentInstructionIdentity,
        Ed25519VerificationEvidenceCoverageCategory::FutureInstructionIdentityBinding,
        Some(FutureEd25519VerificationEvidenceRejectionCase::MissingCurrentInstructionIdentity),
    ),
    coverage(
        FutureEd25519VerificationEvidenceRequirement::Ed25519ProgramIdMatch,
        Ed25519VerificationEvidenceCoverageCategory::FutureEd25519ProgramBinding,
        Some(FutureEd25519VerificationEvidenceRejectionCase::WrongEd25519ProgramId),
    ),
    coverage(
        FutureEd25519VerificationEvidenceRequirement::SupportedOffsetLayout,
        Ed25519VerificationEvidenceCoverageCategory::FutureLayoutBinding,
        Some(FutureEd25519VerificationEvidenceRejectionCase::UnsupportedOffsetLayout),
    ),
    coverage(
        FutureEd25519VerificationEvidenceRequirement::Phase37LayoutConstraints,
        Ed25519VerificationEvidenceCoverageCategory::FutureLayoutBinding,
        Some(FutureEd25519VerificationEvidenceRejectionCase::MalformedEd25519InstructionData),
    ),
    coverage(
        FutureEd25519VerificationEvidenceRequirement::Phase38InstructionDataParsing,
        Ed25519VerificationEvidenceCoverageCategory::FutureLayoutBinding,
        Some(FutureEd25519VerificationEvidenceRejectionCase::MalformedEd25519InstructionData),
    ),
    coverage(
        FutureEd25519VerificationEvidenceRequirement::Phase34PayloadHashMatch,
        Ed25519VerificationEvidenceCoverageCategory::FuturePayloadHashBinding,
        Some(FutureEd25519VerificationEvidenceRejectionCase::MessageHashMismatch),
    ),
    coverage(
        FutureEd25519VerificationEvidenceRequirement::GuardianPublicKeyInActiveSet,
        Ed25519VerificationEvidenceCoverageCategory::FutureGuardianBinding,
        Some(FutureEd25519VerificationEvidenceRejectionCase::PublicKeyMismatch),
    ),
    coverage(
        FutureEd25519VerificationEvidenceRequirement::GuardianSetIdMatch,
        Ed25519VerificationEvidenceCoverageCategory::FutureGuardianBinding,
        Some(FutureEd25519VerificationEvidenceRejectionCase::GuardianSetMismatch),
    ),
    coverage(
        FutureEd25519VerificationEvidenceRequirement::RouteBinding,
        Ed25519VerificationEvidenceCoverageCategory::FutureRouteBinding,
        Some(FutureEd25519VerificationEvidenceRejectionCase::WrongRoute),
    ),
    coverage(
        FutureEd25519VerificationEvidenceRequirement::TargetMintBinding,
        Ed25519VerificationEvidenceCoverageCategory::FutureMintBinding,
        Some(FutureEd25519VerificationEvidenceRejectionCase::WrongTargetMint),
    ),
    coverage(
        FutureEd25519VerificationEvidenceRequirement::RecipientBinding,
        Ed25519VerificationEvidenceCoverageCategory::FutureRecipientBinding,
        Some(FutureEd25519VerificationEvidenceRejectionCase::WrongRecipient),
    ),
    coverage(
        FutureEd25519VerificationEvidenceRequirement::AmountBinding,
        Ed25519VerificationEvidenceCoverageCategory::FutureAmountBinding,
        Some(FutureEd25519VerificationEvidenceRejectionCase::WrongAmount),
    ),
    coverage(
        FutureEd25519VerificationEvidenceRequirement::ExpirationOrFinalityBinding,
        Ed25519VerificationEvidenceCoverageCategory::FutureFinalityOrExpirationBinding,
        Some(FutureEd25519VerificationEvidenceRejectionCase::ExpiredEvidence),
    ),
    coverage(
        FutureEd25519VerificationEvidenceRequirement::DeterministicFailureReason,
        Ed25519VerificationEvidenceCoverageCategory::FutureDeterministicErrorSurface,
        None,
    ),
];

pub const ED25519_VERIFICATION_EVIDENCE_COVERAGE_MATRIX_REPORT:
    Ed25519VerificationEvidenceCoverageMatrixReport =
    Ed25519VerificationEvidenceCoverageMatrixReport {
        matrix_id: ED25519_VERIFICATION_EVIDENCE_COVERAGE_MATRIX_PHASE_40F,
        matrix_version: ED25519_VERIFICATION_EVIDENCE_COVERAGE_MATRIX_VERSION,
        phase_40d_integration_design_required: true,
        phase_40d_integration_design_phase:
            ED25519_VERIFICATION_EVIDENCE_INTEGRATION_DESIGN_PHASE_40D,
        requirement_count: FUTURE_ED25519_VERIFICATION_EVIDENCE_REQUIREMENTS.len(),
        rejection_case_count: FUTURE_ED25519_VERIFICATION_EVIDENCE_REJECTION_CASES.len(),
        matrix_entry_count: ED25519_VERIFICATION_EVIDENCE_REQUIREMENT_COVERAGE_MATRIX.len(),
        every_phase_40d_requirement_has_matrix_entry: true,
        raw_instructions_sysvar_parser_implemented: false,
        account_info_parser_implemented: false,
        ed25519_signature_verification_performed: false,
        cryptographic_signature_proof_accepted: false,
        verification_evidence_accepted: false,
        quorum_counting_enabled: false,
        authorization_enabled: false,
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

pub fn ed25519_verification_evidence_coverage_matrix_report(
) -> &'static Ed25519VerificationEvidenceCoverageMatrixReport {
    &ED25519_VERIFICATION_EVIDENCE_COVERAGE_MATRIX_REPORT
}

pub fn ed25519_verification_evidence_requirement_coverage_matrix(
) -> &'static [Ed25519VerificationEvidenceRequirementCoverage] {
    &ED25519_VERIFICATION_EVIDENCE_REQUIREMENT_COVERAGE_MATRIX
}

pub fn coverage_for_requirement(
    requirement: FutureEd25519VerificationEvidenceRequirement,
) -> Option<&'static Ed25519VerificationEvidenceRequirementCoverage> {
    ED25519_VERIFICATION_EVIDENCE_REQUIREMENT_COVERAGE_MATRIX
        .iter()
        .find(|entry| entry.requirement == requirement)
}

pub fn every_phase_40d_requirement_has_coverage_entry() -> bool {
    FUTURE_ED25519_VERIFICATION_EVIDENCE_REQUIREMENTS
        .iter()
        .all(|requirement| coverage_for_requirement(*requirement).is_some())
}

const fn coverage(
    requirement: FutureEd25519VerificationEvidenceRequirement,
    category: Ed25519VerificationEvidenceCoverageCategory,
    primary_rejection_case: Option<FutureEd25519VerificationEvidenceRejectionCase>,
) -> Ed25519VerificationEvidenceRequirementCoverage {
    Ed25519VerificationEvidenceRequirementCoverage {
        requirement,
        category,
        primary_rejection_case,
        declared_by_phase_40d: true,
        implemented_by_phase_40f: false,
        requires_future_runtime_integration: true,
        raw_instructions_sysvar_parser_implemented: false,
        account_info_parser_implemented: false,
        ed25519_signature_verification_performed: false,
        cryptographic_signature_proof_accepted: false,
        verification_evidence_accepted: false,
        quorum_counted: false,
        authorization_granted: false,
        execution_enabled: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::verifier::{
        ed25519_verification_evidence_integration_design_report, read_only_verifier_boundary,
        READ_ONLY_RUNTIME_VERIFIER_SCAFFOLD_PHASE_32,
    };

    #[test]
    fn marker_and_report_are_stable() {
        let report = ed25519_verification_evidence_coverage_matrix_report();

        assert_eq!(
            report.matrix_id,
            ED25519_VERIFICATION_EVIDENCE_COVERAGE_MATRIX_PHASE_40F
        );
        assert_eq!(
            report.matrix_version,
            ED25519_VERIFICATION_EVIDENCE_COVERAGE_MATRIX_VERSION
        );
        assert!(report.phase_40d_integration_design_required);
        assert_eq!(
            report.phase_40d_integration_design_phase,
            ED25519_VERIFICATION_EVIDENCE_INTEGRATION_DESIGN_PHASE_40D
        );
    }

    #[test]
    fn matrix_covers_every_phase_40d_requirement() {
        let report = ed25519_verification_evidence_coverage_matrix_report();
        let matrix = ed25519_verification_evidence_requirement_coverage_matrix();

        assert_eq!(
            report.requirement_count,
            FUTURE_ED25519_VERIFICATION_EVIDENCE_REQUIREMENTS.len()
        );
        assert_eq!(
            report.matrix_entry_count,
            ED25519_VERIFICATION_EVIDENCE_REQUIREMENT_COVERAGE_MATRIX.len()
        );
        assert_eq!(
            matrix.len(),
            FUTURE_ED25519_VERIFICATION_EVIDENCE_REQUIREMENTS.len()
        );
        assert!(every_phase_40d_requirement_has_coverage_entry());
        assert!(report.every_phase_40d_requirement_has_matrix_entry);
    }

    #[test]
    fn every_matrix_entry_is_declared_future_only_and_non_authorizing() {
        let matrix = ed25519_verification_evidence_requirement_coverage_matrix();

        for entry in matrix {
            assert!(entry.declared_by_phase_40d);
            assert!(!entry.implemented_by_phase_40f);
            assert!(entry.requires_future_runtime_integration);
            assert!(!entry.raw_instructions_sysvar_parser_implemented);
            assert!(!entry.account_info_parser_implemented);
            assert!(!entry.ed25519_signature_verification_performed);
            assert!(!entry.cryptographic_signature_proof_accepted);
            assert!(!entry.verification_evidence_accepted);
            assert!(!entry.quorum_counted);
            assert!(!entry.authorization_granted);
            assert!(!entry.execution_enabled);
        }
    }

    #[test]
    fn instruction_presence_requirement_maps_to_instruction_not_found() {
        let entry = coverage_for_requirement(
            FutureEd25519VerificationEvidenceRequirement::PriorEd25519Instruction,
        )
        .expect("coverage entry");

        assert_eq!(
            entry.category,
            Ed25519VerificationEvidenceCoverageCategory::FutureInstructionPresence
        );
        assert_eq!(
            entry.primary_rejection_case,
            Some(FutureEd25519VerificationEvidenceRejectionCase::Ed25519InstructionNotFound)
        );
    }

    #[test]
    fn sysvar_requirement_maps_to_missing_sysvar() {
        let entry = coverage_for_requirement(
            FutureEd25519VerificationEvidenceRequirement::InstructionsSysvarRuntimeRead,
        )
        .expect("coverage entry");

        assert_eq!(
            entry.category,
            Ed25519VerificationEvidenceCoverageCategory::FutureRuntimeSysvarBinding
        );
        assert_eq!(
            entry.primary_rejection_case,
            Some(FutureEd25519VerificationEvidenceRejectionCase::MissingInstructionsSysvar)
        );
    }

    #[test]
    fn payload_hash_requirement_maps_to_message_hash_mismatch() {
        let entry = coverage_for_requirement(
            FutureEd25519VerificationEvidenceRequirement::Phase34PayloadHashMatch,
        )
        .expect("coverage entry");

        assert_eq!(
            entry.category,
            Ed25519VerificationEvidenceCoverageCategory::FuturePayloadHashBinding
        );
        assert_eq!(
            entry.primary_rejection_case,
            Some(FutureEd25519VerificationEvidenceRejectionCase::MessageHashMismatch)
        );
    }

    #[test]
    fn guardian_requirements_map_to_guardian_errors() {
        let key_entry = coverage_for_requirement(
            FutureEd25519VerificationEvidenceRequirement::GuardianPublicKeyInActiveSet,
        )
        .expect("coverage entry");
        let set_entry = coverage_for_requirement(
            FutureEd25519VerificationEvidenceRequirement::GuardianSetIdMatch,
        )
        .expect("coverage entry");

        assert_eq!(
            key_entry.primary_rejection_case,
            Some(FutureEd25519VerificationEvidenceRejectionCase::PublicKeyMismatch)
        );
        assert_eq!(
            set_entry.primary_rejection_case,
            Some(FutureEd25519VerificationEvidenceRejectionCase::GuardianSetMismatch)
        );
    }

    #[test]
    fn route_mint_recipient_and_amount_requirements_map_to_binding_errors() {
        assert_eq!(
            coverage_for_requirement(FutureEd25519VerificationEvidenceRequirement::RouteBinding)
                .expect("coverage entry")
                .primary_rejection_case,
            Some(FutureEd25519VerificationEvidenceRejectionCase::WrongRoute)
        );
        assert_eq!(
            coverage_for_requirement(
                FutureEd25519VerificationEvidenceRequirement::TargetMintBinding
            )
            .expect("coverage entry")
            .primary_rejection_case,
            Some(FutureEd25519VerificationEvidenceRejectionCase::WrongTargetMint)
        );
        assert_eq!(
            coverage_for_requirement(
                FutureEd25519VerificationEvidenceRequirement::RecipientBinding
            )
            .expect("coverage entry")
            .primary_rejection_case,
            Some(FutureEd25519VerificationEvidenceRejectionCase::WrongRecipient)
        );
        assert_eq!(
            coverage_for_requirement(FutureEd25519VerificationEvidenceRequirement::AmountBinding)
                .expect("coverage entry")
                .primary_rejection_case,
            Some(FutureEd25519VerificationEvidenceRejectionCase::WrongAmount)
        );
    }

    #[test]
    fn deterministic_failure_reason_has_no_single_primary_error() {
        let entry = coverage_for_requirement(
            FutureEd25519VerificationEvidenceRequirement::DeterministicFailureReason,
        )
        .expect("coverage entry");

        assert_eq!(
            entry.category,
            Ed25519VerificationEvidenceCoverageCategory::FutureDeterministicErrorSurface
        );
        assert_eq!(entry.primary_rejection_case, None);
    }

    #[test]
    fn all_phase_40d_rejection_cases_remain_declared() {
        let report = ed25519_verification_evidence_coverage_matrix_report();

        assert_eq!(report.rejection_case_count, 18);
        assert!(
            FUTURE_ED25519_VERIFICATION_EVIDENCE_REJECTION_CASES.contains(
                &FutureEd25519VerificationEvidenceRejectionCase::UnreadableInstructionsSysvar
            )
        );
        assert!(FUTURE_ED25519_VERIFICATION_EVIDENCE_REJECTION_CASES.contains(
            &FutureEd25519VerificationEvidenceRejectionCase::Ed25519InstructionAfterCurrentInstruction
        ));
        assert!(FUTURE_ED25519_VERIFICATION_EVIDENCE_REJECTION_CASES
            .contains(&FutureEd25519VerificationEvidenceRejectionCase::DuplicateGuardianEvidence));
        assert!(FUTURE_ED25519_VERIFICATION_EVIDENCE_REJECTION_CASES
            .contains(&FutureEd25519VerificationEvidenceRejectionCase::AmbiguousCandidateEvidence));
    }

    #[test]
    fn report_preserves_all_disabled_runtime_surfaces() {
        let report = ed25519_verification_evidence_coverage_matrix_report();

        assert!(!report.raw_instructions_sysvar_parser_implemented);
        assert!(!report.account_info_parser_implemented);
        assert!(!report.ed25519_signature_verification_performed);
        assert!(!report.cryptographic_signature_proof_accepted);
        assert!(!report.verification_evidence_accepted);
        assert!(!report.quorum_counting_enabled);
        assert!(!report.authorization_enabled);
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
    fn phase_40d_design_report_remains_non_authorizing() {
        let phase_40d_report = ed25519_verification_evidence_integration_design_report();

        assert!(!phase_40d_report.raw_instructions_sysvar_parser_implemented);
        assert!(!phase_40d_report.account_info_parser_implemented);
        assert!(!phase_40d_report.ed25519_signature_verification_performed);
        assert!(!phase_40d_report.cryptographic_signature_proof_accepted);
        assert!(!phase_40d_report.verification_evidence_accepted);
        assert!(!phase_40d_report.quorum_counting_enabled);
        assert!(!phase_40d_report.authorization_enabled);
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
}
