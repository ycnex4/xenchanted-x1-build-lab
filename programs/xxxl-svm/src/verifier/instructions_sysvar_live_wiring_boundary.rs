use solana_program::{account_info::AccountInfo, ed25519_program};

use super::checked_prior_instruction_loading_runtime_boundary::{
    load_checked_prior_instructions_from_bounded_range,
    Phase41D3_2_2CheckedPriorInstructionLoadingResult,
    Phase41D3_2_2CheckedPriorInstructionLoadingStatus,
};
use super::current_instruction_index_runtime_boundary::{
    acquire_current_instruction_index_from_checked_instructions_sysvar,
    Phase41D3_1CurrentInstructionIndexRuntimeStatus,
};
use super::instructions_sysvar_access_contract_model::Phase41BRejectionCase;
use super::prior_instruction_index_range_runtime_boundary::{
    construct_prior_instruction_index_range_from_checked_current_index,
    Phase41D3_2_1PriorIndexRangeRuntimeStatus,
};

pub const PHASE_41K_1_INSTRUCTIONS_SYSVAR_LIVE_WIRING_BOUNDARY_PHASE: &str = "41K.1";
pub const PHASE_41K_1_INSTRUCTIONS_SYSVAR_LIVE_WIRING_BOUNDARY_VERSION: &str = "0.1.0";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase41K1InstructionsSysvarLiveWiringStatus {
    MissingInstructionsSysvar,
    CurrentInstructionIndexUnavailable,
    PriorIndexRangeUnavailable,
    PriorInstructionLoadingFailed,
    NoPriorInstructions,
    NoPriorEd25519PrecompileInstructions,
    PriorEd25519PrecompileInstructionsLoaded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phase41K1LoadedPriorEd25519PrecompileInstruction {
    pub instruction_index: usize,
    pub instruction_data_len: usize,
    pub strictly_prior_to_current_instruction: bool,
    pub program_id_is_ed25519: bool,
    pub loaded_from_real_instructions_sysvar: bool,
    pub loaded_with_checked_runtime_api: bool,
    pub runtime_data_only: bool,
    pub model_a_applies_to_this_prior_precompile: bool,
    pub accepts_verification_evidence: bool,
    pub authorizes_execution: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Phase41K1InstructionsSysvarLiveWiringResult {
    pub status: Phase41K1InstructionsSysvarLiveWiringStatus,
    pub rejection_case: Option<Phase41BRejectionCase>,
    pub current_instruction_index: Option<usize>,
    pub loaded_prior_instruction_count: usize,
    pub inspected_prior_instruction_count: usize,
    pub discarded_non_ed25519_prior_instruction_count: usize,
    pub prior_ed25519_precompile_count: usize,
    pub prior_ed25519_precompile_instructions:
        Vec<Phase41K1LoadedPriorEd25519PrecompileInstruction>,
    pub consumes_phase_41d3_1_current_index_boundary: bool,
    pub consumes_phase_41d3_2_1_prior_range_boundary: bool,
    pub consumes_phase_41d3_2_2_checked_prior_loading_boundary: bool,
    pub uses_real_instructions_sysvar_account_info: bool,
    pub checks_instructions_sysvar_account_id: bool,
    pub load_current_index_checked_used: bool,
    pub load_instruction_at_checked_used: bool,
    pub current_index_caller_provided: bool,
    pub enumerates_n_prior_ed25519_precompile_instructions: bool,
    pub model_a_live_wiring_precondition_preserved: bool,
    pub all_loaded_ed25519_precompiles_strictly_prior: bool,
    pub all_loaded_ed25519_precompiles_program_id_checked: bool,
    pub accepts_caller_provided_instruction_bytes: bool,
    pub accepts_frontend_or_watcher_ed25519_proof: bool,
    pub guardian_set_runtime_loading_enabled: bool,
    pub processed_registry_runtime_loading_enabled: bool,
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
pub struct Phase41K1InstructionsSysvarLiveWiringBoundaryReport {
    pub phase: &'static str,
    pub version: &'static str,
    pub runtime_surface: &'static str,
    pub uses_real_instructions_sysvar_account_info: bool,
    pub checks_instructions_sysvar_account_id: bool,
    pub load_current_index_checked_used: bool,
    pub load_instruction_at_checked_used: bool,
    pub supports_n_prior_ed25519_precompile_instructions: bool,
    pub accepts_single_prior_instruction_only: bool,
    pub current_index_caller_provided: bool,
    pub accepts_caller_provided_instruction_bytes: bool,
    pub accepts_frontend_or_watcher_ed25519_proof: bool,
    pub model_a_live_wiring_precondition_required: bool,
    pub guardian_set_runtime_loading_enabled: bool,
    pub processed_registry_runtime_loading_enabled: bool,
    pub replay_write_enabled: bool,
    pub processed_event_marking_enabled: bool,
    pub account_mutation_enabled: bool,
    pub cpi_enabled: bool,
    pub invoke_signed_enabled: bool,
    pub spl_token_mint_to_enabled: bool,
    pub process_instruction_handler_added: bool,
    pub live_route_enabled: bool,
}

pub const PHASE_41K_1_INSTRUCTIONS_SYSVAR_LIVE_WIRING_BOUNDARY_REPORT:
    Phase41K1InstructionsSysvarLiveWiringBoundaryReport =
    Phase41K1InstructionsSysvarLiveWiringBoundaryReport {
        phase: PHASE_41K_1_INSTRUCTIONS_SYSVAR_LIVE_WIRING_BOUNDARY_PHASE,
        version: PHASE_41K_1_INSTRUCTIONS_SYSVAR_LIVE_WIRING_BOUNDARY_VERSION,
        runtime_surface: "real_instructions_sysvar_loading",
        uses_real_instructions_sysvar_account_info: true,
        checks_instructions_sysvar_account_id: true,
        load_current_index_checked_used: true,
        load_instruction_at_checked_used: true,
        supports_n_prior_ed25519_precompile_instructions: true,
        accepts_single_prior_instruction_only: false,
        current_index_caller_provided: false,
        accepts_caller_provided_instruction_bytes: false,
        accepts_frontend_or_watcher_ed25519_proof: false,
        model_a_live_wiring_precondition_required: true,
        guardian_set_runtime_loading_enabled: false,
        processed_registry_runtime_loading_enabled: false,
        replay_write_enabled: false,
        processed_event_marking_enabled: false,
        account_mutation_enabled: false,
        cpi_enabled: false,
        invoke_signed_enabled: false,
        spl_token_mint_to_enabled: false,
        process_instruction_handler_added: false,
        live_route_enabled: false,
    };

pub fn phase_41k_1_instructions_sysvar_live_wiring_boundary_report(
) -> Phase41K1InstructionsSysvarLiveWiringBoundaryReport {
    PHASE_41K_1_INSTRUCTIONS_SYSVAR_LIVE_WIRING_BOUNDARY_REPORT
}

pub fn establish_phase_41k_1_instructions_sysvar_live_wiring_boundary(
    instructions_sysvar_account: Option<&AccountInfo<'_>>,
) -> Phase41K1InstructionsSysvarLiveWiringResult {
    let current_index_result = acquire_current_instruction_index_from_checked_instructions_sysvar(
        instructions_sysvar_account,
    );

    if current_index_result.status
        == Phase41D3_1CurrentInstructionIndexRuntimeStatus::MissingInstructionsSysvar
    {
        return result(
            Phase41K1InstructionsSysvarLiveWiringStatus::MissingInstructionsSysvar,
            current_index_result
                .rejection_case
                .or(Some(Phase41BRejectionCase::MissingInstructionsSysvar)),
            current_index_result.current_instruction_index,
            0,
            0,
            0,
            Vec::new(),
            false,
            false,
            false,
            false,
            false,
        );
    }

    if current_index_result.status
        != Phase41D3_1CurrentInstructionIndexRuntimeStatus::CurrentInstructionIndexAcquired
    {
        return result(
            Phase41K1InstructionsSysvarLiveWiringStatus::CurrentInstructionIndexUnavailable,
            current_index_result
                .rejection_case
                .or(Some(Phase41BRejectionCase::UnreadableInstructionsSysvar)),
            current_index_result.current_instruction_index,
            0,
            0,
            0,
            Vec::new(),
            true,
            true,
            false,
            false,
            false,
        );
    }

    let prior_range_result =
        construct_prior_instruction_index_range_from_checked_current_index(&current_index_result);

    if prior_range_result.status
        == Phase41D3_2_1PriorIndexRangeRuntimeStatus::CurrentInstructionIndexUnavailable
    {
        return result(
            Phase41K1InstructionsSysvarLiveWiringStatus::PriorIndexRangeUnavailable,
            prior_range_result
                .rejection_case
                .or(Some(Phase41BRejectionCase::UnreadableInstructionsSysvar)),
            prior_range_result.current_instruction_index,
            0,
            0,
            0,
            Vec::new(),
            true,
            true,
            false,
            false,
            false,
        );
    }

    let loading_result = load_checked_prior_instructions_from_bounded_range(
        &prior_range_result,
        instructions_sysvar_account,
    );

    derive_phase_41k_1_from_checked_prior_loading(&loading_result)
}

pub(crate) fn derive_phase_41k_1_from_checked_prior_loading(
    loading_result: &Phase41D3_2_2CheckedPriorInstructionLoadingResult,
) -> Phase41K1InstructionsSysvarLiveWiringResult {
    let Some(current_instruction_index) = loading_result.current_instruction_index else {
        return result(
            Phase41K1InstructionsSysvarLiveWiringStatus::CurrentInstructionIndexUnavailable,
            loading_result
                .rejection_case
                .or(Some(Phase41BRejectionCase::UnreadableInstructionsSysvar)),
            None,
            loading_result.loaded_instruction_count,
            0,
            0,
            Vec::new(),
            loading_result.uses_instructions_sysvar_account_info,
            loading_result.checks_instructions_sysvar_program_id,
            loading_result.load_instruction_at_checked_used,
            false,
            false,
        );
    };

    match loading_result.status {
        Phase41D3_2_2CheckedPriorInstructionLoadingStatus::PriorIndexRangeUnavailable => {
            return result(
                Phase41K1InstructionsSysvarLiveWiringStatus::PriorIndexRangeUnavailable,
                loading_result
                    .rejection_case
                    .or(Some(Phase41BRejectionCase::UnreadableInstructionsSysvar)),
                Some(current_instruction_index),
                loading_result.loaded_instruction_count,
                0,
                0,
                Vec::new(),
                loading_result.uses_instructions_sysvar_account_info,
                loading_result.checks_instructions_sysvar_program_id,
                loading_result.load_instruction_at_checked_used,
                false,
                false,
            );
        }
        Phase41D3_2_2CheckedPriorInstructionLoadingStatus::MissingInstructionsSysvar => {
            return result(
                Phase41K1InstructionsSysvarLiveWiringStatus::MissingInstructionsSysvar,
                loading_result
                    .rejection_case
                    .or(Some(Phase41BRejectionCase::MissingInstructionsSysvar)),
                Some(current_instruction_index),
                loading_result.loaded_instruction_count,
                0,
                0,
                Vec::new(),
                loading_result.uses_instructions_sysvar_account_info,
                loading_result.checks_instructions_sysvar_program_id,
                loading_result.load_instruction_at_checked_used,
                false,
                false,
            );
        }
        Phase41D3_2_2CheckedPriorInstructionLoadingStatus::CheckedPriorInstructionLoadingFailed => {
            return result(
                Phase41K1InstructionsSysvarLiveWiringStatus::PriorInstructionLoadingFailed,
                loading_result
                    .rejection_case
                    .or(Some(Phase41BRejectionCase::UnreadableInstructionsSysvar)),
                Some(current_instruction_index),
                loading_result.loaded_instruction_count,
                0,
                0,
                Vec::new(),
                loading_result.uses_instructions_sysvar_account_info,
                loading_result.checks_instructions_sysvar_program_id,
                loading_result.load_instruction_at_checked_used,
                false,
                false,
            );
        }
        Phase41D3_2_2CheckedPriorInstructionLoadingStatus::EmptyPriorRangeNoLoadingAttempted => {
            return result(
                Phase41K1InstructionsSysvarLiveWiringStatus::NoPriorInstructions,
                Some(Phase41BRejectionCase::Ed25519InstructionNotFound),
                Some(current_instruction_index),
                0,
                0,
                0,
                Vec::new(),
                loading_result.uses_instructions_sysvar_account_info,
                loading_result.checks_instructions_sysvar_program_id,
                loading_result.load_instruction_at_checked_used,
                false,
                false,
            );
        }
        Phase41D3_2_2CheckedPriorInstructionLoadingStatus::CheckedPriorInstructionsLoaded => {}
    }

    let mut prior_ed25519_precompile_instructions = Vec::new();
    let mut inspected_prior_instruction_count = 0usize;
    let mut discarded_non_ed25519_prior_instruction_count = 0usize;

    for loaded in loading_result.loaded_prior_instructions.iter() {
        inspected_prior_instruction_count += 1;

        if !loaded.loaded_instruction_is_runtime_data_only
            || loaded.is_evidence
            || loaded.authorizes_execution
        {
            return result(
                Phase41K1InstructionsSysvarLiveWiringStatus::PriorInstructionLoadingFailed,
                Some(Phase41BRejectionCase::UnreadableInstructionsSysvar),
                Some(current_instruction_index),
                loading_result.loaded_instruction_count,
                inspected_prior_instruction_count,
                discarded_non_ed25519_prior_instruction_count,
                Vec::new(),
                loading_result.uses_instructions_sysvar_account_info,
                loading_result.checks_instructions_sysvar_program_id,
                loading_result.load_instruction_at_checked_used,
                false,
                false,
            );
        }

        if loaded.instruction_index >= current_instruction_index {
            return result(
                Phase41K1InstructionsSysvarLiveWiringStatus::PriorIndexRangeUnavailable,
                Some(Phase41BRejectionCase::Ed25519InstructionAfterCurrentInstruction),
                Some(current_instruction_index),
                loading_result.loaded_instruction_count,
                inspected_prior_instruction_count,
                discarded_non_ed25519_prior_instruction_count,
                Vec::new(),
                loading_result.uses_instructions_sysvar_account_info,
                loading_result.checks_instructions_sysvar_program_id,
                loading_result.load_instruction_at_checked_used,
                false,
                false,
            );
        }

        if loaded.instruction.program_id != ed25519_program::id() {
            discarded_non_ed25519_prior_instruction_count += 1;
            continue;
        }

        prior_ed25519_precompile_instructions.push(
            Phase41K1LoadedPriorEd25519PrecompileInstruction {
                instruction_index: loaded.instruction_index,
                instruction_data_len: loaded.instruction.data.len(),
                strictly_prior_to_current_instruction: true,
                program_id_is_ed25519: true,
                loaded_from_real_instructions_sysvar: true,
                loaded_with_checked_runtime_api: true,
                runtime_data_only: true,
                model_a_applies_to_this_prior_precompile: true,
                accepts_verification_evidence: false,
                authorizes_execution: false,
            },
        );
    }

    if prior_ed25519_precompile_instructions.is_empty() {
        return result(
            Phase41K1InstructionsSysvarLiveWiringStatus::NoPriorEd25519PrecompileInstructions,
            Some(Phase41BRejectionCase::Ed25519InstructionNotFound),
            Some(current_instruction_index),
            loading_result.loaded_instruction_count,
            inspected_prior_instruction_count,
            discarded_non_ed25519_prior_instruction_count,
            prior_ed25519_precompile_instructions,
            loading_result.uses_instructions_sysvar_account_info,
            loading_result.checks_instructions_sysvar_program_id,
            loading_result.load_instruction_at_checked_used,
            false,
            false,
        );
    }

    result(
        Phase41K1InstructionsSysvarLiveWiringStatus::PriorEd25519PrecompileInstructionsLoaded,
        None,
        Some(current_instruction_index),
        loading_result.loaded_instruction_count,
        inspected_prior_instruction_count,
        discarded_non_ed25519_prior_instruction_count,
        prior_ed25519_precompile_instructions,
        loading_result.uses_instructions_sysvar_account_info,
        loading_result.checks_instructions_sysvar_program_id,
        loading_result.load_instruction_at_checked_used,
        true,
        true,
    )
}

fn result(
    status: Phase41K1InstructionsSysvarLiveWiringStatus,
    rejection_case: Option<Phase41BRejectionCase>,
    current_instruction_index: Option<usize>,
    loaded_prior_instruction_count: usize,
    inspected_prior_instruction_count: usize,
    discarded_non_ed25519_prior_instruction_count: usize,
    prior_ed25519_precompile_instructions: Vec<Phase41K1LoadedPriorEd25519PrecompileInstruction>,
    uses_real_instructions_sysvar_account_info: bool,
    checks_instructions_sysvar_account_id: bool,
    load_instruction_at_checked_used: bool,
    all_loaded_ed25519_precompiles_strictly_prior: bool,
    all_loaded_ed25519_precompiles_program_id_checked: bool,
) -> Phase41K1InstructionsSysvarLiveWiringResult {
    let prior_ed25519_precompile_count = prior_ed25519_precompile_instructions.len();

    Phase41K1InstructionsSysvarLiveWiringResult {
        status,
        rejection_case,
        current_instruction_index,
        loaded_prior_instruction_count,
        inspected_prior_instruction_count,
        discarded_non_ed25519_prior_instruction_count,
        prior_ed25519_precompile_count,
        prior_ed25519_precompile_instructions,
        consumes_phase_41d3_1_current_index_boundary: true,
        consumes_phase_41d3_2_1_prior_range_boundary: true,
        consumes_phase_41d3_2_2_checked_prior_loading_boundary: true,
        uses_real_instructions_sysvar_account_info,
        checks_instructions_sysvar_account_id,
        load_current_index_checked_used: true,
        load_instruction_at_checked_used,
        current_index_caller_provided: false,
        enumerates_n_prior_ed25519_precompile_instructions: true,
        model_a_live_wiring_precondition_preserved: prior_ed25519_precompile_count > 0,
        all_loaded_ed25519_precompiles_strictly_prior,
        all_loaded_ed25519_precompiles_program_id_checked,
        accepts_caller_provided_instruction_bytes: false,
        accepts_frontend_or_watcher_ed25519_proof: false,
        guardian_set_runtime_loading_enabled: false,
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

#[cfg(test)]
mod tests {
    use solana_program::{ed25519_program, instruction::Instruction, pubkey::Pubkey};

    use super::super::checked_prior_instruction_loading_runtime_boundary::{
        Phase41D3_2_2CheckedPriorInstructionLoadingResult,
        Phase41D3_2_2CheckedPriorInstructionLoadingStatus, Phase41D3_2_2LoadedPriorInstruction,
    };
    use super::super::instructions_sysvar_access_contract_model::Phase41BRejectionCase;
    use super::*;

    fn instruction(program_id: Pubkey, data_len: usize) -> Instruction {
        Instruction {
            program_id,
            accounts: Vec::new(),
            data: vec![0_u8; data_len],
        }
    }

    fn loaded_prior(index: usize, program_id: Pubkey) -> Phase41D3_2_2LoadedPriorInstruction {
        Phase41D3_2_2LoadedPriorInstruction {
            instruction_index: index,
            instruction: instruction(program_id, 144),
            loaded_instruction_is_runtime_data_only: true,
            is_evidence: false,
            authorizes_execution: false,
        }
    }

    fn loading_result(
        current_index: usize,
        loaded: Vec<Phase41D3_2_2LoadedPriorInstruction>,
    ) -> Phase41D3_2_2CheckedPriorInstructionLoadingResult {
        let count = loaded.len();

        Phase41D3_2_2CheckedPriorInstructionLoadingResult {
            status:
                Phase41D3_2_2CheckedPriorInstructionLoadingStatus::CheckedPriorInstructionsLoaded,
            rejection_case: None,
            current_instruction_index: Some(current_index),
            prior_index_count: count,
            attempted_loading_count: count,
            loaded_instruction_count: count,
            failed_instruction_index: None,
            loaded_prior_instructions: loaded,
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

    #[test]
    fn report_documents_41k1_runtime_surface_only() {
        let report = phase_41k_1_instructions_sysvar_live_wiring_boundary_report();

        assert_eq!(
            report.phase,
            PHASE_41K_1_INSTRUCTIONS_SYSVAR_LIVE_WIRING_BOUNDARY_PHASE
        );
        assert!(report.uses_real_instructions_sysvar_account_info);
        assert!(report.checks_instructions_sysvar_account_id);
        assert!(report.load_current_index_checked_used);
        assert!(report.load_instruction_at_checked_used);
        assert!(report.supports_n_prior_ed25519_precompile_instructions);
        assert!(!report.accepts_single_prior_instruction_only);
        assert!(!report.current_index_caller_provided);
        assert!(!report.accepts_caller_provided_instruction_bytes);
        assert!(!report.accepts_frontend_or_watcher_ed25519_proof);
        assert!(report.model_a_live_wiring_precondition_required);
        assert!(!report.guardian_set_runtime_loading_enabled);
        assert!(!report.processed_registry_runtime_loading_enabled);
        assert!(!report.replay_write_enabled);
        assert!(!report.processed_event_marking_enabled);
        assert!(!report.account_mutation_enabled);
        assert!(!report.cpi_enabled);
        assert!(!report.invoke_signed_enabled);
        assert!(!report.spl_token_mint_to_enabled);
        assert!(!report.process_instruction_handler_added);
        assert!(!report.live_route_enabled);
    }

    #[test]
    fn derives_n_prior_ed25519_precompiles_for_quorum() {
        let loaded = vec![
            loaded_prior(0, ed25519_program::id()),
            loaded_prior(1, Pubkey::new_from_array([9; 32])),
            loaded_prior(2, ed25519_program::id()),
        ];

        let input = loading_result(3, loaded);
        let result = derive_phase_41k_1_from_checked_prior_loading(&input);

        assert_eq!(
            result.status,
            Phase41K1InstructionsSysvarLiveWiringStatus::PriorEd25519PrecompileInstructionsLoaded
        );
        assert_eq!(result.current_instruction_index, Some(3));
        assert_eq!(result.loaded_prior_instruction_count, 3);
        assert_eq!(result.inspected_prior_instruction_count, 3);
        assert_eq!(result.discarded_non_ed25519_prior_instruction_count, 1);
        assert_eq!(result.prior_ed25519_precompile_count, 2);
        assert!(result.enumerates_n_prior_ed25519_precompile_instructions);
        assert!(result.model_a_live_wiring_precondition_preserved);
        assert!(result.all_loaded_ed25519_precompiles_strictly_prior);
        assert!(result.all_loaded_ed25519_precompiles_program_id_checked);
        assert!(result.load_instruction_at_checked_used);
        assert!(!result.current_index_caller_provided);
        assert!(!result.accepts_caller_provided_instruction_bytes);
        assert!(!result.accepts_frontend_or_watcher_ed25519_proof);
        assert!(!result.guardian_set_runtime_loading_enabled);
        assert!(!result.processed_registry_runtime_loading_enabled);
        assert!(!result.replay_write_enabled);
        assert!(!result.processed_event_marking_enabled);
        assert!(!result.account_mutation_enabled);
        assert!(!result.cpi_enabled);
        assert!(!result.spl_token_mint_to_enabled);
        assert!(!result.process_instruction_handler_added);
        assert!(!result.live_route_enabled);

        assert_eq!(
            result.prior_ed25519_precompile_instructions[0].instruction_index,
            0
        );
        assert_eq!(
            result.prior_ed25519_precompile_instructions[1].instruction_index,
            2
        );
        assert!(result
            .prior_ed25519_precompile_instructions
            .iter()
            .all(|entry| entry.strictly_prior_to_current_instruction));
        assert!(result
            .prior_ed25519_precompile_instructions
            .iter()
            .all(|entry| entry.program_id_is_ed25519));
        assert!(result
            .prior_ed25519_precompile_instructions
            .iter()
            .all(|entry| entry.model_a_applies_to_this_prior_precompile));
    }

    #[test]
    fn no_prior_ed25519_precompile_fails_closed() {
        let loaded = vec![
            loaded_prior(0, Pubkey::new_from_array([8; 32])),
            loaded_prior(1, Pubkey::new_from_array([9; 32])),
        ];

        let input = loading_result(2, loaded);
        let result = derive_phase_41k_1_from_checked_prior_loading(&input);

        assert_eq!(
            result.status,
            Phase41K1InstructionsSysvarLiveWiringStatus::NoPriorEd25519PrecompileInstructions
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::Ed25519InstructionNotFound)
        );
        assert_eq!(result.prior_ed25519_precompile_count, 0);
        assert!(!result.model_a_live_wiring_precondition_preserved);
        assert!(!result.accepts_frontend_or_watcher_ed25519_proof);
        assert!(!result.account_mutation_enabled);
        assert!(!result.live_route_enabled);
    }
}
