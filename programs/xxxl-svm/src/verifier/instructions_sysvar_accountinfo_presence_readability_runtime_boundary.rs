use solana_program::{account_info::AccountInfo, sysvar::instructions};

use super::instructions_sysvar_access_contract_model::Phase41BSafetyFlags;
use super::instructions_sysvar_presence_readability_boundary::{
    check_instructions_sysvar_presence_readability, Phase41C1InstructionsSysvarContainerView,
    Phase41C1InstructionsSysvarPresenceReadabilityResult, Phase41C1RuntimeApiSelection,
};

pub const PHASE_41D1_INSTRUCTIONS_SYSVAR_ACCOUNTINFO_PRESENCE_READABILITY_RUNTIME_BOUNDARY_VERSION: &str =
    "0.1.0";
pub const PHASE_41D1_INSTRUCTIONS_SYSVAR_ACCOUNTINFO_PRESENCE_READABILITY_RUNTIME_BOUNDARY_PHASE:
    &str = "41D1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase41D1RuntimeReadBoundary {
    AccountInfoInstructionsSysvarPresenceReadabilityOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phase41D1InstructionsSysvarAccountInfoPresenceReadabilityRuntimeReport {
    pub phase: &'static str,
    pub version: &'static str,
    pub runtime_read_boundary: Phase41D1RuntimeReadBoundary,
    pub maps_to_phase_41c1_descriptor_boundary: bool,
    pub account_info_parser_implemented: bool,
    pub raw_instructions_sysvar_parser_implemented: bool,
    pub load_instruction_called: bool,
    pub load_instruction_enabled: bool,
    pub reads_concrete_instruction_content: bool,
    pub derives_current_instruction_identity: bool,
    pub locates_prior_ed25519_instruction: bool,
    pub accepts_verification_evidence: bool,
    pub safety_flags: Phase41BSafetyFlags,
}

pub const PHASE_41D1_SAFETY_FLAGS: Phase41BSafetyFlags = Phase41BSafetyFlags {
    raw_instructions_sysvar_parser_implemented: false,
    account_info_parser_implemented: true,
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

pub const PHASE_41D1_INSTRUCTIONS_SYSVAR_ACCOUNTINFO_PRESENCE_READABILITY_RUNTIME_REPORT:
    Phase41D1InstructionsSysvarAccountInfoPresenceReadabilityRuntimeReport =
    Phase41D1InstructionsSysvarAccountInfoPresenceReadabilityRuntimeReport {
        phase:
            PHASE_41D1_INSTRUCTIONS_SYSVAR_ACCOUNTINFO_PRESENCE_READABILITY_RUNTIME_BOUNDARY_PHASE,
        version:
            PHASE_41D1_INSTRUCTIONS_SYSVAR_ACCOUNTINFO_PRESENCE_READABILITY_RUNTIME_BOUNDARY_VERSION,
        runtime_read_boundary:
            Phase41D1RuntimeReadBoundary::AccountInfoInstructionsSysvarPresenceReadabilityOnly,
        maps_to_phase_41c1_descriptor_boundary: true,
        account_info_parser_implemented: true,
        raw_instructions_sysvar_parser_implemented: false,
        load_instruction_called: false,
        load_instruction_enabled: false,
        reads_concrete_instruction_content: false,
        derives_current_instruction_identity: false,
        locates_prior_ed25519_instruction: false,
        accepts_verification_evidence: false,
        safety_flags: PHASE_41D1_SAFETY_FLAGS,
    };

pub fn check_real_instructions_sysvar_accountinfo_presence_readability(
    instructions_sysvar_account: Option<&AccountInfo<'_>>,
) -> Phase41C1InstructionsSysvarPresenceReadabilityResult {
    let Some(account_info) = instructions_sysvar_account else {
        return check_instructions_sysvar_presence_readability(
            Phase41C1InstructionsSysvarContainerView {
                instructions_sysvar_supplied: false,
                instructions_sysvar_readable: false,
            },
        );
    };

    if account_info.key != &instructions::id() {
        return check_instructions_sysvar_presence_readability(
            Phase41C1InstructionsSysvarContainerView {
                instructions_sysvar_supplied: false,
                instructions_sysvar_readable: false,
            },
        );
    }

    let instructions_sysvar_readable = account_info.try_borrow_data().is_ok();

    check_instructions_sysvar_presence_readability(Phase41C1InstructionsSysvarContainerView {
        instructions_sysvar_supplied: true,
        instructions_sysvar_readable,
    })
}

pub fn phase_41d1_instructions_sysvar_accountinfo_presence_readability_runtime_report(
) -> Phase41D1InstructionsSysvarAccountInfoPresenceReadabilityRuntimeReport {
    PHASE_41D1_INSTRUCTIONS_SYSVAR_ACCOUNTINFO_PRESENCE_READABILITY_RUNTIME_REPORT
}

#[cfg(test)]
mod tests {
    use solana_program::{account_info::AccountInfo, pubkey::Pubkey, sysvar::instructions};

    use super::super::instructions_sysvar_access_contract_model::Phase41BRejectionCase;
    use super::super::instructions_sysvar_presence_readability_boundary::Phase41C1InstructionsSysvarReadStatus;
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
        let result = check_real_instructions_sysvar_accountinfo_presence_readability(None);

        assert_eq!(
            result.status,
            Phase41C1InstructionsSysvarReadStatus::MissingInstructionsSysvar
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::MissingInstructionsSysvar)
        );
        assert!(result.concrete_runtime_api_selected);
        assert!(!result.reads_concrete_instruction_content);
        assert!(!result.derives_current_instruction_identity);
        assert!(!result.locates_prior_ed25519_instruction);
        assert!(!result.accepts_verification_evidence);
    }

    #[test]
    fn wrong_account_key_maps_to_missing_instructions_sysvar() {
        let key = Pubkey::new_from_array([7; 32]);
        let owner = Pubkey::new_from_array([9; 32]);
        let mut lamports = 0;
        let mut data = [0_u8; 8];
        let account = account_info_for_key(&key, &owner, &mut lamports, &mut data);

        let result =
            check_real_instructions_sysvar_accountinfo_presence_readability(Some(&account));

        assert_eq!(
            result.status,
            Phase41C1InstructionsSysvarReadStatus::MissingInstructionsSysvar
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::MissingInstructionsSysvar)
        );
    }

    #[test]
    fn readable_instructions_sysvar_account_maps_to_present_and_readable() {
        let key = instructions::id();
        let owner = Pubkey::new_from_array([9; 32]);
        let mut lamports = 0;
        let mut data = [0_u8; 8];
        let account = account_info_for_key(&key, &owner, &mut lamports, &mut data);

        let result =
            check_real_instructions_sysvar_accountinfo_presence_readability(Some(&account));

        assert_eq!(
            result.status,
            Phase41C1InstructionsSysvarReadStatus::PresentAndReadable
        );
        assert_eq!(result.rejection_case, None);
        assert_eq!(
            result.selected_runtime_api,
            Phase41C1RuntimeApiSelection::SvmInstructionsSysvarContainerPresenceReadabilityBoundary
        );
        assert!(result.concrete_runtime_api_selected);
        assert!(!result.reads_concrete_instruction_content);
        assert!(!result.derives_current_instruction_identity);
        assert!(!result.locates_prior_ed25519_instruction);
        assert!(!result.accepts_verification_evidence);
    }

    #[test]
    fn borrow_conflict_maps_to_unreadable_instructions_sysvar() {
        let key = instructions::id();
        let owner = Pubkey::new_from_array([9; 32]);
        let mut lamports = 0;
        let mut data = [0_u8; 8];
        let account = account_info_for_key(&key, &owner, &mut lamports, &mut data);

        let borrow_result = account.try_borrow_mut_data();
        assert!(borrow_result.is_ok());
        let borrow = match borrow_result {
            Ok(borrow) => borrow,
            Err(_) => return,
        };

        let result =
            check_real_instructions_sysvar_accountinfo_presence_readability(Some(&account));

        drop(borrow);

        assert_eq!(
            result.status,
            Phase41C1InstructionsSysvarReadStatus::UnreadableInstructionsSysvar
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::UnreadableInstructionsSysvar)
        );
        assert!(result.concrete_runtime_api_selected);
        assert!(!result.reads_concrete_instruction_content);
        assert!(!result.derives_current_instruction_identity);
        assert!(!result.locates_prior_ed25519_instruction);
        assert!(!result.accepts_verification_evidence);
    }

    #[test]
    fn phase_41d1_flips_only_account_info_parser_and_concrete_api_flags() {
        let flags = PHASE_41D1_SAFETY_FLAGS;

        assert!(!flags.raw_instructions_sysvar_parser_implemented);
        assert!(flags.account_info_parser_implemented);
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
    fn phase_41d1_report_preserves_runtime_read_boundary() {
        let report =
            phase_41d1_instructions_sysvar_accountinfo_presence_readability_runtime_report();

        assert_eq!(report.phase, "41D1");
        assert_eq!(report.version, "0.1.0");
        assert_eq!(
            report.runtime_read_boundary,
            Phase41D1RuntimeReadBoundary::AccountInfoInstructionsSysvarPresenceReadabilityOnly
        );
        assert!(report.maps_to_phase_41c1_descriptor_boundary);
        assert!(report.account_info_parser_implemented);
        assert!(!report.raw_instructions_sysvar_parser_implemented);
        assert!(!report.load_instruction_called);
        assert!(!report.load_instruction_enabled);
        assert!(!report.reads_concrete_instruction_content);
        assert!(!report.derives_current_instruction_identity);
        assert!(!report.locates_prior_ed25519_instruction);
        assert!(!report.accepts_verification_evidence);
    }
}
