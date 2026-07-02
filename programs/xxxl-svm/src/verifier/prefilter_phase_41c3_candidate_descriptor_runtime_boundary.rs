use solana_program::ed25519_program;

use super::checked_prior_instruction_loading_runtime_boundary::{
    Phase41D3_2_2CheckedPriorInstructionLoadingResult,
    Phase41D3_2_2CheckedPriorInstructionLoadingStatus,
};
use super::instructions_sysvar_access_contract_model::{
    Phase41BRejectionCase, Phase41BSafetyFlags,
};
use super::prior_ed25519_lookup_ordering_boundary::{
    locate_prior_ed25519_lookup_ordering_boundary, Phase41C3PriorEd25519CandidateDescriptor,
    Phase41C3PriorEd25519LookupOrderingResult, Phase41C3PriorEd25519LookupOrderingStatus,
};

pub const PHASE_41D3_2_3_PREFILTER_DESCRIPTOR_RUNTIME_BOUNDARY_VERSION: &str = "0.1.0";
pub const PHASE_41D3_2_3_PREFILTER_DESCRIPTOR_RUNTIME_BOUNDARY_PHASE: &str = "41D3.2.3";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase41D3_2_3PrefilterDescriptorStatus {
    LoadedPriorInstructionsUnavailable,
    NoLoadedPriorInstructions,
    NoPriorEd25519CandidateDescriptors,
    SameOrLaterCandidateRejectedByPhase41C3,
    CandidateDescriptorRejectedByPhase41C3,
    PriorEd25519InstructionStructurallyLocated,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Phase41D3_2_3PrefilterDescriptorResult {
    pub status: Phase41D3_2_3PrefilterDescriptorStatus,
    pub rejection_case: Option<Phase41BRejectionCase>,
    pub current_instruction_index: Option<usize>,
    pub loaded_prior_instruction_count: usize,
    pub inspected_loaded_entry_count: usize,
    pub discarded_non_candidate_count: usize,
    pub candidate_descriptor_count: usize,
    pub matched_instruction_index: Option<usize>,
    pub phase_41c3_result: Option<Phase41C3PriorEd25519LookupOrderingResult>,
    pub phase_41c3_candidate_descriptors: Vec<Phase41C3PriorEd25519CandidateDescriptor>,
    pub consumes_phase_41d3_2_2_loaded_prior_instructions: bool,
    pub processes_runtime_data_only_entries: bool,
    pub iterates_loaded_entries_by_reference: bool,
    pub prefilter_enabled: bool,
    pub prefilter_by_ed25519_program_id_only: bool,
    pub discards_non_candidates_immediately: bool,
    pub stores_candidate_metadata_only: bool,
    pub delegates_ordering_ambiguity_to_phase_41c3: bool,
    pub explicit_same_index_reject_boundary: bool,
    pub explicit_later_index_reject_boundary: bool,
    pub phase_41c3_descriptor_construction_enabled: bool,
    pub locates_prior_ed25519_instruction: bool,
    pub ed25519_signature_verification_performed: bool,
    pub cryptographic_signature_proof_accepted: bool,
    pub accepts_verification_evidence: bool,
    pub authorizes_execution: bool,
    pub mutates_runtime_state: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phase41D3_2_3PrefilterDescriptorBoundaryReport {
    pub phase: &'static str,
    pub version: &'static str,
    pub consumes_phase_41d3_2_2_loaded_prior_instructions: bool,
    pub processes_runtime_data_only_entries: bool,
    pub iterates_loaded_entries_by_reference: bool,
    pub prefilter_enabled: bool,
    pub prefilter_by_ed25519_program_id_only: bool,
    pub discards_non_candidates_immediately: bool,
    pub stores_candidate_metadata_only: bool,
    pub delegates_ordering_ambiguity_to_phase_41c3: bool,
    pub explicit_same_index_reject_boundary: bool,
    pub explicit_later_index_reject_boundary: bool,
    pub phase_41c3_descriptor_construction_enabled: bool,
    pub locates_prior_ed25519_instruction: bool,
    pub ed25519_signature_verification_performed: bool,
    pub cryptographic_signature_proof_accepted: bool,
    pub accepts_verification_evidence: bool,
    pub authorizes_execution: bool,
    pub mutates_runtime_state: bool,
    pub safety_flags: Phase41BSafetyFlags,
}

pub const PHASE_41D3_2_3_SAFETY_FLAGS: Phase41BSafetyFlags = Phase41BSafetyFlags {
    raw_instructions_sysvar_parser_implemented: false,
    account_info_parser_implemented: true,
    load_instruction_called: true,
    load_instruction_enabled: true,
    concrete_runtime_api_selected: true,
    current_instruction_identity_derived_from_runtime: true,
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

pub const PHASE_41D3_2_3_PREFILTER_DESCRIPTOR_RUNTIME_BOUNDARY_REPORT:
    Phase41D3_2_3PrefilterDescriptorBoundaryReport =
    Phase41D3_2_3PrefilterDescriptorBoundaryReport {
        phase: PHASE_41D3_2_3_PREFILTER_DESCRIPTOR_RUNTIME_BOUNDARY_PHASE,
        version: PHASE_41D3_2_3_PREFILTER_DESCRIPTOR_RUNTIME_BOUNDARY_VERSION,
        consumes_phase_41d3_2_2_loaded_prior_instructions: true,
        processes_runtime_data_only_entries: true,
        iterates_loaded_entries_by_reference: true,
        prefilter_enabled: true,
        prefilter_by_ed25519_program_id_only: true,
        discards_non_candidates_immediately: true,
        stores_candidate_metadata_only: true,
        delegates_ordering_ambiguity_to_phase_41c3: true,
        explicit_same_index_reject_boundary: true,
        explicit_later_index_reject_boundary: true,
        phase_41c3_descriptor_construction_enabled: true,
        locates_prior_ed25519_instruction: true,
        ed25519_signature_verification_performed: false,
        cryptographic_signature_proof_accepted: false,
        accepts_verification_evidence: false,
        authorizes_execution: false,
        mutates_runtime_state: false,
        safety_flags: PHASE_41D3_2_3_SAFETY_FLAGS,
    };

pub fn prefilter_loaded_prior_instructions_into_phase_41c3_descriptors(
    loading_result: &Phase41D3_2_2CheckedPriorInstructionLoadingResult,
) -> Phase41D3_2_3PrefilterDescriptorResult {
    if loading_result.status
        != Phase41D3_2_2CheckedPriorInstructionLoadingStatus::CheckedPriorInstructionsLoaded
        && loading_result.status
            != Phase41D3_2_2CheckedPriorInstructionLoadingStatus::EmptyPriorRangeNoLoadingAttempted
    {
        return result(
            Phase41D3_2_3PrefilterDescriptorStatus::LoadedPriorInstructionsUnavailable,
            loading_result
                .rejection_case
                .or(Some(Phase41BRejectionCase::UnreadableInstructionsSysvar)),
            loading_result.current_instruction_index,
            loading_result.loaded_instruction_count,
            0,
            0,
            Vec::new(),
            None,
            false,
        );
    }

    let Some(current_instruction_index) = loading_result.current_instruction_index else {
        return result(
            Phase41D3_2_3PrefilterDescriptorStatus::LoadedPriorInstructionsUnavailable,
            Some(Phase41BRejectionCase::UnreadableInstructionsSysvar),
            None,
            loading_result.loaded_instruction_count,
            0,
            0,
            Vec::new(),
            None,
            false,
        );
    };

    if loading_result.loaded_prior_instructions.is_empty() {
        let phase_41c3_result =
            locate_prior_ed25519_lookup_ordering_boundary(current_instruction_index, &[]);

        return result_from_phase_41c3(
            Phase41D3_2_3PrefilterDescriptorStatus::NoLoadedPriorInstructions,
            current_instruction_index,
            loading_result.loaded_instruction_count,
            0,
            0,
            Vec::new(),
            phase_41c3_result,
        );
    }

    let mut phase_41c3_candidate_descriptors = Vec::new();
    let mut inspected_loaded_entry_count = 0usize;
    let mut discarded_non_candidate_count = 0usize;

    for loaded_entry in loading_result.loaded_prior_instructions.iter() {
        inspected_loaded_entry_count += 1;

        if !loaded_entry.loaded_instruction_is_runtime_data_only
            || loaded_entry.is_evidence
            || loaded_entry.authorizes_execution
        {
            return result(
                Phase41D3_2_3PrefilterDescriptorStatus::LoadedPriorInstructionsUnavailable,
                Some(Phase41BRejectionCase::UnreadableInstructionsSysvar),
                Some(current_instruction_index),
                loading_result.loaded_instruction_count,
                inspected_loaded_entry_count,
                discarded_non_candidate_count,
                Vec::new(),
                None,
                false,
            );
        }

        if loaded_entry.instruction.program_id != ed25519_program::id() {
            discarded_non_candidate_count += 1;
            continue;
        }

        phase_41c3_candidate_descriptors.push(phase_41c3_descriptor_for_loaded_ed25519_candidate(
            loaded_entry.instruction_index,
        ));
    }

    let phase_41c3_result = locate_prior_ed25519_lookup_ordering_boundary(
        current_instruction_index,
        &phase_41c3_candidate_descriptors,
    );

    result_from_phase_41c3(
        status_from_phase_41c3(&phase_41c3_result),
        current_instruction_index,
        loading_result.loaded_instruction_count,
        inspected_loaded_entry_count,
        discarded_non_candidate_count,
        phase_41c3_candidate_descriptors,
        phase_41c3_result,
    )
}

pub fn phase_41d3_2_3_prefilter_descriptor_runtime_boundary_report(
) -> Phase41D3_2_3PrefilterDescriptorBoundaryReport {
    PHASE_41D3_2_3_PREFILTER_DESCRIPTOR_RUNTIME_BOUNDARY_REPORT
}

fn phase_41c3_descriptor_for_loaded_ed25519_candidate(
    instruction_index: usize,
) -> Phase41C3PriorEd25519CandidateDescriptor {
    Phase41C3PriorEd25519CandidateDescriptor {
        instruction_index,
        program_id_is_ed25519: true,
        structurally_well_formed_candidate: true,
        guardian_evidence_unique: true,
        matches_expected_current_identity_binding: true,
    }
}

fn status_from_phase_41c3(
    phase_41c3_result: &Phase41C3PriorEd25519LookupOrderingResult,
) -> Phase41D3_2_3PrefilterDescriptorStatus {
    match phase_41c3_result.status {
        Phase41C3PriorEd25519LookupOrderingStatus::PriorEd25519InstructionLocatedAndOrdered => {
            Phase41D3_2_3PrefilterDescriptorStatus::PriorEd25519InstructionStructurallyLocated
        }
        Phase41C3PriorEd25519LookupOrderingStatus::Ed25519InstructionNotBeforeCurrentInstruction => {
            Phase41D3_2_3PrefilterDescriptorStatus::SameOrLaterCandidateRejectedByPhase41C3
        }
        Phase41C3PriorEd25519LookupOrderingStatus::PriorEd25519InstructionNotFound
        | Phase41C3PriorEd25519LookupOrderingStatus::WrongEd25519ProgramId => {
            Phase41D3_2_3PrefilterDescriptorStatus::NoPriorEd25519CandidateDescriptors
        }
        Phase41C3PriorEd25519LookupOrderingStatus::MalformedStructuralCandidate
        | Phase41C3PriorEd25519LookupOrderingStatus::DuplicateGuardianEvidence
        | Phase41C3PriorEd25519LookupOrderingStatus::AmbiguousCandidateEvidence => {
            Phase41D3_2_3PrefilterDescriptorStatus::CandidateDescriptorRejectedByPhase41C3
        }
    }
}

fn result_from_phase_41c3(
    status: Phase41D3_2_3PrefilterDescriptorStatus,
    current_instruction_index: usize,
    loaded_prior_instruction_count: usize,
    inspected_loaded_entry_count: usize,
    discarded_non_candidate_count: usize,
    phase_41c3_candidate_descriptors: Vec<Phase41C3PriorEd25519CandidateDescriptor>,
    phase_41c3_result: Phase41C3PriorEd25519LookupOrderingResult,
) -> Phase41D3_2_3PrefilterDescriptorResult {
    let rejection_case = phase_41c3_result.rejection_case;
    let matched_instruction_index = phase_41c3_result.matched_instruction_index;

    result(
        status,
        rejection_case,
        Some(current_instruction_index),
        loaded_prior_instruction_count,
        inspected_loaded_entry_count,
        discarded_non_candidate_count,
        phase_41c3_candidate_descriptors,
        Some(phase_41c3_result),
        true,
    )
}

fn result(
    status: Phase41D3_2_3PrefilterDescriptorStatus,
    rejection_case: Option<Phase41BRejectionCase>,
    current_instruction_index: Option<usize>,
    loaded_prior_instruction_count: usize,
    inspected_loaded_entry_count: usize,
    discarded_non_candidate_count: usize,
    phase_41c3_candidate_descriptors: Vec<Phase41C3PriorEd25519CandidateDescriptor>,
    phase_41c3_result: Option<Phase41C3PriorEd25519LookupOrderingResult>,
    locates_prior_ed25519_instruction: bool,
) -> Phase41D3_2_3PrefilterDescriptorResult {
    let candidate_descriptor_count = phase_41c3_candidate_descriptors.len();
    let matched_instruction_index = phase_41c3_result
        .as_ref()
        .and_then(|ordering_result| ordering_result.matched_instruction_index);

    Phase41D3_2_3PrefilterDescriptorResult {
        status,
        rejection_case,
        current_instruction_index,
        loaded_prior_instruction_count,
        inspected_loaded_entry_count,
        discarded_non_candidate_count,
        candidate_descriptor_count,
        matched_instruction_index,
        phase_41c3_result,
        phase_41c3_candidate_descriptors,
        consumes_phase_41d3_2_2_loaded_prior_instructions: true,
        processes_runtime_data_only_entries: true,
        iterates_loaded_entries_by_reference: true,
        prefilter_enabled: true,
        prefilter_by_ed25519_program_id_only: true,
        discards_non_candidates_immediately: true,
        stores_candidate_metadata_only: true,
        delegates_ordering_ambiguity_to_phase_41c3: true,
        explicit_same_index_reject_boundary: true,
        explicit_later_index_reject_boundary: true,
        phase_41c3_descriptor_construction_enabled: true,
        locates_prior_ed25519_instruction,
        ed25519_signature_verification_performed: false,
        cryptographic_signature_proof_accepted: false,
        accepts_verification_evidence: false,
        authorizes_execution: false,
        mutates_runtime_state: false,
    }
}

#[cfg(test)]
mod tests {
    use solana_program::{instruction::Instruction, pubkey::Pubkey};

    use super::super::checked_prior_instruction_loading_runtime_boundary::{
        Phase41D3_2_2CheckedPriorInstructionLoadingResult,
        Phase41D3_2_2CheckedPriorInstructionLoadingStatus, Phase41D3_2_2LoadedPriorInstruction,
    };
    use super::*;

    fn non_ed25519_instruction() -> Instruction {
        Instruction {
            program_id: Pubkey::new_from_array([7; 32]),
            accounts: Vec::new(),
            data: vec![1, 2, 3],
        }
    }

    fn ed25519_instruction() -> Instruction {
        Instruction {
            program_id: ed25519_program::id(),
            accounts: Vec::new(),
            data: vec![4, 5, 6],
        }
    }

    fn loaded_prior_instruction(
        instruction_index: usize,
        instruction: Instruction,
    ) -> Phase41D3_2_2LoadedPriorInstruction {
        Phase41D3_2_2LoadedPriorInstruction {
            instruction_index,
            instruction,
            loaded_instruction_is_runtime_data_only: true,
            is_evidence: false,
            authorizes_execution: false,
        }
    }

    fn loading_result(
        current_instruction_index: usize,
        loaded_prior_instructions: Vec<Phase41D3_2_2LoadedPriorInstruction>,
    ) -> Phase41D3_2_2CheckedPriorInstructionLoadingResult {
        Phase41D3_2_2CheckedPriorInstructionLoadingResult {
            status:
                Phase41D3_2_2CheckedPriorInstructionLoadingStatus::CheckedPriorInstructionsLoaded,
            rejection_case: None,
            current_instruction_index: Some(current_instruction_index),
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

    fn empty_loading_result(
        current_instruction_index: usize,
    ) -> Phase41D3_2_2CheckedPriorInstructionLoadingResult {
        Phase41D3_2_2CheckedPriorInstructionLoadingResult {
            status:
                Phase41D3_2_2CheckedPriorInstructionLoadingStatus::EmptyPriorRangeNoLoadingAttempted,
            rejection_case: None,
            current_instruction_index: Some(current_instruction_index),
            prior_index_count: 0,
            attempted_loading_count: 0,
            loaded_instruction_count: 0,
            failed_instruction_index: None,
            loaded_prior_instructions: Vec::new(),
            consumes_phase_41d3_2_1_prior_range: true,
            uses_instructions_sysvar_account_info: false,
            checks_instructions_sysvar_program_id: false,
            iterates_prior_indexes_lazily: true,
            empty_prior_range_causes_no_loading_attempt: true,
            checked_prior_instruction_loading_enabled: true,
            prior_instruction_loading_enabled: true,
            raw_instructions_sysvar_parser_implemented: false,
            load_instruction_called: false,
            load_instruction_enabled: true,
            load_instruction_at_checked_used: false,
            unchecked_instruction_loading_used: false,
            prefilter_enabled: false,
            phase_41c3_descriptor_construction_enabled: false,
            locates_prior_ed25519_instruction: false,
            accepts_verification_evidence: false,
            authorizes_execution: false,
            mutates_runtime_state: false,
        }
    }

    #[test]
    fn empty_loaded_prior_instruction_list_delegates_to_phase_41c3_not_found() {
        let loading_result = empty_loading_result(0);

        let result =
            prefilter_loaded_prior_instructions_into_phase_41c3_descriptors(&loading_result);

        assert_eq!(
            result.status,
            Phase41D3_2_3PrefilterDescriptorStatus::NoLoadedPriorInstructions
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::Ed25519InstructionNotFound)
        );
        assert_eq!(result.candidate_descriptor_count, 0);
        assert_eq!(result.inspected_loaded_entry_count, 0);
        assert_eq!(result.discarded_non_candidate_count, 0);
        assert!(result.phase_41c3_result.is_some());
        assert!(result.phase_41c3_candidate_descriptors.is_empty());
        assert!(result.prefilter_enabled);
        assert!(result.delegates_ordering_ambiguity_to_phase_41c3);
        assert!(result.phase_41c3_descriptor_construction_enabled);
        assert!(result.locates_prior_ed25519_instruction);
        assert!(!result.accepts_verification_evidence);
        assert!(!result.authorizes_execution);
        assert!(!result.mutates_runtime_state);
    }

    #[test]
    fn unrelated_loaded_instruction_is_discarded_without_candidate_descriptor() {
        let loading_result = loading_result(
            2,
            vec![loaded_prior_instruction(0, non_ed25519_instruction())],
        );

        let result =
            prefilter_loaded_prior_instructions_into_phase_41c3_descriptors(&loading_result);

        assert_eq!(
            result.status,
            Phase41D3_2_3PrefilterDescriptorStatus::NoPriorEd25519CandidateDescriptors
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::Ed25519InstructionNotFound)
        );
        assert_eq!(result.inspected_loaded_entry_count, 1);
        assert_eq!(result.discarded_non_candidate_count, 1);
        assert_eq!(result.candidate_descriptor_count, 0);
        assert!(result.phase_41c3_candidate_descriptors.is_empty());
        assert!(result.discards_non_candidates_immediately);
        assert!(!result.accepts_verification_evidence);
        assert!(!result.authorizes_execution);
    }

    #[test]
    fn ed25519_program_id_match_creates_non_authorizing_phase_41c3_descriptor() {
        let loading_result = loading_result(
            3,
            vec![
                loaded_prior_instruction(0, non_ed25519_instruction()),
                loaded_prior_instruction(1, ed25519_instruction()),
            ],
        );

        let result =
            prefilter_loaded_prior_instructions_into_phase_41c3_descriptors(&loading_result);

        assert_eq!(
            result.status,
            Phase41D3_2_3PrefilterDescriptorStatus::PriorEd25519InstructionStructurallyLocated
        );
        assert_eq!(result.rejection_case, None);
        assert_eq!(result.inspected_loaded_entry_count, 2);
        assert_eq!(result.discarded_non_candidate_count, 1);
        assert_eq!(result.candidate_descriptor_count, 1);
        assert_eq!(result.matched_instruction_index, Some(1));
        assert!(result.locates_prior_ed25519_instruction);
        assert!(!result.ed25519_signature_verification_performed);
        assert!(!result.cryptographic_signature_proof_accepted);
        assert!(!result.accepts_verification_evidence);
        assert!(!result.authorizes_execution);
        assert!(!result.mutates_runtime_state);

        match result.phase_41c3_candidate_descriptors.first() {
            Some(descriptor) => {
                assert_eq!(descriptor.instruction_index, 1);
                assert!(descriptor.program_id_is_ed25519);
                assert!(descriptor.structurally_well_formed_candidate);
            }
            None => {
                assert_eq!(result.candidate_descriptor_count, 1);
                assert!(!result.phase_41c3_candidate_descriptors.is_empty());
            }
        }
    }

    #[test]
    fn same_index_ed25519_candidate_is_rejected_by_phase_41c3() {
        let loading_result =
            loading_result(1, vec![loaded_prior_instruction(1, ed25519_instruction())]);

        let result =
            prefilter_loaded_prior_instructions_into_phase_41c3_descriptors(&loading_result);

        assert_eq!(
            result.status,
            Phase41D3_2_3PrefilterDescriptorStatus::SameOrLaterCandidateRejectedByPhase41C3
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::Ed25519InstructionAfterCurrentInstruction)
        );
        assert_eq!(result.candidate_descriptor_count, 1);
        assert_eq!(result.matched_instruction_index, None);
        assert!(result.explicit_same_index_reject_boundary);
        assert!(result.explicit_later_index_reject_boundary);
        assert!(!result.accepts_verification_evidence);
        assert!(!result.authorizes_execution);
    }

    #[test]
    fn later_index_ed25519_candidate_is_rejected_by_phase_41c3() {
        let loading_result =
            loading_result(1, vec![loaded_prior_instruction(2, ed25519_instruction())]);

        let result =
            prefilter_loaded_prior_instructions_into_phase_41c3_descriptors(&loading_result);

        assert_eq!(
            result.status,
            Phase41D3_2_3PrefilterDescriptorStatus::SameOrLaterCandidateRejectedByPhase41C3
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::Ed25519InstructionAfterCurrentInstruction)
        );
        assert_eq!(result.candidate_descriptor_count, 1);
        assert_eq!(result.matched_instruction_index, None);
        assert!(!result.accepts_verification_evidence);
        assert!(!result.authorizes_execution);
    }

    #[test]
    fn duplicate_ed25519_candidates_are_delegated_to_phase_41c3_ambiguity_model() {
        let loading_result = loading_result(
            5,
            vec![
                loaded_prior_instruction(1, ed25519_instruction()),
                loaded_prior_instruction(2, ed25519_instruction()),
            ],
        );

        let result =
            prefilter_loaded_prior_instructions_into_phase_41c3_descriptors(&loading_result);

        assert_eq!(
            result.status,
            Phase41D3_2_3PrefilterDescriptorStatus::CandidateDescriptorRejectedByPhase41C3
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::AmbiguousCandidateEvidence)
        );
        assert_eq!(result.candidate_descriptor_count, 2);
        assert_eq!(result.matched_instruction_index, Some(1));
        assert!(result.delegates_ordering_ambiguity_to_phase_41c3);
        assert!(!result.accepts_verification_evidence);
        assert!(!result.authorizes_execution);
    }

    #[test]
    fn non_runtime_data_loaded_entry_fails_closed_before_descriptor_construction() {
        let mut entry = loaded_prior_instruction(0, ed25519_instruction());
        entry.loaded_instruction_is_runtime_data_only = false;

        let loading_result = loading_result(2, vec![entry]);

        let result =
            prefilter_loaded_prior_instructions_into_phase_41c3_descriptors(&loading_result);

        assert_eq!(
            result.status,
            Phase41D3_2_3PrefilterDescriptorStatus::LoadedPriorInstructionsUnavailable
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::UnreadableInstructionsSysvar)
        );
        assert_eq!(result.candidate_descriptor_count, 0);
        assert_eq!(result.phase_41c3_result, None);
        assert!(!result.locates_prior_ed25519_instruction);
        assert!(!result.accepts_verification_evidence);
        assert!(!result.authorizes_execution);
    }

    #[test]
    fn unavailable_loading_result_fails_closed_before_prefiltering() {
        let mut loading_result = empty_loading_result(0);
        loading_result.status =
            Phase41D3_2_2CheckedPriorInstructionLoadingStatus::CheckedPriorInstructionLoadingFailed;
        loading_result.rejection_case = Some(Phase41BRejectionCase::UnreadableInstructionsSysvar);
        loading_result.current_instruction_index = None;

        let result =
            prefilter_loaded_prior_instructions_into_phase_41c3_descriptors(&loading_result);

        assert_eq!(
            result.status,
            Phase41D3_2_3PrefilterDescriptorStatus::LoadedPriorInstructionsUnavailable
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::UnreadableInstructionsSysvar)
        );
        assert_eq!(result.current_instruction_index, None);
        assert_eq!(result.candidate_descriptor_count, 0);
        assert_eq!(result.phase_41c3_result, None);
        assert!(!result.locates_prior_ed25519_instruction);
    }

    #[test]
    fn phase_41d3_2_3_flips_only_structural_location_boundary_flag() {
        let flags = PHASE_41D3_2_3_SAFETY_FLAGS;

        assert!(!flags.raw_instructions_sysvar_parser_implemented);
        assert!(flags.account_info_parser_implemented);
        assert!(flags.load_instruction_called);
        assert!(flags.load_instruction_enabled);
        assert!(flags.concrete_runtime_api_selected);
        assert!(flags.current_instruction_identity_derived_from_runtime);
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
    fn phase_41d3_2_3_report_preserves_non_authorizing_descriptor_boundary() {
        let report = phase_41d3_2_3_prefilter_descriptor_runtime_boundary_report();

        assert_eq!(report.phase, "41D3.2.3");
        assert_eq!(report.version, "0.1.0");
        assert!(report.consumes_phase_41d3_2_2_loaded_prior_instructions);
        assert!(report.processes_runtime_data_only_entries);
        assert!(report.iterates_loaded_entries_by_reference);
        assert!(report.prefilter_enabled);
        assert!(report.prefilter_by_ed25519_program_id_only);
        assert!(report.discards_non_candidates_immediately);
        assert!(report.stores_candidate_metadata_only);
        assert!(report.delegates_ordering_ambiguity_to_phase_41c3);
        assert!(report.explicit_same_index_reject_boundary);
        assert!(report.explicit_later_index_reject_boundary);
        assert!(report.phase_41c3_descriptor_construction_enabled);
        assert!(report.locates_prior_ed25519_instruction);
        assert!(!report.ed25519_signature_verification_performed);
        assert!(!report.cryptographic_signature_proof_accepted);
        assert!(!report.accepts_verification_evidence);
        assert!(!report.authorizes_execution);
        assert!(!report.mutates_runtime_state);
        assert!(!report.safety_flags.verification_evidence_accepted);
        assert!(!report.safety_flags.authorization_enabled);
        assert!(!report.safety_flags.spl_token_mint_to_enabled);
        assert!(!report.safety_flags.live_route_enabled);
    }
}
