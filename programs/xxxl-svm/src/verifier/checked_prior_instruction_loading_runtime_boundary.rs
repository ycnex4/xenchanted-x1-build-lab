use solana_program::{
    account_info::AccountInfo, instruction::Instruction, program_error::ProgramError,
    sysvar::instructions,
};

use super::instructions_sysvar_access_contract_model::{
    Phase41BRejectionCase, Phase41BSafetyFlags,
};
use super::prior_instruction_index_range_runtime_boundary::{
    Phase41D3_2_1PriorIndexRangeRuntimeResult, Phase41D3_2_1PriorIndexRangeRuntimeStatus,
};

pub const PHASE_41D3_2_2_CHECKED_PRIOR_INSTRUCTION_LOADING_RUNTIME_BOUNDARY_VERSION: &str = "0.1.0";
pub const PHASE_41D3_2_2_CHECKED_PRIOR_INSTRUCTION_LOADING_RUNTIME_BOUNDARY_PHASE: &str =
    "41D3.2.2";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase41D3_2_2CheckedPriorInstructionLoadingStatus {
    PriorIndexRangeUnavailable,
    MissingInstructionsSysvar,
    EmptyPriorRangeNoLoadingAttempted,
    CheckedPriorInstructionsLoaded,
    CheckedPriorInstructionLoadingFailed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase41D3_2_2CheckedPriorInstructionLoadEntryStatus {
    Loaded,
    LoadingFailed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Phase41D3_2_2LoadedPriorInstruction {
    pub instruction_index: usize,
    pub instruction: Instruction,
    pub loaded_instruction_is_runtime_data_only: bool,
    pub is_evidence: bool,
    pub authorizes_execution: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Phase41D3_2_2CheckedPriorInstructionLoadEntry {
    pub status: Phase41D3_2_2CheckedPriorInstructionLoadEntryStatus,
    pub instruction_index: usize,
    pub loaded_instruction: Option<Instruction>,
    pub loading_failure: Option<Phase41BRejectionCase>,
    pub loaded_instruction_is_runtime_data_only: bool,
    pub is_evidence: bool,
    pub authorizes_execution: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Phase41D3_2_2CheckedPriorInstructionLoadingResult {
    pub status: Phase41D3_2_2CheckedPriorInstructionLoadingStatus,
    pub rejection_case: Option<Phase41BRejectionCase>,
    pub current_instruction_index: Option<usize>,
    pub prior_index_count: usize,
    pub attempted_loading_count: usize,
    pub loaded_instruction_count: usize,
    pub failed_instruction_index: Option<usize>,
    pub loaded_prior_instructions: Vec<Phase41D3_2_2LoadedPriorInstruction>,
    pub consumes_phase_41d3_2_1_prior_range: bool,
    pub uses_instructions_sysvar_account_info: bool,
    pub checks_instructions_sysvar_program_id: bool,
    pub iterates_prior_indexes_lazily: bool,
    pub empty_prior_range_causes_no_loading_attempt: bool,
    pub checked_prior_instruction_loading_enabled: bool,
    pub prior_instruction_loading_enabled: bool,
    pub raw_instructions_sysvar_parser_implemented: bool,
    pub load_instruction_called: bool,
    pub load_instruction_enabled: bool,
    pub load_instruction_at_checked_used: bool,
    pub unchecked_instruction_loading_used: bool,
    pub prefilter_enabled: bool,
    pub phase_41c3_descriptor_construction_enabled: bool,
    pub locates_prior_ed25519_instruction: bool,
    pub accepts_verification_evidence: bool,
    pub authorizes_execution: bool,
    pub mutates_runtime_state: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phase41D3_2_2CheckedPriorInstructionLoadingBoundaryReport {
    pub phase: &'static str,
    pub version: &'static str,
    pub consumes_phase_41d3_2_1_prior_range: bool,
    pub iterates_prior_indexes_lazily: bool,
    pub empty_prior_range_causes_no_loading_attempt: bool,
    pub prior_instruction_loading_enabled: bool,
    pub load_instruction_called: bool,
    pub load_instruction_enabled: bool,
    pub load_instruction_at_checked_used: bool,
    pub unchecked_instruction_loading_used: bool,
    pub raw_instructions_sysvar_parser_implemented: bool,
    pub prefilter_enabled: bool,
    pub phase_41c3_descriptor_construction_enabled: bool,
    pub locates_prior_ed25519_instruction: bool,
    pub accepts_verification_evidence: bool,
    pub authorizes_execution: bool,
    pub mutates_runtime_state: bool,
    pub safety_flags: Phase41BSafetyFlags,
}

pub const PHASE_41D3_2_2_SAFETY_FLAGS: Phase41BSafetyFlags = Phase41BSafetyFlags {
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

pub const PHASE_41D3_2_2_CHECKED_PRIOR_INSTRUCTION_LOADING_RUNTIME_BOUNDARY_REPORT:
    Phase41D3_2_2CheckedPriorInstructionLoadingBoundaryReport =
    Phase41D3_2_2CheckedPriorInstructionLoadingBoundaryReport {
        phase: PHASE_41D3_2_2_CHECKED_PRIOR_INSTRUCTION_LOADING_RUNTIME_BOUNDARY_PHASE,
        version: PHASE_41D3_2_2_CHECKED_PRIOR_INSTRUCTION_LOADING_RUNTIME_BOUNDARY_VERSION,
        consumes_phase_41d3_2_1_prior_range: true,
        iterates_prior_indexes_lazily: true,
        empty_prior_range_causes_no_loading_attempt: true,
        prior_instruction_loading_enabled: true,
        load_instruction_called: true,
        load_instruction_enabled: true,
        load_instruction_at_checked_used: true,
        unchecked_instruction_loading_used: false,
        raw_instructions_sysvar_parser_implemented: false,
        prefilter_enabled: false,
        phase_41c3_descriptor_construction_enabled: false,
        locates_prior_ed25519_instruction: false,
        accepts_verification_evidence: false,
        authorizes_execution: false,
        mutates_runtime_state: false,
        safety_flags: PHASE_41D3_2_2_SAFETY_FLAGS,
    };

pub fn load_checked_prior_instructions_from_bounded_range(
    prior_range_result: &Phase41D3_2_1PriorIndexRangeRuntimeResult,
    instructions_sysvar_account: Option<&AccountInfo<'_>>,
) -> Phase41D3_2_2CheckedPriorInstructionLoadingResult {
    if prior_range_result.status
        == Phase41D3_2_1PriorIndexRangeRuntimeStatus::CurrentInstructionIndexUnavailable
    {
        return result(
            Phase41D3_2_2CheckedPriorInstructionLoadingStatus::PriorIndexRangeUnavailable,
            prior_range_result
                .rejection_case
                .or(Some(Phase41BRejectionCase::UnreadableInstructionsSysvar)),
            prior_range_result.current_instruction_index,
            prior_range_result.prior_index_count,
            0,
            None,
            Vec::new(),
            false,
            false,
            false,
        );
    }

    let Some(current_instruction_index) = prior_range_result.current_instruction_index else {
        return result(
            Phase41D3_2_2CheckedPriorInstructionLoadingStatus::PriorIndexRangeUnavailable,
            Some(Phase41BRejectionCase::UnreadableInstructionsSysvar),
            None,
            prior_range_result.prior_index_count,
            0,
            None,
            Vec::new(),
            false,
            false,
            false,
        );
    };

    if prior_range_result
        .prior_instruction_indexes
        .iter()
        .any(|index| *index >= current_instruction_index)
    {
        return result(
            Phase41D3_2_2CheckedPriorInstructionLoadingStatus::PriorIndexRangeUnavailable,
            Some(Phase41BRejectionCase::Ed25519InstructionAfterCurrentInstruction),
            Some(current_instruction_index),
            prior_range_result.prior_index_count,
            0,
            None,
            Vec::new(),
            false,
            false,
            false,
        );
    }

    if prior_range_result.prior_instruction_indexes.is_empty() {
        return result(
            Phase41D3_2_2CheckedPriorInstructionLoadingStatus::EmptyPriorRangeNoLoadingAttempted,
            None,
            Some(current_instruction_index),
            0,
            0,
            None,
            Vec::new(),
            false,
            false,
            false,
        );
    }

    let Some(account_info) = instructions_sysvar_account else {
        return result(
            Phase41D3_2_2CheckedPriorInstructionLoadingStatus::MissingInstructionsSysvar,
            Some(Phase41BRejectionCase::MissingInstructionsSysvar),
            Some(current_instruction_index),
            prior_range_result.prior_index_count,
            0,
            None,
            Vec::new(),
            true,
            false,
            false,
        );
    };

    if account_info.key != &instructions::id() {
        return result(
            Phase41D3_2_2CheckedPriorInstructionLoadingStatus::MissingInstructionsSysvar,
            Some(Phase41BRejectionCase::MissingInstructionsSysvar),
            Some(current_instruction_index),
            prior_range_result.prior_index_count,
            0,
            None,
            Vec::new(),
            true,
            true,
            false,
        );
    }

    let mut loaded_prior_instructions = Vec::new();
    let mut attempted_loading_count = 0;

    for instruction_index in prior_range_result.prior_instruction_indexes.iter().copied() {
        attempted_loading_count += 1;

        let load_entry = map_checked_prior_instruction_load_result(
            instruction_index,
            instructions::load_instruction_at_checked(instruction_index, account_info),
        );

        match load_entry.status {
            Phase41D3_2_2CheckedPriorInstructionLoadEntryStatus::Loaded => {
                let Some(instruction) = load_entry.loaded_instruction else {
                    return result(
                        Phase41D3_2_2CheckedPriorInstructionLoadingStatus::CheckedPriorInstructionLoadingFailed,
                        Some(Phase41BRejectionCase::UnreadableInstructionsSysvar),
                        Some(current_instruction_index),
                        prior_range_result.prior_index_count,
                        attempted_loading_count,
                        Some(instruction_index),
                        Vec::new(),
                        true,
                        true,
                        true,
                    );
                };

                loaded_prior_instructions.push(Phase41D3_2_2LoadedPriorInstruction {
                    instruction_index,
                    instruction,
                    loaded_instruction_is_runtime_data_only: true,
                    is_evidence: false,
                    authorizes_execution: false,
                });
            }
            Phase41D3_2_2CheckedPriorInstructionLoadEntryStatus::LoadingFailed => {
                return result(
                    Phase41D3_2_2CheckedPriorInstructionLoadingStatus::CheckedPriorInstructionLoadingFailed,
                    load_entry
                        .loading_failure
                        .or(Some(Phase41BRejectionCase::UnreadableInstructionsSysvar)),
                    Some(current_instruction_index),
                    prior_range_result.prior_index_count,
                    attempted_loading_count,
                    Some(instruction_index),
                    Vec::new(),
                    true,
                    true,
                    true,
                );
            }
        }
    }

    result(
        Phase41D3_2_2CheckedPriorInstructionLoadingStatus::CheckedPriorInstructionsLoaded,
        None,
        Some(current_instruction_index),
        prior_range_result.prior_index_count,
        attempted_loading_count,
        None,
        loaded_prior_instructions,
        true,
        true,
        true,
    )
}

pub fn map_checked_prior_instruction_load_result(
    instruction_index: usize,
    checked_load_result: Result<Instruction, ProgramError>,
) -> Phase41D3_2_2CheckedPriorInstructionLoadEntry {
    match checked_load_result {
        Ok(instruction) => Phase41D3_2_2CheckedPriorInstructionLoadEntry {
            status: Phase41D3_2_2CheckedPriorInstructionLoadEntryStatus::Loaded,
            instruction_index,
            loaded_instruction: Some(instruction),
            loading_failure: None,
            loaded_instruction_is_runtime_data_only: true,
            is_evidence: false,
            authorizes_execution: false,
        },
        Err(_) => Phase41D3_2_2CheckedPriorInstructionLoadEntry {
            status: Phase41D3_2_2CheckedPriorInstructionLoadEntryStatus::LoadingFailed,
            instruction_index,
            loaded_instruction: None,
            loading_failure: Some(Phase41BRejectionCase::UnreadableInstructionsSysvar),
            loaded_instruction_is_runtime_data_only: true,
            is_evidence: false,
            authorizes_execution: false,
        },
    }
}

pub fn phase_41d3_2_2_checked_prior_instruction_loading_runtime_boundary_report(
) -> Phase41D3_2_2CheckedPriorInstructionLoadingBoundaryReport {
    PHASE_41D3_2_2_CHECKED_PRIOR_INSTRUCTION_LOADING_RUNTIME_BOUNDARY_REPORT
}

fn result(
    status: Phase41D3_2_2CheckedPriorInstructionLoadingStatus,
    rejection_case: Option<Phase41BRejectionCase>,
    current_instruction_index: Option<usize>,
    prior_index_count: usize,
    attempted_loading_count: usize,
    failed_instruction_index: Option<usize>,
    loaded_prior_instructions: Vec<Phase41D3_2_2LoadedPriorInstruction>,
    uses_instructions_sysvar_account_info: bool,
    checks_instructions_sysvar_program_id: bool,
    load_instruction_called: bool,
) -> Phase41D3_2_2CheckedPriorInstructionLoadingResult {
    let loaded_instruction_count = loaded_prior_instructions.len();

    Phase41D3_2_2CheckedPriorInstructionLoadingResult {
        status,
        rejection_case,
        current_instruction_index,
        prior_index_count,
        attempted_loading_count,
        loaded_instruction_count,
        failed_instruction_index,
        loaded_prior_instructions,
        consumes_phase_41d3_2_1_prior_range: true,
        uses_instructions_sysvar_account_info,
        checks_instructions_sysvar_program_id,
        iterates_prior_indexes_lazily: true,
        empty_prior_range_causes_no_loading_attempt: true,
        checked_prior_instruction_loading_enabled: true,
        prior_instruction_loading_enabled: true,
        raw_instructions_sysvar_parser_implemented: false,
        load_instruction_called,
        load_instruction_enabled: true,
        load_instruction_at_checked_used: load_instruction_called,
        unchecked_instruction_loading_used: false,
        prefilter_enabled: false,
        phase_41c3_descriptor_construction_enabled: false,
        locates_prior_ed25519_instruction: false,
        accepts_verification_evidence: false,
        authorizes_execution: false,
        mutates_runtime_state: false,
    }
}

#[cfg(test)]
mod tests {
    use solana_program::{account_info::AccountInfo, instruction::Instruction, pubkey::Pubkey};

    use super::super::current_instruction_index_runtime_boundary::map_checked_current_instruction_index_result;
    use super::super::prior_instruction_index_range_runtime_boundary::{
        construct_prior_instruction_index_range_from_checked_current_index,
        Phase41D3_2_1PriorIndexRangeRuntimeStatus,
    };
    use super::*;

    fn account_info_for_key<'a>(
        key: &'a Pubkey,
        owner: &'a Pubkey,
        lamports: &'a mut u64,
        data: &'a mut [u8],
    ) -> AccountInfo<'a> {
        AccountInfo::new(key, false, false, lamports, data, owner, false, 0)
    }

    fn prior_range_for_current_index(
        current_index: u16,
    ) -> Phase41D3_2_1PriorIndexRangeRuntimeResult {
        let current_index_result = map_checked_current_instruction_index_result(Ok(current_index));

        construct_prior_instruction_index_range_from_checked_current_index(&current_index_result)
    }

    #[test]
    fn empty_prior_range_causes_no_loading_attempt_and_does_not_require_sysvar_account() {
        let prior_range = prior_range_for_current_index(0);

        let result = load_checked_prior_instructions_from_bounded_range(&prior_range, None);

        assert_eq!(
            result.status,
            Phase41D3_2_2CheckedPriorInstructionLoadingStatus::EmptyPriorRangeNoLoadingAttempted
        );
        assert_eq!(result.rejection_case, None);
        assert_eq!(result.current_instruction_index, Some(0));
        assert_eq!(result.prior_index_count, 0);
        assert_eq!(result.attempted_loading_count, 0);
        assert_eq!(result.loaded_instruction_count, 0);
        assert!(result.loaded_prior_instructions.is_empty());
        assert!(!result.uses_instructions_sysvar_account_info);
        assert!(!result.checks_instructions_sysvar_program_id);
        assert!(!result.load_instruction_called);
        assert!(!result.load_instruction_at_checked_used);
        assert!(result.load_instruction_enabled);
        assert!(result.prior_instruction_loading_enabled);
        assert!(!result.raw_instructions_sysvar_parser_implemented);
        assert!(!result.prefilter_enabled);
        assert!(!result.phase_41c3_descriptor_construction_enabled);
        assert!(!result.locates_prior_ed25519_instruction);
        assert!(!result.accepts_verification_evidence);
        assert!(!result.authorizes_execution);
        assert!(!result.mutates_runtime_state);
    }

    #[test]
    fn missing_sysvar_account_fails_closed_for_nonempty_prior_range() {
        let prior_range = prior_range_for_current_index(1);

        let result = load_checked_prior_instructions_from_bounded_range(&prior_range, None);

        assert_eq!(
            result.status,
            Phase41D3_2_2CheckedPriorInstructionLoadingStatus::MissingInstructionsSysvar
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::MissingInstructionsSysvar)
        );
        assert_eq!(result.current_instruction_index, Some(1));
        assert_eq!(result.prior_index_count, 1);
        assert_eq!(result.attempted_loading_count, 0);
        assert!(result.loaded_prior_instructions.is_empty());
        assert!(result.uses_instructions_sysvar_account_info);
        assert!(!result.checks_instructions_sysvar_program_id);
        assert!(!result.load_instruction_called);
        assert!(!result.load_instruction_at_checked_used);
        assert!(!result.accepts_verification_evidence);
        assert!(!result.authorizes_execution);
    }

    #[test]
    fn wrong_sysvar_account_key_fails_closed_before_loading() {
        let prior_range = prior_range_for_current_index(1);
        let key = Pubkey::new_from_array([7; 32]);
        let owner = Pubkey::new_from_array([9; 32]);
        let mut lamports = 0;
        let mut data = [0_u8; 8];
        let account = account_info_for_key(&key, &owner, &mut lamports, &mut data);

        let result =
            load_checked_prior_instructions_from_bounded_range(&prior_range, Some(&account));

        assert_eq!(
            result.status,
            Phase41D3_2_2CheckedPriorInstructionLoadingStatus::MissingInstructionsSysvar
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::MissingInstructionsSysvar)
        );
        assert_eq!(result.current_instruction_index, Some(1));
        assert_eq!(result.attempted_loading_count, 0);
        assert!(result.uses_instructions_sysvar_account_info);
        assert!(result.checks_instructions_sysvar_program_id);
        assert!(!result.load_instruction_called);
        assert!(!result.load_instruction_at_checked_used);
        assert!(!result.locates_prior_ed25519_instruction);
    }

    #[test]
    fn unavailable_prior_range_fails_closed_before_loading() {
        let mut prior_range = prior_range_for_current_index(0);
        prior_range.status =
            Phase41D3_2_1PriorIndexRangeRuntimeStatus::CurrentInstructionIndexUnavailable;
        prior_range.rejection_case = Some(Phase41BRejectionCase::UnreadableInstructionsSysvar);
        prior_range.current_instruction_index = None;

        let result = load_checked_prior_instructions_from_bounded_range(&prior_range, None);

        assert_eq!(
            result.status,
            Phase41D3_2_2CheckedPriorInstructionLoadingStatus::PriorIndexRangeUnavailable
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::UnreadableInstructionsSysvar)
        );
        assert_eq!(result.current_instruction_index, None);
        assert_eq!(result.attempted_loading_count, 0);
        assert!(!result.load_instruction_called);
        assert!(!result.load_instruction_at_checked_used);
        assert!(!result.accepts_verification_evidence);
        assert!(!result.authorizes_execution);
    }

    #[test]
    fn inconsistent_prior_range_with_same_index_fails_closed_before_loading() {
        let mut prior_range = prior_range_for_current_index(1);
        prior_range.prior_instruction_indexes.push(1);
        prior_range.prior_index_count = prior_range.prior_instruction_indexes.len();

        let result = load_checked_prior_instructions_from_bounded_range(&prior_range, None);

        assert_eq!(
            result.status,
            Phase41D3_2_2CheckedPriorInstructionLoadingStatus::PriorIndexRangeUnavailable
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::Ed25519InstructionAfterCurrentInstruction)
        );
        assert_eq!(result.current_instruction_index, Some(1));
        assert_eq!(result.attempted_loading_count, 0);
        assert!(!result.load_instruction_called);
        assert!(!result.load_instruction_at_checked_used);
        assert!(!result.locates_prior_ed25519_instruction);
    }

    #[test]
    fn checked_load_success_maps_to_runtime_data_only_entry() {
        let instruction = Instruction {
            program_id: Pubkey::new_from_array([3; 32]),
            accounts: Vec::new(),
            data: vec![1, 2, 3],
        };

        let entry = map_checked_prior_instruction_load_result(7, Ok(instruction));

        assert_eq!(
            entry.status,
            Phase41D3_2_2CheckedPriorInstructionLoadEntryStatus::Loaded
        );
        assert_eq!(entry.instruction_index, 7);
        assert_eq!(entry.loading_failure, None);
        assert!(entry.loaded_instruction.is_some());
        assert!(entry.loaded_instruction_is_runtime_data_only);
        assert!(!entry.is_evidence);
        assert!(!entry.authorizes_execution);

        match entry.loaded_instruction.as_ref() {
            Some(loaded_instruction) => {
                assert_eq!(
                    loaded_instruction.program_id,
                    Pubkey::new_from_array([3; 32])
                );
                assert_eq!(loaded_instruction.data, vec![1, 2, 3]);
            }
            None => {
                assert_eq!(
                    entry.status,
                    Phase41D3_2_2CheckedPriorInstructionLoadEntryStatus::Loaded
                );
                assert!(entry.loaded_instruction.is_some());
            }
        }
    }

    #[test]
    fn checked_load_failure_maps_to_deterministic_non_authorizing_entry() {
        let entry =
            map_checked_prior_instruction_load_result(2, Err(ProgramError::InvalidArgument));

        assert_eq!(
            entry.status,
            Phase41D3_2_2CheckedPriorInstructionLoadEntryStatus::LoadingFailed
        );
        assert_eq!(entry.instruction_index, 2);
        assert_eq!(entry.loaded_instruction, None);
        assert_eq!(
            entry.loading_failure,
            Some(Phase41BRejectionCase::UnreadableInstructionsSysvar)
        );
        assert!(entry.loaded_instruction_is_runtime_data_only);
        assert!(!entry.is_evidence);
        assert!(!entry.authorizes_execution);
    }

    #[test]
    fn phase_41d3_2_2_flips_only_loading_capability_flags() {
        let flags = PHASE_41D3_2_2_SAFETY_FLAGS;

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
    fn phase_41d3_2_2_report_preserves_loading_only_boundary() {
        let report = phase_41d3_2_2_checked_prior_instruction_loading_runtime_boundary_report();

        assert_eq!(report.phase, "41D3.2.2");
        assert_eq!(report.version, "0.1.0");
        assert!(report.consumes_phase_41d3_2_1_prior_range);
        assert!(report.iterates_prior_indexes_lazily);
        assert!(report.empty_prior_range_causes_no_loading_attempt);
        assert!(report.prior_instruction_loading_enabled);
        assert!(report.load_instruction_called);
        assert!(report.load_instruction_enabled);
        assert!(report.load_instruction_at_checked_used);
        assert!(!report.unchecked_instruction_loading_used);
        assert!(!report.raw_instructions_sysvar_parser_implemented);
        assert!(!report.prefilter_enabled);
        assert!(!report.phase_41c3_descriptor_construction_enabled);
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
