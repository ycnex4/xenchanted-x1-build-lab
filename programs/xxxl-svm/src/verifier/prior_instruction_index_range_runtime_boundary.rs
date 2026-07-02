use super::current_instruction_index_runtime_boundary::{
    Phase41D3_1CurrentInstructionIndexRuntimeResult,
    Phase41D3_1CurrentInstructionIndexRuntimeStatus,
};
use super::instructions_sysvar_access_contract_model::{
    Phase41BRejectionCase, Phase41BSafetyFlags,
};

pub const PHASE_41D3_2_1_PRIOR_INDEX_RANGE_RUNTIME_BOUNDARY_VERSION: &str = "0.1.0";
pub const PHASE_41D3_2_1_PRIOR_INDEX_RANGE_RUNTIME_BOUNDARY_PHASE: &str = "41D3.2.1";

pub const MAX_CHECKED_CURRENT_INSTRUCTION_INDEX: usize = u16::MAX as usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase41D3_2_1PriorIndexRangeRuntimeStatus {
    CurrentInstructionIndexUnavailable,
    EmptyPriorIndexRange,
    PriorIndexRangeConstructed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Phase41D3_2_1PriorIndexRangeRuntimeResult {
    pub status: Phase41D3_2_1PriorIndexRangeRuntimeStatus,
    pub rejection_case: Option<Phase41BRejectionCase>,
    pub current_instruction_index: Option<usize>,
    pub prior_instruction_indexes: Vec<usize>,
    pub prior_index_count: usize,
    pub current_index_used_for_ordering_only: bool,
    pub strict_prior_ordering_enforced: bool,
    pub current_index_zero_maps_to_empty_range: bool,
    pub same_index_excluded_by_range_construction: bool,
    pub later_indexes_excluded_by_range_construction: bool,
    pub prior_index_range_construction_enabled: bool,
    pub prior_instruction_loading_enabled: bool,
    pub raw_instructions_sysvar_parser_implemented: bool,
    pub load_instruction_called: bool,
    pub load_instruction_enabled: bool,
    pub locates_prior_ed25519_instruction: bool,
    pub accepts_verification_evidence: bool,
    pub authorizes_execution: bool,
    pub mutates_runtime_state: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phase41D3_2_1PriorIndexRangeRuntimeBoundaryReport {
    pub phase: &'static str,
    pub version: &'static str,
    pub accepts_checked_current_index_from_phase_41d3_1: bool,
    pub prior_index_range_construction_enabled: bool,
    pub strict_prior_ordering_enforced: bool,
    pub current_index_zero_maps_to_empty_range: bool,
    pub same_index_excluded_by_range_construction: bool,
    pub later_indexes_excluded_by_range_construction: bool,
    pub prior_instruction_loading_enabled: bool,
    pub raw_instructions_sysvar_parser_implemented: bool,
    pub load_instruction_called: bool,
    pub load_instruction_enabled: bool,
    pub locates_prior_ed25519_instruction: bool,
    pub accepts_verification_evidence: bool,
    pub authorizes_execution: bool,
    pub mutates_runtime_state: bool,
    pub safety_flags: Phase41BSafetyFlags,
}

pub const PHASE_41D3_2_1_SAFETY_FLAGS: Phase41BSafetyFlags = Phase41BSafetyFlags {
    raw_instructions_sysvar_parser_implemented: false,
    account_info_parser_implemented: true,
    load_instruction_called: false,
    load_instruction_enabled: false,
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

pub const PHASE_41D3_2_1_PRIOR_INDEX_RANGE_RUNTIME_BOUNDARY_REPORT:
    Phase41D3_2_1PriorIndexRangeRuntimeBoundaryReport =
    Phase41D3_2_1PriorIndexRangeRuntimeBoundaryReport {
        phase: PHASE_41D3_2_1_PRIOR_INDEX_RANGE_RUNTIME_BOUNDARY_PHASE,
        version: PHASE_41D3_2_1_PRIOR_INDEX_RANGE_RUNTIME_BOUNDARY_VERSION,
        accepts_checked_current_index_from_phase_41d3_1: true,
        prior_index_range_construction_enabled: true,
        strict_prior_ordering_enforced: true,
        current_index_zero_maps_to_empty_range: true,
        same_index_excluded_by_range_construction: true,
        later_indexes_excluded_by_range_construction: true,
        prior_instruction_loading_enabled: false,
        raw_instructions_sysvar_parser_implemented: false,
        load_instruction_called: false,
        load_instruction_enabled: false,
        locates_prior_ed25519_instruction: false,
        accepts_verification_evidence: false,
        authorizes_execution: false,
        mutates_runtime_state: false,
        safety_flags: PHASE_41D3_2_1_SAFETY_FLAGS,
    };

pub fn construct_prior_instruction_index_range_from_checked_current_index(
    checked_current_index_result: &Phase41D3_1CurrentInstructionIndexRuntimeResult,
) -> Phase41D3_2_1PriorIndexRangeRuntimeResult {
    if checked_current_index_result.status
        != Phase41D3_1CurrentInstructionIndexRuntimeStatus::CurrentInstructionIndexAcquired
    {
        return result(
            Phase41D3_2_1PriorIndexRangeRuntimeStatus::CurrentInstructionIndexUnavailable,
            checked_current_index_result
                .rejection_case
                .or(Some(Phase41BRejectionCase::UnreadableInstructionsSysvar)),
            None,
            Vec::new(),
        );
    }

    let Some(current_instruction_index) = checked_current_index_result.current_instruction_index
    else {
        return result(
            Phase41D3_2_1PriorIndexRangeRuntimeStatus::CurrentInstructionIndexUnavailable,
            Some(Phase41BRejectionCase::UnreadableInstructionsSysvar),
            None,
            Vec::new(),
        );
    };

    if current_instruction_index > MAX_CHECKED_CURRENT_INSTRUCTION_INDEX {
        return result(
            Phase41D3_2_1PriorIndexRangeRuntimeStatus::CurrentInstructionIndexUnavailable,
            Some(Phase41BRejectionCase::UnreadableInstructionsSysvar),
            None,
            Vec::new(),
        );
    }

    let prior_instruction_indexes = construct_strict_prior_index_range(current_instruction_index);
    let status = if prior_instruction_indexes.is_empty() {
        Phase41D3_2_1PriorIndexRangeRuntimeStatus::EmptyPriorIndexRange
    } else {
        Phase41D3_2_1PriorIndexRangeRuntimeStatus::PriorIndexRangeConstructed
    };

    result(
        status,
        None,
        Some(current_instruction_index),
        prior_instruction_indexes,
    )
}

pub fn phase_41d3_2_1_prior_index_range_runtime_boundary_report(
) -> Phase41D3_2_1PriorIndexRangeRuntimeBoundaryReport {
    PHASE_41D3_2_1_PRIOR_INDEX_RANGE_RUNTIME_BOUNDARY_REPORT
}

fn construct_strict_prior_index_range(current_instruction_index: usize) -> Vec<usize> {
    (0..current_instruction_index).collect()
}

fn result(
    status: Phase41D3_2_1PriorIndexRangeRuntimeStatus,
    rejection_case: Option<Phase41BRejectionCase>,
    current_instruction_index: Option<usize>,
    prior_instruction_indexes: Vec<usize>,
) -> Phase41D3_2_1PriorIndexRangeRuntimeResult {
    let prior_index_count = prior_instruction_indexes.len();

    Phase41D3_2_1PriorIndexRangeRuntimeResult {
        status,
        rejection_case,
        current_instruction_index,
        prior_instruction_indexes,
        prior_index_count,
        current_index_used_for_ordering_only: true,
        strict_prior_ordering_enforced: true,
        current_index_zero_maps_to_empty_range: true,
        same_index_excluded_by_range_construction: true,
        later_indexes_excluded_by_range_construction: true,
        prior_index_range_construction_enabled: true,
        prior_instruction_loading_enabled: false,
        raw_instructions_sysvar_parser_implemented: false,
        load_instruction_called: false,
        load_instruction_enabled: false,
        locates_prior_ed25519_instruction: false,
        accepts_verification_evidence: false,
        authorizes_execution: false,
        mutates_runtime_state: false,
    }
}

#[cfg(test)]
mod tests {
    use super::super::current_instruction_index_runtime_boundary::{
        map_checked_current_instruction_index_result,
        Phase41D3_1CurrentInstructionIndexRuntimeStatus,
    };
    use super::*;

    #[test]
    fn current_index_zero_maps_to_empty_prior_range() {
        let current_index_result = map_checked_current_instruction_index_result(Ok(0));

        let result = construct_prior_instruction_index_range_from_checked_current_index(
            &current_index_result,
        );

        assert_eq!(
            result.status,
            Phase41D3_2_1PriorIndexRangeRuntimeStatus::EmptyPriorIndexRange
        );
        assert_eq!(result.rejection_case, None);
        assert_eq!(result.current_instruction_index, Some(0));
        assert_eq!(result.prior_index_count, 0);
        assert!(result.prior_instruction_indexes.is_empty());
        assert!(result.current_index_used_for_ordering_only);
        assert!(result.strict_prior_ordering_enforced);
        assert!(result.current_index_zero_maps_to_empty_range);
        assert!(result.same_index_excluded_by_range_construction);
        assert!(result.later_indexes_excluded_by_range_construction);
        assert!(!result.prior_instruction_loading_enabled);
        assert!(!result.raw_instructions_sysvar_parser_implemented);
        assert!(!result.load_instruction_called);
        assert!(!result.locates_prior_ed25519_instruction);
        assert!(!result.accepts_verification_evidence);
        assert!(!result.authorizes_execution);
        assert!(!result.mutates_runtime_state);
    }

    #[test]
    fn current_index_one_constructs_single_prior_index_zero() {
        let current_index_result = map_checked_current_instruction_index_result(Ok(1));

        let result = construct_prior_instruction_index_range_from_checked_current_index(
            &current_index_result,
        );

        assert_eq!(
            result.status,
            Phase41D3_2_1PriorIndexRangeRuntimeStatus::PriorIndexRangeConstructed
        );
        assert_eq!(result.current_instruction_index, Some(1));
        assert_eq!(result.prior_index_count, 1);
        assert_eq!(
            result.prior_instruction_indexes,
            (0..1).collect::<Vec<usize>>()
        );
        assert!(result
            .prior_instruction_indexes
            .iter()
            .all(|index| *index < 1));
    }

    #[test]
    fn current_index_five_constructs_zero_to_four_only() {
        let current_index_result = map_checked_current_instruction_index_result(Ok(5));

        let result = construct_prior_instruction_index_range_from_checked_current_index(
            &current_index_result,
        );

        assert_eq!(
            result.status,
            Phase41D3_2_1PriorIndexRangeRuntimeStatus::PriorIndexRangeConstructed
        );
        assert_eq!(result.current_instruction_index, Some(5));
        assert_eq!(result.prior_index_count, 5);
        assert_eq!(
            result.prior_instruction_indexes,
            (0..5).collect::<Vec<usize>>()
        );
        assert!(result
            .prior_instruction_indexes
            .iter()
            .all(|index| *index < 5));
        assert!(!result
            .prior_instruction_indexes
            .iter()
            .any(|index| *index == 5));
        assert!(!result
            .prior_instruction_indexes
            .iter()
            .any(|index| *index > 5));
    }

    #[test]
    fn unavailable_current_index_fails_closed_without_prior_range() {
        let mut current_index_result = map_checked_current_instruction_index_result(Ok(0));
        current_index_result.status =
            Phase41D3_1CurrentInstructionIndexRuntimeStatus::CurrentInstructionIndexUnavailable;
        current_index_result.current_instruction_index = None;
        current_index_result.rejection_case =
            Some(Phase41BRejectionCase::UnreadableInstructionsSysvar);

        let result = construct_prior_instruction_index_range_from_checked_current_index(
            &current_index_result,
        );

        assert_eq!(
            result.status,
            Phase41D3_2_1PriorIndexRangeRuntimeStatus::CurrentInstructionIndexUnavailable
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::UnreadableInstructionsSysvar)
        );
        assert_eq!(result.current_instruction_index, None);
        assert_eq!(result.prior_index_count, 0);
        assert!(result.prior_instruction_indexes.is_empty());
        assert!(!result.load_instruction_called);
        assert!(!result.locates_prior_ed25519_instruction);
        assert!(!result.accepts_verification_evidence);
        assert!(!result.authorizes_execution);
    }

    #[test]
    fn acquired_status_without_current_index_fails_closed() {
        let mut current_index_result = map_checked_current_instruction_index_result(Ok(0));
        current_index_result.current_instruction_index = None;

        let result = construct_prior_instruction_index_range_from_checked_current_index(
            &current_index_result,
        );

        assert_eq!(
            result.status,
            Phase41D3_2_1PriorIndexRangeRuntimeStatus::CurrentInstructionIndexUnavailable
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::UnreadableInstructionsSysvar)
        );
        assert_eq!(result.current_instruction_index, None);
        assert!(result.prior_instruction_indexes.is_empty());
    }

    #[test]
    fn forged_oversized_current_index_fails_closed_without_allocation() {
        let mut current_index_result = map_checked_current_instruction_index_result(Ok(0));
        current_index_result.current_instruction_index =
            Some(MAX_CHECKED_CURRENT_INSTRUCTION_INDEX + 1);

        let result = construct_prior_instruction_index_range_from_checked_current_index(
            &current_index_result,
        );

        assert_eq!(
            result.status,
            Phase41D3_2_1PriorIndexRangeRuntimeStatus::CurrentInstructionIndexUnavailable
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::UnreadableInstructionsSysvar)
        );
        assert_eq!(result.current_instruction_index, None);
        assert!(result.prior_instruction_indexes.is_empty());
        assert_eq!(result.prior_index_count, 0);
    }

    #[test]
    fn phase_41d3_2_1_flips_no_loading_locating_or_trust_flags() {
        let flags = PHASE_41D3_2_1_SAFETY_FLAGS;

        assert!(!flags.raw_instructions_sysvar_parser_implemented);
        assert!(flags.account_info_parser_implemented);
        assert!(!flags.load_instruction_called);
        assert!(!flags.load_instruction_enabled);
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
    fn phase_41d3_2_1_report_preserves_range_only_boundary() {
        let report = phase_41d3_2_1_prior_index_range_runtime_boundary_report();

        assert_eq!(report.phase, "41D3.2.1");
        assert_eq!(report.version, "0.1.0");
        assert!(report.accepts_checked_current_index_from_phase_41d3_1);
        assert!(report.prior_index_range_construction_enabled);
        assert!(report.strict_prior_ordering_enforced);
        assert!(report.current_index_zero_maps_to_empty_range);
        assert!(report.same_index_excluded_by_range_construction);
        assert!(report.later_indexes_excluded_by_range_construction);
        assert!(!report.prior_instruction_loading_enabled);
        assert!(!report.raw_instructions_sysvar_parser_implemented);
        assert!(!report.load_instruction_called);
        assert!(!report.load_instruction_enabled);
        assert!(!report.locates_prior_ed25519_instruction);
        assert!(!report.accepts_verification_evidence);
        assert!(!report.authorizes_execution);
        assert!(!report.mutates_runtime_state);
        assert!(!report.safety_flags.verification_evidence_accepted);
        assert!(!report.safety_flags.authorization_enabled);
        assert!(!report.safety_flags.spl_token_mint_to_enabled);
        assert!(!report.safety_flags.live_route_enabled);
    }
}
