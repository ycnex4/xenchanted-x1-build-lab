#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase41BRequirement {
    InstructionsSysvarRuntimeRead,
    InstructionsSysvarReadable,
    CurrentInstructionIdentity,
    PriorEd25519Instruction,
    PriorEd25519InstructionOrdering,
    Ed25519ProgramIdMatch,
    Phase37LayoutConstraints,
    Phase38InstructionDataParsing,
    SupportedOffsetLayout,
    GuardianPublicKeyInActiveSet,
    Phase34PayloadHashMatch,
    GuardianSetIdMatch,
    GuardianEvidenceUniqueness,
    SingleCandidateResolution,
    ExpirationOrFinalityBinding,
    RouteBinding,
    TargetMintBinding,
    RecipientBinding,
    AmountBinding,
    DeterministicFailureReason,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase41BRejectionCase {
    MissingInstructionsSysvar,
    UnreadableInstructionsSysvar,
    MissingCurrentInstructionIdentity,
    Ed25519InstructionNotFound,
    Ed25519InstructionAfterCurrentInstruction,
    WrongEd25519ProgramId,
    MalformedEd25519InstructionData,
    UnsupportedOffsetLayout,
    PublicKeyMismatch,
    MessageHashMismatch,
    GuardianSetMismatch,
    DuplicateGuardianEvidence,
    AmbiguousCandidateEvidence,
    ExpiredEvidence,
    WrongRoute,
    WrongTargetMint,
    WrongRecipient,
    WrongAmount,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phase41BRequirementCoverageEntry {
    pub requirement: Phase41BRequirement,
    pub primary_rejection_case: Option<Phase41BRejectionCase>,
    pub closes_phase_40_orphan_rejection_case: bool,
    pub implemented_by_phase_41b: bool,
    pub requires_future_runtime_integration: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phase41BSafetyFlags {
    pub raw_instructions_sysvar_parser_implemented: bool,
    pub account_info_parser_implemented: bool,
    pub load_instruction_called: bool,
    pub load_instruction_enabled: bool,
    pub concrete_runtime_api_selected: bool,
    pub current_instruction_identity_derived_from_runtime: bool,
    pub ed25519_signature_verification_performed: bool,
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
pub struct Phase41BModelReport {
    pub phase: &'static str,
    pub requirements_declared: usize,
    pub rejection_cases_declared: usize,
    pub coverage_entries_declared: usize,
    pub model_only: bool,
    pub closes_all_phase_40_orphan_rejection_cases: bool,
    pub all_rejection_cases_have_owners: bool,
    pub safety_flags: Phase41BSafetyFlags,
}

pub const PHASE_41B_REQUIREMENTS: [Phase41BRequirement; 20] = [
    Phase41BRequirement::InstructionsSysvarRuntimeRead,
    Phase41BRequirement::InstructionsSysvarReadable,
    Phase41BRequirement::CurrentInstructionIdentity,
    Phase41BRequirement::PriorEd25519Instruction,
    Phase41BRequirement::PriorEd25519InstructionOrdering,
    Phase41BRequirement::Ed25519ProgramIdMatch,
    Phase41BRequirement::Phase37LayoutConstraints,
    Phase41BRequirement::Phase38InstructionDataParsing,
    Phase41BRequirement::SupportedOffsetLayout,
    Phase41BRequirement::GuardianPublicKeyInActiveSet,
    Phase41BRequirement::Phase34PayloadHashMatch,
    Phase41BRequirement::GuardianSetIdMatch,
    Phase41BRequirement::GuardianEvidenceUniqueness,
    Phase41BRequirement::SingleCandidateResolution,
    Phase41BRequirement::ExpirationOrFinalityBinding,
    Phase41BRequirement::RouteBinding,
    Phase41BRequirement::TargetMintBinding,
    Phase41BRequirement::RecipientBinding,
    Phase41BRequirement::AmountBinding,
    Phase41BRequirement::DeterministicFailureReason,
];

pub const PHASE_41B_REJECTION_CASES: [Phase41BRejectionCase; 18] = [
    Phase41BRejectionCase::MissingInstructionsSysvar,
    Phase41BRejectionCase::UnreadableInstructionsSysvar,
    Phase41BRejectionCase::MissingCurrentInstructionIdentity,
    Phase41BRejectionCase::Ed25519InstructionNotFound,
    Phase41BRejectionCase::Ed25519InstructionAfterCurrentInstruction,
    Phase41BRejectionCase::WrongEd25519ProgramId,
    Phase41BRejectionCase::MalformedEd25519InstructionData,
    Phase41BRejectionCase::UnsupportedOffsetLayout,
    Phase41BRejectionCase::PublicKeyMismatch,
    Phase41BRejectionCase::MessageHashMismatch,
    Phase41BRejectionCase::GuardianSetMismatch,
    Phase41BRejectionCase::DuplicateGuardianEvidence,
    Phase41BRejectionCase::AmbiguousCandidateEvidence,
    Phase41BRejectionCase::ExpiredEvidence,
    Phase41BRejectionCase::WrongRoute,
    Phase41BRejectionCase::WrongTargetMint,
    Phase41BRejectionCase::WrongRecipient,
    Phase41BRejectionCase::WrongAmount,
];

pub const PHASE_41B_REQUIREMENT_COVERAGE_MATRIX: [Phase41BRequirementCoverageEntry; 20] = [
    Phase41BRequirementCoverageEntry {
        requirement: Phase41BRequirement::InstructionsSysvarRuntimeRead,
        primary_rejection_case: Some(Phase41BRejectionCase::MissingInstructionsSysvar),
        closes_phase_40_orphan_rejection_case: false,
        implemented_by_phase_41b: false,
        requires_future_runtime_integration: true,
    },
    Phase41BRequirementCoverageEntry {
        requirement: Phase41BRequirement::InstructionsSysvarReadable,
        primary_rejection_case: Some(Phase41BRejectionCase::UnreadableInstructionsSysvar),
        closes_phase_40_orphan_rejection_case: true,
        implemented_by_phase_41b: false,
        requires_future_runtime_integration: true,
    },
    Phase41BRequirementCoverageEntry {
        requirement: Phase41BRequirement::CurrentInstructionIdentity,
        primary_rejection_case: Some(Phase41BRejectionCase::MissingCurrentInstructionIdentity),
        closes_phase_40_orphan_rejection_case: false,
        implemented_by_phase_41b: false,
        requires_future_runtime_integration: true,
    },
    Phase41BRequirementCoverageEntry {
        requirement: Phase41BRequirement::PriorEd25519Instruction,
        primary_rejection_case: Some(Phase41BRejectionCase::Ed25519InstructionNotFound),
        closes_phase_40_orphan_rejection_case: false,
        implemented_by_phase_41b: false,
        requires_future_runtime_integration: true,
    },
    Phase41BRequirementCoverageEntry {
        requirement: Phase41BRequirement::PriorEd25519InstructionOrdering,
        primary_rejection_case: Some(
            Phase41BRejectionCase::Ed25519InstructionAfterCurrentInstruction,
        ),
        closes_phase_40_orphan_rejection_case: true,
        implemented_by_phase_41b: false,
        requires_future_runtime_integration: true,
    },
    Phase41BRequirementCoverageEntry {
        requirement: Phase41BRequirement::Ed25519ProgramIdMatch,
        primary_rejection_case: Some(Phase41BRejectionCase::WrongEd25519ProgramId),
        closes_phase_40_orphan_rejection_case: false,
        implemented_by_phase_41b: false,
        requires_future_runtime_integration: true,
    },
    Phase41BRequirementCoverageEntry {
        requirement: Phase41BRequirement::Phase37LayoutConstraints,
        primary_rejection_case: Some(Phase41BRejectionCase::MalformedEd25519InstructionData),
        closes_phase_40_orphan_rejection_case: false,
        implemented_by_phase_41b: false,
        requires_future_runtime_integration: true,
    },
    Phase41BRequirementCoverageEntry {
        requirement: Phase41BRequirement::Phase38InstructionDataParsing,
        primary_rejection_case: Some(Phase41BRejectionCase::MalformedEd25519InstructionData),
        closes_phase_40_orphan_rejection_case: false,
        implemented_by_phase_41b: false,
        requires_future_runtime_integration: true,
    },
    Phase41BRequirementCoverageEntry {
        requirement: Phase41BRequirement::SupportedOffsetLayout,
        primary_rejection_case: Some(Phase41BRejectionCase::UnsupportedOffsetLayout),
        closes_phase_40_orphan_rejection_case: false,
        implemented_by_phase_41b: false,
        requires_future_runtime_integration: true,
    },
    Phase41BRequirementCoverageEntry {
        requirement: Phase41BRequirement::GuardianPublicKeyInActiveSet,
        primary_rejection_case: Some(Phase41BRejectionCase::PublicKeyMismatch),
        closes_phase_40_orphan_rejection_case: false,
        implemented_by_phase_41b: false,
        requires_future_runtime_integration: true,
    },
    Phase41BRequirementCoverageEntry {
        requirement: Phase41BRequirement::Phase34PayloadHashMatch,
        primary_rejection_case: Some(Phase41BRejectionCase::MessageHashMismatch),
        closes_phase_40_orphan_rejection_case: false,
        implemented_by_phase_41b: false,
        requires_future_runtime_integration: true,
    },
    Phase41BRequirementCoverageEntry {
        requirement: Phase41BRequirement::GuardianSetIdMatch,
        primary_rejection_case: Some(Phase41BRejectionCase::GuardianSetMismatch),
        closes_phase_40_orphan_rejection_case: false,
        implemented_by_phase_41b: false,
        requires_future_runtime_integration: true,
    },
    Phase41BRequirementCoverageEntry {
        requirement: Phase41BRequirement::GuardianEvidenceUniqueness,
        primary_rejection_case: Some(Phase41BRejectionCase::DuplicateGuardianEvidence),
        closes_phase_40_orphan_rejection_case: true,
        implemented_by_phase_41b: false,
        requires_future_runtime_integration: true,
    },
    Phase41BRequirementCoverageEntry {
        requirement: Phase41BRequirement::SingleCandidateResolution,
        primary_rejection_case: Some(Phase41BRejectionCase::AmbiguousCandidateEvidence),
        closes_phase_40_orphan_rejection_case: true,
        implemented_by_phase_41b: false,
        requires_future_runtime_integration: true,
    },
    Phase41BRequirementCoverageEntry {
        requirement: Phase41BRequirement::ExpirationOrFinalityBinding,
        primary_rejection_case: Some(Phase41BRejectionCase::ExpiredEvidence),
        closes_phase_40_orphan_rejection_case: false,
        implemented_by_phase_41b: false,
        requires_future_runtime_integration: true,
    },
    Phase41BRequirementCoverageEntry {
        requirement: Phase41BRequirement::RouteBinding,
        primary_rejection_case: Some(Phase41BRejectionCase::WrongRoute),
        closes_phase_40_orphan_rejection_case: false,
        implemented_by_phase_41b: false,
        requires_future_runtime_integration: true,
    },
    Phase41BRequirementCoverageEntry {
        requirement: Phase41BRequirement::TargetMintBinding,
        primary_rejection_case: Some(Phase41BRejectionCase::WrongTargetMint),
        closes_phase_40_orphan_rejection_case: false,
        implemented_by_phase_41b: false,
        requires_future_runtime_integration: true,
    },
    Phase41BRequirementCoverageEntry {
        requirement: Phase41BRequirement::RecipientBinding,
        primary_rejection_case: Some(Phase41BRejectionCase::WrongRecipient),
        closes_phase_40_orphan_rejection_case: false,
        implemented_by_phase_41b: false,
        requires_future_runtime_integration: true,
    },
    Phase41BRequirementCoverageEntry {
        requirement: Phase41BRequirement::AmountBinding,
        primary_rejection_case: Some(Phase41BRejectionCase::WrongAmount),
        closes_phase_40_orphan_rejection_case: false,
        implemented_by_phase_41b: false,
        requires_future_runtime_integration: true,
    },
    Phase41BRequirementCoverageEntry {
        requirement: Phase41BRequirement::DeterministicFailureReason,
        primary_rejection_case: None,
        closes_phase_40_orphan_rejection_case: false,
        implemented_by_phase_41b: false,
        requires_future_runtime_integration: true,
    },
];

pub const PHASE_41B_SAFETY_FLAGS: Phase41BSafetyFlags = Phase41BSafetyFlags {
    raw_instructions_sysvar_parser_implemented: false,
    account_info_parser_implemented: false,
    load_instruction_called: false,
    load_instruction_enabled: false,
    concrete_runtime_api_selected: false,
    current_instruction_identity_derived_from_runtime: false,
    ed25519_signature_verification_performed: false,
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

pub fn phase_41b_model_report() -> Phase41BModelReport {
    Phase41BModelReport {
        phase: "41B",
        requirements_declared: PHASE_41B_REQUIREMENTS.len(),
        rejection_cases_declared: PHASE_41B_REJECTION_CASES.len(),
        coverage_entries_declared: PHASE_41B_REQUIREMENT_COVERAGE_MATRIX.len(),
        model_only: true,
        closes_all_phase_40_orphan_rejection_cases: closes_all_phase_40_orphan_rejection_cases(),
        all_rejection_cases_have_owners: all_rejection_cases_have_owners(),
        safety_flags: PHASE_41B_SAFETY_FLAGS,
    }
}

pub fn closes_all_phase_40_orphan_rejection_cases() -> bool {
    has_owner(Phase41BRejectionCase::UnreadableInstructionsSysvar)
        && has_owner(Phase41BRejectionCase::Ed25519InstructionAfterCurrentInstruction)
        && has_owner(Phase41BRejectionCase::DuplicateGuardianEvidence)
        && has_owner(Phase41BRejectionCase::AmbiguousCandidateEvidence)
}

pub fn all_rejection_cases_have_owners() -> bool {
    PHASE_41B_REJECTION_CASES.iter().copied().all(has_owner)
}

pub fn has_owner(rejection_case: Phase41BRejectionCase) -> bool {
    PHASE_41B_REQUIREMENT_COVERAGE_MATRIX
        .iter()
        .any(|entry| entry.primary_rejection_case == Some(rejection_case))
}

pub fn owners_for_rejection_case(
    rejection_case: Phase41BRejectionCase,
) -> impl Iterator<Item = Phase41BRequirement> {
    PHASE_41B_REQUIREMENT_COVERAGE_MATRIX
        .iter()
        .filter(move |entry| entry.primary_rejection_case == Some(rejection_case))
        .map(|entry| entry.requirement)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase_41b_declares_twenty_requirements() {
        assert_eq!(PHASE_41B_REQUIREMENTS.len(), 20);
        assert_eq!(PHASE_41B_REQUIREMENT_COVERAGE_MATRIX.len(), 20);
    }

    #[test]
    fn phase_41b_declares_eighteen_rejection_cases() {
        assert_eq!(PHASE_41B_REJECTION_CASES.len(), 18);
    }

    #[test]
    fn every_rejection_case_has_an_owning_requirement() {
        for rejection_case in PHASE_41B_REJECTION_CASES {
            assert!(
                has_owner(rejection_case),
                "missing owning requirement for {:?}",
                rejection_case
            );
        }
    }

    #[test]
    fn four_phase_40_orphan_rejection_cases_are_closed() {
        assert_eq!(
            owners_for_rejection_case(Phase41BRejectionCase::UnreadableInstructionsSysvar)
                .collect::<Vec<_>>(),
            vec![Phase41BRequirement::InstructionsSysvarReadable]
        );

        assert_eq!(
            owners_for_rejection_case(
                Phase41BRejectionCase::Ed25519InstructionAfterCurrentInstruction
            )
            .collect::<Vec<_>>(),
            vec![Phase41BRequirement::PriorEd25519InstructionOrdering]
        );

        assert_eq!(
            owners_for_rejection_case(Phase41BRejectionCase::DuplicateGuardianEvidence)
                .collect::<Vec<_>>(),
            vec![Phase41BRequirement::GuardianEvidenceUniqueness]
        );

        assert_eq!(
            owners_for_rejection_case(Phase41BRejectionCase::AmbiguousCandidateEvidence)
                .collect::<Vec<_>>(),
            vec![Phase41BRequirement::SingleCandidateResolution]
        );
    }

    #[test]
    fn deterministic_failure_reason_remains_meta_requirement() {
        let entry = PHASE_41B_REQUIREMENT_COVERAGE_MATRIX
            .iter()
            .find(|entry| entry.requirement == Phase41BRequirement::DeterministicFailureReason)
            .expect("deterministic failure requirement must remain declared");

        assert_eq!(entry.primary_rejection_case, None);
        assert!(!entry.implemented_by_phase_41b);
        assert!(entry.requires_future_runtime_integration);
    }

    #[test]
    fn phase_41b_materializes_review_notes_but_implements_no_runtime_integration() {
        for entry in PHASE_41B_REQUIREMENT_COVERAGE_MATRIX {
            assert!(!entry.implemented_by_phase_41b);
            assert!(entry.requires_future_runtime_integration);
        }

        let orphan_closures = PHASE_41B_REQUIREMENT_COVERAGE_MATRIX
            .iter()
            .filter(|entry| entry.closes_phase_40_orphan_rejection_case)
            .count();

        assert_eq!(orphan_closures, 4);
    }

    #[test]
    fn safety_flags_remain_false() {
        let flags = PHASE_41B_SAFETY_FLAGS;

        assert!(!flags.raw_instructions_sysvar_parser_implemented);
        assert!(!flags.account_info_parser_implemented);
        assert!(!flags.load_instruction_called);
        assert!(!flags.load_instruction_enabled);
        assert!(!flags.concrete_runtime_api_selected);
        assert!(!flags.current_instruction_identity_derived_from_runtime);
        assert!(!flags.ed25519_signature_verification_performed);
        assert!(!flags.cryptographic_signature_proof_accepted);
        assert!(!flags.verification_evidence_accepted);
        assert!(!flags.quorum_counting_enabled);
        assert!(!flags.authorization_enabled);
        assert!(!flags.replay_write_enabled);
        assert!(!flags.processed_event_marking_enabled);
        assert!(!flags.account_mutation_enabled);
        assert!(!flags.cpi_enabled);
        assert!(!flags.invoke_signed_enabled);
        assert!(!flags.spl_token_mint_to_enabled);
        assert!(!flags.process_instruction_handler_added);
        assert!(!flags.live_route_enabled);
    }

    #[test]
    fn model_report_preserves_boundary() {
        let report = phase_41b_model_report();

        assert_eq!(report.phase, "41B");
        assert_eq!(report.requirements_declared, 20);
        assert_eq!(report.rejection_cases_declared, 18);
        assert_eq!(report.coverage_entries_declared, 20);
        assert!(report.model_only);
        assert!(report.closes_all_phase_40_orphan_rejection_cases);
        assert!(report.all_rejection_cases_have_owners);

        assert!(!report.safety_flags.verification_evidence_accepted);
        assert!(!report.safety_flags.quorum_counting_enabled);
        assert!(!report.safety_flags.authorization_enabled);
        assert!(!report.safety_flags.spl_token_mint_to_enabled);
        assert!(!report.safety_flags.live_route_enabled);
    }
}
