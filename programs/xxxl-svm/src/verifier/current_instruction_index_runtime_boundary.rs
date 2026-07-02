use solana_program::{
    account_info::AccountInfo, program_error::ProgramError, sysvar::instructions,
};

use super::instructions_sysvar_access_contract_model::{
    Phase41BRejectionCase, Phase41BSafetyFlags,
};

pub const PHASE_41D3_1_CURRENT_INSTRUCTION_INDEX_RUNTIME_BOUNDARY_VERSION: &str = "0.1.0";
pub const PHASE_41D3_1_CURRENT_INSTRUCTION_INDEX_RUNTIME_BOUNDARY_PHASE: &str = "41D3.1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase41D3_1CurrentInstructionIndexRuntimeStatus {
    MissingInstructionsSysvar,
    CurrentInstructionIndexUnavailable,
    CurrentInstructionIndexAcquired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase41D3_1CurrentInstructionIndexRuntimeSource {
    InstructionsSysvarLoadCurrentIndexChecked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phase41D3_1CurrentInstructionIndexRuntimeResult {
    pub status: Phase41D3_1CurrentInstructionIndexRuntimeStatus,
    pub rejection_case: Option<Phase41BRejectionCase>,
    pub current_instruction_index: Option<usize>,
    pub runtime_source: Phase41D3_1CurrentInstructionIndexRuntimeSource,
    pub uses_instructions_sysvar_account_info: bool,
    pub checks_instructions_sysvar_program_id: bool,
    pub checked_current_index_acquisition_attempted: bool,
    pub current_index_used_for_ordering_only: bool,
    pub raw_instructions_sysvar_parser_implemented: bool,
    pub load_instruction_called: bool,
    pub load_instruction_enabled: bool,
    pub locates_prior_ed25519_instruction: bool,
    pub accepts_verification_evidence: bool,
    pub authorizes_execution: bool,
    pub mutates_runtime_state: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phase41D3_1CurrentInstructionIndexRuntimeBoundaryReport {
    pub phase: &'static str,
    pub version: &'static str,
    pub runtime_source: Phase41D3_1CurrentInstructionIndexRuntimeSource,
    pub current_index_acquired_from_checked_runtime_api: bool,
    pub current_index_used_for_ordering_only: bool,
    pub prior_instruction_enumeration_enabled: bool,
    pub prior_instruction_loading_enabled: bool,
    pub raw_instructions_sysvar_parser_implemented: bool,
    pub load_instruction_called: bool,
    pub load_instruction_enabled: bool,
    pub locates_prior_ed25519_instruction: bool,
    pub accepts_verification_evidence: bool,
    pub authorizes_execution: bool,
    pub safety_flags: Phase41BSafetyFlags,
}

pub const PHASE_41D3_1_SAFETY_FLAGS: Phase41BSafetyFlags = Phase41BSafetyFlags {
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

pub const PHASE_41D3_1_CURRENT_INSTRUCTION_INDEX_RUNTIME_BOUNDARY_REPORT:
    Phase41D3_1CurrentInstructionIndexRuntimeBoundaryReport =
    Phase41D3_1CurrentInstructionIndexRuntimeBoundaryReport {
        phase: PHASE_41D3_1_CURRENT_INSTRUCTION_INDEX_RUNTIME_BOUNDARY_PHASE,
        version: PHASE_41D3_1_CURRENT_INSTRUCTION_INDEX_RUNTIME_BOUNDARY_VERSION,
        runtime_source:
            Phase41D3_1CurrentInstructionIndexRuntimeSource::InstructionsSysvarLoadCurrentIndexChecked,
        current_index_acquired_from_checked_runtime_api: true,
        current_index_used_for_ordering_only: true,
        prior_instruction_enumeration_enabled: false,
        prior_instruction_loading_enabled: false,
        raw_instructions_sysvar_parser_implemented: false,
        load_instruction_called: false,
        load_instruction_enabled: false,
        locates_prior_ed25519_instruction: false,
        accepts_verification_evidence: false,
        authorizes_execution: false,
        safety_flags: PHASE_41D3_1_SAFETY_FLAGS,
    };

pub fn acquire_current_instruction_index_from_checked_instructions_sysvar(
    instructions_sysvar_account: Option<&AccountInfo<'_>>,
) -> Phase41D3_1CurrentInstructionIndexRuntimeResult {
    let Some(account_info) = instructions_sysvar_account else {
        return result(
            Phase41D3_1CurrentInstructionIndexRuntimeStatus::MissingInstructionsSysvar,
            Some(Phase41BRejectionCase::MissingInstructionsSysvar),
            None,
            false,
            false,
            false,
        );
    };

    if account_info.key != &instructions::id() {
        return result(
            Phase41D3_1CurrentInstructionIndexRuntimeStatus::MissingInstructionsSysvar,
            Some(Phase41BRejectionCase::MissingInstructionsSysvar),
            None,
            true,
            true,
            false,
        );
    }

    map_checked_current_instruction_index_result(instructions::load_current_index_checked(
        account_info,
    ))
}

pub fn map_checked_current_instruction_index_result(
    checked_current_index_result: Result<u16, ProgramError>,
) -> Phase41D3_1CurrentInstructionIndexRuntimeResult {
    match checked_current_index_result {
        Ok(current_instruction_index) => result(
            Phase41D3_1CurrentInstructionIndexRuntimeStatus::CurrentInstructionIndexAcquired,
            None,
            Some(current_instruction_index as usize),
            true,
            true,
            true,
        ),
        Err(_) => result(
            Phase41D3_1CurrentInstructionIndexRuntimeStatus::CurrentInstructionIndexUnavailable,
            Some(Phase41BRejectionCase::UnreadableInstructionsSysvar),
            None,
            true,
            true,
            true,
        ),
    }
}

pub fn phase_41d3_1_current_instruction_index_runtime_boundary_report(
) -> Phase41D3_1CurrentInstructionIndexRuntimeBoundaryReport {
    PHASE_41D3_1_CURRENT_INSTRUCTION_INDEX_RUNTIME_BOUNDARY_REPORT
}

fn result(
    status: Phase41D3_1CurrentInstructionIndexRuntimeStatus,
    rejection_case: Option<Phase41BRejectionCase>,
    current_instruction_index: Option<usize>,
    uses_instructions_sysvar_account_info: bool,
    checks_instructions_sysvar_program_id: bool,
    checked_current_index_acquisition_attempted: bool,
) -> Phase41D3_1CurrentInstructionIndexRuntimeResult {
    Phase41D3_1CurrentInstructionIndexRuntimeResult {
        status,
        rejection_case,
        current_instruction_index,
        runtime_source:
            Phase41D3_1CurrentInstructionIndexRuntimeSource::InstructionsSysvarLoadCurrentIndexChecked,
        uses_instructions_sysvar_account_info,
        checks_instructions_sysvar_program_id,
        checked_current_index_acquisition_attempted,
        current_index_used_for_ordering_only: true,
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
    use solana_program::{account_info::AccountInfo, pubkey::Pubkey, sysvar::instructions};

    use super::*;

    fn account_info_for_key<'a>(
        key: &'a Pubkey,
        owner: &'a Pubkey,
        lamports: &'a mut u64,
        data: &'a mut [u8],
    ) -> AccountInfo<'a> {
        AccountInfo::new(key, false, false, lamports, data, owner, false, 0)
    }

    #[test]
    fn missing_account_maps_to_missing_instructions_sysvar() {
        let result = acquire_current_instruction_index_from_checked_instructions_sysvar(None);

        assert_eq!(
            result.status,
            Phase41D3_1CurrentInstructionIndexRuntimeStatus::MissingInstructionsSysvar
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::MissingInstructionsSysvar)
        );
        assert_eq!(result.current_instruction_index, None);
        assert!(!result.uses_instructions_sysvar_account_info);
        assert!(!result.checks_instructions_sysvar_program_id);
        assert!(!result.checked_current_index_acquisition_attempted);
        assert!(result.current_index_used_for_ordering_only);
        assert!(!result.raw_instructions_sysvar_parser_implemented);
        assert!(!result.load_instruction_called);
        assert!(!result.locates_prior_ed25519_instruction);
        assert!(!result.accepts_verification_evidence);
        assert!(!result.authorizes_execution);
        assert!(!result.mutates_runtime_state);
    }

    #[test]
    fn wrong_account_key_maps_to_missing_instructions_sysvar() {
        let key = Pubkey::new_from_array([7; 32]);
        let owner = Pubkey::new_from_array([9; 32]);
        let mut lamports = 0;
        let mut data = [0_u8; 8];
        let account = account_info_for_key(&key, &owner, &mut lamports, &mut data);

        let result =
            acquire_current_instruction_index_from_checked_instructions_sysvar(Some(&account));

        assert_eq!(
            result.status,
            Phase41D3_1CurrentInstructionIndexRuntimeStatus::MissingInstructionsSysvar
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::MissingInstructionsSysvar)
        );
        assert_eq!(result.current_instruction_index, None);
        assert!(result.uses_instructions_sysvar_account_info);
        assert!(result.checks_instructions_sysvar_program_id);
        assert!(!result.checked_current_index_acquisition_attempted);
        assert!(!result.load_instruction_called);
        assert!(!result.accepts_verification_evidence);
    }

    #[test]
    fn zeroed_sysvar_data_can_only_map_to_ordering_index_not_authorization() {
        let key = instructions::id();
        let owner = Pubkey::new_from_array([9; 32]);
        let mut lamports = 0;
        let mut data = [0_u8; 8];
        let account = account_info_for_key(&key, &owner, &mut lamports, &mut data);

        let result =
            acquire_current_instruction_index_from_checked_instructions_sysvar(Some(&account));

        assert_eq!(
            result.status,
            Phase41D3_1CurrentInstructionIndexRuntimeStatus::CurrentInstructionIndexAcquired
        );
        assert_eq!(result.rejection_case, None);
        assert!(result.current_instruction_index.is_some());
        assert!(result.uses_instructions_sysvar_account_info);
        assert!(result.checks_instructions_sysvar_program_id);
        assert!(result.checked_current_index_acquisition_attempted);
        assert!(result.current_index_used_for_ordering_only);
        assert!(!result.raw_instructions_sysvar_parser_implemented);
        assert!(!result.load_instruction_called);
        assert!(!result.locates_prior_ed25519_instruction);
        assert!(!result.accepts_verification_evidence);
        assert!(!result.authorizes_execution);
        assert!(!result.mutates_runtime_state);
    }

    #[test]
    fn checked_current_index_success_maps_to_ordering_only_index() {
        let result = map_checked_current_instruction_index_result(Ok(5));

        assert_eq!(
            result.status,
            Phase41D3_1CurrentInstructionIndexRuntimeStatus::CurrentInstructionIndexAcquired
        );
        assert_eq!(result.rejection_case, None);
        assert_eq!(result.current_instruction_index, Some(5));
        assert!(result.uses_instructions_sysvar_account_info);
        assert!(result.checks_instructions_sysvar_program_id);
        assert!(result.checked_current_index_acquisition_attempted);
        assert!(result.current_index_used_for_ordering_only);
        assert!(!result.raw_instructions_sysvar_parser_implemented);
        assert!(!result.load_instruction_called);
        assert!(!result.load_instruction_enabled);
        assert!(!result.locates_prior_ed25519_instruction);
        assert!(!result.accepts_verification_evidence);
        assert!(!result.authorizes_execution);
        assert!(!result.mutates_runtime_state);
    }

    #[test]
    fn checked_current_index_error_maps_to_unreadable_instructions_sysvar() {
        let result =
            map_checked_current_instruction_index_result(Err(ProgramError::InvalidArgument));

        assert_eq!(
            result.status,
            Phase41D3_1CurrentInstructionIndexRuntimeStatus::CurrentInstructionIndexUnavailable
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::UnreadableInstructionsSysvar)
        );
        assert_eq!(result.current_instruction_index, None);
        assert!(result.checked_current_index_acquisition_attempted);
        assert!(!result.load_instruction_called);
        assert!(!result.locates_prior_ed25519_instruction);
        assert!(!result.accepts_verification_evidence);
        assert!(!result.authorizes_execution);
    }

    #[test]
    fn phase_41d3_1_flips_no_instruction_loading_or_prior_lookup_flags() {
        let flags = PHASE_41D3_1_SAFETY_FLAGS;

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
    fn phase_41d3_1_report_preserves_current_index_only_boundary() {
        let report = phase_41d3_1_current_instruction_index_runtime_boundary_report();

        assert_eq!(report.phase, "41D3.1");
        assert_eq!(report.version, "0.1.0");
        assert_eq!(
            report.runtime_source,
            Phase41D3_1CurrentInstructionIndexRuntimeSource::InstructionsSysvarLoadCurrentIndexChecked
        );
        assert!(report.current_index_acquired_from_checked_runtime_api);
        assert!(report.current_index_used_for_ordering_only);
        assert!(!report.prior_instruction_enumeration_enabled);
        assert!(!report.prior_instruction_loading_enabled);
        assert!(!report.raw_instructions_sysvar_parser_implemented);
        assert!(!report.load_instruction_called);
        assert!(!report.load_instruction_enabled);
        assert!(!report.locates_prior_ed25519_instruction);
        assert!(!report.accepts_verification_evidence);
        assert!(!report.authorizes_execution);
        assert!(!report.safety_flags.verification_evidence_accepted);
        assert!(!report.safety_flags.authorization_enabled);
        assert!(!report.safety_flags.spl_token_mint_to_enabled);
        assert!(!report.safety_flags.live_route_enabled);
    }
}
