use super::instructions_sysvar_access_contract_model::{
    Phase41BRejectionCase, Phase41BSafetyFlags,
};

pub const PHASE_41C3_PRIOR_ED25519_LOOKUP_ORDERING_BOUNDARY_VERSION: &str = "0.1.0";
pub const PHASE_41C3_PRIOR_ED25519_LOOKUP_ORDERING_BOUNDARY_PHASE: &str = "41C3";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase41C3PriorEd25519LookupOrderingStatus {
    PriorEd25519InstructionNotFound,
    WrongEd25519ProgramId,
    MalformedStructuralCandidate,
    DuplicateGuardianEvidence,
    Ed25519InstructionNotBeforeCurrentInstruction,
    AmbiguousCandidateEvidence,
    PriorEd25519InstructionLocatedAndOrdered,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phase41C3PriorEd25519CandidateDescriptor {
    pub instruction_index: usize,
    pub program_id_is_ed25519: bool,
    pub structurally_well_formed_candidate: bool,
    pub guardian_evidence_unique: bool,
    pub matches_expected_current_identity_binding: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phase41C3PriorEd25519LookupOrderingResult {
    pub status: Phase41C3PriorEd25519LookupOrderingStatus,
    pub rejection_case: Option<Phase41BRejectionCase>,
    pub current_instruction_index: usize,
    pub candidate_count: usize,
    pub matching_prior_candidate_count: usize,
    pub matched_instruction_index: Option<usize>,
    pub prior_lookup_boundary_enabled: bool,
    pub strict_ordering_enforced: bool,
    pub real_runtime_sysvar_population_deferred: bool,
    pub reads_concrete_instruction_content: bool,
    pub accepts_verification_evidence: bool,
    pub authorizes_execution: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phase41C3PriorEd25519LookupOrderingBoundaryReport {
    pub phase: &'static str,
    pub version: &'static str,
    pub model_or_boundary_only: bool,
    pub prior_lookup_boundary_enabled: bool,
    pub strict_ordering_required: bool,
    pub real_runtime_sysvar_population_deferred: bool,
    pub account_info_parser_implemented: bool,
    pub load_instruction_deferred: bool,
    pub reads_concrete_instruction_content: bool,
    pub accepts_verification_evidence: bool,
    pub authorizes_execution: bool,
    pub allowed_result_count: usize,
    pub safety_flags: Phase41BSafetyFlags,
}

pub const PHASE_41C3_ALLOWED_LOOKUP_ORDERING_STATUSES: [Phase41C3PriorEd25519LookupOrderingStatus;
    7] = [
    Phase41C3PriorEd25519LookupOrderingStatus::PriorEd25519InstructionNotFound,
    Phase41C3PriorEd25519LookupOrderingStatus::WrongEd25519ProgramId,
    Phase41C3PriorEd25519LookupOrderingStatus::MalformedStructuralCandidate,
    Phase41C3PriorEd25519LookupOrderingStatus::DuplicateGuardianEvidence,
    Phase41C3PriorEd25519LookupOrderingStatus::Ed25519InstructionNotBeforeCurrentInstruction,
    Phase41C3PriorEd25519LookupOrderingStatus::AmbiguousCandidateEvidence,
    Phase41C3PriorEd25519LookupOrderingStatus::PriorEd25519InstructionLocatedAndOrdered,
];

pub const PHASE_41C3_SAFETY_FLAGS: Phase41BSafetyFlags = Phase41BSafetyFlags {
    raw_instructions_sysvar_parser_implemented: false,
    account_info_parser_implemented: false,
    load_instruction_called: false,
    load_instruction_enabled: false,
    concrete_runtime_api_selected: true,
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

pub const PHASE_41C3_PRIOR_ED25519_LOOKUP_ORDERING_BOUNDARY_REPORT:
    Phase41C3PriorEd25519LookupOrderingBoundaryReport =
    Phase41C3PriorEd25519LookupOrderingBoundaryReport {
        phase: PHASE_41C3_PRIOR_ED25519_LOOKUP_ORDERING_BOUNDARY_PHASE,
        version: PHASE_41C3_PRIOR_ED25519_LOOKUP_ORDERING_BOUNDARY_VERSION,
        model_or_boundary_only: true,
        prior_lookup_boundary_enabled: true,
        strict_ordering_required: true,
        real_runtime_sysvar_population_deferred: true,
        account_info_parser_implemented: false,
        load_instruction_deferred: true,
        reads_concrete_instruction_content: false,
        accepts_verification_evidence: false,
        authorizes_execution: false,
        allowed_result_count: 7,
        safety_flags: PHASE_41C3_SAFETY_FLAGS,
    };

pub fn locate_prior_ed25519_lookup_ordering_boundary(
    current_instruction_index: usize,
    candidates: &[Phase41C3PriorEd25519CandidateDescriptor],
) -> Phase41C3PriorEd25519LookupOrderingResult {
    if candidates.is_empty() {
        return result(
            Phase41C3PriorEd25519LookupOrderingStatus::PriorEd25519InstructionNotFound,
            Some(Phase41BRejectionCase::Ed25519InstructionNotFound),
            current_instruction_index,
            candidates.len(),
            0,
            None,
        );
    }

    let mut first_wrong_program_id_index = None;
    let mut first_malformed_candidate_index = None;
    let mut first_duplicate_guardian_evidence_index = None;
    let mut first_not_before_current_index = None;
    let mut first_matching_prior_index = None;
    let mut matching_prior_candidate_count = 0usize;

    for candidate in candidates.iter() {
        if !candidate.program_id_is_ed25519 {
            if first_wrong_program_id_index.is_none() {
                first_wrong_program_id_index = Some(candidate.instruction_index);
            }
            continue;
        }

        if !candidate.structurally_well_formed_candidate {
            if first_malformed_candidate_index.is_none() {
                first_malformed_candidate_index = Some(candidate.instruction_index);
            }
            continue;
        }

        if !candidate.guardian_evidence_unique {
            if first_duplicate_guardian_evidence_index.is_none() {
                first_duplicate_guardian_evidence_index = Some(candidate.instruction_index);
            }
            continue;
        }

        if !candidate.matches_expected_current_identity_binding {
            continue;
        }

        if candidate.instruction_index >= current_instruction_index {
            if first_not_before_current_index.is_none() {
                first_not_before_current_index = Some(candidate.instruction_index);
            }
            continue;
        }

        matching_prior_candidate_count += 1;

        if first_matching_prior_index.is_none() {
            first_matching_prior_index = Some(candidate.instruction_index);
        }
    }

    if first_duplicate_guardian_evidence_index.is_some() {
        return result(
            Phase41C3PriorEd25519LookupOrderingStatus::DuplicateGuardianEvidence,
            Some(Phase41BRejectionCase::DuplicateGuardianEvidence),
            current_instruction_index,
            candidates.len(),
            matching_prior_candidate_count,
            None,
        );
    }

    if first_malformed_candidate_index.is_some() {
        return result(
            Phase41C3PriorEd25519LookupOrderingStatus::MalformedStructuralCandidate,
            Some(Phase41BRejectionCase::MalformedEd25519InstructionData),
            current_instruction_index,
            candidates.len(),
            matching_prior_candidate_count,
            None,
        );
    }

    if matching_prior_candidate_count > 1 {
        return result(
            Phase41C3PriorEd25519LookupOrderingStatus::AmbiguousCandidateEvidence,
            Some(Phase41BRejectionCase::AmbiguousCandidateEvidence),
            current_instruction_index,
            candidates.len(),
            matching_prior_candidate_count,
            first_matching_prior_index,
        );
    }

    if let Some(matched_instruction_index) = first_matching_prior_index {
        return result(
            Phase41C3PriorEd25519LookupOrderingStatus::PriorEd25519InstructionLocatedAndOrdered,
            None,
            current_instruction_index,
            candidates.len(),
            matching_prior_candidate_count,
            Some(matched_instruction_index),
        );
    }

    if first_not_before_current_index.is_some() {
        return result(
            Phase41C3PriorEd25519LookupOrderingStatus::Ed25519InstructionNotBeforeCurrentInstruction,
            Some(Phase41BRejectionCase::Ed25519InstructionAfterCurrentInstruction),
            current_instruction_index,
            candidates.len(),
            matching_prior_candidate_count,
            None,
        );
    }

    if first_wrong_program_id_index.is_some() {
        return result(
            Phase41C3PriorEd25519LookupOrderingStatus::WrongEd25519ProgramId,
            Some(Phase41BRejectionCase::WrongEd25519ProgramId),
            current_instruction_index,
            candidates.len(),
            matching_prior_candidate_count,
            None,
        );
    }

    result(
        Phase41C3PriorEd25519LookupOrderingStatus::PriorEd25519InstructionNotFound,
        Some(Phase41BRejectionCase::Ed25519InstructionNotFound),
        current_instruction_index,
        candidates.len(),
        matching_prior_candidate_count,
        None,
    )
}

pub fn phase_41c3_prior_ed25519_lookup_ordering_boundary_report(
) -> Phase41C3PriorEd25519LookupOrderingBoundaryReport {
    PHASE_41C3_PRIOR_ED25519_LOOKUP_ORDERING_BOUNDARY_REPORT
}

fn result(
    status: Phase41C3PriorEd25519LookupOrderingStatus,
    rejection_case: Option<Phase41BRejectionCase>,
    current_instruction_index: usize,
    candidate_count: usize,
    matching_prior_candidate_count: usize,
    matched_instruction_index: Option<usize>,
) -> Phase41C3PriorEd25519LookupOrderingResult {
    Phase41C3PriorEd25519LookupOrderingResult {
        status,
        rejection_case,
        current_instruction_index,
        candidate_count,
        matching_prior_candidate_count,
        matched_instruction_index,
        prior_lookup_boundary_enabled: true,
        strict_ordering_enforced: true,
        real_runtime_sysvar_population_deferred: true,
        reads_concrete_instruction_content: false,
        accepts_verification_evidence: false,
        authorizes_execution: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn matching_candidate_at(instruction_index: usize) -> Phase41C3PriorEd25519CandidateDescriptor {
        Phase41C3PriorEd25519CandidateDescriptor {
            instruction_index,
            program_id_is_ed25519: true,
            structurally_well_formed_candidate: true,
            guardian_evidence_unique: true,
            matches_expected_current_identity_binding: true,
        }
    }

    #[test]
    fn empty_candidate_set_maps_to_ed25519_not_found() {
        let result = locate_prior_ed25519_lookup_ordering_boundary(3, &[]);

        assert_eq!(
            result.status,
            Phase41C3PriorEd25519LookupOrderingStatus::PriorEd25519InstructionNotFound
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::Ed25519InstructionNotFound)
        );
        assert_eq!(result.candidate_count, 0);
        assert_eq!(result.matching_prior_candidate_count, 0);
        assert_eq!(result.matched_instruction_index, None);
        assert!(result.prior_lookup_boundary_enabled);
        assert!(result.strict_ordering_enforced);
        assert!(result.real_runtime_sysvar_population_deferred);
        assert!(!result.reads_concrete_instruction_content);
        assert!(!result.accepts_verification_evidence);
        assert!(!result.authorizes_execution);
    }

    #[test]
    fn wrong_program_id_maps_to_phase_41b_wrong_program_id() {
        let candidates = [Phase41C3PriorEd25519CandidateDescriptor {
            instruction_index: 1,
            program_id_is_ed25519: false,
            structurally_well_formed_candidate: true,
            guardian_evidence_unique: true,
            matches_expected_current_identity_binding: true,
        }];

        let result = locate_prior_ed25519_lookup_ordering_boundary(3, &candidates);

        assert_eq!(
            result.status,
            Phase41C3PriorEd25519LookupOrderingStatus::WrongEd25519ProgramId
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::WrongEd25519ProgramId)
        );
        assert_eq!(result.matched_instruction_index, None);
    }

    #[test]
    fn malformed_candidate_maps_to_phase_41b_malformed_instruction_data() {
        let candidates = [Phase41C3PriorEd25519CandidateDescriptor {
            instruction_index: 1,
            program_id_is_ed25519: true,
            structurally_well_formed_candidate: false,
            guardian_evidence_unique: true,
            matches_expected_current_identity_binding: true,
        }];

        let result = locate_prior_ed25519_lookup_ordering_boundary(3, &candidates);

        assert_eq!(
            result.status,
            Phase41C3PriorEd25519LookupOrderingStatus::MalformedStructuralCandidate
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::MalformedEd25519InstructionData)
        );
        assert_eq!(result.matched_instruction_index, None);
    }

    #[test]
    fn duplicate_guardian_evidence_maps_to_phase_41b_duplicate_guardian_evidence() {
        let candidates = [Phase41C3PriorEd25519CandidateDescriptor {
            instruction_index: 1,
            program_id_is_ed25519: true,
            structurally_well_formed_candidate: true,
            guardian_evidence_unique: false,
            matches_expected_current_identity_binding: true,
        }];

        let result = locate_prior_ed25519_lookup_ordering_boundary(3, &candidates);

        assert_eq!(
            result.status,
            Phase41C3PriorEd25519LookupOrderingStatus::DuplicateGuardianEvidence
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::DuplicateGuardianEvidence)
        );
        assert_eq!(result.matched_instruction_index, None);
    }

    #[test]
    fn same_or_later_instruction_maps_to_phase_41b_after_current_instruction() {
        let candidates = [matching_candidate_at(3)];

        let result = locate_prior_ed25519_lookup_ordering_boundary(3, &candidates);

        assert_eq!(
            result.status,
            Phase41C3PriorEd25519LookupOrderingStatus::Ed25519InstructionNotBeforeCurrentInstruction
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::Ed25519InstructionAfterCurrentInstruction)
        );
        assert_eq!(result.matched_instruction_index, None);
    }

    #[test]
    fn ambiguous_prior_matches_map_to_phase_41b_ambiguous_candidate_evidence() {
        let candidates = [matching_candidate_at(1), matching_candidate_at(2)];

        let result = locate_prior_ed25519_lookup_ordering_boundary(3, &candidates);

        assert_eq!(
            result.status,
            Phase41C3PriorEd25519LookupOrderingStatus::AmbiguousCandidateEvidence
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::AmbiguousCandidateEvidence)
        );
        assert_eq!(result.matching_prior_candidate_count, 2);
        assert_eq!(result.matched_instruction_index, Some(1));
    }

    #[test]
    fn single_prior_match_is_non_authorizing() {
        let candidates = [matching_candidate_at(1)];

        let result = locate_prior_ed25519_lookup_ordering_boundary(3, &candidates);

        assert_eq!(
            result.status,
            Phase41C3PriorEd25519LookupOrderingStatus::PriorEd25519InstructionLocatedAndOrdered
        );
        assert_eq!(result.rejection_case, None);
        assert_eq!(result.matching_prior_candidate_count, 1);
        assert_eq!(result.matched_instruction_index, Some(1));
        assert!(result.prior_lookup_boundary_enabled);
        assert!(result.strict_ordering_enforced);
        assert!(result.real_runtime_sysvar_population_deferred);
        assert!(!result.reads_concrete_instruction_content);
        assert!(!result.accepts_verification_evidence);
        assert!(!result.authorizes_execution);
    }

    #[test]
    fn non_matching_ed25519_candidate_maps_to_not_found() {
        let candidates = [Phase41C3PriorEd25519CandidateDescriptor {
            instruction_index: 1,
            program_id_is_ed25519: true,
            structurally_well_formed_candidate: true,
            guardian_evidence_unique: true,
            matches_expected_current_identity_binding: false,
        }];

        let result = locate_prior_ed25519_lookup_ordering_boundary(3, &candidates);

        assert_eq!(
            result.status,
            Phase41C3PriorEd25519LookupOrderingStatus::PriorEd25519InstructionNotFound
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::Ed25519InstructionNotFound)
        );
        assert_eq!(result.matched_instruction_index, None);
    }

    #[test]
    fn phase_41c3_declares_only_seven_structural_results() {
        assert_eq!(PHASE_41C3_ALLOWED_LOOKUP_ORDERING_STATUSES.len(), 7);
        assert_eq!(
            PHASE_41C3_ALLOWED_LOOKUP_ORDERING_STATUSES,
            [
                Phase41C3PriorEd25519LookupOrderingStatus::PriorEd25519InstructionNotFound,
                Phase41C3PriorEd25519LookupOrderingStatus::WrongEd25519ProgramId,
                Phase41C3PriorEd25519LookupOrderingStatus::MalformedStructuralCandidate,
                Phase41C3PriorEd25519LookupOrderingStatus::DuplicateGuardianEvidence,
                Phase41C3PriorEd25519LookupOrderingStatus::Ed25519InstructionNotBeforeCurrentInstruction,
                Phase41C3PriorEd25519LookupOrderingStatus::AmbiguousCandidateEvidence,
                Phase41C3PriorEd25519LookupOrderingStatus::PriorEd25519InstructionLocatedAndOrdered,
            ]
        );
    }

    #[test]
    fn phase_41c3_safety_flags_keep_real_runtime_read_deferred() {
        let flags = PHASE_41C3_SAFETY_FLAGS;

        assert!(!flags.raw_instructions_sysvar_parser_implemented);
        assert!(!flags.account_info_parser_implemented);
        assert!(!flags.load_instruction_called);
        assert!(!flags.load_instruction_enabled);
        assert!(flags.concrete_runtime_api_selected);
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
    fn phase_41c3_report_preserves_boundary() {
        let report = phase_41c3_prior_ed25519_lookup_ordering_boundary_report();

        assert_eq!(report.phase, "41C3");
        assert_eq!(report.version, "0.1.0");
        assert!(report.model_or_boundary_only);
        assert!(report.prior_lookup_boundary_enabled);
        assert!(report.strict_ordering_required);
        assert!(report.real_runtime_sysvar_population_deferred);
        assert!(!report.account_info_parser_implemented);
        assert!(report.load_instruction_deferred);
        assert!(!report.reads_concrete_instruction_content);
        assert!(!report.accepts_verification_evidence);
        assert!(!report.authorizes_execution);
        assert_eq!(report.allowed_result_count, 7);

        assert!(report.safety_flags.concrete_runtime_api_selected);
        assert!(
            !report
                .safety_flags
                .current_instruction_identity_derived_from_runtime
        );
        assert!(!report.safety_flags.load_instruction_called);
        assert!(!report.safety_flags.load_instruction_enabled);
        assert!(!report.safety_flags.verification_evidence_accepted);
        assert!(!report.safety_flags.quorum_counting_enabled);
        assert!(!report.safety_flags.authorization_enabled);
        assert!(!report.safety_flags.spl_token_mint_to_enabled);
        assert!(!report.safety_flags.live_route_enabled);
    }
}
